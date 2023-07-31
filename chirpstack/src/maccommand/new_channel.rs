use std::collections::HashMap;

use anyhow::Result;
use tracing::{error, info};

use crate::storage::device;
use chirpstack_api::internal;
use lrwn::region::Channel;

pub fn request(
    max_channels: usize,
    current_channels: &HashMap<usize, Channel>,
    wanted_channels: &HashMap<usize, Channel>,
) -> Option<lrwn::MACCommandSet> {
    let mut out: Vec<lrwn::MACCommand> = Vec::new();

    let mut wanted_channel_numbers: Vec<usize> = wanted_channels.keys().cloned().collect();
    wanted_channel_numbers.sort_unstable();

    for i in &wanted_channel_numbers {
        let wanted = wanted_channels.get(i).unwrap(); // we already know the key is in the map
        match current_channels.get(i) {
            Some(current) => {
                // Channel needs to be updated.
                if current.frequency != wanted.frequency
                    || current.min_dr != wanted.min_dr
                    || current.max_dr != wanted.max_dr
                {
                    out.push(lrwn::MACCommand::NewChannelReq(
                        lrwn::NewChannelReqPayload {
                            ch_index: *i as u8,
                            freq: wanted.frequency,
                            min_dr: wanted.min_dr,
                            max_dr: wanted.max_dr,
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
                        min_dr: wanted.min_dr,
                        max_dr: wanted.max_dr,
                    },
                ));
            }
        }
    }

    if out.len() > max_channels {
        out.drain(max_channels..);
    }

    if out.is_empty() {
        return None;
    }

    Some(lrwn::MACCommandSet::new(out))
}

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
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

            ds.extra_uplink_channels.insert(
                req_pl.ch_index as u32,
                internal::DeviceSessionChannel {
                    frequency: req_pl.freq,
                    min_dr: req_pl.min_dr as u32,
                    max_dr: req_pl.max_dr as u32,
                },
            );

            if !ds
                .enabled_uplink_channel_indices
                .contains(&(req_pl.ch_index as u32))
            {
                ds.enabled_uplink_channel_indices
                    .push(req_pl.ch_index as u32);
            }

            info!(dev_eui = %dev.dev_eui, freq = req_pl.freq, channel = req_pl.ch_index, min_dr = req_pl.min_dr, max_dr = req_pl.max_dr, "NewChannelReq acknowledged");
        } else {
            let count = ds
                .mac_command_error_count
                .entry(lrwn::CID::NewChannelReq.to_u8() as u32)
                .or_insert(0);
            *count += 1;

            error!(dev_eui = %dev.dev_eui, freq = req_pl.freq, channel = req_pl.ch_index, min_dr = req_pl.min_dr, max_dr = req_pl.max_dr, dr_range_ok = ans_pl.dr_range_ok, channel_freq_ok = ans_pl.channel_freq_ok, "NewChannelReq not acknowledged");
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
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            min_dr: 3,
                            max_dr: 5,
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
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        6,
                        Channel {
                            frequency: 868900000,
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        7,
                        Channel {
                            frequency: 869000000,
                            min_dr: 3,
                            max_dr: 5,
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
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868700000,
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            min_dr: 3,
                            max_dr: 5,
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
                            min_dr: 3,
                            max_dr: 5,
                            ..Default::default()
                        },
                    ),
                    (
                        4,
                        Channel {
                            frequency: 868650000,
                            min_dr: 2,
                            max_dr: 4,
                            ..Default::default()
                        },
                    ),
                    (
                        5,
                        Channel {
                            frequency: 868800000,
                            min_dr: 3,
                            max_dr: 5,
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
        ];

        for tst in &tests {
            println!("> {}", tst.name);
            let resp = request(3, &tst.current_channels, &tst.wanted_channels);
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
                            min_dr: 3,
                            max_dr: 5,
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
                            min_dr: 3,
                            max_dr: 5,
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
                            min_dr: 3,
                            max_dr: 5,
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
            let mut ds = tst.device_session.clone();

            let res = handle(
                &device::Device {
                    ..Default::default()
                },
                &mut ds,
                &tst.new_channel_ans,
                tst.new_channel_req.as_ref(),
            );

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
