use anyhow::Result;
use tracing::info;

use crate::storage::device;
use chirpstack_api::internal;

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    _block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Expected pending ConfigureFwdLimitReq mac-command"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let req_pl = if let lrwn::MACCommand::ConfigureFwdLimitReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("ConfigureFwdLimitReq is expected"));
    };

    if ds.relay.is_none() {
        ds.relay = Some(internal::Relay::default());
    }

    info!(dev_eui = %dev.dev_eui, "ConfigureFwdLimitReq acknowledged");

    if let Some(relay) = &mut ds.relay {
        relay.join_req_limit_reload_rate = req_pl.reload_rate.join_req_reload_rate as u32;
        relay.notify_limit_reload_rate = req_pl.reload_rate.notify_reload_rate as u32;
        relay.global_uplink_limit_reload_rate = req_pl.reload_rate.global_uplink_reload_rate as u32;
        relay.overall_limit_reload_rate = req_pl.reload_rate.overall_reload_rate as u32;

        relay.join_req_limit_bucket_size = req_pl.load_capacity.join_req_limit_size as u32;
        relay.notify_limit_bucket_size = req_pl.load_capacity.notify_limit_size as u32;
        relay.global_uplink_limit_bucket_size =
            req_pl.load_capacity.global_uplink_limit_size as u32;
        relay.overall_limit_bucket_size = req_pl.load_capacity.overall_limit_size as u32;
    }

    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        configure_fwd_limit_req: Option<lrwn::MACCommandSet>,
        configure_fwd_limit_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_response() {
        let tests = vec![
            Test {
                name: "acked, nothing pending".into(),
                device_session: internal::DeviceSession::default(),
                configure_fwd_limit_req: None,
                configure_fwd_limit_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::ConfigureFwdLimitAns,
                ]),
                expected_device_session: internal::DeviceSession::default(),
                expected_error: Some(
                    "Expected pending ConfigureFwdLimitReq mac-command".to_string(),
                ),
            },
            Test {
                name: "acked".into(),
                device_session: internal::DeviceSession::default(),
                configure_fwd_limit_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::ConfigureFwdLimitReq(lrwn::ConfigureFwdLimitReqPayload {
                        reload_rate: lrwn::FwdLimitReloadRatePL {
                            overall_reload_rate: 10,
                            global_uplink_reload_rate: 20,
                            notify_reload_rate: 30,
                            join_req_reload_rate: 40,
                            reset_limit_counter: lrwn::ResetLimitCounter::NoChange,
                        },
                        load_capacity: lrwn::FwdLimitLoadCapacityPL {
                            overall_limit_size: 0,
                            global_uplink_limit_size: 1,
                            notify_limit_size: 2,
                            join_req_limit_size: 3,
                        },
                    }),
                ])),
                configure_fwd_limit_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::ConfigureFwdLimitAns,
                ]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        overall_limit_reload_rate: 10,
                        global_uplink_limit_reload_rate: 20,
                        notify_limit_reload_rate: 30,
                        join_req_limit_reload_rate: 40,
                        overall_limit_bucket_size: 0,
                        global_uplink_limit_bucket_size: 1,
                        notify_limit_bucket_size: 2,
                        join_req_limit_bucket_size: 3,
                        ..Default::default()
                    }),
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
                &tst.configure_fwd_limit_ans,
                tst.configure_fwd_limit_req.as_ref(),
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
