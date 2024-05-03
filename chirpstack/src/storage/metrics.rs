use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use anyhow::Result;
use chrono::{
    DateTime, Datelike, Duration as ChronoDuration, Local, Months, NaiveDate, NaiveDateTime,
    Timelike,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::storage::{get_async_redis_conn, redis_key};

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Aggregation {
    MINUTE,
    HOUR,
    DAY,
    MONTH,
}

impl Aggregation {
    pub fn default_aggregations() -> Vec<Aggregation> {
        vec![Aggregation::HOUR, Aggregation::DAY, Aggregation::MONTH]
    }
}

impl fmt::Display for Aggregation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    COUNTER,
    ABSOLUTE,
    GAUGE,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Record {
    pub time: DateTime<Local>,
    pub kind: Kind,
    pub metrics: HashMap<String, f64>,
}

fn get_ttl(a: Aggregation) -> Duration {
    match a {
        Aggregation::MINUTE => Duration::from_secs(60 * 60 * 2), // two hours
        Aggregation::HOUR => Duration::from_secs(60 * 60 * 24 * 2), // two days
        Aggregation::DAY => Duration::from_secs(60 * 60 * 24 * 31 * 2), // two months
        Aggregation::MONTH => Duration::from_secs(60 * 60 * 24 * 365 * 2), // two years
    }
}

fn get_key(name: &str, a: Aggregation, dt: NaiveDateTime) -> String {
    redis_key(format!(
        "metrics:{{{}}}:{}:{}",
        name,
        a,
        dt.format("%Y%m%d%H%M")
    ))
}

pub async fn save_state(name: &str, state: &str) -> Result<()> {
    let key = redis_key(format!("metrics:{{{}}}", name));
    let ttl = get_ttl(Aggregation::MONTH);

    redis::cmd("PSETEX")
        .arg(key)
        .arg(ttl.as_millis() as usize)
        .arg(state)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(state = %state, "State saved");
    Ok(())
}

pub async fn save(name: &str, record: &Record, aggregations: &[Aggregation]) -> Result<()> {
    if record.metrics.is_empty() {
        return Ok(());
    }

    let mut pipe = redis::pipe();
    pipe.atomic();

    for a in aggregations {
        let ttl = get_ttl(*a);

        let ts: NaiveDateTime = match a {
            Aggregation::MINUTE => {
                NaiveDate::from_ymd_opt(record.time.year(), record.time.month(), record.time.day())
                    .ok_or_else(|| anyhow!("Invalid date"))?
                    .and_hms_opt(record.time.hour(), record.time.minute(), 0)
                    .ok_or_else(|| anyhow!("Invalid time"))?
            }
            Aggregation::HOUR => {
                NaiveDate::from_ymd_opt(record.time.year(), record.time.month(), record.time.day())
                    .ok_or_else(|| anyhow!("Invalid date"))?
                    .and_hms_opt(record.time.hour(), 0, 0)
                    .ok_or_else(|| anyhow!("Invalid time"))?
            }
            Aggregation::DAY => {
                NaiveDate::from_ymd_opt(record.time.year(), record.time.month(), record.time.day())
                    .ok_or_else(|| anyhow!("Invalid date"))?
                    .and_hms_opt(0, 0, 0)
                    .ok_or_else(|| anyhow!("Invalid time"))?
            }
            Aggregation::MONTH => {
                NaiveDate::from_ymd_opt(record.time.year(), record.time.month(), 1)
                    .ok_or_else(|| anyhow!("Invalid date"))?
                    .and_hms_opt(0, 0, 0)
                    .ok_or_else(|| anyhow!("Invalid time"))?
            }
        };

        let key = get_key(name, *a, ts);

        for (k, v) in &record.metrics {
            // Passing a reference to hincr will return a runtime error.
            let k = k.clone();
            let v = *v;

            match record.kind {
                Kind::COUNTER => {
                    pipe.cmd("HSET").arg(&key).arg(k).arg(v).ignore();
                }
                Kind::ABSOLUTE => {
                    pipe.cmd("HINCRBYFLOAT").arg(&key).arg(k).arg(v).ignore();
                }
                Kind::GAUGE => {
                    pipe.cmd("HINCRBYFLOAT")
                        .arg(&key)
                        .arg(format!("_{}_count", k))
                        .arg(1.0)
                        .ignore();
                    pipe.cmd("HINCRBYFLOAT").arg(&key).arg(k).arg(v).ignore();
                }
            }
        }

        pipe.cmd("PEXPIRE")
            .arg(&key)
            .arg(ttl.as_millis() as usize)
            .ignore();

        info!(name = %name, aggregation = %a, "Metrics saved");
    }

    pipe.query_async(&mut get_async_redis_conn().await?).await?;

    Ok(())
}

pub async fn get_state(name: &str) -> Result<String> {
    let key = redis_key(format!("metrics:{{{}}}", name));

    let v: Option<String> = redis::cmd("GET")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;
    Ok(v.unwrap_or_default())
}

pub async fn get(
    name: &str,
    kind: Kind,
    a: Aggregation,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<Vec<Record>> {
    let mut keys: Vec<String> = Vec::new();
    let mut timestamps: Vec<NaiveDateTime> = Vec::new();

    match a {
        Aggregation::MINUTE => {
            let mut ts = NaiveDate::from_ymd_opt(start.year(), start.month(), start.day())
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(start.hour(), start.minute(), 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;
            let end = NaiveDate::from_ymd_opt(end.year(), end.month(), end.day())
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(end.hour(), end.minute(), 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts += ChronoDuration::minutes(1);
            }
        }
        Aggregation::HOUR => {
            let mut ts = NaiveDate::from_ymd_opt(start.year(), start.month(), start.day())
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(start.hour(), 0, 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;
            let end = NaiveDate::from_ymd_opt(end.year(), end.month(), end.day())
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(end.hour(), 0, 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts += ChronoDuration::hours(1);
            }
        }
        Aggregation::DAY => {
            let mut ts = NaiveDate::from_ymd_opt(start.year(), start.month(), start.day())
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;
            let end = NaiveDate::from_ymd_opt(end.year(), end.month(), end.day())
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts += ChronoDuration::days(1);
            }
        }
        Aggregation::MONTH => {
            let mut ts = NaiveDate::from_ymd_opt(start.year(), start.month(), 1)
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;
            let end = NaiveDate::from_ymd_opt(end.year(), end.month(), 1)
                .ok_or_else(|| anyhow!("Invalid date"))?
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| anyhow!("Invalid time"))?;

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts = ts
                    .checked_add_months(Months::new(1))
                    .ok_or_else(|| anyhow!("Add month error"))?;
            }
        }
    }

    if keys.is_empty() {
        return Ok(Vec::new());
    }

    let mut pipe = redis::pipe();

    for k in &keys {
        pipe.cmd("HGETALL").arg(k);
    }

    let res: Vec<HashMap<String, f64>> =
        pipe.query_async(&mut get_async_redis_conn().await?).await?;
    let mut out: Vec<Record> = Vec::new();

    for (i, r) in res.iter().enumerate() {
        let tz = match timestamps[i].and_local_timezone(Local) {
            chrono::LocalResult::Single(v) => v,
            _ => continue,
        };

        let mut metrics = r.clone();

        // In case of GAUGE values, the total aggregated value must be divided by the
        // number of measurements.
        if kind == Kind::GAUGE {
            let counts: HashMap<String, f64> = r
                .iter()
                .filter(|(k, _)| k.starts_with('_') && k.ends_with("_count"))
                .map(|(k, v)| (k.to_string(), *v))
                .collect();

            for (k, count) in counts {
                let k = k.strip_prefix('_').unwrap().strip_suffix("_count").unwrap();
                if let Some(v) = metrics.get_mut(k) {
                    *v /= count;
                }
            }
        }

        out.push(Record {
            time: tz,
            kind,
            metrics: metrics
                .iter()
                .filter(|(k, _)| !k.starts_with('_'))
                .map(|(k, v)| (k.to_string(), *v))
                .collect(),
        });
    }

    Ok(out)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;
    use chrono::TimeZone;

    #[tokio::test]
    async fn test_minute() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 1, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 1, 10).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 4f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 2, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::MINUTE]).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_hour() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 1, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 2, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 2, 1, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::HOUR]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::ABSOLUTE,
            Aggregation::HOUR,
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2018, 1, 1, 2, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.with_ymd_and_hms(2018, 1, 1, 2, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                }
            ],
            resp
        );
    }

    #[tokio::test]
    async fn test_day() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 2, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 2, 1, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::DAY]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::ABSOLUTE,
            Aggregation::DAY,
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2018, 1, 2, 1, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.with_ymd_and_hms(2018, 1, 1, 0, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.with_ymd_and_hms(2018, 1, 2, 0, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                }
            ],
            resp
        );
    }

    #[tokio::test]
    async fn test_day_dst_transition() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2022, 10, 30, 1, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2022, 10, 30, 5, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2022, 10, 31, 1, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::DAY]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::ABSOLUTE,
            Aggregation::DAY,
            Local.with_ymd_and_hms(2022, 10, 30, 1, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2022, 10, 31, 1, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.with_ymd_and_hms(2022, 10, 30, 0, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.with_ymd_and_hms(2022, 10, 31, 0, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                }
            ],
            resp
        );
    }

    #[tokio::test]
    async fn test_month() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 0, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 2, 0, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 2, 1, 0, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::MONTH]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::ABSOLUTE,
            Aggregation::MONTH,
            Local.with_ymd_and_hms(2018, 1, 1, 0, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2018, 2, 1, 0, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.with_ymd_and_hms(2018, 1, 1, 0, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.with_ymd_and_hms(2018, 2, 1, 0, 0, 0).unwrap(),
                    kind: Kind::ABSOLUTE,
                    metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                }
            ],
            resp
        );
    }

    #[tokio::test]
    async fn test_counter() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 1, 0).unwrap(),
                kind: Kind::COUNTER,
                metrics: [("foo".into(), 1.0), ("bar".into(), 2.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 2, 0).unwrap(),
                kind: Kind::COUNTER,
                metrics: [("foo".into(), 2.0), ("bar".into(), 4.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::HOUR]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::COUNTER,
            Aggregation::HOUR,
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
                kind: Kind::COUNTER,
                metrics: [("foo".into(), 2.0), ("bar".into(), 4.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },],
            resp
        );
    }

    #[tokio::test]
    async fn test_absolute() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 1, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 1.0), ("bar".into(), 2.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 2, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 2.0), ("bar".into(), 4.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::HOUR]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::ABSOLUTE,
            Aggregation::HOUR,
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
                kind: Kind::ABSOLUTE,
                metrics: [("foo".into(), 3.0), ("bar".into(), 6.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },],
            resp
        );
    }

    #[tokio::test]
    async fn test_gauge() {
        let _guard = test::prepare().await;

        let records = vec![
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 1, 0).unwrap(),
                kind: Kind::GAUGE,
                metrics: [("foo".into(), 1.0), ("bar".into(), 2.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 2, 0).unwrap(),
                kind: Kind::GAUGE,
                metrics: [("foo".into(), 2.0), ("bar".into(), 4.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },
        ];
        for r in &records {
            save("test", r, &[Aggregation::HOUR]).await.unwrap();
        }

        let resp = get(
            "test",
            Kind::GAUGE,
            Aggregation::HOUR,
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
            Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![Record {
                time: Local.with_ymd_and_hms(2018, 1, 1, 1, 0, 0).unwrap(),
                kind: Kind::GAUGE,
                metrics: [("foo".into(), 1.5), ("bar".into(), 3.0)]
                    .iter()
                    .cloned()
                    .collect(),
            },],
            resp
        );
    }
}
