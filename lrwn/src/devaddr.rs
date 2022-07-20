use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use serde::{Serialize, Serializer};

use super::netid::NetID;
use crate::Error;

#[derive(PartialEq, Copy, Clone, AsExpression, FromSqlRow, Default)]
#[diesel(sql_type = diesel::sql_types::Binary)]
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
        dev_addr.set_addr_prefix(&net_id);

        *self == dev_addr
    }

    pub fn netid_type(&self) -> u8 {
        for i in (0..=7).rev() {
            if self.0[0] & (1 << i) == 0 {
                return 7 - i;
            }
        }

        panic!("netid_type bug");
    }

    pub fn nwkid(&self) -> Vec<u8> {
        match self.netid_type() {
            0 => self.get_nwkid(1, 6),
            1 => self.get_nwkid(2, 6),
            2 => self.get_nwkid(3, 9),
            3 => self.get_nwkid(4, 11),
            4 => self.get_nwkid(5, 12),
            5 => self.get_nwkid(6, 13),
            6 => self.get_nwkid(7, 15),
            7 => self.get_nwkid(8, 17),
            _ => vec![],
        }
    }

    pub fn set_addr_prefix(&mut self, netid: &NetID) {
        match netid.netid_type() {
            0 => self.set_set_addr_prefix(1, 6, netid),
            1 => self.set_set_addr_prefix(2, 6, netid),
            2 => self.set_set_addr_prefix(3, 9, netid),
            3 => self.set_set_addr_prefix(4, 11, netid),
            4 => self.set_set_addr_prefix(5, 12, netid),
            5 => self.set_set_addr_prefix(6, 13, netid),
            6 => self.set_set_addr_prefix(7, 15, netid),
            7 => self.set_set_addr_prefix(8, 17, netid),
            _ => {}
        }
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

    fn set_set_addr_prefix(&mut self, prefix_length: u32, nwkid_bits: u32, netid: &NetID) {
        // convert devaddr to u32
        let mut devaddr = u32::from_be_bytes(self.0);

        // clear the bits for the prefix and NwkID
        let mask = u32::MAX; // all u32 bits to 1
        devaddr &= !(mask << (32 - prefix_length - nwkid_bits));

        // set the type prefix
        let prefix: u32 = 254 << (32 - prefix_length);
        let mut devaddr = devaddr | prefix;

        // set the NwkID
        let mut netid_bytes: [u8; 4] = [0; 4];
        let netid_id = netid.id();
        netid_bytes[4 - netid_id.len()..].clone_from_slice(&netid_id);

        let mut nwkid = u32::from_be_bytes(netid_bytes);
        nwkid <<= 32 - nwkid_bits; // truncate the MSB of the NetID ID field
        nwkid >>= prefix_length; // shift base for the prefix MSB
        devaddr |= nwkid;

        self.0 = devaddr.to_be_bytes();
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

impl Serialize for DevAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

use diesel::backend::{self, Backend};
use diesel::sql_types::Binary;
use diesel::{deserialize, serialize};

impl<DB> deserialize::FromSql<Binary, DB> for DevAddr
where
    DB: Backend,
    *const [u8]: deserialize::FromSql<Binary, DB>,
{
    fn from_sql(value: backend::RawValue<DB>) -> deserialize::Result<Self> {
        let bytes = Vec::<u8>::from_sql(value)?;
        if bytes.len() != 4 {
            return Err("DevAddr type expects exactly 4 bytes".into());
        }

        let mut b: [u8; 4] = [0; 4];
        b.copy_from_slice(&bytes);

        Ok(DevAddr(b))
    }
}

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
    fn test_to_le_bytes() {
        for tst in tests() {
            assert_eq!(tst.bytes, tst.devaddr.to_le_bytes());
        }
    }

    #[test]
    fn test_from_le_bytes() {
        for tst in tests() {
            assert_eq!(tst.devaddr, DevAddr::from_le_bytes(tst.bytes));
        }
    }

    #[test]
    fn test_netid_type() {
        for tst in tests() {
            assert_eq!(tst.netid_type, tst.devaddr.netid_type());
        }
    }

    #[test]
    fn test_nwkid() {
        for tst in tests() {
            assert_eq!(tst.nwkid, tst.devaddr.nwkid());
        }
    }

    #[test]
    fn test_string() {
        for tst in tests() {
            assert_eq!(tst.string, tst.devaddr.to_string());
        }
    }

    #[test]
    fn test_set_addr_prefix() {
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
            devaddr.set_addr_prefix(&tst.netid);
            assert_eq!(tst.expected_devaddr, devaddr);
        }
    }
}
