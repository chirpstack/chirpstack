use std::fmt;

use anyhow::Result;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum FType {
    JoinRequest,
    JoinAccept,
    UnconfirmedDataUp,
    UnconfirmedDataDown,
    ConfirmedDataUp,
    ConfirmedDataDown,
    RejoinRequest,
    Proprietary,
}

impl fmt::Display for FType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Major {
    LoRaWANR1,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MHDR {
    pub f_type: FType,
    pub major: Major,
}

impl MHDR {
    pub fn from_le_bytes(b: [u8; 1]) -> Result<Self> {
        Ok(MHDR {
            f_type: match b[0] >> 5 {
                0x00 => FType::JoinRequest,
                0x01 => FType::JoinAccept,
                0x02 => FType::UnconfirmedDataUp,
                0x03 => FType::UnconfirmedDataDown,
                0x04 => FType::ConfirmedDataUp,
                0x05 => FType::ConfirmedDataDown,
                0x06 => FType::RejoinRequest,
                0x07 => FType::Proprietary,
                _ => return Err(anyhow!("unexpected mtype")),
            },
            major: match b[0] & 0x03 {
                0x00 => Major::LoRaWANR1,
                _ => return Err(anyhow!("unexpected major")),
            },
        })
    }

    pub fn to_le_bytes(&self) -> [u8; 1] {
        let mut mhdr = match self.f_type {
            FType::JoinRequest => 0x00,
            FType::JoinAccept => 0x01,
            FType::UnconfirmedDataUp => 0x02,
            FType::UnconfirmedDataDown => 0x03,
            FType::ConfirmedDataUp => 0x04,
            FType::ConfirmedDataDown => 0x05,
            FType::RejoinRequest => 0x06,
            FType::Proprietary => 0x07,
        };

        mhdr = (mhdr << 5)
            | match self.major {
                Major::LoRaWANR1 => 0x00,
            };

        [mhdr]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_le_bytes() {
        let mhdr = MHDR {
            f_type: FType::Proprietary,
            major: Major::LoRaWANR1,
        };

        assert_eq!(mhdr.to_le_bytes(), [0xe0]);
    }

    #[test]
    fn test_from_le_bytes() {
        let mhdr = MHDR::from_le_bytes([0xe0]).unwrap();
        assert_eq!(
            mhdr,
            MHDR {
                f_type: FType::Proprietary,
                major: Major::LoRaWANR1,
            }
        );
    }
}
