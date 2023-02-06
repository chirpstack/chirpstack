use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use anyhow::Result;
use chrono::{DateTime, Datelike, Duration as ChronoDuration, Local, TimeZone, Timelike};
use serde::{Deserialize, Serialize};
use tokio::task;
use tracing::info;

use crate::storage::{get_redis_conn, redis_key};

#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Aggregation {
    HOUR,
    DAY,
    MONTH,
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
        Aggregation::HOUR => Duration::from_secs(60 * 60 * 24 * 2), // two days
        Aggregation::DAY => Duration::from_secs(60 * 60 * 24 * 31 * 2), // two months
        Aggregation::MONTH => Duration::from_secs(60 * 60 * 24 * 365 * 2), // two years
    }
}

fn get_aggregations() -> Vec<Aggregation> {
    vec![Aggregation::HOUR, Aggregation::DAY, Aggregation::MONTH]
}

fn get_key(name: &str, a: Aggregation, dt: DateTime<Local>) -> String {
    redis_key(format!(
        "metrics:{{{}}}:{}:{}",
        name,
        a,
        dt.format("%Y%m%d%H%M")
    ))
}

pub async fn save_state(name: &str, state: &str) -> Result<()> {
    task::spawn_blocking({
        let key = redis_key(format!("metrics:{{{}}}", name));
        let state = state.to_string();
        let ttl = get_ttl(Aggregation::MONTH);

        move || -> Result<()> {
            let mut c = get_redis_conn()?;
            redis::cmd("PSETEX")
                .arg(key)
                .arg(ttl.as_millis() as usize)
                .arg(state)
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await??;

    info!(state = %state, "State saved");
    Ok(())
}

pub async fn save(name: &str, record: &Record) -> Result<()> {
    for a in get_aggregations() {
        save_for_interval(a, name, record).await?;
    }

    Ok(())
}

async fn save_for_interval(a: Aggregation, name: &str, record: &Record) -> Result<()> {
    if record.metrics.is_empty() {
        return Ok(());
    }

    task::spawn_blocking({
        let name = name.to_string();
        let record = record.clone();
        let ttl = get_ttl(a);

        let ts: DateTime<Local> = match a {
            Aggregation::HOUR => Local
                .with_ymd_and_hms(
                    record.time.year(),
                    record.time.month(),
                    record.time.day(),
                    record.time.hour(),
                    0,
                    0,
                )
                .unwrap(),
            Aggregation::DAY => Local
                .with_ymd_and_hms(
                    record.time.year(),
                    record.time.month(),
                    record.time.day(),
                    0,
                    0,
                    0,
                )
                .unwrap(),
            Aggregation::MONTH => Local
                .with_ymd_and_hms(record.time.year(), record.time.month(), 1, 0, 0, 0)
                .unwrap(),
        };

        move || -> Result<()> {
            let mut c = get_redis_conn()?;
            let key = get_key(&name, a, ts);
            let mut pipe = c.new_pipeline();
            pipe.atomic();

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
                .ignore()
                .query(&mut c)?;

            Ok(())
        }
    })
    .await??;
    info!(name = %name, aggregation = %a, "Metrics saved");
    Ok(())
}

pub async fn get_state(name: &str) -> Result<String> {
    task::spawn_blocking({
        let key = redis_key(format!("metrics:{{{}}}", name));
        move || -> Result<String> {
            let mut c = get_redis_conn()?;
            let v: Option<String> = redis::cmd("GET").arg(key).query(&mut *c)?;
            Ok(v.unwrap_or_default())
        }
    })
    .await?
}

pub async fn get(
    name: &str,
    kind: Kind,
    a: Aggregation,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<Vec<Record>> {
    let mut keys: Vec<String> = Vec::new();
    let mut timestamps: Vec<DateTime<Local>> = Vec::new();

    match a {
        Aggregation::HOUR => {
            let mut ts = Local
                .with_ymd_and_hms(start.year(), start.month(), start.day(), start.hour(), 0, 0)
                .unwrap();
            let end = Local
                .with_ymd_and_hms(end.year(), end.month(), end.day(), end.hour(), 0, 0)
                .unwrap();

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts += ChronoDuration::hours(1);
            }
        }
        Aggregation::DAY => {
            let mut ts = Local
                .with_ymd_and_hms(start.year(), start.month(), start.day(), 0, 0, 0)
                .unwrap();
            let end = Local
                .with_ymd_and_hms(end.year(), end.month(), end.day(), 0, 0, 0)
                .unwrap();

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts = {
                    if (ts + ChronoDuration::days(1)).day() == ts.day() {
                        // In case of DST to non-DST transition, the ts is incremented with less
                        // than 24h and we end up with the same day. Therefore we increment by two
                        // days.
                        (ts + ChronoDuration::days(2))
                            .date_naive()
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_local_timezone(Local)
                            .unwrap()
                    } else {
                        // Make sure that the timestamp stays at midnight in case of non-DST to DST
                        // change.
                        (ts + ChronoDuration::days(1))
                            .date_naive()
                            .and_hms_opt(0, 0, 0)
                            .unwrap()
                            .and_local_timezone(Local)
                            .unwrap()
                    }
                };
            }
        }
        Aggregation::MONTH => {
            let mut ts = Local
                .with_ymd_and_hms(start.year(), start.month(), 1, 0, 0, 0)
                .unwrap();
            let end = Local
                .with_ymd_and_hms(end.year(), end.month(), 1, 0, 0, 0)
                .unwrap();

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts = if ts.month() == 12 {
                    Local
                        .with_ymd_and_hms(ts.year() + 1, 1, 1, 0, 0, 0)
                        .unwrap()
                } else {
                    Local
                        .with_ymd_and_hms(ts.year(), ts.month() + 1, 1, 0, 0, 0)
                        .unwrap()
                };
            }
        }
    }

    if keys.is_empty() {
        return Ok(Vec::new());
    }

    task::spawn_blocking({
        let keys = keys.clone();
        move || -> Result<Vec<Record>> {
            let mut c = get_redis_conn()?;
            let mut pipe = c.new_pipeline();

            for k in &keys {
                pipe.cmd("HGETALL").arg(k);
            }

            let res: Vec<HashMap<String, f64>> = pipe.query(&mut c)?;
            let mut out: Vec<Record> = Vec::new();

            for (i, r) in res.iter().enumerate() {
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
                    time: timestamps[i],
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
    })
    .await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

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
            save_for_interval(Aggregation::HOUR, "test", r)
                .await
                .unwrap();
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
            save_for_interval(Aggregation::DAY, "test", r)
                .await
                .unwrap();
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
            save_for_interval(Aggregation::DAY, "test", r)
                .await
                .unwrap();
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
            save_for_interval(Aggregation::MONTH, "test", r)
                .await
                .unwrap();
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
            save_for_interval(Aggregation::HOUR, "test", r)
                .await
                .unwrap();
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
            save_for_interval(Aggregation::HOUR, "test", r)
                .await
                .unwrap();
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
            save_for_interval(Aggregation::HOUR, "test", r)
                .await
                .unwrap();
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
