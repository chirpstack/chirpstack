use std::fmt;
use std::str::FromStr;

use anyhow::Result;
#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    {Deserialize, Deserializer, Serialize, Serializer},
};

use crate::devaddr::DevAddrPrefix;
use crate::Error;

#[derive(Default, PartialEq, Clone, Copy, Hash, Eq)]
pub struct NetID([u8; 3]);

impl NetID {
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 3 {
            return Err(Error::NetIdLength);
        }

        let mut bb: [u8; 3] = [0; 3];
        bb.copy_from_slice(b);

        Ok(NetID(bb))
    }

    pub fn from_be_bytes(b: [u8; 3]) -> Self {
        NetID(b)
    }

    pub fn from_le_bytes(b: [u8; 3]) -> Self {
        let mut b = b;
        b.reverse();
        NetID(b)
    }

    pub fn to_le_bytes(&self) -> [u8; 3] {
        let mut b = self.0;
        b.reverse();
        b
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn netid_type(&self) -> u8 {
        self.0[0] >> 5
    }

    pub fn id(&self) -> Vec<u8> {
        match self.netid_type() {
            0 | 1 => self.get_id(6),
            2 => self.get_id(9),
            3 | 4 | 5 | 6 | 7 => self.get_id(21),
            _ => vec![],
        }
    }

    fn get_id(&self, bits: u8) -> Vec<u8> {
        // convert netid to u32
        let mut b: [u8; 4] = [0; 4];
        b[1..].clone_from_slice(self.0.as_ref());
        let mut temp = u32::from_be_bytes(b);

        // clear prefix
        temp <<= 32 - bits;
        temp >>= 32 - bits;

        // back to bytes
        let out = temp.to_be_bytes();
        let mut blen: usize = (bits / 8) as usize;
        if bits % 8 != 0 {
            blen += 1;
        }

        out[4 - blen..].to_vec()
    }

    pub fn dev_addr_prefix(&self) -> DevAddrPrefix {
        match self.netid_type() {
            0 => self.get_dev_addr_prefix(1, 6),
            1 => self.get_dev_addr_prefix(2, 6),
            2 => self.get_dev_addr_prefix(3, 9),
            3 => self.get_dev_addr_prefix(4, 11),
            4 => self.get_dev_addr_prefix(5, 12),
            5 => self.get_dev_addr_prefix(6, 13),
            6 => self.get_dev_addr_prefix(7, 15),
            7 => self.get_dev_addr_prefix(8, 17),
            _ => panic!("netid_type bug"),
        }
    }

    fn get_dev_addr_prefix(&self, prefix_length: u32, nwkid_bits: u32) -> DevAddrPrefix {
        // type prefix
        let mut prefix: u32 = 254 << (32 - prefix_length);

        // NwkID prefix
        let mut netid_bytes: [u8; 4] = [0; 4];
        let netid_id = self.id();
        netid_bytes[4 - netid_id.len()..].clone_from_slice(&netid_id);
        let mut nwkid = u32::from_be_bytes(netid_bytes);
        nwkid <<= 32 - nwkid_bits; // truncate the MSB of the NetID ID field
        nwkid >>= prefix_length; // shift base for the prefix MSB

        // merge type prefix with nwkid.
        prefix |= nwkid;

        DevAddrPrefix::new(prefix.to_be_bytes(), prefix_length + nwkid_bits)
    }
}

impl fmt::Display for NetID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Debug for NetID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for NetID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes: [u8; 3] = [0; 3];
        hex::decode_to_slice(s, &mut bytes)?;

        Ok(NetID(bytes))
    }
}

#[cfg(feature = "serde")]
impl Serialize for NetID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NetID {
    fn deserialize<D>(deserialize: D) -> Result<NetID, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(NetIdVisitor)
    }
}

