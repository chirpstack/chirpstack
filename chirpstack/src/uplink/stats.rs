use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use tracing::{error, info, span, trace, Instrument, Level};
use uuid::Uuid;

use crate::gateway::backend as gateway_backend;
use crate::storage::{gateway, metrics};
use crate::{config, region};
use chirpstack_api::{common, gw};
use lrwn::EUI64;

pub struct Stats {
    id: Uuid,
    gateway_id: EUI64,
    stats: gw::GatewayStats,
    gateway: Option<gateway::Gateway>,
}

impl Stats {
    pub async fn handle(s: gw::GatewayStats) {
        let id = match Uuid::from_slice(&s.stats_id) {
            Ok(v) => v,
            Err(_) => Uuid::nil(),
        };

        let span = span!(Level::INFO, "stats", stats_id = %id);

        if let Err(e) = Stats::_handle(id, s).instrument(span).await {
            error!(error = %e, "Handle gateway stats error");
        }
    }

    async fn _handle(id: Uuid, s: gw::GatewayStats) -> Result<()> {
        let mut ctx = Stats {
            id,
            gateway_id: EUI64::from_slice(&s.gateway_id)?,
            stats: s,
            gateway: None,
        };

        ctx.update_gateway_state().await?;
        ctx.save_stats().await?;
        ctx.update_gateway_configuration().await?;

        Ok(())
    }

    async fn update_gateway_state(&mut self) -> Result<()> {
        trace!("Update gateway state");

        if let Some(loc) = &self.stats.location {
            self.gateway = Some(
                gateway::update_state_and_loc(
                    &self.gateway_id,
                    loc.latitude,
                    loc.longitude,
                    loc.altitude as f32,
                    &self.stats.meta_data,
                )
                .await
                .context("Update gateway state")?,
            );
        } else {
            self.gateway = Some(
                gateway::update_state(&self.gateway_id, &self.stats.meta_data)
                    .await
                    .context("Update gateway state")?,
            );
        }

        Ok(())
    }

    async fn save_stats(&self) -> Result<()> {
        trace!("Saving stats");

        let mut m = metrics::Record {
            time: match &self.stats.time {
                Some(v) => DateTime::try_from(v.clone())?.into(),
                None => Local::now(),
            },
            kind: metrics::Kind::ABSOLUTE,
            metrics: HashMap::new(),
        };

        let region_name = self
            .stats
            .meta_data
            .get("region_name")
            .ok_or(anyhow!("No region_name in meta-data"))?;

        let tx_per_dr =
            per_modultation_to_per_dr(region_name, false, &self.stats.tx_packets_per_modulation)
                .context("tx packet per modulation to tx packets per DR")?;
        let rx_per_dr =
            per_modultation_to_per_dr(region_name, true, &self.stats.rx_packets_per_modulation)
                .context("rx packet per modulation to rx packets per DR")?;

        m.metrics
            .insert("rx_count".into(), self.stats.rx_packets_received_ok as f64);
        m.metrics
            .insert("tx_count".into(), self.stats.tx_packets_emitted as f64);

        for (k, v) in &self.stats.tx_packets_per_frequency {
            m.metrics.insert(format!("tx_freq_{}", k), *v as f64);
        }

        for (k, v) in &self.stats.rx_packets_per_frequency {
            m.metrics.insert(format!("rx_freq_{}", k), *v as f64);
        }

        for (k, v) in &self.stats.tx_packets_per_status {
            m.metrics.insert(format!("tx_status_{}", k), *v as f64);
        }

        for (k, v) in &tx_per_dr {
            m.metrics.insert(format!("tx_dr_{}", k), *v as f64);
        }

        for (k, v) in &rx_per_dr {
            m.metrics.insert(format!("rx_dr_{}", k), *v as f64);
        }

        metrics::save(
            &format!("gw:{}", self.gateway.as_ref().unwrap().gateway_id),
            &m,
        )
        .await
        .context("Save metrics")?;

        Ok(())
    }

