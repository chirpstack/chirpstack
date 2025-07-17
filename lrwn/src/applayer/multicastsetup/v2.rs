#[cfg(feature = "crypto")]
use aes::{
    cipher::BlockDecrypt,
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes128, Block,
};
use anyhow::Result;

use crate::applayer::PayloadCodec;
use crate::helpers::{decode_freq, encode_freq};
use crate::{AES128Key, DevAddr};

pub enum Cid {
    PackageVersionReq,
    PackageVersionAns,
    McGroupStatusReq,
    McGroupStatusAns,
    McGroupSetupReq,
    McGroupSetupAns,
    McGroupDeleteReq,
    McGroupDeleteAns,
    McClassCSessionReq,
    McClassCSessionAns,
    McClassBSessionReq,
    McClassBSessionAns,
}

impl Cid {
    pub fn from_u8(uplink: bool, value: u8) -> Result<Self> {
        Ok(match uplink {
            true => match value {
                0x00 => Cid::PackageVersionAns,
                0x01 => Cid::McGroupStatusAns,
                0x02 => Cid::McGroupSetupAns,
                0x03 => Cid::McGroupDeleteAns,
                0x04 => Cid::McClassCSessionAns,
                0x05 => Cid::McClassBSessionAns,
                _ => return Err(anyhow!("Invalid CID: {}", value)),
            },
            false => match value {
                0x00 => Cid::PackageVersionReq,
                0x01 => Cid::McGroupStatusReq,
                0x02 => Cid::McGroupSetupReq,
                0x03 => Cid::McGroupDeleteReq,
                0x04 => Cid::McClassCSessionReq,
                0x05 => Cid::McClassBSessionReq,
                _ => return Err(anyhow!("Invalid CID: {}", value)),
            },
        })
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Cid::PackageVersionReq | Cid::PackageVersionAns => 0x00,
            Cid::McGroupStatusReq | Cid::McGroupStatusAns => 0x01,
            Cid::McGroupSetupReq | Cid::McGroupSetupAns => 0x02,
            Cid::McGroupDeleteReq | Cid::McGroupDeleteAns => 0x03,
            Cid::McClassCSessionReq | Cid::McClassCSessionAns => 0x04,
            Cid::McClassBSessionReq | Cid::McClassBSessionAns => 0x05,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    PackageVersionReq,
    PackageVersionAns(PackageVersionAnsPayload),
    McGroupStatusReq(McGroupStatusReqPayload),
    McGroupStatusAns(McGroupStatusAnsPayload),
    McGroupSetupReq(McGroupSetupReqPayload),
    McGroupSetupAns(McGroupSetupAnsPayload),
    McGroupDeleteReq(McGroupDeleteReqPayload),
    McGroupDeleteAns(McGroupDeleteAnsPayload),
    McClassCSessionReq(McClassCSessionReqPayload),
    McClassCSessionAns(McClassCSessionAnsPayload),
    McClassBSessionReq(McClassBSessionReqPayload),
    McClassBSessionAns(McClassBSessionAnsPayload),
}

impl Payload {
    pub fn cid(&self) -> Cid {
        match self {
            Self::PackageVersionReq => Cid::PackageVersionReq,
            Self::PackageVersionAns(_) => Cid::PackageVersionAns,
            Self::McGroupStatusReq(_) => Cid::McGroupStatusReq,
            Self::McGroupStatusAns(_) => Cid::McGroupStatusAns,
            Self::McGroupSetupReq(_) => Cid::McGroupSetupReq,
            Self::McGroupSetupAns(_) => Cid::McGroupSetupAns,
            Self::McGroupDeleteReq(_) => Cid::McGroupDeleteReq,
            Self::McGroupDeleteAns(_) => Cid::McGroupDeleteAns,
            Self::McClassCSessionReq(_) => Cid::McClassCSessionReq,
            Self::McClassCSessionAns(_) => Cid::McClassCSessionAns,
            Self::McClassBSessionReq(_) => Cid::McClassBSessionReq,
            Self::McClassBSessionAns(_) => Cid::McClassBSessionAns,
        }
    }

