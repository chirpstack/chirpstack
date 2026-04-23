use anyhow::{Result, anyhow};
use chrono::{TimeDelta, Utc};
use tracing::{info, warn};

use crate::gpstime::ToGpsTime;
use crate::storage::fields::device_profile::Ts005Version;
use crate::storage::{device, device_keys, device_profile, device_queue, fuota, multicast};
use lrwn::applayer::multicastsetup;
use lrwn::region::MacVersion;
use lrwn::{AES128Key, DevAddr};

pub async fn handle_uplink(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    data: &[u8],
) -> Result<()> {
    let version = dp
        .app_layer_params
        .ts005_version
        .ok_or_else(|| anyhow!("Device does not support TS005"))?;

    match version {
        Ts005Version::V100 => handle_uplink_v100(dev, dp, data).await,
        Ts005Version::V200 => handle_uplink_v200(dev, dp, data).await,
    }
}

async fn handle_uplink_v100(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    data: &[u8],
) -> Result<()> {
    let pl = multicastsetup::v1::Payload::from_slice(true, data)?;

    match pl {
        multicastsetup::v1::Payload::McGroupSetupAns(pl) => {
            handle_v1_mc_group_setup_ans(dev, dp, pl).await?
        }
        multicastsetup::v1::Payload::McGroupDeleteAns(pl) => {
            handle_v1_mc_group_delete_ans(dev, pl).await?
        }
        multicastsetup::v1::Payload::McClassBSessionAns(pl) => {
            handle_v1_mc_class_b_session_ans(dev, pl).await?
        }
        multicastsetup::v1::Payload::McClassCSessionAns(pl) => {
            handle_v1_mc_class_c_session_ans(dev, pl).await?
        }
        _ => {}
    }

    Ok(())
}

async fn handle_uplink_v200(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    data: &[u8],
) -> Result<()> {
    let pl = multicastsetup::v2::Payload::from_slice(true, data)?;

    match pl {
        multicastsetup::v2::Payload::McGroupSetupAns(pl) => {
            handle_v2_mc_group_setup_ans(dev, dp, pl).await?
        }
        multicastsetup::v2::Payload::McGroupDeleteAns(pl) => {
            handle_v2_mc_group_delete_ans(dev, pl).await?
        }
        multicastsetup::v2::Payload::McClassBSessionAns(pl) => {
            handle_v2_mc_class_b_session_ans(dev, pl).await?
        }
        multicastsetup::v2::Payload::McClassCSessionAns(pl) => {
            handle_v2_mc_class_c_session_ans(dev, pl).await?
        }
        _ => {}
    }

    Ok(())
}

async fn handle_v1_mc_group_setup_ans(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    pl: multicastsetup::v1::McGroupSetupAnsPayload,
) -> Result<()> {
    info!("Handling McGroupSetupAns");

    handle_multicast_group_setup_ans(
        dev,
        dp,
        Ts005Version::V100,
        pl.mc_group_id_header.mc_group_id,
        pl.mc_group_id_header.id_error,
    )
    .await
}

async fn handle_v2_mc_group_setup_ans(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    pl: multicastsetup::v2::McGroupSetupAnsPayload,
) -> Result<()> {
    info!("Handling McGroupSetupAns");

    handle_multicast_group_setup_ans(
        dev,
        dp,
        Ts005Version::V200,
        pl.mc_group_id_header.mc_group_id,
        pl.mc_group_id_header.id_error,
    )
    .await
}

async fn handle_v1_mc_group_delete_ans(
    dev: &device::Device,
    pl: multicastsetup::v1::McGroupDeleteAnsPayload,
) -> Result<()> {
    info!("Handling McGroupDeleteAns");
    handle_multicast_group_delete_ans(
        dev,
        pl.mc_group_id_header.mc_group_id,
        pl.mc_group_id_header.mc_group_undefined,
    )
    .await
}

