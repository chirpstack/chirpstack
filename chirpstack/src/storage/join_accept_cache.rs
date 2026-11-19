//! Short-lived cache of the last JoinAccept sent to a device.
//!
//! A join-request from a TS011 relay-armed end-device can reach the network
//! server twice: directly from a gateway and, 0.7–3.7 s later, wrapped in the
//! relay's ForwardUplinkReq (or the other way around). The first copy consumes
//! the DevNonce; this cache lets the second copy be recognised as the same
//! request and answered with the identical JoinAccept on its own path. The
//! accept is re-sent at most once: the entry is removed after that re-send, so
//! a third copy (or a replayed request) gets the normal InvalidDevNonce
//! handling. The TTL only needs to cover the relay forwarding delay — a copy
//! arriving later is an unrelated replay and is treated the same way.
//!
//! The re-send's downlink flow appends the second path's gateway to the
//! in-memory `gateway_rx_info_history` only; it is not persisted (there is no
//! `update_device` on this path), which is sufficient because downlink gateway
//! selection uses only the last uplink.

use std::io::Cursor;

use anyhow::Result;
use prost::Message;
use tracing::info;

use super::{get_async_redis_conn, redis_key};
use chirpstack_api::internal;
use lrwn::EUI64;

pub const JOIN_ACCEPT_DUP_TTL_SECS: u64 = 10;

fn key(dev_eui: &EUI64) -> String {
    redis_key(format!("join_accept_dup:{}", dev_eui))
}

pub async fn save(dev_eui: &EUI64, c: &internal::CachedJoinAccept) -> Result<()> {
    let b = c.encode_to_vec();

    () = redis::cmd("SETEX")
        .arg(key(dev_eui))
        .arg(JOIN_ACCEPT_DUP_TTL_SECS)
        .arg(b)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(dev_eui = %dev_eui, dev_nonce = c.dev_nonce, "Join-accept cached for second-path duplicates");
    Ok(())
}

pub async fn get(dev_eui: &EUI64) -> Result<Option<internal::CachedJoinAccept>> {
    let v: Vec<u8> = redis::cmd("GET")
        .arg(key(dev_eui))
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    if v.is_empty() {
        // No key: treat as a miss.
        return Ok(None);
    }

    Ok(Some(internal::CachedJoinAccept::decode(&mut Cursor::new(
        v,
    ))?))
}

pub async fn delete(dev_eui: &EUI64) -> Result<()> {
    () = redis::cmd("DEL")
        .arg(key(dev_eui))
        .query_async(&mut get_async_redis_conn().await?)
        .await?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

    #[tokio::test]
    async fn test_join_accept_cache() {
        let _guard = test::prepare().await;
        let dev_eui = EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]);

        // Empty before save.
        assert_eq!(None, get(&dev_eui).await.unwrap());

        let c = internal::CachedJoinAccept {
            join_eui: EUI64::from_be_bytes([8, 7, 6, 5, 4, 3, 2, 1])
                .to_be_bytes()
                .to_vec(),
            dev_nonce: 0x0102,
            phy_payload: vec![0x20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        };
        save(&dev_eui, &c).await.unwrap();
        assert_eq!(Some(c.clone()), get(&dev_eui).await.unwrap());

        // A newer save replaces the old value.
        let c2 = internal::CachedJoinAccept {
            dev_nonce: 0x0103,
            ..c.clone()
        };
        save(&dev_eui, &c2).await.unwrap();
        assert_eq!(Some(c2), get(&dev_eui).await.unwrap());

        // Delete empties it.
        delete(&dev_eui).await.unwrap();
        assert_eq!(None, get(&dev_eui).await.unwrap());
    }
}
