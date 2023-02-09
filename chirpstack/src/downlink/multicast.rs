use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::{DefaultIx, Graph, NodeIndex, UnGraph};
use rand::Rng;
use tracing::{span, trace, warn, Instrument, Level};

use crate::downlink::helpers;
use crate::gateway::backend as gateway_backend;
use crate::storage::{device_gateway, downlink_frame, gateway, multicast};
use crate::{config, region};
use chirpstack_api::{gw, internal};
use lrwn::EUI64;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Node {
    Gateway(EUI64),
    Device(EUI64),
}

pub struct Multicast {
    multicast_group_queue_item: multicast::MulticastGroupQueueItem,
    downlink_frame: gw::DownlinkFrame,
    region_config_id: String,
    gateway: Option<gateway::Gateway>,
    multicast_group: Option<multicast::MulticastGroup>,
}

impl Multicast {
    pub async fn handle_schedule_queue_item(qi: multicast::MulticastGroupQueueItem) -> Result<()> {
        let span = span!(Level::TRACE, "schedule", multicast_group_id = %qi.multicast_group_id, gateway_id = %qi.gateway_id);

        Multicast::_handle_schedule_queue_item(qi)
            .instrument(span)
            .await
    }

    async fn _handle_schedule_queue_item(qi: multicast::MulticastGroupQueueItem) -> Result<()> {
        trace!("Handle schedule queue item flow");

        let mut ctx = Multicast {
            downlink_frame: gw::DownlinkFrame {
                downlink_id: rand::thread_rng().gen(),
                gateway_id: qi.gateway_id.to_string(),
                ..Default::default()
            },
            multicast_group_queue_item: qi,
            region_config_id: "".into(),
            gateway: None,
            multicast_group: None,
        };

        ctx.get_gateway().await?;
        ctx.set_region_config_id()?;
        ctx.get_multicast_group().await?;
        ctx.validate_payload_size().await?;
        ctx.set_tx_info()?;
        ctx.set_phy_payload()?;
        ctx.save_downlink_frame().await?;
        ctx.send_downlink_frame().await?;

        Ok(())
    }

    async fn get_gateway(&mut self) -> Result<()> {
        trace!("Get gateway");
        let gw = gateway::get(&self.multicast_group_queue_item.gateway_id).await?;
        self.gateway = Some(gw);
        Ok(())
    }

    fn set_region_config_id(&mut self) -> Result<()> {
        trace!("Setting region name");
        let gw = self.gateway.as_ref().unwrap();
        let region_config_id = &*(gw.properties)
            .get("region_config_id")
            .cloned()
            .ok_or_else(|| anyhow!("Gateway does not have region_config_id property"))?;
        self.region_config_id = region_config_id.to_string();

        Ok(())
    }

    async fn get_multicast_group(&mut self) -> Result<()> {
        trace!("Getting multicast-group");
        let mg = multicast::get(&self.multicast_group_queue_item.multicast_group_id).await?;
        self.multicast_group = Some(mg);
        Ok(())
    }

    async fn validate_payload_size(&self) -> Result<()> {
        trace!("Validating payload size for DR");
        let mg = self.multicast_group.as_ref().unwrap();
        let region_conf = region::get(&self.region_config_id)?;

        let max_pl_size = region_conf.get_max_payload_size(
            lrwn::region::MacVersion::Latest,
            lrwn::region::Revision::Latest,
            mg.dr as u8,
        )?;

        if self.multicast_group_queue_item.data.len() > max_pl_size.n {
            warn!(
                dr = mg.dr,
                max_pl_size = max_pl_size.n,
                pl_size = self.multicast_group_queue_item.data.len(),
                "Discarding multicast-group queue item because it exceeds max. payload size"
            );
            multicast::delete_queue_item(&self.multicast_group_queue_item.id).await?;
            return Err(anyhow!(
                "Queue item exceeds max payload and has been discarded"
            ));
        }

        Ok(())
    }

    fn set_tx_info(&mut self) -> Result<()> {
        trace!("Setting tx-info");

        let network_conf = config::get_region_network(&self.region_config_id)?;
        let region_conf = region::get(&self.region_config_id)?;
        let mg = self.multicast_group.as_ref().unwrap();
        let mc_dr = region_conf.get_data_rate(mg.dr as u8)?;

        let mut tx_info = gw::DownlinkTxInfo {
            frequency: mg.frequency as u32,
            ..Default::default()
        };

        helpers::set_tx_info_data_rate(&mut tx_info, &mc_dr)?;

        // set tx power
        if network_conf.downlink_tx_power != -1 {
            tx_info.power = network_conf.downlink_tx_power;
        } else {
            tx_info.power = region_conf.get_downlink_tx_power(tx_info.frequency) as i32;
        }

        match self.multicast_group_queue_item.emit_at_time_since_gps_epoch {
            Some(v) => {
                tx_info.timing = Some(gw::Timing {
                    parameters: Some(gw::timing::Parameters::GpsEpoch(gw::GpsEpochTimingInfo {
                        time_since_gps_epoch: Some(pbjson_types::Duration::from(
                            std::time::Duration::from_millis(v as u64),
                        )),
                    })),
                });
            }
            None => {
                tx_info.timing = Some(gw::Timing {
                    parameters: Some(gw::timing::Parameters::Immediately(
                        gw::ImmediatelyTimingInfo {},
                    )),
                });
            }
        }

        self.downlink_frame.items.push(gw::DownlinkFrameItem {
            phy_payload: vec![],
            tx_info: Some(tx_info),
            tx_info_legacy: None,
        });

        Ok(())
    }

