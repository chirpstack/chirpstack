use anyhow::Result;
#[cfg(feature = "serde")]
use serde::Serialize;

use super::devaddr::DevAddr;
use super::maccommand::MACCommandSet;

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FHDR {
    pub devaddr: DevAddr,
    pub f_ctrl: FCtrl,
    pub f_cnt: u32,
    pub f_opts: MACCommandSet,
}

impl Default for FHDR {
    fn default() -> Self {
        FHDR {
            devaddr: DevAddr::from_be_bytes([0x00, 0x00, 0x00, 0x00]),
            f_ctrl: FCtrl::default(),
            f_cnt: 0,
            f_opts: MACCommandSet::new(Vec::new()),
        }
    }
}

impl FHDR {
    pub fn from_slice(b: &[u8]) -> Result<Self> {
        if b.len() < 7 {
            return Err(anyhow!("at least 7 bytes are expected"));
        }

        let mut fhdr = FHDR {
            devaddr: {
                let mut devaddr: [u8; 4] = [0; 4];
                devaddr.clone_from_slice(&b[0..4]);
                DevAddr::from_le_bytes(devaddr)
            },
            f_ctrl: {
                let mut f_ctrl: [u8; 1] = [0];
                f_ctrl.clone_from_slice(&b[4..5]);
                FCtrl::from_le_bytes(f_ctrl)
            },
            f_cnt: {
                // note that only the 16lsb are encoded!
                let mut f_cnt: [u8; 4] = [0; 4];
                f_cnt[0..2].clone_from_slice(&b[5..7]);
                u32::from_le_bytes(f_cnt)
            },
            f_opts: MACCommandSet::new(vec![]),
        };

        if fhdr.f_ctrl.f_opts_len != 0 {
            // check that the remaining bytes equal the f_opts_len
            if b.len() - 7 != fhdr.f_ctrl.f_opts_len as usize {
                return Err(anyhow!(
                    "available f_opts bytes does not match with f_opts_len"
                ));
            }

            fhdr.f_opts = MACCommandSet::from_slice(&b[7..]);
        }

        Ok(fhdr)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        // clone FCtrl as mutable
        let mut f_ctrl = self.f_ctrl.clone();

        // get f_opts bytes and set f_opts_len to number of f_opts bytes
        let f_opts = self.f_opts.to_vec()?;
        f_ctrl.f_opts_len = f_opts.len() as u8;

        let mut b = Vec::with_capacity(7 + f_opts.len());

        b.extend_from_slice(&self.devaddr.to_le_bytes());
        b.extend_from_slice(&f_ctrl.to_le_bytes()?);
        b.extend_from_slice(&self.f_cnt.to_le_bytes()[0..2]); // only take the 16 lsb
        b.extend_from_slice(&f_opts);

        Ok(b)
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FCtrl {
    pub adr: bool,
    pub adr_ack_req: bool,
    pub ack: bool,
    pub f_pending: bool,
    pub class_b: bool,
    /// Only used when decoding from_le_bytes.
    pub f_opts_len: u8,
}

impl FCtrl {
    pub fn from_le_bytes(b: [u8; 1]) -> Self {
        FCtrl {
            adr: b[0] & 0x80 != 0,
            adr_ack_req: b[0] & 0x40 != 0,
            ack: b[0] & 0x20 != 0,
            class_b: b[0] & 0x10 != 0,
            f_pending: b[0] & 0x010 != 0,
            f_opts_len: b[0] & 0x0f,
        }
    }

    pub fn to_le_bytes(&self) -> Result<[u8; 1]> {
        if self.f_opts_len > 15 {
            return Err(anyhow!("max value of f_opts_len is 15"));
        }

        let mut b: u8 = 0;
        if self.adr {
            b |= 0x80;
        }
        if self.adr_ack_req {
            b |= 0x40;
        }
        if self.ack {
            b |= 0x20;
        }
        if self.class_b || self.f_pending {
            b |= 0x10;
        }
        b |= self.f_opts_len & 0x0f;

        Ok([b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FCtrlTest {
        f_ctrl: FCtrl,
        bytes: [u8; 1],
    }

    #[test]
    fn test_f_ctrl() {
        let tests = vec![
            FCtrlTest {
                f_ctrl: FCtrl {
                    adr: true,
                    f_opts_len: 2,
                    ..Default::default()
                },
                bytes: [0x82],
            },
            FCtrlTest {
                f_ctrl: FCtrl {
                    adr_ack_req: true,
                    f_opts_len: 3,
                    ..Default::default()
                },
                bytes: [0x43],
            },
            FCtrlTest {
                f_ctrl: FCtrl {
                    ack: true,
                    f_opts_len: 4,
                    ..Default::default()
                },
                bytes: [0x24],
            },
            FCtrlTest {
                f_ctrl: FCtrl {
                    f_pending: true,
                    f_opts_len: 5,
                    ..Default::default()
                },
                bytes: [0x15],
            },
            FCtrlTest {
                f_ctrl: FCtrl {
                    class_b: true,
                    f_opts_len: 5,
                    ..Default::default()
                },
                bytes: [0x15],
            },
            FCtrlTest {
                f_ctrl: FCtrl {
                    adr: true,
                    adr_ack_req: true,
                    ack: true,
                    f_pending: true,
                    f_opts_len: 6,
                    ..Default::default()
                },
                bytes: [0xf6],
            },
            FCtrlTest {
                f_ctrl: FCtrl {
                    adr: true,
                    adr_ack_req: true,
                    ack: true,
                    class_b: true,
                    f_opts_len: 6,
                    ..Default::default()
                },
                bytes: [0xf6],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.f_ctrl.to_le_bytes().unwrap());

            // as class_b and f_pending share the same bit, we can't directly compare
            let f_ctrl = FCtrl::from_le_bytes(tst.bytes);
            assert_eq!(tst.f_ctrl.adr, f_ctrl.adr);
            assert_eq!(tst.f_ctrl.adr_ack_req, f_ctrl.adr_ack_req);
            assert_eq!(tst.f_ctrl.ack, f_ctrl.ack);
            assert_eq!(tst.f_ctrl.f_pending || tst.f_ctrl.class_b, f_ctrl.f_pending);
            assert_eq!(tst.f_ctrl.f_pending || tst.f_ctrl.class_b, f_ctrl.class_b);
            assert_eq!(tst.f_ctrl.f_opts_len, f_ctrl.f_opts_len);
        }
    }
}
