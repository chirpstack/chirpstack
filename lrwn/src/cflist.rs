use anyhow::Result;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum CFList {
    Channels(CFListChannels),
    ChannelMask(CFListChannelMasks),
}

impl CFList {
    pub fn from_bytes(b: [u8; 16]) -> Result<Self> {
        let mut bb: [u8; 15] = [0; 15];
        bb.clone_from_slice(&b[..15]);

        // match on CFListType.
        match b[15] {
            0x00 => Ok(CFList::Channels(CFListChannels::from_bytes(bb))),
            0x01 => Ok(CFList::ChannelMask(CFListChannelMasks::from_bytes(bb))),
            _ => {
                return Err(anyhow!("unexpected CFListType"));
            }
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; 16]> {
        let mut b: [u8; 16] = [0; 16];

        match self {
            CFList::Channels(v) => {
                b[..15].clone_from_slice(&v.to_bytes()?);
                b[15] = 0x00;
            }
            CFList::ChannelMask(v) => {
                b[..15].clone_from_slice(&v.to_bytes()?);
                b[15] = 0x01;
            }
        }

        Ok(b)
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct CFListChannels([u32; 5]);

impl CFListChannels {
    pub fn new(channels: [u32; 5]) -> Self {
        CFListChannels(channels)
    }

    pub fn from_slice(channels: &[u32]) -> Result<Self> {
        let mut channels_array: [u32; 5] = [0; 5];
        if channels.len() > channels_array.len() {
            return Err(anyhow!(
                "Max number of channels is {}",
                channels_array.len()
            ));
        }

        for (i, c) in channels.iter().enumerate() {
            channels_array[i] = *c;
        }

        Ok(Self::new(channels_array))
    }

    pub fn from_bytes(b: [u8; 15]) -> Self {
        let mut channels: [u32; 5] = [0; 5];

        for (i, ch) in channels.iter_mut().enumerate() {
            let index = i * 3;
            *ch = u32::from_le_bytes([b[index], b[index + 1], b[index + 2], 0x00]) * 100;
        }

        CFListChannels(channels)
    }

    pub fn to_bytes(&self) -> Result<[u8; 15]> {
        let mut b: [u8; 15] = [0; 15];

        for (i, f) in self.0.iter().enumerate() {
            if f % 100 != 0 {
                return Err(anyhow!("frequency must be a multiple of 100"));
            }

            let f = f / 100;
            if f > (1 << 24) - 1 {
                return Err(anyhow!("max value of frequency is 2^24-1"));
            }

            let index = i * 3;
            b[index..index + 3].clone_from_slice(&f.to_le_bytes()[..3]);
        }

        Ok(b)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.0.iter()
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct CFListChannelMasks(Vec<ChMask>);

impl CFListChannelMasks {
    pub fn new(masks: Vec<ChMask>) -> Self {
        CFListChannelMasks(masks)
    }

    pub fn from_bytes(b: [u8; 15]) -> Self {
        let b = b[..14].to_vec(); // each mask uses 2 bytes

        let mut masks: Vec<ChMask> = Vec::with_capacity(b.len() / 3);
        let mut pending: Vec<ChMask> = Vec::with_capacity(b.len() / 2);

        for i in (0..b.len()).step_by(2) {
            pending.push(ChMask::from_bytes([b[i], b[i + 1]]));

            if b[i..i + 2] != vec![0x00, 0x00] {
                masks.append(&mut pending);
            }
        }

        CFListChannelMasks(masks)
    }

    pub fn to_bytes(&self) -> Result<[u8; 15]> {
        if self.0.len() > 6 {
            return Err(anyhow!("max number of channel-masks is 6"));
        }

        let mut b: [u8; 15] = [0; 15];

        for (i, mask) in self.0.iter().enumerate() {
            let index = i * 2;
            b[index..index + 2].clone_from_slice(&mask.to_bytes());
        }

        Ok(b)
    }
}

/// ChMask encodes the channels usable for uplink access. 0 = channel 1,
/// 15 = channel 16.
#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub struct ChMask([bool; 16]);

impl ChMask {
    const SIZE: usize = 2;

    pub fn new(mask: [bool; 16]) -> Self {
        ChMask(mask)
    }

    pub fn set(&mut self, i: usize, v: bool) {
        self.0[i] = v
    }

    pub fn from_slice(mask: &[bool]) -> Result<Self> {
        let mut mask_array: [bool; 16] = [false; 16];

        if mask.len() > mask_array.len() {
            return Err(anyhow!("Max number of masks is {}", mask_array.len()));
        }

        for (i, m) in mask.iter().enumerate() {
            mask_array[i] = *m;
        }

        Ok(Self::new(mask_array))
    }

    pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
        let mut mask: [bool; 16] = [false; 16];

        let n = u16::from_le_bytes(b);
        for (i, m) in mask.iter_mut().enumerate() {
            *m = n & (1 << i) != 0;
        }

        ChMask(mask)
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut n: u16 = 0;
        for i in 0..16 {
            if self.0[i] {
                n |= 1 << i;
            }
        }

        n.to_le_bytes()
    }
}

impl IntoIterator for ChMask {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[allow(clippy::unnecessary_to_owned)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        cflist: CFList,
        bytes: [u8; 16],
    }

    fn tests() -> Vec<Test> {
        vec![
            Test {
                cflist: CFList::ChannelMask(CFListChannelMasks::new(vec![ChMask::new([
                    true, true, true, true, true, true, true, true, false, false, false, false,
                    false, false, false, false,
                ])])),
                bytes: [
                    0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x01,
                ],
            },
            Test {
                cflist: CFList::ChannelMask(CFListChannelMasks::new(vec![
                    ChMask::new([false; 16]),
                    ChMask::new([
                        true, true, true, true, true, true, true, true, false, false, false, false,
                        false, false, false, false,
                    ]),
                ])),
                bytes: [
                    0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x01,
                ],
            },
            Test {
                cflist: CFList::Channels(CFListChannels::new([
                    867100000, 867300000, 867500000, 867700000, 867900000,
                ])),
                bytes: [
                    0x18, 0x4f, 0x84, 0xe8, 0x56, 0x84, 0xb8, 0x5e, 0x84, 0x88, 0x66, 0x84, 0x58,
                    0x6e, 0x84, 0x0,
                ],
            },
        ]
    }

    #[test]
    fn test_to_bytes() {
        for tst in tests() {
            assert_eq!(tst.bytes, tst.cflist.to_bytes().unwrap());
        }
    }

    #[test]
    fn test_from_bytes() {
        for tst in tests() {
            assert_eq!(tst.cflist, CFList::from_bytes(tst.bytes).unwrap());
        }
    }
}
