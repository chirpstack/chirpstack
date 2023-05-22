use anyhow::Result;
use tracing::{info, warn};

use crate::storage::device;
use chirpstack_api::internal;

pub fn request(rx1_dr_offset: u8, rx2_freq: u32, rx2_dr: u8) -> lrwn::MACCommandSet {
    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RxParamSetupReq(
        lrwn::RxParamSetupReqPayload {
            frequency: rx2_freq,
            dl_settings: lrwn::DLSettings {
                opt_neg: false, // not used
                rx2_dr,
                rx1_dr_offset,
            },
        },
    )])
}

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Expected pending RxParamSetupReq"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let ans_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let req_pl = if let lrwn::MACCommand::RxParamSetupReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("RxParamSetupReq expected"));
    };
    let ans_pl = if let lrwn::MACCommand::RxParamSetupAns(pl) = ans_mac {
        pl
    } else {
        return Err(anyhow!("RxParamSetupAns expected"));
    };

    if ans_pl.channel_ack && ans_pl.rx1_dr_offset_ack && ans_pl.rx2_dr_ack {
        // Reset the error-counter.
        ds.mac_command_error_count
            .remove(&(lrwn::CID::RxParamSetupReq.to_u8() as u32));

        ds.rx2_frequency = req_pl.frequency;
        ds.rx2_dr = req_pl.dl_settings.rx2_dr as u32;
        ds.rx1_dr_offset = req_pl.dl_settings.rx1_dr_offset as u32;

        info!(dev_eui = %dev.dev_eui, rx2_freq = req_pl.frequency, rx2_dr = req_pl.dl_settings.rx2_dr, rx1_dr_offset = req_pl.dl_settings.rx1_dr_offset, "RxParamSetupReq acknowledged");
    } else {
        let count = ds
            .mac_command_error_count
            .entry(lrwn::CID::RxParamSetupReq.to_u8() as u32)
            .or_insert(0);
        *count += 1;
        warn!(dev_eui = %dev.dev_eui, rx2_freq = req_pl.frequency, rx2_dr = req_pl.dl_settings.rx2_dr, rx1_dr_offset = req_pl.dl_settings.rx1_dr_offset, "RxParamSetupReq not acknowledged");
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        rx_param_setup_req: Option<lrwn::MACCommandSet>,
        rx_param_setup_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_request() {
        let resp = request(2, 868700000, 5);
        assert_eq!(
            lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RxParamSetupReq(
                lrwn::RxParamSetupReqPayload {
                    frequency: 868700000,
                    dl_settings: lrwn::DLSettings {
                        opt_neg: false, // not used
                        rx2_dr: 5,
                        rx1_dr_offset: 2,
                    },
                }
            )]),
            resp
        );
    }

    #[test]
    fn test_handle() {
        let tests = vec![
            Test {
                name: "rx param setup ack".into(),
                device_session: internal::DeviceSession {
                    rx2_frequency: 868100000,
                    rx2_dr: 0,
                    rx1_dr_offset: 1,
                    mac_command_error_count: [(lrwn::CID::RxParamSetupReq.to_u8() as u32, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                rx_param_setup_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxParamSetupReq(lrwn::RxParamSetupReqPayload {
                        frequency: 868700000,
                        dl_settings: lrwn::DLSettings {
                            rx2_dr: 5,
                            rx1_dr_offset: 2,
                            opt_neg: false,
                        },
                    }),
                ])),
                rx_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxParamSetupAns(lrwn::RxParamSetupAnsPayload {
                        channel_ack: true,
                        rx2_dr_ack: true,
                        rx1_dr_offset_ack: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    rx2_frequency: 868700000,
                    rx2_dr: 5,
                    rx1_dr_offset: 2,
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "rx param setup nack".into(),
                device_session: internal::DeviceSession {
                    rx2_frequency: 868100000,
                    rx2_dr: 0,
                    rx1_dr_offset: 1,
                    mac_command_error_count: [(lrwn::CID::RxParamSetupReq.to_u8() as u32, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                rx_param_setup_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxParamSetupReq(lrwn::RxParamSetupReqPayload {
                        frequency: 868700000,
                        dl_settings: lrwn::DLSettings {
                            rx2_dr: 5,
                            rx1_dr_offset: 2,
                            opt_neg: false,
                        },
                    }),
                ])),
                rx_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxParamSetupAns(lrwn::RxParamSetupAnsPayload {
                        channel_ack: true,
                        rx2_dr_ack: false,
                        rx1_dr_offset_ack: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    rx2_frequency: 868100000,
                    rx2_dr: 0,
                    rx1_dr_offset: 1,
                    mac_command_error_count: [(lrwn::CID::RxParamSetupReq.to_u8() as u32, 2)]
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
            let resp = handle(
                &device::Device {
                    ..Default::default()
                },
                &mut ds,
                &tst.rx_param_setup_ans,
                tst.rx_param_setup_req.as_ref(),
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
