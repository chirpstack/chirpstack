use std::io::Cursor;
use std::str::FromStr;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use prost::Message;
use tokio::task;
use tracing::{debug, info};
use uuid::Uuid;

use super::error::Error;
use super::{get_redis_conn, redis_key};
use crate::config;
use chirpstack_api::internal;
use lrwn::{AES128Key, DevAddr, EUI64};

pub async fn save(ds: &internal::PassiveRoamingDeviceSession) -> Result<()> {
    let sess_id = Uuid::from_slice(&ds.session_id)?;
    let dev_addr = DevAddr::from_slice(&ds.dev_addr)?;
    let dev_eui = if ds.dev_eui.is_empty() {
        EUI64::default()
    } else {
        EUI64::from_slice(&ds.dev_eui)?
    };

    let lifetime: DateTime<Utc> = match ds.lifetime.clone() {
        Some(v) => v.try_into().map_err(anyhow::Error::msg)?,
        None => {
            debug!("Not saving passive-roaming device-session, no passive-roaming lifetime set");
            return Ok(());
        }
    };

    let lifetime = lifetime - Utc::now();
    if lifetime <= Duration::seconds(0) {
        debug!("Not saving passive-roaming device-session, lifetime of passive-roaming session expired");
        return Ok(());
    }

    task::spawn_blocking({
        let ds = ds.clone();
        move || -> Result<()> {
            let conf = config::get();

            let dev_addr_key = redis_key(format!("pr:devaddr:{{{}}}", dev_addr));
            let dev_eui_key = redis_key(format!("pr:dev:{{{}}}", dev_eui));
            let sess_key = redis_key(format!("pr:sess:{{{}}}", sess_id));
            let b = ds.encode_to_vec();
            let ttl = conf.network.device_session_ttl.as_millis() as usize;
            let pr_ttl = lifetime.num_milliseconds() as usize;

            let mut c = get_redis_conn()?;

            // We need to store a pointer from both the DevAddr and DevEUI to the
            // passive-roaming device-session ID. This is needed:
            //  * Because the DevAddr is not guaranteed to be unique
            //  * Because the DevEUI might not be given (thus is also not guaranteed
            //    to be an unique identifier).
            //
            // But:
            //  * We need to be able to lookup the session using the DevAddr (potentially
            //    using the MIC validation).
            //  * We need to be able to stop a passive-roaming session given a DevEUI.
            c.new_pipeline()
                .atomic()
                .cmd("SADD")
                .arg(&dev_addr_key)
                .arg(&sess_id.to_string())
                .ignore()
                .cmd("SADD")
                .arg(&dev_eui_key)
                .arg(&sess_id.to_string())
                .ignore()
                .cmd("PEXPIRE")
                .arg(&dev_addr_key)
                .arg(ttl)
                .ignore()
                .cmd("PEXPIRE")
                .arg(&dev_eui_key)
                .arg(ttl)
                .ignore()
                .cmd("PSETEX")
                .arg(&sess_key)
                .arg(pr_ttl)
                .arg(b)
                .ignore()
                .query(&mut c)?;

            Ok(())
        }
    })
    .await??;

    info!(id = %sess_id, "Passive-roaming device-session saved");

    Ok(())
}

pub async fn get(id: Uuid) -> Result<internal::PassiveRoamingDeviceSession, Error> {
    task::spawn_blocking({
        move || -> Result<internal::PassiveRoamingDeviceSession, Error> {
            let key = redis_key(format!("pr:sess:{{{}}}", id));
            let mut c = get_redis_conn()?;
            let v: Vec<u8> = redis::cmd("GET")
                .arg(key)
                .query(&mut *c)
                .context("Get passive-roaming device-session")?;
            if v.is_empty() {
                return Err(Error::NotFound(id.to_string()));
            }
            let ds = internal::PassiveRoamingDeviceSession::decode(&mut Cursor::new(v))
                .context("Decode passive-roaming device-session")?;
            Ok(ds)
        }
    })
    .await?
}

