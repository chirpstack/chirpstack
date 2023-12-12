use anyhow::Result;
use prost::Message;

use crate::config;
use crate::storage::{get_async_redis_conn, redis_key};
use chirpstack_api::stream;

pub async fn log_uplink(up: &stream::UplinkMeta) -> Result<()> {
    let conf = config::get();

    if conf.monitoring.meta_log_max_history > 0 {
        let key = redis_key("stream:meta".to_string());
        let b = up.encode_to_vec();
        redis::cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.meta_log_max_history)
            .arg("*")
            .arg("up")
            .arg(&b)
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    Ok(())
}

pub async fn log_downlink(down: &stream::DownlinkMeta) -> Result<()> {
    let conf = config::get();

    if conf.monitoring.meta_log_max_history > 0 {
        let key = redis_key("stream:meta".to_string());
        let b = down.encode_to_vec();

        redis::cmd("XADD")
            .arg(&key)
            .arg("MAXLEN")
            .arg(conf.monitoring.meta_log_max_history)
            .arg("*")
            .arg("down")
            .arg(&b)
            .query_async(&mut get_async_redis_conn().await?)
            .await?;
    }

    Ok(())
}
