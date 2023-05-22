use anyhow::Result;
use tracing::{info, warn};

use crate::storage::device;
use chirpstack_api::internal;

pub fn request(dr: u8, freq: u32) -> lrwn::MACCommandSet {
    lrwn::MACCommandSet::new(vec![lrwn::MACCommand::PingSlotChannelReq(
        lrwn::PingSlotChannelReqPayload { freq, dr },
    )])
}

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
    pending: Option<&lrwn::MACCommandSet>,
) -> Result<Option<lrwn::MACCommandSet>> {
    if pending.is_none() {
        return Err(anyhow!("Pending PingSlotChannelReq expected"));
    }

    let block_macs = &**block;
    let pending_macs = &**pending.unwrap();

    let req_pl = if let lrwn::MACCommand::PingSlotChannelReq(pl) = pending_macs
        .first()
        .ok_or_else(|| anyhow!("Empty MACCommandSet"))?
    {
        pl
    } else {
        return Err(anyhow!("Expected PingSlotChannelReq"));
    };

    let ans_pl = if let lrwn::MACCommand::PingSlotChannelAns(pl) = block_macs
        .first()
        .ok_or_else(|| anyhow!("Empty MACCommandSet"))?
    {
        pl
    } else {
        return Err(anyhow!("Expected PingSlotChannelAns"));
    };

    if ans_pl.channel_freq_ok && ans_pl.dr_ok {
        // Reset the error-counter.
        ds.mac_command_error_count
            .remove(&(lrwn::CID::PingSlotChannelReq.to_u8() as u32));

        ds.class_b_ping_slot_dr = req_pl.dr as u32;
        ds.class_b_ping_slot_freq = req_pl.freq;

        info!(dev_eui = %dev.dev_eui, channel_freq = req_pl.freq, dr = req_pl.dr, "PingSlotChannelReq acknowledged");
    } else {
        let count = ds
            .mac_command_error_count
            .entry(lrwn::CID::PingSlotChannelReq.to_u8() as u32)
            .or_insert(0);
        *count += 1;

        warn!(dev_eui = %dev.dev_eui, channel_freq_ok = ans_pl.channel_freq_ok, dr_ok = ans_pl.dr_ok, "PingSlotChannelReq not acknowledged");
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct Test {
        name: String,
        device_session: internal::DeviceSession,
        ping_slot_channel_req: Option<lrwn::MACCommandSet>,
        ping_slot_channel_ans: lrwn::MACCommandSet,
        expected_device_session: internal::DeviceSession,
        expected_error: Option<String>,
    }

    #[test]
    fn test_request() {
        let resp = request(5, 868100000);
        assert_eq!(
            lrwn::MACCommandSet::new(vec![lrwn::MACCommand::PingSlotChannelReq(
                lrwn::PingSlotChannelReqPayload {
                    dr: 5,
                    freq: 868100000,
                }
            )]),
            resp
        );
    }

    #[test]
    fn test_handle() {
        let tests = vec![
            Test {
                name: "pending request and positive ACK updates frequency and data-rate".into(),
                device_session: internal::DeviceSession {
                    class_b_ping_slot_freq: 868100000,
                    class_b_ping_slot_dr: 3,
                    mac_command_error_count: [(lrwn::CID::PingSlotChannelReq.to_u8() as u32, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                ping_slot_channel_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::PingSlotChannelReq(lrwn::PingSlotChannelReqPayload {
                        freq: 868300000,
                        dr: 4,
                    }),
                ])),
                ping_slot_channel_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::PingSlotChannelAns(lrwn::PingSlotChannelAnsPayload {
                        dr_ok: true,
                        channel_freq_ok: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    class_b_ping_slot_freq: 868300000,
                    class_b_ping_slot_dr: 4,
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "pending request and negative ACK does not update".into(),
                device_session: internal::DeviceSession {
                    class_b_ping_slot_freq: 868100000,
                    class_b_ping_slot_dr: 3,
                    mac_command_error_count: [(lrwn::CID::PingSlotChannelReq.to_u8() as u32, 1)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                ping_slot_channel_req: Some(lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::PingSlotChannelReq(lrwn::PingSlotChannelReqPayload {
                        freq: 868300000,
                        dr: 4,
                    }),
                ])),
                ping_slot_channel_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::PingSlotChannelAns(lrwn::PingSlotChannelAnsPayload {
                        dr_ok: false,
                        channel_freq_ok: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    class_b_ping_slot_freq: 868100000,
                    class_b_ping_slot_dr: 3,
                    mac_command_error_count: [(lrwn::CID::PingSlotChannelReq.to_u8() as u32, 2)]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                },
                expected_error: None,
            },
            Test {
                name: "no pending request and positive ACK returns an error".into(),
                device_session: internal::DeviceSession {
                    class_b_ping_slot_freq: 868100000,
                    class_b_ping_slot_dr: 3,
                    ..Default::default()
                },
                ping_slot_channel_req: None,
                ping_slot_channel_ans: lrwn::MACCommandSet::new(vec![
                    lrwn::MACCommand::PingSlotChannelAns(lrwn::PingSlotChannelAnsPayload {
                        dr_ok: true,
                        channel_freq_ok: true,
                    }),
                ]),
                expected_device_session: internal::DeviceSession {
                    class_b_ping_slot_freq: 868100000,
                    class_b_ping_slot_dr: 3,
                    ..Default::default()
                },
                expected_error: Some("Pending PingSlotChannelReq expected".to_string()),
            },
        ];

        for tst in &tests {
            let mut ds = tst.device_session.clone();
            let resp = handle(
                &device::Device {
                    ..Default::default()
                },
                &mut ds,
                &tst.ping_slot_channel_ans,
                tst.ping_slot_channel_req.as_ref(),
            );

            if let Some(e) = &tst.expected_error {
                assert_eq!(true, resp.is_err(), "{}", tst.name);
                assert_eq!(e, &format!("{}", resp.err().unwrap()), "{}", tst.name);
            } else {
                assert_eq!(true, resp.unwrap().is_none());
            }

            assert_eq!(tst.expected_device_session, ds);
        }
    }
}
