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
        return Err(anyhow!("Expected pending EndDeviceConfReq mac-command"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let req_pl = if let lrwn::MACCommand::EndDeviceConfReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("EndDeviceConfReq is expected"));
    };

    let ans_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let ans_pl = if let lrwn::MACCommand::EndDeviceConfAns(pl) = ans_mac {
        pl
    } else {
        return Err(anyhow!("EndDeviceConfAns is expected"));
    };

    if ds.relay.is_none() {
        ds.relay = Some(internal::Relay::default());
    }

    if ans_pl.second_ch_freq_ack
        && ans_pl.second_ch_dr_ack
        && ans_pl.second_ch_idx_ack
        && ans_pl.backoff_ack
    {
        info!(dev_eui = %dev.dev_eui, "EndDeviceConfReq acknowledged");

        if let Some(relay) = &mut ds.relay {
            relay.ed_activation_mode =
                req_pl.activation_relay_mode.relay_mode_activation.to_u8() as u32;
            relay.ed_smart_enable_level = req_pl.activation_relay_mode.smart_enable_level as u32;

            relay.second_channel_ack_offset =
                req_pl.channel_settings_ed.second_ch_ack_offset as u32;
            relay.second_channel_dr = req_pl.channel_settings_ed.second_ch_dr as u32;
            relay.ed_back_off = req_pl.channel_settings_ed.backoff as u32;

            relay.second_channel_freq = req_pl.second_ch_freq;
        }
    } else {
        error!(
            dev_eui = %dev.dev_eui,
            second_ch_freq_ack = ans_pl.second_ch_freq_ack,
            second_ch_dr_ack = ans_pl.second_ch_dr_ack,
            second_ch_idx_ack = ans_pl.second_ch_idx_ack,
            backoff_ack = ans_pl.backoff_ack,
            "EndDeviceConfReq not acknowledged"
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
        end_device_conf_req: Option<lrwn::MACCommandSet>,
        end_device_conf_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_response() {
        let tests = vec![
            Test {
                name: "acked, nothing pending".into(),
                device_session: internal::DeviceSession::default(),
                end_device_conf_req: None,
                end_device_conf_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::EndDeviceConfAns(lrwn::EndDeviceConfAnsPayload {
                        second_ch_freq_ack: true,
                        second_ch_dr_ack: true,
                        second_ch_idx_ack: true,
                        backoff_ack: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession::default(),
                expected_error: Some("Expected pending EndDeviceConfReq mac-command".into()),
            },
            Test {
                name: "ackend".into(),
                device_session: internal::DeviceSession::default(),
                end_device_conf_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::EndDeviceConfReq(lrwn::EndDeviceConfReqPayload {
                        activation_relay_mode: lrwn::ActivationRelayMode {
                            relay_mode_activation: lrwn::RelayModeActivation::EnableRelayMode,
                            smart_enable_level: 2,
                        },
                        channel_settings_ed: lrwn::ChannelSettingsED {
                            second_ch_ack_offset: 3,
                            second_ch_dr: 4,
                            second_ch_idx: 1,
                            backoff: 5,
                        },
                        second_ch_freq: 868300000,
                    }),
                ])),
                end_device_conf_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::EndDeviceConfAns(lrwn::EndDeviceConfAnsPayload {
                        second_ch_freq_ack: true,
                        second_ch_dr_ack: true,
                        second_ch_idx_ack: true,
                        backoff_ack: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        ed_activation_mode: 1,
                        ed_smart_enable_level: 2,
                        second_channel_ack_offset: 3,
                        second_channel_dr: 4,
                        ed_back_off: 5,
                        second_channel_freq: 868300000,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "nacked".into(),
                device_session: internal::DeviceSession::default(),
                end_device_conf_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::EndDeviceConfReq(lrwn::EndDeviceConfReqPayload {
                        activation_relay_mode: lrwn::ActivationRelayMode {
                            relay_mode_activation: lrwn::RelayModeActivation::EnableRelayMode,
                            smart_enable_level: 2,
                        },
                        channel_settings_ed: lrwn::ChannelSettingsED {
                            second_ch_ack_offset: 3,
                            second_ch_dr: 4,
                            second_ch_idx: 1,
                            backoff: 5,
                        },
                        second_ch_freq: 868300000,
                    }),
                ])),
                end_device_conf_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::EndDeviceConfAns(lrwn::EndDeviceConfAnsPayload {
                        second_ch_freq_ack: false,
                        second_ch_dr_ack: true,
                        second_ch_idx_ack: true,
                        backoff_ack: true,
                    }),
                ]),
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
                &tst.end_device_conf_ans,
                tst.end_device_conf_req.as_ref(),
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
