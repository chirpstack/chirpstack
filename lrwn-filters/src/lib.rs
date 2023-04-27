use std::fmt;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    {Deserialize, Deserializer, Serialize, Serializer},
};

pub mod error;

/// Configuration.
#[derive(Default, Clone)]
pub struct Filters {
    /// DevAddr prefixes.
    pub dev_addr_prefixes: Vec<DevAddrPrefix>,

    /// JoinEUIs.
    pub join_eui_prefixes: Vec<EuiPrefix>,
}

/// Returns true if the given PhyPayload matches the given filters
///
/// If no DevAddr prefixes are given, then all data uplink frames will pass the filter.
/// If no JoinEui prefixes are given, then all join requests will pass the filter.
/// PhyPayloads that can't be filtered will pass the filter.
pub fn matches(phy_payload: &[u8], config: &Filters) -> bool {
    if phy_payload.is_empty() {
        return true;
    }

    let mhdr = phy_payload[0];
    let m_type = mhdr >> 5;

    let dev_addr: Option<u32> = match m_type {
        // DataUp
        0x02 | 0x04 => {
            // MHDR + DevAddr
            // [1]    [4]
            if phy_payload.len() >= 5 {
                let mut dev_addr: [u8; 4] = [0; 4];
                dev_addr.clone_from_slice(&phy_payload[1..5]);
                Some(u32::from_le_bytes(dev_addr))
            } else {
                None
            }
        }
        _ => None,
    };

    let join_eui: Option<u64> = match m_type {
        // JoinRequest
        0x00 => {
            // MHDR + JoinEUI + DevEUI
            // [1]    [8]       [8]
            if phy_payload.len() >= 17 {
                let mut join_eui: [u8; 8] = [0; 8];
                join_eui.clone_from_slice(&phy_payload[1..9]);
                Some(u64::from_le_bytes(join_eui))
            } else {
                None
            }
        }
        _ => None,
    };

    // We could not extract the DevAddr or JoinEUI from the PhyPayload. In this case we let the
    // message pass.
    if dev_addr.is_none() && join_eui.is_none() {
        return true;
    }

    if let Some(dev_addr) = dev_addr {
        if config.dev_addr_prefixes.is_empty() {
            return true;
        }

        for p in &config.dev_addr_prefixes {
            let prefix = u32::from_be_bytes(p.prefix());
            if dev_addr >> (32 - p.size()) == prefix >> (32 - p.size()) {
                return true;
            }
        }
    }

    if let Some(join_eui) = join_eui {
        if config.join_eui_prefixes.is_empty() {
            return true;
        }

        for p in &config.join_eui_prefixes {
            let prefix = u64::from_be_bytes(p.prefix());
            if join_eui >> (64 - p.size()) == prefix >> (64 - p.size()) {
                return true;
            }
        }
    }

    false
}

/// DevAddr prefix.
#[derive(Clone, Copy)]
pub struct DevAddrPrefix([u8; 4], u32);

impl DevAddrPrefix {
    pub fn new(prefix: [u8; 4], size: u32) -> Self {
        DevAddrPrefix(prefix, size)
    }

    fn prefix(&self) -> [u8; 4] {
        self.0
    }

    fn size(&self) -> u32 {
        self.1
    }
}

impl fmt::Display for DevAddrPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.0), self.1)
    }
}

impl fmt::Debug for DevAddrPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.0), self.1)
    }
}

impl FromStr for DevAddrPrefix {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(error::Error::DevAddrPrefixFormat);
        }
        if parts[0].len() != 8 {
            return Err(error::Error::DevAddrPrefixFormat);
        }

        let mut mask: [u8; 4] = [0; 4];
        hex::decode_to_slice(parts[0], &mut mask)?;
        let size: u32 = parts[1]
            .parse()
            .map_err(|_| error::Error::DevAddrPrefixFormat)?;

        if size > 32 {
            return Err(error::Error::DevAddrPrefixSize);
        }

        Ok(DevAddrPrefix(mask, size))
    }
}

#[cfg(feature = "serde")]
impl Serialize for DevAddrPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for DevAddrPrefix {
    fn deserialize<D>(deserialize: D) -> Result<DevAddrPrefix, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(DevAddrPrefixVisitor)
    }
}

#[cfg(feature = "serde")]
struct DevAddrPrefixVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for DevAddrPrefixVisitor {
    type Value = DevAddrPrefix;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A DevAddrPrefix in the format 00000000/0 is expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        DevAddrPrefix::from_str(value).map_err(|e| E::custom(format!("{}", e)))
    }
}

/// JoinEUI prefix.
#[derive(Clone, Copy)]
pub struct EuiPrefix([u8; 8], u32);

impl EuiPrefix {
    pub fn new(prefix: [u8; 8], size: u32) -> Self {
        EuiPrefix(prefix, size)
    }

    fn prefix(&self) -> [u8; 8] {
        self.0
    }

    fn size(&self) -> u32 {
        self.1
    }
}

impl fmt::Display for EuiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.0), self.1)
    }
}

impl fmt::Debug for EuiPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.0), self.1)
    }
}

