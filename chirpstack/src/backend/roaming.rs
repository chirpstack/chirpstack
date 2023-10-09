use std::collections::HashMap;
use std::io::Cursor;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use chrono::{Duration, DurationRound};
use prost::Message;
use tracing::{debug, info, span, Level};

use crate::config;
use crate::gpstime::ToGpsTime;
use backend::{Client, ClientConfig, GWInfoElement, ULMetaData};
use chirpstack_api::{common, gw};
use lrwn::{region, DevAddr, NetID, EUI64};

lazy_static! {
    static ref CLIENTS: RwLock<HashMap<NetID, Arc<Client>>> = RwLock::new(HashMap::new());
}

pub fn setup() -> Result<()> {
    info!("Setting up roaming clients");
    let conf = config::get();

    for s in &conf.roaming.servers {
        let span = span!(Level::INFO, "setup", net_id  = %s.net_id);
        let _guard = span.enter();

        let server = if s.server.is_empty() {
            format!(
                "https://{}{}",
                s.net_id, conf.roaming.resolve_net_id_domain_suffix,
            )
        } else {
            s.server.clone()
        };

        info!(
            passive_roaming_lifetime = ?s.passive_roaming_lifetime,
            server = %server,
            async_timeout = ?s.async_timeout,
            "Configuring roaming client"
        );

        let c = Client::new(ClientConfig {
            sender_id: conf.network.net_id.to_vec(),
            receiver_id: s.net_id.to_vec(),
            server,
            use_target_role_suffix: s.use_target_role_suffix,
            ca_cert: s.ca_cert.clone(),
            tls_cert: s.tls_cert.clone(),
            tls_key: s.tls_key.clone(),
            authorization: if s.authorization_header.is_empty() {
                None
            } else {
                Some(s.authorization_header.clone())
            },
            async_timeout: s.async_timeout,
        })?;

        set(&s.net_id, c);
    }

    Ok(())
}

pub fn set(net_id: &NetID, c: Client) {
    let mut clients_w = CLIENTS.write().unwrap();
    clients_w.insert(*net_id, Arc::new(c));
}

pub fn get(net_id: &NetID) -> Result<Arc<Client>> {
    let clients_r = CLIENTS.write().unwrap();

    if let Some(client) = clients_r.get(net_id) {
        return Ok(client.clone());
    }

    let conf = config::get();
    if conf.roaming.default.enabled {
        debug!(net_id = %net_id, "Configuring default roaming client");

        let server = if conf.roaming.default.server.is_empty() {
            format!(
                "https://{}{}",
                net_id, conf.roaming.resolve_net_id_domain_suffix,
            )
        } else {
            conf.roaming.default.server.clone()
        };

        let c = Client::new(ClientConfig {
            sender_id: conf.network.net_id.to_vec(),
            receiver_id: net_id.to_vec(),
            server,
            use_target_role_suffix: conf.roaming.default.use_target_role_suffix,
            ca_cert: conf.roaming.default.ca_cert.clone(),
            tls_cert: conf.roaming.default.tls_cert.clone(),
            tls_key: conf.roaming.default.tls_key.clone(),
            authorization: if conf.roaming.default.authorization_header.is_empty() {
                None
            } else {
                Some(conf.roaming.default.authorization_header.clone())
            },
            async_timeout: conf.roaming.default.async_timeout,
        })?;

        return Ok(Arc::new(c));
    }

    Err(anyhow!(
        "Roaming client for net_id {} does not exist",
        net_id
    ))
}

pub fn get_passive_roaming_lifetime(net_id: NetID) -> Result<std::time::Duration> {
    let conf = config::get();

    for s in &conf.roaming.servers {
        if s.net_id == net_id {
            return Ok(s.passive_roaming_lifetime);
        }
    }

    if conf.roaming.default.enabled {
        return Ok(conf.roaming.default.passive_roaming_lifetime);
    }

    Err(anyhow!(
        "Passive-roaming lifetime for net_id {} does not exist",
        net_id
    ))
}

