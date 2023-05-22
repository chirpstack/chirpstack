use anyhow::Result;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

use crate::cflist::CFList;
use crate::devaddr::DevAddr;
use crate::dl_settings::DLSettings;
use crate::eui64::EUI64;
use crate::fhdr::FCtrl;
use crate::fhdr::FHDR;
use crate::maccommand::MACCommandSet;
use crate::mhdr::MType;
use crate::netid::NetID;
use crate::relay::{ForwardDownlinkReq, ForwardUplinkReq};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Payload {
    JoinRequest(JoinRequestPayload),
    JoinAccept(JoinAcceptPayload),
    MACPayload(MACPayload),
    RejoinRequestType02(RejoinRequestType02Payload),
    RejoinRequestType1(RejoinRequestType1Payload),
    Raw(Vec<u8>),
}

#[cfg(feature = "serde")]
impl Serialize for Payload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Payload::JoinRequest(v) => v.serialize(serializer),
            Payload::JoinAccept(v) => v.serialize(serializer),
            Payload::MACPayload(v) => v.serialize(serializer),
            Payload::RejoinRequestType02(v) => v.serialize(serializer),
            Payload::RejoinRequestType1(v) => v.serialize(serializer),
            Payload::Raw(v) => serializer.serialize_str(&hex::encode(v)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum JoinType {
    Join,
    RejoinType0,
    RejoinType1,
    RejoinType2,
}

impl Payload {
    pub fn from_slice(m_type: MType, b: &[u8]) -> Result<Self> {
        Ok(match m_type {
            MType::JoinRequest => Payload::JoinRequest(JoinRequestPayload::from_slice(b)?),
            MType::JoinAccept => Payload::Raw(b.to_vec()), // the join-accept is encrypted
            MType::UnconfirmedDataUp
            | MType::ConfirmedDataUp
            | MType::UnconfirmedDataDown
            | MType::ConfirmedDataDown => Payload::MACPayload(MACPayload::from_slice(b)?),
            MType::RejoinRequest => {
                if b.is_empty() {
                    return Err(anyhow!("RejoinRequest payload is empty"));
                }

                match b[0] {
                    0x00 | 0x02 => {
                        Payload::RejoinRequestType02(RejoinRequestType02Payload::from_slice(b)?)
                    }
                    0x01 => Payload::RejoinRequestType1(RejoinRequestType1Payload::from_slice(b)?),
                    _ => {
                        return Err(anyhow!("invalid RejoinType"));
                    }
                }
            }
            MType::Proprietary => Payload::Raw(b.to_vec()),
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        match self {
            Payload::JoinRequest(v) => Ok(v.to_vec()),
            Payload::JoinAccept(v) => v.to_vec(),
            Payload::MACPayload(v) => Ok(v.to_vec()?),
            Payload::RejoinRequestType02(v) => Ok(v.to_vec()?),
            Payload::RejoinRequestType1(v) => Ok(v.to_vec()?),
            Payload::Raw(v) => Ok(v.clone()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JoinRequestPayload {
    pub join_eui: EUI64,
    pub dev_eui: EUI64,
    pub dev_nonce: u16,
}

impl JoinRequestPayload {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != 18 {
            return Err(anyhow!("join-request payload must be exactly 18 bytes"));
        }

        let mut join_eui: [u8; 8] = [0; 8];
        let mut dev_eui: [u8; 8] = [0; 8];

        join_eui.clone_from_slice(&b[0..8]);
        dev_eui.clone_from_slice(&b[8..16]);

        Ok(JoinRequestPayload {
            join_eui: EUI64::from_le_bytes(join_eui),
            dev_eui: EUI64::from_le_bytes(dev_eui),
            dev_nonce: u16::from_le_bytes([b[16], b[17]]),
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(18);
        v.append(&mut self.join_eui.to_le_bytes().to_vec());
        v.append(&mut self.dev_eui.to_le_bytes().to_vec());
        v.append(&mut self.dev_nonce.to_le_bytes().to_vec());
        v
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JoinAcceptPayload {
    pub join_nonce: u32, // the actual max value is (2^24 -1)
    pub home_netid: NetID,
    pub devaddr: DevAddr,
    pub dl_settings: DLSettings,
    pub rx_delay: u8, // 0=1s, 1=1s, ... 15=15s
    pub cflist: Option<CFList>,
}

impl JoinAcceptPayload {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != 12 && b.len() != 28 {
            return Err(anyhow!("12 or 28 bytes are expected for JoinAcceptPayload"));
        }

        Ok(JoinAcceptPayload {
            join_nonce: u32::from_le_bytes([b[0], b[1], b[2], 0x00]),
            home_netid: {
                let mut netid: [u8; 3] = [0; 3];
                netid.clone_from_slice(&b[3..6]);
                NetID::from_le_bytes(netid)
            },
            devaddr: {
                let mut devaddr: [u8; 4] = [0; 4];
                devaddr.clone_from_slice(&b[6..10]);
                DevAddr::from_le_bytes(devaddr)
            },
            dl_settings: {
                let mut dl_settings: [u8; 1] = [0];
                dl_settings.clone_from_slice(&b[10..11]);
                DLSettings::from_le_bytes(dl_settings)
            },
            rx_delay: b[11],
            cflist: match b.len() {
                28 => {
                    let mut cflist: [u8; 16] = [0; 16];
                    cflist.clone_from_slice(&b[12..]);
                    Some(CFList::from_bytes(cflist)?)
                }
                _ => None,
            },
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        if self.rx_delay > 15 {
            return Err(anyhow!("max value of rx_delay is 15"));
        }

        let mut b = Vec::with_capacity(28);
        b.extend_from_slice(&self.join_nonce.to_le_bytes()[..3]);
        b.extend_from_slice(&self.home_netid.to_le_bytes());
        b.extend_from_slice(&self.devaddr.to_le_bytes());
        b.extend_from_slice(&self.dl_settings.to_le_bytes()?);
        b.push(self.rx_delay);

        if let Some(v) = &self.cflist {
            b.extend_from_slice(&v.to_bytes()?);
        }

        Ok(b)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FRMPayload {
    Raw(Vec<u8>),
    MACCommandSet(MACCommandSet),
    ForwardUplinkReq(ForwardUplinkReq),
    ForwardDownlinkReq(ForwardDownlinkReq),
}

#[cfg(feature = "serde")]
impl Serialize for FRMPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FRMPayload::Raw(v) => serializer.serialize_str(&hex::encode(v)),
            FRMPayload::MACCommandSet(v) => v.serialize(serializer),
            FRMPayload::ForwardUplinkReq(v) => {
                serializer.serialize_newtype_variant("FRMPayload", 2, "ForwardUplinkReq", v)
            }
            FRMPayload::ForwardDownlinkReq(v) => {
                serializer.serialize_newtype_variant("FRMPayload", 3, "ForwardDownlinkReq", v)
            }
        }
    }
}

impl FRMPayload {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(match self {
            FRMPayload::Raw(v) => v.clone(),
            FRMPayload::MACCommandSet(v) => v.to_vec()?,
            FRMPayload::ForwardUplinkReq(v) => v.to_vec()?,
            FRMPayload::ForwardDownlinkReq(v) => v.to_vec()?,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MACPayload {
    pub fhdr: FHDR,
    pub f_port: Option<u8>,
    pub frm_payload: Option<FRMPayload>,
}

impl MACPayload {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        let b_len = b.len();
        if b_len < 7 {
            return Err(anyhow!("MACPayload requires at least 7 bytes"));
        }

        // decode FCtrl as we need f_opts_len.
        let mut f_ctrl: [u8; 1] = [0];
        f_ctrl.clone_from_slice(&b[4..5]);
        let f_ctrl = FCtrl::from_le_bytes(f_ctrl);

        // check that there are at least as many bytes as FOptsLen claims
        let fhdr_size = 7 + f_ctrl.f_opts_len as usize;
        if b_len < fhdr_size {
            return Err(anyhow!("not enough bytes to decode FHDR"));
        }

        Ok(MACPayload {
            fhdr: FHDR::from_slice(&b[..fhdr_size])?,
            f_port: match b_len > fhdr_size {
                true => Some(b[fhdr_size]),
                false => None,
            },
            frm_payload: match b_len > fhdr_size + 1 {
                true => Some(FRMPayload::Raw(b[fhdr_size + 1..].to_vec())),
                false => None,
            },
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        // validation of frm_payload
        if self.frm_payload.is_some() {
            // validate that f_port is set
            if self.f_port.is_none() {
                return Err(anyhow!("f_port not not be None when frm_payload is set"));
            }

            // mac-commands must have f_port=0
            if let FRMPayload::MACCommandSet(_) = &self.frm_payload.as_ref().unwrap() {
                if self.f_port.unwrap() != 0 {
                    return Err(anyhow!(
                        "f_port must be set to 0 for mac-commands in frm_payload"
                    ));
                }
            }
        }

        let mut b = Vec::new();

        // fhdr
        b.append(&mut self.fhdr.to_vec()?);

        // f_port
        if let Some(v) = self.f_port {
            b.push(v);
        }

        // frm_payload
        if let Some(v) = &self.frm_payload {
            b.append(&mut v.to_vec()?);
        }

        Ok(b)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RejoinRequestType02Payload {
    pub rejoin_type: JoinType,
    pub netid: NetID,
    pub dev_eui: EUI64,
    pub rj_count_0: u16,
}

impl RejoinRequestType02Payload {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != 14 {
            return Err(anyhow!(
                "rejoin-request type 0/2 payload must be exactly 14 bytes"
            ));
        }

        Ok(RejoinRequestType02Payload {
            rejoin_type: match b[0] {
                0x00 => JoinType::RejoinType0,
                0x02 => JoinType::RejoinType2,
                _ => {
                    return Err(anyhow!("invalid rejoin_type"));
                }
            },
            netid: {
                let mut netid: [u8; 3] = [0; 3];
                netid.clone_from_slice(&b[1..4]);
                NetID::from_le_bytes(netid)
            },
            dev_eui: {
                let mut deveui: [u8; 8] = [0; 8];
                deveui.clone_from_slice(&b[4..12]);
                EUI64::from_le_bytes(deveui)
            },
            rj_count_0: u16::from_le_bytes([b[12], b[13]]),
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut b = Vec::with_capacity(14);
        b.push(match self.rejoin_type {
            JoinType::RejoinType0 => 0x00,
            JoinType::RejoinType2 => 0x02,
            _ => {
                return Err(anyhow!("rejoin_type must be 0 or 2"));
            }
        });

        b.extend_from_slice(&self.netid.to_le_bytes());
        b.extend_from_slice(&self.dev_eui.to_le_bytes());
        b.extend_from_slice(&self.rj_count_0.to_le_bytes());

        Ok(b)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RejoinRequestType1Payload {
    pub rejoin_type: JoinType,
    pub join_eui: EUI64,
    pub dev_eui: EUI64,
    pub rj_count_1: u16,
}

impl RejoinRequestType1Payload {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != 19 {
            return Err(anyhow!(
                "rejoin-request type 1 payload must be exactly 19 bytes"
            ));
        }

        Ok(RejoinRequestType1Payload {
            rejoin_type: match b[0] {
                0x01 => JoinType::RejoinType1,
                _ => {
                    return Err(anyhow!("invalid rejoin_type"));
                }
            },
            join_eui: {
                let mut joineui: [u8; 8] = [0; 8];
                joineui.clone_from_slice(&b[1..9]);
                EUI64::from_le_bytes(joineui)
            },
            dev_eui: {
                let mut deveui: [u8; 8] = [0; 8];
                deveui.clone_from_slice(&b[9..17]);
                EUI64::from_le_bytes(deveui)
            },
            rj_count_1: u16::from_le_bytes([b[17], b[18]]),
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut b = Vec::with_capacity(19);
        b.push(match self.rejoin_type {
            JoinType::RejoinType1 => 0x01,
            _ => {
                return Err(anyhow!("rejoin_type must be 1"));
            }
        });

        b.extend_from_slice(&self.join_eui.to_le_bytes());
        b.extend_from_slice(&self.dev_eui.to_le_bytes());
        b.extend_from_slice(&self.rj_count_1.to_le_bytes());

        Ok(b)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::super::cflist::CFListChannels;
    use super::super::maccommand::MACCommand;
    use super::*;

    struct PayloadTest {
        pl: Payload,
        bytes: Vec<u8>,
    }

    struct JoinAcceptTest {
        pl: JoinAcceptPayload,
        bytes: Vec<u8>,
    }

    #[test]
    fn test_join_request_payload() {
        let tests = vec![PayloadTest {
            pl: Payload::JoinRequest(JoinRequestPayload {
                join_eui: EUI64::from_str("0102030405060708").unwrap(),
                dev_eui: EUI64::from_str("0807060504030201").unwrap(),
                dev_nonce: 1024,
            }),
            bytes: vec![
                0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                0x07, 0x08, 0x00, 0x04,
            ],
        }];

        for tst in tests {
            assert_eq!(tst.bytes, tst.pl.to_vec().unwrap());
            assert_eq!(
                tst.pl,
                Payload::from_slice(MType::JoinRequest, &tst.bytes).unwrap()
            );
        }
    }

    #[test]
    fn test_join_accept_payload() {
        // in this case the payload is converted to Raw as the join-accept must first be decrypted
        // before it can be decoded
        assert_eq!(
            Payload::Raw(vec![0x01, 0x02, 0x03]),
            Payload::from_slice(MType::JoinAccept, &vec![0x01, 0x02, 0x03]).unwrap()
        );

        // test decoding the (decrypted) join-accept payload
        let tests = vec![
            JoinAcceptTest {
                pl: JoinAcceptPayload {
                    join_nonce: 65793,
                    home_netid: NetID::from_str("020202").unwrap(),
                    devaddr: DevAddr::from_str("01020304").unwrap(),
                    dl_settings: DLSettings {
                        rx2_dr: 7,
                        rx1_dr_offset: 6,
                        opt_neg: false,
                    },
                    rx_delay: 9,
                    cflist: None,
                },
                bytes: vec![
                    0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x04, 0x03, 0x02, 0x01, 0x67, 0x09,
                ],
            },
            JoinAcceptTest {
                pl: JoinAcceptPayload {
                    join_nonce: 65793,
                    home_netid: NetID::from_str("020202").unwrap(),
                    devaddr: DevAddr::from_str("01020304").unwrap(),
                    dl_settings: DLSettings {
                        rx2_dr: 7,
                        rx1_dr_offset: 6,
                        opt_neg: false,
                    },
                    rx_delay: 9,
                    cflist: Some(CFList::Channels(CFListChannels::new([
                        867100000, 867300000, 867500000, 867700000, 867900000,
                    ]))),
                },
                bytes: vec![
                    0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x04, 0x03, 0x02, 0x01, 0x67, 0x09, 0x18,
                    0x4f, 0x84, 0xe8, 0x56, 0x84, 0xb8, 0x5e, 0x84, 0x88, 0x66, 0x84, 0x58, 0x6e,
                    0x84, 0x00,
                ],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.pl.to_vec().unwrap());
            assert_eq!(tst.pl, JoinAcceptPayload::from_slice(&tst.bytes).unwrap());
        }
    }

    #[test]
    fn test_mac_payload() {
        let tests = vec![
            PayloadTest {
                pl: Payload::MACPayload(MACPayload {
                    fhdr: FHDR {
                        devaddr: DevAddr::from_str("01020304").unwrap(),
                        f_ctrl: FCtrl {
                            adr: true,
                            ..Default::default()
                        },
                        f_cnt: 123,
                        f_opts: MACCommandSet::new(vec![]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                bytes: vec![0x04, 0x03, 0x02, 0x01, 0x80, 0x7b, 0x00],
            },
            PayloadTest {
                pl: Payload::MACPayload(MACPayload {
                    fhdr: FHDR {
                        devaddr: DevAddr::from_str("01020304").unwrap(),
                        f_ctrl: FCtrl {
                            adr: true,
                            f_opts_len: 3, // only needed for assert_eq!
                            ..Default::default()
                        },
                        f_cnt: 123,
                        f_opts: MACCommandSet::new(vec![MACCommand::Raw(vec![0x01, 0x02, 0x03])]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                bytes: vec![0x04, 0x03, 0x02, 0x01, 0x83, 0x7b, 0x00, 0x01, 0x02, 0x03],
            },
            PayloadTest {
                pl: Payload::MACPayload(MACPayload {
                    fhdr: FHDR {
                        devaddr: DevAddr::from_str("01020304").unwrap(),
                        f_ctrl: FCtrl {
                            adr: true,
                            ..Default::default()
                        },
                        f_cnt: 123,
                        f_opts: MACCommandSet::new(vec![]),
                    },
                    f_port: Some(10),
                    frm_payload: None,
                }),
                bytes: vec![0x04, 0x03, 0x02, 0x01, 0x80, 0x7b, 0x00, 0x0a],
            },
            PayloadTest {
                pl: Payload::MACPayload(MACPayload {
                    fhdr: FHDR {
                        devaddr: DevAddr::from_str("01020304").unwrap(),
                        f_ctrl: FCtrl {
                            adr: true,
                            ..Default::default()
                        },
                        f_cnt: 123,
                        f_opts: MACCommandSet::new(vec![]),
                    },
                    f_port: Some(10),
                    frm_payload: Some(FRMPayload::Raw(vec![0x01, 0x02, 0x03])),
                }),
                bytes: vec![
                    0x04, 0x03, 0x02, 0x01, 0x80, 0x7b, 0x00, 0x0a, 0x01, 0x02, 0x03,
                ],
            },
            PayloadTest {
                pl: Payload::MACPayload(MACPayload {
                    fhdr: FHDR {
                        devaddr: DevAddr::from_str("01020304").unwrap(),
                        f_ctrl: FCtrl {
                            adr: true,
                            f_opts_len: 3,
                            ..Default::default()
                        },
                        f_cnt: 123,
                        f_opts: MACCommandSet::new(vec![MACCommand::Raw(vec![0x03, 0x02, 0x01])]),
                    },
                    f_port: Some(10),
                    frm_payload: Some(FRMPayload::Raw(vec![0x01, 0x02, 0x03])),
                }),
                bytes: vec![
                    0x04, 0x03, 0x02, 0x01, 0x83, 0x7b, 0x00, 0x03, 0x02, 0x01, 0x0a, 0x01, 0x02,
                    0x03,
                ],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.pl.to_vec().unwrap());
            assert_eq!(
                tst.pl,
                Payload::from_slice(MType::UnconfirmedDataUp, &tst.bytes).unwrap()
            );
        }
    }

    #[test]
    fn test_rejoin_request() {
        let tests = vec![
            PayloadTest {
                pl: Payload::RejoinRequestType02(RejoinRequestType02Payload {
                    rejoin_type: JoinType::RejoinType0,
                    netid: NetID::from_str("010203").unwrap(),
                    dev_eui: EUI64::from_str("0102030405060708").unwrap(),
                    rj_count_0: 219,
                }),
                bytes: vec![
                    0x00, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0xdb,
                    0x00,
                ],
            },
            PayloadTest {
                pl: Payload::RejoinRequestType02(RejoinRequestType02Payload {
                    rejoin_type: JoinType::RejoinType2,
                    netid: NetID::from_str("010203").unwrap(),
                    dev_eui: EUI64::from_str("0102030405060708").unwrap(),
                    rj_count_0: 219,
                }),
                bytes: vec![
                    0x02, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0xdb,
                    0x00,
                ],
            },
            PayloadTest {
                pl: Payload::RejoinRequestType1(RejoinRequestType1Payload {
                    rejoin_type: JoinType::RejoinType1,
                    join_eui: EUI64::from_str("0102030405060708").unwrap(),
                    dev_eui: EUI64::from_str("0807060504030201").unwrap(),
                    rj_count_1: 219,
                }),
                bytes: vec![
                    0x01, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x02, 0x03, 0x04,
                    0x05, 0x06, 0x07, 0x08, 0xdb, 0x00,
                ],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.pl.to_vec().unwrap());
            assert_eq!(
                tst.pl,
                Payload::from_slice(MType::RejoinRequest, &tst.bytes).unwrap()
            );
        }
    }

    #[test]
    fn test_proprietary() {
        let tests = vec![PayloadTest {
            pl: Payload::Raw(vec![0x01, 0x02, 0x3]),
            bytes: vec![0x01, 0x02, 0x03],
        }];

        for tst in tests {
            assert_eq!(tst.bytes, tst.pl.to_vec().unwrap());
            assert_eq!(
                tst.pl,
                Payload::from_slice(MType::Proprietary, &tst.bytes).unwrap()
            );
        }
    }
}
