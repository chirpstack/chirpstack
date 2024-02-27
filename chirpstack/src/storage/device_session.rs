use std::io::Cursor;

use anyhow::{Context, Result};
use prost::Message;

use super::error::Error;
use super::{get_async_redis_conn, redis_key};
use chirpstack_api::internal;
use lrwn::EUI64;

pub async fn get(dev_eui: &EUI64) -> Result<internal::DeviceSession, Error> {
    let key = redis_key(format!("device:{{{}}}:ds", dev_eui));

    let v: Vec<u8> = redis::cmd("GET")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await
        .context("Get device-session")?;
    if v.is_empty() {
        return Err(Error::NotFound(dev_eui.to_string()));
    }
    let ds =
        internal::DeviceSession::decode(&mut Cursor::new(v)).context("Decode device-session")?;
    Ok(ds)
}
