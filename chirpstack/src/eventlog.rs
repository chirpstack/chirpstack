use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Context, Result};
use prost::Message;
use redis::streams::StreamReadReply;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{debug, error, trace};

use crate::config;
use crate::storage::{get_redis_conn, redis_key};
use chirpstack_api::{api, integration};

#[allow(clippy::enum_variant_names)]

pub async fn log_event_for_device(typ: &str, dev_eui: &str, b: &[u8]) -> Result<()> {
    task::spawn_blocking({
        let typ = typ.to_string();
        let dev_eui = dev_eui.to_string();
        let b = b.to_vec();
        move || -> Result<()> {
            let conf = config::get();
            let mut c = get_redis_conn()?;

            // per device stream
            if conf.monitoring.per_device_event_log_max_history > 0 {
                let key = redis_key(format!("device:{{{}}}:stream:event", dev_eui));
                c.new_pipeline()
                    .atomic()
                    .cmd("XADD")
                    .arg(&key)
                    .arg("MAXLEN")
                    .arg(conf.monitoring.per_device_event_log_max_history)
                    .arg("*")
                    .arg(&typ)
                    .arg(&b)
                    .ignore()
                    .cmd("PEXPIRE")
                    .arg(&key)
                    .arg(conf.monitoring.per_device_event_log_ttl.as_millis() as usize)
                    .ignore()
                    .query(&mut c)?;
            }

            // global device stream
            if conf.monitoring.device_event_log_max_history > 0 {
                let key = redis_key("device:stream:event".to_string());
                redis::cmd("XADD")
                    .arg(&key)
                    .arg("MAXLEN")
                    .arg(conf.monitoring.device_event_log_max_history)
                    .arg("*")
                    .arg(&typ)
                    .arg(&b)
                    .query(&mut *c)?;
            }

            Ok(())
        }
    })
    .await?
}

pub async fn get_event_logs(
    key: String,
    count: usize,
    channel: mpsc::Sender<api::LogItem>,
) -> Result<()> {
    task::spawn_blocking({
        let key = key.clone();
        let channel = channel.clone();

        move || -> Result<()> {
            let mut last_id = "0".to_string();
            let mut c = get_redis_conn()?;

            loop {
                if channel.is_closed() {
                    debug!("Channel has been closed, returning");
                    return Ok(());
                }

                let srr: StreamReadReply = redis::cmd("XREAD")
                    .arg("COUNT")
                    .arg(count)
                    .arg("STREAMS")
                    .arg(&key)
                    .arg(&last_id)
                    .query(&mut *c)
                    .context("XREAD event stream")?;

                for stream_key in &srr.keys {
                    for stream_id in &stream_key.ids {
                        last_id = stream_id.id.clone();
                        for (k, v) in &stream_id.map {
                            let res = || -> Result<()> {
                                match k.as_ref() {
                                    "up" => {
                                        trace!(key = %k, id = %last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::UplinkEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [
                                                    ("DR".to_string(), pl.dr.to_string()),
                                                    ("FPort".to_string(), pl.f_port.to_string()),
                                                    ("FCnt".to_string(), pl.f_cnt.to_string()),
                                                    ("Data".to_string(), hex::encode(&pl.data)),
                                                ]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "join" => {
                                        trace!(key = %k, id = %last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::JoinEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [("DevAddr".to_string(), pl.dev_addr)]
                                                    .iter()
                                                    .cloned()
                                                    .collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "ack" => {
                                        trace!(key = %k, id = %last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::AckEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [].iter().cloned().collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "txack" => {
                                        trace!(key = %k, id = %last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::TxAckEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [].iter().cloned().collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "status" => {
                                        trace!(key = %k, id = %last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::StatusEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [
                                                    ("Margin".into(), format!("{}", pl.margin)),
                                                    (
                                                        "Battery level".into(),
                                                        format!("{:.0}%", pl.battery_level),
                                                    ),
                                                    (
                                                        "Battery level unavailable".into(),
                                                        format!("{}", pl.battery_level_unavailable),
                                                    ),
                                                    (
                                                        "External power source".into(),
                                                        format!("{}", pl.external_power_source),
                                                    ),
                                                ]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "log" => {
                                        trace!(key = %k, id =%last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::LogEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [
                                                    ("Level".into(), pl.level().into()),
                                                    ("Code".into(), pl.code().into()),
                                                ]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "location" => {
                                        trace!(key = %k, id=%last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl = integration::LocationEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                    seconds: v.seconds,
                                                    nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [].iter().cloned().collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    "integration" => {
                                        trace!(key = %k, id=%last_id, "Event-log received from stream");
                                        if let redis::Value::Data(b) = v {
                                            let pl =
                                                integration::IntegrationEvent::decode(&mut Cursor::new(b))?;
                                            let pl = api::LogItem {
                                                id: stream_id.id.clone(),
                                                time: pl.time.as_ref().map(|v| prost_types::Timestamp{
                                                        seconds: v.seconds,
                                                        nanos: v.nanos,
                                                }),
                                                description: k.clone(),
                                                body: serde_json::to_string(&pl)?,
                                                properties: [
                                                    ("Integration".into(), pl.integration_name.clone()),
                                                    ("Event".into(), pl.event_type.clone()),
                                                ]
                                                .iter()
                                                .cloned()
                                                .collect(),
                                            };

                                            channel.blocking_send(pl)?;
                                        }
                                    }
                                    _ => {
                                        error!(key = %k, "Unexpected key in in event-log stream");
                                    }

                                }

                                Ok(())
                            }();

                            if let Err(e) = res {
                                // Return in case of channel error, in any other case we just log
                                // the error.
                                if e.downcast_ref::<mpsc::error::SendError<api::LogItem>>().is_some() {
                                    return Err(e);
                                }

                                error!(key = %k, error = %e, "Parsing frame-log error");
                            }
                        }
                    }
                }

                sleep(Duration::from_secs(1));
            }
        }
    })
    .await?
}
