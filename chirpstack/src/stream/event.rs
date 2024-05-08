use std::io::Cursor;
use std::time::Duration;

use anyhow::{Context, Result};
use prost::Message;
use redis::streams::StreamReadReply;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, trace};

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{get_async_redis_conn, redis_key};
use chirpstack_api::{api, integration};

#[allow(clippy::enum_variant_names)]

pub async fn log_event_for_device(typ: &str, dev_eui: &str, b: &[u8]) -> Result<()> {
    let conf = config::get();

    // per device stream
    if conf.monitoring.per_device_event_log_max_history > 0 {
        let key = redis_key(format!("device:{{{}}}:stream:event", dev_eui));
        redis::pipe()
            .atomic()
            .cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.per_device_event_log_max_history)
            .arg("*")
            .arg(typ)
            .arg(b)
            .ignore()
            .cmd("PEXPIRE")
            .arg(&key)
            .arg(conf.monitoring.per_device_event_log_ttl.as_millis() as usize)
            .ignore()
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    // global device stream
    if conf.monitoring.device_event_log_max_history > 0 {
        let key = redis_key("device:stream:event".to_string());
        redis::cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.device_event_log_max_history)
            .arg("*")
            .arg(typ)
            .arg(b)
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    Ok(())
}

pub async fn get_event_logs(
    key: String,
    count: usize,
    channel: mpsc::Sender<api::LogItem>,
) -> Result<()> {
    let mut last_id = "0".to_string();

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
            .query_async(&mut get_async_redis_conn().await?)
            .await
            .context("XREAD event stream")?;

        for stream_key in &srr.keys {
            for stream_id in &stream_key.ids {
                last_id.clone_from(&stream_id.id);
                for (k, v) in &stream_id.map {
                    let res = handle_stream(&last_id, &channel, k, v).await;

                    if let Err(e) = res {
                        // Return in case of channel error, in any other case we just log
                        // the error.
                        if e.downcast_ref::<mpsc::error::SendError<api::LogItem>>()
                            .is_some()
                        {
                            return Err(e);
                        }

                        error!(key = %k, error = %e.full(), "Parsing frame-log error");
                    }
                }
            }
        }

        // If we use xread with block=0, the connection can't be used by other requests. Now we
        // check every 1 second if there are new messages, which should be sufficient.
        sleep(Duration::from_secs(1)).await;
    }
}

async fn handle_stream(
    stream_id: &str,
    channel: &mpsc::Sender<api::LogItem>,
    k: &str,
    v: &redis::Value,
) -> Result<()> {
    match k {
        "up" => {
            trace!(key = %k, id = %stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::UplinkEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
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

                channel.send(pl).await?;
            }
        }
        "join" => {
            trace!(key = %k, id = %stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::JoinEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [("DevAddr".to_string(), pl.dev_addr)]
                        .iter()
                        .cloned()
                        .collect(),
                };

                channel.send(pl).await?;
            }
        }
        "ack" => {
            trace!(key = %k, id = %stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::AckEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [].iter().cloned().collect(),
                };

                channel.send(pl).await?;
            }
        }
        "txack" => {
            trace!(key = %k, id = %stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::TxAckEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [].iter().cloned().collect(),
                };

                channel.send(pl).await?;
            }
        }
        "status" => {
            trace!(key = %k, id = %stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::StatusEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [
                        ("Margin".into(), format!("{}", pl.margin)),
                        ("Battery level".into(), format!("{:.0}%", pl.battery_level)),
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

                channel.send(pl).await?;
            }
        }
        "log" => {
            trace!(key = %k, id =%stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::LogEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [
                        ("Level".into(), pl.level().into()),
                        ("Code".into(), pl.code().into()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                channel.send(pl).await?;
            }
        }
        "location" => {
            trace!(key = %k, id=%stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::LocationEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [].iter().cloned().collect(),
                };

                channel.send(pl).await?;
            }
        }
        "integration" => {
            trace!(key = %k, id=%stream_id, "Event-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = integration::IntegrationEvent::decode(&mut Cursor::new(b))?;
                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|v| prost_types::Timestamp {
                        seconds: v.seconds,
                        nanos: v.nanos,
                    }),
                    description: k.to_string(),
                    body: serde_json::to_string(&pl)?,
                    properties: [
                        ("Integration".into(), pl.integration_name.clone()),
                        ("Event".into(), pl.event_type.clone()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                channel.send(pl).await?;
            }
        }
        _ => {
            error!(key = %k, "Unexpected key in in event-log stream");
        }
    }

    Ok(())
}
