use std::collections::HashMap;

use anyhow::Result;
use tracing::error;

use crate::config;
use crate::storage::{application, device, device_profile, mac_command, tenant};
use crate::uplink::UplinkFrameSet;
use chirpstack_api::internal;
use lrwn::EUI64;

pub mod configure_fwd_limit;
pub mod ctrl_uplink_list;
pub mod dev_status;
pub mod device_mode_ind;
pub mod device_time;
pub mod end_device_conf;
pub mod filter_list;
pub mod link_adr;
pub mod link_check;
pub mod new_channel;
pub mod notify_new_end_device;
pub mod ping_slot_channel;
pub mod ping_slot_info;
pub mod rejoin_param_setup;
pub mod rekey;
pub mod relay_conf;
pub mod reset;
pub mod rx_param_setup;
pub mod rx_timing_setup;
pub mod tx_param_setup;
pub mod update_uplink_list;

// This returns the mac-commands which must be sent back to the device as response and a bool
// indicating if a downlink must be sent. For some mac-commands, no mac-command answer is required,
// but the device expects a downlink as confirmation, even if the downlink frame is empty.
pub async fn handle_uplink<'a>(
    uplink_frame_set: &UplinkFrameSet,
    cmds: &lrwn::MACCommandSet,
    tenant: &tenant::Tenant,
    app: &application::Application,
    dp: &device_profile::DeviceProfile,
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
) -> Result<(Vec<lrwn::MACCommandSet>, bool)> {
    let conf = config::get();
    if conf.network.mac_commands_disabled {
        return Ok((Vec::new(), false));
    }

    let dev_eui = EUI64::from_slice(&ds.dev_eui)?;
    let mut cids: Vec<lrwn::CID> = Vec::new(); // to maintain the CID order
    let mut blocks: HashMap<lrwn::CID, lrwn::MACCommandSet> = HashMap::new();

    // Group mac-commands in blocks.
    for cmd in &**cmds {
        let cid = cmd.cid();
        match blocks.get_mut(&cid) {
            Some(v) => {
                v.push(cmd.clone());
            }
            None => {
                cids.push(cid);
                blocks.insert(cid, lrwn::MACCommandSet::new(vec![cmd.clone()]));
            }
        }
    }

    // Some mac-commands require a downlink response (not mac-command answer) to confirm that the
    // uplink has been received.
    let mut must_respond_with_downlink = false;
    let mut out: Vec<lrwn::MACCommandSet> = Vec::new();

    // Iterate over mac-commands in order of CID.
    for cid in cids {
        must_respond_with_downlink = must_respond_with_downlink
            || matches!(
                cid,
                lrwn::CID::RxTimingSetupAns | lrwn::CID::RxParamSetupAns
            );

        // Get pending mac-command block, this could return None.
        let pending = match mac_command::get_pending(&dev_eui, cid).await {
            Ok(v) => v,
            Err(e) => {
                error!(dev_eui = %dev_eui, cid = %cid, error = %e, "Get pending mac-command block error");
                continue;
            }
        };

        // Delete the pending mac-command.
        if pending.is_some() {
            if let Err(e) = mac_command::delete_pending(&dev_eui, cid).await {
                error!(dev_eui = %dev_eui, cid = %cid, error = %e, "Delete pending mac-command error");
            }
        }

        // Handle the mac-command, which might return a block to answer the uplink mac-command
        // request.
        let res = match handle(
            uplink_frame_set,
            cid,
            blocks.get(&cid).unwrap(),
            pending.as_ref(),
            tenant,
            app,
            dp,
            dev,
            ds,
        )
        .await
        {
            Ok(v) => v,
            Err(e) => {
                error!(dev_eui = %dev_eui, cid = %cid, error = %e, "Handle mac-command error");
                continue;
            }
        };

        if let Some(block) = res {
            out.push(block);
        }
    }

    Ok((out, must_respond_with_downlink))
}

