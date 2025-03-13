use anyhow::Result;
use chrono::Utc;
use tracing::{info, warn};

use crate::storage::fields::device_profile::Ts005Version;
use crate::storage::{device, device_profile, fuota};
use lrwn::applayer::multicastsetup;

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
        Ts005Version::V100 => handle_uplink_v100(dev, data).await,
        Ts005Version::V200 => handle_uplink_v200(dev, data).await,
    }
}

async fn handle_uplink_v100(dev: &device::Device, data: &[u8]) -> Result<()> {
    let pl = multicastsetup::v1::Payload::from_slice(true, data)?;

    match pl {
        multicastsetup::v1::Payload::McGroupSetupAns(pl) => {
            handle_v1_mc_group_setup_ans(dev, pl).await?
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

async fn handle_uplink_v200(dev: &device::Device, data: &[u8]) -> Result<()> {
    let pl = multicastsetup::v2::Payload::from_slice(true, data)?;

    match pl {
        multicastsetup::v2::Payload::McGroupSetupAns(pl) => {
            handle_v2_mc_group_setup_ans(dev, pl).await?
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
    pl: multicastsetup::v1::McGroupSetupAnsPayload,
) -> Result<()> {
    info!("Handling McGroupSetupAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.mc_group_id_header.id_error {
        warn!(
            mc_group_id = pl.mc_group_id_header.mc_group_id,
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

async fn handle_v2_mc_group_setup_ans(
    dev: &device::Device,
    pl: multicastsetup::v2::McGroupSetupAnsPayload,
) -> Result<()> {
    info!("Handling McGroupSetupAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.mc_group_id_header.id_error {
        warn!(
            mc_group_id = pl.mc_group_id_header.mc_group_id,
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

async fn handle_v1_mc_class_b_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v1::McClassBSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassBSessionAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.status_and_mc_group_id.dr_error
        | pl.status_and_mc_group_id.freq_error
        | pl.status_and_mc_group_id.mc_group_undefined
    {
        warn!(
            dr_error = pl.status_and_mc_group_id.dr_error,
            freq_error = pl.status_and_mc_group_id.freq_error,
            mc_group_undefined = pl.status_and_mc_group_id.mc_group_undefined,
            "McClassBSessionAns contains errors"
        );

        fuota_dev.error_msg= format!("Error: McClassBSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
        );
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassBSessionAns OK"
        );
        fuota_dev.mc_session_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;

    Ok(())
}

async fn handle_v2_mc_class_b_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v2::McClassBSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassBSessionAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.status_and_mc_group_id.dr_error
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

        fuota_dev.error_msg= format!("Error: McClassBSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}, start_missed: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
            pl.status_and_mc_group_id.start_missed,
        );
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassBSessionAns OK"
        );
        fuota_dev.mc_session_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;

    Ok(())
}

async fn handle_v1_mc_class_c_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v1::McClassCSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassCSessionAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.status_and_mc_group_id.dr_error
        | pl.status_and_mc_group_id.freq_error
        | pl.status_and_mc_group_id.mc_group_undefined
    {
        warn!(
            dr_error = pl.status_and_mc_group_id.dr_error,
            freq_error = pl.status_and_mc_group_id.freq_error,
            mc_group_undefined = pl.status_and_mc_group_id.mc_group_undefined,
            "McClassCSessionAns contains errors"
        );

        fuota_dev.error_msg = format!("Error: McClassCSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
        );
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassCSessionAns OK"
        );
        fuota_dev.mc_session_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;

    Ok(())
}

async fn handle_v2_mc_class_c_session_ans(
    dev: &device::Device,
    pl: multicastsetup::v2::McClassCSessionAnsPayload,
) -> Result<()> {
    info!("Handling McClassCSessionAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.status_and_mc_group_id.dr_error
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

        fuota_dev.error_msg = format!("Error: McClassCSessionAns response dr_error: {}, freq_error: {}, mc_group_undefined: {}, start_missed: {}",
            pl.status_and_mc_group_id.dr_error,
            pl.status_and_mc_group_id.freq_error,
            pl.status_and_mc_group_id.mc_group_undefined,
            pl.status_and_mc_group_id.start_missed,
        );
    } else {
        info!(
            time_to_start = pl.time_to_start.unwrap_or_default(),
            "McClassCSessionAns OK"
        );
        fuota_dev.mc_session_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;

    Ok(())
}
