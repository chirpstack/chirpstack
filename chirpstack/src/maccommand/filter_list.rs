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
        return Err(anyhow!("Expected pending FilterListReq mac-command"));
    }

    let req_mac = (**pending.unwrap())
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let req_pl = if let lrwn::MACCommand::FilterListReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("FilterListReq is expected"));
    };

    let ans_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let ans_pl = if let lrwn::MACCommand::FilterListAns(pl) = ans_mac {
        pl
    } else {
        return Err(anyhow!("FilterListAns is expected"));
    };

    if ans_pl.filter_list_action_ack && ans_pl.filter_list_len_ack && ans_pl.combined_rules_ack {
        if let Some(relay) = &mut ds.relay {
            for f in &mut relay.filters {
                if req_pl.filter_list_idx as u32 == f.index {
                    info!(
                        filter_list_idx = req_pl.filter_list_idx,
                        filter_list_eui = %hex::encode(&req_pl.filter_list_eui),
                        "FilterListReq acknowledged"
                    );
                    f.provisioned = true;
                }
            }

            // If the action was NoRule, we remove the filter from the relay filters.
            if req_pl.filter_list_action == lrwn::FilterListAction::NoRule {
                relay
                    .filters
                    .retain(|f| f.index != req_pl.filter_list_idx as u32);
            }
        }
    } else {
        error!(
            dev_eui = %dev.dev_eui,
            filter_list_action_ack = ans_pl.filter_list_action_ack,
            filter_list_len_ack = ans_pl.filter_list_len_ack,
            combined_rules_ack = ans_pl.combined_rules_ack,
            "FilterListReq not acknowledged"
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
        filter_list_req: Option<lrwn::MACCommandSet>,
        filter_list_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_response() {
        let tests = vec![
            Test {
                name: "acked, nothing pending".into(),
                device_session: internal::DeviceSession::default(),
                filter_list_req: None,
                filter_list_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListAns(
                    lrwn::FilterListAnsPayload {
                        filter_list_action_ack: true,
                        filter_list_len_ack: true,
                        combined_rules_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession::default(),
                expected_error: Some("Expected pending FilterListReq mac-command".into()),
            },
            Test {
                name: "nacked".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 1,
                            provisioned: false,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                filter_list_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 1,
                        filter_list_action: lrwn::FilterListAction::Forward,
                        filter_list_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    }),
                ])),
                filter_list_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListAns(
                    lrwn::FilterListAnsPayload {
                        filter_list_action_ack: false,
                        filter_list_len_ack: true,
                        combined_rules_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 1,
                            provisioned: false,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "acked".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 1,
                            provisioned: false,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                filter_list_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 1,
                        filter_list_action: lrwn::FilterListAction::Forward,
                        filter_list_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    }),
                ])),
                filter_list_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListAns(
                    lrwn::FilterListAnsPayload {
                        filter_list_action_ack: true,
                        filter_list_len_ack: true,
                        combined_rules_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 1,
                            provisioned: true,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "acked removing filter".into(),
                device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![internal::RelayFilter {
                            index: 1,
                            provisioned: true,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                filter_list_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::FilterListReq(lrwn::FilterListReqPayload {
                        filter_list_idx: 1,
                        filter_list_action: lrwn::FilterListAction::NoRule,
                        filter_list_eui: vec![],
                    }),
                ])),
                filter_list_ans: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::FilterListAns(
                    lrwn::FilterListAnsPayload {
                        filter_list_action_ack: true,
                        filter_list_len_ack: true,
                        combined_rules_ack: true,
                    },
                )]),
                expected_device_session: internal::DeviceSession {
                    relay: Some(internal::Relay {
                        filters: vec![],
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
                &tst.filter_list_ans,
                tst.filter_list_req.as_ref(),
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
