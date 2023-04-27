use anyhow::Result;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DLSettings {
    pub opt_neg: bool,
    pub rx2_dr: u8,
    pub rx1_dr_offset: u8,
}

impl DLSettings {
    pub fn from_le_bytes(b: [u8; 1]) -> Self {
        DLSettings {
            opt_neg: (b[0] & (1 << 7)) != 0,
            rx2_dr: b[0] & 15,
            rx1_dr_offset: (b[0] & 112) >> 4,
        }
    }

    pub fn to_le_bytes(&self) -> Result<[u8; 1]> {
        if self.rx2_dr > 15 {
            return Err(anyhow!("max value of rx2_dr is 15"));
        }

        if self.rx1_dr_offset > 7 {
            return Err(anyhow!("max value of rx1_dr_offset is 7"));
        }

        let mut b: u8 = self.rx2_dr | (self.rx1_dr_offset << 4);

        if self.opt_neg {
            b |= 1 << 7;
        }

        Ok([b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dl_settings() {
        let dl_settings = DLSettings {
            rx2_dr: 15,
            rx1_dr_offset: 7,
            opt_neg: true,
        };

        assert_eq!([0xff], dl_settings.to_le_bytes().unwrap());
        assert_eq!(dl_settings, DLSettings::from_le_bytes([0xff]));
    }
}
