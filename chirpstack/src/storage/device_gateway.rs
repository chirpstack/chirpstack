use std::io::Cursor;

use anyhow::{Context, Result};
use prost::Message;

use super::{error::Error, get_async_redis_conn, redis_key};
use chirpstack_api::internal;
use lrwn::EUI64;

pub async fn get_rx_info(dev_eui: &EUI64) -> Result<internal::DeviceGatewayRxInfo, Error> {
    let key = redis_key(format!("device:{{{}}}:gwrx", dev_eui));

    let b: Vec<u8> = redis::cmd("GET")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("Get rx-info")?;
    if b.is_empty() {
        return Err(Error::NotFound(dev_eui.to_string()));
    }

    Ok(internal::DeviceGatewayRxInfo::decode(&mut Cursor::new(b)).context("Decode rx-info")?)
}

pub async fn get_rx_info_for_dev_euis(
    dev_euis: &[EUI64],
) -> Result<Vec<internal::DeviceGatewayRxInfo>, Error> {
    if dev_euis.is_empty() {
        return Ok(Vec::new());
    }

    let mut keys: Vec<String> = Vec::new();
    for dev_eui in dev_euis {
        keys.push(redis_key(format!("device:{{{}}}:gwrx", dev_eui)));
    }

    let bb: Vec<Vec<u8>> = redis::cmd("MGET")
        .arg(keys)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("MGET")?;
    let mut out: Vec<internal::DeviceGatewayRxInfo> = Vec::new();
    for b in bb {
        if b.is_empty() {
            continue;
        }

        out.push(
            internal::DeviceGatewayRxInfo::decode(&mut Cursor::new(b)).context("Decode rx-info")?,
        );
    }
    Ok(out)
}
