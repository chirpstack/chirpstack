use anyhow::Result;
use tracing::info;

use crate::storage::device;

pub fn request(rx1_delay: u8) -> lrwn::MACCommandSet {
    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RxTimingSetupReq(
        lrwn::RxTimingSetupReqPayload { delay: rx1_delay },
    )])
}

pub fn handle(
    dev: &mut device::Device,
    _block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    let dev_eui = dev.dev_eui;
    let ds = dev.get_device_session_mut()?;

    if pending.is_none() {
        return Err(anyhow!("Pending RxTimingSetupReq expected"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let req_pl = if let lrwn::MACCommand::RxTimingSetupReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("RxTimingSetupReq expected"));
    };

    ds.rx1_delay = req_pl.delay as u32;
    info!(dev_eui = %dev_eui, rx1_delay = req_pl.delay, "RxTimingSetupReq acknowledged");

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chirpstack_api::internal;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        rx_timing_setup_req: Option<lrwn::MACCommandSet>,
        rx_timing_setup_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_request() {
        let resp = request(14);
        assert_eq!(
            lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RxTimingSetupReq(
                lrwn::RxTimingSetupReqPayload { delay: 14 }
            )]),
            resp
        );
    }

    #[test]
    fn test_response() {
        let tests = vec![
            Test {
                name: "rx timing setup ack".into(),
                device_session: internal::DeviceSession {
                    rx1_delay: 4,
                    ..Default::default()
                },
                rx_timing_setup_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxTimingSetupReq(lrwn::RxTimingSetupReqPayload { delay: 14 }),
                ])),
                rx_timing_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxTimingSetupAns,
                ]),
                expected_device_session: internal::DeviceSession {
                    rx1_delay: 14,
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "nothing pending".into(),
                device_session: internal::DeviceSession {
                    rx1_delay: 4,
                    ..Default::default()
                },
                rx_timing_setup_req: None,
                rx_timing_setup_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::RxTimingSetupAns,
                ]),
                expected_device_session: internal::DeviceSession {
                    rx1_delay: 4,
                    ..Default::default()
                },
                expected_error: Some("Pending RxTimingSetupReq expected".to_string()),
            },
        ];

        for tst in &tests {
            let mut dev = device::Device {
                device_session: Some(tst.device_session.clone().into()),
                ..Default::default()
            };
            let resp = handle(
                &mut dev,
                &tst.rx_timing_setup_ans,
                tst.rx_timing_setup_req.as_ref(),
            );

            if let Some(e) = &tst.expected_error {
                assert_eq!(true, resp.is_err(), "{}", tst.name);
                assert_eq!(e, &format!("{}", resp.err().unwrap()), "{}", tst.name);
            } else {
                assert_eq!(true, resp.unwrap().is_none());
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
