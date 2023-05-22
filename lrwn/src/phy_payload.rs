#[cfg(feature = "crypto")]
use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt},
    Aes128, Block,
};
use anyhow::Result;
#[cfg(feature = "crypto")]
use cmac::{Cmac, Mac};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::maccommand::{MACCommand, MACCommandSet};
use super::mhdr::{MType, MHDR};
use super::payload::{FRMPayload, MACPayload, Payload};
#[cfg(feature = "crypto")]
use super::{
    aes128::AES128Key,
    devaddr::DevAddr,
    eui64::EUI64,
    payload::{JoinAcceptPayload, JoinType},
};
use crate::relay::{ForwardDownlinkReq, ForwardUplinkReq};
use crate::LA_FPORT_RELAY;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MACVersion {
    LoRaWAN1_0,
    LoRaWAN1_1,
}

/// PhyPayload represents the LoRaWAN PHY payload.
///
/// Join-request example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// let app_key = AES128Key::from_str("0102030405060708090a0b0c0d0e0f10").unwrap();
///
/// let mut phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinRequest,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinRequest(JoinRequestPayload {
///         join_eui: EUI64::from_str("0101010101010101").unwrap(),
///         dev_eui: EUI64::from_str("0202020202020202").unwrap(),
///         dev_nonce: 771,
///     }),
///     mic: None,
/// };
///
/// phy.set_join_request_mic(&app_key).unwrap();
/// assert_eq!([0x9, 0xb9, 0x7b, 0x32], phy.mic.unwrap());
///
/// let bytes = phy.to_vec().unwrap();
/// assert_eq!(vec![0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x03, 0x03, 0x09, 0xb9, 0x7b, 0x32], bytes);
///
/// let phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// assert_eq!(phy, phy_decoded);
/// assert_eq!(true, phy_decoded.validate_join_request_mic(&app_key).unwrap());
/// ```
///
/// LoRaWAN 1.0.x Join-accept example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// let app_key = AES128Key::from_str("0102030405060708090a0b0c0d0e0f10").unwrap();
/// let join_eui = EUI64::from_str("0807060504030201").unwrap();
/// let dev_nonce = 258;
///
/// let mut phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinAccept,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinAccept(JoinAcceptPayload {
///         join_nonce: 65793,
///         home_netid: NetID::from_str("020202").unwrap(),
///         devaddr: DevAddr::from_str("01020304").unwrap(),
///         dl_settings: DLSettings {
///             opt_neg: false,
///             rx2_dr: 0,
///             rx1_dr_offset: 0,
///         },
///         cflist: None,
///         rx_delay: 0,
///     }),
///     mic: None,
/// };
///
/// phy.set_join_accept_mic(JoinType::Join, &join_eui, dev_nonce, &app_key).unwrap();
/// assert_eq!([0x34, 0x49, 0xf2, 0x12], phy.mic.unwrap());
///
/// phy.encrypt_join_accept_payload(&app_key).unwrap();
///
/// let bytes = phy.to_vec().unwrap();
/// assert_eq!(vec![0x20, 0x23, 0xcf, 0x33, 0x54, 0x89, 0xaa, 0xe3, 0x18, 0x3c, 0x0b, 0xe0, 0xba, 0xa8, 0xde, 0xe5, 0xf3], bytes);
///
/// let mut phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// phy_decoded.decrypt_join_accept_payload(&app_key).unwrap();
/// assert_eq!(true, phy_decoded.validate_join_accept_mic(JoinType::Join, &join_eui, dev_nonce, &app_key).unwrap());
///
/// assert_eq!(PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinAccept,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinAccept(JoinAcceptPayload {
///         join_nonce: 65793,
///         home_netid: NetID::from_str("020202").unwrap(),
///         devaddr: DevAddr::from_str("01020304").unwrap(),
///         dl_settings: DLSettings {
///             opt_neg: false,
///             rx2_dr: 0,
///             rx1_dr_offset: 0,
///         },
///         cflist: None,
///         rx_delay: 0,
///     }),
///     mic: Some([0x34, 0x49, 0xf2, 0x12]),
/// }, phy_decoded);
/// ```
///
/// LoRaWAN 1.1.x Join-accept example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// let app_key = AES128Key::from_str("0102030405060708090a0b0c0d0e0f10").unwrap();
/// let join_eui = EUI64::from_str("0807060504030201").unwrap();
/// let dev_nonce = 258;
///
/// let mut phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinAccept,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinAccept(JoinAcceptPayload {
///         join_nonce: 65793,
///         home_netid: NetID::from_str("020202").unwrap(),
///         devaddr: DevAddr::from_str("01020304").unwrap(),
///         dl_settings: DLSettings {
///             opt_neg: true, // Note that opt_neg is set to true!
///             rx2_dr: 0,
///             rx1_dr_offset: 0,
///         },
///         cflist: None,
///         rx_delay: 0,
///     }),
///     mic: None,
/// };
///
/// phy.set_join_accept_mic(JoinType::Join, &join_eui, dev_nonce, &app_key).unwrap();
/// assert_eq!([0x93, 0xff, 0x9a, 0x3a], phy.mic.unwrap());
///
/// phy.encrypt_join_accept_payload(&app_key).unwrap();
///
/// let bytes = phy.to_vec().unwrap();
/// assert_eq!(vec![0x20, 0x7a, 0xbe, 0xea, 0x06, 0xb0, 0x29, 0x20, 0xf1, 0x1c, 0x02, 0xd0, 0x34, 0x8f, 0xcf, 0x18, 0x15], bytes);
///
/// let mut phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// phy_decoded.decrypt_join_accept_payload(&app_key).unwrap();
/// assert_eq!(true, phy_decoded.validate_join_accept_mic(JoinType::Join, &join_eui, dev_nonce, &app_key).unwrap());
///
/// assert_eq!(PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinAccept,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinAccept(JoinAcceptPayload {
///         join_nonce: 65793,
///         home_netid: NetID::from_str("020202").unwrap(),
///         devaddr: DevAddr::from_str("01020304").unwrap(),
///         dl_settings: DLSettings {
///             opt_neg: true,
///             rx2_dr: 0,
///             rx1_dr_offset: 0,
///         },
///         cflist: None,
///         rx_delay: 0,
///     }),
///     mic: Some([0x93, 0xff, 0x9a, 0x3a]),
/// }, phy_decoded);
/// ```
///
/// LoRaWAN 1.0.x confirmed uplink example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// let nwk_s_key = AES128Key::from_str("0102030405060708090a0b0c0d0e0f10").unwrap();
/// let app_s_key = AES128Key::from_str("100f0e0d0c0b0a090807060504030201").unwrap();
///
/// let mut phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::ConfirmedDataUp,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::MACPayload(MACPayload{
///         fhdr: FHDR{
///             devaddr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
///             f_ctrl: FCtrl::default(),
///             f_cnt: 0,
///             f_opts: MACCommandSet::new(vec![
///                 MACCommand::DevStatusAns(DevStatusAnsPayload{
///                     battery: 115,
///                     margin: 7,
///                 }),
///             ]),
///         },
///         f_port: Some(10),
///         frm_payload: Some(FRMPayload::Raw(vec![0x01, 0x02, 0x03, 0x04])),
///     }),
///     mic: None,
/// };
///
/// phy.encrypt_frm_payload(&app_s_key).unwrap();
/// phy.set_uplink_data_mic(MACVersion::LoRaWAN1_0, 0, 0, 0, &nwk_s_key, &nwk_s_key);
///
/// let bytes = phy.to_vec().unwrap();
/// assert_eq!(vec![0x80, 0x04, 0x03, 0x02, 0x01, 0x03, 0x00, 0x00, 0x06, 0x73, 0x07, 0x0a, 0xe2, 0x64, 0xd4, 0xf7, 0xe1, 0x17, 0xd2, 0xc0], bytes);
///
/// let mut phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// assert_eq!(true, phy_decoded.validate_uplink_data_mic(MACVersion::LoRaWAN1_0, 0, 0, 0, &nwk_s_key, &nwk_s_key).unwrap());
///
/// phy_decoded.decrypt_frm_payload(&app_s_key).unwrap();
///
/// if let Payload::MACPayload(pl) = &phy_decoded.payload {
///     if let FRMPayload::Raw(b) = &pl.frm_payload.as_ref().unwrap() {
///         assert_eq!(&vec![0x01, 0x02, 0x03, 0x04], b);
///     } else {
///         panic!("No FrmPayload!");
///     }
/// } else {
///     panic!("No MacPayload!");
/// }
/// ```
///
/// LoRaWAN 1.1.x downlink with encrypted f_opts example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// let s_nwk_s_int_key = AES128Key::from_str("01010101010101010101010101010100").unwrap();
/// let nwk_s_enc_key = AES128Key::from_str("01010101010101010101010101010200").unwrap();
/// let app_s_key = AES128Key::from_str("100f0e0d0c0b0a090807060504030201").unwrap();
///
/// let mut phy = PhyPayload {
///     mhdr: MHDR{
///         m_type: MType::UnconfirmedDataDown,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::MACPayload(MACPayload{
///         fhdr: FHDR{
///             devaddr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
///             f_ctrl: FCtrl::default(),
///             f_cnt: 0,
///             f_opts: MACCommandSet::new(vec![
///                 MACCommand::LinkCheckAns(LinkCheckAnsPayload{
///                     margin: 7,
///                     gw_cnt: 1,
///                 }),
///             ]),
///         },
///         f_port: Some(1),
///         frm_payload: Some(FRMPayload::Raw(vec![0x01, 0x02, 0x03, 0x04])),
///     }),
///     mic: None,
/// };
///
/// phy.encrypt_f_opts(&nwk_s_enc_key).unwrap();
/// phy.encrypt_frm_payload(&app_s_key).unwrap();
/// phy.set_downlink_data_mic(MACVersion::LoRaWAN1_1, 0, &s_nwk_s_int_key).unwrap();
///
/// let bytes = phy.to_vec().unwrap();
/// assert_eq!(vec![0x60, 0x04, 0x03, 0x02, 0x01, 0x03, 0x00, 0x00, 0x22, 0xac, 0x0a, 0x01, 0xf0, 0xb4, 0x68, 0xdd, 0xaa, 0x5e, 0xd1, 0x3a], bytes);
///
/// let mut phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// assert_eq!(true, phy_decoded.validate_downlink_data_mic(MACVersion::LoRaWAN1_1, 0, &s_nwk_s_int_key).unwrap());
///
/// phy_decoded.decrypt_f_opts(&nwk_s_enc_key).unwrap();
/// phy_decoded.decrypt_frm_payload(&app_s_key).unwrap();
///
/// if let Payload::MACPayload(pl) = &phy_decoded.payload {
///     assert_eq!(MACCommandSet::new(vec![
///         MACCommand::LinkCheckAns(LinkCheckAnsPayload{
///             margin: 7,
///             gw_cnt: 1,
///         }),
///     ]), pl.fhdr.f_opts);
///
///     if let FRMPayload::Raw(b) = &pl.frm_payload.as_ref().unwrap() {
///         assert_eq!(&vec![0x01, 0x02, 0x03, 0x04], b);
///     } else {
///         panic!("No FrmPayload!");
///     }
/// } else {
///     panic!("No MacPayload");
/// }
/// ```
///
/// Proprietary example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// let phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::Proprietary,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::Raw(vec![0x01, 0x02, 0x03]),
///     mic: None,
/// };
///
/// let bytes = phy.to_vec().unwrap();
/// assert_eq!(vec![0xe0, 0x01, 0x02, 0x03], bytes);
///
/// let phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// assert_eq!(phy, phy_decoded);
/// ```
///
/// LoRaWAN 1.0.x Relay ForwardUplinkReq example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// // Payload from the end-device.
/// let ed_app_key = AES128Key::from_str("01020304050607080102030405060708").unwrap();
/// let mut ed_phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinRequest,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinRequest(JoinRequestPayload {
///         join_eui: EUI64::from_str("0101010101010101").unwrap(),
///         dev_eui: EUI64::from_str("0202020202020202").unwrap(),
///         dev_nonce: 771,
///     }),
///     mic: None,
/// };
///
/// ed_phy.set_join_request_mic(&ed_app_key).unwrap();
///
/// // Relay ForwardUplinkReq (which will forward the end-device payload).
/// let relay_nwk_s_key = AES128Key::from_str("08070605040302010807060504030201").unwrap();
/// let mut relay_phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::UnconfirmedDataUp,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::MACPayload(MACPayload {
///         fhdr: FHDR {
///             devaddr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
///             f_cnt: 10,
///             ..Default::default()
///         },
///         f_port: Some(226),
///         frm_payload: Some(FRMPayload::ForwardUplinkReq(ForwardUplinkReq {
///             metadata: UplinkMetadata {
///                 dr: 5,
///                 snr: 7,
///                 rssi: -80,
///                 wor_channel: 1,
///             },
///             frequency: 868100000,
///             payload: Box::new(ed_phy.clone()),
///         })),
///     }),
///     mic: None,
/// };
/// relay_phy.encrypt_frm_payload(&relay_nwk_s_key).unwrap();
/// relay_phy.set_uplink_data_mic(MACVersion::LoRaWAN1_0, 0, 0, 0, &relay_nwk_s_key, &relay_nwk_s_key);
///
/// let bytes = relay_phy.to_vec().unwrap();
/// assert_eq!(vec![0x40, 0x04, 0x03, 0x02, 0x01, 0x00, 0x0a, 0x00, 0xe2, 0x2f, 0x68, 0xf4, 0xa5, 0x0a, 0xdf, 0xfb, 0x64, 0xef, 0x37, 0x91, 0x0f, 0x14, 0x6a, 0x6c, 0x2b, 0xda, 0x4f, 0x7e, 0x2d, 0xb9, 0x6a, 0xc8, 0x99, 0xa8, 0xa4, 0x72, 0x7d, 0x0a, 0xbd, 0xc9, 0xae, 0x51], bytes);
///
/// let mut relay_phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// assert_eq!(relay_phy, relay_phy_decoded);
///
/// relay_phy_decoded.decrypt_frm_payload(&relay_nwk_s_key).unwrap();
/// assert_eq!(PhyPayload{
///     mhdr: MHDR {
///         m_type: MType::UnconfirmedDataUp,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::MACPayload(MACPayload {
///         fhdr: FHDR {
///             devaddr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
///             f_cnt: 10,
///             ..Default::default()
///         },
///         f_port: Some(226),
///         frm_payload: Some(FRMPayload::ForwardUplinkReq(ForwardUplinkReq {
///             metadata: UplinkMetadata {
///                 dr: 5,
///                 snr: 7,
///                 rssi: -80,
///                 wor_channel: 1,
///             },
///             frequency: 868100000,
///             payload: Box::new(ed_phy),
///         })),
///     }),
///     mic: Some([0xbd, 0xc9, 0xae, 0x51]),
/// }, relay_phy_decoded);
/// ```
///
/// LoRaWAN 1.0.x Relay ForwardDownlinkReq example:
/// ```rust
/// use std::str::FromStr;
/// use lrwn::*;
///
/// // Payload for the end-device.
/// let ed_app_key = AES128Key::from_str("0102030405060708090a0b0c0d0e0f10").unwrap();
/// let ed_join_eui = EUI64::from_str("0807060504030201").unwrap();
/// let ed_dev_nonce = 258;
/// let mut ed_phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::JoinAccept,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::JoinAccept(JoinAcceptPayload {
///         join_nonce: 65793,
///         home_netid: NetID::from_str("020202").unwrap(),
///         devaddr: DevAddr::from_str("01020304").unwrap(),
///         dl_settings: DLSettings {
///             opt_neg: false,
///             rx2_dr: 0,
///             rx1_dr_offset: 0,
///         },
///         cflist: None,
///         rx_delay: 0,
///     }),
///     mic: None,
/// };
///
/// ed_phy.set_join_accept_mic(JoinType::Join, &ed_join_eui, ed_dev_nonce, &ed_app_key).unwrap();
/// ed_phy.encrypt_join_accept_payload(&ed_app_key).unwrap();
///
/// // Payload for the Relay containing the ForwardDownlinkReq.
/// let relay_nwk_s_key = AES128Key::from_str("08070605040302010807060504030201").unwrap();
/// let mut relay_phy = PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::UnconfirmedDataDown,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::MACPayload(MACPayload {
///         fhdr: FHDR {
///             devaddr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
///             f_cnt: 10,
///             ..Default::default()
///         },
///         f_port: Some(226),
///         frm_payload: Some(FRMPayload::ForwardDownlinkReq(ForwardDownlinkReq {
///             payload: Box::new(ed_phy.clone()),
///         })),
///     }),
///     mic: None,
/// };
/// relay_phy.encrypt_frm_payload(&relay_nwk_s_key).unwrap();
/// relay_phy.set_downlink_data_mic(MACVersion::LoRaWAN1_0, 0, &relay_nwk_s_key).unwrap();
///
/// let bytes = relay_phy.to_vec().unwrap();
/// assert_eq!(vec![0x60, 0x04, 0x03, 0x02, 0x01, 0x00, 0x0a, 0x00, 0xe2, 0xc9, 0x60, 0x41, 0x64, 0xc9, 0x7d, 0x76, 0xf9, 0xea, 0x8e, 0x1a, 0x79, 0x2b, 0xa0, 0x87, 0x9b, 0x85, 0x24, 0x3e, 0x5a, 0xf5], bytes);
///
/// let mut relay_phy_decoded = PhyPayload::from_slice(&bytes).unwrap();
/// assert_eq!(relay_phy, relay_phy_decoded);
///
/// relay_phy_decoded.decrypt_frm_payload(&relay_nwk_s_key).unwrap();
/// assert_eq!(PhyPayload {
///     mhdr: MHDR {
///         m_type: MType::UnconfirmedDataDown,
///         major: Major::LoRaWANR1,
///     },
///     payload: Payload::MACPayload(MACPayload {
///         fhdr: FHDR {
///             devaddr: DevAddr::from_be_bytes([0x01, 0x02, 0x03, 0x04]),
///             f_cnt: 10,
///             ..Default::default()
///         },
///         f_port: Some(226),
///         frm_payload: Some(FRMPayload::ForwardDownlinkReq(ForwardDownlinkReq {
///             payload: Box::new(ed_phy),
///         })),
///     }),
///     mic: Some([0x24, 0x3e, 0x5a, 0xf5]),
/// }, relay_phy_decoded);
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct PhyPayload {
    pub mhdr: MHDR,
    pub payload: Payload,
    /// This field is not used in case of the proprietary message-type.
    pub mic: Option<[u8; 4]>,
}

