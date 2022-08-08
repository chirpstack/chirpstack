use std::fmt;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use anyhow::Result;
use serde::Serialize;

use super::cflist::ChMask;
use super::dl_settings::DLSettings;

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CID {
    ResetInd,
    ResetConf,
    LinkCheckReq,
    LinkCheckAns,
    LinkADRReq,
    LinkADRAns,
    DutyCycleReq,
    DutyCycleAns,
    RxParamSetupReq,
    RxParamSetupAns,
    DevStatusReq,
    DevStatusAns,
    NewChannelReq,
    NewChannelAns,
    RxTimingSetupReq,
    RxTimingSetupAns,
    TxParamSetupReq,
    TxParamSetupAns,
    DlChannelReq,
    DlChannelAns,
    RekeyConf,
    RekeyInd,
    ADRParamSetupReq,
    ADRParamSetupAns,
    DeviceTimeReq,
    DeviceTimeAns,
    ForceRejoinReq,
    RejoinParamSetupReq,
    RejoinParamSetupAns,
    PingSlotInfoReq,
    PingSlotInfoAns,
    PingSlotChannelReq,
    PingSlotChannelAns,
    BeaconFreqReq,
    BeaconFreqAns,
    DeviceModeInd,
    DeviceModeConf,
    Raw,
}

