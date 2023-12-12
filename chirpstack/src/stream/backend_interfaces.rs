use anyhow::Result;
use prost::Message;
use tokio::sync::mpsc::{self, Sender};
use tracing::error;

use crate::config;
use crate::storage::{get_async_redis_conn, redis_key};
use chirpstack_api::stream;

pub async fn get_log_sender() -> Option<Sender<stream::BackendInterfacesRequest>> {
    let conf = config::get();
    if conf.monitoring.backend_interfaces_log_max_history == 0 {
        return None;
    }

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        while let Some(pl) = rx.recv().await {
            tokio::spawn(async move {
                if let Err(e) = log_request(pl).await {
                    error!(error = %e, "Log request error");
                }
            });
        }
    });

    Some(tx)
}

pub async fn log_request(pl: stream::BackendInterfacesRequest) -> Result<()> {
    let conf = config::get();

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
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    Ok(())
}
