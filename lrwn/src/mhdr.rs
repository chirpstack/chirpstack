use std::fmt;

use anyhow::Result;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum MType {
    JoinRequest,
    JoinAccept,
    UnconfirmedDataUp,
    UnconfirmedDataDown,
    ConfirmedDataUp,
    ConfirmedDataDown,
    RejoinRequest,
    Proprietary,
}

impl fmt::Display for MType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum Major {
    LoRaWANR1,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct MHDR {
    pub m_type: MType,
    pub major: Major,
}

impl MHDR {
    pub fn from_le_bytes(b: [u8; 1]) -> Result<Self> {
        Ok(MHDR {
            m_type: match b[0] >> 5 {
                0x00 => MType::JoinRequest,
                0x01 => MType::JoinAccept,
                0x02 => MType::UnconfirmedDataUp,
                0x03 => MType::UnconfirmedDataDown,
                0x04 => MType::ConfirmedDataUp,
                0x05 => MType::ConfirmedDataDown,
                0x06 => MType::RejoinRequest,
                0x07 => MType::Proprietary,
                _ => return Err(anyhow!("unexpected mtype")),
            },
            major: match b[0] & 0x03 {
                0x00 => Major::LoRaWANR1,
                _ => return Err(anyhow!("unexpected major")),
            },
        })
    }

    pub fn to_le_bytes(&self) -> [u8; 1] {
        let mut mhdr = match self.m_type {
            MType::JoinRequest => 0x00,
            MType::JoinAccept => 0x01,
            MType::UnconfirmedDataUp => 0x02,
            MType::UnconfirmedDataDown => 0x03,
            MType::ConfirmedDataUp => 0x04,
            MType::ConfirmedDataDown => 0x05,
            MType::RejoinRequest => 0x06,
            MType::Proprietary => 0x07,
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
            m_type: MType::Proprietary,
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
                m_type: MType::Proprietary,
                major: Major::LoRaWANR1,
            }
        );
    }
}
