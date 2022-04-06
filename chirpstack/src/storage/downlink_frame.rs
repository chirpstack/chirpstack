use std::io::Cursor;

use anyhow::Result;
use prost::Message;
use tokio::task;
use tracing::info;
use uuid::Uuid;

use super::{get_redis_conn, redis_key};
use chirpstack_api::internal;

pub async fn save(df: &internal::DownlinkFrame) -> Result<()> {
    let id = Uuid::from_slice(&df.downlink_frame.as_ref().unwrap().downlink_id)?;
    task::spawn_blocking({
        let df = df.clone();
        move || -> Result<()> {
            let b = df.encode_to_vec();
            let key = redis_key(format!("frame:{}", id));
            let mut c = get_redis_conn()?;
            redis::cmd("SETEX").arg(key).arg(30).arg(b).query(&mut *c)?;
            Ok(())
        }
    })
    .await??;
    info!(downlink_id = %id, "Downlink-frame saved");
    Ok(())
}

pub async fn get(id: &Uuid) -> Result<internal::DownlinkFrame> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<internal::DownlinkFrame> {
            let mut c = get_redis_conn()?;
            let key = redis_key(format!("frame:{}", id));
            let v: Vec<u8> = redis::cmd("GET").arg(key).query(&mut *c)?;
            let df = internal::DownlinkFrame::decode(&mut Cursor::new(v))?;
            Ok(df)
        }
    })
    .await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;
    use chirpstack_api::gw;

    #[tokio::test]
    async fn test_downlink_frame() {
        let _guard = test::prepare().await;
        let id = Uuid::new_v4();
        let df = internal::DownlinkFrame {
            downlink_frame: Some(gw::DownlinkFrame {
                downlink_id: id.as_bytes().to_vec(),
                ..Default::default()
            }),
            ..Default::default()
        };

        save(&df).await.unwrap();
        let df_get = get(&id).await.unwrap();
        assert_eq!(df, df_get);
    }
}
