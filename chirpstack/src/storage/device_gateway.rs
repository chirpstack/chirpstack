use std::io::Cursor;

use anyhow::{Context, Result};
use prost::Message;
use tracing::info;

use super::{error::Error, get_async_redis_conn, redis_key};
use crate::config;
use chirpstack_api::internal;
use lrwn::EUI64;

pub async fn save_rx_info(rx_info: &internal::DeviceGatewayRxInfo) -> Result<()> {
    let dev_eui = EUI64::from_slice(&rx_info.dev_eui)?;
    let conf = config::get();
    let key = redis_key(format!("device:{{{}}}:gwrx", dev_eui));
    let ttl = conf.network.device_session_ttl.as_millis() as usize;
    let b = rx_info.encode_to_vec();

    redis::cmd("PSETEX")
        .arg(key)
        .arg(ttl)
        .arg(b)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(dev_eui = %dev_eui, "Gateway rx-info saved");
    Ok(())
}

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

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

    #[tokio::test]
    async fn test_rx_info() {
        let _guard = test::prepare().await;
        let rx_info = internal::DeviceGatewayRxInfo {
            dev_eui: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            ..Default::default()
        };
        let dev_eui = EUI64::from_slice(&rx_info.dev_eui).unwrap();

        // save
        save_rx_info(&rx_info).await.unwrap();

        // get
        let res = get_rx_info(&dev_eui).await.unwrap();
        assert_eq!(rx_info, res);
    }
}
