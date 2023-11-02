use anyhow::Result;
use prost::Message;
use tokio::task;

use crate::config;
use crate::storage::{get_redis_conn, redis_key};
use chirpstack_api::stream;

pub async fn log_request(pl: stream::BackendInterfacesRequest) -> Result<()> {
    task::spawn_blocking({
        move || -> Result<()> {
            let conf = config::get();
            let mut c = get_redis_conn()?;

            if conf.monitoring.backend_interfaces_log_max_history == 0 {
                return Ok(());
            }

            let key = redis_key("backend_interfaces:stream:request".to_string());
            let b = pl.encode_to_vec();
            redis::cmd("XADD")
                .arg(&key)
                .arg("MAXLEN")
                .arg(conf.monitoring.backend_interfaces_log_max_history)
                .arg("*")
                .arg("request")
                .arg(&b)
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await?
}
