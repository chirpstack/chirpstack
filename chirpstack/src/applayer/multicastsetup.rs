use anyhow::{Result, anyhow};
use chrono::Utc;
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

pub fn derive_mc_keys(mc_key: AES128Key, mc_addr: DevAddr) -> Result<(AES128Key, AES128Key)> {
    Ok((
        multicastsetup::v1::get_mc_app_s_key(mc_key, mc_addr)?,
        multicastsetup::v1::get_mc_net_s_key(mc_key, mc_addr)?,
    ))
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

struct McSessionReq<'a> {
    version: Ts005Version,
    group_type: &'a str,
    mc_group_id: u8,
    session_start: u32,
    time_out: u8,
    periodicity: u8,
    frequency: u32,
    dr: u8,
}

fn build_mc_session_req(req: McSessionReq<'_>) -> Result<Vec<u8>> {
    match req.version {
        Ts005Version::V100 => match req.group_type {
            "B" => multicastsetup::v1::Payload::McClassBSessionReq(
                multicastsetup::v1::McClassBSessionReqPayload {
                    mc_group_id_header:
                        multicastsetup::v1::McClassBSessionReqPayloadMcGroupIdHeader {
                            mc_group_id: req.mc_group_id,
                        },
                    session_time: req.session_start - (req.session_start % 128),
                    time_out_periodicity:
                        multicastsetup::v1::McClassBSessionReqPayloadTimeOutPeriodicity {
                            time_out: req.time_out,
                            periodicity: req.periodicity,
                        },
                    dl_frequ: req.frequency,
                    dr: req.dr,
                },
            )
            .to_vec(),
            "C" => multicastsetup::v1::Payload::McClassCSessionReq(
                multicastsetup::v1::McClassCSessionReqPayload {
                    mc_group_id_header:
                        multicastsetup::v1::McClassCSessionReqPayloadMcGroupIdHeader {
                            mc_group_id: req.mc_group_id,
                        },
                    session_time: req.session_start,
                    session_time_out: multicastsetup::v1::McClassCSessionReqPayloadSessionTimeOut {
                        time_out: req.time_out,
                    },
                    dl_frequ: req.frequency,
                    dr: req.dr,
                },
            )
            .to_vec(),
            _ => Err(anyhow!("Invalid multicast-group type")),
        },
        Ts005Version::V200 => match req.group_type {
            "B" => multicastsetup::v2::Payload::McClassBSessionReq(
                multicastsetup::v2::McClassBSessionReqPayload {
                    mc_group_id_header:
                        multicastsetup::v2::McClassBSessionReqPayloadMcGroupIdHeader {
                            mc_group_id: req.mc_group_id,
                        },
                    session_time: req.session_start - (req.session_start % 128),
                    time_out_periodicity:
                        multicastsetup::v2::McClassBSessionReqPayloadTimeOutPeriodicity {
                            time_out: req.time_out,
                            periodicity: req.periodicity,
                        },
                    dl_frequ: req.frequency,
                    dr: req.dr,
                },
            )
            .to_vec(),
            "C" => multicastsetup::v2::Payload::McClassCSessionReq(
                multicastsetup::v2::McClassCSessionReqPayload {
                    mc_group_id_header:
                        multicastsetup::v2::McClassCSessionReqPayloadMcGroupIdHeader {
                            mc_group_id: req.mc_group_id,
                        },
                    session_time: req.session_start,
                    session_time_out: multicastsetup::v2::McClassCSessionReqPayloadSessionTimeOut {
                        time_out: req.time_out,
                    },
                    dl_frequ: req.frequency,
                    dr: req.dr,
                },
            )
            .to_vec(),
            _ => Err(anyhow!("Invalid multicast-group type")),
        },
    }
}

