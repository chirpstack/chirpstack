use anyhow::{Context, Result};
use tracing::{info, warn};

use crate::region;
use crate::storage::device;
use crate::uplink::UplinkFrameSet;
use chirpstack_api::internal;

pub fn handle(
    uplink_frame_set: &UplinkFrameSet,
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Expected pending LinkADRReq mac-command"));
    }

    let region_conf = region::get(&uplink_frame_set.region_config_id)?;

    let mut ch_mask_ack = true;
    let mut dr_ack = true;
    let mut tx_power_ack = true;

    for mac in &**block {
        if let lrwn::MACCommand::LinkADRAns(pl) = mac {
            if !pl.ch_mask_ack {
                ch_mask_ack = false;
            }
            if !pl.dr_ack {
                dr_ack = false;
            }
            if !pl.tx_power_ack {
                tx_power_ack = false;
            }
        }
    }

    let mut link_adr_reqs: Vec<lrwn::LinkADRReqPayload> = Vec::new();

    // we already validated that this is not None
    for mac in &**pending.unwrap() {
        if let lrwn::MACCommand::LinkADRReq(pl) = mac {
            link_adr_reqs.push(pl.clone());
        }
    }

    // as we're sending the same txpower and nbrep for each channel we
    // take the last one
    let link_adr_req = link_adr_reqs.last().unwrap();

    if ch_mask_ack && dr_ack && tx_power_ack {
        // The device acked all request (channel-mask, data-rate and power),
        // in this case we update the device-session with all the requested
        // modifications.

        // reset the error counter
        ds.mac_command_error_count
            .remove(&(lrwn::CID::LinkADRReq.to_u8() as u32));

        let chans = region_conf
            .get_enabled_uplink_channel_indices_for_link_adr_payloads(
                &ds.enabled_uplink_channel_indices
                    .iter()
                    .map(|i| *i as usize)
                    .collect::<Vec<usize>>(),
                &link_adr_reqs,
            )
            .context("Get enabled uplink-channels for LinkADRReq payloads")?;

        // Reset the uplink ADR history table in case one of the TxPower, DR or NbTrans parameters have changed.
        if ds.tx_power_index != link_adr_req.tx_power as u32
            || ds.dr != link_adr_req.dr as u32
            || ds.nb_trans != link_adr_req.redundancy.nb_rep as u32
        {
            ds.uplink_adr_history = vec![];
        }

        ds.tx_power_index = link_adr_req.tx_power as u32;
        ds.dr = link_adr_req.dr as u32;
        ds.nb_trans = link_adr_req.redundancy.nb_rep as u32;
        ds.enabled_uplink_channel_indices = chans.iter().map(|i| *i as u32).collect::<Vec<u32>>();

        info!(dev_eui = %dev.dev_eui, tx_power_index = ds.tx_power_index, dr = ds.dr, nb_trans = ds.nb_trans, enabled_channels = ?ds.enabled_uplink_channel_indices, "LinkADRReq acknowledged");
    } else if !ds.adr && ch_mask_ack {
        // In case the device has ADR disabled, at least it must acknowledge the
        // channel-mask. It does not have to acknowledge the other parameters.
        // See 4.3.1.1 of LoRaWAN 1.0.4 specs.

        // reset the error counter
        ds.mac_command_error_count
            .remove(&(lrwn::CID::LinkADRReq.to_u8() as u32));

        let chans = region_conf
            .get_enabled_uplink_channel_indices_for_link_adr_payloads(
                &ds.enabled_uplink_channel_indices
                    .iter()
                    .map(|i| *i as usize)
                    .collect::<Vec<usize>>(),
                &link_adr_reqs,
            )
            .context("Get enabled uplink-channels for LinkADRReq payloads")?;

        ds.enabled_uplink_channel_indices = chans.iter().map(|i| *i as u32).collect::<Vec<u32>>();
        ds.nb_trans = link_adr_req.redundancy.nb_rep as u32;

        if dr_ack {
            ds.dr = link_adr_req.dr as u32;
        }

        if tx_power_ack {
            ds.tx_power_index = link_adr_req.tx_power as u32;
        }

        info!(dev_eui = %dev.dev_eui, tx_power_index = ds.tx_power_index, dr = ds.dr, nb_trans = ds.nb_trans, enabled_channels = ?ds.enabled_uplink_channel_indices, "LinkADRReq acknowledged (device has ADR disabled)");
    } else {
        // increase the error counter
        let count = ds
            .mac_command_error_count
            .entry(lrwn::CID::LinkADRReq.to_u8() as u32)
            .or_insert(0);
        *count += 1;

        // TODO: remove workaround once all RN2483 nodes have the issue below
        // fixed.
        //
        // This is a workaround for the RN2483 firmware (1.0.3) which sends
        // a nACK on TXPower 0 (this is incorrect behaviour, following the
        // specs). It should ACK and operate at its maximum possible power
        // when TXPower 0 is not supported. See also section 5.2 in the
        // LoRaWAN specs.
        if !tx_power_ack && link_adr_req.tx_power == 0 {
            ds.tx_power_index = 1;
            ds.min_supported_tx_power_index = 1;
        }

        // It is possible that the node does not support all TXPower
        // indices. In this case we set the MaxSupportedTXPowerIndex
        // to the request - 1. If that index is not supported, it will
        // be lowered by 1 at the next nACK.
        if !tx_power_ack && link_adr_req.tx_power > 0 {
            ds.max_supported_tx_power_index = (link_adr_req.tx_power - 1) as u32;
        }

        warn!(dev_eui = %dev.dev_eui, ch_mask_ack = ch_mask_ack, dr_ack = dr_ack, tx_power_ack = tx_power_ack, "LinkADRReq not acknowledged");
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::region;
    use std::collections::HashMap;
    use std::str::FromStr;
    use uuid::Uuid;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        link_adr_req: Option<lrwn::LinkADRReqPayload>,
        link_adr_ans: lrwn::LinkADRAnsPayload,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test() {
        region::set(
            &"eu868",
            lrwn::region::get(lrwn::region::CommonName::EU868, false, false),
        );

        let tests = vec![Test {
            name: "pending request and positive ACK updates tx-power, nbtrans and channels".into(),
            device_session: internal::DeviceSession {
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                mac_command_error_count: [(lrwn::CID::LinkADRReq.to_u8() as u32, 1)]
                    .iter()
                    .cloned()
                    .collect(),
                    uplink_adr_history: vec![internal::UplinkAdrHistory {
                        ..Default::default()
                    }],
                ..Default::default()
            },
            link_adr_req: Some(lrwn::LinkADRReqPayload {
                dr: 5,
                tx_power: 3,
                ch_mask: lrwn::ChMask::from_slice(&[true, true, true]).unwrap(),
                redundancy: lrwn::Redundancy {
                    ch_mask_cntl: 0,
                    nb_rep: 2,
                },
            }),
            link_adr_ans: lrwn::LinkADRAnsPayload {
                ch_mask_ack: true,
                dr_ack: true,
                tx_power_ack: true,
            },
            expected_device_session: internal::DeviceSession {
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1, 2],
                tx_power_index: 3,
                nb_trans: 2,
                dr: 5,
                mac_command_error_count: HashMap::new(),
                uplink_adr_history: vec![],
                ..Default::default()
            },
            expected_error: None,
        }, Test {
            name: "pending request and negative tx-power ack decrements the max allowed tx-power index".into(),
            device_session: internal::DeviceSession{
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                ..Default::default()
            },
            link_adr_req: Some(lrwn::LinkADRReqPayload {
                dr: 5,
                tx_power: 3,
                ch_mask: lrwn::ChMask::from_slice(&[true, true, true]).unwrap(),
                redundancy: lrwn::Redundancy {
                    ch_mask_cntl: 0,
                    nb_rep: 2,
                },
            }),
            link_adr_ans: lrwn::LinkADRAnsPayload {
                ch_mask_ack: true,
                dr_ack: true,
                tx_power_ack: false,
            },
            expected_device_session: internal::DeviceSession {
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                max_supported_tx_power_index: 2,
                mac_command_error_count: [(lrwn::CID::LinkADRReq.to_u8() as u32, 1)]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            },
            expected_error: None,
        }, Test {
            name: "pending request and negative tx-power ack on tx-power 0 sets (min) tx-power to 1".into(),
            device_session: internal::DeviceSession{
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                ..Default::default()
            },
            link_adr_req: Some(lrwn::LinkADRReqPayload {
                dr: 5,
                tx_power: 0,
                ch_mask: lrwn::ChMask::from_slice(&[true, true, true]).unwrap(),
                redundancy: lrwn::Redundancy {
                    ch_mask_cntl: 0,
                    nb_rep: 2,
                },
            }),
            link_adr_ans: lrwn::LinkADRAnsPayload {
                ch_mask_ack: true,
                dr_ack: true,
                tx_power_ack: false,
            },
            expected_device_session: internal::DeviceSession {
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                tx_power_index: 1,
                min_supported_tx_power_index: 1,
                mac_command_error_count: [(lrwn::CID::LinkADRReq.to_u8() as u32, 1)]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            },
            expected_error: None,
        }, Test {
            name: "nothing pending and positive ACK returns an error".into(),
            device_session: internal::DeviceSession{
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                ..Default::default()
            },
            link_adr_req: None,
            link_adr_ans: lrwn::LinkADRAnsPayload {
                ch_mask_ack: true,
                dr_ack: true,
                tx_power_ack: true,
            },
            expected_device_session: internal::DeviceSession{
                adr: true,
                enabled_uplink_channel_indices: vec![0, 1],
                ..Default::default()
            },
            expected_error: Some("Expected pending LinkADRReq mac-command".into()),
        }, Test {
            name: "adr disabled, only DR acknowledged".into(),
            device_session: internal::DeviceSession{
                adr: false,
                enabled_uplink_channel_indices: vec![0, 1],
                tx_power_index: 1,
                dr: 3,
                ..Default::default()
            },
            link_adr_req: Some(lrwn::LinkADRReqPayload {
                dr: 5,
                tx_power: 3,
                ch_mask: lrwn::ChMask::from_slice(&[true, true, true]).unwrap(),
                redundancy: lrwn::Redundancy {
                    ch_mask_cntl: 0,
                    nb_rep: 2,
                },
            }),
            link_adr_ans: lrwn::LinkADRAnsPayload {
                ch_mask_ack: true,
                dr_ack: false,
                tx_power_ack: false,
            },
            expected_device_session: internal::DeviceSession{
                adr: false,
                enabled_uplink_channel_indices: vec![0, 1, 2],
                tx_power_index: 1,
                dr: 3,
                nb_trans: 2,
                ..Default::default()
            },
            expected_error: None,
        }];

        let ufs = UplinkFrameSet {
            uplink_set_id: Uuid::new_v4(),
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
                mic: None,
            },
            tx_info: Default::default(),
            rx_info_set: vec![],
            gateway_private_up_map: HashMap::new(),
            gateway_private_down_map: HashMap::new(),
            gateway_tenant_id_map: HashMap::new(),
            region_common_name: lrwn::region::CommonName::EU868,
            region_config_id: "eu868".into(),
            roaming_meta_data: None,
        };

        for tst in &tests {
            let dev = device::Device {
                dev_eui: lrwn::EUI64::from_str("0102030405060708").unwrap(),
                ..Default::default()
            };
            let mut ds = tst.device_session.clone();
            let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRAns(
                tst.link_adr_ans.clone(),
            )]);
            let pending = match &tst.link_adr_req {
                Some(v) => Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::LinkADRReq(v.clone()),
                ])),
                None => None,
            };

            let res = handle(&ufs, &dev, &mut ds, &block, pending.as_ref());
            if let Some(e) = &tst.expected_error {
                assert_eq!(true, res.is_err(), "{}", tst.name);
                assert_eq!(e, &format!("{}", res.err().unwrap()), "{}", tst.name);
            } else {
                assert_eq!(true, res.unwrap().is_none(), "{}", tst.name);
            }

            assert_eq!(tst.expected_device_session, ds, "{}", tst.name);
        }
    }
}
