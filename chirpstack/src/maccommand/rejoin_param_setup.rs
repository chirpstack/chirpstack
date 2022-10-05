use anyhow::Result;
use tracing::{info, warn};

use crate::storage::device;
use chirpstack_api::internal;

pub fn request(max_time_n: u8, max_count_n: u8) -> lrwn::MACCommandSet {
    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RejoinParamSetupReq(
        lrwn::RejoinParamSetupReqPayload {
            max_time_n,
            max_count_n,
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
        return Err(anyhow!("Pending RejoinParamSetupReq expected"));
    }

    let ans_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let req_pl = if let lrwn::MACCommand::RejoinParamSetupReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("RejoinParamSetupReq expected"));
    };
    let ans_pl = if let lrwn::MACCommand::RejoinParamSetupAns(pl) = ans_mac {
        pl
    } else {
        return Err(anyhow!("RejoinParamSetupAns expected"));
    };

    ds.rejoin_request_enabled = true;
    ds.rejoin_request_max_count_n = req_pl.max_count_n as u32;
    ds.rejoin_request_max_time_n = req_pl.max_time_n as u32;

    if ans_pl.time_ok {
        info!(dev_eui = %dev.dev_eui, time_ok = ans_pl.time_ok, "RejoinParamSetupReq acknowledged");
    } else {
        warn!(dev_eui = %dev.dev_eui, time_ok = ans_pl.time_ok, "RejoinParamSetupReq acknowledged");
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        rejoin_param_setup_req: Option<lrwn::MACCommandSet>,
        rejoin_param_setup_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_request() {
        let resp = request(5, 10);
        assert_eq!(
            lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RejoinParamSetupReq(
                lrwn::RejoinParamSetupReqPayload {
                    max_time_n: 5,
                    max_count_n: 10,
                }
            ),]),
            resp
        );
    }

    #[test]
    fn test_handle() {
        let tests = vec![
            Test {
                name: "acknowledged with time ok".into(),
                device_session: internal::DeviceSession {
                    rejoin_request_max_count_n: 1,
                    rejoin_request_max_time_n: 2,
                    ..Default::default()
                },
                rejoin_param_setup_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RejoinParamSetupReq(lrwn::RejoinParamSetupReqPayload {
                        max_count_n: 10,
                        max_time_n: 5,
                    }),
                ])),
                rejoin_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RejoinParamSetupAns(lrwn::RejoinParamSetupAnsPayload {
                        time_ok: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    rejoin_request_enabled: true,
                    rejoin_request_max_count_n: 10,
                    rejoin_request_max_time_n: 5,
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "acknowledged with time not ok".into(),
                device_session: internal::DeviceSession {
                    rejoin_request_max_count_n: 1,
                    rejoin_request_max_time_n: 2,
                    ..Default::default()
                },
                rejoin_param_setup_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RejoinParamSetupReq(lrwn::RejoinParamSetupReqPayload {
                        max_count_n: 10,
                        max_time_n: 5,
                    }),
                ])),
                rejoin_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RejoinParamSetupAns(lrwn::RejoinParamSetupAnsPayload {
                        time_ok: false,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    rejoin_request_enabled: true,
                    rejoin_request_max_count_n: 10,
                    rejoin_request_max_time_n: 5,
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "acknowledged, but nothing pending".into(),
                device_session: internal::DeviceSession {
                    rejoin_request_max_count_n: 1,
                    rejoin_request_max_time_n: 2,
                    ..Default::default()
                },
                rejoin_param_setup_req: None,
                rejoin_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RejoinParamSetupAns(lrwn::RejoinParamSetupAnsPayload {
                        time_ok: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    rejoin_request_max_count_n: 1,
                    rejoin_request_max_time_n: 2,
                    ..Default::default()
                },
                expected_error: Some("Pending RejoinParamSetupReq expected".to_string()),
            },
        ];

        for tst in &tests {
            let mut ds = tst.device_session.clone();
            let resp = handle(
                &device::Device {
                    ..Default::default()
                },
                &mut ds,
                &tst.rejoin_param_setup_ans,
                tst.rejoin_param_setup_req.as_ref(),
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
