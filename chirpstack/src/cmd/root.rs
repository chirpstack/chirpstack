use anyhow::Result;
use tracing::info;

use crate::gateway;
use crate::{adr, api, backend, downlink, integration, region, storage};

pub async fn run() -> Result<()> {
    info!(
        version = env!("CARGO_PKG_VERSION"),
        docs = "https://www.chirpstack.io/",
        "Starting ChirpStack LoRaWAN Network Server"
    );

    storage::setup().await?;
    region::setup()?;
    backend::setup()?;
    adr::setup().await?;
    integration::setup().await?;
    gateway::backend::setup().await?;
    downlink::setup().await;
    api::setup().await?;

    Ok(())
}