    async fn update_gateway_configuration(&self) -> Result<()> {
        trace!("Updating gateway configuration");

        if !self.stats.meta_data.contains_key("concentratord_version") {
            trace!("Gateway configuration only works with Concentratord, skipping");
            return Ok(());
        }

        let gw = self.gateway.as_ref().unwrap();
        let region_name = self
            .stats
            .meta_data
            .get("region_name")
            .cloned()
            .unwrap_or_default();

        let gateway_conf = config::get_region_gateway(&region_name)?;
        if gateway_conf.channels.is_empty() {
            trace!("Skipping gateway configuration, channels is empty");
            return Ok(());
        }

        // get gw config version
        let gw_config_version = self
            .stats
            .meta_data
            .get("config_version")
            .cloned()
            .unwrap_or_default();

        // We use the Hash trait to generate the config version.
        let mut hasher = DefaultHasher::new();
        gw.stats_interval_secs.hash(&mut hasher);
        gateway_conf.channels.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());

        if gw_config_version == hash {
            trace!(config_version = %hash, "Config version is equal, no need for config update");
            return Ok(());
        }

        info!(current_config_version = %gw_config_version, desired_config_version = %hash, "Updating gateway configuration");

        let gw_conf = gw::GatewayConfiguration {
            gateway_id: self.stats.gateway_id.clone(),
            version: hash,
            channels: gateway_conf
                .channels
                .iter()
                .map(|c| gw::ChannelConfiguration {
                    frequency: c.frequency,
                    modulation: match c.modulation {
                        config::GatewayChannelModulation::LORA => common::Modulation::Lora,
                        config::GatewayChannelModulation::FSK => common::Modulation::Fsk,
                    }
                    .into(),
                    modulation_config: Some(match c.modulation {
                        config::GatewayChannelModulation::LORA => {
                            gw::channel_configuration::ModulationConfig::LoraModulationConfig(
                                gw::LoRaModulationConfig {
                                    bandwidth: c.bandwidth / 1000,
                                    spreading_factors: c.spreading_factors.clone(),
                                },
                            )
                        }
                        config::GatewayChannelModulation::FSK => {
                            gw::channel_configuration::ModulationConfig::FskModulationConfig(
                                gw::FskModulationConfig {
                                    bandwidth: c.bandwidth / 1000,
                                    bitrate: c.datarate,
                                },
                            )
                        }
                    }),
                    ..Default::default()
                })
                .collect(),
            stats_interval: Some(pbjson_types::Duration {
                nanos: 0,
                seconds: gw.stats_interval_secs.into(),
            }),
        };

        gateway_backend::send_configuration(&region_name, &gw_conf)
            .await
            .context("Send gateway configuration")
    }
}

fn per_modultation_to_per_dr(
    region_name: &str,
    uplink: bool,
    items: &[gw::PerModulationCount],
) -> Result<HashMap<u8, usize>> {
    let mut out: HashMap<u8, usize> = HashMap::new();
    let region_conf = region::get(region_name)?;

    for item in items {
        let modu = item
            .modulation
            .as_ref()
            .ok_or(anyhow!("modulation is None"))?;
        let params = modu
            .parameters
            .as_ref()
            .ok_or(anyhow!("parameters is None"))?;

        let dr_modulation = match params {
            gw::modulation::Parameters::Lora(v) => {
                lrwn::region::DataRateModulation::Lora(lrwn::region::LoraDataRate {
                    spreading_factor: v.spreading_factor as u8,
                    bandwidth: v.bandwidth * 1000,
                })
            }
            gw::modulation::Parameters::Fsk(v) => {
                lrwn::region::DataRateModulation::Fsk(lrwn::region::FskDataRate {
                    bitrate: v.datarate,
                })
            }
            gw::modulation::Parameters::LrFhss(v) => {
                lrwn::region::DataRateModulation::LrFhss(lrwn::region::LrFhssDataRate {
                    coding_rate: v.code_rate.clone(),
                    occupied_channel_width: v.operating_channel_width,
                })
            }
        };

        if let Ok(v) = region_conf.get_data_rate_index(uplink, &dr_modulation) {
            let count = out.entry(v).or_insert(0);
            *count += 1;
        }
    }

    Ok(out)
}
