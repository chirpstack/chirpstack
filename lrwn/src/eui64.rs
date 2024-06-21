use std::fmt;
use std::str::FromStr;

use anyhow::{Context, Result};
#[cfg(feature = "sqlite")]
use diesel::sqlite::Sqlite;
#[cfg(feature = "diesel")]
use diesel::{backend::Backend, deserialize, serialize, sql_types::Binary};
#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::Error;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "diesel", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "diesel", diesel(sql_type = diesel::sql_types::Binary))]
pub struct EUI64([u8; 8]);

impl EUI64 {
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        if b.len() != 8 {
            return Err(Error::Eui64Length);
        }

        let mut bb: [u8; 8] = [0; 8];
        bb.copy_from_slice(b);

        Ok(EUI64(bb))
    }

    pub fn from_be_bytes(b: [u8; 8]) -> Self {
        EUI64(b)
    }

    pub fn from_le_bytes(b: [u8; 8]) -> Self {
        let mut b = b;
        b.reverse();
        EUI64(b)
    }

    pub fn to_be_bytes(&self) -> [u8; 8] {
        self.0
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        let mut b = self.0;
        b.reverse();
        b
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl fmt::Display for EUI64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl fmt::Debug for EUI64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl FromStr for EUI64 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes: [u8; 8] = [0; 8];
        hex::decode_to_slice(s, &mut bytes)?;
        Ok(EUI64(bytes))
    }
}

#[cfg(feature = "serde")]
impl Serialize for EUI64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EUI64 {
    fn deserialize<D>(deserialize: D) -> Result<EUI64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(Eui64Visitor)
    }
}

#[cfg(feature = "serde")]
struct Eui64Visitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for Eui64Visitor {
    type Value = EUI64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An EUI64 in the format 0102030405060708 is expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EUI64::from_str(value).map_err(|e| E::custom(format!("{}", e)))
    }
}

#[cfg(feature = "diesel")]
impl<ST, DB> deserialize::FromSql<ST, DB> for EUI64
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<ST, DB>,
{
    fn from_sql(value: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes = <Vec<u8> as deserialize::FromSql<ST, DB>>::from_sql(value)?;
        if bytes.len() != 8 {
            return Err("EUI64 type expects exactly 8 bytes".into());
        }

        let mut b: [u8; 8] = [0; 8];
        b.copy_from_slice(&bytes);

        Ok(EUI64(b))
    }
}

#[cfg(feature = "postgres")]
impl serialize::ToSql<Binary, diesel::pg::Pg> for EUI64
where
    [u8]: serialize::ToSql<Binary, diesel::pg::Pg>,
{
    fn to_sql(&self, out: &mut serialize::Output<'_, '_, diesel::pg::Pg>) -> serialize::Result {
        <[u8] as serialize::ToSql<Binary, diesel::pg::Pg>>::to_sql(
            &self.to_be_bytes(),
            &mut out.reborrow(),
        )
    }
}

#[cfg(feature = "sqlite")]
impl serialize::ToSql<Binary, Sqlite> for EUI64 {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(Vec::from(self.to_be_bytes().as_slice()));
        Ok(serialize::IsNull::No)
    }
}

#[cfg(feature = "diesel")]
impl diesel::sql_types::SqlType for EUI64 {
    type IsNull = diesel::sql_types::is_nullable::NotNull;
}

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EUI64Prefix([u8; 8], u64);

impl EUI64Prefix {
    pub fn new(prefix: [u8; 8], size: u64) -> Self {
        EUI64Prefix(prefix, size)
    }

    pub fn matches(&self, eui: EUI64) -> bool {
        if self.size() == 0 {
            return true;
        }

        let eui = u64::from_be_bytes(eui.to_be_bytes());
        let prefix = u64::from_be_bytes(self.prefix());
        let shift = 64 - self.size();

        (prefix >> shift) == (eui >> shift)
    }

    fn prefix(&self) -> [u8; 8] {
        self.0
    }

    fn size(&self) -> u64 {
        self.1
    }
}

impl fmt::Display for EUI64Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.prefix()), self.size())
    }
}

impl fmt::Debug for EUI64Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", hex::encode(self.prefix()), self.size())
    }
}

impl FromStr for EUI64Prefix {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        let mut size: u64 = 64;
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            size = parts[1].parse().map_err(|_| Error::EUI64PrefixFormat)?;
        }
        if parts.len() > 2 {
            return Err(Error::EUI64PrefixFormat);
        }
        if parts[0].len() != 16 {
            return Err(Error::EUI64PrefixFormat);
        }

        let mut mask: [u8; 8] = [0; 8];
        hex::decode_to_slice(parts[0], &mut mask).context("Decode EUI64Prefix")?;

        Ok(EUI64Prefix(mask, size))
    }
}

#[cfg(feature = "serde")]
impl Serialize for EUI64Prefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EUI64Prefix {
    fn deserialize<D>(deserialize: D) -> Result<EUI64Prefix, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(EUI64PrefixVisitor)
    }
}

#[cfg(feature = "serde")]
struct EUI64PrefixVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for EUI64PrefixVisitor {
    type Value = EUI64Prefix;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A EUI64Prefix in the format 0000000000000000/0 is expected")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        EUI64Prefix::from_str(value).map_err(|e| E::custom(format!("{}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eui64_to_le_bytes() {
        let eui = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

        assert_eq!(
            eui.to_le_bytes(),
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
    }

    #[test]
    fn test_eui64_from_le_bytes() {
        let eui64_from_le = EUI64::from_le_bytes([0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
        let eui64_from_be = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

        assert_eq!(eui64_from_be, eui64_from_le);
    }

    #[test]
    fn test_eui64_to_string() {
        let eui = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(eui.to_string(), "0102030405060708");
    }

    #[test]
    fn test_eui64_from_str() {
        let eui = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(eui, EUI64::from_str(&"0102030405060708").unwrap());
    }

    #[test]
    fn test_eui64_prefix() {
        let p = EUI64Prefix::from_str("0102030405060708").unwrap();
        assert_eq!(EUI64Prefix::new([1, 2, 3, 4, 5, 6, 7, 8], 64), p);
        assert_eq!("0102030405060708/64", p.to_string());

        let p = EUI64Prefix::from_str("0100000000000000/8").unwrap();
        assert_eq!(EUI64Prefix::new([1, 0, 0, 0, 0, 0, 0, 0], 8), p);
        assert_eq!("0100000000000000/8", p.to_string());
    }

    #[test]
    fn test_eui64_prefix_is_eui64() {
        struct Test {
            prefix: EUI64Prefix,
            eui: EUI64,
            matches: bool,
        }

        let tests = vec![
            Test {
                prefix: EUI64Prefix::from_str("0000000000000000/0").unwrap(),
                eui: EUI64::from_str("0000000000000000").unwrap(),
                matches: true,
            },
            Test {
                prefix: EUI64Prefix::from_str("0000000000000000/0").unwrap(),
                eui: EUI64::from_str("ffffffffffffffff").unwrap(),
                matches: true,
            },
            Test {
                eui: EUI64::from_str("ffffffff00000000").unwrap(),
                prefix: EUI64Prefix::from_str("ff00000000000000/8").unwrap(),
                matches: true,
            },
            Test {
                eui: EUI64::from_str("ffffffff00000000").unwrap(),
                prefix: EUI64Prefix::from_str("ff00000000000000/9").unwrap(),
                matches: false,
            },
        ];

        for tst in &tests {
            assert_eq!(tst.matches, tst.prefix.matches(tst.eui));
        }
    }
}