async fn handle_v2_mc_group_delete_ans(
    dev: &device::Device,
    pl: multicastsetup::v2::McGroupDeleteAnsPayload,
) -> Result<()> {
    info!("Handling McGroupDeleteAns");
    handle_multicast_group_delete_ans(
        dev,
        pl.mc_group_id_header.mc_group_id,
        pl.mc_group_id_header.mc_group_undefined,
    )
    .await
}

async fn handle_multicast_group_setup_ans(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    version: Ts005Version,
    mc_group_id: u8,
    id_error: bool,
) -> Result<()> {
    if let Ok(mgd) = multicast::get_device_by_mc_group_id(&dev.dev_eui, mc_group_id as i16).await {
        if let Ok(fuota_dev) = fuota::get_device(mgd.multicast_group_id.into(), dev.dev_eui).await
            && fuota_dev.completed_at.is_none()
        {
            return handle_fuota_mc_group_setup_ans(fuota_dev, mc_group_id, id_error).await;
        }

        return handle_normal_mc_group_setup_ans(dev, dp, version, mgd, mc_group_id, id_error)
            .await;
    }

    if mc_group_id == 0
        && let Ok(fuota_dev) = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await
        && fuota_dev.completed_at.is_none()
    {
        return handle_fuota_mc_group_setup_ans(fuota_dev, mc_group_id, id_error).await;
    }

    warn!(
        dev_eui = %dev.dev_eui,
        mc_group_id = mc_group_id,
        "McGroupSetupAns does not match any multicast-group for device"
    );
    Ok(())
}

async fn handle_v1_mc_class_b_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v1::McClassBSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassBSessionAns");
    let err = if pl.status_and_mc_group_id.dr_error
        | pl.status_and_mc_group_id.freq_error
        | pl.status_and_mc_group_id.mc_group_undefined
    {
        Some(format!(
            "Error: McClassBSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
        ))
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassBSessionAns OK"
        );
        None
    };

    handle_multicast_session_ans(dev, pl.status_and_mc_group_id.mc_group_id, err).await
}

async fn handle_v2_mc_class_b_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v2::McClassBSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassBSessionAns");
    let err = if pl.status_and_mc_group_id.dr_error
        | pl.status_and_mc_group_id.freq_error
        | pl.status_and_mc_group_id.mc_group_undefined
        | pl.status_and_mc_group_id.start_missed
    {
        Some(format!(
            "Error: McClassBSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}, start_missed: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
            pl.status_and_mc_group_id.start_missed,
        ))
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassBSessionAns OK"
        );
        None
    };

    handle_multicast_session_ans(dev, pl.status_and_mc_group_id.mc_group_id, err).await
}

async fn handle_v1_mc_class_c_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v1::McClassCSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassCSessionAns");
    let err = if pl.status_and_mc_group_id.dr_error
        | pl.status_and_mc_group_id.freq_error
        | pl.status_and_mc_group_id.mc_group_undefined
    {
        Some(format!(
            "Error: McClassCSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
        ))
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassCSessionAns OK"
        );
        None
    };

    handle_multicast_session_ans(dev, pl.status_and_mc_group_id.mc_group_id, err).await
}

async fn handle_v2_mc_class_c_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v2::McClassCSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassCSessionAns");
    let err = if pl.status_and_mc_group_id.dr_error
        | pl.status_and_mc_group_id.freq_error
        | pl.status_and_mc_group_id.mc_group_undefined
        | pl.status_and_mc_group_id.start_missed
    {
        Some(format!(
            "Error: McClassCSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}, start_missed: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
            pl.status_and_mc_group_id.start_missed,
        ))
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassCSessionAns OK"
        );
        None
    };

    handle_multicast_session_ans(dev, pl.status_and_mc_group_id.mc_group_id, err).await
}

