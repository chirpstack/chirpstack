use std::io::Cursor;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use prost::Message;
use redis::streams::StreamReadReply;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{debug, error, trace, warn};

use lrwn::EUI64;

use crate::config;
use crate::helpers::errors::PrintFullError;
use crate::storage::{get_async_redis_conn, redis_key};
use chirpstack_api::{api, stream};

pub async fn log_uplink_for_gateways(ufl: &stream::UplinkFrameLog) -> Result<()> {
    let conf = config::get();

    for rx_info in &ufl.rx_info {
        let gateway_id = EUI64::from_str(&rx_info.gateway_id)?;

        let ufl_copy = stream::UplinkFrameLog {
            phy_payload: ufl.phy_payload.clone(),
            tx_info: ufl.tx_info.clone(),
            rx_info: vec![rx_info.clone()],
            m_type: ufl.m_type,
            dev_addr: ufl.dev_addr.clone(),
            dev_eui: ufl.dev_eui.clone(),
            time: ufl.time.clone(),
            plaintext_f_opts: ufl.plaintext_f_opts,
            plaintext_frm_payload: ufl.plaintext_frm_payload,
        };

        let b = ufl_copy.encode_to_vec();

        // per gateway stream
        if conf.monitoring.per_gateway_frame_log_max_history > 0 {
            let key = redis_key(format!("gw:{{{}}}:stream:frame", gateway_id));

            redis::pipe()
                .atomic()
                .cmd("XADD")
                .arg(&key)
                .arg("MAXLEN")
                .arg(conf.monitoring.per_gateway_frame_log_max_history)
                .arg("*")
                .arg("up")
                .arg(&b)
                .ignore()
                .cmd("PEXPIRE")
                .arg(&key)
                .arg(conf.monitoring.per_gateway_frame_log_ttl.as_millis() as usize)
                .ignore()
                .query_async(&mut get_async_redis_conn().await?)
                .await?;
        }

        // global gateway stream
        if conf.monitoring.gateway_frame_log_max_history > 0 {
            let key = redis_key("gw:stream:frame".to_string());
            redis::cmd("XADD")
                .arg(&key)
                .arg("MAXLEN")
                .arg(conf.monitoring.gateway_frame_log_max_history)
                .arg("*")
                .arg("up")
                .arg(&b)
                .query_async(&mut get_async_redis_conn().await?)
                .await?;
        }
    }

    Ok(())
}

pub async fn log_downlink_for_gateway(dfl: &stream::DownlinkFrameLog) -> Result<()> {
    if dfl.gateway_id.is_empty() {
        return Err(anyhow!("gateway_id must be set"));
    }

    let conf = config::get();

    let b = dfl.encode_to_vec();

    // per gateway stream
    if conf.monitoring.per_gateway_frame_log_max_history > 0 {
        let key = redis_key(format!("gw:{{{}}}:stream:frame", dfl.gateway_id));
        redis::pipe()
            .atomic()
            .cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.per_gateway_frame_log_max_history)
            .arg("*")
            .arg("down")
            .arg(&b)
            .ignore()
            .cmd("PEXPIRE")
            .arg(&key)
            .arg(conf.monitoring.per_gateway_frame_log_ttl.as_millis() as usize)
            .ignore()
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    // global gateway stream
    if conf.monitoring.gateway_frame_log_max_history > 0 {
        let key = redis_key("gw:stream:frame".to_string());
        redis::cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.gateway_frame_log_max_history)
            .arg("*")
            .arg("down")
            .arg(&b)
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    Ok(())
}

pub async fn log_uplink_for_device(ufl: &stream::UplinkFrameLog) -> Result<()> {
    if ufl.dev_eui.is_empty() {
        return Err(anyhow!("dev_eui must be set"));
    }

    let conf = config::get();

    let b = ufl.encode_to_vec();

    // per device stream
    if conf.monitoring.per_device_frame_log_max_history > 0 {
        let key = redis_key(format!("device:{{{}}}:stream:frame", ufl.dev_eui));

        redis::pipe()
            .atomic()
            .cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.per_device_frame_log_max_history)
            .arg("*")
            .arg("up")
            .arg(&b)
            .ignore()
            .cmd("PEXPIRE")
            .arg(&key)
            .arg(conf.monitoring.per_device_frame_log_ttl.as_millis() as usize)
            .ignore()
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    // global device stream
    if conf.monitoring.device_frame_log_max_history > 0 {
        let key = redis_key("device:stream:frame".to_string());
        redis::cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.device_frame_log_max_history)
            .arg("*")
            .arg("up")
            .arg(&b)
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    Ok(())
}

