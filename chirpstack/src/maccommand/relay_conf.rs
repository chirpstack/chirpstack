use anyhow::Result;
use tracing::{error, info};

use crate::storage::device;
use chirpstack_api::internal;

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Expected pending RelayConfReq mac-command"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let req_pl = if let lrwn::MACCommand::RelayConfReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("RelayConfReq is expected"));
    };

    let ans_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let ans_pl = if let lrwn::MACCommand::RelayConfAns(pl) = ans_mac {
        pl
    } else {
        return Err(anyhow!("RelayConfAns is expected"));
    };

    if ds.relay.is_none() {
        ds.relay = Some(internal::Relay::default());
    }

    if ans_pl.second_ch_freq_ack
        && ans_pl.second_ch_ack_offset_ack
        && ans_pl.second_ch_dr_ack
        && ans_pl.second_ch_idx_ack
        && ans_pl.default_ch_idx_ack
        && ans_pl.cad_periodicity_ack
    {
        info!(dev_eui = %dev.dev_eui, "RelayConfReq acknowledged");

        if let Some(relay) = &mut ds.relay {
            relay.enabled = req_pl.channel_settings_relay.start_stop == 1;
            relay.cad_periodicity = req_pl.channel_settings_relay.cad_periodicity as u32;
            relay.default_channel_index = req_pl.channel_settings_relay.default_ch_idx as u32;
            relay.second_channel_freq = req_pl.second_ch_freq;
            relay.second_channel_dr = req_pl.channel_settings_relay.second_ch_dr as u32;
            relay.second_channel_ack_offset =
                req_pl.channel_settings_relay.second_ch_ack_offset as u32;
        }
    } else {
        error!(
            dev_eui = %dev.dev_eui,
            second_ch_ack_offset_ack = ans_pl.second_ch_ack_offset_ack,
            second_ch_dr_ack = ans_pl.second_ch_dr_ack,
            second_ch_idx_ack = ans_pl.second_ch_idx_ack,
            default_ch_idx_ack = ans_pl.default_ch_idx_ack,
            cad_periodicity_ack = ans_pl.cad_periodicity_ack,
            "RelayConfReq not acknowledged"
        );
    }

    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        relay_conf_req: Option<lrwn::MACCommandSet>,
        relay_conf_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_response() {
        let tests = vec![
            Test {
                name: "acked, nothing pending".into(),
                device_session: internal::DeviceSession::default(),
                relay_conf_req: None,
                relay_conf_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RelayConfAns(
                    lrwn::RelayConfAnsPayload {
                        second_ch_freq_ack: true,
                        second_ch_ack_offset_ack: true,
                        second_ch_dr_ack: true,
                        second_ch_idx_ack: true,
                        default_ch_idx_ack: true,
                        cad_periodicity_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession::default(),
                expected_error: Some("Expected pending RelayConfReq mac-command".into()),
            },
            Test {
                name: "ackend".into(),
                device_session: internal::DeviceSession::default(),
                relay_conf_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RelayConfReq(lrwn::RelayConfReqPayload {
                        channel_settings_relay: lrwn::ChannelSettingsRelay {
                            second_ch_ack_offset: 3,
                            second_ch_dr: 2,
                            second_ch_idx: 1,
                            default_ch_idx: 0,
                            cad_periodicity: 4,
                            start_stop: 1,
                        },
                        second_ch_freq: 868300000,
                    }),
                ])),
                relay_conf_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RelayConfAns(
                    lrwn::RelayConfAnsPayload {
                        second_ch_freq_ack: true,
                        second_ch_ack_offset_ack: true,
                        second_ch_dr_ack: true,
                        second_ch_idx_ack: true,
                        default_ch_idx_ack: true,
                        cad_periodicity_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        enabled: true,
                        cad_periodicity: 4,
                        default_channel_index: 0,
                        second_channel_freq: 868300000,
                        second_channel_dr: 2,
                        second_channel_ack_offset: 3,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "nackend".into(),
                device_session: internal::DeviceSession::default(),
                relay_conf_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RelayConfReq(lrwn::RelayConfReqPayload {
                        channel_settings_relay: lrwn::ChannelSettingsRelay {
                            second_ch_ack_offset: 3,
                            second_ch_dr: 2,
                            second_ch_idx: 1,
                            default_ch_idx: 0,
                            cad_periodicity: 4,
                            start_stop: 1,
                        },
                        second_ch_freq: 868300000,
                    }),
                ])),
                relay_conf_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RelayConfAns(
                    lrwn::RelayConfAnsPayload {
                        second_ch_freq_ack: false,
                        second_ch_ack_offset_ack: true,
                        second_ch_dr_ack: true,
                        second_ch_idx_ack: true,
                        default_ch_idx_ack: true,
                        cad_periodicity_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay::default()),
                    ..Default::default()
                },
                expected_error: None,
            },
        ];

        for tst in &tests {
            let mut ds = tst.device_session.clone();
            let resp = handle(
                &device::Device::default(),
                &mut ds,
                &tst.relay_conf_ans,
                tst.relay_conf_req.as_ref(),
            );

            if let Some(e) = &tst.expected_error {
                assert_eq!(true, resp.is_err(), "{}", tst.name);
                assert_eq!(e, &format!("{}", resp.err().unwrap()), "{}", tst.name);
            } else {
                assert_eq!(true, resp.unwrap().is_none());
            }

            assert_eq!(tst.expected_device_session, ds, "{}", tst.name);
        }
    }
}
