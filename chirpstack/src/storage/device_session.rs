use std::collections::HashSet;
use std::io::Cursor;

use anyhow::{Context, Result};
use prost::Message;
use tokio::task;
use tracing::{error, info, trace, warn};

use super::error::Error;
use super::{get_redis_conn, redis_key};
use crate::api::helpers::FromProto;
use crate::config;
use chirpstack_api::internal;
use lrwn::{AES128Key, DevAddr, Payload, PhyPayload, EUI64};

pub enum ValidationStatus {
    Ok(u32, internal::DeviceSession),
    Retransmission(u32, internal::DeviceSession),
    Reset(u32, internal::DeviceSession),
}

pub async fn save(ds: &internal::DeviceSession) -> Result<()> {
    let eui = EUI64::from_slice(&ds.dev_eui)?;
    let addr = DevAddr::from_slice(&ds.dev_addr)?;

    task::spawn_blocking({
        let ds = ds.clone();
        move || -> Result<()> {
            let conf = config::get();
            let addr_key = redis_key(format!("devaddr:{{{}}}", addr));
            let ds_key = redis_key(format!("device:{{{}}}:ds", eui));
            let b = ds.encode_to_vec();
            let ttl = conf.network.device_session_ttl.as_millis() as usize;
            let mut c = get_redis_conn()?;

            // Atomic add and pexpire.
            c.new_pipeline()
                .atomic()
                .cmd("SADD")
                .arg(&addr_key)
                .arg(&eui.to_be_bytes())
                .ignore()
                .cmd("PEXPIRE")
                .arg(&addr_key)
                .arg(ttl)
                .ignore()
                .query(&mut c)?;

            // In case there is a pending rejoin session, make sure that the new
            // DevAddr also resolves to the device-session.
            if let Some(pending_ds) = &ds.pending_rejoin_device_session {
                let pending_addr = DevAddr::from_slice(&pending_ds.dev_addr)?;
                let pending_addr_key = redis_key(format!("devaddr:{{{}}}", pending_addr));

                c.new_pipeline()
                    .atomic()
                    .cmd("SADD")
                    .arg(&pending_addr_key)
                    .arg(&eui.to_be_bytes())
                    .ignore()
                    .cmd("PEXPIRE")
                    .arg(&pending_addr_key)
                    .arg(ttl)
                    .ignore()
                    .query(&mut c)?;
            }

            redis::cmd("PSETEX")
                .arg(ds_key)
                .arg(ttl)
                .arg(b)
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await??;

    info!(dev_eui = %eui, dev_addr = %addr, "Device-session saved");
    Ok(())
}

pub async fn get(dev_eui: &EUI64) -> Result<chirpstack_api::internal::DeviceSession, Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<chirpstack_api::internal::DeviceSession, Error> {
            let key = redis_key(format!("device:{{{}}}:ds", dev_eui));
            let mut c = get_redis_conn()?;
            let v: Vec<u8> = redis::cmd("GET")
                .arg(key)
                .query(&mut *c)
                .context("Get device-session")?;
            if v.is_empty() {
                return Err(Error::NotFound(dev_eui.to_string()));
            }
            let ds = chirpstack_api::internal::DeviceSession::decode(&mut Cursor::new(v))
                .context("Decode device-session")?;
            Ok(ds)
        }
    })
    .await?
}

pub async fn delete(dev_eui: &EUI64) -> Result<()> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<()> {
            let key = redis_key(format!("device:{{{}}}:ds", dev_eui));
            let mut c = get_redis_conn()?;
            redis::cmd("DEL").arg(&key).query(&mut *c)?;

            Ok(())
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, "Device-session deleted");
    Ok(())
}