#[cfg(feature = "serde")]
struct NetIdVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for NetIdVisitor {
    type Value = NetID;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A NetID in the format 010203 is expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NetID::from_str(value).map_err(|e| E::custom(format!("{}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        netid: NetID,
        netid_type: u8,
        id: Vec<u8>,
        bytes: [u8; 3],
        string: String,
        dev_addr_prefix: DevAddrPrefix,
    }

    fn tests() -> Vec<Test> {
        vec![
            Test {
                netid: NetID::from_be_bytes([0x00, 0x00, 0x6d]),
                netid_type: 0,
                id: vec![0x2d],
                bytes: [0x6d, 0x00, 0x00],
                string: "00006d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0x5a, 0x00, 0x00, 0x00], 7),
            },
            Test {
                netid: NetID::from_be_bytes([0x20, 0x00, 0x6d]),
                netid_type: 1,
                id: vec![0x2d],
                bytes: [0x6d, 0x00, 0x20],
                string: "20006d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xad, 0x00, 0x00, 0x00], 8),
            },
            Test {
                netid: NetID::from_be_bytes([0x40, 0x03, 0x6d]),
                netid_type: 2,
                id: vec![0x01, 0x6d],
                bytes: [0x6d, 0x03, 0x40],
                string: "40036d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xd6, 0xd0, 0x00, 0x00], 12),
            },
            Test {
                netid: NetID::from_be_bytes([0x76, 0xdb, 0x6d]),
                netid_type: 3,
                id: vec![0x16, 0xdb, 0x6d],
                bytes: [0x6d, 0xdb, 0x76],
                string: "76db6d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xe6, 0xda, 0x00, 0x00], 15),
            },
            Test {
                netid: NetID::from_be_bytes([0x96, 0xdb, 0x6d]),
                netid_type: 4,
                id: vec![0x16, 0xdb, 0x6d],
                bytes: [0x6d, 0xdb, 0x96],
                string: "96db6d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xf5, 0xb6, 0x80, 0x00], 17),
            },
            Test {
                netid: NetID::from_be_bytes([0xb6, 0xdb, 0x6d]),
                netid_type: 5,
                id: vec![0x16, 0xdb, 0x6d],
                bytes: [0x6d, 0xdb, 0xb6],
                string: "b6db6d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xfb, 0x6d, 0xa0, 0x00], 19),
            },
            Test {
                netid: NetID::from_be_bytes([0xd6, 0xdb, 0x6d]),
                netid_type: 6,
                id: vec![0x16, 0xdb, 0x6d],
                bytes: [0x6d, 0xdb, 0xd6],
                string: "d6db6d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xfd, 0x6d, 0xb4, 00], 22),
            },
            Test {
                netid: NetID::from_be_bytes([0xf6, 0xdb, 0x6d]),
                netid_type: 7,
                id: vec![0x16, 0xdb, 0x6d],
                bytes: [0x6d, 0xdb, 0xf6],
                string: "f6db6d".into(),
                dev_addr_prefix: DevAddrPrefix::new([0xfe, 0x6d, 0xb6, 0x80], 25),
            },
        ]
    }

    #[test]
    fn test_to_le_bytes() {
        for tst in tests() {
            assert_eq!(tst.bytes, tst.netid.to_le_bytes());
        }
    }

    #[test]
    fn test_from_le_bytes() {
        for tst in tests() {
            assert_eq!(tst.netid, NetID::from_le_bytes(tst.bytes));
        }
    }

    #[test]
    fn test_netid_type() {
        for tst in tests() {
            assert_eq!(tst.netid_type, tst.netid.netid_type());
        }
    }

    #[test]
    fn test_id() {
        for tst in tests() {
            assert_eq!(tst.id, tst.netid.id());
        }
    }

    #[test]
    fn test_display() {
        for tst in tests() {
            assert_eq!(tst.string, tst.netid.to_string());
        }
    }

    #[test]
    fn test_from_str() {
        for tst in tests() {
            assert_eq!(tst.netid, NetID::from_str(&tst.string).unwrap());
        }
    }

    #[test]
    fn test_dev_addr_prefix() {
        for tst in tests() {
            assert_eq!(tst.dev_addr_prefix, tst.netid.dev_addr_prefix());
        }
    }
}