pub async fn log_downlink_for_device(dfl: &stream::DownlinkFrameLog) -> Result<()> {
    if dfl.dev_eui.is_empty() {
        return Err(anyhow!("dev_eui must be set"));
    }

    let conf = config::get();

    let b = dfl.encode_to_vec();

    // per device stream
    if conf.monitoring.per_device_frame_log_max_history > 0 {
        let key = redis_key(format!("device:{{{}}}:stream:frame", dfl.dev_eui));

        redis::pipe()
            .atomic()
            .cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.per_device_frame_log_max_history)
            .arg("*")
            .arg("down")
            .arg(&b)
            .ignore()
            .cmd("PEXPIRE")
            .arg(&key)
            .arg(conf.monitoring.per_device_frame_log_ttl.as_millis() as usize)
            .ignore()
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    // global device stream
    if conf.monitoring.device_frame_log_max_history > 0 {
        let key = redis_key("device:stream:frame".to_string());
        redis::cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.device_frame_log_max_history)
            .arg("*")
            .arg("down")
            .arg(&b)
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    Ok(())
}

pub async fn get_frame_logs(
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
            .context("XREAD frame stream")?;

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
            trace!(key = %k, id = %stream_id, "Frame-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = stream::UplinkFrameLog::decode(&mut Cursor::new(b))?;
                let mut phy = lrwn::PhyPayload::from_slice(&pl.phy_payload)?;
                if pl.plaintext_f_opts {
                    if let Err(e) = phy.decode_f_opts_to_mac_commands() {
                        warn!(error = %e.full(), "Decode f_opts to mac-commands error");
                    }
                }
                if pl.plaintext_frm_payload {
                    if let Err(e) = phy.decode_frm_payload() {
                        warn!(error = %e.full(), "Decode frm_payload error");
                    }
                }

                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|t| prost_types::Timestamp {
                        seconds: t.seconds,
                        nanos: t.nanos,
                    }),
                    description: pl.m_type().into(),
                    body: json!({
                        "phy_payload": phy,
                        "tx_info": pl.tx_info,
                        "rx_info": pl.rx_info,
                    })
                    .to_string(),
                    properties: [
                        ("DevAddr".to_string(), pl.dev_addr),
                        ("DevEUI".to_string(), pl.dev_eui),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                channel.send(pl).await?;
            }
        }
        "down" => {
            trace!(key = %k, id = %stream_id, "frame-log received from stream");
            if let redis::Value::Data(b) = v {
                let pl = stream::DownlinkFrameLog::decode(&mut Cursor::new(b))?;
                let mut phy = lrwn::PhyPayload::from_slice(&pl.phy_payload)?;
                if pl.plaintext_f_opts {
                    if let Err(e) = phy.decode_f_opts_to_mac_commands() {
                        warn!(error = %e.full(), "Decode f_opts to mac-commands error");
                    }
                }
                if pl.plaintext_frm_payload {
                    if let Err(e) = phy.decode_frm_payload() {
                        warn!(error = %e.full(), "Decode frm_payload error");
                    }
                }

                let pl = api::LogItem {
                    id: stream_id.to_string(),
                    time: pl.time.as_ref().map(|t| prost_types::Timestamp {
                        seconds: t.seconds,
                        nanos: t.nanos,
                    }),
                    description: pl.m_type().into(),
                    body: json!({
                        "phy_payload": phy,
                        "tx_info": pl.tx_info,
                    })
                    .to_string(),
                    properties: [
                        ("DevAddr".to_string(), pl.dev_addr),
                        ("DevEUI".to_string(), pl.dev_eui),
                        ("Gateway ID".to_string(), pl.gateway_id),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                };

                channel.send(pl).await?;
            }
        }
        _ => {
            error!(key = %k, "Unexpected key in frame-log stream");
        }
    }

    Ok(())
}
