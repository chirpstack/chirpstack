use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use tracing::{info, warn};

use crate::storage::device;
use chirpstack_api::internal;
use lrwn::region::Channel;

pub fn request(
    max_channels: usize,
    current_channels: &HashMap<usize, Channel>,
    wanted_channels: &HashMap<usize, Channel>,
    region_conf: Arc<Box<dyn lrwn::region::Region + Send + Sync>>,
) -> Result<Option<lrwn::MACCommandSet>> {
    let mut out: Vec<lrwn::MACCommand> = Vec::new();

    let mut wanted_channel_numbers: Vec<usize> = wanted_channels.keys().cloned().collect();
    wanted_channel_numbers.sort_unstable();

    for i in &wanted_channel_numbers {
        let wanted = wanted_channels.get(i).unwrap(); // we already know the key is in the map
        let (min_dr, max_dr) = region_conf.get_new_channel_req_dr_range(&wanted.data_rates)?;

        match current_channels.get(i) {
            Some(current) => {
                // Channel needs to be updated.
                if current.frequency != wanted.frequency || current.data_rates != wanted.data_rates
                {
                    out.push(lrwn::MACCommand::NewChannelReq(
                        lrwn::NewChannelReqPayload {
                            ch_index: *i as u8,
                            freq: wanted.frequency,
                            min_dr,
                            max_dr,
                        },
                    ));
                }
            }
            None => {
                // Channel needs to be added.
                out.push(lrwn::MACCommand::NewChannelReq(
                    lrwn::NewChannelReqPayload {
                        ch_index: *i as u8,
                        freq: wanted.frequency,
                        min_dr,
                        max_dr,
                    },
                ));
            }
        }
    }

    if out.len() > max_channels {
        out.drain(max_channels..);
    }

    if out.is_empty() {
        return Ok(None);
    }

    Ok(Some(lrwn::MACCommandSet::new(out)))
}

