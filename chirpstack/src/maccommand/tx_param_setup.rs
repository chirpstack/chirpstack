use anyhow::Result;
use tracing::info;

use crate::storage::device;
use chirpstack_api::internal;

pub fn request(
    uplink_dwell_time_400ms: bool,
    downlink_dwell_time_400ms: bool,
    max_eirp: u8,
) -> lrwn::MACCommandSet {
    let uplink_dwell_time = match uplink_dwell_time_400ms {
        true => lrwn::DwellTime::Limit400ms,
        false => lrwn::DwellTime::NoLimit,
    };

    let downlink_dwell_time = match downlink_dwell_time_400ms {
        true => lrwn::DwellTime::Limit400ms,
        false => lrwn::DwellTime::NoLimit,
    };

    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::TxParamSetupReq(
        lrwn::TxParamSetupReqPayload {
            downlink_dwell_time,
            uplink_dwell_time,
            max_eirp,
        },
    )])
}

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    _block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Expected pending TxParamSetupReq"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let req_pl = if let lrwn::MACCommand::TxParamSetupReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("TxParamSetupReq expected"));
    };

    ds.uplink_dwell_time_400ms = req_pl.uplink_dwell_time == lrwn::DwellTime::Limit400ms;
    ds.downlink_dwell_time_400ms = req_pl.downlink_dwell_time == lrwn::DwellTime::Limit400ms;
    ds.uplink_max_eirp_index = req_pl.max_eirp as u32;

    info!(dev_eui = %dev.dev_eui, uplink_dwell_time_400ms = ds.uplink_dwell_time_400ms, downlink_dwell_time_400ms = ds.downlink_dwell_time_400ms, uplink_max_eirp_index = ds.uplink_max_eirp_index, "TxParamSetupReq acknowledged");
    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        tx_param_setup_req: Option<lrwn::MACCommandSet>,
        tx_param_setup_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_request() {
        let resp = request(true, false, 10);
        assert_eq!(
            lrwn::MACCommandSet::new(vec![lrwn::MACCommand::TxParamSetupReq(
                lrwn::TxParamSetupReqPayload {
                    downlink_dwell_time: lrwn::DwellTime::NoLimit,
                    uplink_dwell_time: lrwn::DwellTime::Limit400ms,
                    max_eirp: 10,
                }
            )]),
            resp
        );
    }

    #[test]
    fn test_handle() {
        let tests = vec![
            Test {
                name: "request acked".into(),
                device_session: internal::DeviceSession {
                    uplink_dwell_time_400ms: false,
                    downlink_dwell_time_400ms: true,
                    uplink_max_eirp_index: 10,
                    ..Default::default()
                },
                tx_param_setup_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::TxParamSetupReq(lrwn::TxParamSetupReqPayload {
                        uplink_dwell_time: lrwn::DwellTime::Limit400ms,
                        downlink_dwell_time: lrwn::DwellTime::NoLimit,
                        max_eirp: 14,
                    }),
                ])),
                tx_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::TxParamSetupAns,
                ]),
                expected_device_session: internal::DeviceSession {
                    uplink_dwell_time_400ms: true,
                    downlink_dwell_time_400ms: false,
                    uplink_max_eirp_index: 14,
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "pending missing".into(),
                device_session: internal::DeviceSession {
                    uplink_dwell_time_400ms: false,
                    downlink_dwell_time_400ms: true,
                    uplink_max_eirp_index: 10,
                    ..Default::default()
                },
                tx_param_setup_req: None,
                tx_param_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::TxParamSetupAns,
                ]),
                expected_device_session: internal::DeviceSession {
                    uplink_dwell_time_400ms: false,
                    downlink_dwell_time_400ms: true,
                    uplink_max_eirp_index: 10,
                    ..Default::default()
                },
                expected_error: Some("Expected pending TxParamSetupReq".to_string()),
            },
        ];

        for tst in &tests {
            let mut ds = tst.device_session.clone();
            let resp = handle(
                &device::Device {
                    ..Default::default()
                },
                &mut ds,
                &tst.tx_param_setup_ans,
                tst.tx_param_setup_req.as_ref(),
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
