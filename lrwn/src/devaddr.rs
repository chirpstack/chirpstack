use std::fmt;
use std::str::FromStr;

use anyhow::Result;
#[cfg(feature = "postgres")]
use diesel::pg::Pg;
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
#[cfg(feature = "diesel")]
use diesel::{backend::Backend, deserialize, serialize, sql_types::Binary, sql_types::Text};
#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use super::netid::NetID;
use crate::Error;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DevAddrPrefix([u8; 4], u32);

impl DevAddrPrefix {
    pub fn new(prefix: [u8; 4], size: u32) -> Self {
        DevAddrPrefix(prefix, size)
    }

    pub fn is_subset_of(&self, other: &DevAddrPrefix) -> bool {
        self.range_min() >= other.range_min() && self.range_max() <= other.range_max()
    }

    fn prefix(&self) -> [u8; 4] {
        self.0
    }

    fn size(&self) -> u32 {
        self.1
    }

    fn range_min(&self) -> u32 {
        let mask = u32::from_be_bytes(self.prefix());
        mask & (u32::MAX << (32 - self.size()))
    }

    fn range_max(&self) -> u32 {
        self.range_min() + (u32::MAX >> self.size())
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
        let mut size: u32 = 32;
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            size = parts[1].parse().map_err(|_| Error::DevAddrPrefixFormat)?;
        }
        if parts.len() > 2 {
            return Err(Error::DevAddrPrefixFormat);
        }

        if parts[0].len() != 8 {
            return Err(Error::DevAddrPrefixFormat);
        }

        if size > 32 {
            return Err(Error::DevAddrPrefixFormat);
        }

        let mut mask: [u8; 4] = [0; 4];
        hex::decode_to_slice(parts[0], &mut mask)?;

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
impl Visitor<'_> for DevAddrPrefixVisitor {
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

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Text, Pg> for DevAddrPrefix {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as deserialize::FromSql<Text, Pg>>::from_sql(bytes)?;
        Ok(DevAddrPrefix::from_str(&value)?)
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Text, Pg> for DevAddrPrefix {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <String as serialize::ToSql<Text, Pg>>::to_sql(&self.to_string(), &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Text, Sqlite> for DevAddrPrefix {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as deserialize::FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(DevAddrPrefix::from_str(&value)?)
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Text, Sqlite> for DevAddrPrefix {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.to_string());
        Ok(serialize::IsNull::No)
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
        if !s.is_empty() {
            hex::decode_to_slice(s, &mut bytes)?;
        }
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

#[cfg(feature = "postgres")]
impl deserialize::FromSql<Binary, Pg> for DevAddr {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <Vec<u8> as deserialize::FromSql<Binary, Pg>>::from_sql(bytes)?;
        if value.len() != 4 {
            return Err("DevAddr type expects exactly 4 bytes".into());
        }

        let mut b: [u8; 4] = [0; 4];
        b.copy_from_slice(&value);
        Ok(DevAddr(b))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Binary, Pg> for DevAddr {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        <[u8] as serialize::ToSql<Binary, Pg>>::to_sql(&self.to_be_bytes(), &mut out.reborrow())
    }
}

#[cfg(feature = "sqlite")]
impl deserialize::FromSql<Binary, Sqlite> for DevAddr {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <Vec<u8> as deserialize::FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        if value.len() != 4 {
            return Err("DevAddr type expects exactly 4 bytes".into());
        }

        let mut b: [u8; 4] = [0; 4];
        b.copy_from_slice(&value);
        Ok(DevAddr(b))
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Binary, Sqlite> for DevAddr {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.to_be_bytes().to_vec());
        Ok(serialize::IsNull::No)
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

        let p = DevAddrPrefix::from_str("01020304").unwrap();
        assert_eq!(DevAddrPrefix::new([1, 2, 3, 4], 32), p);
        assert_eq!("01020304/32", p.to_string());
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
            let mut devaddr = tst.devaddr;
            devaddr.set_dev_addr_prefix(tst.netid.dev_addr_prefix());
            assert_eq!(tst.expected_devaddr, devaddr);
        }
    }

    #[test]
    fn test_prefix_is_subset() {
        assert!(
            DevAddrPrefix::from_str("ffff0000/16")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("ff000000/8").unwrap()),
            "prefix was not a subset"
        );

        assert!(
            DevAddrPrefix::from_str("ffff0000/16")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("ffff0000/16").unwrap()),
            "prefix was not equal"
        );

        assert!(
            !DevAddrPrefix::from_str("ffff0000/15")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("ffff0000/16").unwrap()),
            "prefix was a sub-set"
        );

        assert!(
            DevAddrPrefix::from_str("00000000/16")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("00000000/15").unwrap()),
            "prefix was not a sub-set"
        );

        assert!(
            !DevAddrPrefix::from_str("00000000/15")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("00000000/16").unwrap()),
            "prefix was a sub-set"
        );

        assert!(
            DevAddrPrefix::from_str("00000000/8")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("00000000/7").unwrap()),
            "prefix was not a sub-set"
        );

        assert!(
            !DevAddrPrefix::from_str("80000000/8")
                .unwrap()
                .is_subset_of(&DevAddrPrefix::from_str("00000000/8").unwrap()),
            "prefix was a sub-set"
        );
    }
}
