use anyhow::Result;
use prost::Message;
use tokio::task;

use crate::config;
use crate::storage::{get_redis_conn, redis_key};
use chirpstack_api::api;

pub async fn log_request(pl: &api::RequestLog) -> Result<()> {
    task::spawn_blocking({
        let pl = pl.clone();

        move || -> Result<()> {
            let conf = config::get();
            let mut c = get_redis_conn()?;

            if conf.monitoring.api_request_log_max_history == 0 {
                return Ok(());
            }

            let key = redis_key("api:stream:request".to_string());
            let b = pl.encode_to_vec();
            redis::cmd("XADD")
                .arg(&key)
                .arg("MAXLEN")
                .arg(conf.monitoring.api_request_log_max_history)
                .arg("*")
                .arg("request")
                .arg(&b)
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use redis::streams::StreamReadReply;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_log_request() {
        let _guard = test::prepare().await;

        let pl = api::RequestLog {
            service: "ap.Foo".to_string(),
            method: "bar".to_string(),
            metadata: [("user_id".to_string(), "foo_user".to_string())]
                .iter()
                .cloned()
                .collect(),
        };
        log_request(&pl).await.unwrap();

        let mut c = get_redis_conn().unwrap();
        let key = redis_key("api:stream:request".to_string());
        let srr: StreamReadReply = redis::cmd("XREAD")
            .arg("COUNT")
            .arg(1 as usize)
            .arg("STREAMS")
            .arg(&key)
            .arg("0")
            .query(&mut *c)
            .unwrap();

        assert_eq!(1, srr.keys.len());
        assert_eq!(1, srr.keys[0].ids.len());

        if let Some(redis::Value::Data(b)) = srr.keys[0].ids[0].map.get("request") {
            let pl_recv = api::RequestLog::decode(&mut Cursor::new(b)).unwrap();
            assert_eq!(pl, pl_recv);
        } else {
            panic!("No request log");
        }
    }
}
