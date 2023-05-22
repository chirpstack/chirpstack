use anyhow::Result;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::helpers::{decode_freq, encode_freq};
use crate::phy_payload::PhyPayload;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UplinkMetadata {
    pub dr: u8,
    pub snr: isize,
    pub rssi: isize,
    pub wor_channel: u8,
}

impl UplinkMetadata {
    pub fn from_bytes(b: [u8; 3]) -> Self {
        UplinkMetadata {
            dr: b[0] & 0x0f,
            snr: ((b[0] >> 4) | ((b[1] & 0x01) << 4)) as isize - 20,
            rssi: -1 * (b[1] >> 1) as isize - 15,
            wor_channel: b[2] & 0x03,
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; 3]> {
        if self.dr > 15 {
            return Err(anyhow!("max dr value is 15"));
        }

        if self.wor_channel > 1 {
            return Err(anyhow!("max wor_channel value is 1"));
        }

        let mut snr = self.snr;
        let mut rssi = self.rssi;

        // Set to closest possible value.
        if snr < -20 {
            snr = -20;
        }
        if snr > 11 {
            snr = 11;
        }
        if rssi > -15 {
            rssi = -15;
        }
        if rssi < -142 {
            rssi = -142;
        }

        // Encode values
        let snr = (snr + 20) as u8;
        let rssi = ((rssi as isize + 15) * -1) as u8;

        Ok([self.dr | snr << 4, snr >> 4 | rssi << 1, self.wor_channel])
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ForwardUplinkReq {
    pub metadata: UplinkMetadata,
    pub frequency: u32,
    pub payload: Box<PhyPayload>,
}

impl ForwardUplinkReq {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() < 6 {
            return Err(anyhow!("at least 6 bytes are expected"));
        }

        Ok(ForwardUplinkReq {
            metadata: UplinkMetadata::from_bytes([b[0], b[1], b[2]]),
            frequency: decode_freq(&b[3..6])?,
            payload: Box::new(PhyPayload::from_slice(&b[6..])?),
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut b = Vec::new();
        b.extend_from_slice(&self.metadata.to_bytes()?);
        b.extend_from_slice(&encode_freq(self.frequency)?);
        b.extend_from_slice(&self.payload.to_vec()?);
        Ok(b)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ForwardDownlinkReq {
    pub payload: Box<PhyPayload>,
}

impl ForwardDownlinkReq {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        Ok(ForwardDownlinkReq {
            payload: Box::new(PhyPayload::from_slice(&b)?),
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.payload.to_vec()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_forward_uplink_req() {
        let req = ForwardUplinkReq {
            metadata: UplinkMetadata {
                dr: 5,
                snr: 9,
                rssi: -110,
                wor_channel: 1,
            },
            frequency: 868100000,
            payload: Box::new(PhyPayload {
                mhdr: MHDR {
                    m_type: MType::Proprietary,
                    major: Major::LoRaWANR1,
                },
                payload: Payload::Raw(vec![0x01, 0x02, 0x03]),
                mic: None,
            }),
        };

        let b = req.to_vec().unwrap();
        assert_eq!(vec![213, 191, 1, 40, 118, 132, 224, 1, 2, 3], b);

        let req_decoded = ForwardUplinkReq::from_slice(&b).unwrap();
        assert_eq!(req, req_decoded);
    }

    #[test]
    fn test_forward_downlink_req() {
        let req = ForwardDownlinkReq {
            payload: Box::new(PhyPayload {
                mhdr: MHDR {
                    m_type: MType::Proprietary,
                    major: Major::LoRaWANR1,
                },
                payload: Payload::Raw(vec![0x01, 0x02, 0x03]),
                mic: None,
            }),
        };

        let b = req.to_vec().unwrap();
        assert_eq!(vec![224, 1, 2, 3], b);

        let req_decoded = ForwardDownlinkReq::from_slice(&b).unwrap();
        assert_eq!(req, req_decoded);
    }
}
