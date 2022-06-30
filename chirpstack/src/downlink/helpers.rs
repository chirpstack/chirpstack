use anyhow::Result;
use rand::seq::SliceRandom;

use chirpstack_api::gw;
use lrwn::region::DataRateModulation;

use crate::config;
use crate::region;

// Returns the gateway to use for downlink.
// In the current implementation it will sort the given slice based on SNR / RSSI,
// and return:
//  * A random item from the elements with an SNR > minSNR
//  * The first item of the sorted slice (failing the above)
pub fn select_downlink_gateway(
    region_name: &str,
    min_snr_margin: f32,
    rx_info: &mut chirpstack_api::internal::DeviceGatewayRxInfo,
) -> Result<chirpstack_api::internal::DeviceGatewayRxInfoItem> {
    if rx_info.items.is_empty() {
        return Err(anyhow!("rx_info.items can not be empty"));
    }

    let region_conf = region::get(region_name)?;

    let dr = region_conf.get_data_rate(rx_info.dr as u8)?;
    let mut required_snr: Option<f32> = None;
    if let DataRateModulation::Lora(dr) = dr {
        required_snr = Some(config::get_required_snr_for_sf(dr.spreading_factor)?);
    }

    // sort items by SNR or if SNR is equal between A and B, by RSSI.
    rx_info.items.sort_by(|a, b| {
        if a.lora_snr == b.lora_snr {
            return b.rssi.partial_cmp(&a.rssi).unwrap();
        }
        b.lora_snr.partial_cmp(&a.lora_snr).unwrap()
    });

    let mut new_items = Vec::new();
    for item in &rx_info.items {
        if let Some(required_snr) = required_snr {
            if item.lora_snr - required_snr >= min_snr_margin {
                new_items.push(item.clone());
            }
        }
    }

    // Return a random item from the new_items slice (filtered by min_snr_margin).
    // If new_items is empty, then choose will return None and we return the first item from
    // rx_info.item.
    Ok(match new_items.choose(&mut rand::thread_rng()) {
        Some(v) => v.clone(),
        None => rx_info.items[0].clone(),
    })
}

pub fn set_tx_info_data_rate(
    tx_info: &mut chirpstack_api::gw::DownlinkTxInfo,
    dr: &DataRateModulation,
) -> Result<()> {
    match dr {
        DataRateModulation::Lora(v) => {
            tx_info.modulation = Some(gw::Modulation {
                parameters: Some(gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                    bandwidth: v.bandwidth,
                    spreading_factor: v.spreading_factor as u32,
                    code_rate: chirpstack_api::gw::CodeRate::Cr45.into(),
                    polarization_inversion: true,
                    code_rate_legacy: "".into(),
                })),
            });
        }
        DataRateModulation::Fsk(v) => {
            tx_info.modulation = Some(gw::Modulation {
                parameters: Some(gw::modulation::Parameters::Fsk(gw::FskModulationInfo {
                    datarate: v.bitrate,
                    frequency_deviation: v.bitrate / 2, // see: https://github.com/brocaar/chirpstack-gateway-bridge/issues/16
                })),
            });
        }
        DataRateModulation::LrFhss(_) => {
            return Err(anyhow!("LR-FHSS is not supported for downlink"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::test;

    struct Test {
        min_snr_margin: f32,
        rx_info: chirpstack_api::internal::DeviceGatewayRxInfo,
        expected_gws: Vec<Vec<u8>>,
    }

    #[tokio::test]
    async fn test_select_downlink_gateway() {
        let _guard = test::prepare().await;

        let tests = vec![
            // single item
            Test {
                min_snr_margin: 0.0,
                rx_info: chirpstack_api::internal::DeviceGatewayRxInfo {
                    dr: 0,
                    items: vec![chirpstack_api::internal::DeviceGatewayRxInfoItem {
                        lora_snr: -5.0,
                        gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]],
            },
            // two items, below min snr
            Test {
                min_snr_margin: 5.0,
                rx_info: chirpstack_api::internal::DeviceGatewayRxInfo {
                    dr: 2, // -15 is required
                    items: vec![
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -11.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
            // two items, one below min snr
            Test {
                min_snr_margin: 5.0,
                rx_info: chirpstack_api::internal::DeviceGatewayRxInfo {
                    dr: 2, // -15 is required
                    items: vec![
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -10.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
            // four items, two below min snr
            Test {
                min_snr_margin: 5.0,
                rx_info: chirpstack_api::internal::DeviceGatewayRxInfo {
                    dr: 2, // -15 is required
                    items: vec![
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -11.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -10.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
                            ..Default::default()
                        },
                        chirpstack_api::internal::DeviceGatewayRxInfoItem {
                            lora_snr: -9.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                expected_gws: vec![
                    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
                    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04],
                ],
            },
        ];

        for test in &tests {
            let mut rx_info = test.rx_info.clone();
            let mut gw_map = HashMap::new();

            let mut expected_gws = HashMap::new();
            for gw_id in &test.expected_gws {
                expected_gws.insert(gw_id.clone(), ());
            }

            for _ in 0..100 {
                let out =
                    select_downlink_gateway(&"eu868", test.min_snr_margin, &mut rx_info).unwrap();
                gw_map.insert(out.gateway_id, ());
            }

            assert_eq!(test.expected_gws.len(), gw_map.len());
            assert_eq!(
                true,
                expected_gws.keys().all(|k| gw_map.contains_key(k)),
                "Expected: {:?}, got: {:?}",
                expected_gws,
                gw_map
            );
        }
    }
}