async fn handle_multicast_session_ans(
    dev: &device::Device,
    mc_group_id: u8,
    error_msg: Option<String>,
) -> Result<()> {
    if let Ok(mgd) = multicast::get_device_by_mc_group_id(&dev.dev_eui, mc_group_id as i16).await {
        if let Ok(fuota_dev) = fuota::get_device(mgd.multicast_group_id.into(), dev.dev_eui).await
            && fuota_dev.completed_at.is_none()
        {
            return handle_fuota_mc_session_ans(fuota_dev, mc_group_id, error_msg).await;
        }

        return handle_normal_mc_session_ans(dev, mc_group_id, mgd, error_msg).await;
    }

    if mc_group_id == 0
        && let Ok(fuota_dev) = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await
        && fuota_dev.completed_at.is_none()
    {
        return handle_fuota_mc_session_ans(fuota_dev, mc_group_id, error_msg).await;
    }

    warn!(
        dev_eui = %dev.dev_eui,
        mc_group_id = mc_group_id,
        "McSessionAns does not match any multicast-group for device"
    );
    Ok(())
}

async fn handle_fuota_mc_group_setup_ans(
    mut fuota_dev: fuota::FuotaDeploymentDevice,
    mc_group_id: u8,
    id_error: bool,
) -> Result<()> {
    if id_error {
        warn!(
            dev_eui = %fuota_dev.dev_eui,
            mc_group_id = mc_group_id,
            id_error = true,
            "McGroupSetupAns contains errors"
        );
        fuota_dev.error_msg = "Error: McGroupSetupAns response id_error=true".into();
    } else {
        fuota_dev.mc_group_setup_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;
    Ok(())
}

async fn handle_normal_mc_group_setup_ans(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    version: Ts005Version,
    mut mgd: multicast::MulticastGroupDevice,
    mc_group_id: u8,
    id_error: bool,
) -> Result<()> {
    if id_error {
        warn!(
            dev_eui = %dev.dev_eui,
            mc_group_id = mc_group_id,
            id_error = true,
            "McGroupSetupAns contains errors"
        );
        mgd.error_msg = "Error: McGroupSetupAns response id_error=true".into();
        let _ = multicast::update_device(mgd).await?;
        return Ok(());
    }

    mgd.mc_group_setup_completed_at = Some(Utc::now());
    mgd.error_msg = "".into();
    let _ = multicast::update_device(mgd.clone()).await?;

    if mgd.pending_delete {
        return Ok(());
    }

    let mg = multicast::get(&mgd.multicast_group_id.into()).await?;
    let session_start = (Utc::now() + TimeDelta::seconds(dp.uplink_interval as i64))
        .to_gps_time()
        .num_seconds()
        % (1 << 32);

    let (time_out, periodicity) = match mg.group_type.as_ref() {
        "B" => {
            if !dp.supports_class_b {
                mgd.error_msg = "Error: device does not support Class-B".into();
                let _ = multicast::update_device(mgd).await?;
                return Ok(());
            }
            let class_b_params = match &dp.class_b_params {
                Some(v) => v,
                None => {
                    mgd.error_msg = "Error: class_b_params is not set".into();
                    let _ = multicast::update_device(mgd).await?;
                    return Ok(());
                }
            };

            if class_b_params.timeout > 15 {
                mgd.error_msg = "Error: class_b_timeout exceeds 15".into();
                let _ = multicast::update_device(mgd).await?;
                return Ok(());
            }

            let periodicity = mg.class_b_ping_slot_periodicity as u8;
            if periodicity > 7 {
                mgd.error_msg = "Error: class_b_ping_slot_periodicity exceeds 7".into();
                let _ = multicast::update_device(mgd).await?;
                return Ok(());
            }

            (class_b_params.timeout as u8, periodicity)
        }
        "C" => {
            if !dp.supports_class_c {
                mgd.error_msg = "Error: device does not support Class-C".into();
                let _ = multicast::update_device(mgd).await?;
                return Ok(());
            }
            let class_c_params = match &dp.class_c_params {
                Some(v) => v,
                None => {
                    mgd.error_msg = "Error: class_c_params is not set".into();
                    let _ = multicast::update_device(mgd).await?;
                    return Ok(());
                }
            };

            if class_c_params.timeout > 15 {
                mgd.error_msg = "Error: class_c_timeout exceeds 15".into();
                let _ = multicast::update_device(mgd).await?;
                return Ok(());
            }

            (class_c_params.timeout as u8, 0)
        }
        _ => {
            mgd.error_msg = "Error: invalid multicast group-type".into();
            let _ = multicast::update_device(mgd).await?;
            return Ok(());
        }
    };

    let pl = build_mc_session_req(
        version,
        &mg.group_type,
        mc_group_id,
        session_start as u32,
        time_out,
        periodicity,
        mg.frequency as u32,
        mg.dr as u8,
    )?;

    device_queue::enqueue_item(device_queue::DeviceQueueItem {
        dev_eui: dev.dev_eui,
        f_port: dp.app_layer_params.ts005_f_port.into(),
        data: pl,
        ..Default::default()
    })
    .await?;

    Ok(())
}

async fn handle_multicast_group_delete_ans(
    dev: &device::Device,
    mc_group_id: u8,
    mc_group_undefined: bool,
) -> Result<()> {
    let mgd = match multicast::get_device_by_mc_group_id(&dev.dev_eui, mc_group_id as i16).await {
        Ok(v) => v,
        Err(_) => {
            warn!(
                dev_eui = %dev.dev_eui,
                mc_group_id = mc_group_id,
                "McGroupDeleteAns does not match any multicast-group for device"
            );
            return Ok(());
        }
    };

    if !mgd.pending_delete {
        warn!(
            dev_eui = %dev.dev_eui,
            mc_group_id = mc_group_id,
            "McGroupDeleteAns received without a pending delete request"
        );
        return Ok(());
    }

    if mc_group_undefined {
        warn!(
            dev_eui = %dev.dev_eui,
            mc_group_id = mc_group_id,
            "McGroupDeleteAns reports that the multicast-group is already undefined"
        );
    }

    multicast::remove_device(&mgd.multicast_group_id.into(), &dev.dev_eui).await?;
    Ok(())
}

async fn handle_fuota_mc_session_ans(
    mut fuota_dev: fuota::FuotaDeploymentDevice,
    mc_group_id: u8,
    error_msg: Option<String>,
) -> Result<()> {
    if let Some(msg) = error_msg {
        warn!(
            dev_eui = %fuota_dev.dev_eui,
            mc_group_id = mc_group_id,
            "McSessionAns contains errors"
        );
        fuota_dev.error_msg = msg;
    } else {
        fuota_dev.mc_session_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;
    Ok(())
}

async fn handle_normal_mc_session_ans(
    dev: &device::Device,
    mc_group_id: u8,
    mut mgd: multicast::MulticastGroupDevice,
    error_msg: Option<String>,
) -> Result<()> {
    if let Some(msg) = error_msg {
        warn!(
            dev_eui = %dev.dev_eui,
            mc_group_id = mc_group_id,
            "McSessionAns contains errors"
        );
        mgd.error_msg = msg;
    } else {
        mgd.mc_session_completed_at = Some(Utc::now());
        mgd.error_msg = "".into();
    }

    let _ = multicast::update_device(mgd).await?;
    Ok(())
}

pub fn derive_mc_keys(
    version: Ts005Version,
    mc_key: AES128Key,
    mc_addr: DevAddr,
) -> Result<(AES128Key, AES128Key)> {
    Ok(match version {
        Ts005Version::V100 => (
            multicastsetup::v1::get_mc_app_s_key(mc_key, mc_addr)?,
            multicastsetup::v1::get_mc_net_s_key(mc_key, mc_addr)?,
        ),
        Ts005Version::V200 => (
            multicastsetup::v2::get_mc_app_s_key(mc_key, mc_addr)?,
            multicastsetup::v2::get_mc_net_s_key(mc_key, mc_addr)?,
        ),
    })
}

pub fn build_mc_group_setup_req(
    version: Ts005Version,
    mac_version: MacVersion,
    mc_group_id: u8,
    mc_addr: DevAddr,
    mc_key: AES128Key,
    dev_keys: &device_keys::DeviceKeys,
) -> Result<Vec<u8>> {
    match version {
        Ts005Version::V100 => {
            let mc_root_key = match mac_version {
                MacVersion::LORAWAN_1_0_0
                | MacVersion::LORAWAN_1_0_1
                | MacVersion::LORAWAN_1_0_2
                | MacVersion::LORAWAN_1_0_3
                | MacVersion::LORAWAN_1_0_4 => {
                    multicastsetup::v1::get_mc_root_key_for_gen_app_key(dev_keys.gen_app_key)?
                }
                MacVersion::LORAWAN_1_1_0 | MacVersion::Latest => {
                    multicastsetup::v1::get_mc_root_key_for_app_key(dev_keys.app_key)?
                }
            };
            let mc_ke_key = multicastsetup::v1::get_mc_ke_key(mc_root_key)?;
            let mc_key_encrypted = multicastsetup::v1::encrypt_mc_key(mc_ke_key, mc_key);

            multicastsetup::v1::Payload::McGroupSetupReq(
                multicastsetup::v1::McGroupSetupReqPayload {
                    mc_group_id_header: multicastsetup::v1::McGroupSetupReqPayloadMcGroupIdHeader {
                        mc_group_id,
                    },
                    mc_addr,
                    mc_key_encrypted,
                    min_mc_f_count: 0,
                    max_mc_f_count: u32::MAX,
                },
            )
            .to_vec()
        }
        Ts005Version::V200 => {
            let mc_root_key = match mac_version {
                MacVersion::LORAWAN_1_0_0
                | MacVersion::LORAWAN_1_0_1
                | MacVersion::LORAWAN_1_0_2
                | MacVersion::LORAWAN_1_0_3
                | MacVersion::LORAWAN_1_0_4 => {
                    multicastsetup::v2::get_mc_root_key_for_gen_app_key(dev_keys.gen_app_key)?
                }
                MacVersion::LORAWAN_1_1_0 | MacVersion::Latest => {
                    multicastsetup::v2::get_mc_root_key_for_app_key(dev_keys.app_key)?
                }
            };
            let mc_ke_key = multicastsetup::v2::get_mc_ke_key(mc_root_key)?;
            let mc_key_encrypted = multicastsetup::v2::encrypt_mc_key(mc_ke_key, mc_key);

            multicastsetup::v2::Payload::McGroupSetupReq(
                multicastsetup::v2::McGroupSetupReqPayload {
                    mc_group_id_header: multicastsetup::v2::McGroupSetupReqPayloadMcGroupIdHeader {
                        mc_group_id,
                    },
                    mc_addr,
                    mc_key_encrypted,
                    min_mc_f_count: 0,
                    max_mc_f_count: u32::MAX,
                },
            )
            .to_vec()
        }
    }
}

pub fn build_mc_group_delete_req(version: Ts005Version, mc_group_id: u8) -> Result<Vec<u8>> {
    match version {
        Ts005Version::V100 => multicastsetup::v1::Payload::McGroupDeleteReq(
            multicastsetup::v1::McGroupDeleteReqPayload {
                mc_group_id_header: multicastsetup::v1::McGroupDeleteReqPayloadMcGroupIdHeader {
                    mc_group_id,
                },
            },
        )
        .to_vec(),
        Ts005Version::V200 => multicastsetup::v2::Payload::McGroupDeleteReq(
            multicastsetup::v2::McGroupDeleteReqPayload {
                mc_group_id_header: multicastsetup::v2::McGroupDeleteReqPayloadMcGroupIdHeader {
                    mc_group_id,
                },
            },
        )
        .to_vec(),
    }
}

pub fn build_mc_session_req(
    version: Ts005Version,
    group_type: &str,
    mc_group_id: u8,
    session_start: u32,
    time_out: u8,
    periodicity: u8,
    frequency: u32,
    dr: u8,
) -> Result<Vec<u8>> {
    match version {
        Ts005Version::V100 => {
            match group_type {
                "B" => multicastsetup::v1::Payload::McClassBSessionReq(
                    multicastsetup::v1::McClassBSessionReqPayload {
                        mc_group_id_header:
                            multicastsetup::v1::McClassBSessionReqPayloadMcGroupIdHeader {
                                mc_group_id,
                            },
                        session_time: session_start - (session_start % 128),
                        time_out_periodicity:
                            multicastsetup::v1::McClassBSessionReqPayloadTimeOutPeriodicity {
                                time_out,
                                periodicity,
                            },
                        dl_frequ: frequency,
                        dr,
                    },
                )
                .to_vec(),
                "C" => {
                    multicastsetup::v1::Payload::McClassCSessionReq(
                        multicastsetup::v1::McClassCSessionReqPayload {
                            mc_group_id_header:
                                multicastsetup::v1::McClassCSessionReqPayloadMcGroupIdHeader {
                                    mc_group_id,
                                },
                            session_time: session_start,
                            session_time_out:
                                multicastsetup::v1::McClassCSessionReqPayloadSessionTimeOut {
                                    time_out,
                                },
                            dl_frequ: frequency,
                            dr,
                        },
                    )
                    .to_vec()
                }
                _ => Err(anyhow!("Unsupported group-type: {}", group_type)),
            }
        }
        Ts005Version::V200 => {
            match group_type {
                "B" => multicastsetup::v2::Payload::McClassBSessionReq(
                    multicastsetup::v2::McClassBSessionReqPayload {
                        mc_group_id_header:
                            multicastsetup::v2::McClassBSessionReqPayloadMcGroupIdHeader {
                                mc_group_id,
                            },
                        session_time: session_start - (session_start % 128),
                        time_out_periodicity:
                            multicastsetup::v2::McClassBSessionReqPayloadTimeOutPeriodicity {
                                time_out,
                                periodicity,
                            },
                        dl_frequ: frequency,
                        dr,
                    },
                )
                .to_vec(),
                "C" => {
                    multicastsetup::v2::Payload::McClassCSessionReq(
                        multicastsetup::v2::McClassCSessionReqPayload {
                            mc_group_id_header:
                                multicastsetup::v2::McClassCSessionReqPayloadMcGroupIdHeader {
                                    mc_group_id,
                                },
                            session_time: session_start,
                            session_time_out:
                                multicastsetup::v2::McClassCSessionReqPayloadSessionTimeOut {
                                    time_out,
                                },
                            dl_frequ: frequency,
                            dr,
                        },
                    )
                    .to_vec()
                }
                _ => Err(anyhow!("Unsupported group-type: {}", group_type)),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::{
        application, device, device_keys, device_profile, device_queue, fields, multicast, tenant,
    };
    use crate::test;
    use lrwn::EUI64;
    use lrwn::region::{CommonName, MacVersion, Revision};

    async fn setup_multicast_device() -> (
        device::Device,
        device_profile::DeviceProfile,
        multicast::MulticastGroup,
        multicast::MulticastGroupDevice,
    ) {
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: Some(t.id),
            name: "test-dp".into(),
            region: CommonName::EU868,
            mac_version: MacVersion::LORAWAN_1_0_2,
            reg_params_revision: Revision::B,
            adr_algorithm_id: "default".into(),
            uplink_interval: 60,
            supports_otaa: true,
            supports_class_c: true,
            class_c_params: Some(fields::ClassCParams { timeout: 10 }),
            app_layer_params: fields::AppLayerParams {
                ts005_version: Some(Ts005Version::V100),
                ts005_f_port: 200,
                ..Default::default()
            },
            ..Default::default()
        })
        .await
        .unwrap();

        let d = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp.id,
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            name: "test-device".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let _ = device_keys::create(device_keys::DeviceKeys {
            dev_eui: d.dev_eui,
            ..Default::default()
        })
        .await
        .unwrap();

        let mc_key = AES128Key::from_bytes([9, 8, 7, 6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
        let mc_addr = DevAddr::from_be_bytes([1, 2, 3, 4]);
        let (mc_app_s_key, mc_nwk_s_key) =
            derive_mc_keys(Ts005Version::V100, mc_key, mc_addr).unwrap();

        let mg = multicast::create(multicast::MulticastGroup {
            application_id: app.id,
            name: "test-mg".into(),
            region: CommonName::EU868,
            mc_addr,
            mc_nwk_s_key,
            mc_app_s_key,
            mc_key: Some(mc_key),
            f_cnt: 0,
            group_type: "C".into(),
            dr: 3,
            frequency: 868300000,
            class_b_ping_slot_periodicity: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        let mgd = multicast::add_device_with_next_mc_group_id(&mg.id.into(), &d.dev_eui)
            .await
            .unwrap();

        (d, dp, mg, mgd)
    }

    #[tokio::test]
    async fn test_multicast_group_setup_ans_enqueues_session_req() {
        let _guard = test::prepare().await;
        let (d, dp, mg, mgd) = setup_multicast_device().await;
        let mc_group_id = mgd.mc_group_id.unwrap() as u8;

        let data = multicastsetup::v1::Payload::McGroupSetupAns(
            multicastsetup::v1::McGroupSetupAnsPayload {
                mc_group_id_header: multicastsetup::v1::McGroupSetupAnsPayloadMcGroupIdHeader {
                    mc_group_id,
                    id_error: false,
                },
            },
        )
        .to_vec()
        .unwrap();

        handle_uplink(&d, &dp, &data).await.unwrap();

        let mgd = multicast::get_device(&mg.id.into(), &d.dev_eui)
            .await
            .unwrap();
        assert!(mgd.mc_group_setup_completed_at.is_some());

        let queue = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(1, queue.len());
        assert_eq!(dp.app_layer_params.ts005_f_port as i16, queue[0].f_port);

        let payload = multicastsetup::v1::Payload::from_slice(false, &queue[0].data).unwrap();
        match payload {
            multicastsetup::v1::Payload::McClassCSessionReq(pl) => {
                assert_eq!(mc_group_id, pl.mc_group_id_header.mc_group_id);
                assert_eq!(10, pl.session_time_out.time_out);
                assert_eq!(868300000, pl.dl_frequ);
                assert_eq!(3, pl.dr);
            }
            _ => panic!("expected McClassCSessionReq"),
        }
    }

    #[tokio::test]
    async fn test_multicast_group_delete_ans_removes_pending_device() {
        let _guard = test::prepare().await;
        let (d, dp, mg, mut mgd) = setup_multicast_device().await;
        let mc_group_id = mgd.mc_group_id.unwrap() as u8;

        mgd.pending_delete = true;
        let _ = multicast::update_device(mgd).await.unwrap();

        let data = multicastsetup::v1::Payload::McGroupDeleteAns(
            multicastsetup::v1::McGroupDeleteAnsPayload {
                mc_group_id_header: multicastsetup::v1::McGroupDeleteAnsPayloadMcGroupIdHeader {
                    mc_group_id,
                    mc_group_undefined: false,
                },
            },
        )
        .to_vec()
        .unwrap();

        handle_uplink(&d, &dp, &data).await.unwrap();

        assert!(
            multicast::get_device(&mg.id.into(), &d.dev_eui)
                .await
                .is_err()
        );
    }
}
