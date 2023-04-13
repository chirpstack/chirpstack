use std::str::FromStr;

use anyhow::Result;
use rand::seq::SliceRandom;
use uuid::Uuid;

use chirpstack_api::{gw, internal};
use lrwn::region::DataRateModulation;

use crate::config;
use crate::region;

// Returns the gateway to use for downlink.
// It will filter out private gateways (gateways from a different tenant ID,
// that do not allow downlinks). The result will be sorted based on SNR / RSSI.
// The returned value is:
//  * A random item from the elements with an SNR > minSNR
//  * The first item of the sorted slice (failing the above)
//  * An error in case no gateways are available
pub fn select_downlink_gateway(
    tenant_id: Option<Uuid>,
    region_config_id: &str,
    min_snr_margin: f32,
    rx_info: &mut internal::DeviceGatewayRxInfo,
) -> Result<internal::DeviceGatewayRxInfoItem> {
    rx_info.items.retain(|rx_info| {
        if let Some(tenant_id) = &tenant_id {
            if tenant_id.as_bytes().to_vec() == rx_info.tenant_id {
                // The tenant is the same as the gateway tenant.
                true
            } else {
                // If tenant_id is different, filter out rx_info elements that have
                // is_private_down=true.
                !rx_info.is_private_down
            }
        } else {
            // If tenant_id is None, filter out rx_info elements that have
            // is_private_down=true.
            !rx_info.is_private_down
        }
    });

    if rx_info.items.is_empty() {
        return Err(anyhow!(
            "RxInfo set is empty after applying filters, no downlink gateway available"
        ));
    }

    let region_conf = region::get(region_config_id)?;

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
                    code_rate: gw::CodeRate::from_str(&v.coding_rate)
                        .map_err(|e| anyhow!("{}", e))?
                        .into(),
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
    use crate::storage::tenant;
    use crate::test;

    struct Test {
        min_snr_margin: f32,
        tenant_id: Option<Uuid>,
        rx_info: internal::DeviceGatewayRxInfo,
        expected_gws: Vec<Vec<u8>>,
    }

    #[tokio::test]
    async fn test_select_downlink_gateway() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let tests = vec![
            // single item
            Test {
                tenant_id: None,
                min_snr_margin: 0.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    dr: 0,
                    items: vec![internal::DeviceGatewayRxInfoItem {
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
                tenant_id: None,
                min_snr_margin: 5.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    dr: 2, // -15 is required
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
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
                tenant_id: None,
                min_snr_margin: 5.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    dr: 2, // -15 is required
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
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
                tenant_id: None,
                min_snr_margin: 5.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    dr: 2, // -15 is required
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
                            lora_snr: -11.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
                            lora_snr: -10.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
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
            // is_private_down is set, first gateway matches tenant.
            Test {
                tenant_id: Some(t.id),
                min_snr_margin: 0.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            is_private_down: true,
                            tenant_id: t.id.as_bytes().to_vec(),
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            is_private_down: true,
                            tenant_id: Uuid::new_v4().as_bytes().to_vec(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]],
            },
            // is_private_down is set, second gateway matches tenant.
            Test {
                tenant_id: Some(t.id),
                min_snr_margin: 0.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            is_private_down: true,
                            tenant_id: Uuid::new_v4().as_bytes().to_vec(),
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            is_private_down: true,
                            tenant_id: t.id.as_bytes().to_vec(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
            // is_private_down is set for one gateway, no tenant id given.
            Test {
                tenant_id: None,
                min_snr_margin: 0.0,
                rx_info: internal::DeviceGatewayRxInfo {
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            is_private_down: true,
                            tenant_id: t.id.as_bytes().to_vec(),
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            is_private_down: false,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
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
                let out = select_downlink_gateway(
                    test.tenant_id,
                    &"eu868",
                    test.min_snr_margin,
                    &mut rx_info,
                )
                .unwrap();
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