// Return the device-session matching the given PhyPayload. This will fetch all device-session
// associated with the used DevAddr and based on f_cont and mic, decides which one to use.
// This function will increment the uplink frame-counter and will immediately update the
// device-session in the database, to make sure that in case this function is called multiple
// times, at most one will be valid.
// On Ok response, the PhyPayload f_cnt will be set to the full 32bit frame-counter based on the
// device-session context.
pub async fn get_for_phypayload_and_incr_f_cnt_up(
    relayed: bool,
    phy: &mut PhyPayload,
    tx_dr: u8,
    tx_ch: u8,
) -> Result<ValidationStatus, Error> {
    let mut _dev_addr = DevAddr::from_be_bytes([0x00, 0x00, 0x00, 0x00]);
    let mut _f_cnt_orig = 0;

    // Get the dev_addr and original f_cnt.
    if let Payload::MACPayload(pl) = &phy.payload {
        _dev_addr = pl.fhdr.devaddr;
        _f_cnt_orig = pl.fhdr.f_cnt;
    } else {
        return Err(Error::InvalidPayload("MacPayload".to_string()));
    }

    let device_sessions = get_for_dev_addr(_dev_addr)
        .await
        .context("Get device-sessions for DevAddr")?;
    if device_sessions.is_empty() {
        return Err(Error::NotFound(_dev_addr.to_string()));
    }

    for mut ds in device_sessions {
        // Get the full 32bit frame-counter.
        let full_f_cnt = get_full_f_cnt_up(ds.f_cnt_up, _f_cnt_orig);
        let f_nwk_s_int_key = AES128Key::from_slice(&ds.f_nwk_s_int_key)?;
        let s_nwk_s_int_key = AES128Key::from_slice(&ds.s_nwk_s_int_key)?;

        // Check both the full frame-counter and the received frame-counter
        // truncated to the 16LSB.
        // The latter is needed in case of a frame-counter reset as the
        // GetFullFCntUp will think the 16LSB has rolled over and will
        // increment the 16MSB bit.
        let mut mic_ok = false;
        for f_cnt in &[full_f_cnt, _f_cnt_orig] {
            // Set the full f_cnt.
            if let Payload::MACPayload(pl) = &mut phy.payload {
                pl.fhdr.f_cnt = *f_cnt;
            }

            mic_ok = phy
                .validate_uplink_data_mic(
                    ds.mac_version().from_proto(),
                    ds.conf_f_cnt,
                    tx_dr,
                    tx_ch,
                    &f_nwk_s_int_key,
                    &s_nwk_s_int_key,
                )
                .context("Validate MIC")?;

            if mic_ok {
                break;
            }
        }

        if mic_ok {
            let full_f_cnt = if let Payload::MACPayload(pl) = &phy.payload {
                pl.fhdr.f_cnt
            } else {
                0
            };

            if let Some(relay) = &ds.relay {
                if !relayed && relay.ed_relay_only {
                    warn!(
                        dev_eui = hex::encode(ds.dev_eui),
                        "Only communication through relay is allowed"
                    );
                    return Err(Error::NotFound(_dev_addr.to_string()));
                }
            }

            if full_f_cnt >= ds.f_cnt_up {
                // Make sure that in case of concurrent calls for the same uplink only one will
                // pass. Either the concurrent call would read the incremented uplink frame-counter
                // or it is unable to aquire the lock.
                let mut c = get_redis_conn()?;
                let lock_key = redis_key(format!(
                    "device:{{{}}}:ds:lock:{}",
                    hex::encode(&ds.dev_eui),
                    full_f_cnt,
                ));
                let set: bool = redis::cmd("SET")
                    .arg(&lock_key)
                    .arg("lock")
                    .arg("EX")
                    .arg(1_usize)
                    .arg("NX")
                    .query(&mut *c)?;

                if !set {
                    return Ok(ValidationStatus::Retransmission(full_f_cnt, ds));
                }

                // We immediately save the device-session to make sure that concurrent calls for
                // the same uplink will fail on the frame-counter validation.
                let ds_f_cnt_up = ds.f_cnt_up;
                ds.f_cnt_up = full_f_cnt + 1;
                save(&ds).await?;
                ds.f_cnt_up = ds_f_cnt_up;

                return Ok(ValidationStatus::Ok(full_f_cnt, ds));
            } else if ds.skip_f_cnt_check {
                // re-transmission or frame-counter reset
                ds.f_cnt_up = 0;
                return Ok(ValidationStatus::Ok(full_f_cnt, ds));
            } else if full_f_cnt == (ds.f_cnt_up - 1) {
                // re-transmission, the frame-counter did not increment
                return Ok(ValidationStatus::Retransmission(full_f_cnt, ds));
            } else {
                return Ok(ValidationStatus::Reset(full_f_cnt, ds));
            }
        }

        // Restore the original f_cnt.
        if let Payload::MACPayload(pl) = &mut phy.payload {
            pl.fhdr.f_cnt = _f_cnt_orig;
        }
    }

    Err(Error::InvalidMIC)
}

