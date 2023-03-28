use std::error::Error;
use std::fmt;
use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/common/common.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/common/common.serde.rs"));

#[allow(clippy::from_over_into)]
impl Into<String> for MType {
    fn into(self) -> String {
        match self {
            MType::JoinRequest => "JoinRequest",
            MType::JoinAccept => "JoinAccept",
            MType::UnconfirmedDataUp => "UnconfirmedDataUp",
            MType::UnconfirmedDataDown => "UnconfirmedDataDown",
            MType::ConfirmedDataUp => "ConfirmedDataUp",
            MType::ConfirmedDataDown => "ConfirmedDataDown",
            MType::RejoinRequest => "RejoinRequest",
            MType::Proprietary => "Proprietary",
        }
        .to_string()
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for Region {
    fn into(self) -> String {
        match self {
            Region::Eu868 => "EU868",
            Region::Us915 => "US915",
            Region::Cn779 => "CN779",
            Region::Eu433 => "EU433",
            Region::Au915 => "AU915",
            Region::Cn470 => "CN470",
            Region::As923 => "AS923",
            Region::As9232 => "AS923_2",
            Region::As9233 => "AS923_3",
            Region::As9234 => "AS923_4",
            Region::Kr920 => "KR920",
            Region::In865 => "IN865",
            Region::Ru864 => "RU864",
            Region::Ism2400 => "ISM2400",
        }
        .to_string()
    }
}

impl FromStr for Region {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match s {
            "EU868" => Region::Eu868,
            "US915" => Region::Us915,
            "CN779" => Region::Cn779,
            "EU433" => Region::Eu433,
            "AU915" => Region::Au915,
            "CN470" => Region::Cn470,
            "AS923" => Region::As923,
            "AS923_2" => Region::As9232,
            "AS923_3" => Region::As9233,
            "AS923_4" => Region::As9234,
            "KR920" => Region::Kr920,
            "IN865" => Region::In865,
            "RU864" => Region::Ru864,
            "ISM2400" => Region::Ism2400,
            _ => {
                return Err("invalid region".into());
            }
        })
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for MacVersion {
    fn into(self) -> String {
        match self {
            MacVersion::Lorawan100 => "1.0.0",
            MacVersion::Lorawan101 => "1.0.1",
            MacVersion::Lorawan102 => "1.0.2",
            MacVersion::Lorawan103 => "1.0.3",
            MacVersion::Lorawan104 => "1.0.4",
            MacVersion::Lorawan110 => "1.1.0",
        }
        .to_string()
    }
}

impl fmt::Display for MacVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = (*self).into();
        write!(f, "{}", s)
    }
}

impl FromStr for MacVersion {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match s {
            "1.0.0" => MacVersion::Lorawan100,
            "1.0.1" => MacVersion::Lorawan101,
            "1.0.2" => MacVersion::Lorawan102,
            "1.0.3" => MacVersion::Lorawan103,
            "1.0.4" => MacVersion::Lorawan104,
            "1.1.0" => MacVersion::Lorawan110,
            _ => {
                return Err("invalid mac-version".into());
            }
        })
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for RegParamsRevision {
    fn into(self) -> String {
        match self {
            RegParamsRevision::A => "A",
            RegParamsRevision::B => "B",
            RegParamsRevision::Rp002100 => "RP002_1.0.0",
            RegParamsRevision::Rp002101 => "RP002_1.0.1",
            RegParamsRevision::Rp002102 => "RP002_1.0.2",
            RegParamsRevision::Rp002103 => "RP002_1.0.3",
        }
        .to_string()
    }
}

impl FromStr for RegParamsRevision {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match s {
            "A" => RegParamsRevision::A,
            "B" => RegParamsRevision::B,
            "RP002_1.0.0" => RegParamsRevision::Rp002100,
            "RP002_1.0.1" => RegParamsRevision::Rp002101,
            "RP002_1.0.2" => RegParamsRevision::Rp002102,
            "RP002_1.0.3" => RegParamsRevision::Rp002103,
            _ => {
                return Err("invalid reg param revision".into());
            }
        })
    }
}
