// TODO: provide SHA hash of the script so it can be shown in UI? (to ensure CI correctness)

use anyhow::{Result};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, trace, warn};
use chrono::{DateTime, Utc};
use crate::config;

pub mod plugin;
pub mod passthrough;

lazy_static! {
    static ref CODEC_PLUGINS: RwLock<HashMap<String, Box<dyn Handler + Sync + Send>>> =
        RwLock::new(HashMap::new());
}

pub async fn setup() -> Result<()> {
    info!("Setting up codec plugins");
    let mut plugins = CODEC_PLUGINS.write().await;

    trace!("Setting up included algorithms");
    let a = plugin::Plugin::default()?;
    plugins.insert(a.get_id(), Box::new(a));

    trace!("Setting up provided codec plugins");
    let conf = config::get();
    for file_path in &conf.codec.js.plugins {
        info!(file_path = %file_path, "Setting up codec plugin");
        let a = plugin::Plugin::new(file_path)?;
        plugins.insert(a.get_id(), Box::new(a));
    }

    Ok(())
}

pub async fn get_plugins() -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();

    let plugins = CODEC_PLUGINS.read().await;
    for (_, v) in plugins.iter() {
        out.insert(v.get_id(), v.get_name());
    }

    out
}

pub async fn encode(plugin_id: &str, f_port: u8, variables: &HashMap<String, String>, obj: &prost_types::Struct) -> Result<Vec<u8>> {
    let plugins = CODEC_PLUGINS.read().await;
    match plugins.get(plugin_id) {
        Some(v) => {
            v.encode(f_port, variables, obj).await
        },
        None => {
            warn!(plugin_id = %plugin_id, "No codec plugin configured with given ID");
            Err(anyhow!("No codec plugin configured with given ID: {}", plugin_id))
        }
    }
}

pub async fn decode(plugin_id: &str, recv_time: DateTime<Utc>, f_port: u8, variables: &HashMap<String, String>, b: &[u8]) -> Result<pbjson_types::Struct> {
    let plugins = CODEC_PLUGINS.read().await;
    match plugins.get(plugin_id) {
        Some(v) => {
            v.decode(recv_time, f_port, variables, b).await
        },
        None => {
            warn!(plugin_id = %plugin_id, "No codec plugin configured with given ID");
            Err(anyhow!("No codec plugin configured with given ID: {}", plugin_id))
        }
    }
}

#[async_trait]
pub trait Handler {
    // Returns the name.
    fn get_name(&self) -> String;

    // Get the ID.
    fn get_id(&self) -> String;

    // Encode downlink
    async fn encode(&self, f_port: u8, variables: &HashMap<String, String>, obj: &prost_types::Struct) -> Result<Vec<u8>>;

    // Decode uplink
    async fn decode(&self, recv_time: DateTime<Utc>, f_port: u8, variables: &HashMap<String, String>, b: &[u8]) -> Result<pbjson_types::Struct>;
}

// TODO: add tests