    pub fn from_slice(uplink: bool, b: &[u8]) -> Result<Self> {
        if b.is_empty() {
            return Err(anyhow!("At least one byte is expected"));
        }

        let cid = Cid::from_u8(uplink, b[0])?;

        Ok(match cid {
            Cid::PackageVersionReq => Payload::PackageVersionReq,
            Cid::PackageVersionAns => {
                Payload::PackageVersionAns(PackageVersionAnsPayload::decode(&b[1..])?)
            }
            Cid::McGroupStatusReq => {
                Payload::McGroupStatusReq(McGroupStatusReqPayload::decode(&b[1..])?)
            }
            Cid::McGroupStatusAns => {
                Payload::McGroupStatusAns(McGroupStatusAnsPayload::decode(&b[1..])?)
            }
            Cid::McGroupSetupReq => {
                Payload::McGroupSetupReq(McGroupSetupReqPayload::decode(&b[1..])?)
            }
            Cid::McGroupSetupAns => {
                Payload::McGroupSetupAns(McGroupSetupAnsPayload::decode(&b[1..])?)
            }
            Cid::McGroupDeleteReq => {
                Payload::McGroupDeleteReq(McGroupDeleteReqPayload::decode(&b[1..])?)
            }
            Cid::McGroupDeleteAns => {
                Payload::McGroupDeleteAns(McGroupDeleteAnsPayload::decode(&b[1..])?)
            }
            Cid::McClassCSessionReq => {
                Payload::McClassCSessionReq(McClassCSessionReqPayload::decode(&b[1..])?)
            }
            Cid::McClassCSessionAns => {
                Payload::McClassCSessionAns(McClassCSessionAnsPayload::decode(&b[1..])?)
            }
            Cid::McClassBSessionReq => {
                Payload::McClassBSessionReq(McClassBSessionReqPayload::decode(&b[1..])?)
            }
            Cid::McClassBSessionAns => {
                Payload::McClassBSessionAns(McClassBSessionAnsPayload::decode(&b[1..])?)
            }
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut out = vec![self.cid().to_u8()];

        match self {
            Self::PackageVersionReq => {}
            Self::PackageVersionAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McGroupStatusReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McGroupStatusAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McGroupSetupReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McGroupSetupAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McGroupDeleteReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McGroupDeleteAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McClassCSessionReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McClassCSessionAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McClassBSessionReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::McClassBSessionAns(pl) => out.extend_from_slice(&pl.encode()?),
        }

        Ok(out)
    }
}

#[derive(Debug, PartialEq)]
pub struct PackageVersionAnsPayload {
    pub package_identifier: u8,
    pub package_version: u8,
}

impl PayloadCodec for PackageVersionAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 2 {
            return Err(anyhow!("Expected 2 bytes"));
        }

        Ok(PackageVersionAnsPayload {
            package_identifier: b[0],
            package_version: b[1],
        })
    }
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.package_identifier, self.package_version])
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupStatusReqPayload {
    pub cmd_mask: McGroupStatusReqPayloadCmdMask,
}

impl PayloadCodec for McGroupStatusReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        let mut out = McGroupStatusReqPayload {
            cmd_mask: McGroupStatusReqPayloadCmdMask {
                req_group_mask: [false; 4],
            },
        };

        for (i, v) in out.cmd_mask.req_group_mask.iter_mut().enumerate() {
            *v = b[0] & (1 << i) != 0;
        }

        Ok(out)
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut out = vec![0; 1];
        for (i, v) in self.cmd_mask.req_group_mask.iter().enumerate() {
            if *v {
                out[0] |= 1 << i;
            }
        }
        Ok(out)
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupStatusReqPayloadCmdMask {
    pub req_group_mask: [bool; 4],
}

#[derive(Debug, PartialEq)]
pub struct McGroupStatusAnsPayload {
    pub status: McGroupStatusAnsPayloadStatus,
    pub items: Vec<McGroupStatusAnsPayloadItem>,
}