    fn set_phy_payload(&mut self) -> Result<()> {
        trace!("Setting phy payload");
        let mg = self.multicast_group.as_ref().unwrap();

        let mut phy = lrwn::PhyPayload {
            mhdr: lrwn::MHDR {
                m_type: lrwn::MType::UnconfirmedDataDown,
                major: lrwn::Major::LoRaWANR1,
            },
            payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                fhdr: lrwn::FHDR {
                    devaddr: mg.mc_addr,
                    f_cnt: self.multicast_group_queue_item.f_cnt as u32,
                    ..Default::default()
                },
                f_port: Some(self.multicast_group_queue_item.f_port as u8),
                frm_payload: Some(lrwn::FRMPayload::Raw(
                    self.multicast_group_queue_item.data.clone(),
                )),
            }),
            mic: None,
        };

        phy.encrypt_frm_payload(&mg.mc_app_s_key)?;

        // using LoRaWAN1_0 vs LoRaWAN1_1 only makes a difference when setting the
        // confirmed frame-counter
        phy.set_downlink_data_mic(lrwn::MACVersion::LoRaWAN1_1, 0, &mg.mc_nwk_s_key)?;

        self.downlink_frame.items[0].phy_payload = phy.to_vec()?;

        Ok(())
    }

    async fn save_downlink_frame(&self) -> Result<()> {
        trace!("Saving downlink frame");

        downlink_frame::save(&internal::DownlinkFrame {
            downlink_id: self.downlink_frame.downlink_id,
            multicast_group_id: self
                .multicast_group_queue_item
                .multicast_group_id
                .as_bytes()
                .to_vec(),
            multicast_group_queue_item_id: self.multicast_group_queue_item.id.as_bytes().to_vec(),
            downlink_frame: Some(self.downlink_frame.clone()),
            ..Default::default()
        })
        .await
        .context("Save downlink frame")?;

        Ok(())
    }

    async fn send_downlink_frame(&self) -> Result<()> {
        trace!("Sending downlink frame");

        gateway_backend::send_downlink(&self.region_config_id, &self.downlink_frame)
            .await
            .context("Send downlink frame")?;

        Ok(())
    }
}

pub async fn enqueue(qi: multicast::MulticastGroupQueueItem) -> Result<u32> {
    // Try first to get configured gateways for multicast-group.
    let mut gateway_ids = multicast::get_gateway_ids(&qi.multicast_group_id).await?;

    // Fallback to automatic gateway-set detection.
    if gateway_ids.is_empty() {
        // get deveuis for multicast-group
        let dev_euis = multicast::get_dev_euis(&qi.multicast_group_id).await?;

        // get DeviceGatewayRxInfo for all devices.
        let dev_gw_set = device_gateway::get_rx_info_for_dev_euis(&dev_euis).await?;

        // get minimum gateway set to cover all devices
        gateway_ids = get_minimum_gateway_set(&dev_gw_set)?;
    }

    // Enqueue multicast downlink for the given gw set.
    let (_, f_cnt) = multicast::enqueue(qi, &gateway_ids).await?;
    Ok(f_cnt)
}