impl fmt::Display for CID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CID {
    pub fn byte(&self) -> u8 {
        match self {
            CID::ResetInd | CID::ResetConf => 0x01,
            CID::LinkCheckReq | CID::LinkCheckAns => 0x02,
            CID::LinkADRReq | CID::LinkADRAns => 0x03,
            CID::DutyCycleReq | CID::DutyCycleAns => 0x04,
            CID::RxParamSetupReq | CID::RxParamSetupAns => 0x05,
            CID::DevStatusReq | CID::DevStatusAns => 0x06,
            CID::NewChannelReq | CID::NewChannelAns => 0x07,
            CID::RxTimingSetupReq | CID::RxTimingSetupAns => 0x08,
            CID::TxParamSetupReq | CID::TxParamSetupAns => 0x09,
            CID::DlChannelReq | CID::DlChannelAns => 0x0a,
            CID::RekeyConf | CID::RekeyInd => 0x0b,
            CID::ADRParamSetupReq | CID::ADRParamSetupAns => 0x0c,
            CID::DeviceTimeReq | CID::DeviceTimeAns => 0x0d,
            CID::ForceRejoinReq => 0x0e,
            CID::RejoinParamSetupReq | CID::RejoinParamSetupAns => 0x0f,
            CID::PingSlotInfoReq | CID::PingSlotInfoAns => 0x10,
            CID::PingSlotChannelReq | CID::PingSlotChannelAns => 0x11,
            CID::BeaconFreqReq | CID::BeaconFreqAns => 0x13, // 0x12 is deprecated
            CID::DeviceModeInd | CID::DeviceModeConf => 0x20,
            CID::Raw => 0xff,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum MACCommand {
    ResetInd(ResetIndPayload),
    ResetConf(ResetConfPayload),
    LinkCheckReq,
    LinkCheckAns(LinkCheckAnsPayload),
    LinkADRReq(LinkADRReqPayload),
    LinkADRAns(LinkADRAnsPayload),
    DutyCycleReq(DutyCycleReqPayload),
    DutyCycleAns,
    RxParamSetupReq(RxParamSetupReqPayload),
    RxParamSetupAns(RxParamSetupAnsPayload),
    DevStatusReq,
    DevStatusAns(DevStatusAnsPayload),
    NewChannelReq(NewChannelReqPayload),
    NewChannelAns(NewChannelAnsPayload),
    RxTimingSetupReq(RxTimingSetupReqPayload),
    RxTimingSetupAns,
    TxParamSetupReq(TxParamSetupReqPayload),
    TxParamSetupAns,
    DlChannelReq(DlChannelReqPayload),
    DlChannelAns(DlChannelAnsPayload),
    RekeyConf(RekeyConfPayload),
    RekeyInd(RekeyIndPayload),
    ADRParamSetupReq(ADRParamSetupReqPayload),
    ADRParamSetupAns,
    DeviceTimeReq,
    DeviceTimeAns(DeviceTimeAnsPayload),
    ForceRejoinReq(ForceRejoinReqPayload),
    RejoinParamSetupReq(RejoinParamSetupReqPayload),
    RejoinParamSetupAns(RejoinParamSetupAnsPayload),
    PingSlotInfoReq(PingSlotInfoReqPayload),
    PingSlotInfoAns,
    PingSlotChannelReq(PingSlotChannelReqPayload),
    PingSlotChannelAns(PingSlotChannelAnsPayload),
    BeaconFreqReq(BeaconFreqReqPayload),
    BeaconFreqAns(BeaconFreqAnsPayload),
    DeviceModeInd(DeviceModeIndPayload),
    DeviceModeConf(DeviceModeConfPayload),
    Raw(Vec<u8>),
}

impl MACCommand {
    pub fn cid(&self) -> CID {
        match self {
            MACCommand::ResetInd(_) => CID::ResetInd,
            MACCommand::ResetConf(_) => CID::ResetConf,
            MACCommand::LinkCheckReq => CID::LinkCheckReq,
            MACCommand::LinkCheckAns(_) => CID::LinkCheckAns,
            MACCommand::LinkADRReq(_) => CID::LinkADRReq,
            MACCommand::LinkADRAns(_) => CID::LinkADRAns,
            MACCommand::DutyCycleReq(_) => CID::DutyCycleReq,
            MACCommand::DutyCycleAns => CID::DutyCycleAns,
            MACCommand::RxParamSetupReq(_) => CID::RxParamSetupReq,
            MACCommand::RxParamSetupAns(_) => CID::RxParamSetupAns,
            MACCommand::DevStatusReq => CID::DevStatusReq,
            MACCommand::DevStatusAns(_) => CID::DevStatusAns,
            MACCommand::NewChannelReq(_) => CID::NewChannelReq,
            MACCommand::NewChannelAns(_) => CID::NewChannelAns,
            MACCommand::RxTimingSetupReq(_) => CID::RxTimingSetupReq,
            MACCommand::RxTimingSetupAns => CID::RxTimingSetupAns,
            MACCommand::TxParamSetupReq(_) => CID::TxParamSetupReq,
            MACCommand::TxParamSetupAns => CID::TxParamSetupAns,
            MACCommand::DlChannelReq(_) => CID::DlChannelReq,
            MACCommand::DlChannelAns(_) => CID::DlChannelAns,
            MACCommand::RekeyConf(_) => CID::RekeyConf,
            MACCommand::RekeyInd(_) => CID::RekeyInd,
            MACCommand::ADRParamSetupReq(_) => CID::ADRParamSetupReq,
            MACCommand::ADRParamSetupAns => CID::ADRParamSetupAns,
            MACCommand::DeviceTimeReq => CID::DeviceTimeReq,
            MACCommand::DeviceTimeAns(_) => CID::DeviceTimeAns,
            MACCommand::ForceRejoinReq(_) => CID::ForceRejoinReq,
            MACCommand::RejoinParamSetupReq(_) => CID::RejoinParamSetupReq,
            MACCommand::RejoinParamSetupAns(_) => CID::RejoinParamSetupAns,
            MACCommand::PingSlotInfoReq(_) => CID::PingSlotInfoReq,
            MACCommand::PingSlotInfoAns => CID::PingSlotInfoAns,
            MACCommand::PingSlotChannelReq(_) => CID::PingSlotChannelReq,
            MACCommand::PingSlotChannelAns(_) => CID::PingSlotChannelAns,
            MACCommand::BeaconFreqReq(_) => CID::BeaconFreqReq,
            MACCommand::BeaconFreqAns(_) => CID::BeaconFreqAns, // 0x12 is deprecated
            MACCommand::DeviceModeInd(_) => CID::DeviceModeInd,
            MACCommand::DeviceModeConf(_) => CID::DeviceModeConf,
            MACCommand::Raw(_) => CID::Raw,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum Version {
    LoRaWAN1_1,
}

impl Version {
    pub fn to_u8(&self) -> u8 {
        match self {
            Version::LoRaWAN1_1 => 0x01,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self> {
        Ok(match v {
            0x01 => Version::LoRaWAN1_1,
            _ => {
                return Err(anyhow!("invalid version"));
            }
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum DwellTime {
    NoLimit,
    Limit400ms,
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum DeviceModeClass {
    ClassA,
    ClassC,
}

impl DeviceModeClass {
    pub fn to_u8(&self) -> u8 {
        match self {
            DeviceModeClass::ClassA => 0x00,
            DeviceModeClass::ClassC => 0x02,
        }
    }

    pub fn from_u8(v: u8) -> Result<Self> {
        Ok(match v {
            0x00 => DeviceModeClass::ClassA,
            0x02 => DeviceModeClass::ClassC,
            _ => {
                return Err(anyhow!("invalid device mode"));
            }
        })
    }
}

impl fmt::Display for DeviceModeClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DeviceModeClass::ClassA => "A",
                DeviceModeClass::ClassC => "C",
            }
        )
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct MACCommandSet(Vec<MACCommand>);

impl Deref for MACCommandSet {
    type Target = Vec<MACCommand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MACCommandSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MACCommandSet {
    pub fn new(macs: Vec<MACCommand>) -> Self {
        MACCommandSet(macs)
    }

    pub fn size(&self) -> Result<usize> {
        let b = self.to_vec()?;
        Ok(b.len())
    }

    pub fn from_slice(b: &[u8]) -> Self {
        // For LoRaWAN 1.1, this payload must be first decrypted before it can be parsed.
        // Therefore, the decoding into separate mac-commands is handled by decode_from_raw.
        MACCommandSet(vec![MACCommand::Raw(b.to_vec())])
    }

    pub fn push(&mut self, m: MACCommand) {
        self.0.push(m);
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();

        for mac in &self.0 {
            match mac {
                MACCommand::ResetInd(pl) => {
                    out.push(0x01);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::ResetConf(pl) => {
                    out.push(0x01);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::LinkCheckReq => {
                    out.push(0x02);
                }
                MACCommand::LinkCheckAns(pl) => {
                    out.push(0x02);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::LinkADRReq(pl) => {
                    out.push(0x03);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::LinkADRAns(pl) => {
                    out.push(0x03);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::DutyCycleReq(pl) => {
                    out.push(0x04);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::DutyCycleAns => {
                    out.push(0x04);
                }
                MACCommand::RxParamSetupReq(pl) => {
                    out.push(0x05);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::RxParamSetupAns(pl) => {
                    out.push(0x05);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::DevStatusReq => {
                    out.push(0x06);
                }
                MACCommand::DevStatusAns(pl) => {
                    out.push(0x06);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::NewChannelReq(pl) => {
                    out.push(0x07);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::NewChannelAns(pl) => {
                    out.push(0x07);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::RxTimingSetupReq(pl) => {
                    out.push(0x08);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::RxTimingSetupAns => {
                    out.push(0x08);
                }
                MACCommand::TxParamSetupReq(pl) => {
                    out.push(0x09);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::TxParamSetupAns => {
                    out.push(0x09);
                }
                MACCommand::DlChannelReq(pl) => {
                    out.push(0x0a);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::DlChannelAns(pl) => {
                    out.push(0x0a);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::RekeyConf(pl) => {
                    out.push(0x0b);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::RekeyInd(pl) => {
                    out.push(0x0b);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::ADRParamSetupReq(pl) => {
                    out.push(0x0c);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::ADRParamSetupAns => {
                    out.push(0x0c);
                }
                MACCommand::DeviceTimeReq => {
                    out.push(0x0d);
                }
                MACCommand::DeviceTimeAns(pl) => {
                    out.push(0x0d);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::ForceRejoinReq(pl) => {
                    out.push(0x0e);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::RejoinParamSetupReq(pl) => {
                    out.push(0x0f);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::RejoinParamSetupAns(pl) => {
                    out.push(0x0f);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::PingSlotInfoReq(pl) => {
                    out.push(0x10);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::PingSlotInfoAns => {
                    out.push(0x10);
                }
                MACCommand::PingSlotChannelReq(pl) => {
                    out.push(0x11);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::PingSlotChannelAns(pl) => {
                    out.push(0x11);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::BeaconFreqReq(pl) => {
                    out.push(0x13);
                    out.extend_from_slice(&pl.to_bytes()?);
                }
                MACCommand::BeaconFreqAns(pl) => {
                    out.push(0x13);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::DeviceModeInd(pl) => {
                    out.push(0x20);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::DeviceModeConf(pl) => {
                    out.push(0x20);
                    out.extend_from_slice(&pl.to_bytes());
                }
                MACCommand::Raw(v) => out.extend_from_slice(v),
            };
        }

        Ok(out)
    }

    pub fn decode_from_raw(&mut self, uplink: bool) -> Result<()> {
        // nothing to parse
        if self.0.is_empty() {
            return Ok(());
        }

        // in any other case there must be exactly one MACCommand::Raw.
        if self.0.len() == 1 {
            if let MACCommand::Raw(b) = &self.0[0] {
                let mut index = 0;
                let mut commands = vec![];
                let len = b.len();

                loop {
                    if index == len {
                        break;
                    }

                    let cid_index = index;
                    let pl_index = cid_index + 1;

                    match uplink {
                        true => match b[cid_index] {
                            0x01 => {
                                index += ResetIndPayload::SIZE;
                                commands.push(MACCommand::ResetInd(ResetIndPayload::from_slice(
                                    try_slice(b, pl_index, index + 1)?,
                                )?));
                            }
                            0x02 => {
                                commands.push(MACCommand::LinkCheckReq);
                            }
                            0x03 => {
                                index += LinkADRAnsPayload::SIZE;
                                commands.push(MACCommand::LinkADRAns(
                                    LinkADRAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x04 => {
                                commands.push(MACCommand::DutyCycleAns);
                            }
                            0x05 => {
                                index += RxParamSetupAnsPayload::SIZE;
                                commands.push(MACCommand::RxParamSetupAns(
                                    RxParamSetupAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x06 => {
                                index += DevStatusAnsPayload::SIZE;
                                commands.push(MACCommand::DevStatusAns(
                                    DevStatusAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x07 => {
                                index += NewChannelAnsPayload::SIZE;
                                commands.push(MACCommand::NewChannelAns(
                                    NewChannelAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x08 => {
                                commands.push(MACCommand::RxTimingSetupAns);
                            }
                            0x09 => {
                                commands.push(MACCommand::TxParamSetupAns);
                            }
                            0x0a => {
                                index += DlChannelAnsPayload::SIZE;
                                commands.push(MACCommand::DlChannelAns(
                                    DlChannelAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x0b => {
                                index += RekeyIndPayload::SIZE;
                                commands.push(MACCommand::RekeyInd(RekeyIndPayload::from_slice(
                                    try_slice(b, pl_index, index + 1)?,
                                )?));
                            }
                            0x0c => {
                                commands.push(MACCommand::ADRParamSetupAns);
                            }
                            0x0d => {
                                commands.push(MACCommand::DeviceTimeReq);
                            }
                            0x0f => {
                                index += RejoinParamSetupAnsPayload::SIZE;
                                commands.push(MACCommand::RejoinParamSetupAns(
                                    RejoinParamSetupAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x10 => {
                                index += PingSlotInfoReqPayload::SIZE;
                                commands.push(MACCommand::PingSlotInfoReq(
                                    PingSlotInfoReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x11 => {
                                index += PingSlotChannelAnsPayload::SIZE;
                                commands.push(MACCommand::PingSlotChannelAns(
                                    PingSlotChannelAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x13 => {
                                index += BeaconFreqAnsPayload::SIZE;
                                commands.push(MACCommand::BeaconFreqAns(
                                    BeaconFreqAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x20 => {
                                index += DeviceModeIndPayload::SIZE;
                                commands.push(MACCommand::DeviceModeInd(
                                    DeviceModeIndPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            _ => {
                                index += b[index..].len() - 1;
                                commands.push(MACCommand::Raw(b[index..].to_vec()));
                            }
                        },
                        false => match b[index] {
                            0x01 => {
                                index += ResetConfPayload::SIZE;
                                commands.push(MACCommand::ResetConf(ResetConfPayload::from_slice(
                                    try_slice(b, pl_index, index + 1)?,
                                )?));
                            }
                            0x02 => {
                                index += LinkCheckAnsPayload::SIZE;
                                commands.push(MACCommand::LinkCheckAns(
                                    LinkCheckAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x03 => {
                                index += LinkADRReqPayload::SIZE;
                                commands.push(MACCommand::LinkADRReq(
                                    LinkADRReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x04 => {
                                index += DutyCycleReqPayload::SIZE;
                                commands.push(MACCommand::DutyCycleReq(
                                    DutyCycleReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x05 => {
                                index += RxParamSetupReqPayload::SIZE;
                                commands.push(MACCommand::RxParamSetupReq(
                                    RxParamSetupReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x06 => {
                                commands.push(MACCommand::DevStatusReq);
                            }
                            0x07 => {
                                index += NewChannelReqPayload::SIZE;
                                commands.push(MACCommand::NewChannelReq(
                                    NewChannelReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x08 => {
                                index += RxTimingSetupReqPayload::SIZE;
                                commands.push(MACCommand::RxTimingSetupReq(
                                    RxTimingSetupReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x09 => {
                                index += TxParamSetupReqPayload::SIZE;
                                commands.push(MACCommand::TxParamSetupReq(
                                    TxParamSetupReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x0a => {
                                index += DlChannelReqPayload::SIZE;
                                commands.push(MACCommand::DlChannelReq(
                                    DlChannelReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x0b => {
                                index += RekeyConfPayload::SIZE;
                                commands.push(MACCommand::RekeyConf(RekeyConfPayload::from_slice(
                                    try_slice(b, pl_index, index + 1)?,
                                )?));
                            }
                            0x0c => {
                                index += ADRParamSetupReqPayload::SIZE;
                                commands.push(MACCommand::ADRParamSetupReq(
                                    ADRParamSetupReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x0d => {
                                index += DeviceTimeAnsPayload::SIZE;
                                commands.push(MACCommand::DeviceTimeAns(
                                    DeviceTimeAnsPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x0e => {
                                index += ForceRejoinReqPayload::SIZE;
                                commands.push(MACCommand::ForceRejoinReq(
                                    ForceRejoinReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x0f => {
                                index += RejoinParamSetupReqPayload::SIZE;
                                commands.push(MACCommand::RejoinParamSetupReq(
                                    RejoinParamSetupReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x10 => {
                                commands.push(MACCommand::PingSlotInfoAns);
                            }
                            0x11 => {
                                index += PingSlotChannelReqPayload::SIZE;
                                commands.push(MACCommand::PingSlotChannelReq(
                                    PingSlotChannelReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x13 => {
                                index += BeaconFreqReqPayload::SIZE;
                                commands.push(MACCommand::BeaconFreqReq(
                                    BeaconFreqReqPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            0x20 => {
                                index += DeviceModeConfPayload::SIZE;
                                commands.push(MACCommand::DeviceModeConf(
                                    DeviceModeConfPayload::from_slice(try_slice(
                                        b,
                                        pl_index,
                                        index + 1,
                                    )?)?,
                                ));
                            }
                            _ => {
                                index += b[index..].len() - 1;
                                commands.push(MACCommand::Raw(b[index..].to_vec()));
                            }
                        },
                    }

                    // CID byte
                    index += 1;
                }

                self.0 = commands;
                return Ok(());
            }
        }

        return Err(anyhow!(
            "MACCommandSet must contain exactly 1 MACCommand::Raw for decoding"
        ));
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ResetIndPayload {
    pub dev_lorawan_version: Version,
}

impl ResetIndPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ResetIndPayload expects 1 byte"));
        }

        Ok(ResetIndPayload {
            dev_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.dev_lorawan_version.to_u8()]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ResetConfPayload {
    pub serv_lorawan_version: Version,
}

impl ResetConfPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ResetConfPayload expects 1 byte"));
        }

        Ok(ResetConfPayload {
            serv_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.serv_lorawan_version.to_u8()]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LinkCheckAnsPayload {
    pub margin: u8,
    pub gw_cnt: u8,
}

impl LinkCheckAnsPayload {
    const SIZE: usize = 2;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("LinkCheckAnsPayload expects 2 bytes"));
        }

        Ok(LinkCheckAnsPayload {
            margin: b[0],
            gw_cnt: b[1],
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.margin, self.gw_cnt]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LinkADRReqPayload {
    pub dr: u8,
    pub tx_power: u8,
    pub ch_mask: ChMask,
    pub redundancy: Redundancy,
}

impl LinkADRReqPayload {
    const SIZE: usize = 4;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("LinkADRReqPayload expects 4 bytes"));
        }

        Ok(LinkADRReqPayload {
            dr: (b[0] & 0xf0) >> 4,
            tx_power: b[0] & 0x0f,
            ch_mask: ChMask::from_bytes([b[1], b[2]]),
            redundancy: Redundancy::from_bytes([b[3]]),
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        let mut b: [u8; Self::SIZE] = [0; Self::SIZE];

        if self.dr > 15 {
            return Err(anyhow!("max value of dr is 15"));
        }

        if self.tx_power > 15 {
            return Err(anyhow!("max value of tx_power is 15"));
        }

        b[0] = self.tx_power | (self.dr << 4);
        b[1..3].clone_from_slice(&self.ch_mask.to_bytes());
        b[3..].clone_from_slice(&self.redundancy.to_bytes()?);

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Redundancy {
    pub ch_mask_cntl: u8,
    pub nb_rep: u8,
}

impl Redundancy {
    const SIZE: usize = 1;

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.nb_rep > 15 {
            return Err(anyhow!("max value of nb_rep is 15"));
        }
        if self.ch_mask_cntl > 7 {
            return Err(anyhow!("max value of ch_mask_cntl is 7"));
        }

        Ok([self.nb_rep | (self.ch_mask_cntl << 4)])
    }

    pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
        Redundancy {
            nb_rep: b[0] & 0x0f,
            ch_mask_cntl: (b[0] & 0x70) >> 4,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct LinkADRAnsPayload {
    pub ch_mask_ack: bool,
    pub dr_ack: bool,
    pub tx_power_ack: bool,
}

impl LinkADRAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("LinkADRAnsPayload expects 1 byte"));
        }

        Ok(LinkADRAnsPayload {
            ch_mask_ack: b[0] & 0x01 != 0,
            dr_ack: b[0] & 0x02 != 0,
            tx_power_ack: b[0] & 0x04 != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b: u8 = 0;

        if self.ch_mask_ack {
            b |= 0x01;
        }
        if self.dr_ack {
            b |= 0x02;
        }
        if self.tx_power_ack {
            b |= 0x04;
        }

        [b]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DutyCycleReqPayload {
    pub max_duty_cycle: u8,
}

impl DutyCycleReqPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DutyCycleReqPayload expects 1 byte"));
        }

        Ok(DutyCycleReqPayload {
            max_duty_cycle: b[0],
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.max_duty_cycle > 15 && self.max_duty_cycle != 255 {
            return Err(anyhow!("max_duty_cycle must have value 0 - 15 or 255"));
        }

        Ok([self.max_duty_cycle])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RxParamSetupReqPayload {
    pub frequency: u32,
    pub dl_settings: DLSettings,
}

impl RxParamSetupReqPayload {
    const SIZE: usize = 4;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RxParamSetupReqPayload expects 4 bytes"));
        }

        Ok(RxParamSetupReqPayload {
            dl_settings: DLSettings::from_le_bytes([b[0]]),
            frequency: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[1..]);
                u32::from_le_bytes(freq_b) * 100
            },
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.frequency / 100 >= (1 << 24) {
            return Err(anyhow!("max frequency value is 2^24-1"));
        }
        if self.frequency % 100 != 0 {
            return Err(anyhow!("frequency must be a multiple of 100"));
        }

        let mut b: [u8; Self::SIZE] = [0; Self::SIZE];
        b[0..1].copy_from_slice(&self.dl_settings.to_le_bytes()?);

        let freq_b = (self.frequency / 100).to_le_bytes();
        b[1..4].copy_from_slice(&freq_b[0..3]);
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RxParamSetupAnsPayload {
    pub channel_ack: bool,
    pub rx2_dr_ack: bool,
    pub rx1_dr_offset_ack: bool,
}

impl RxParamSetupAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RxParamSetupAnsPayload expects 1 byte"));
        }

        Ok(RxParamSetupAnsPayload {
            channel_ack: b[0] & 0x01 != 0,
            rx2_dr_ack: b[0] & 0x02 != 0,
            rx1_dr_offset_ack: b[0] & 0x04 != 0,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        let mut b: u8 = 0;
        if self.channel_ack {
            b |= 0x01;
        }
        if self.rx2_dr_ack {
            b |= 0x02;
        }
        if self.rx1_dr_offset_ack {
            b |= 0x04;
        }
        Ok([b])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DevStatusAnsPayload {
    pub battery: u8,
    pub margin: i8,
}

impl DevStatusAnsPayload {
    const SIZE: usize = 2;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DevStatusAnsPayload expects 2 bytes"));
        }

        Ok(DevStatusAnsPayload {
            battery: b[0],
            margin: {
                if b[1] > 31 {
                    (b[1] as i8) - 64
                } else {
                    b[1] as i8
                }
            },
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.margin < -32 {
            return Err(anyhow!("min margin value is -32"));
        }
        if self.margin > 31 {
            return Err(anyhow!("max margin value is 31"));
        }

        Ok([self.battery, {
            if self.margin < 0 {
                (self.margin + 64) as u8
            } else {
                self.margin as u8
            }
        }])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct NewChannelReqPayload {
    pub ch_index: u8,
    pub freq: u32,
    pub min_dr: u8,
    pub max_dr: u8,
}

impl NewChannelReqPayload {
    const SIZE: usize = 5;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("NewChannelReqPayload expects 5 bytes"));
        }

        Ok(NewChannelReqPayload {
            ch_index: b[0],
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[1..4]);
                let freq = u32::from_le_bytes(freq_b);

                if freq >= 12000000 {
                    // 2.4GHz frequency
                    freq * 200
                } else {
                    freq * 100
                }
            },
            min_dr: b[4] & 0x0f,
            max_dr: (b[4] & 0xf0) >> 4,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        let mut freq = self.freq;

        // Support LoRaWAN 2.4GHz, in which case the stepping is 200Hz:
        // See Frequency Encoding in MAC Commands
        // https://lora-developers.semtech.com/documentation/tech-papers-and-guides/physical-layer-proposal-2.4ghz/
        if freq >= 2400000000 {
            freq = freq / 2;
        }

        if freq / 100 >= (1 << 24) {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if freq % 100 != 0 {
            return Err(anyhow!("freq must be multiple of 100"));
        }
        if self.min_dr > 15 {
            return Err(anyhow!("max min_dr value is 15"));
        }
        if self.max_dr > 15 {
            return Err(anyhow!("max max_dr value is 15"));
        }

        let mut b: [u8; Self::SIZE] = [0; Self::SIZE];
        b[0] = self.ch_index;
        b[1..5].copy_from_slice(&(freq / 100).to_le_bytes());
        b[4] = self.min_dr | (self.max_dr << 4);

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct NewChannelAnsPayload {
    pub channel_freq_ok: bool,
    pub dr_range_ok: bool,
}

impl NewChannelAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("NewChannelAnsPayload expects 1 byte"));
        }

        Ok(NewChannelAnsPayload {
            channel_freq_ok: b[0] & 0x01 != 0,
            dr_range_ok: b[0] & 0x02 != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b: [u8; Self::SIZE] = [0];
        if self.channel_freq_ok {
            b[0] = 0x01;
        }
        if self.dr_range_ok {
            b[0] |= 0x02;
        }
        b
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RxTimingSetupReqPayload {
    pub delay: u8,
}

impl RxTimingSetupReqPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RxTimingSetupReqPayload expects 1 byte"));
        }

        Ok(RxTimingSetupReqPayload { delay: b[0] })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.delay > 15 {
            return Err(anyhow!("max delay value is 15"));
        }

        Ok([self.delay])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct TxParamSetupReqPayload {
    pub uplink_dwell_time: DwellTime,
    pub downlink_dwell_time: DwellTime,
    pub max_eirp: u8,
}

impl TxParamSetupReqPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("TxParamSetupReqPayload expects 1 byte"));
        }

        Ok(TxParamSetupReqPayload {
            uplink_dwell_time: {
                if b[0] & 0x10 != 0 {
                    DwellTime::Limit400ms
                } else {
                    DwellTime::NoLimit
                }
            },
            downlink_dwell_time: {
                if b[0] & 0x20 != 0 {
                    DwellTime::Limit400ms
                } else {
                    DwellTime::NoLimit
                }
            },
            max_eirp: b[0] & 0x0f,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.max_eirp > 15 {
            return Err(anyhow!("max max_eirp value is 15"));
        }

        let mut b: [u8; Self::SIZE] = [self.max_eirp];
        if self.uplink_dwell_time == DwellTime::Limit400ms {
            b[0] |= 0x10;
        }
        if self.downlink_dwell_time == DwellTime::Limit400ms {
            b[0] |= 0x20;
        }

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DlChannelReqPayload {
    pub ch_index: u8,
    pub freq: u32,
}

impl DlChannelReqPayload {
    const SIZE: usize = 4;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DlChannelReqPayload expects 4 bytes"));
        }

        Ok(DlChannelReqPayload {
            ch_index: b[0],
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[1..4]);
                u32::from_le_bytes(freq_b) * 100
            },
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.freq / 100 >= 1 << 24 {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if self.freq % 100 != 0 {
            return Err(anyhow!("freq must be a multiple of 100"));
        }

        let mut b: [u8; Self::SIZE] = [0; Self::SIZE];
        b[0] = self.ch_index;

        let freq_b = (self.freq / 100).to_le_bytes();
        b[1..4].copy_from_slice(&freq_b[0..3]);

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DlChannelAnsPayload {
    pub uplink_freq_exists: bool,
    pub channel_freq_ok: bool,
}

impl DlChannelAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DlChannelReqPayload expects 1 byte"));
        }

        Ok(DlChannelAnsPayload {
            channel_freq_ok: b[0] & 0x01 != 0,
            uplink_freq_exists: b[0] & 0x02 != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b: u8 = 0;

        if self.channel_freq_ok {
            b |= 0x01;
        }
        if self.uplink_freq_exists {
            b |= 0x02;
        }

        [b]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RekeyConfPayload {
    pub serv_lorawan_version: Version,
}

impl RekeyConfPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RekeyConfPayload expects 1 byte"));
        }

        Ok(RekeyConfPayload {
            serv_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.serv_lorawan_version.to_u8()]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RekeyIndPayload {
    pub dev_lorawan_version: Version,
}

impl RekeyIndPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RekeyIndPayload expects 1 byte"));
        }

        Ok(RekeyIndPayload {
            dev_lorawan_version: Version::from_u8(b[0])?,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.dev_lorawan_version.to_u8()]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ADRParamSetupReqPayload {
    pub adr_param: ADRParam,
}

impl ADRParamSetupReqPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ADRParamSetupReqPayload expects 1 byte"));
        }

        Ok(ADRParamSetupReqPayload {
            adr_param: ADRParam::from_slice(b)?,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        self.adr_param.to_bytes()
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ADRParam {
    pub limit_exp: u8,
    pub delay_exp: u8,
}

impl ADRParam {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ADRParam expects 1 byte"));
        }

        Ok(ADRParam {
            delay_exp: b[0] & 0x0f,
            limit_exp: b[0] >> 4,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.limit_exp > 15 {
            return Err(anyhow!("max limit_exp value is 15"));
        }
        if self.delay_exp > 15 {
            return Err(anyhow!("max delay_exp value is 15"));
        }

        Ok([self.delay_exp | (self.limit_exp << 4)])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DeviceTimeAnsPayload {
    pub time_since_gps_epoch: Duration,
}

impl DeviceTimeAnsPayload {
    const SIZE: usize = 5;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DeviceTimeAnsPayload expects 5 bytes"));
        }

        let secs = {
            let mut secs_b: [u8; 4] = [0; 4];
            secs_b.copy_from_slice(&b[0..4]);
            u32::from_le_bytes(secs_b)
        } as u64;

        let nanos = (b[4] as u32) * 3906250; // second / 256

        Ok(DeviceTimeAnsPayload {
            time_since_gps_epoch: Duration::new(secs, nanos),
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b: [u8; Self::SIZE] = [0; Self::SIZE];
        b[0..4].copy_from_slice(&(self.time_since_gps_epoch.as_secs() as u32).to_le_bytes());
        b[4] = ((self.time_since_gps_epoch.as_nanos() % 1_000_000_000) / 3906250) as u8;
        b
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ForceRejoinReqPayload {
    pub period: u8,
    pub max_retries: u8,
    pub rejoin_type: u8,
    pub dr: u8,
}

impl ForceRejoinReqPayload {
    const SIZE: usize = 2;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("ForceRejoinReqPayload expects 2 bytes"));
        }

        Ok(ForceRejoinReqPayload {
            dr: b[0] & 0x0f,
            rejoin_type: (b[0] & 0x70) >> 4,
            max_retries: b[1] & 0x07,
            period: (b[1] & 0x38) >> 3,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.period > 7 {
            return Err(anyhow!("max period value is 7"));
        }
        if self.max_retries > 7 {
            return Err(anyhow!("max max_retries value is 7"));
        }
        if self.rejoin_type != 0 && self.rejoin_type != 2 {
            return Err(anyhow!("rejoin_type must be 0 or 2"));
        }
        if self.dr > 15 {
            return Err(anyhow!("max dr value is 15"));
        }

        Ok([
            self.dr | (self.rejoin_type << 4),
            self.max_retries | (self.period << 3),
        ])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RejoinParamSetupReqPayload {
    pub max_time_n: u8,
    pub max_count_n: u8,
}

impl RejoinParamSetupReqPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RejoinParamSetupReq expects 1 byte"));
        }

        Ok(RejoinParamSetupReqPayload {
            max_count_n: b[0] & 0x0f,
            max_time_n: (b[0] & 0xf0) >> 4,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.max_time_n > 15 {
            return Err(anyhow!("max max_time_n value is 15"));
        }
        if self.max_count_n > 15 {
            return Err(anyhow!("max max_count_n value is 15"));
        }

        Ok([self.max_count_n | (self.max_time_n << 4)])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RejoinParamSetupAnsPayload {
    pub time_ok: bool,
}

impl RejoinParamSetupAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("RejoinParamSetupAnsPayload expects 1 byte"));
        }

        Ok(RejoinParamSetupAnsPayload {
            time_ok: b[0] & 0x01 != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b: u8 = 0;
        if self.time_ok {
            b = 0x01;
        }
        [b]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct PingSlotInfoReqPayload {
    pub periodicity: u8,
}

impl PingSlotInfoReqPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("PingSlotInfoReqPayload expects 1 byte"));
        }

        Ok(PingSlotInfoReqPayload {
            periodicity: b[0] & 0x07,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.periodicity > 7 {
            return Err(anyhow!("max periodicity value is 7"));
        }

        Ok([self.periodicity])
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct PingSlotChannelReqPayload {
    pub freq: u32,
    pub dr: u8,
}

impl PingSlotChannelReqPayload {
    const SIZE: usize = 4;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("PingSlotChannelReqPayload expects 4 btes"));
        }

        Ok(PingSlotChannelReqPayload {
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(&b[0..3]);
                u32::from_le_bytes(freq_b) * 100
            },
            dr: b[3] & 0x0f,
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.freq / 100 >= 1 << 24 {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if self.freq % 100 != 0 {
            return Err(anyhow!("freq must be a multiple of 100"));
        }
        if self.dr > 15 {
            return Err(anyhow!("max dr value is 15"));
        }

        let mut b = (self.freq / 100).to_le_bytes();
        b[3] = self.dr;

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct PingSlotChannelAnsPayload {
    pub dr_ok: bool,
    pub channel_freq_ok: bool,
}

impl PingSlotChannelAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("PingSlotChannelAnsPayload expects 1 byte"));
        }

        Ok(PingSlotChannelAnsPayload {
            channel_freq_ok: b[0] & 0x01 != 0,
            dr_ok: b[0] & 0x02 != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b = 0;

        if self.channel_freq_ok {
            b = 0x01;
        }

        if self.dr_ok {
            b |= 0x02;
        }

        [b]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct BeaconFreqReqPayload {
    pub freq: u32,
}

impl BeaconFreqReqPayload {
    const SIZE: usize = 3;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("BeaconFreqReqPayload expects 3 bytes"));
        }

        Ok(BeaconFreqReqPayload {
            freq: {
                let mut freq_b: [u8; 4] = [0; 4];
                freq_b[0..3].copy_from_slice(b);
                u32::from_le_bytes(freq_b) * 100
            },
        })
    }

    pub fn to_bytes(&self) -> Result<[u8; Self::SIZE]> {
        if self.freq / 100 >= 1 << 24 {
            return Err(anyhow!("max freq value is 2^24 - 1"));
        }
        if self.freq % 100 != 0 {
            return Err(anyhow!("freq must be a multiple of 100"));
        }

        let freq_b = (self.freq / 100).to_le_bytes();
        let mut b: [u8; Self::SIZE] = [0; Self::SIZE];
        b[0..3].copy_from_slice(&freq_b[0..3]);
        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct BeaconFreqAnsPayload {
    beacon_freq_ok: bool,
}

impl BeaconFreqAnsPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("BeaconFreqAnsPayload expects 1 byte"));
        }

        Ok(BeaconFreqAnsPayload {
            beacon_freq_ok: b[0] & 0x01 != 0,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut b: u8 = 0;
        if self.beacon_freq_ok {
            b = 0x01;
        }
        [b]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DeviceModeIndPayload {
    pub class: DeviceModeClass,
}

impl DeviceModeIndPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DeviceModeIndPayload expects 1 byte"));
        }

        Ok(DeviceModeIndPayload {
            class: DeviceModeClass::from_u8(b[0])?,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.class.to_u8()]
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct DeviceModeConfPayload {
    pub class: DeviceModeClass,
}

impl DeviceModeConfPayload {
    const SIZE: usize = 1;

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() != Self::SIZE {
            return Err(anyhow!("DeviceModeConfPayload expects 1 byte"));
        }

        Ok(DeviceModeConfPayload {
            class: DeviceModeClass::from_u8(b[0])?,
        })
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        [self.class.to_u8()]
    }
}

fn try_slice(b: &[u8], start: usize, end: usize) -> Result<&[u8]> {
    if end > b.len() {
        return Err(anyhow!("not enough data"));
    }

    Ok(&b[start..end])
}

#[cfg(test)]
mod test {
    use super::*;

    struct MACTest {
        uplink: bool,
        maccommand_set: MACCommandSet,
        bytes: Vec<u8>,
    }

    #[test]
    fn test_maccommand_set() {
        let tests = vec![
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ResetInd(ResetIndPayload {
                    dev_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x01, 0x01],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ResetConf(ResetConfPayload {
                    serv_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x01, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkCheckReq]),
                bytes: vec![0x02],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkCheckAns(
                    LinkCheckAnsPayload {
                        margin: 10,
                        gw_cnt: 15,
                    },
                )]),
                bytes: vec![0x02, 0x0a, 0x0f],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRReq(
                    LinkADRReqPayload {
                        dr: 1,
                        tx_power: 2,
                        ch_mask: ChMask::new({
                            let mut mask: [bool; 16] = [false; 16];
                            mask[2] = true;
                            mask
                        }),
                        redundancy: Redundancy {
                            ch_mask_cntl: 4,
                            nb_rep: 5,
                        },
                    },
                )]),
                bytes: vec![0x03, 0x12, 0x04, 0x00, 0x45],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: true,
                        dr_ack: false,
                        tx_power_ack: false,
                    },
                )]),
                bytes: vec![0x03, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: false,
                        dr_ack: true,
                        tx_power_ack: false,
                    },
                )]),
                bytes: vec![0x03, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: false,
                        dr_ack: false,
                        tx_power_ack: true,
                    },
                )]),
                bytes: vec![0x03, 0x04],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::LinkADRAns(
                    LinkADRAnsPayload {
                        ch_mask_ack: true,
                        dr_ack: true,
                        tx_power_ack: true,
                    },
                )]),
                bytes: vec![0x03, 0x07],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DutyCycleReq(
                    DutyCycleReqPayload { max_duty_cycle: 13 },
                )]),
                bytes: vec![0x04, 0x0d],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DutyCycleAns]),
                bytes: vec![0x04],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxParamSetupReq(
                    RxParamSetupReqPayload {
                        frequency: 26265700,
                        dl_settings: DLSettings {
                            rx2_dr: 11,
                            rx1_dr_offset: 3,
                            opt_neg: false,
                        },
                    },
                )]),
                bytes: vec![0x05, 0x3b, 0x01, 0x02, 0x04],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxParamSetupAns(
                    RxParamSetupAnsPayload {
                        channel_ack: true,
                        rx2_dr_ack: false,
                        rx1_dr_offset_ack: true,
                    },
                )]),
                bytes: vec![0x05, 0x05],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusReq]),
                bytes: vec![0x06],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 0,
                        margin: -30,
                    },
                )]),
                bytes: vec![0x06, 0x00, 0x22],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 255,
                        margin: 30,
                    },
                )]),
                bytes: vec![0x06, 0xff, 0x1e],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 127,
                        margin: -1,
                    },
                )]),
                bytes: vec![0x06, 0x7f, 0x3f],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DevStatusAns(
                    DevStatusAnsPayload {
                        battery: 127,
                        margin: 0,
                    },
                )]),
                bytes: vec![0x06, 0x7f, 0x00],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelReq(
                    NewChannelReqPayload {
                        ch_index: 3,
                        freq: 26265700,
                        max_dr: 5,
                        min_dr: 10,
                    },
                )]),
                bytes: vec![0x07, 0x03, 0x01, 0x02, 0x04, 0x5a],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelReq(
                    NewChannelReqPayload {
                        ch_index: 3,
                        freq: 2410_000_000,
                        max_dr: 5,
                        min_dr: 0,
                    },
                )]),
                bytes: vec![7, 3, 80, 222, 183, 80],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: false,
                        dr_range_ok: false,
                    },
                )]),
                bytes: vec![0x07, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: true,
                        dr_range_ok: false,
                    },
                )]),
                bytes: vec![0x07, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: false,
                        dr_range_ok: true,
                    },
                )]),
                bytes: vec![0x07, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::NewChannelAns(
                    NewChannelAnsPayload {
                        channel_freq_ok: true,
                        dr_range_ok: true,
                    },
                )]),
                bytes: vec![0x07, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxTimingSetupReq(
                    RxTimingSetupReqPayload { delay: 15 },
                )]),
                bytes: vec![0x08, 0x0f],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RxTimingSetupAns]),
                bytes: vec![0x08],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupReq(
                    TxParamSetupReqPayload {
                        uplink_dwell_time: DwellTime::NoLimit,
                        downlink_dwell_time: DwellTime::NoLimit,
                        max_eirp: 15,
                    },
                )]),
                bytes: vec![0x09, 0x0f],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupReq(
                    TxParamSetupReqPayload {
                        uplink_dwell_time: DwellTime::Limit400ms,
                        downlink_dwell_time: DwellTime::NoLimit,
                        max_eirp: 15,
                    },
                )]),
                bytes: vec![0x09, 0x1f],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupReq(
                    TxParamSetupReqPayload {
                        uplink_dwell_time: DwellTime::NoLimit,
                        downlink_dwell_time: DwellTime::Limit400ms,
                        max_eirp: 15,
                    },
                )]),
                bytes: vec![0x09, 0x2f],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::TxParamSetupAns]),
                bytes: vec![0x09],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelReq(
                    DlChannelReqPayload {
                        ch_index: 0,
                        freq: 868100000,
                    },
                )]),
                bytes: vec![0x0a, 0x00, 0x28, 0x76, 0x84],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelReq(
                    DlChannelReqPayload {
                        ch_index: 1,
                        freq: 868200000,
                    },
                )]),
                bytes: vec![0x0a, 0x01, 0x10, 0x7a, 0x84],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: false,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x0a, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: false,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x0a, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: true,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x0a, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DlChannelAns(
                    DlChannelAnsPayload {
                        uplink_freq_exists: true,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x0a, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RekeyConf(RekeyConfPayload {
                    serv_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x0b, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RekeyInd(RekeyIndPayload {
                    dev_lorawan_version: Version::LoRaWAN1_1,
                })]),
                bytes: vec![0x0b, 0x01],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ADRParamSetupReq(
                    ADRParamSetupReqPayload {
                        adr_param: ADRParam {
                            limit_exp: 10,
                            delay_exp: 15,
                        },
                    },
                )]),
                bytes: vec![0x0c, 0xaf],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ADRParamSetupAns]),
                bytes: vec![0x0c],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceTimeReq]),
                bytes: vec![0x0d],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceTimeAns(
                    DeviceTimeAnsPayload {
                        time_since_gps_epoch: Duration::from_secs(1),
                    },
                )]),
                bytes: vec![0x0d, 0x01, 0x00, 0x00, 0x00, 0x00],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceTimeAns(
                    DeviceTimeAnsPayload {
                        time_since_gps_epoch: Duration::new(1, 2 * 3906250),
                    },
                )]),
                bytes: vec![0x0d, 0x01, 0x00, 0x00, 0x00, 0x02],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::ForceRejoinReq(
                    ForceRejoinReqPayload {
                        period: 3,
                        max_retries: 4,
                        rejoin_type: 2,
                        dr: 5,
                    },
                )]),
                bytes: vec![0x0e, 0x25, 0x1c],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RejoinParamSetupReq(
                    RejoinParamSetupReqPayload {
                        max_time_n: 14,
                        max_count_n: 15,
                    },
                )]),
                bytes: vec![0x0f, 0xef],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::RejoinParamSetupAns(
                    RejoinParamSetupAnsPayload { time_ok: true },
                )]),
                bytes: vec![0x0f, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotInfoReq(
                    PingSlotInfoReqPayload { periodicity: 3 },
                )]),
                bytes: vec![0x10, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotInfoAns]),
                bytes: vec![0x10],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelReq(
                    PingSlotChannelReqPayload {
                        freq: 868100000,
                        dr: 5,
                    },
                )]),
                bytes: vec![0x11, 0x28, 0x76, 0x84, 0x05],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: false,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x011, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: false,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x011, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: true,
                        channel_freq_ok: false,
                    },
                )]),
                bytes: vec![0x011, 0x02],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::PingSlotChannelAns(
                    PingSlotChannelAnsPayload {
                        dr_ok: true,
                        channel_freq_ok: true,
                    },
                )]),
                bytes: vec![0x11, 0x03],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::BeaconFreqReq(
                    BeaconFreqReqPayload { freq: 868100000 },
                )]),
                bytes: vec![0x13, 0x28, 0x76, 0x84],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::BeaconFreqAns(
                    BeaconFreqAnsPayload {
                        beacon_freq_ok: false,
                    },
                )]),
                bytes: vec![0x13, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::BeaconFreqAns(
                    BeaconFreqAnsPayload {
                        beacon_freq_ok: true,
                    },
                )]),
                bytes: vec![0x13, 0x01],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeInd(
                    DeviceModeIndPayload {
                        class: DeviceModeClass::ClassA,
                    },
                )]),
                bytes: vec![0x20, 0x00],
            },
            MACTest {
                uplink: true,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeInd(
                    DeviceModeIndPayload {
                        class: DeviceModeClass::ClassC,
                    },
                )]),
                bytes: vec![0x20, 0x02],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeConf(
                    DeviceModeConfPayload {
                        class: DeviceModeClass::ClassA,
                    },
                )]),
                bytes: vec![0x20, 0x00],
            },
            MACTest {
                uplink: false,
                maccommand_set: MACCommandSet::new(vec![MACCommand::DeviceModeConf(
                    DeviceModeConfPayload {
                        class: DeviceModeClass::ClassC,
                    },
                )]),
                bytes: vec![0x20, 0x02],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.maccommand_set.to_vec().unwrap());

            let mut maccommand_set = MACCommandSet::from_slice(&tst.bytes);
            maccommand_set.decode_from_raw(tst.uplink).unwrap();

            assert_eq!(tst.maccommand_set, maccommand_set);
        }
    }
}