// Simmilar to get_for_phypayload_and_incr_f_cnt_up, but only retrieves the device-session for the
// given PhyPayload. As it does not return the ValidationStatus, it only returns the DeviceSession
// in case of a valid frame-counter.
// On Ok response, the PhyPayload f_cnt will be set to the full 32bit frame-counter based on the
// device-session context.
pub async fn get_for_phypayload(
    phy: &mut PhyPayload,
    tx_dr: u8,
    tx_ch: u8,
) -> Result<internal::DeviceSession, Error> {
    // Get the dev_addr and original f_cnt.
    let (dev_addr, f_cnt_orig) = if let Payload::MACPayload(pl) = &phy.payload {
        (pl.fhdr.devaddr, pl.fhdr.f_cnt)
    } else {
        return Err(Error::InvalidPayload("MacPayload".to_string()));
    };

    let device_sessions = get_for_dev_addr(dev_addr)
        .await
        .context("Get device-sessions for DevAddr")?;
    if device_sessions.is_empty() {
        return Err(Error::NotFound(dev_addr.to_string()));
    }

    for ds in device_sessions {
        // Get the full 32bit frame-counter.
        let full_f_cnt = get_full_f_cnt_up(ds.f_cnt_up, f_cnt_orig);
        let f_nwk_s_int_key = AES128Key::from_slice(&ds.f_nwk_s_int_key)?;
        let s_nwk_s_int_key = AES128Key::from_slice(&ds.s_nwk_s_int_key)?;

        // Set the full f_cnt
        if let Payload::MACPayload(pl) = &mut phy.payload {
            pl.fhdr.f_cnt = full_f_cnt;
        }

        let mic_ok = phy
            .validate_uplink_data_mic(
                ds.mac_version().from_proto(),
                ds.conf_f_cnt,
                tx_dr,
                tx_ch,
                &f_nwk_s_int_key,
                &s_nwk_s_int_key,
            )
            .context("Validate MIC")?;

        if mic_ok && full_f_cnt >= ds.f_cnt_up {
            return Ok(ds);
        }

        // Restore the original f_cnt.
        if let Payload::MACPayload(pl) = &mut phy.payload {
            pl.fhdr.f_cnt = f_cnt_orig;
        }
    }

    Err(Error::InvalidMIC)
}

async fn get_dev_euis_for_dev_addr(dev_addr: DevAddr) -> Result<Vec<EUI64>> {
    task::spawn_blocking({
        let dev_addr = dev_addr;
        move || -> Result<Vec<EUI64>> {
            let key = redis_key(format!("devaddr:{{{}}}", dev_addr));
            let mut c = get_redis_conn()?;
            let dev_euis: HashSet<Vec<u8>> = redis::cmd("SMEMBERS")
                .arg(key)
                .query(&mut *c)
                .context("Get DevEUIs for DevAddr")?;

            let mut out = Vec::new();
            for dev_eui in &dev_euis {
                out.push(EUI64::from_slice(dev_eui)?);
            }

            Ok(out)
        }
    })
    .await?
}