pub fn handle(
    dev: &mut device::Device,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
    region_conf: Arc<Box<dyn lrwn::region::Region + Send + Sync>>,
) -> Result<Option<lrwn::MACCommandSet>> {
    let dev_eui = dev.dev_eui;
    let ds = dev.get_device_session_mut()?;

    if pending.is_none() {
        return Err(anyhow!("Expected pending NewChannelReq"));
    }

    let block_macs = &**block;
    let pending_macs = &**pending.unwrap();

    if block_macs.len() != pending_macs.len() {
        return Err(anyhow!(
            "Requested number of NewChannelReq items does not match NewChannelAns items"
        ));
    }

    for (i, ans_mac) in block_macs.iter().enumerate() {
        let ans_pl = if let lrwn::MACCommand::NewChannelAns(ans_pl) = &ans_mac {
            ans_pl
        } else {
            return Err(anyhow!("Expected NewChannelAns"));
        };

        let req_pl = if let lrwn::MACCommand::NewChannelReq(req_pl) = &pending_macs[i] {
            req_pl
        } else {
            return Err(anyhow!("Expected NewChannelReq"));
        };

        if ans_pl.channel_freq_ok && ans_pl.dr_range_ok {
            // Reset the error-counter.
            ds.mac_command_error_count
                .remove(&(lrwn::CID::NewChannelReq.to_u8() as u32));

            let data_rates = region_conf
                .get_data_rates_for_new_channel_req_dr_range(req_pl.min_dr, req_pl.max_dr)?;

            ds.extra_uplink_channels.insert(
                req_pl.ch_index as u32,
                internal::DeviceSessionChannel {
                    frequency: req_pl.freq,
                    data_rates: data_rates.into_iter().map(|v| v as u32).collect(),
                    ..Default::default()
                },
            );

            if !ds
                .enabled_uplink_channel_indices
                .contains(&(req_pl.ch_index as u32))
            {
                ds.enabled_uplink_channel_indices
                    .push(req_pl.ch_index as u32);
            }

            info!(dev_eui = %dev_eui, freq = req_pl.freq, channel = req_pl.ch_index, min_dr = req_pl.min_dr, max_dr = req_pl.max_dr, "NewChannelReq acknowledged");
        } else {
            let count = ds
                .mac_command_error_count
                .entry(lrwn::CID::NewChannelReq.to_u8() as u32)
                .or_insert(0);
            *count += 1;

            warn!(
                dev_eui = %dev_eui,
                freq = req_pl.freq,
                channel = req_pl.ch_index,
                min_dr = req_pl.min_dr,
                max_dr = req_pl.max_dr,
                dr_range_ok = ans_pl.dr_range_ok,
                channel_freq_ok = ans_pl.channel_freq_ok,
                "NewChannelReq not acknowledged");
        }
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct RequestTest {
        name: String,
        current_channels: HashMap<usize, Channel>,
        wanted_channels: HashMap<usize, Channel>,
        expected_mac_commands: Option<lrwn::MACCommandSet>,
    }

    struct AnsTest {
        name: String,
        device_session: internal::DeviceSession,
        new_channel_req: Option<lrwn::MACCommandSet>,
        new_channel_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_request() {
        let tests = vec![
            RequestTest {
                name: "adding new channel".into(),
                current_channels: [
                    (
                        3,
                        Channel {
                            frequency: 868600000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                wanted_channels: [
                    (
                        3,
                        Channel {
                            frequency: 868600000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        6,
                        Channel {
                            frequency: 868900000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        7,
                        Channel {
                            frequency: 869000000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                expected_mac_commands: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 6,
                        freq: 868900000,
                        min_dr: 3,
                        max_dr: 5,
                    }),
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 7,
                        freq: 869000000,
                        min_dr: 3,
                        max_dr: 5,
                    }),
                ])),
            },
            RequestTest {
                name: "modifying channel".into(),
                current_channels: [
                    (
                        3,
                        Channel {
                            frequency: 868600000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                wanted_channels: [
                    (
                        3,
                        Channel {
                            frequency: 868600000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868650000,
                            data_rates: vec![2, 3, 4],
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                expected_mac_commands: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 4,
                        freq: 868650000,
                        min_dr: 2,
                        max_dr: 4,
                    }),
                ])),
            },
            RequestTest {
                name: "add SF12 - SF5 channel".into(),
                current_channels: [
                    (
                        3,
                        Channel {
                            frequency: 868600000,
                            data_rates: vec![0, 1, 2, 3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            data_rates: vec![0, 1, 2, 3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            data_rates: vec![0, 1, 2, 3, 4, 5],
                            ..Default::default()
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                wanted_channels: [
                    (
                        3,
                        Channel {
                            frequency: 868600000,
                            data_rates: vec![0, 1, 2, 3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            data_rates: vec![0, 1, 2, 3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            data_rates: vec![0, 1, 2, 3, 4, 5],
                            ..Default::default()
                        },
                    ),
                    (
                        6,
                        Channel {
                            frequency: 868900000,
                            data_rates: vec![0, 1, 2, 3, 4, 5, 12, 13],
                            ..Default::default()
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
                expected_mac_commands: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 6,
                        freq: 868900000,
                        min_dr: 1,
                        max_dr: 0,
                    }),
                ])),
            },
        ];

        for tst in &tests {
            let region_conf = lrwn::region::get(lrwn::region::CommonName::EU868, false, false);
            let region_conf = Arc::new(region_conf);

            println!("> {}", tst.name);
            let resp =
                request(3, &tst.current_channels, &tst.wanted_channels, region_conf).unwrap();
            assert_eq!(tst.expected_mac_commands, resp);
        }
    }

    #[test]
    fn test_handle() {
        let tests = vec![
            AnsTest {
                name: "add new channels (ack)".into(),
                device_session: internal::DeviceSession {
                    enabled_uplink_channel_indices: vec![0, 1, 2],
                    mac_command_error_count: [(lrwn::CID::NewChannelReq.to_u8() as u32, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                new_channel_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::NewChannelAns(
                    lrwn::NewChannelAnsPayload {
                        channel_freq_ok: true,
                        dr_range_ok: true,
                    },
                )]),
                new_channel_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 3,
                        freq: 868600000,
                        min_dr: 3,
                        max_dr: 5,
                    }),
                ])),
                expected_device_session: internal::DeviceSession {
                    enabled_uplink_channel_indices: vec![0, 1, 2, 3],
                    extra_uplink_channels: [(
                        3,
                        internal::DeviceSessionChannel {
                            frequency: 868600000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    )]
                    .iter()
                    .cloned()
                    .collect(),
                    ..Default::default()
                },
                expected_error: None,
            },
            AnsTest {
                name: "add new channels (nack)".into(),
                device_session: internal::DeviceSession {
                    enabled_uplink_channel_indices: vec![0, 1, 2],
                    ..Default::default()
                },
                new_channel_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::NewChannelAns(
                    lrwn::NewChannelAnsPayload {
                        channel_freq_ok: false,
                        dr_range_ok: true,
                    },
                )]),
                new_channel_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 3,
                        freq: 868600000,
                        min_dr: 3,
                        max_dr: 5,
                    }),
                ])),
                expected_device_session: internal::DeviceSession {
                    enabled_uplink_channel_indices: vec![0, 1, 2],
                    mac_command_error_count: [(lrwn::CID::NewChannelReq.to_u8() as u32, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                expected_error: None,
            },
            AnsTest {
                name: "modify existing channels".into(),
                device_session: internal::DeviceSession {
                    enabled_uplink_channel_indices: vec![0, 1, 2, 3],
                    extra_uplink_channels: [(
                        3,
                        internal::DeviceSessionChannel {
                            frequency: 868700000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    )]
                    .iter()
                    .cloned()
                    .collect(),
                    ..Default::default()
                },
                new_channel_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::NewChannelAns(
                    lrwn::NewChannelAnsPayload {
                        channel_freq_ok: true,
                        dr_range_ok: true,
                    },
                )]),
                new_channel_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::NewChannelReq(lrwn::NewChannelReqPayload {
                        ch_index: 3,
                        freq: 868600000,
                        min_dr: 3,
                        max_dr: 5,
                    }),
                ])),
                expected_device_session: internal::DeviceSession {
                    enabled_uplink_channel_indices: vec![0, 1, 2, 3],
                    extra_uplink_channels: [(
                        3,
                        internal::DeviceSessionChannel {
                            frequency: 868600000,
                            data_rates: vec![3, 4, 5],
                            ..Default::default()
                        },
                    )]
                    .iter()
                    .cloned()
                    .collect(),
                    ..Default::default()
                },
                expected_error: None,
            },
        ];

        for tst in &tests {
            let mut dev = device::Device {
                device_session: Some(tst.device_session.clone().into()),
                ..Default::default()
            };

            let region_config = lrwn::region::get(lrwn::region::CommonName::EU868, false, false);
            let region_config = Arc::new(region_config);

            let res = handle(
                &mut dev,
                &tst.new_channel_ans,
                tst.new_channel_req.as_ref(),
                region_config,
            );

            if let Some(e) = &tst.expected_error {
                assert!(res.is_err(), "{}", tst.name);
                assert_eq!(e, &format!("{}", res.err().unwrap()), "{}", tst.name);
            } else {
                assert!(res.unwrap().is_none(), "{}", tst.name);
            }

            assert_eq!(
                &tst.expected_device_session,
                dev.get_device_session().unwrap(),
                "{}",
                tst.name
            );
        }
    }
}
