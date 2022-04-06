use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

#[derive(Copy, Clone, PartialEq, Eq, Hash, AsExpression, FromSqlRow, Default)]
#[sql_type = "diesel::sql_types::Binary"]
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

impl Serialize for EUI64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EUI64 {
    fn deserialize<D>(deserialize: D) -> Result<EUI64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize.deserialize_str(Eui64Visitor)
    }
}

struct Eui64Visitor;

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

use std::io::Write;

use diesel::backend::Backend;
use diesel::sql_types::Binary;
use diesel::{deserialize, serialize};

impl<DB> deserialize::FromSql<diesel::sql_types::Binary, DB> for EUI64
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<diesel::sql_types::Binary, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let bytes = Vec::<u8>::from_sql(bytes)?;
        if bytes.len() != 8 {
            return Err("EUI64 type expects exactly 8 bytes".into());
        }

        let mut b: [u8; 8] = [0; 8];
        b.copy_from_slice(&bytes);

        Ok(EUI64(b))
    }
}

impl<DB> serialize::ToSql<Binary, DB> for EUI64
where
    DB: Backend,
    [u8]: serialize::ToSql<Binary, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        (&self.to_be_bytes() as &[u8]).to_sql(out)
    }
}

impl diesel::sql_types::NotNull for EUI64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_le_bytes() {
        let eui = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

        assert_eq!(
            eui.to_le_bytes(),
            [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
        );
    }

    #[test]
    fn test_from_le_bytes() {
        let eui64_from_le = EUI64::from_le_bytes([0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
        let eui64_from_be = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

        assert_eq!(eui64_from_be, eui64_from_le);
    }

    #[test]
    fn test_to_string() {
        let eui = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(eui.to_string(), "0102030405060708");
    }

    #[test]
    fn test_from_str() {
        let eui = EUI64::from_be_bytes([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(eui, EUI64::from_str(&"0102030405060708").unwrap());
    }
}