async fn remove_dev_eui_from_dev_addr_set(dev_addr: DevAddr, dev_eui: EUI64) -> Result<()> {
    task::spawn_blocking({
        let dev_addr = dev_addr;
        let dev_eui = dev_eui;
        move || -> Result<()> {
            let key = redis_key(format!("devaddr:{{{}}}", dev_addr));
            let mut c = get_redis_conn()?;
            redis::cmd("SREM")
                .arg(key)
                .arg(&dev_eui.to_be_bytes())
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await?
}

async fn get_for_dev_addr(dev_addr: DevAddr) -> Result<Vec<internal::DeviceSession>> {
    trace!(dev_addr = %dev_addr, "Getting device-session for DevAddr");
    let dev_euis = get_dev_euis_for_dev_addr(dev_addr).await?;

    let mut out = Vec::new();
    for dev_eui in dev_euis {
        let ds = match get(&dev_eui).await {
            Ok(v) => v,
            Err(e) => {
                if let Error::NotFound(_) = e {
                    if let Err(e) = remove_dev_eui_from_dev_addr_set(dev_addr, dev_eui).await {
                        error!(dev_addr = %dev_addr, dev_eui = %dev_eui, error = %e, "Remove DevEUI from DevAddr->DevEUI set error");
                    }
                } else {
                    error!(dev_addr = %dev_addr, dev_eui = %dev_eui, error = %e, "Get device-session for DevEUI error");
                }
                continue;
            }
        };

        let ds_dev_addr = DevAddr::from_slice(&ds.dev_addr)?;

        // When a pending rejoin device-session context is set and it has
        // the given devAddr, add it to the items list.
        if let Some(pending_ds) = &ds.pending_rejoin_device_session {
            let pending_dev_addr = DevAddr::from_slice(&pending_ds.dev_addr)?;
            if pending_dev_addr == dev_addr {
                out.push(*pending_ds.clone());
            }
        }

        // It is possible that the "main" device-session maps to a different
        // devAddr as the PendingRejoinDeviceSession is set (using the devAddr
        // that is used for the lookup).
        if ds_dev_addr == dev_addr {
            out.push(ds);
        }
    }

    Ok(out)
}

// GetFullFCntUp returns the full 32bit frame-counter, given the fCntUp which
// has been truncated to the last 16 LSB.
// Notes:
// * After a succesful validation of the FCntUp and the MIC, don't forget
//   to synchronize the device FCntUp with the packet FCnt.
// * In case of a frame-counter rollover, the returned values will be less
//   than the given DeviceSession FCntUp. This must be validated outside this
//   function!
// * In case of a re-transmission, the returned frame-counter equals
//   DeviceSession.FCntUp - 1, as the FCntUp value holds the next expected
//   frame-counter, not the FCntUp which was last seen.
fn get_full_f_cnt_up(next_expected_full_fcnt: u32, truncated_f_cnt: u32) -> u32 {
    // Handle re-transmission.
    if truncated_f_cnt == (((next_expected_full_fcnt % (1 << 16)) as u16).wrapping_sub(1)) as u32 {
        return next_expected_full_fcnt - 1;
    }

    let gap = ((truncated_f_cnt as u16).wrapping_sub((next_expected_full_fcnt % (1 << 16)) as u16))
        as u32;

    next_expected_full_fcnt.wrapping_add(gap)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

    #[test]
    fn test_get_full_f_cnt_up() {
        // server, device, expected
        let tests = vec![
            (1, 1, 1),                                 // frame-counter is as expected
            (1 << 16, 0, 1 << 16),                     // frame-counter is as expected
            ((1 << 16) + 1, 1, (1 << 16) + 1),         // frame-counter is as expected
            (0, 1, 1),                                 // one frame packet-loss
            ((1 << 16) + 1, 2, (1 << 16) + 2),         // one frame packet-loss
            (2, 1, 1),                                 // re-transmission of previous frame
            ((1 << 16) + 1, 0, (1 << 16)),             // re-transmission of previous frame
            ((1 << 16), (1 << 16) - 1, (1 << 16) - 1), // re-transmission of previous frame
            (u32::MAX, 0, 0),                          // 32bit frame-counter rollover
        ];

        for (i, tst) in tests.iter().enumerate() {
            let out = get_full_f_cnt_up(tst.0, tst.1);
            assert_eq!(tst.2, out, "Test: {}, expected: {}, got: {}", i, tst.2, out);
        }
    }

    #[tokio::test]
    async fn test_device_session() {
        let _guard = test::prepare().await;

        let device_sessions = vec![
            internal::DeviceSession {
                dev_addr: vec![0x01, 0x02, 0x03, 0x04],
                dev_eui: vec![0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01],
                s_nwk_s_int_key: vec![
                    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                    0x01, 0x01, 0x01,
                ],
                f_nwk_s_int_key: vec![
                    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                    0x01, 0x01, 0x01,
                ],
                nwk_s_enc_key: vec![
                    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                    0x01, 0x01, 0x01,
                ],
                f_cnt_up: 100,
                skip_f_cnt_check: true,
                ..Default::default()
            },
            internal::DeviceSession {
                dev_addr: vec![0x01, 0x02, 0x03, 0x04],
                dev_eui: vec![0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02],
                s_nwk_s_int_key: vec![
                    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                    0x02, 0x02, 0x02,
                ],
                f_nwk_s_int_key: vec![
                    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                    0x02, 0x02, 0x02,
                ],
                nwk_s_enc_key: vec![
                    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                    0x02, 0x02, 0x02,
                ],
                f_cnt_up: 200,
                ..Default::default()
            },
            internal::DeviceSession {
                dev_addr: vec![0x01, 0x02, 0x03, 0x04],
                dev_eui: vec![0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03],
                s_nwk_s_int_key: vec![
                    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
                    0x03, 0x03, 0x03,
                ],
                f_nwk_s_int_key: vec![
                    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
                    0x03, 0x03, 0x03,
                ],
                nwk_s_enc_key: vec![
                    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
                    0x03, 0x03, 0x03,
                ],
                f_cnt_up: 300,
                pending_rejoin_device_session: Some(Box::new(internal::DeviceSession {
                    dev_addr: vec![0x04, 0x03, 0x02, 0x01],
                    dev_eui: vec![0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03],
                    s_nwk_s_int_key: vec![
                        0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                        0x04, 0x04, 0x04, 0x04,
                    ],
                    f_nwk_s_int_key: vec![
                        0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                        0x04, 0x04, 0x04, 0x04,
                    ],
                    nwk_s_enc_key: vec![
                        0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                        0x04, 0x04, 0x04, 0x04,
                    ],
                    f_cnt_up: 0,
                    ..Default::default()
                })),
                ..Default::default()
            },
            internal::DeviceSession {
                dev_addr: vec![0x01, 0x02, 0x03, 0x04],
                dev_eui: vec![0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05],
                s_nwk_s_int_key: vec![
                    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                    0x05, 0x05, 0x05,
                ],
                f_nwk_s_int_key: vec![
                    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                    0x05, 0x05, 0x05,
                ],
                nwk_s_enc_key: vec![
                    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                    0x05, 0x05, 0x05,
                ],
                f_cnt_up: (1 << 16) + 1,
                ..Default::default()
            },
        ];

        for ds in &device_sessions {
            save(ds).await.unwrap();
        }

        #[derive(Default)]
        struct Test {
            name: String,
            dev_addr: DevAddr,
            s_nwk_s_int_key: AES128Key,
            f_nwk_s_int_key: AES128Key,
            f_cnt: u32,
            expected_retransmission: bool,
            expected_reset: bool,
            expected_dev_eui: EUI64,
            expected_fcnt_up: u32,
            expected_error: Option<String>,
        }

        let tests = vec![
            Test {
                name: "matching dev_eui 0101010101010101".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
                f_nwk_s_int_key: AES128Key::from_slice(&device_sessions[0].f_nwk_s_int_key)
                    .unwrap(),
                s_nwk_s_int_key: AES128Key::from_slice(&device_sessions[0].s_nwk_s_int_key)
                    .unwrap(),
                f_cnt: device_sessions[0].f_cnt_up,
                expected_retransmission: false,
                expected_reset: false,
                expected_fcnt_up: device_sessions[0].f_cnt_up,
                expected_dev_eui: EUI64::from_slice(&device_sessions[0].dev_eui).unwrap(),
                expected_error: None,
            },
            Test {
                name: "matching dev_eui 0202020202020202".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
                f_nwk_s_int_key: AES128Key::from_slice(&device_sessions[1].f_nwk_s_int_key)
                    .unwrap(),
                s_nwk_s_int_key: AES128Key::from_slice(&device_sessions[1].s_nwk_s_int_key)
                    .unwrap(),
                f_cnt: device_sessions[1].f_cnt_up,
                expected_retransmission: false,
                expected_reset: false,
                expected_fcnt_up: device_sessions[1].f_cnt_up,
                expected_dev_eui: EUI64::from_slice(&device_sessions[1].dev_eui).unwrap(),
                expected_error: None,
            },
            Test {
                name: "matching dev_eui 0101010101010101 with frame-counter reset".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
                f_nwk_s_int_key: AES128Key::from_slice(&device_sessions[0].f_nwk_s_int_key)
                    .unwrap(),
                s_nwk_s_int_key: AES128Key::from_slice(&device_sessions[0].s_nwk_s_int_key)
                    .unwrap(),
                f_cnt: 0,
                expected_retransmission: false,
                expected_reset: false,
                expected_fcnt_up: 0,
                expected_dev_eui: EUI64::from_slice(&device_sessions[0].dev_eui).unwrap(),
                expected_error: None,
            },
            Test {
                name: "matching dev_eui 0202020202020202 with invalid frame-counter".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
                f_nwk_s_int_key: AES128Key::from_slice(&device_sessions[1].f_nwk_s_int_key)
                    .unwrap(),
                s_nwk_s_int_key: AES128Key::from_slice(&device_sessions[1].s_nwk_s_int_key)
                    .unwrap(),
                f_cnt: 0,
                expected_reset: true,
                expected_dev_eui: EUI64::from_slice(&device_sessions[1].dev_eui).unwrap(),
                ..Default::default()
            },
            Test {
                name: "invalid DevAddr".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x01, 0x01, 0x01]),
                f_nwk_s_int_key: AES128Key::from_slice(&device_sessions[0].f_nwk_s_int_key)
                    .unwrap(),
                s_nwk_s_int_key: AES128Key::from_slice(&device_sessions[0].s_nwk_s_int_key)
                    .unwrap(),
                f_cnt: device_sessions[0].f_cnt_up,
                expected_error: Some("Object does not exist (id: 01010101)".to_string()),
                ..Default::default()
            },
            Test {
                name: "invalid nwk_s_key".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
                f_nwk_s_int_key: AES128Key::from_bytes([
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                ]),
                s_nwk_s_int_key: AES128Key::from_bytes([
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                ]),
                f_cnt: device_sessions[0].f_cnt_up,
                expected_error: Some("Invalid MIC".to_string()),
                ..Default::default()
            },
            Test {
                name: "matching pending rejoin device-session".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x04, 0x03, 0x02, 0x01]),
                f_nwk_s_int_key: AES128Key::from_bytes([
                    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                    0x04, 0x04, 0x04,
                ]),
                s_nwk_s_int_key: AES128Key::from_bytes([
                    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                    0x04, 0x04, 0x04,
                ]),
                f_cnt: 0,
                expected_dev_eui: EUI64::from_slice(&device_sessions[2].dev_eui).unwrap(),
                expected_fcnt_up: 0,
                expected_retransmission: false,
                expected_error: None,
                expected_reset: false,
            },
            Test {
                name: "frame-counter rollover (16lsb)".to_string(),
                dev_addr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
                f_nwk_s_int_key: AES128Key::from_bytes([
                    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                    0x05, 0x05, 0x05,
                ]),
                s_nwk_s_int_key: AES128Key::from_bytes([
                    0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                    0x05, 0x05, 0x05,
                ]),
                f_cnt: (1 << 16) + 11,
                expected_dev_eui: EUI64::from_slice(&device_sessions[3].dev_eui).unwrap(),
                expected_fcnt_up: (1 << 16) + 11,
                expected_retransmission: false,
                expected_error: None,
                expected_reset: false,
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);
            let mut phy = lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: tst.dev_addr,
                        f_ctrl: lrwn::FCtrl::default(),
                        f_cnt: tst.f_cnt,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                mic: None,
            };

            phy.set_uplink_data_mic(
                lrwn::MACVersion::LoRaWAN1_0,
                0,
                0,
                0,
                &tst.f_nwk_s_int_key,
                &tst.s_nwk_s_int_key,
            )
            .unwrap();

            // Truncate to 16LSB (as it would be transmitted over the air).
            if let lrwn::Payload::MACPayload(pl) = &mut phy.payload {
                pl.fhdr.f_cnt = tst.f_cnt % (1 << 16);
            }

            let ds_res = get_for_phypayload_and_incr_f_cnt_up(false, &mut phy, 0, 0).await;
            if tst.expected_error.is_some() {
                assert_eq!(true, ds_res.is_err());
                assert_eq!(
                    tst.expected_error.as_ref().unwrap(),
                    &ds_res.err().unwrap().to_string()
                );
                if let lrwn::Payload::MACPayload(pl) = &phy.payload {
                    assert_eq!(tst.f_cnt, pl.fhdr.f_cnt);
                }
            } else {
                let ds = ds_res.unwrap();

                // Validate that the f_cnt of the PhyPayload was set to the full frame-counter.
                if let lrwn::Payload::MACPayload(pl) = &phy.payload {
                    assert_eq!(tst.expected_fcnt_up, pl.fhdr.f_cnt);
                }

                if let ValidationStatus::Ok(full_f_cnt, ds) = ds {
                    assert_eq!(false, tst.expected_retransmission);
                    assert_eq!(
                        tst.expected_dev_eui,
                        EUI64::from_slice(&ds.dev_eui).unwrap()
                    );
                    assert_eq!(tst.expected_fcnt_up, full_f_cnt);
                } else if let ValidationStatus::Retransmission(full_f_cnt, ds) = ds {
                    assert_eq!(true, tst.expected_retransmission);
                    assert_eq!(
                        tst.expected_dev_eui,
                        EUI64::from_slice(&ds.dev_eui).unwrap()
                    );
                    assert_eq!(tst.expected_fcnt_up, full_f_cnt);
                } else if let ValidationStatus::Reset(_, ds) = ds {
                    assert_eq!(true, tst.expected_reset);
                    assert_eq!(
                        tst.expected_dev_eui,
                        EUI64::from_slice(&ds.dev_eui).unwrap()
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_for_dev_addr() {
        let _guard = test::prepare().await;

        let dev_eui_1 = EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]);
        let dev_eui_2 = EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]);
        let dev_addr = DevAddr::from_be_bytes([1, 2, 3, 4]);

        let ds_1 = internal::DeviceSession {
            dev_addr: dev_addr.to_vec(),
            dev_eui: dev_eui_1.to_vec(),
            ..Default::default()
        };

        let ds_2 = internal::DeviceSession {
            dev_addr: dev_addr.to_vec(),
            dev_eui: dev_eui_2.to_vec(),
            ..Default::default()
        };

        save(&ds_1).await.unwrap();
        save(&ds_2).await.unwrap();

        let dss = get_for_dev_addr(dev_addr).await.unwrap();
        assert_eq!(2, dss.len());

        let dev_euis = get_dev_euis_for_dev_addr(dev_addr).await.unwrap();
        assert_eq!(2, dev_euis.len());

        // At this point there is still a 'dangling' pointer from DevAddr->DevEUI.
        delete(&dev_eui_2).await.unwrap();
        let dev_euis = get_dev_euis_for_dev_addr(dev_addr).await.unwrap();
        assert_eq!(2, dev_euis.len());

        // This should only return one device-session.
        let dss = get_for_dev_addr(dev_addr).await.unwrap();
        assert_eq!(1, dss.len());
        assert_eq!(dev_eui_1.to_vec(), dss[0].dev_eui);

        // 'dangling' DevAddr->DevEUI pointers have been cleaned up.
        let dev_euis = get_dev_euis_for_dev_addr(dev_addr).await.unwrap();
        assert_eq!(1, dev_euis.len());
        assert_eq!(dev_eui_1, dev_euis[0]);
    }
}
