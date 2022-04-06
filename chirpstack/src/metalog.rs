use anyhow::Result;
use prost::Message;
use tokio::task;

use crate::config;
use crate::storage::{get_redis_conn, redis_key};
use chirpstack_api::meta;

pub async fn log_uplink(up: &meta::UplinkMeta) -> Result<()> {
    task::spawn_blocking({
        let up = up.clone();
        move || -> Result<()> {
            let conf = config::get();
            let mut c = get_redis_conn()?;

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
                    .query(&mut *c)?;
            }

            Ok(())
        }
    })
    .await?
}

pub async fn log_downlink(down: &meta::DownlinkMeta) -> Result<()> {
    task::spawn_blocking({
        let down = down.clone();
        move || -> Result<()> {
            let conf = config::get();
            let mut c = get_redis_conn()?;

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
                    .query(&mut *c)?;
            }

            Ok(())
        }
    })
    .await?
}
