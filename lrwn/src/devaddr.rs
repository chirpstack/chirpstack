use std::fmt;
use std::str::FromStr;

use anyhow::Result;
#[cfg(feature = "diesel")]
use diesel::{backend::Backend, deserialize, serialize, sql_types::Binary};
#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::netid::NetID;
use crate::Error;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(Error::DevAddrPrefixFormat);
        }

        if parts[0].len() != 8 {
            return Err(Error::DevAddrPrefixFormat);
        }

        let mut mask: [u8; 4] = [0; 4];
        hex::decode_to_slice(parts[0], &mut mask)?;
        let size: u32 = parts[1].parse().map_err(|_| Error::DevAddrPrefixFormat)?;

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

#[derive(PartialEq, Eq, Copy, Clone, Default)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::Binary))]
pub struct DevAddr([u8; 4]);

impl DevAddr {
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 4 {
            return Err(Error::DevAddrLength);
        }

        let mut bb: [u8; 4] = [0; 4];
        bb.copy_from_slice(b);

        Ok(DevAddr(bb))
    }

    pub fn from_be_bytes(b: [u8; 4]) -> Self {
        DevAddr(b)
    }

    pub fn from_le_bytes(b: [u8; 4]) -> Self {
        let mut devaddr: [u8; 4] = [0; 4];

        // little endian
        for (i, v) in b.iter().enumerate() {
            devaddr[4 - i - 1] = *v;
        }

        DevAddr(devaddr)
    }

    pub fn to_be_bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn to_le_bytes(&self) -> [u8; 4] {
        let mut b = self.0;
        b.reverse();
        b
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn is_net_id(&self, net_id: NetID) -> bool {
        let mut dev_addr = *self;
        dev_addr.set_dev_addr_prefix(net_id.dev_addr_prefix());

        *self == dev_addr
    }

    pub fn netid_type(&self) -> Result<u8> {
        for i in (0..=7).rev() {
            if self.0[0] & (1 << i) == 0 {
                return Ok(7 - i);
            }
        }

        Err(anyhow!("Invalid type prefix value"))
    }

    pub fn nwkid(&self) -> Result<Vec<u8>> {
        Ok(match self.netid_type()? {
            0 => self.get_nwkid(1, 6),
            1 => self.get_nwkid(2, 6),
            2 => self.get_nwkid(3, 9),
            3 => self.get_nwkid(4, 11),
            4 => self.get_nwkid(5, 12),
            5 => self.get_nwkid(6, 13),
            6 => self.get_nwkid(7, 15),
            7 => self.get_nwkid(8, 17),
            _ => vec![],
        })
    }

    pub fn set_dev_addr_prefix(&mut self, prefix: DevAddrPrefix) {
        // convert devaddr to u32
        let mut devaddr = u32::from_be_bytes(self.0);

        // clean the prefix bits
        let mask = u32::MAX; // all u32 bits to 1
        devaddr &= !(mask << (32 - prefix.size()));

        // set the prefix
        devaddr |= u32::from_be_bytes(prefix.prefix());

        self.0 = devaddr.to_be_bytes();
    }

    fn get_nwkid(&self, prefix_length: u32, nwkid_bits: u32) -> Vec<u8> {
        // convert devaddr to u32
        let mut temp = u32::from_be_bytes(self.0);

        // clear prefix
        temp <<= prefix_length;

        // shift to starting of nwkid
        temp >>= 32 - nwkid_bits;

        // back to bytes
        let out = temp.to_be_bytes();

        let mut blen: usize = (nwkid_bits / 8) as usize;
        if nwkid_bits % 8 != 0 {
            blen += 1;
        }

        out[4 - blen..].to_vec()
    }
}

impl fmt::Display for DevAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Debug for DevAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for DevAddr {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes: [u8; 4] = [0; 4];
        hex::decode_to_slice(s, &mut bytes)?;
        Ok(DevAddr(bytes))
    }
}

