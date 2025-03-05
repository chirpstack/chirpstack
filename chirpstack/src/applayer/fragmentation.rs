use anyhow::Result;
use chrono::Utc;
use tracing::{info, warn};

use crate::storage::fields::device_profile::Ts004Version;
use crate::storage::{device, device_profile, fuota};
use lrwn::applayer::fragmentation;

pub async fn handle_uplink(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    data: &[u8],
) -> Result<()> {
    let version = dp
        .app_layer_params
        .ts004_version
        .ok_or_else(|| anyhow!("Device does not support TS004"))?;

    match version {
        Ts004Version::V100 => handle_uplink_v100(dev, data).await,
    }
}

async fn handle_uplink_v100(dev: &device::Device, data: &[u8]) -> Result<()> {
    let pl = fragmentation::v1::Payload::from_slice(true, data)?;

    match pl {
        fragmentation::v1::Payload::FragSessionSetupAns(pl) => {
            handle_v1_frag_session_setup_ans(dev, pl).await?
        }
        fragmentation::v1::Payload::FragSessionStatusAns(pl) => {
            handle_v1_frag_session_status_ans(dev, pl).await?
        }
        _ => {}
    }

    Ok(())
}

async fn handle_v1_frag_session_setup_ans(
    dev: &device::Device,
    pl: fragmentation::v1::FragSessionSetupAnsPayload,
) -> Result<()> {
    info!("Handling FragSessionSetupAns");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.encoding_unsupported
        | pl.not_enough_memory
        | pl.frag_session_index_not_supported
        | pl.wrong_descriptor
    {
        warn!(
            frag_index = pl.frag_index,
            encoding_unsupported = pl.encoding_unsupported,
            not_enough_memory = pl.not_enough_memory,
            frag_session_index_not_supported = pl.frag_session_index_not_supported,
            wrong_descriptor = pl.wrong_descriptor,
            "FragSessionAns contains errors"
        );
        fuota_dev.error_msg = format!("Error: FragSessionAns response encoding_unsupported={}, not_enough_memory={}, frag_session_index_not_supported={}, wrong_descriptor={}", pl.encoding_unsupported, pl.not_enough_memory, pl.frag_session_index_not_supported, pl.wrong_descriptor);
    } else {
        fuota_dev.frag_session_setup_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;

    Ok(())
}

async fn handle_v1_frag_session_status_ans(
    dev: &device::Device,
    pl: fragmentation::v1::FragSessionStatusAnsPayload,
) -> Result<()> {
    info!("Handling FragSessionStatusAnsPayload");

    let mut fuota_dev = fuota::get_latest_device_by_dev_eui(dev.dev_eui).await?;

    if pl.missing_frag != 0 || pl.status.not_enough_matrix_memory {
        warn!(
            frag_index = pl.received_and_index.frag_index,
            nb_frag_received = pl.received_and_index.nb_frag_received,
            missing_frag = pl.missing_frag,
            not_enough_matrix_memory = pl.status.not_enough_matrix_memory,
            "FragSessionStatusAns contains errors"
        );

        fuota_dev.error_msg = format!("Error: FragSessionStatusAns response nb_frag_received={}, missing_frag={}, not_enough_matrix_memory={}", pl.received_and_index.nb_frag_received, pl.missing_frag, pl.status.not_enough_matrix_memory);
    } else {
        fuota_dev.frag_status_completed_at = Some(Utc::now());
    }

    let _ = fuota::update_device(fuota_dev).await?;

    Ok(())
}