impl PhyPayload {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut b = Vec::new();

        b.extend_from_slice(&self.mhdr.to_le_bytes());
        b.append(&mut self.payload.to_vec()?);

        if let Some(v) = &self.mic {
            b.extend_from_slice(&v.clone());
        }

        Ok(b)
    }

    pub fn from_slice(b: &[u8]) -> Result<Self> {
        let b_len = b.len();

        // We need 1 byte to decode the mhdr.
        if b_len == 0 {
            return Err(anyhow!("at least 1 byte required to decode PhyPayload"));
        }

        let mhdr = {
            let mhdr: [u8; 1] = [b[0]];
            MHDR::from_le_bytes(mhdr)?
        };

        if mhdr.m_type == MType::Proprietary {
            return Ok(PhyPayload {
                mhdr,
                payload: Payload::from_slice(MType::Proprietary, &b[1..])?,
                mic: None,
            });
        }

        // Validate the minimum required bytes for not running into slicing errors.
        if b_len < 5 {
            return Err(anyhow!(
                "at least 5 bytes are required to decode PhyPayload"
            ));
        }

        let m_type = mhdr.m_type;
        let mut mic: [u8; 4] = [0; 4];
        mic.clone_from_slice(&b[b_len - 4..]);

        Ok(PhyPayload {
            mhdr,
            payload: Payload::from_slice(m_type, &b[1..b_len - 4])?,
            mic: Some(mic),
        })
    }

    /// Calculate and set the MIC field for uplink data frames.
    /// The conf_f_cnt, tx_dr, tx_ch and s_nwk_s_int_key are only required for LoRaWAN 1.1 and can
    /// be left blank for LoRaWAN 1.0.
    #[cfg(feature = "crypto")]
    pub fn set_uplink_data_mic(
        &mut self,
        mac_version: MACVersion,
        conf_f_cnt: u32,
        tx_dr: u8,
        tx_ch: u8,
        f_nwk_s_int_key: &AES128Key,
        s_nwk_s_int_key: &AES128Key,
    ) -> Result<()> {
        self.mic = Some(self.calculate_uplink_data_mic(
            mac_version,
            conf_f_cnt,
            tx_dr,
            tx_ch,
            f_nwk_s_int_key,
            s_nwk_s_int_key,
        )?);
        Ok(())
    }

    /// Validate the MIC of an uplink data frame.
    /// In order to validate the MIC, the f_cnt value must be first set to the full 32 bit
    /// frame-counter value, as only the 16 lsb are transmitted over the air.
    /// The conf_f_cnt, tx_dr, tx_ch and s_nwk_s_int_key are only required for LoRaWAN 1.1 and can
    /// be left blank for LoRaWAN 1.0.
    #[cfg(feature = "crypto")]
    pub fn validate_uplink_data_mic(
        &self,
        mac_version: MACVersion,
        conf_f_cnt: u32,
        tx_dr: u8,
        tx_ch: u8,
        f_nwk_s_int_key: &AES128Key,
        s_nwk_s_int_key: &AES128Key,
    ) -> Result<bool> {
        if let Some(mic) = self.mic {
            return Ok(mic
                == self.calculate_uplink_data_mic(
                    mac_version,
                    conf_f_cnt,
                    tx_dr,
                    tx_ch,
                    f_nwk_s_int_key,
                    s_nwk_s_int_key,
                )?);
        }

        Ok(false)
    }

    /// Set the MIC for downlink data frames.
    /// The conf_f_cnt is only required for LoRaWAN 1.1 and can be left blank for LoRaWAN 1.0.
    #[cfg(feature = "crypto")]
    pub fn set_downlink_data_mic(
        &mut self,
        mac_version: MACVersion,
        conf_f_cnt: u32,
        s_nwk_s_int_key: &AES128Key,
    ) -> Result<()> {
        self.mic =
            Some(self.calculate_downlink_data_mic(mac_version, conf_f_cnt, s_nwk_s_int_key)?);
        Ok(())
    }

    #[cfg(feature = "crypto")]
    pub fn validate_downlink_data_mic(
        &mut self,
        mac_version: MACVersion,
        conf_f_cnt: u32,
        s_nwk_s_int_key: &AES128Key,
    ) -> Result<bool> {
        if let Some(mic) = self.mic {
            return Ok(mic
                == self.calculate_downlink_data_mic(mac_version, conf_f_cnt, s_nwk_s_int_key)?);
        }

        Ok(false)
    }

    /// Validate the cmacF part of the uplink data MIC (LoRaWAN 1.1 only).
    /// In order to validate the MIC, the f_cnt value must be first set to the full 32 bit
    /// frame-counter value, as only the 16 lsb are transmitted over the air.
    #[cfg(feature = "crypto")]
    pub fn validate_uplink_data_micf(&self, f_nwk_s_int_key: &AES128Key) -> Result<bool> {
        // We are only interested in mic[2:] (cmacF bytes), therefore there is no
        // need to pass the correct confFCnt, txDR, txCh and sNwkSIntKey parameters.
        if let Some(v) = self.mic {
            let mic = self.calculate_uplink_data_mic(
                MACVersion::LoRaWAN1_1,
                0,
                0,
                0,
                f_nwk_s_int_key,
                f_nwk_s_int_key,
            )?;

            return Ok(v[2..] == mic[2..]);
        }

        Ok(false)
    }

    /// Set the join-request MIC.
    #[cfg(feature = "crypto")]
    pub fn set_join_request_mic(&mut self, key: &AES128Key) -> Result<()> {
        self.mic = Some(self.calculate_upink_join_mic(key)?);
        Ok(())
    }

    /// Validate the join-request MIC.
    #[cfg(feature = "crypto")]
    pub fn validate_join_request_mic(&self, key: &AES128Key) -> Result<bool> {
        if let Some(v) = self.mic {
            let mic = self.calculate_upink_join_mic(key)?;
            return Ok(v == mic);
        }

        Ok(false)
    }

    /// Set the the downlink join-accept MIC.
    #[cfg(feature = "crypto")]
    pub fn set_join_accept_mic(
        &mut self,
        join_req_type: JoinType,
        join_eui: &EUI64,
        dev_nonce: u16,
        key: &AES128Key,
    ) -> Result<()> {
        self.mic =
            Some(self.calculate_downlink_join_mic(join_req_type, join_eui, dev_nonce, key)?);
        Ok(())
    }

    /// Validate the downlink join-accept MIC.
    #[cfg(feature = "crypto")]
    pub fn validate_join_accept_mic(
        &self,
        join_req_type: JoinType,
        join_eui: &EUI64,
        dev_nonce: u16,
        key: &AES128Key,
    ) -> Result<bool> {
        if let Some(v) = self.mic {
            let mic = self.calculate_downlink_join_mic(join_req_type, join_eui, dev_nonce, key)?;
            return Ok(v == mic);
        }

        Ok(false)
    }

    /// Encrypt the join-accept payload with the given key.
    /// Note that the encryption must be performed after setting the MIC, since the MIC is part of
    /// the encrypted payload.
    /// For encrypting a join-request response, use the nwk_key, for rejoin-request 0, 1 and 3
    /// response, use the js_enc_key.
    #[cfg(feature = "crypto")]
    pub fn encrypt_join_accept_payload(&mut self, key: &AES128Key) -> Result<()> {
        use aes::cipher::KeyInit;

        if self.mic.is_none() {
            return Err(anyhow!("mic must be set first"));
        }

        if let Payload::JoinAccept(pl) = &self.payload {
            let mut pt = pl.to_vec()?;
            pt.extend_from_slice(&self.mic.unwrap());

            if pt.len() % 16 != 0 {
                return Err(anyhow!("plaintext must be a multiple of 16 bytes"));
            }

            let key_bytes = key.to_bytes();
            let key = GenericArray::from_slice(&key_bytes);
            let cipher = Aes128::new(key);

            let mut ct = Vec::new();

            for i in 0..(pt.len() / 16) {
                let index = i * 16;

                let mut block = Block::clone_from_slice(&pt[index..index + 16]);
                cipher.decrypt_block(&mut block);
                ct.extend_from_slice(block.as_slice());
            }

            self.payload = Payload::Raw(ct[0..ct.len() - 4].to_vec());
            let mut mic: [u8; 4] = [0; 4];
            mic.clone_from_slice(&ct[ct.len() - 4..]);
            self.mic = Some(mic);
            return Ok(());
        }

        Err(anyhow!("payload must be of type JoinAcceptPayload"))
    }

    /// Decrypt the join-accept payload with the given key.
    /// Note that the decryption must be performed before validating the MIC, since the MIC is part
    /// of the encrypted payload.
    /// For decrypting a join-request response, use the nwk_key, for rejoin-request 0, 1 and 3
    /// response, use the js_enc_key.
    #[cfg(feature = "crypto")]
    pub fn decrypt_join_accept_payload(&mut self, key: &AES128Key) -> Result<()> {
        use aes::cipher::KeyInit;

        if self.mic.is_none() {
            return Err(anyhow!("mic must be set first"));
        }

        if let Payload::Raw(pl) = &self.payload {
            // append MIC since it is encrypted too
            let mut ct = pl.clone();
            ct.extend_from_slice(&self.mic.unwrap());

            if ct.len() % 16 != 0 {
                return Err(anyhow!("ciphertext must be a multiple of 16 bytes"));
            }

            let key_bytes = key.to_bytes();
            let key = GenericArray::from_slice(&key_bytes);
            let cipher = Aes128::new(key);

            let mut pt = Vec::new();

            for i in 0..(ct.len() / 16) {
                let index = i * 16;

                let mut block = Block::clone_from_slice(&ct[index..index + 16]);
                cipher.encrypt_block(&mut block);
                pt.extend_from_slice(block.as_slice());
            }

            let mut mic: [u8; 4] = [0; 4];
            mic.clone_from_slice(&pt[pt.len() - 4..]);
            self.mic = Some(mic);
            self.payload = Payload::JoinAccept(JoinAcceptPayload::from_slice(&pt[..pt.len() - 4])?);

            return Ok(());
        }

        Err(anyhow!("payload must be of type Raw"))
    }

    /// Encrypt the f_opts with the given key.
    #[cfg(feature = "crypto")]
    pub fn encrypt_f_opts(&mut self, nwk_s_enc_key: &AES128Key) -> Result<()> {
        if let Payload::MACPayload(pl) = &mut self.payload {
            let f_opts_bytes = pl.fhdr.f_opts.to_vec()?;
            if f_opts_bytes.is_empty() {
                return Ok(());
            }

            let uplink = is_uplink(self.mhdr.m_type);

            // a_fcnt_down is used on downlink when f_port > 0
            let a_fcnt_down = !uplink && pl.f_port.is_some() && pl.f_port.unwrap() > 0;

            let f_opts_enc = encrypt_f_opts(
                nwk_s_enc_key,
                a_fcnt_down,
                uplink,
                &pl.fhdr.devaddr,
                pl.fhdr.f_cnt,
                &f_opts_bytes,
            )?;

            pl.fhdr.f_opts = MACCommandSet::new(vec![MACCommand::Raw(f_opts_enc)]);
            return Ok(());
        }

        Err(anyhow!("payload must be of type MACPayload"))
    }

    /// Decrypt the f_opts with the given key.
    /// This automatically calls decode_f_opts_to_mac_commands.
    #[cfg(feature = "crypto")]
    pub fn decrypt_f_opts(&mut self, nwk_s_enc_key: &AES128Key) -> Result<()> {
        self.encrypt_f_opts(nwk_s_enc_key)?;
        self.decode_f_opts_to_mac_commands()?;

        Ok(())
    }

    /// Decode f_opts to mac-commands.
    pub fn decode_f_opts_to_mac_commands(&mut self) -> Result<()> {
        if let Payload::MACPayload(pl) = &mut self.payload {
            let uplink = is_uplink(self.mhdr.m_type);
            pl.fhdr.f_opts.decode_from_raw(uplink)?;
        }
        Ok(())
    }

    /// Decode frm_payload payload.
    ///
    /// This will decode as follow based on f_port:
    /// 0:   MACCommandSet
    /// 226: ForwardDownlinkReq / ForwardDownlinkReq
    ///
    /// For other f_port values, it will not try to decode the payload.
    /// Note that this requires a decrypted frm_payload.
    pub fn decode_frm_payload(&mut self) -> Result<()> {
        if let Payload::MACPayload(pl) = &mut self.payload {
            let uplink = is_uplink(self.mhdr.m_type);
            let f_port = pl.f_port.unwrap_or(0);
            let b = match &pl.frm_payload {
                Some(FRMPayload::Raw(v)) => v.clone(),
                _ => {
                    // Nothing to do.
                    return Ok(());
                }
            };

            return decode_frm_payload(pl, uplink, f_port, b);
        }

        Ok(())
    }

    /// Encrypt the frm_payload with the given key.
    #[cfg(feature = "crypto")]
    pub fn encrypt_frm_payload(&mut self, key: &AES128Key) -> Result<()> {
        if let Payload::MACPayload(pl) = &mut self.payload {
            // nothing to do
            if pl.frm_payload.is_none() {
                return Ok(());
            }

            let uplink = is_uplink(self.mhdr.m_type);
            let data = pl.frm_payload.as_ref().unwrap().to_vec()?;
            let data = encrypt_frm_payload(key, uplink, &pl.fhdr.devaddr, pl.fhdr.f_cnt, &data)?;

            pl.frm_payload = Some(FRMPayload::Raw(data));
            return Ok(());
        }

        Err(anyhow!("payload must be of type MACPayload"))
    }

    /// Decrypt the frm_payload with the given key.
    ///
    /// This will automatically call decode_frm_payload.
    #[cfg(feature = "crypto")]
    pub fn decrypt_frm_payload(&mut self, key: &AES128Key) -> Result<()> {
        if let Payload::MACPayload(pl) = &mut self.payload {
            // nothing to do
            if pl.frm_payload.is_none() {
                return Ok(());
            }

            let uplink = is_uplink(self.mhdr.m_type);
            let data = pl.frm_payload.as_ref().unwrap().to_vec()?;
            let data = encrypt_frm_payload(key, uplink, &pl.fhdr.devaddr, pl.fhdr.f_cnt, &data)?;

            return decode_frm_payload(pl, uplink, pl.f_port.unwrap_or(0), data);
        }

        Err(anyhow!("payload must be of type MACPayload"))
    }

    #[cfg(feature = "crypto")]
    fn calculate_uplink_data_mic(
        &self,
        mac_version: MACVersion,
        conf_f_cnt: u32,
        tx_dr: u8,
        tx_ch: u8,
        f_nwk_s_int_key: &AES128Key,
        s_nwk_s_int_key: &AES128Key,
    ) -> Result<[u8; 4]> {
        if let Payload::MACPayload(pl) = &self.payload {
            // set to 0 if the uplink does not contain an ACK
            let mut conf_f_cnt = conf_f_cnt;
            if !pl.fhdr.f_ctrl.ack {
                conf_f_cnt = 0;
            }

            // truncate to 16 lsb
            let conf_f_cnt = (conf_f_cnt % (1 << 16)) as u16;

            let mut mic_bytes = Vec::new();
            mic_bytes.extend_from_slice(&self.mhdr.to_le_bytes());
            mic_bytes.extend_from_slice(&self.payload.to_vec()?);

            let mut b0: [u8; 16] = [0; 16];
            let mut b1: [u8; 16] = [0; 16];

            b0[0] = 0x49;
            b1[0] = 0x49;

            // devaddr
            let devaddr_b = pl.fhdr.devaddr.to_le_bytes();
            b0[6..10].clone_from_slice(&devaddr_b);
            b1[6..10].clone_from_slice(&devaddr_b);

            // fcntup
            b0[10..14].clone_from_slice(&pl.fhdr.f_cnt.to_le_bytes());
            b1[10..14].clone_from_slice(&pl.fhdr.f_cnt.to_le_bytes());

            // msg len
            b0[15] = mic_bytes.len() as u8;
            b1[15] = mic_bytes.len() as u8;

            // remaining b1 fields
            b1[1..3].clone_from_slice(&conf_f_cnt.to_le_bytes());
            b1[3] = tx_dr;
            b1[4] = tx_ch;

            let mut mac = Cmac::<Aes128>::new_from_slice(&s_nwk_s_int_key.to_bytes()).unwrap();
            mac.update(&b1);
            mac.update(&mic_bytes);

            let cmac_s = mac.finalize().into_bytes();
            if cmac_s.len() < 4 {
                return Err(anyhow!("cmac_s is less than 4 bytes"));
            }

            let mut mac = Cmac::<Aes128>::new_from_slice(&f_nwk_s_int_key.to_bytes()).unwrap();
            mac.update(&b0);
            mac.update(&mic_bytes);

            let cmac_f = mac.finalize().into_bytes();
            if cmac_f.len() < 4 {
                return Err(anyhow!("cmac_f is less than 4 bytes"));
            }

            let mut mic: [u8; 4] = [0; 4];
            if mac_version == MACVersion::LoRaWAN1_0 {
                mic.clone_from_slice(&cmac_f[0..4]);
                return Ok(mic);
            } else {
                mic[0..2].clone_from_slice(&cmac_s[0..2]);
                mic[2..4].clone_from_slice(&cmac_f[0..2]);
                return Ok(mic);
            }
        }

        Err(anyhow!("payload must be of type MACPayload"))
    }

    #[cfg(feature = "crypto")]
    fn calculate_downlink_data_mic(
        &self,
        mac_version: MACVersion,
        conf_f_cnt: u32,
        s_nwk_s_int_key: &AES128Key,
    ) -> Result<[u8; 4]> {
        if let Payload::MACPayload(pl) = &self.payload {
            // set to 0 if the downlink does not contain an ack or in case of LoRaWAN 1.0
            let mut conf_f_cnt = conf_f_cnt;
            if mac_version == MACVersion::LoRaWAN1_0 || !pl.fhdr.f_ctrl.ack {
                conf_f_cnt = 0;
            }

            // truncate to 16 lsb
            let conf_f_cnt = (conf_f_cnt % (1 << 16)) as u16;

            // mic bytes
            let mut mic_bytes = Vec::new();
            mic_bytes.extend_from_slice(&self.mhdr.to_le_bytes());
            mic_bytes.extend_from_slice(&self.payload.to_vec()?);

            // b0
            let mut b0: [u8; 16] = [0; 16];
            b0[0] = 0x49;
            b0[1..3].clone_from_slice(&conf_f_cnt.to_le_bytes());
            b0[5] = 0x01;
            b0[6..10].clone_from_slice(&pl.fhdr.devaddr.to_le_bytes());
            b0[10..14].clone_from_slice(&pl.fhdr.f_cnt.to_le_bytes());
            b0[15] = mic_bytes.len() as u8;

            let mut mac = Cmac::<Aes128>::new_from_slice(&s_nwk_s_int_key.to_bytes()).unwrap();
            mac.update(&b0);
            mac.update(&mic_bytes);

            let hash = mac.finalize().into_bytes();
            if hash.len() < 4 {
                return Err(anyhow!("hash is less than 4 bytes"));
            }

            let mut mic: [u8; 4] = [0; 4];
            mic.clone_from_slice(&hash[0..4]);
            return Ok(mic);
        }

        Err(anyhow!("payload must be of type MACPayload"))
    }

    #[cfg(feature = "crypto")]
    fn calculate_upink_join_mic(&self, key: &AES128Key) -> Result<[u8; 4]> {
        // mic bytes
        let mut mic_bytes = Vec::new();

        mic_bytes.extend_from_slice(&self.mhdr.to_le_bytes());
        mic_bytes.extend_from_slice(&self.payload.to_vec()?);

        let mut mac = Cmac::<Aes128>::new_from_slice(&key.to_bytes()).unwrap();
        mac.update(&mic_bytes);

        let hash = mac.finalize().into_bytes();
        if hash.len() < 4 {
            return Err(anyhow!("hash is less than 4 bytes"));
        }

        let mut mic: [u8; 4] = [0; 4];
        mic.clone_from_slice(&hash[0..4]);
        Ok(mic)
    }

    #[cfg(feature = "crypto")]
    fn calculate_downlink_join_mic(
        &self,
        join_req_type: JoinType,
        join_eui: &EUI64,
        dev_nonce: u16,
        key: &AES128Key,
    ) -> Result<[u8; 4]> {
        if let Payload::JoinAccept(pl) = &self.payload {
            let mut mic_bytes = Vec::new();

            // LoRaWAN 1.1
            if pl.dl_settings.opt_neg {
                mic_bytes.push(match join_req_type {
                    JoinType::Join => 0xff,
                    JoinType::RejoinType0 => 0x00,
                    JoinType::RejoinType1 => 0x01,
                    JoinType::RejoinType2 => 0x02,
                });

                mic_bytes.extend_from_slice(&join_eui.to_le_bytes());
                mic_bytes.extend_from_slice(&dev_nonce.to_le_bytes());
            }

            mic_bytes.extend_from_slice(&self.mhdr.to_le_bytes());

            // JoinNonce | NetID | DevAddr | DLSettings | RxDelay | CFList
            mic_bytes.extend_from_slice(&pl.to_vec()?);

            let mut mac = Cmac::<Aes128>::new_from_slice(&key.to_bytes()).unwrap();
            mac.update(&mic_bytes);

            let hash = mac.finalize().into_bytes();
            if hash.len() < 4 {
                return Err(anyhow!("hash is less than 4 bytes"));
            }

            let mut mic: [u8; 4] = [0; 4];
            mic.clone_from_slice(&hash[0..4]);
            return Ok(mic);
        }

        Err(anyhow!("payload must be of type JoinAcceptPayload"))
    }
}

