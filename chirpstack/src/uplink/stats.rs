use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use tracing::{error, info, span, trace, warn, Instrument, Level};

use crate::gateway::backend as gateway_backend;
use crate::helpers::errors::PrintFullError;
use crate::storage::{error::Error, fields, gateway, metrics};
use crate::{config, region};
use chirpstack_api::{common, gw};
use lrwn::EUI64;

pub struct Stats {
    gateway_id: EUI64,
    stats: gw::GatewayStats,
    gateway: Option<gateway::Gateway>,
}

impl Stats {
    pub async fn handle(s: gw::GatewayStats) {
        let gateway_id = match if !s.gateway_id.is_empty() {
            EUI64::from_str(&s.gateway_id).context("Gateway ID")
        } else {
            EUI64::from_slice(&s.gateway_id_legacy).context("Legacy Gateway ID")
        } {
            Ok(v) => v,
            Err(e) => {
                warn!(error = %e.full(), "Decode stats gateway_id error");
                return;
            }
        };

        let span = span!(Level::INFO, "stats", gateway_id = %gateway_id);

        if let Err(e) = Stats::_handle(gateway_id, s).instrument(span).await {
            match e.downcast_ref::<Error>() {
                Some(Error::NotFound(_)) => {
                    // Only log an error in case allow_unknown_gateways is not set. Else it is
                    // expected that we will see NotFound errors as the gateway might not exist in
                    // the database.
                    let conf = config::get();
                    if !conf.gateway.allow_unknown_gateways {
                        error!(error = %e.full(), "Handle gateway stats error");
                    }
                }
                Some(_) | None => {
                    error!(error = %e.full(), "Handle gateway stats error");
                }
            }
        }
    }

    async fn _handle(gateway_id: EUI64, s: gw::GatewayStats) -> Result<()> {
        let mut ctx = Stats {
            gateway_id,
            stats: s,
            gateway: None,
        };

        ctx.update_gateway_state().await?;
        ctx.save_stats().await?;
        ctx.save_duty_cycle_stats().await?;
        ctx.update_gateway_configuration().await?;

        Ok(())
    }

    async fn update_gateway_state(&mut self) -> Result<()> {
        trace!("Update gateway state");

        let mut gw_cs = gateway::GatewayChangeset {
            last_seen_at: Some(Some(Utc::now())),
            properties: Some(fields::KeyValue::new(self.stats.metadata.clone())),
            ..Default::default()
        };

        if let Some(loc) = &self.stats.location {
            // Sanity check to make sure there is a location.
            if !(loc.latitude == 0.0 && loc.longitude == 0.0 && loc.altitude == 0.0) {
                gw_cs.latitude = Some(loc.latitude);
                gw_cs.longitude = Some(loc.longitude);
                gw_cs.altitude = Some(loc.altitude as f32);
            }
        }

        self.gateway = Some(
            gateway::partial_update(self.gateway_id, &gw_cs)
                .await
                .context("Update gateway state")?,
        );

        Ok(())
    }

    async fn save_stats(&self) -> Result<()> {
        trace!("Saving stats");

        let mut m = metrics::Record {
            time: match &self.stats.time {
                Some(v) => DateTime::try_from(v.clone())
                    .map_err(anyhow::Error::msg)?
                    .into(),
                None => Local::now(),
            },
            kind: metrics::Kind::ABSOLUTE,
            metrics: HashMap::new(),
        };

        let region_config_id = self
            .stats
            .metadata
            .get("region_config_id")
            .ok_or_else(|| anyhow!("No region_config_id in meta-data"))?;

        let tx_per_dr = per_modultation_to_per_dr(
            region_config_id,
            false,
            &self.stats.tx_packets_per_modulation,
        )
        .context("tx packet per modulation to tx packets per DR")?;
        let rx_per_dr = per_modultation_to_per_dr(
            region_config_id,
            true,
            &self.stats.rx_packets_per_modulation,
        )
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
            &metrics::Aggregation::default_aggregations(),
        )
        .await
        .context("Save gateway stats")?;