#[cfg(feature = "serde")]
impl Serialize for DevAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "diesel")]
impl<ST, DB> deserialize::FromSql<ST, DB> for DevAddr
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<ST, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes = <Vec<u8> as deserialize::FromSql<ST, DB>>::from_sql(value)?;
        if bytes.len() != 4 {
            return Err("DevAddr type expects exactly 4 bytes".into());
        }

        let mut b: [u8; 4] = [0; 4];
        b.copy_from_slice(&bytes);

        Ok(DevAddr(b))
    }
}

#[cfg(feature = "diesel")]
impl serialize::ToSql<Binary, diesel::pg::Pg> for DevAddr
where
    [u8]: serialize::ToSql<Binary, diesel::pg::Pg>,
{
    fn to_sql<'b>(&self, out: &mut serialize::Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        <[u8] as serialize::ToSql<Binary, diesel::pg::Pg>>::to_sql(
            &self.to_be_bytes(),
            &mut out.reborrow(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        devaddr: DevAddr,
        netid_type: u8,
        nwkid: Vec<u8>,
        bytes: [u8; 4],
        string: String,
    }

    struct SetAddrPrefixTest {
        devaddr: DevAddr,
        netid: NetID,
        expected_devaddr: DevAddr,
    }

    fn tests() -> Vec<Test> {
        vec![Test {
            devaddr: DevAddr::from_be_bytes([0x5b, 0xff, 0xff, 0xff]),
            netid_type: 0,
            nwkid: vec![0x2d],
            bytes: [0xff, 0xff, 0xff, 0x5b],
            string: "5bffffff".into(),
        }]
    }

    #[test]
    fn test_dev_addr_prefix() {
        let p = DevAddrPrefix::from_str("01000000/8").unwrap();
        assert_eq!(DevAddrPrefix::new([1, 0, 0, 0], 8), p);
        assert_eq!("01000000/8", p.to_string());
    }

    #[test]
    fn test_dev_addr_to_le_bytes() {
        for tst in tests() {
            assert_eq!(tst.bytes, tst.devaddr.to_le_bytes());
        }
    }

    #[test]
    fn test_dev_addr_from_le_bytes() {
        for tst in tests() {
            assert_eq!(tst.devaddr, DevAddr::from_le_bytes(tst.bytes));
        }
    }

    #[test]
    fn test_dev_addr_netid_type() {
        for tst in tests() {
            assert_eq!(tst.netid_type, tst.devaddr.netid_type().unwrap());
        }
    }

    #[test]
    fn test_dev_addr_nwkid() {
        for tst in tests() {
            assert_eq!(tst.nwkid, tst.devaddr.nwkid().unwrap());
        }
    }

    #[test]
    fn test_dev_addr_string() {
        for tst in tests() {
            assert_eq!(tst.string, tst.devaddr.to_string());
        }
    }

    #[test]
    fn test_dev_addr_set_addr_prefix() {
        let tests = vec![
            SetAddrPrefixTest {
                devaddr: DevAddr::from_be_bytes([0xff, 0xff, 0xff, 0xff]),
                netid: NetID::from_be_bytes([0x00, 0x00, 0x00]),
                expected_devaddr: DevAddr::from_be_bytes([0x01, 0xff, 0xff, 0xff]),
            },
            SetAddrPrefixTest {
                devaddr: DevAddr::from_be_bytes([0xff, 0xff, 0xff, 0xff]),
                netid: NetID::from_be_bytes([0x00, 0x00, 0x3f]),
                expected_devaddr: DevAddr::from_be_bytes([0x7f, 0xff, 0xff, 0xff]),
            },
        ];

        for tst in tests {
            let mut devaddr = tst.devaddr.clone();
            devaddr.set_dev_addr_prefix(tst.netid.dev_addr_prefix());
            assert_eq!(tst.expected_devaddr, devaddr);
        }
    }
}