/// Encrypt f_opts mac-command data.
/// For uplink:
///   Set the a_fcnt_down to false and use the f_cnt_up as f_cnt.
/// For downlink if f_port is unset or equal to 0:
///   Set the a_fcnt_down to false and use the n_fcnt_down as f_cnt.
/// For downlink if f_port > 0:
///   Set the a_fcnt_down to true and use the a_f_cnt_down as f_cnt.
#[cfg(feature = "crypto")]
pub fn encrypt_f_opts(
    nwk_s_enc_key: &AES128Key,
    a_fcnt_down: bool,
    uplink: bool,
    devaddr: &DevAddr,
    f_cnt: u32,
    data: &[u8],
) -> Result<Vec<u8>> {
    use aes::cipher::KeyInit;

    if data.len() > 15 {
        return Err(anyhow!("max size of f_opts is 15 bytes"));
    }

    let key_bytes = nwk_s_enc_key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut a = vec![0; 16];
    a[0] = 0x01;
    if a_fcnt_down {
        a[4] = 0x02;
    } else {
        a[4] = 0x01;
    }

    if !uplink {
        a[5] = 0x01;
    }

    a[6..10].clone_from_slice(&devaddr.to_le_bytes());
    a[10..14].clone_from_slice(&f_cnt.to_le_bytes());
    a[15] = 0x01;

    let block = Block::from_mut_slice(&mut a);
    cipher.encrypt_block(block);

    let mut out = vec![0; data.len()];
    for i in 0..data.len() {
        out[i] = data[i] ^ block[i];
    }

    Ok(out)
}

