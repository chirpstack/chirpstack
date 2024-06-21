use std::iter::zip;

use anyhow::Result;
use tracing::{info, warn};

use crate::storage::device;
use lrwn::EUI64;

pub async fn handle(
    dev: &mut device::Device,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    let dev_eui = dev.dev_eui;
    let ds = dev.get_device_session_mut()?;

    if pending.is_none() {
        return Err(anyhow!("Expected pending CtrlUplinkListReq mac-command"));
    }

    let req_pls: Vec<&lrwn::CtrlUplinkListReqPayload> = (**pending.unwrap())
        .iter()
        .filter_map(|v| {
            if let lrwn::MACCommand::CtrlUplinkListReq(pl) = v {
                Some(pl)
            } else {
                None
            }
        })
        .collect();

    let ans_pls: Vec<&lrwn::CtrlUplinkListAnsPayload> = (**block)
        .iter()
        .filter_map(|v| {
            if let lrwn::MACCommand::CtrlUplinkListAns(pl) = v {
                Some(pl)
            } else {
                None
            }
        })
        .collect();

    if req_pls.len() != ans_pls.len() {
        return Err(anyhow!("CtrlUplinkListAns mac-command count does not equal CtrlUplinkListReq mac-command count"));
    }

    for (req_pl, ans_pl) in zip(req_pls, ans_pls) {
        let action = req_pl.ctrl_uplink_action.ctrl_uplink_action;

        if ans_pl.uplink_list_idx_ack {
            if let Some(relay) = &mut ds.relay {
                info!(
                    dev_eui = %dev_eui,
                    uplink_list_idx = req_pl.ctrl_uplink_action.uplink_list_idx,
                    ctrl_uplink_action = action,
                    w_f_cnt = ans_pl.w_fcnt,
                    "CtrlUplinkListReq acknowledged",
                );

                if action == 0 {
                    for rd in &relay.devices {
                        if req_pl.ctrl_uplink_action.uplink_list_idx as u32 == rd.index {
                            let dev_eui = EUI64::from_slice(&rd.dev_eui)?;
                            let mut d = device::get(&dev_eui).await?;
                            let ds = d.get_device_session_mut()?;
                            if let Some(relay) = &mut ds.relay {
                                relay.w_f_cnt = ans_pl.w_fcnt;
                            };
                            device::partial_update(
                                dev_eui,
                                &device::DeviceChangeset {
                                    device_session: Some(d.device_session.clone()),
                                    ..Default::default()
                                },
                            )
                            .await?;
                        }
                    }
                } else if action == 1 {
                    relay
                        .devices
                        .retain(|d| d.index != req_pl.ctrl_uplink_action.uplink_list_idx as u32);
                }
            }
        } else {
            warn!(
                dev_eui = %dev_eui,
                uplink_list_idx = req_pl.ctrl_uplink_action.uplink_list_idx,
                "CtrlUplinkListReq not acknowledged",
            );
        }
    }

    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage;
    use crate::test;
    use chirpstack_api::internal;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        device_session_ed: internal::DeviceSession,
        ctrl_uplink_list_req: Option<lrwn::MACCommandSet>,
        ctrl_uplink_list_ans: lrwn::MACCommandSet,
        expected_device_session_ed: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[tokio::test]
    async fn test_response() {
        let _handle = test::prepare().await;

        let t = storage::tenant::create(storage::tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = storage::application::create(storage::application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = storage::device_profile::create(storage::device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dev = storage::device::create(storage::device::Device {
            name: "test-dev".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let tests = vec![
            Test {
                name: "acked, nothing pending".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_session_ed: internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    relay: Some(internal::Relay {
                        w_f_cnt: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ctrl_uplink_list_req: None,
                ctrl_uplink_list_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListAns(lrwn::CtrlUplinkListAnsPayload {
                        uplink_list_idx_ack: true,
                        w_fcnt: 10,
                    }),
                ]),
                expected_device_session_ed: internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    relay: Some(internal::Relay {
                        w_f_cnt: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_error: Some("Expected pending CtrlUplinkListReq mac-command".to_string()),
            },
            Test {
                name: "acked WFCnt sync".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_session_ed: internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    relay: Some(internal::Relay {
                        w_f_cnt: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ctrl_uplink_list_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 1,
                            ctrl_uplink_action: 0,
                        },
                    }),
                ])),
                ctrl_uplink_list_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListAns(lrwn::CtrlUplinkListAnsPayload {
                        uplink_list_idx_ack: true,
                        w_fcnt: 10,
                    }),
                ]),
                expected_device_session_ed: internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    relay: Some(internal::Relay {
                        w_f_cnt: 10,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "acked delete".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        devices: vec![internal::RelayDevice {
                            index: 1,
                            dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                device_session_ed: internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    ..Default::default()
                },
                ctrl_uplink_list_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListReq(lrwn::CtrlUplinkListReqPayload {
                        ctrl_uplink_action: lrwn::CtrlUplinkActionPL {
                            uplink_list_idx: 1,
                            ctrl_uplink_action: 1,
                        },
                    }),
                ])),
                ctrl_uplink_list_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::CtrlUplinkListAns(lrwn::CtrlUplinkListAnsPayload {
                        uplink_list_idx_ack: true,
                        w_fcnt: 10,
                    }),
                ]),
                expected_device_session_ed: internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    ..Default::default()
                },
                expected_error: None,
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);

            device::partial_update(
                dev.dev_eui,
                &device::DeviceChangeset {
                    device_session: Some(Some(tst.device_session_ed.clone().into())),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

            let mut relay_dev = device::Device {
                device_session: Some(tst.device_session.clone().into()),
                ..Default::default()
            };

            let resp = handle(
                &mut relay_dev,
                &tst.ctrl_uplink_list_ans,
                tst.ctrl_uplink_list_req.as_ref(),
            )
            .await;

            if let Some(e) = &tst.expected_error {
                assert_eq!(true, resp.is_err(), "{}", tst.name);
                assert_eq!(e, &format!("{}", resp.err().unwrap()), "{}", tst.name);
            } else {
                assert_eq!(true, resp.unwrap().is_none());
            }

            let d = device::get(&dev.dev_eui).await.unwrap();
            let ds = d.get_device_session().unwrap();
            assert_eq!(&tst.expected_device_session_ed, ds);
        }
    }
}
