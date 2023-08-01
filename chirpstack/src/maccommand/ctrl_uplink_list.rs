use std::iter::zip;

use anyhow::Result;
use tracing::{info, warn};

use crate::storage::{device, device_session};
use chirpstack_api::internal;
use lrwn::EUI64;

pub async fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
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
                    dev_eui = %dev.dev_eui,
                    uplink_list_idx = req_pl.ctrl_uplink_action.uplink_list_idx,
                    ctrl_uplink_action = action,
                    w_f_cnt = ans_pl.w_fcnt,
                    "CtrlUplinkListReq acknowledged",
                );

                if action == 0 {
                    for rd in &relay.devices {
                        if req_pl.ctrl_uplink_action.uplink_list_idx as u32 == rd.index {
                            let mut ds =
                                device_session::get(&EUI64::from_slice(&rd.dev_eui)?).await?;
                            if let Some(relay) = &mut ds.relay {
                                relay.w_f_cnt = ans_pl.w_fcnt;
                            };
                            device_session::save(&ds).await?;
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
                dev_eui = %dev.dev_eui,
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
    use crate::test;

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
                    dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
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
                    dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
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
                    dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
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
                    dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
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
                    dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
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
                    dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    ..Default::default()
                },
                expected_error: None,
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);

            device_session::save(&tst.device_session_ed).await.unwrap();

            let mut ds = tst.device_session.clone();
            let resp = handle(
                &device::Device::default(),
                &mut ds,
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

            let ds =
                device_session::get(&EUI64::from_slice(&tst.device_session_ed.dev_eui).unwrap())
                    .await
                    .unwrap();
            assert_eq!(tst.expected_device_session_ed, ds);
        }
    }
}