fn get_minimum_gateway_set(dev_gw_set: &[internal::DeviceGatewayRxInfo]) -> Result<Vec<EUI64>> {
    if dev_gw_set.is_empty() {
        return Ok(vec![]);
    }

    let mut g: Graph<Node, f64, _> = Graph::new_undirected();
    let mut node_index: HashMap<Node, NodeIndex<DefaultIx>> = HashMap::new();

    // Get the unique set of gateways.
    let gw_set = get_gateway_set(dev_gw_set)?;

    // Add all gateways & connect them together
    // W -999 is used so that the mst algorithm will remove the edge between
    // the gateway and a device first, over removing an edge between two
    // gateways.
    for (i, gw_id) in gw_set.iter().enumerate() {
        let idx = g.add_node(Node::Gateway(*gw_id));
        node_index.insert(Node::Gateway(*gw_id), idx);

        if i != 0 {
            g.add_edge(
                *node_index.get(&Node::Gateway(gw_set[0])).unwrap(),
                *node_index.get(&Node::Gateway(*gw_id)).unwrap(),
                -999.0,
            );
        }
    }

    // Get the device count per gateway ID.
    let gateway_device_count = get_gateway_device_count_map(dev_gw_set)?;

    // Add all devices and add edges to gateways
    for dev_gw in dev_gw_set {
        let dev_eui = EUI64::from_slice(&dev_gw.dev_eui)?;
        let idx = g.add_node(Node::Device(dev_eui));
        node_index.insert(Node::Device(dev_eui), idx);

        // Add edge between device and each receiving gateway.
        for gw_rx_info in &dev_gw.items {
            let gateway_id = EUI64::from_slice(&gw_rx_info.gateway_id)?;
            g.add_edge(
                *node_index.get(&Node::Gateway(gateway_id)).unwrap(),
                *node_index.get(&Node::Device(dev_eui)).unwrap(),
                1.0 / gateway_device_count
                    .get(&gateway_id)
                    .cloned()
                    .unwrap_or_default() as f64,
            );
        }
    }

    // Get the min spanning tree to determine the minimum set of gateways that is needed to cover
    // all devices.
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));

    Ok(mst
        .node_indices()
        .filter(|i| match mst[*i] {
            // We want to get all gateway nodes that have an edge to a device. So if the node is of
            // type gateway, we request all neighbors and filter on Node::Device. In case this does
            // not return any elements, we know it has no edge to a device.
            Node::Gateway(_) => mst
                .neighbors(*i)
                .filter(|ii| match mst[*ii] {
                    Node::Device(_) => true,
                    Node::Gateway(_) => false,
                })
                .peekable()
                .peek()
                .is_some(),
            Node::Device(_) => false,
        })
        .map(|i| match mst[i] {
            Node::Gateway(v) => v,
            Node::Device(v) => v, // Should not happen as we have already filtered on gateways.
        })
        .collect())
}

fn get_gateway_set(dev_gw_set: &[internal::DeviceGatewayRxInfo]) -> Result<Vec<EUI64>> {
    let mut out: HashSet<EUI64> = HashSet::new();
    for dev_gw in dev_gw_set {
        for rx_info in &dev_gw.items {
            let gateway_id = EUI64::from_slice(&rx_info.gateway_id)?;
            out.insert(gateway_id);
        }
    }
    Ok(out.iter().cloned().collect())
}

fn get_gateway_device_count_map(
    dev_gw_set: &[internal::DeviceGatewayRxInfo],
) -> Result<HashMap<EUI64, usize>> {
    let mut out: HashMap<EUI64, usize> = HashMap::new();

    for dev_gw in dev_gw_set {
        for gw_rx_info in &dev_gw.items {
            let gateway_id = EUI64::from_slice(&gw_rx_info.gateway_id)?;
            *out.entry(gateway_id).or_insert(0) += 1;
        }
    }

    Ok(out)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_minimum_gateway_set() {
        struct Test {
            name: String,
            dev_gw_set: Vec<internal::DeviceGatewayRxInfo>,
            expected_gws: Vec<EUI64>,
        }

        let tests = vec![
            Test {
                name: "one device - one gateway".into(),
                dev_gw_set: vec![internal::DeviceGatewayRxInfo {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    items: vec![internal::DeviceGatewayRxInfoItem {
                        gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 2],
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                expected_gws: vec![EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2])],
            },
            Test {
                name: "one device - two gateways".into(),
                dev_gw_set: vec![internal::DeviceGatewayRxInfo {
                    dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                    items: vec![
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 1],
                            ..Default::default()
                        },
                        internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }],
                expected_gws: vec![EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 1])],
            },
            Test {
                name: "two devices - two gateways (no overlap)".into(),
                dev_gw_set: vec![
                    internal::DeviceGatewayRxInfo {
                        dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                        items: vec![internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 1],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    internal::DeviceGatewayRxInfo {
                        dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                        items: vec![internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                expected_gws: vec![
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 1]),
                    EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
                ],
            },
            Test {
                name: "two devices - two gateways (overlap, second gw covers two devices)".into(),
                dev_gw_set: vec![
                    internal::DeviceGatewayRxInfo {
                        dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                        items: vec![internal::DeviceGatewayRxInfoItem {
                            gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 2],
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    internal::DeviceGatewayRxInfo {
                        dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 2],
                        items: vec![
                            internal::DeviceGatewayRxInfoItem {
                                gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 1],
                                ..Default::default()
                            },
                            internal::DeviceGatewayRxInfoItem {
                                gateway_id: vec![2, 2, 2, 2, 2, 2, 2, 2],
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                ],
                expected_gws: vec![EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2])],
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);
            let gws: HashSet<EUI64> = get_minimum_gateway_set(&tst.dev_gw_set)
                .unwrap()
                .iter()
                .cloned()
                .collect();
            let expected: HashSet<EUI64> = tst.expected_gws.iter().cloned().collect();

            assert_eq!(expected, gws);
        }
    }
}
