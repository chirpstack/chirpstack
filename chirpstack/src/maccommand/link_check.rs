use anyhow::Result;
use tracing::{info, warn};

use crate::config;
use crate::storage::device;
use crate::uplink::UplinkFrameSet;
use chirpstack_api::gw;

pub fn handle(
    ufs: &UplinkFrameSet,
    dev: &device::Device,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let _ = (**block)
        .first()
        .ok_or_else(|| anyhow!("Expected LinkCheckReq"));

    info!(dev_eui = %dev.dev_eui, "Received LinkCheckReq");

    let mod_info = ufs
        .tx_info
        .modulation
        .as_ref()
        .ok_or_else(|| anyhow!("modulation can not be None"))?;
    let mod_params = mod_info
        .parameters
        .as_ref()
        .ok_or_else(|| anyhow!("parameters can not be None"))?;

    if let gw::modulation::Parameters::Lora(pl) = mod_params {
        let required_snr = config::get_required_snr_for_sf(pl.spreading_factor as u8)?;
        let mut max_snr: f32 = 0.0;

        for (i, rx_info) in ufs.rx_info_set.iter().enumerate() {
            if i == 0 || rx_info.snr > max_snr {
                max_snr = rx_info.snr;
            }
        }

        let mut margin = max_snr - required_snr;
        if margin < 0.0 {
            margin = 0.0;
        }

        return Ok(Some(lrwn::MACCommandSet::new(vec![
            lrwn::MACCommand::LinkCheckAns(lrwn::LinkCheckAnsPayload {
                margin: margin as u8,
                gw_cnt: ufs.rx_info_set.len() as u8,
            }),
        ])));
    }

    warn!("Unsupported modulation for LinkCheckReq");
    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_handle() {
        let ufs = UplinkFrameSet {
            uplink_set_id: Uuid::new_v4(),
            dr: 0,
            ch: 0,
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: Default::default(),
                    f_port: None,
                    frm_payload: None,
                }),
                mic: None,
            },
            tx_info: gw::UplinkTxInfo {
                modulation: Some(gw::Modulation {
                    parameters: Some(gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                        spreading_factor: 10,
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            },
            rx_info_set: vec![
                gw::UplinkRxInfo {
                    snr: -2.0,
                    ..Default::default()
                },
                gw::UplinkRxInfo {
                    snr: 2.0,
                    ..Default::default()
                },
            ],
            gateway_private_up_map: HashMap::new(),
            gateway_private_down_map: HashMap::new(),
            gateway_tenant_id_map: HashMap::new(),
            region_common_name: lrwn::region::CommonName::EU868,
            region_config_id: "eu868".into(),
            roaming_meta_data: None,
        };

        let dev: device::Device = Default::default();
        let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkCheckReq]);

        let resp = handle(&ufs, &dev, &block).unwrap();

        assert_eq!(
            Some(lrwn::MACCommandSet::new(vec![
                lrwn::MACCommand::LinkCheckAns(lrwn::LinkCheckAnsPayload {
                    margin: 17,
                    gw_cnt: 2,
                })
            ])),
            resp
        );
    }
}