impl PayloadCodec for McGroupStatusAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.is_empty() {
            return Err(anyhow!("At least 1 byte is expected"));
        }

        if (b.len() - 1) % 5 != 0 {
            return Err(anyhow!("Expected a multiple of 5 bytes + 1"));
        }

        Ok(McGroupStatusAnsPayload {
            status: McGroupStatusAnsPayloadStatus {
                ans_group_mask: {
                    let mut mask = [false; 4];
                    for (i, v) in mask.iter_mut().enumerate() {
                        *v = b[0] & (1 << i) != 0;
                    }
                    mask
                },
                nb_total_groups: (b[0] >> 4) & 0x07,
            },
            items: b[1..]
                .chunks(5)
                .map(|b| {
                    let mut mc_addr = [0; 4];
                    mc_addr.copy_from_slice(&b[1..5]);
                    McGroupStatusAnsPayloadItem {
                        mc_group_id: b[0],
                        mc_addr: DevAddr::from_le_bytes(mc_addr),
                    }
                })
                .collect(),
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b = Vec::with_capacity(1 + (self.items.len() * 5));

        b.push(self.status.nb_total_groups << 4);
        for (i, v) in self.status.ans_group_mask.iter().enumerate() {
            if *v {
                b[0] |= 1 << i;
            }
        }

        for v in &self.items {
            b.push(v.mc_group_id);
            b.extend_from_slice(&v.mc_addr.to_le_bytes());
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupStatusAnsPayloadStatus {
    pub ans_group_mask: [bool; 4],
    pub nb_total_groups: u8,
}

#[derive(Debug, PartialEq)]
pub struct McGroupStatusAnsPayloadItem {
    pub mc_group_id: u8,
    pub mc_addr: DevAddr,
}

#[derive(Debug, PartialEq)]
pub struct McGroupSetupReqPayload {
    pub mc_group_id_header: McGroupSetupReqPayloadMcGroupIdHeader,
    pub mc_addr: DevAddr,
    pub mc_key_encrypted: [u8; 16],
    pub min_mc_f_count: u32,
    pub max_mc_f_count: u32,
}

impl PayloadCodec for McGroupSetupReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 29 {
            return Err(anyhow!("Expected 29 bytes"));
        }

        Ok(McGroupSetupReqPayload {
            mc_group_id_header: McGroupSetupReqPayloadMcGroupIdHeader {
                mc_group_id: b[0] & 0x03,
            },
            mc_addr: {
                let mut mc_addr = [0; 4];
                mc_addr.copy_from_slice(&b[1..5]);
                DevAddr::from_le_bytes(mc_addr)
            },
            mc_key_encrypted: {
                let mut mc_key_encrypted = [0; 16];
                mc_key_encrypted.copy_from_slice(&b[5..21]);
                mc_key_encrypted
            },
            min_mc_f_count: {
                let mut f_count = [0; 4];
                f_count.copy_from_slice(&b[21..25]);
                u32::from_le_bytes(f_count)
            },
            max_mc_f_count: {
                let mut f_count = [0; 4];
                f_count.copy_from_slice(&b[25..29]);
                u32::from_le_bytes(f_count)
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.mc_group_id_header.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        let mut b = Vec::with_capacity(29);
        b.push(self.mc_group_id_header.mc_group_id);
        b.extend_from_slice(&self.mc_addr.to_le_bytes());
        b.extend_from_slice(&self.mc_key_encrypted);
        b.extend_from_slice(&self.min_mc_f_count.to_le_bytes());
        b.extend_from_slice(&self.max_mc_f_count.to_le_bytes());

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupSetupReqPayloadMcGroupIdHeader {
    pub mc_group_id: u8,
}

#[derive(Debug, PartialEq)]
pub struct McGroupSetupAnsPayload {
    pub mc_group_id_header: McGroupSetupAnsPayloadMcGroupIdHeader,
}

impl PayloadCodec for McGroupSetupAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(McGroupSetupAnsPayload {
            mc_group_id_header: McGroupSetupAnsPayloadMcGroupIdHeader {
                mc_group_id: b[0] & 0x03,
                id_error: b[0] & 0x04 != 0,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.mc_group_id_header.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        let mut b = vec![self.mc_group_id_header.mc_group_id];
        if self.mc_group_id_header.id_error {
            b[0] |= 0x04;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupSetupAnsPayloadMcGroupIdHeader {
    pub mc_group_id: u8,
    pub id_error: bool,
}

#[derive(Debug, PartialEq)]
pub struct McGroupDeleteReqPayload {
    pub mc_group_id_header: McGroupDeleteReqPayloadMcGroupIdHeader,
}

impl PayloadCodec for McGroupDeleteReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(McGroupDeleteReqPayload {
            mc_group_id_header: McGroupDeleteReqPayloadMcGroupIdHeader {
                mc_group_id: b[0] & 0x03,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.mc_group_id_header.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        Ok(vec![self.mc_group_id_header.mc_group_id])
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupDeleteReqPayloadMcGroupIdHeader {
    pub mc_group_id: u8,
}

#[derive(Debug, PartialEq)]
pub struct McGroupDeleteAnsPayload {
    pub mc_group_id_header: McGroupDeleteAnsPayloadMcGroupIdHeader,
}

impl PayloadCodec for McGroupDeleteAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(McGroupDeleteAnsPayload {
            mc_group_id_header: McGroupDeleteAnsPayloadMcGroupIdHeader {
                mc_group_id: b[0] & 0x03,
                mc_group_undefined: b[0] & 0x04 != 0,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.mc_group_id_header.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        let mut b = vec![self.mc_group_id_header.mc_group_id];
        if self.mc_group_id_header.mc_group_undefined {
            b[0] |= 0x04;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McGroupDeleteAnsPayloadMcGroupIdHeader {
    pub mc_group_id: u8,
    pub mc_group_undefined: bool,
}

#[derive(Debug, PartialEq)]
pub struct McClassCSessionReqPayload {
    pub mc_group_id_header: McClassCSessionReqPayloadMcGroupIdHeader,
    pub session_time: u32,
    pub session_time_out: McClassCSessionReqPayloadSessionTimeOut,
    pub dl_frequ: u32,
    pub dr: u8,
}

impl PayloadCodec for McClassCSessionReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 10 {
            return Err(anyhow!("Expected 10 bytes"));
        }

        Ok(McClassCSessionReqPayload {
            mc_group_id_header: McClassCSessionReqPayloadMcGroupIdHeader {
                mc_group_id: b[0] & 0x03,
            },
            session_time: {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&b[1..5]);
                u32::from_le_bytes(bytes)
            },
            session_time_out: McClassCSessionReqPayloadSessionTimeOut {
                time_out: b[5] & 0x0f,
            },
            dl_frequ: decode_freq(&b[6..9])?,
            dr: b[9],
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.mc_group_id_header.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        if self.session_time_out.time_out > 15 {
            return Err(anyhow!("Max time_out value is 15"));
        }

        let mut b = Vec::with_capacity(10);
        b.push(self.mc_group_id_header.mc_group_id);
        b.extend_from_slice(&self.session_time.to_le_bytes());
        b.push(self.session_time_out.time_out);
        b.extend_from_slice(&encode_freq(self.dl_frequ)?);
        b.push(self.dr);

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McClassCSessionReqPayloadMcGroupIdHeader {
    pub mc_group_id: u8,
}

#[derive(Debug, PartialEq)]
pub struct McClassCSessionReqPayloadSessionTimeOut {
    pub time_out: u8,
}

#[derive(Debug, PartialEq)]
pub struct McClassCSessionAnsPayload {
    pub status_and_mc_group_id: McClassCSessionAnsPayloadStatusAnsMcGroupId,
    pub time_to_start: Option<u32>,
}

impl PayloadCodec for McClassCSessionAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.is_empty() {
            return Err(anyhow!("At least 1 byte expected"));
        }

        Ok(McClassCSessionAnsPayload {
            status_and_mc_group_id: McClassCSessionAnsPayloadStatusAnsMcGroupId {
                mc_group_id: b[0] & 0x03,
                dr_error: b[0] & 0x04 != 0,
                freq_error: b[0] & 0x08 != 0,
                mc_group_undefined: b[0] & 0x10 != 0,
                start_missed: b[0] & 0x20 != 0,
            },
            time_to_start: if b[0] & 0x1c != 0 {
                None
            } else {
                if b.len() != 4 {
                    return Err(anyhow!("Expected 4 bytes"));
                }

                let mut bytes = [0; 4];
                bytes[0..3].copy_from_slice(&b[1..4]);
                Some(u32::from_le_bytes(bytes))
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.status_and_mc_group_id.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        if let Some(v) = self.time_to_start {
            if v >= (1 << 24) {
                return Err(anyhow!("Max time_to_start is 1^24 - 1"));
            }
        }

        let mut b = Vec::with_capacity(4);
        b.push(self.status_and_mc_group_id.mc_group_id);
        if self.status_and_mc_group_id.dr_error {
            b[0] |= 0x04;
        }
        if self.status_and_mc_group_id.freq_error {
            b[0] |= 0x08;
        }
        if self.status_and_mc_group_id.mc_group_undefined {
            b[0] |= 0x10;
        }
        if self.status_and_mc_group_id.start_missed {
            b[0] |= 0x20;
        }

        if let Some(v) = self.time_to_start {
            b.extend_from_slice(&v.to_le_bytes()[0..3]);
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McClassCSessionAnsPayloadStatusAnsMcGroupId {
    pub mc_group_id: u8,
    pub dr_error: bool,
    pub freq_error: bool,
    pub mc_group_undefined: bool,
    pub start_missed: bool,
}

#[derive(Debug, PartialEq)]
pub struct McClassBSessionReqPayload {
    pub mc_group_id_header: McClassBSessionReqPayloadMcGroupIdHeader,
    pub session_time: u32,
    pub time_out_periodicity: McClassBSessionReqPayloadTimeOutPeriodicity,
    pub dl_frequ: u32,
    pub dr: u8,
}

impl PayloadCodec for McClassBSessionReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 10 {
            return Err(anyhow!("Expected 10 bytes"));
        }

        Ok(McClassBSessionReqPayload {
            mc_group_id_header: McClassBSessionReqPayloadMcGroupIdHeader {
                mc_group_id: b[0] & 0x03,
            },
            session_time: {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&b[1..5]);
                u32::from_le_bytes(bytes)
            },
            time_out_periodicity: McClassBSessionReqPayloadTimeOutPeriodicity {
                time_out: b[5] & 0x0f,
                periodicity: (b[5] >> 4) & 0x07,
            },
            dl_frequ: decode_freq(&b[6..9])?,
            dr: b[9],
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.mc_group_id_header.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        if self.time_out_periodicity.time_out > 15 {
            return Err(anyhow!("Max time_out value is 15"));
        }

        if self.time_out_periodicity.periodicity > 7 {
            return Err(anyhow!("Max periodicity value is 7"));
        }

        let mut b = Vec::with_capacity(10);
        b.push(self.mc_group_id_header.mc_group_id);
        b.extend_from_slice(&self.session_time.to_le_bytes());
        b.push((self.time_out_periodicity.periodicity << 4) | self.time_out_periodicity.time_out);
        b.extend_from_slice(&encode_freq(self.dl_frequ)?);
        b.push(self.dr);

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McClassBSessionReqPayloadMcGroupIdHeader {
    pub mc_group_id: u8,
}

#[derive(Debug, PartialEq)]
pub struct McClassBSessionReqPayloadTimeOutPeriodicity {
    pub time_out: u8,
    pub periodicity: u8,
}

#[derive(Debug, PartialEq)]
pub struct McClassBSessionAnsPayload {
    pub status_and_mc_group_id: McClassBSessionAnsPayloadStatusAndMcGroupId,
    pub time_to_start: Option<u32>,
}

impl PayloadCodec for McClassBSessionAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.is_empty() {
            return Err(anyhow!("At least 1 byte expected"));
        }

        Ok(McClassBSessionAnsPayload {
            status_and_mc_group_id: McClassBSessionAnsPayloadStatusAndMcGroupId {
                mc_group_id: b[0] & 0x03,
                dr_error: b[0] & 0x04 != 0,
                freq_error: b[0] & 0x08 != 0,
                mc_group_undefined: b[0] & 0x10 != 0,
                start_missed: b[0] & 0x20 != 0,
            },
            time_to_start: if b[0] & 0x1c != 0 {
                None
            } else {
                if b.len() != 4 {
                    return Err(anyhow!("Expected 4 bytes"));
                }

                let mut bytes = [0; 4];
                bytes[0..3].copy_from_slice(&b[1..4]);
                Some(u32::from_le_bytes(bytes))
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.status_and_mc_group_id.mc_group_id > 3 {
            return Err(anyhow!("Max mc_group_id value is 3"));
        }

        if let Some(v) = self.time_to_start {
            if v >= (1 << 24) {
                return Err(anyhow!("Max time_to_start is 1^24 - 1"));
            }
        }

        let mut b = Vec::with_capacity(4);
        b.push(self.status_and_mc_group_id.mc_group_id);

        if self.status_and_mc_group_id.dr_error {
            b[0] |= 0x04;
        }
        if self.status_and_mc_group_id.freq_error {
            b[0] |= 0x08;
        }
        if self.status_and_mc_group_id.mc_group_undefined {
            b[0] |= 0x10;
        }
        if self.status_and_mc_group_id.start_missed {
            b[0] |= 0x20;
        }

        if let Some(v) = self.time_to_start {
            b.extend_from_slice(&v.to_le_bytes()[0..3]);
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct McClassBSessionAnsPayloadStatusAndMcGroupId {
    pub mc_group_id: u8,
    pub dr_error: bool,
    pub freq_error: bool,
    pub mc_group_undefined: bool,
    pub start_missed: bool,
}

pub fn get_mc_root_key_for_gen_app_key(gen_app_key: AES128Key) -> Result<AES128Key> {
    get_key(gen_app_key, [0; 16])
}

pub fn get_mc_root_key_for_app_key(app_key: AES128Key) -> Result<AES128Key> {
    get_key(app_key, [0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

pub fn get_mc_ke_key(mc_root_key: AES128Key) -> Result<AES128Key> {
    get_key(mc_root_key, [0; 16])
}

pub fn get_mc_app_s_key(mc_key: AES128Key, mc_addr: DevAddr) -> Result<AES128Key> {
    let mut b = [0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    b[1..5].copy_from_slice(&mc_addr.to_le_bytes());
    get_key(mc_key, b)
}

pub fn get_mc_net_s_key(mc_key: AES128Key, mc_addr: DevAddr) -> Result<AES128Key> {
    let mut b = [0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    b[1..5].copy_from_slice(&mc_addr.to_le_bytes());
    get_key(mc_key, b)
}

pub fn encrypt_mc_key(mc_ke_key: AES128Key, mc_key: AES128Key) -> [u8; 16] {
    let mc_ke_key_bytes = mc_ke_key.to_bytes();
    let mut mc_key_bytes = mc_key.to_bytes();

    let key = GenericArray::from_slice(&mc_ke_key_bytes);
    let cipher = Aes128::new(key);

    let block = Block::from_mut_slice(&mut mc_key_bytes);
    cipher.decrypt_block(block);

    mc_key_bytes
}

fn get_key(key: AES128Key, b: [u8; 16]) -> Result<AES128Key> {
    let key_bytes = key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b = b;
    let block = Block::from_mut_slice(&mut b);
    cipher.encrypt_block(block);

    Ok(AES128Key::from_slice(block)?)
}

#[cfg(test)]
mod test {
    use super::*;

    struct CommandTest {
        name: String,
        uplink: bool,
        command: Payload,
        bytes: Vec<u8>,
        expected_error: Option<String>,
    }

    #[test]
    fn test_package_version_req() {
        let encode_tests = [CommandTest {
            name: "encode PackageVersionReq".into(),
            uplink: false,
            command: Payload::PackageVersionReq,
            bytes: vec![0x00],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode PackageVersionReq".into(),
            uplink: false,
            command: Payload::PackageVersionReq,
            bytes: vec![0x00],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_package_version_ans() {
        let encode_tests = [CommandTest {
            name: "encode PackageVersionAns".into(),
            uplink: true,
            command: Payload::PackageVersionAns(PackageVersionAnsPayload {
                package_identifier: 1,
                package_version: 1,
            }),
            bytes: vec![0x00, 0x01, 0x01],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode PackageVersionAns".into(),
            uplink: true,
            command: Payload::PackageVersionAns(PackageVersionAnsPayload {
                package_identifier: 1,
                package_version: 1,
            }),
            bytes: vec![0x00, 0x01, 0x01],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_group_status_req() {
        let encode_tests = [CommandTest {
            name: "encode McGroupStatusReq".into(),
            uplink: false,
            command: Payload::McGroupStatusReq(McGroupStatusReqPayload {
                cmd_mask: McGroupStatusReqPayloadCmdMask {
                    req_group_mask: [true, true, false, false],
                },
            }),
            bytes: vec![0x01, 0x03],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McGroupStatusReq".into(),
            uplink: false,
            command: Payload::McGroupStatusReq(McGroupStatusReqPayload {
                cmd_mask: McGroupStatusReqPayloadCmdMask {
                    req_group_mask: [true, true, false, false],
                },
            }),
            bytes: vec![0x01, 0x03],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_group_status_ans() {
        let encode_tests = [CommandTest {
            name: "encode McGroupStatusAns".into(),
            uplink: true,
            command: Payload::McGroupStatusAns(McGroupStatusAnsPayload {
                status: McGroupStatusAnsPayloadStatus {
                    ans_group_mask: [true, true, false, false],
                    nb_total_groups: 2,
                },
                items: vec![
                    McGroupStatusAnsPayloadItem {
                        mc_group_id: 0,
                        mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
                    },
                    McGroupStatusAnsPayloadItem {
                        mc_group_id: 1,
                        mc_addr: DevAddr::from_be_bytes([2, 2, 3, 4]),
                    },
                ],
            }),
            bytes: vec![
                0x01, 0x23, 0x00, 0x04, 0x03, 0x02, 0x01, 0x01, 0x04, 0x03, 0x02, 0x02,
            ],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McGroupStatusAns".into(),
            uplink: true,
            command: Payload::McGroupStatusAns(McGroupStatusAnsPayload {
                status: McGroupStatusAnsPayloadStatus {
                    ans_group_mask: [true, true, false, false],
                    nb_total_groups: 2,
                },
                items: vec![
                    McGroupStatusAnsPayloadItem {
                        mc_group_id: 0,
                        mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
                    },
                    McGroupStatusAnsPayloadItem {
                        mc_group_id: 1,
                        mc_addr: DevAddr::from_be_bytes([2, 2, 3, 4]),
                    },
                ],
            }),
            bytes: vec![
                0x01, 0x23, 0x00, 0x04, 0x03, 0x02, 0x01, 0x01, 0x04, 0x03, 0x02, 0x02,
            ],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_group_setup_req() {
        let encode_tests = [CommandTest {
            name: "encode McGroupSetupReq".into(),
            uplink: false,
            command: Payload::McGroupSetupReq(McGroupSetupReqPayload {
                mc_group_id_header: McGroupSetupReqPayloadMcGroupIdHeader { mc_group_id: 2 },
                mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
                mc_key_encrypted: [1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                min_mc_f_count: 1024,
                max_mc_f_count: 2048,
            }),
            bytes: vec![
                0x02, 0x02, 0x04, 0x03, 0x02, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08,
                0x00, 0x00,
            ],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McGroupSetupReq".into(),
            uplink: false,
            command: Payload::McGroupSetupReq(McGroupSetupReqPayload {
                mc_group_id_header: McGroupSetupReqPayloadMcGroupIdHeader { mc_group_id: 2 },
                mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
                mc_key_encrypted: [1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                min_mc_f_count: 1024,
                max_mc_f_count: 2048,
            }),
            bytes: vec![
                0x02, 0x02, 0x04, 0x03, 0x02, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08,
                0x00, 0x00,
            ],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_group_setup_ans() {
        let encode_tests = [CommandTest {
            name: "encode McGroupSetupAns".into(),
            uplink: true,
            command: Payload::McGroupSetupAns(McGroupSetupAnsPayload {
                mc_group_id_header: McGroupSetupAnsPayloadMcGroupIdHeader {
                    mc_group_id: 2,
                    id_error: true,
                },
            }),
            bytes: vec![0x02, 0x06],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McGroupSetupAns".into(),
            uplink: true,
            command: Payload::McGroupSetupAns(McGroupSetupAnsPayload {
                mc_group_id_header: McGroupSetupAnsPayloadMcGroupIdHeader {
                    mc_group_id: 2,
                    id_error: true,
                },
            }),
            bytes: vec![0x02, 0x06],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_group_delete_req() {
        let encode_tests = [CommandTest {
            name: "encode McGroupDeleteReq".into(),
            uplink: false,
            command: Payload::McGroupDeleteReq(McGroupDeleteReqPayload {
                mc_group_id_header: McGroupDeleteReqPayloadMcGroupIdHeader { mc_group_id: 3 },
            }),
            bytes: vec![0x03, 0x03],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McGroupDeleteReq".into(),
            uplink: false,
            command: Payload::McGroupDeleteReq(McGroupDeleteReqPayload {
                mc_group_id_header: McGroupDeleteReqPayloadMcGroupIdHeader { mc_group_id: 3 },
            }),
            bytes: vec![0x03, 0x03],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_group_delete_ans() {
        let encode_tests = [CommandTest {
            name: "encode McGroupDeleteAns".into(),
            uplink: true,
            command: Payload::McGroupDeleteAns(McGroupDeleteAnsPayload {
                mc_group_id_header: McGroupDeleteAnsPayloadMcGroupIdHeader {
                    mc_group_id: 3,
                    mc_group_undefined: true,
                },
            }),
            bytes: vec![0x03, 0x07],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McGroupDeleteAns".into(),
            uplink: true,
            command: Payload::McGroupDeleteAns(McGroupDeleteAnsPayload {
                mc_group_id_header: McGroupDeleteAnsPayloadMcGroupIdHeader {
                    mc_group_id: 3,
                    mc_group_undefined: true,
                },
            }),
            bytes: vec![0x03, 0x07],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_class_c_session_req() {
        let encode_tests = [CommandTest {
            name: "encode McClassCSessionReq".into(),
            uplink: false,
            command: Payload::McClassCSessionReq(McClassCSessionReqPayload {
                mc_group_id_header: McClassCSessionReqPayloadMcGroupIdHeader { mc_group_id: 2 },
                session_time: 1024,
                session_time_out: McClassCSessionReqPayloadSessionTimeOut { time_out: 15 },
                dl_frequ: 868100000,
                dr: 5,
            }),
            bytes: vec![
                0x04, 0x02, 0x00, 0x04, 0x00, 0x00, 0x0f, 0x28, 0x76, 0x84, 0x05,
            ],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McClassCSessionReq".into(),
            uplink: false,
            command: Payload::McClassCSessionReq(McClassCSessionReqPayload {
                mc_group_id_header: McClassCSessionReqPayloadMcGroupIdHeader { mc_group_id: 2 },
                session_time: 1024,
                session_time_out: McClassCSessionReqPayloadSessionTimeOut { time_out: 15 },
                dl_frequ: 868100000,
                dr: 5,
            }),
            bytes: vec![
                0x04, 0x02, 0x00, 0x04, 0x00, 0x00, 0x0f, 0x28, 0x76, 0x84, 0x05,
            ],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_class_c_session_ans() {
        let encode_tests = [
            CommandTest {
                name: "encode McClassCSessionAns no error".into(),
                uplink: true,
                command: Payload::McClassCSessionAns(McClassCSessionAnsPayload {
                    status_and_mc_group_id: McClassCSessionAnsPayloadStatusAnsMcGroupId {
                        mc_group_id: 2,
                        dr_error: false,
                        freq_error: false,
                        mc_group_undefined: false,
                        start_missed: false,
                    },
                    time_to_start: Some(1024),
                }),
                bytes: vec![0x04, 0x02, 0x00, 0x04, 0x00],
                expected_error: None,
            },
            CommandTest {
                name: "encode McClassCSessionAns with error".into(),
                uplink: true,
                command: Payload::McClassCSessionAns(McClassCSessionAnsPayload {
                    status_and_mc_group_id: McClassCSessionAnsPayloadStatusAnsMcGroupId {
                        mc_group_id: 2,
                        dr_error: true,
                        freq_error: true,
                        mc_group_undefined: true,
                        start_missed: false,
                    },
                    time_to_start: None,
                }),
                bytes: vec![0x04, 0x1e],
                expected_error: None,
            },
        ];

        let decode_tests = [
            CommandTest {
                name: "decode McClassCSessionAns no error".into(),
                uplink: true,
                command: Payload::McClassCSessionAns(McClassCSessionAnsPayload {
                    status_and_mc_group_id: McClassCSessionAnsPayloadStatusAnsMcGroupId {
                        mc_group_id: 2,
                        dr_error: false,
                        freq_error: false,
                        mc_group_undefined: false,
                        start_missed: false,
                    },
                    time_to_start: Some(1024),
                }),
                bytes: vec![0x04, 0x02, 0x00, 0x04, 0x00],
                expected_error: None,
            },
            CommandTest {
                name: "decode McClassCSessionAns with error".into(),
                uplink: true,
                command: Payload::McClassCSessionAns(McClassCSessionAnsPayload {
                    status_and_mc_group_id: McClassCSessionAnsPayloadStatusAnsMcGroupId {
                        mc_group_id: 2,
                        dr_error: true,
                        freq_error: true,
                        mc_group_undefined: true,
                        start_missed: false,
                    },
                    time_to_start: None,
                }),
                bytes: vec![0x04, 0x1e],
                expected_error: None,
            },
        ];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_class_b_session_req() {
        let encode_tests = [CommandTest {
            name: "encode McClassBSessionReq".into(),
            uplink: false,
            command: Payload::McClassBSessionReq(McClassBSessionReqPayload {
                mc_group_id_header: McClassBSessionReqPayloadMcGroupIdHeader { mc_group_id: 3 },
                session_time: 1024,
                time_out_periodicity: McClassBSessionReqPayloadTimeOutPeriodicity {
                    time_out: 15,
                    periodicity: 4,
                },
                dl_frequ: 868100000,
                dr: 5,
            }),
            bytes: vec![
                0x05, 0x03, 0x00, 0x04, 0x00, 0x00, 0x4f, 0x28, 0x76, 0x84, 0x05,
            ],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode McClassBSessionReq".into(),
            uplink: false,
            command: Payload::McClassBSessionReq(McClassBSessionReqPayload {
                mc_group_id_header: McClassBSessionReqPayloadMcGroupIdHeader { mc_group_id: 3 },
                session_time: 1024,
                time_out_periodicity: McClassBSessionReqPayloadTimeOutPeriodicity {
                    time_out: 15,
                    periodicity: 4,
                },
                dl_frequ: 868100000,
                dr: 5,
            }),
            bytes: vec![
                0x05, 0x03, 0x00, 0x04, 0x00, 0x00, 0x4f, 0x28, 0x76, 0x84, 0x05,
            ],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_mc_class_b_session_ans() {
        let encode_tests = [
            CommandTest {
                name: "encode McClassBSessionAns no errors".into(),
                uplink: true,
                command: Payload::McClassBSessionAns(McClassBSessionAnsPayload {
                    status_and_mc_group_id: McClassBSessionAnsPayloadStatusAndMcGroupId {
                        mc_group_id: 3,
                        dr_error: false,
                        freq_error: false,
                        mc_group_undefined: false,
                        start_missed: false,
                    },
                    time_to_start: Some(1024),
                }),
                bytes: vec![0x05, 0x03, 0x00, 0x04, 0x00],
                expected_error: None,
            },
            CommandTest {
                name: "encode McClassBSessionAns with errors".into(),
                uplink: true,
                command: Payload::McClassBSessionAns(McClassBSessionAnsPayload {
                    status_and_mc_group_id: McClassBSessionAnsPayloadStatusAndMcGroupId {
                        mc_group_id: 3,
                        dr_error: true,
                        freq_error: true,
                        mc_group_undefined: true,
                        start_missed: false,
                    },
                    time_to_start: None,
                }),
                bytes: vec![0x05, 0x1f],
                expected_error: None,
            },
        ];

        let decode_tests = [
            CommandTest {
                name: "decode McClassBSessionAns no errors".into(),
                uplink: true,
                command: Payload::McClassBSessionAns(McClassBSessionAnsPayload {
                    status_and_mc_group_id: McClassBSessionAnsPayloadStatusAndMcGroupId {
                        mc_group_id: 3,
                        dr_error: false,
                        freq_error: false,
                        mc_group_undefined: false,
                        start_missed: false,
                    },
                    time_to_start: Some(1024),
                }),
                bytes: vec![0x05, 0x03, 0x00, 0x04, 0x00],
                expected_error: None,
            },
            CommandTest {
                name: "decode McClassBSessionAns with errors".into(),
                uplink: true,
                command: Payload::McClassBSessionAns(McClassBSessionAnsPayload {
                    status_and_mc_group_id: McClassBSessionAnsPayloadStatusAndMcGroupId {
                        mc_group_id: 3,
                        dr_error: true,
                        freq_error: true,
                        mc_group_undefined: true,
                        start_missed: false,
                    },
                    time_to_start: None,
                }),
                bytes: vec![0x05, 0x1f],
                expected_error: None,
            },
        ];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    const MC_ADDR: [u8; 4] = [1, 2, 3, 4];
    const MC_KEY: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    const APP_KEY: [u8; 16] = [2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    const GEN_APP_KEY: [u8; 16] = [3, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    const MC_ROOT_KEY: [u8; 16] = [4, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    #[test]
    fn test_get_mc_root_key_for_gen_app_key() {
        let key = get_mc_root_key_for_gen_app_key(AES128Key::from_bytes(GEN_APP_KEY)).unwrap();
        assert_eq!(
            AES128Key::from_bytes([
                0x55, 0x34, 0x4e, 0x82, 0x57, 0xe, 0xae, 0xc8, 0xbf, 0x3, 0xb9, 0x99, 0x62, 0xd1,
                0xf4, 0x45
            ]),
            key
        )
    }

    #[test]
    fn test_get_mc_root_key_for_app_key() {
        let key = get_mc_root_key_for_app_key(AES128Key::from_bytes(APP_KEY)).unwrap();
        assert_eq!(
            AES128Key::from_bytes([
                0x26, 0x4f, 0xd8, 0x59, 0x58, 0x3f, 0xcc, 0x67, 0x2, 0x41, 0xac, 0x7, 0x1c, 0xc9,
                0xf5, 0xbb
            ]),
            key
        );
    }

    #[test]
    fn test_get_mc_ke_key() {
        let key = get_mc_ke_key(AES128Key::from_bytes(MC_ROOT_KEY)).unwrap();
        assert_eq!(
            AES128Key::from_bytes([
                0x90, 0x83, 0xbe, 0xbf, 0x70, 0x42, 0x57, 0x88, 0x31, 0x60, 0xdb, 0xfc, 0xde, 0x33,
                0xad, 0x71
            ]),
            key
        );
    }

    #[test]
    fn test_get_mc_app_key() {
        let key = get_mc_app_s_key(
            AES128Key::from_bytes(MC_KEY),
            DevAddr::from_be_bytes(MC_ADDR),
        )
        .unwrap();
        assert_eq!(
            AES128Key::from_bytes([
                0x95, 0xcb, 0x45, 0x18, 0xee, 0x37, 0x56, 0x6, 0x73, 0x5b, 0xba, 0xcb, 0xdc, 0xe8,
                0x37, 0xfa
            ]),
            key
        );
    }

    #[test]
    fn test_get_mc_net_s_key() {
        let key = get_mc_net_s_key(
            AES128Key::from_bytes(MC_KEY),
            DevAddr::from_be_bytes(MC_ADDR),
        )
        .unwrap();
        assert_eq!(
            AES128Key::from_bytes([
                0xc3, 0xf6, 0xb3, 0x88, 0xba, 0xd6, 0xc0, 0x0, 0xb2, 0x32, 0x91, 0xad, 0x52, 0xc1,
                0x1c, 0x7b
            ]),
            key
        );
    }

    #[test]
    fn test_encrypt_mc_key() {
        let ke_key = AES128Key::from_bytes([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        let mc_key = AES128Key::from_bytes([2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);

        assert_eq!(
            [52, 55, 214, 226, 49, 215, 2, 65, 155, 81, 180, 148, 114, 113, 182, 17],
            encrypt_mc_key(ke_key, mc_key)
        );
    }

    fn run_tests_encode(tests: &[CommandTest]) {
        for tst in tests {
            println!("> {}", tst.name);
            let resp = tst.command.to_vec();
            if let Some(e) = &tst.expected_error {
                assert!(resp.is_err());
                assert_eq!(e, &resp.err().unwrap().to_string());
            } else {
                assert_eq!(tst.bytes, resp.unwrap());
            }
        }
    }

    fn run_tests_decode(tests: &[CommandTest]) {
        for tst in tests {
            println!("> {}", tst.name);
            let resp = Payload::from_slice(tst.uplink, &tst.bytes);
            if let Some(e) = &tst.expected_error {
                assert!(resp.is_err());
                assert_eq!(e, &resp.err().unwrap().to_string());
            } else {
                assert_eq!(tst.command, resp.unwrap());
            }
        }
    }
}
