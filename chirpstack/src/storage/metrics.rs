use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use anyhow::{Context, Result};
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

#[derive(Debug, PartialEq, Clone)]
pub struct Record {
    pub time: DateTime<Local>,
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
                .ymd(record.time.year(), record.time.month(), record.time.day())
                .and_hms(record.time.hour(), 0, 0),
            Aggregation::DAY => Local
                .ymd(record.time.year(), record.time.month(), record.time.day())
                .and_hms(0, 0, 0),
            Aggregation::MONTH => Local
                .ymd(record.time.year(), record.time.month(), 1)
                .and_hms(0, 0, 0),
        };

        move || -> Result<()> {
            let mut c = get_redis_conn()?;
            let key = get_key(&name, a, ts);
            let mut pipe = redis::pipe();
            pipe.atomic();

            for (k, v) in &record.metrics {
                // Passing a reference to hincr will return a runtime error.
                let k = k.clone();
                let v = *v;
                pipe.cmd("HINCRBYFLOAT").arg(&key).arg(k).arg(v).ignore();
            }

            pipe.cmd("PEXPIRE")
                .arg(&key)
                .arg(ttl.as_millis() as usize)
                .ignore()
                .query(&mut *c)
                .context("Execute metrics pipeline")?;

            Ok(())
        }
    })
    .await??;
    info!(name = %name, aggregation = %a, "Metrics saved");
    Ok(())
}

pub async fn get(
    name: &str,
    a: Aggregation,
    start: DateTime<Local>,
    end: DateTime<Local>,
) -> Result<Vec<Record>> {
    let mut keys: Vec<String> = Vec::new();
    let mut timestamps: Vec<DateTime<Local>> = Vec::new();

    match a {
        Aggregation::HOUR => {
            let mut ts =
                Local
                    .ymd(start.year(), start.month(), start.day())
                    .and_hms(start.hour(), 0, 0);
            let end = Local
                .ymd(end.year(), end.month(), end.day())
                .and_hms(end.hour(), 0, 0);

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts = ts + ChronoDuration::hours(1);
            }
        }
        Aggregation::DAY => {
            let mut ts = Local
                .ymd(start.year(), start.month(), start.day())
                .and_hms(0, 0, 0);
            let end = Local
                .ymd(end.year(), end.month(), end.day())
                .and_hms(0, 0, 0);

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                // Make sure that the timestamp stays at midnight at daylight saving change.
                ts = (ts + ChronoDuration::days(1)).date().and_hms(0, 0, 0);
            }
        }
        Aggregation::MONTH => {
            let mut ts = Local.ymd(start.year(), start.month(), 1).and_hms(0, 0, 0);
            let end = Local.ymd(end.year(), end.month(), 1).and_hms(0, 0, 0);

            while ts.le(&end) {
                timestamps.push(ts);
                keys.push(get_key(name, a, ts));
                ts = if ts.month() == 12 {
                    Local.ymd(ts.year() + 1, 1, 1).and_hms(0, 0, 0)
                } else {
                    Local.ymd(ts.year(), ts.month() + 1, 1).and_hms(0, 0, 0)
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
            let mut pipe = redis::pipe();

            for k in &keys {
                pipe.cmd("HGETALL").arg(k);
            }

            let res: Vec<HashMap<String, f64>> = pipe.query(&mut *c)?;
            let mut out: Vec<Record> = Vec::new();

            for (i, r) in res.iter().enumerate() {
                out.push(Record {
                    time: timestamps[i],
                    metrics: r.clone(),
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
                time: Local.ymd(2018, 1, 1).and_hms(1, 1, 0),
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.ymd(2018, 1, 1).and_hms(1, 2, 0),
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.ymd(2018, 1, 1).and_hms(2, 1, 0),
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
            Aggregation::HOUR,
            Local.ymd(2018, 1, 1).and_hms(1, 0, 0),
            Local.ymd(2018, 1, 1).and_hms(2, 0, 0),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.ymd(2018, 1, 1).and_hms(1, 0, 0),
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.ymd(2018, 1, 1).and_hms(2, 0, 0),
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
                time: Local.ymd(2018, 1, 1).and_hms(1, 0, 0),
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.ymd(2018, 1, 1).and_hms(2, 0, 0),
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.ymd(2018, 1, 2).and_hms(1, 0, 0),
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
            Aggregation::DAY,
            Local.ymd(2018, 1, 1).and_hms(1, 0, 0),
            Local.ymd(2018, 1, 2).and_hms(1, 0, 0),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.ymd(2018, 1, 1).and_hms(0, 0, 0),
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.ymd(2018, 1, 2).and_hms(0, 0, 0),
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
                time: Local.ymd(2018, 1, 1).and_hms(0, 0, 0),
                metrics: [("foo".into(), 1f64), ("bar".into(), 2f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.ymd(2018, 1, 2).and_hms(0, 0, 0),
                metrics: [("foo".into(), 3f64), ("bar".into(), 4f64)]
                    .iter()
                    .cloned()
                    .collect(),
            },
            Record {
                time: Local.ymd(2018, 2, 1).and_hms(0, 0, 0),
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
            Aggregation::MONTH,
            Local.ymd(2018, 1, 1).and_hms(0, 0, 0),
            Local.ymd(2018, 2, 1).and_hms(0, 0, 0),
        )
        .await
        .unwrap();

        assert_eq!(
            vec![
                Record {
                    time: Local.ymd(2018, 1, 1).and_hms(0, 0, 0),
                    metrics: [("foo".into(), 4f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                },
                Record {
                    time: Local.ymd(2018, 2, 1).and_hms(0, 0, 0),
                    metrics: [("foo".into(), 5f64), ("bar".into(), 6f64)]
                        .iter()
                        .cloned()
                        .collect(),
                }
            ],
            resp
        );
    }
}
