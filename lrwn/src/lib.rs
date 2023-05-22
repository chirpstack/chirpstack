#[cfg(feature = "diesel")]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

pub use self::aes128::*;
pub use self::cflist::*;
pub use self::devaddr::*;
pub use self::dl_settings::*;
pub use self::error::*;
pub use self::eui64::*;
pub use self::fhdr::*;
pub use self::maccommand::*;
pub use self::mhdr::*;
pub use self::netid::*;
pub use self::payload::*;
pub use self::phy_payload::*;
pub use self::relay::*;

mod aes128;
mod cflist;
mod devaddr;
mod dl_settings;
mod error;
mod eui64;
mod fhdr;
mod helpers;
#[cfg(feature = "crypto")]
pub mod keys;
mod maccommand;
mod mhdr;
mod netid;
mod payload;
mod phy_payload;
#[cfg(feature = "regions")]
pub mod region;
mod relay;

pub const LA_FPORT_RELAY: u8 = 226;

lazy_static! {
    static ref EIRP_INDEX: Vec<f32> = vec![
        8.0,  // 0
        10.0, // 1
        12.0, // 2
        13.0, // 3
        14.0, // 4
        16.0, // 5
        18.0, // 6
        20.0, // 7
        21.0, // 8
        24.0, // 9
        26.0, // 10
        27.0, // 11
        29.0, // 12
        30.0, // 13
        33.0, // 14
        36.0, // 15
    ];
}

pub fn get_tx_param_setup_eirp_index(eirp: f32) -> u8 {
    let mut out: u8 = 0;
    for (i, e) in EIRP_INDEX.iter().cloned().enumerate() {
        if e > eirp {
            return out;
        }
        out = i as u8;
    }

    out
}

use anyhow::Result;

pub fn get_tx_param_setup_eirp(i: u8) -> Result<f32> {
    EIRP_INDEX
        .get(i as usize)
        .cloned()
        .ok_or_else(|| anyhow!("Index does not exist"))
}