pub fn get_passive_roaming_kek_label(net_id: NetID) -> Result<String> {
    let conf = config::get();

    for s in &conf.roaming.servers {
        if s.net_id == net_id {
            return Ok(s.passive_roaming_kek_label.clone());
        }
    }

    Err(anyhow!(
        "Passive-roaming kek-label for net_id {} does not exist",
        net_id
    ))
}

pub fn is_enabled() -> bool {
    let conf = config::get();
    conf.roaming.default.enabled || !conf.roaming.servers.is_empty()
}

pub fn is_roaming_dev_addr(dev_addr: DevAddr) -> bool {
    let conf = config::get();

    if !is_enabled() {
        return false;
    }

    for net_id in &[
        // Configured NetID.
        conf.network.net_id,
        // Test NetIDs. For roaming it is expected that non-testing NetIDs will be used. These are
        // included as non-roaming NetIDs as one might start with a test-NetID and then acquires an
        // official NetID to setup roaming. Not including these would mean that all devices must
        // re-join to obtain a new DevAddr.
        NetID::from_be_bytes([0, 0, 0]),
        NetID::from_be_bytes([0, 0, 1]),
    ] {
        if dev_addr.is_net_id(*net_id) {
            return false;
        }
    }

    for net_id in &conf.network.secondary_net_ids {
        if dev_addr.is_net_id(*net_id) {
            return false;
        }
    }

    true
}

pub fn get_net_ids_for_dev_addr(dev_addr: DevAddr) -> Vec<NetID> {
    let mut out: Vec<NetID> = Vec::new();
    let conf = config::get();

    for agreement in &conf.roaming.servers {
        if dev_addr.is_net_id(agreement.net_id) {
            out.push(agreement.net_id);
        }
    }

    out
}

pub fn rx_info_to_gw_info(rx_info_set: &[gw::UplinkRxInfo]) -> Result<Vec<GWInfoElement>> {
    let mut out: Vec<GWInfoElement> = Vec::new();

    for rx_info in rx_info_set {
        let gw_id = EUI64::from_str(&rx_info.gateway_id)?;

        out.push(GWInfoElement {
            id: gw_id.to_be_bytes()[4..8].to_vec(),
            fine_recv_time: rx_info
                .fine_time_since_gps_epoch
                .as_ref()
                .map(|v| v.nanos as usize),
            rf_region: "".to_string(),
            rssi: Some(rx_info.rssi as isize),
            snr: Some(rx_info.snr),
            lat: rx_info.location.as_ref().map(|v| v.latitude),
            lon: rx_info.location.as_ref().map(|v| v.longitude),
            ul_token: rx_info.encode_to_vec(),
            dl_allowed: Some(true),
        });
    }

    Ok(out)
}

pub fn ul_meta_data_to_rx_info(ul_meta_data: &ULMetaData) -> Result<Vec<gw::UplinkRxInfo>> {
    let mut out: Vec<gw::UplinkRxInfo> = Vec::new();
    for gw_info in &ul_meta_data.gw_info {
        out.push(gw::UplinkRxInfo {
            gateway_id: hex::encode(&gw_info.id),
            context: gw_info.ul_token.clone(),
            rssi: gw_info.rssi.unwrap_or_default() as i32,
            snr: gw_info.snr.unwrap_or_default(),
            location: if gw_info.lat.is_some() && gw_info.lon.is_some() {
                Some(common::Location {
                    latitude: gw_info.lat.unwrap(),
                    longitude: gw_info.lon.unwrap(),
                    ..Default::default()
                })
            } else {
                None
            },
            fine_time_since_gps_epoch: if gw_info.fine_recv_time.is_some() {
                let ts = ul_meta_data
                    .recv_time
                    .duration_round(Duration::seconds(1))?;
                let ts = ts + Duration::nanoseconds(gw_info.fine_recv_time.unwrap() as i64);

                Some(ts.to_gps_time().to_std()?.into())
            } else {
                None
            },
            ..Default::default()
        });
    }

    Ok(out)
}

