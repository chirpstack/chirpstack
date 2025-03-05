use anyhow::Result;
use futures::stream::StreamExt;
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook_tokio::Signals;
use tracing::{info, warn};

use crate::gateway;
use crate::{adr, api, applayer::fuota, backend, downlink, integration, region, storage};

pub async fn run() -> Result<()> {
    info!(
        version = env!("CARGO_PKG_VERSION"),
        docs = "https://www.chirpstack.io/",
        "Starting ChirpStack LoRaWAN Network Server"
    );

    storage::setup().await?;
    region::setup()?;
    backend::setup().await?;
    adr::setup().await?;
    integration::setup().await?;
    gateway::backend::setup().await?;
    downlink::setup().await;
    fuota::setup().await;
    api::setup().await?;

    let mut signals = Signals::new([SIGINT, SIGTERM]).unwrap();
    if let Some(signal) = signals.next().await {
        warn!(signal = ?signal, "Signal received, terminating process");
    }

    Ok(())
}