impl FromStr for EuiPrefix {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(error::Error::EuiPrefixFormat);
        }
        if parts[0].len() != 16 {
            return Err(error::Error::EuiPrefixFormat);
        }

        let mut mask: [u8; 8] = [0; 8];
        hex::decode_to_slice(parts[0], &mut mask)?;
        let size: u32 = parts[1]
            .parse()
            .map_err(|_| error::Error::EuiPrefixFormat)?;

        if size > 64 {
            return Err(error::Error::EuiPrefixSize);
        }

        Ok(EuiPrefix(mask, size))
    }
}

#[cfg(feature = "serde")]
impl Serialize for EuiPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EuiPrefix {
    fn deserialize<D>(deserialize: D) -> Result<EuiPrefix, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(EuiPrefixVisitor)
    }
}

#[cfg(feature = "serde")]
struct EuiPrefixVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for EuiPrefixVisitor {
    type Value = EuiPrefix;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An EuiPrefix in the format 0000000000000000/0 is expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EuiPrefix::from_str(value).map_err(|e| E::custom(format!("{}", e)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dev_addr() {
        struct Test {
            name: String,
            filters: Filters,
            dev_addr: lrwn::DevAddr,
            passes: bool,
        }

        let tests = vec![
            Test {
                name: "empty filters".to_string(),
                filters: Filters {
                    dev_addr_prefixes: vec![],
                    join_eui_prefixes: vec![],
                },
                dev_addr: lrwn::DevAddr::from_str("01020304").unwrap(),
                passes: true,
            },
            Test {
                name: "dev_addr passes filter".to_string(),
                filters: Filters {
                    dev_addr_prefixes: vec![DevAddrPrefix::from_str("01000000/8").unwrap()],
                    join_eui_prefixes: vec![],
                },
                dev_addr: lrwn::DevAddr::from_str("01020304").unwrap(),
                passes: true,
            },
            Test {
                name: "dev_addr does not pass filter".to_string(),
                filters: Filters {
                    dev_addr_prefixes: vec![DevAddrPrefix::from_str("01000000/16").unwrap()],
                    join_eui_prefixes: vec![],
                },
                dev_addr: lrwn::DevAddr::from_str("01020304").unwrap(),
                passes: false,
            },
            Test {
                name: "dev_addr passes one of the two filters".to_string(),
                filters: Filters {
                    dev_addr_prefixes: vec![
                        DevAddrPrefix::from_str("01000000/16").unwrap(),
                        DevAddrPrefix::from_str("01000000/8").unwrap(),
                    ],
                    join_eui_prefixes: vec![],
                },
                dev_addr: lrwn::DevAddr::from_str("01020304").unwrap(),
                passes: true,
            },
        ];

        for test in &tests {
            println!("> {}", test.name);
            let phy = lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: test.dev_addr,
                        f_ctrl: Default::default(),
                        f_cnt: 0,
                        f_opts: lrwn::MACCommandSet::new(vec![]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: None,
            };
            let phy_b = phy.to_vec().unwrap();
            assert_eq!(test.passes, matches(&phy_b, &test.filters));

            let phy = lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::ConfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: test.dev_addr,
                        f_ctrl: Default::default(),
                        f_cnt: 0,
                        f_opts: lrwn::MACCommandSet::new(vec![]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: None,
            };
            let phy_b = phy.to_vec().unwrap();
            assert_eq!(test.passes, matches(&phy_b, &test.filters));
        }
    }

    #[test]
    fn test_join_eui() {
        struct Test {
            name: String,
            filters: Filters,
            join_eui: lrwn::EUI64,
            passes: bool,
        }

        let tests = vec![
            Test {
                name: "no filters".into(),
                filters: Filters {
                    dev_addr_prefixes: vec![],
                    join_eui_prefixes: vec![],
                },
                join_eui: lrwn::EUI64::from_str("0102030405060708").unwrap(),
                passes: true,
            },
            Test {
                name: "passes filter".into(),
                filters: Filters {
                    dev_addr_prefixes: vec![],
                    join_eui_prefixes: vec![EuiPrefix::from_str("0100000000000000/8").unwrap()],
                },
                join_eui: lrwn::EUI64::from_str("0102030405060708").unwrap(),
                passes: true,
            },
            Test {
                name: "does not pass filter".into(),
                filters: Filters {
                    dev_addr_prefixes: vec![],
                    join_eui_prefixes: vec![EuiPrefix::from_str("0100000000000000/16").unwrap()],
                },
                join_eui: lrwn::EUI64::from_str("0102030405060708").unwrap(),
                passes: false,
            },
            Test {
                name: "passes one of two filters".into(),
                filters: Filters {
                    dev_addr_prefixes: vec![],
                    join_eui_prefixes: vec![
                        EuiPrefix::from_str("0100000000000000/16").unwrap(),
                        EuiPrefix::from_str("0100000000000000/8").unwrap(),
                    ],
                },
                join_eui: lrwn::EUI64::from_str("0102030405060708").unwrap(),
                passes: true,
            },
        ];

        for test in &tests {
            println!("> {}", test.name);
            let phy = lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::JoinRequest,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
                    join_eui: test.join_eui,
                    dev_eui: Default::default(),
                    dev_nonce: 0,
                }),
                mic: None,
            };
            let phy_b = phy.to_vec().unwrap();
            assert_eq!(test.passes, matches(&phy_b, &test.filters));
        }
    }
}