pub fn ul_meta_data_to_tx_info(ul_meta_data: &ULMetaData) -> Result<gw::UplinkTxInfo> {
    let region_cn = region::CommonName::from_str(&ul_meta_data.rf_region)?;
    let region_conf = region::get(region_cn, false, false);
    let dr = match ul_meta_data.data_rate {
        Some(v) => v,
        None => {
            return Err(anyhow!("DataRate is not set"));
        }
    };
    let freq = match ul_meta_data.ul_freq {
        Some(v) => (v * 1_000_000.0) as u32,
        None => {
            return Err(anyhow!("ULFreq is not set"));
        }
    };
    let params = region_conf.get_data_rate(dr)?;

    Ok(gw::UplinkTxInfo {
        frequency: freq,
        modulation: Some(gw::Modulation {
            parameters: Some(match params {
                lrwn::region::DataRateModulation::Lora(v) => {
                    gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                        bandwidth: v.bandwidth,
                        spreading_factor: v.spreading_factor as u32,
                        code_rate: gw::CodeRate::Cr45.into(),
                        code_rate_legacy: "".into(),
                        polarization_inversion: true,
                    })
                }
                lrwn::region::DataRateModulation::Fsk(v) => {
                    gw::modulation::Parameters::Fsk(gw::FskModulationInfo {
                        datarate: v.bitrate,
                        ..Default::default()
                    })
                }
                lrwn::region::DataRateModulation::LrFhss(v) => {
                    gw::modulation::Parameters::LrFhss(gw::LrFhssModulationInfo {
                        operating_channel_width: v.occupied_channel_width,
                        code_rate: gw::CodeRate::from_str(&v.coding_rate)
                            .map_err(|e| anyhow!("{}", e))?
                            .into(),
                        // GridSteps: this value can't be derived from a DR?
                        ..Default::default()
                    })
                }
            }),
        }),
    })
}

pub fn dl_meta_data_to_uplink_rx_info(
    dl_meta: &backend::DLMetaData,
) -> Result<Vec<gw::UplinkRxInfo>> {
    let mut out: Vec<gw::UplinkRxInfo> = Vec::new();

    for gw_info in &dl_meta.gw_info {
        out.push(gw::UplinkRxInfo::decode(&mut Cursor::new(
            &gw_info.ul_token,
        ))?);
    }

    Ok(out)
}

#[cfg(test)]
pub fn reset() {
    let mut clients_w = CLIENTS.write().unwrap();
    *clients_w = HashMap::new();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_roaming_dev_addr() {
        struct Test {
            dev_addr: DevAddr,
            net_id: NetID,
            secondary_net_ids: Vec<NetID>,
            is_roaming: bool,
        }

        let tests = vec![
            Test {
                dev_addr: {
                    let mut dev_addr = DevAddr::from_be_bytes([1, 2, 3, 4]);
                    dev_addr.set_dev_addr_prefix(NetID::from_be_bytes([1, 2, 3]).dev_addr_prefix());
                    dev_addr
                },
                net_id: NetID::from_be_bytes([1, 2, 3]),
                secondary_net_ids: vec![],
                is_roaming: false,
            },
            Test {
                dev_addr: {
                    let mut dev_addr = DevAddr::from_be_bytes([1, 2, 3, 4]);
                    dev_addr.set_dev_addr_prefix(NetID::from_be_bytes([1, 2, 3]).dev_addr_prefix());
                    dev_addr
                },
                net_id: NetID::from_be_bytes([3, 2, 1]),
                secondary_net_ids: vec![],
                is_roaming: true,
            },
            Test {
                dev_addr: {
                    let mut dev_addr = DevAddr::from_be_bytes([1, 2, 3, 4]);
                    dev_addr.set_dev_addr_prefix(NetID::from_be_bytes([1, 2, 3]).dev_addr_prefix());
                    dev_addr
                },
                net_id: NetID::from_be_bytes([3, 2, 1]),
                secondary_net_ids: vec![NetID::from_be_bytes([1, 2, 3])],
                is_roaming: false,
            },
        ];

        for tst in &tests {
            let mut conf = config::Configuration::default();
            conf.network.net_id = tst.net_id;
            conf.network.secondary_net_ids = tst.secondary_net_ids.clone();
            conf.roaming.default.enabled = true;
            config::set(conf);

            assert_eq!(tst.is_roaming, is_roaming_dev_addr(tst.dev_addr));
        }
    }
}