#[allow(clippy::too_many_arguments)]
async fn handle(
    uplink_frame_set: &UplinkFrameSet,
    cid: lrwn::CID,
    block: &lrwn::MACCommandSet,
    pending_block: Option<&lrwn::MACCommandSet>,
    tenant: &tenant::Tenant,
    app: &application::Application,
    dp: &device_profile::DeviceProfile,
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
) -> Result<Option<lrwn::MACCommandSet>> {
    match cid {
        lrwn::CID::DevStatusAns => {
            dev_status::handle(uplink_frame_set, tenant, app, dp, dev, block).await
        }
        lrwn::CID::DeviceModeInd => device_mode_ind::handle(dev, block).await,
        lrwn::CID::DeviceTimeReq => device_time::handle(uplink_frame_set, dev, block),
        lrwn::CID::LinkADRAns => link_adr::handle(uplink_frame_set, dev, ds, block, pending_block),
        lrwn::CID::LinkCheckReq => link_check::handle(uplink_frame_set, dev, block),
        lrwn::CID::NewChannelAns => new_channel::handle(dev, ds, block, pending_block),
        lrwn::CID::PingSlotChannelAns => ping_slot_channel::handle(dev, ds, block, pending_block),
        lrwn::CID::PingSlotInfoReq => ping_slot_info::handle(dev, ds, block),
        lrwn::CID::RejoinParamSetupAns => rejoin_param_setup::handle(dev, ds, block, pending_block),
        lrwn::CID::RekeyInd => rekey::handle(dev, block),
        lrwn::CID::ResetInd => reset::handle(dev, dp, ds, block),
        lrwn::CID::RxParamSetupAns => rx_param_setup::handle(dev, ds, block, pending_block),
        lrwn::CID::RxTimingSetupAns => rx_timing_setup::handle(dev, ds, block, pending_block),
        lrwn::CID::TxParamSetupAns => tx_param_setup::handle(dev, ds, block, pending_block),
        lrwn::CID::RelayConfAns => relay_conf::handle(dev, ds, block, pending_block),
        lrwn::CID::EndDeviceConfAns => end_device_conf::handle(dev, ds, block, pending_block),
        lrwn::CID::FilterListAns => filter_list::handle(dev, ds, block, pending_block),
        lrwn::CID::UpdateUplinkListAns => update_uplink_list::handle(dev, ds, block, pending_block),
        lrwn::CID::ConfigureFwdLimitAns => {
            configure_fwd_limit::handle(dev, ds, block, pending_block)
        }
        lrwn::CID::NotifyNewEndDeviceReq => {
            notify_new_end_device::handle(tenant, dp, app, dev, block).await
        }
        lrwn::CID::CtrlUplinkListAns => {
            ctrl_uplink_list::handle(dev, ds, block, pending_block).await
        }
        _ => {
            error!(cid = %cid, "Unexpected CID");
            // Return OK, we don't want to break out of the uplink handling.
            Ok(None)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::config;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_handle_uplink() {
        let upfs = UplinkFrameSet {
            uplink_set_id: Uuid::nil(),
            dr: 0,
            ch: 0,
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: Default::default(),
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([0, 0, 0, 0]),
            },
            tx_info: Default::default(),
            rx_info_set: Default::default(),
            gateway_private_up_map: Default::default(),
            gateway_private_down_map: Default::default(),
            gateway_tenant_id_map: Default::default(),
            region_common_name: lrwn::region::CommonName::EU868,
            region_config_id: "eu868".into(),
            roaming_meta_data: None,
        };

        let t: tenant::Tenant = Default::default();
        let app: application::Application = Default::default();
        let dp: device_profile::DeviceProfile = Default::default();
        let dev: device::Device = Default::default();
        let mut ds = internal::DeviceSession {
            dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
            ..Default::default()
        };

        // must respond
        let cmds = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RxTimingSetupAns]);

        let (resp, must_respond) = handle_uplink(&upfs, &cmds, &t, &app, &dp, &dev, &mut ds)
            .await
            .unwrap();
        assert_eq!(0, resp.len());
        assert!(must_respond);

        // mac-commands disabled
        let mut conf = (*config::get()).clone();
        conf.network.mac_commands_disabled = true;
        config::set(conf);

        let (resp, must_respond) = handle_uplink(&upfs, &cmds, &t, &app, &dp, &dev, &mut ds)
            .await
            .unwrap();
        assert_eq!(0, resp.len());
        assert!(!must_respond); // must_respond is false as mac-command is ignored
    }
}