pub async fn delete(id: Uuid) -> Result<()> {
    task::spawn_blocking({
        move || -> Result<()> {
            let key = redis_key(format!("pr:sess:{{{}}}", id));
            let mut c = get_redis_conn()?;
            redis::cmd("DEL").arg(&key).query(&mut *c)?;
            Ok(())
        }
    })
    .await??;
    info!(id = %id, "Passive-roaming device-session deleted");
    Ok(())
}

pub async fn get_for_phy_payload(
    phy: &lrwn::PhyPayload,
) -> Result<Vec<internal::PassiveRoamingDeviceSession>, Error> {
    // Clone the PhyPayload, as we will update the f_cnt to the full (32bit) frame-counter value
    // for calculating the MIC.
    let mut phy = phy.clone();

    let (dev_addr, f_cnt_orig) = if let lrwn::Payload::MACPayload(v) = &phy.payload {
        (v.fhdr.devaddr, v.fhdr.f_cnt)
    } else {
        return Err(Error::InvalidPayload("MacPayload".to_string()));
    };

    let sessions = get_sessions_for_dev_addr(dev_addr).await?;
    let mut out: Vec<internal::PassiveRoamingDeviceSession> = Vec::new();

    for ds in sessions {
        // We will not validate the MIC.
        if !ds.validate_mic {
            out.push(ds);
            continue;
        }

        let f_nwk_s_int_key = AES128Key::from_slice(&ds.f_nwk_s_int_key)?;

        // Set the full frame-counter.
        if let lrwn::Payload::MACPayload(pl) = &mut phy.payload {
            pl.fhdr.f_cnt = get_full_f_cnt_up(ds.f_cnt_up, f_cnt_orig);
        }

        let mic_ok = if ds.lorawan_1_1 {
            phy.validate_uplink_data_micf(&f_nwk_s_int_key)?
        } else {
            phy.validate_uplink_data_mic(
                lrwn::MACVersion::LoRaWAN1_0,
                0,
                0,
                0,
                &f_nwk_s_int_key,
                &f_nwk_s_int_key,
            )?
        };

        if mic_ok {
            out.push(ds);
        }
    }

    Ok(out)
}

async fn get_sessions_for_dev_addr(
    dev_addr: DevAddr,
) -> Result<Vec<internal::PassiveRoamingDeviceSession>> {
    let mut out: Vec<internal::PassiveRoamingDeviceSession> = Vec::new();
    let ids = get_session_ids_for_dev_addr(dev_addr).await?;

    for id in ids {
        if let Ok(v) = get(id).await {
            out.push(v);
        }
    }

    Ok(out)
}

async fn get_session_ids_for_dev_addr(dev_addr: DevAddr) -> Result<Vec<Uuid>> {
    task::spawn_blocking({
        move || -> Result<Vec<Uuid>> {
            let key = redis_key(format!("pr:devaddr:{{{}}}", dev_addr));
            let mut c = get_redis_conn()?;
            let v: Vec<String> = redis::cmd("SMEMBERS").arg(key).query(&mut *c)?;

            let mut out: Vec<Uuid> = Vec::new();
            for id in &v {
                out.push(Uuid::from_str(id)?);
            }

            Ok(out)
        }
    })
    .await?
}

pub async fn get_session_ids_for_dev_eui(dev_eui: EUI64) -> Result<Vec<Uuid>> {
    task::spawn_blocking({
        move || -> Result<Vec<Uuid>> {
            let key = redis_key(format!("pr:dev:{{{}}}", dev_eui));
            let mut c = get_redis_conn()?;
            let v: Vec<String> = redis::cmd("SMEMBERS").arg(key).query(&mut *c)?;

            let mut out: Vec<Uuid> = Vec::new();
            for id in &v {
                out.push(Uuid::from_str(id)?);
            }

            Ok(out)
        }
    })
    .await?
}

fn get_full_f_cnt_up(next_expected_full_fcnt: u32, truncated_f_cnt: u32) -> u32 {
    // Handle re-transmission.
    if truncated_f_cnt == (((next_expected_full_fcnt % (1 << 16)) as u16).wrapping_sub(1)) as u32 {
        return next_expected_full_fcnt - 1;
    }

    let gap = ((truncated_f_cnt as u16).wrapping_sub((next_expected_full_fcnt % (1 << 16)) as u16))
        as u32;

    next_expected_full_fcnt.wrapping_add(gap)
}