/// Encrypt (and decrypt) the frm_payload.
/// Note that the same function is used for encryption and decryption.
#[cfg(feature = "crypto")]
pub fn encrypt_frm_payload(
    key: &AES128Key,
    uplink: bool,
    devaddr: &DevAddr,
    f_cnt: u32,
    data: &[u8],
) -> Result<Vec<u8>> {
    use aes::cipher::KeyInit;

    let mut data = data.to_vec();
    let data_len = data.len();

    // make pt length multiple of 16
    if data.len() % 16 != 0 {
        data.append(&mut vec![0; 16 - (data.len() % 16)]);
    }

    let key_bytes = key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut a = vec![0; 16];
    a[0] = 0x01;
    if !uplink {
        a[5] = 0x01;
    }

    a[6..10].clone_from_slice(&devaddr.to_le_bytes());
    a[10..14].clone_from_slice(&f_cnt.to_le_bytes());

    for i in 0..(data.len() / 16) {
        a[15] = (i + 1) as u8;

        let mut block = Block::clone_from_slice(&a);
        cipher.encrypt_block(&mut block);

        for j in 0..16 {
            data[(i * 16) + j] ^= block[j];
        }
    }

    Ok(data[0..data_len].to_vec())
}

fn is_uplink(m_type: MType) -> bool {
    match m_type {
        MType::JoinRequest
        | MType::UnconfirmedDataUp
        | MType::ConfirmedDataUp
        | MType::RejoinRequest => true,
        MType::JoinAccept | MType::UnconfirmedDataDown | MType::ConfirmedDataDown => false,
        MType::Proprietary => false,
    }
}

