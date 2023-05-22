use anyhow::Result;
use tracing::info;

use crate::storage::device;
use chirpstack_api::internal;

pub fn handle(
    _dev: &device::Device,
    ds: &mut internal::DeviceSession,
    _block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Expected pending UpdateUplinkListReq mac-command"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let req_pl = if let lrwn::MACCommand::UpdateUplinkListReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("UpdateUplinkListReq is expected"));
    };

    if let Some(relay) = &mut ds.relay {
        for rd in &mut relay.devices {
            if req_pl.uplink_list_idx as u32 == rd.index {
                info!(
                    uplink_list_idx = req_pl.uplink_list_idx,
                    dev_addr = %req_pl.dev_addr,
                    "UpdateUplinkListReq acknowledged"
                );
                rd.provisioned = true;
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        update_uplink_list_req: Option<lrwn::MACCommandSet>,
        update_uplink_list_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_response() {
        let tests = vec![
            Test {
                name: "nothing pending".into(),
                device_session: internal::DeviceSession::default(),
                update_uplink_list_req: None,
                update_uplink_list_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::UpdateUplinkListAns,
                ]),
                expected_device_session: internal::DeviceSession::default(),
                expected_error: Some("Expected pending UpdateUplinkListReq mac-command".into()),
            },
            Test {
                name: "update uplink list ack".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            dev_addr: vec![1, 2, 3, 4],
                            dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            root_wor_s_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                            provisioned: false,
                            uplink_limit_bucket_size: 2,
                            uplink_limit_reload_rate: 1,
                            w_f_cnt_last_request: None,
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                update_uplink_list_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::UpdateUplinkListReq(lrwn::UpdateUplinkListReqPayload {
                        uplink_list_idx: 1,
                        uplink_limit: lrwn::UplinkLimitPL {
                            reload_rate: 1,
                            bucket_size: 2,
                        },
                        w_fcnt: 0,
                        dev_addr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        root_wor_s_key: lrwn::AES128Key::from_bytes([
                            1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8,
                        ]),
                    }),
                ])),
                update_uplink_list_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::UpdateUplinkListAns,
                ]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            dev_addr: vec![1, 2, 3, 4],
                            dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            root_wor_s_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                            provisioned: true,
                            uplink_limit_bucket_size: 2,
                            uplink_limit_reload_rate: 1,
                            w_f_cnt_last_request: None,
                        }],
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
                &device::Device {
                    ..Default::default()
                },
                &mut ds,
                &tst.update_uplink_list_ans,
                tst.update_uplink_list_req.as_ref(),
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