pub async fn enqueue_mc_session_req(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    version: Ts005Version,
    mgd: &multicast::MulticastGroupDevice,
    mg: &multicast::MulticastGroup,
) -> Result<()> {
    let mc_group_id = mgd
        .mc_group_id
        .ok_or_else(|| anyhow!("mc_group_id is not set"))?;
    let session_start = mg
        .mc_session_start
        .ok_or_else(|| anyhow!("mc_session_start is not set"))?
        .to_gps_time()
        .num_seconds()
        % (1 << 32);
    let periodicity = match mg.group_type.as_ref() {
        "B" => mg.class_b_ping_slot_periodicity as u8,
        "C" => 0,
        _ => return Err(anyhow!("Invalid multicast-group type")),
    };

    let pl = build_mc_session_req(McSessionReq {
        version,
        group_type: &mg.group_type,
        mc_group_id: mc_group_id as u8,
        session_start: session_start as u32,
        time_out: mg.mc_session_timeout as u8,
        periodicity,
        frequency: mg.frequency as u32,
        dr: mg.dr as u8,
    })?;

    device_queue::enqueue_item(device_queue::DeviceQueueItem {
        dev_eui: dev.dev_eui,
        f_port: dp.app_layer_params.ts005_f_port.into(),
        data: pl,
        ..Default::default()
    })
    .await?;

    Ok(())
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

async fn handle_multicast_group_setup_ans(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    version: Ts005Version,
    mc_group_id: u8,
    id_error: bool,
) -> Result<()> {
    if let Ok(mgd) = multicast::get_device_by_mc_group_id(&dev.dev_eui, mc_group_id as i16).await {
        let mg = multicast::get(&mgd.multicast_group_id.into()).await?;
        if mg.setup == "TS005" {
            return handle_normal_mc_group_setup_ans(
                dev,
                dp,
                version,
                mgd,
                mg,
                mc_group_id,
                id_error,
            )
            .await;
        }
    }

    handle_fuota_mc_group_setup_ans(dev, mc_group_id, id_error).await
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

async fn handle_multicast_group_delete_ans(
    dev: &device::Device,
    mc_group_id: u8,
    _mc_group_undefined: bool,
) -> Result<()> {
    if let Ok(mgd) = multicast::get_device_by_mc_group_id(&dev.dev_eui, mc_group_id as i16).await {
        let mg = multicast::get(&mgd.multicast_group_id.into()).await?;
        if mg.setup == "TS005" {
            if !mgd.pending_delete {
                warn!(
                    dev_eui = %dev.dev_eui,
                    mc_group_id = mc_group_id,
                    "Ignoring McGroupDeleteAns for active TS005 multicast-group device"
                );
                return Ok(());
            }

            multicast::remove_device(&mg.id.into(), &dev.dev_eui).await?;
            return Ok(());
        }
    }

    warn!(
        dev_eui = %dev.dev_eui,
        mc_group_id = mc_group_id,
        "McGroupDeleteAns does not match any multicast-group for device"
    );
    Ok(())
}

async fn handle_fuota_mc_group_setup_ans(
    dev: &device::Device,
    mc_group_id: u8,
    id_error: bool,
) -> Result<()> {
    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if id_error {
        warn!(
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
    mg: multicast::MulticastGroup,
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

    enqueue_mc_session_req(dev, dp, version, &mgd, &mg).await
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
        warn!(
            dr_error = pl.status_and_mc_group_id.dr_error,
            freq_error = pl.status_and_mc_group_id.freq_error,
            mc_group_undefined = pl.status_and_mc_group_id.mc_group_undefined,
            "McClassBSessionAns contains errors"
        );

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
        warn!(
            dr_error = pl.status_and_mc_group_id.dr_error,
            freq_error = pl.status_and_mc_group_id.freq_error,
            mc_group_undefined = pl.status_and_mc_group_id.mc_group_undefined,
            start_missed = pl.status_and_mc_group_id.start_missed,
            "McClassBSessionAns contains errors"
        );

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
        warn!(
            dr_error = pl.status_and_mc_group_id.dr_error,
            freq_error = pl.status_and_mc_group_id.freq_error,
            mc_group_undefined = pl.status_and_mc_group_id.mc_group_undefined,
            "McClassCSessionAns contains errors"
        );

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
        warn!(
            dr_error = pl.status_and_mc_group_id.dr_error,
            freq_error = pl.status_and_mc_group_id.freq_error,
            mc_group_undefined = pl.status_and_mc_group_id.mc_group_undefined,
            start_missed = pl.status_and_mc_group_id.start_missed,
            "McClassCSessionAns contains errors"
        );

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
    if let Ok(mut mgd) =
        multicast::get_device_by_mc_group_id(&dev.dev_eui, mc_group_id as i16).await
    {
        let mg = multicast::get(&mgd.multicast_group_id.into()).await?;
        if mg.setup == "TS005" {
            if let Some(error_msg) = error_msg {
                mgd.error_msg = error_msg;
            } else {
                mgd.mc_session_completed_at = Some(Utc::now());
                mgd.error_msg = "".into();
            }
            let _ = multicast::update_device(mgd).await?;
            return Ok(());
        }
    }

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;
    if let Some(error_msg) = error_msg {
        fuota_dev.error_msg = error_msg;
    } else {
        fuota_dev.mc_session_completed_at = Some(Utc::now());
    }
    let _ = fuota::update_device(fuota_dev).await?;
    Ok(())
}