        Ok(())
    }

    async fn save_duty_cycle_stats(&self) -> Result<()> {
        trace!("Saving duty-cycle stats");

        let duty_cycle_stats = match self.stats.duty_cycle_stats.as_ref() {
            Some(v) => v,
            None => {
                // No stats, nothing to do.
                return Ok(());
            }
        };

        let window: Duration = duty_cycle_stats
            .window
            .clone()
            .map(|v| v.try_into().unwrap_or_default())
            .unwrap_or_default();

        let mut m = metrics::Record {
            time: match &self.stats.time {
                Some(v) => DateTime::try_from(v.clone())
                    .map_err(anyhow::Error::msg)?
                    .into(),
                None => Local::now(),
            },
            kind: metrics::Kind::COUNTER,
            metrics: HashMap::new(),
        };

        for b in &duty_cycle_stats.bands {
            let load_max: Duration = b
                .load_max
                .clone()
                .map(|d| d.try_into().unwrap_or_default())
                .unwrap_or_default();
            let load_tracked: Duration = b
                .load_tracked
                .clone()
                .map(|d| d.try_into().unwrap_or_default())
                .unwrap_or_default();

            let permille = load_max.as_nanos() / (window.as_nanos() / 1000);
            let key = format!(
                "{}_{}_{}_{}",
                b.name, b.frequency_min, b.frequency_max, permille
            );
            let dc_max_load_perc_key = format!("max_load_perc_{}", key);
            let dc_window_perc_key = format!("window_perc_{}", key);

            let dc_max_load_perc =
                load_tracked.as_nanos() as f64 / load_max.as_nanos() as f64 * 100.0;
            let dc_window_perc = load_tracked.as_nanos() as f64 / window.as_nanos() as f64 * 100.0;

            m.metrics.insert(dc_max_load_perc_key, dc_max_load_perc);
            m.metrics.insert(dc_window_perc_key, dc_window_perc);
        }

        metrics::save(
            &format!("gw:dc:{}", self.gateway.as_ref().unwrap().gateway_id),
            &m,
            &[metrics::Aggregation::MINUTE],
        )
        .await
        .context("Save gateway duty-cycle stats")?;

        Ok(())
    }

    async fn update_gateway_configuration(&self) -> Result<()> {
        trace!("Updating gateway configuration");

        if !self.stats.metadata.contains_key("concentratord_version") {
            trace!("Gateway configuration only works with Concentratord, skipping");
            return Ok(());
        }

        let gw = self.gateway.as_ref().unwrap();
        let region_config_id = self
            .stats
            .metadata
            .get("region_config_id")
            .cloned()
            .unwrap_or_default();

        let gateway_conf = config::get_region_gateway(&region_config_id)?;
        if gateway_conf.channels.is_empty() {
            trace!("Skipping gateway configuration, channels is empty");
            return Ok(());
        }

        // get gw config version
        let gw_config_version = self
            .stats
            .metadata
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
            gateway_id_legacy: self.stats.gateway_id_legacy.clone(),
            version: hash,
            channels: gateway_conf
                .channels
                .iter()
                .map(|c| gw::ChannelConfiguration {
                    frequency: c.frequency,
                    modulation_legacy: match c.modulation {
                        config::GatewayChannelModulation::LORA => common::Modulation::Lora,
                        config::GatewayChannelModulation::FSK => common::Modulation::Fsk,
                    }
                    .into(),
                    modulation_config: Some(match c.modulation {
                        config::GatewayChannelModulation::LORA => {
                            gw::channel_configuration::ModulationConfig::LoraModulationConfig(
                                gw::LoraModulationConfig {
                                    bandwidth_legacy: c.bandwidth / 1000,
                                    bandwidth: c.bandwidth,
                                    spreading_factors: c.spreading_factors.clone(),
                                },
                            )
                        }
                        config::GatewayChannelModulation::FSK => {
                            gw::channel_configuration::ModulationConfig::FskModulationConfig(
                                gw::FskModulationConfig {
                                    bandwidth_legacy: c.bandwidth / 1000,
                                    bandwidth: c.bandwidth,
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

        gateway_backend::send_configuration(&region_config_id, &gw_conf)
            .await
            .context("Send gateway configuration")
    }
}

fn per_modultation_to_per_dr(
    region_config_id: &str,
    uplink: bool,
    items: &[gw::PerModulationCount],
) -> Result<HashMap<u8, usize>> {
    let mut out: HashMap<u8, usize> = HashMap::new();
    let region_conf = region::get(region_config_id)?;

    for item in items {
        let modu = item
            .modulation
            .as_ref()
            .ok_or_else(|| anyhow!("modulation is None"))?;
        let params = modu
            .parameters
            .as_ref()
            .ok_or_else(|| anyhow!("parameters is None"))?;

        let dr_modulation = match params {
            gw::modulation::Parameters::Lora(v) => {
                lrwn::region::DataRateModulation::Lora(lrwn::region::LoraDataRate {
                    spreading_factor: v.spreading_factor as u8,
                    bandwidth: v.bandwidth,
                    coding_rate: v.code_rate().into(),
                })
            }
            gw::modulation::Parameters::Fsk(v) => {
                lrwn::region::DataRateModulation::Fsk(lrwn::region::FskDataRate {
                    bitrate: v.datarate,
                })
            }
            gw::modulation::Parameters::LrFhss(v) => {
                lrwn::region::DataRateModulation::LrFhss(lrwn::region::LrFhssDataRate {
                    coding_rate: v.code_rate().into(),
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