fn decode_frm_payload(pl: &mut MACPayload, uplink: bool, f_port: u8, b: Vec<u8>) -> Result<()> {
    if f_port == 0 {
        let mut macs = MACCommandSet::new(vec![MACCommand::Raw(b)]);
        macs.decode_from_raw(uplink)?;
        pl.frm_payload = Some(FRMPayload::MACCommandSet(macs));
    } else if f_port == LA_FPORT_RELAY && uplink {
        pl.frm_payload = Some(FRMPayload::ForwardUplinkReq(ForwardUplinkReq::from_slice(
            &b,
        )?));
    } else if f_port == LA_FPORT_RELAY && !uplink {
        pl.frm_payload = Some(FRMPayload::ForwardDownlinkReq(
            ForwardDownlinkReq::from_slice(&b)?,
        ));
    } else {
        pl.frm_payload = Some(FRMPayload::Raw(b));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::super::eui64::EUI64;
    use super::super::mhdr::Major;
    use super::super::payload::JoinRequestPayload;
    use super::*;

    struct PhyPayloadTest {
        phy: PhyPayload,
        bytes: Vec<u8>,
    }

    #[test]
    fn test_proprietary() {
        let tests = vec![
            PhyPayloadTest {
                phy: PhyPayload {
                    mhdr: MHDR {
                        m_type: MType::Proprietary,
                        major: Major::LoRaWANR1,
                    },
                    payload: Payload::Raw(vec![]),
                    mic: None,
                },
                bytes: vec![0xe0],
            },
            PhyPayloadTest {
                phy: PhyPayload {
                    mhdr: MHDR {
                        m_type: MType::Proprietary,
                        major: Major::LoRaWANR1,
                    },
                    payload: Payload::Raw(vec![0x01, 0x02, 0x03]),
                    mic: None,
                },
                bytes: vec![0xe0, 0x01, 0x02, 0x03],
            },
        ];

        for tst in tests {
            assert_eq!(tst.bytes, tst.phy.to_vec().unwrap());
            assert_eq!(tst.phy, PhyPayload::from_slice(&tst.bytes).unwrap());
        }
    }

    #[test]
    // No need to test all the different mtypes, this is handled by the Payload type.
    fn test_non_proprietary() {
        let tests = vec![PhyPayloadTest {
            phy: PhyPayload {
                mhdr: MHDR {
                    m_type: MType::JoinRequest,
                    major: Major::LoRaWANR1,
                },
                payload: Payload::JoinRequest(JoinRequestPayload {
                    join_eui: EUI64::from_str("0102030405060708").unwrap(),
                    dev_eui: EUI64::from_str("0807060504030201").unwrap(),
                    dev_nonce: 1024,
                }),
                mic: Some([0x01, 0x02, 0x03, 0x04]),
            },
            bytes: vec![
                0x00, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05,
                0x06, 0x07, 0x08, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04,
            ],
        }];

        for tst in tests {
            assert_eq!(tst.bytes, tst.phy.to_vec().unwrap());
            assert_eq!(tst.phy, PhyPayload::from_slice(&tst.bytes).unwrap());
        }
    }
}
