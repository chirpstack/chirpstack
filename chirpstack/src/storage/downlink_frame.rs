use std::io::Cursor;

use anyhow::Result;
use prost::Message;
use tracing::info;

use super::{error::Error, get_async_redis_conn, redis_key};
use chirpstack_api::internal;

pub async fn save(df: &internal::DownlinkFrame) -> Result<()> {
    let b = df.encode_to_vec();
    let key = redis_key(format!("frame:{}", df.downlink_id));

    redis::cmd("SETEX")
        .arg(key)
        .arg(30)
        .arg(b)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(downlink_id = df.downlink_id, "Downlink-frame saved");
    Ok(())
}

pub async fn get(id: u32) -> Result<internal::DownlinkFrame, Error> {
    let key = redis_key(format!("frame:{}", id));
    let v: Vec<u8> = redis::cmd("GET")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;
    if v.is_empty() {
        return Err(Error::NotFound(format!("{}", id)));
    }
    let df = internal::DownlinkFrame::decode(&mut Cursor::new(v))?;
    Ok(df)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;
    use chirpstack_api::gw;

    #[tokio::test]
    async fn test_downlink_frame() {
        let _guard = test::prepare().await;
        let df = internal::DownlinkFrame {
            downlink_id: 12345,
            downlink_frame: Some(gw::DownlinkFrame {
                ..Default::default()
            }),
            ..Default::default()
        };

        save(&df).await.unwrap();
        let df_get = get(12345).await.unwrap();
        assert_eq!(df, df_get);
    }
}
