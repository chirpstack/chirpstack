use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
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
    history: &[internal::GatewayRxInfoHistory],
    use_only_last_uplink: bool,
) -> Result<internal::DownlinkGateway> {
    let region_conf = region::get(region_config_id)?;
    let tenant_id_bytes = tenant_id.map(|v| v.as_bytes().to_vec()).unwrap_or_default();

    // In case of Class-A and OTAA, we only use the last item from the list, as this contains the context
    // blobs related to the Class-A uplink. We need this context blob as it contains the uplink
    // timestamp info.
    let mut history = if use_only_last_uplink {
        if let Some(h) = history.last() {
            vec![h.clone()]
        } else {
            vec![]
        }
    } else {
        history.to_vec()
    };

    // Filter out private gateways that are not ours.
    for h in &mut history {
        h.items.retain(|rx_info| {
            if tenant_id_bytes.is_empty() {
                !rx_info.is_private_down
            } else if tenant_id_bytes == rx_info.tenant_id {
                true
            } else {
                !rx_info.is_private_down
            }
        });
    }

    // Filter out empty history records.
    history.retain(|h| !h.items.is_empty());

    if history.is_empty() {
        return Err(anyhow!(
            "gateway rx history is empty after filtering, no downlink path available"
        ));
    }

    #[derive(Debug, Default, Clone)]
    struct GatewayStats {
        gateway_id: Vec<u8>,
        count: usize,
        total_snr: f32,
        total_rssi: i32,
        total_link_margin: f32,
        board: u32,
        antenna: u32,
        context: Vec<u8>,
        gateway_downlink_priority: usize,
    }

    // Deduplicate per gateway. We store the last board, antenna and context blob.
    let mut stats: HashMap<Vec<u8>, GatewayStats> = HashMap::new();
    for h in &history {
        let dr = region_conf.get_data_rate(true, h.dr as u8)?;
        let required_snr = if let DataRateModulation::Lora(dr) = dr {
            config::get_required_snr_for_sf(dr.spreading_factor)?
        } else {
            0.0
        };

        for i in &h.items {
            let entry = stats.entry(i.gateway_id.clone()).or_default();

            entry.count += 1;
            entry.total_snr += i.lora_snr;
            entry.total_rssi += i.rssi;
            entry.total_link_margin += i.lora_snr - required_snr;
            entry.board = i.board;
            entry.antenna = i.antenna;
            entry.context = i.context.clone();
            entry.gateway_id = i.gateway_id.clone();
            entry.gateway_downlink_priority = if i.gateway_downlink_priority > 0 {
                i.gateway_downlink_priority as usize
            } else {
                1
            };
        }
    }
    let mut stats: Vec<GatewayStats> = stats.into_values().collect();

    // Sort by avg link-margin.
    stats.sort_by(|a, b| {
        let avg_rssi_a = a.total_rssi / a.count as i32;
        let avg_rssi_b = b.total_rssi / b.count as i32;
        let avg_link_margin_a = a.total_link_margin / a.count as f32;
        let avg_link_margin_b = b.total_link_margin / b.count as f32;

        if avg_link_margin_a == avg_link_margin_b {
            return avg_rssi_b.partial_cmp(&avg_rssi_a).unwrap();
        }

        avg_link_margin_b.partial_cmp(&avg_link_margin_a).unwrap()
    });

    // Create new vec where avg. link-margin is above min_snr_margin
    let filtered_stats: Vec<GatewayStats> = stats
        .iter()
        .filter(|v| {
            let avg_link_margin = v.total_link_margin / v.count as f32;
            avg_link_margin >= min_snr_margin
        })
        .cloned()
        .collect();

    let gw = if filtered_stats.is_empty() {
        // If none of the gateways are above min_snr_margin, take the first one from the sorted
        // list.
        stats[0].clone()
    } else {
        // Else take a random one from the list, taking the number of times a gateway reported an uplink
        // + downlink gateway priority for this device into account as weight. More stable gateways aretherefore
        // therefore the most likely candidate for sending the downlink.
        let mut rng = rand::rng();
        let dist = WeightedIndex::new(
            filtered_stats
                .iter()
                .map(|v| v.gateway_downlink_priority * v.count),
        )?;
        filtered_stats[dist.sample(&mut rng)].clone()
    };

    Ok(internal::DownlinkGateway {
        gateway_id: gw.gateway_id,
        antenna: gw.antenna,
        board: gw.board,
        context: gw.context,
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
                    preamble: 0,
                    no_crc: false,
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
        history: Vec<internal::GatewayRxInfoHistory>,
        expected_gws: Vec<Vec<u8>>,
        class_a: bool,
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
                class_a: false,
                min_snr_margin: 0.0,
                history: vec![internal::GatewayRxInfoHistory {
                    dr: 0,
                    items: vec![internal::GatewayRxInfoHistoryItem {
                        lora_snr: -5.0,
                        gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                        ..Default::default()
                    }],
                }],
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]],
            },
            // two items, below min snr
            Test {
                tenant_id: None,
                class_a: false,
                min_snr_margin: 5.0,
                history: vec![internal::GatewayRxInfoHistory {
                    dr: 2, // -15 is required
                    items: vec![
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -11.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                    ],
                }],
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
            // two items, one below min snr
            Test {
                tenant_id: None,
                class_a: false,
                min_snr_margin: 5.0,
                history: vec![internal::GatewayRxInfoHistory {
                    dr: 2, // -15 is required
                    items: vec![
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -10.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
            // four items, two below min snr
            Test {
                tenant_id: None,
                class_a: false,
                min_snr_margin: 5.0,
                history: vec![internal::GatewayRxInfoHistory {
                    dr: 2, // -15 is required
                    items: vec![
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -12.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -11.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -10.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            lora_snr: -9.0,
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                expected_gws: vec![
                    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03],
                    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04],
                ],
            },
            // is_private_down is set, first gateway matches tenant.
            Test {
                tenant_id: Some(t.id.into()),
                class_a: false,
                min_snr_margin: 0.0,
                history: vec![internal::GatewayRxInfoHistory {
                    items: vec![
                        internal::GatewayRxInfoHistoryItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            is_private_down: true,
                            tenant_id: t.id.as_bytes().to_vec(),
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            is_private_down: true,
                            tenant_id: Uuid::new_v4().as_bytes().to_vec(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]],
            },
            // is_private_down is set, second gateway matches tenant.
            Test {
                tenant_id: Some(t.id.into()),
                class_a: false,
                min_snr_margin: 0.0,
                history: vec![internal::GatewayRxInfoHistory {
                    items: vec![
                        internal::GatewayRxInfoHistoryItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            is_private_down: true,
                            tenant_id: Uuid::new_v4().as_bytes().to_vec(),
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            is_private_down: true,
                            tenant_id: t.id.as_bytes().to_vec(),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
            // is_private_down is set for one gateway, no tenant id given.
            Test {
                tenant_id: None,
                class_a: false,
                min_snr_margin: 0.0,
                history: vec![internal::GatewayRxInfoHistory {
                    items: vec![
                        internal::GatewayRxInfoHistoryItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
                            is_private_down: true,
                            tenant_id: t.id.as_bytes().to_vec(),
                            ..Default::default()
                        },
                        internal::GatewayRxInfoHistoryItem {
                            gateway_id: vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02],
                            is_private_down: false,
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                expected_gws: vec![vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]],
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            println!("> Test: {}", i);
            let mut gw_map = HashMap::new();

            let mut expected_gws = HashMap::new();
            for gw_id in &test.expected_gws {
                expected_gws.insert(gw_id.clone(), ());
            }

            for _ in 0..100 {
                let out = select_downlink_gateway(
                    test.tenant_id,
                    "eu868",
                    test.min_snr_margin,
                    &test.history,
                    test.class_a,
                )
                .unwrap();
                gw_map.insert(out.gateway_id, ());
            }

            assert_eq!(test.expected_gws.len(), gw_map.len());
            assert!(
                expected_gws.keys().all(|k| gw_map.contains_key(k)),
                "Expected: {:?}, got: {:?}",
                expected_gws,
                gw_map
            );
        }
    }
}
