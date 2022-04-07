use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::config;

#[cfg(test)]
pub mod mock;
mod mqtt;

lazy_static! {
    static ref BACKENDS: RwLock<HashMap<String, Box<dyn GatewayBackend + Sync + Send>>> =
        RwLock::new(HashMap::new());
}

#[async_trait]
pub trait GatewayBackend {
    async fn send_downlink(&self, df: &chirpstack_api::gw::DownlinkFrame) -> Result<()>;
    async fn send_configuration(
        &self,
        gw_conf: &chirpstack_api::gw::GatewayConfiguration,
    ) -> Result<()>;
}

pub async fn setup() -> Result<()> {
    let conf = config::get();

    info!("Setting up gateway backends for the different regions");
    for region in &conf.regions {
        if !conf.network.enabled_regions.contains(&region.name) {
            warn!("Config exists, but region is not enabled. To enable it, add it to 'network.enabled_regions'");
            continue;
        }

        info!(
            region_name = %region.name,
            region_common_name = %region.common_name,
            "Setting up gateway backend for region"
        );

        let backend = mqtt::MqttBackend::new(
            &region.name,
            region.common_name,
            &region.gateway.backend.mqtt,
        )
        .await
        .context("New MQTT gateway backend error")?;

        set_backend(&region.name, Box::new(backend)).await;
    }

    Ok(())
}

pub async fn set_backend(region_name: &str, b: Box<dyn GatewayBackend + Sync + Send>) {
    let mut b_w = BACKENDS.write().await;
    b_w.insert(region_name.to_string(), b);
}

pub async fn send_downlink(
    region_name: &str,
    df: &chirpstack_api::gw::DownlinkFrame,
) -> Result<()> {
    let b_r = BACKENDS.read().await;
    let b = b_r
        .get(region_name)
        .ok_or_else(|| anyhow!("region_name '{}' does not exist in BACKENDS", region_name))?;

    b.send_downlink(df).await?;

    Ok(())
}

pub async fn send_configuration(
    region_name: &str,
    gw_conf: &chirpstack_api::gw::GatewayConfiguration,
) -> Result<()> {
    let b_r = BACKENDS.read().await;
    let b = b_r
        .get(region_name)
        .ok_or_else(|| anyhow!("region_name '{}' does not exist in BACKENDS", region_name))?;

    b.send_configuration(gw_conf).await?;

    Ok(())
}
