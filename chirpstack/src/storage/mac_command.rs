use anyhow::Result;
use tracing::info;

use super::{get_async_redis_conn, redis_key};
use crate::config;
use lrwn::EUI64;

pub async fn set_pending(dev_eui: &EUI64, cid: lrwn::CID, set: &lrwn::MACCommandSet) -> Result<()> {
    let conf = config::get();

    let key = redis_key(format!("device:{}:mac:pending:{}", dev_eui, cid.to_u8()));
    let ttl = conf.network.device_session_ttl.as_millis() as usize;
    let b = set.to_vec()?;

    redis::cmd("PSETEX")
        .arg(key)
        .arg(ttl)
        .arg(b)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(dev_eui = %dev_eui, cid = %cid, "Pending mac-command block set");
    Ok(())
}

pub async fn get_pending(dev_eui: &EUI64, cid: lrwn::CID) -> Result<Option<lrwn::MACCommandSet>> {
    let key = redis_key(format!("device:{}:mac:pending:{}", dev_eui, cid.to_u8()));
    let b: Vec<u8> = redis::cmd("GET")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    let out = if !b.is_empty() {
        let mut mac = lrwn::MACCommandSet::from_slice(&b);

        // Per definition, the uplink flag is set to false as this function is intended to retrieve
        // pending mac-commands that were previously sent to the device.
        mac.decode_from_raw(false)?;

        Some(mac)
    } else {
        None
    };

    Ok(out)
}

pub async fn delete_pending(dev_eui: &EUI64, cid: lrwn::CID) -> Result<()> {
    let key = redis_key(format!("device:{}:mac:pending:{}", dev_eui, cid.to_u8()));

    redis::cmd("DEL")
        .arg(key)
        .query_async(&mut get_async_redis_conn().await?)
        .await?;

    info!(dev_eui = %dev_eui, cid = %cid, "Pending mac-command block deleted");
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

    #[tokio::test]
    async fn test_mac_command() {
        let _guard = test::prepare().await;

        let dev_eui = EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]);
        let mac = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::DevStatusReq]);

        // set
        set_pending(&dev_eui, lrwn::CID::DevStatusReq, &mac)
            .await
            .unwrap();

        // get
        let mac_get = get_pending(&dev_eui, lrwn::CID::DevStatusReq)
            .await
            .unwrap();
        assert_eq!(mac, mac_get.unwrap());

        // delete
        delete_pending(&dev_eui, lrwn::CID::DevStatusReq)
            .await
            .unwrap();
        let resp = get_pending(&dev_eui, lrwn::CID::DevStatusReq)
            .await
            .unwrap();
        assert_eq!(true, resp.is_none());
    }
}
