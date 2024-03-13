use std::io::Cursor;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use prost::Message;
use tracing::{info, trace};

use crate::storage::{get_async_redis_conn, redis_key};
use chirpstack_api::{gw, internal};
use lrwn::EUI64;

pub async fn get_geoloc_buffer(
    dev_eui: &EUI64,
    ttl: Duration,
) -> Result<Vec<Vec<gw::UplinkRxInfo>>> {
    if ttl == Duration::zero() {
        return Ok(Vec::new());
    }

    trace!(dev_eui = %dev_eui, "Getting geolocation buffer");
    let key = redis_key(format!("device:{{{}}}:loracloud:buffer", dev_eui));

    let b: Vec<u8> = redis::cmd("GET")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("Get geolocation buffer")?;
    if b.is_empty() {
        return Ok(Vec::new());
    }

    let buffer = internal::LoraCloudGeolocBuffer::decode(&mut Cursor::new(b))
        .context("Decode geolocation buffer")?;

    let mut out: Vec<Vec<gw::UplinkRxInfo>> = Vec::new();

    for uplink in &buffer.uplinks {
        let rx_info: Vec<gw::UplinkRxInfo> = uplink
            .rx_info
            .iter()
            .filter(|&rx_info| {
                let ts: DateTime<Utc> = match &rx_info.gw_time {
                    None => {
                        return false;
                    }
                    Some(v) => match v.clone().try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return false;
                        }
                    },
                };

                // The interval between now and then must be smaller than the TTL
                (ts - Utc::now()) < ttl
            })
            .cloned()
            .collect();

        if rx_info.len() > 3 {
            out.push(rx_info);
        }
    }

    Ok(out)
}

pub async fn save_geoloc_buffer(
    dev_eui: &EUI64,
    ttl: &Duration,
    items: &[Vec<gw::UplinkRxInfo>],
) -> Result<()> {
    if *ttl == Duration::zero() || items.is_empty() {
        return Ok(());
    }

    trace!(dev_eui = %dev_eui, "Saving geolocation buffer");
    let key = redis_key(format!("device:{{{}}}:loracloud:buffer", dev_eui));

    let buffer = internal::LoraCloudGeolocBuffer {
        uplinks: items
            .iter()
            .cloned()
            .map(|rx_info| internal::LoraCloudGeolocBufferUplink { rx_info })
            .collect(),
    };
    let b = buffer.encode_to_vec();

    redis::cmd("PSETEX")
        .arg(key)
        .arg(ttl.num_milliseconds())
        .arg(b)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(dev_eui = %dev_eui, "Geolocation buffer saved");

    Ok(())
}
