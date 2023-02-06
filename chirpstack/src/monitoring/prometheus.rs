use std::sync::RwLock;

use anyhow::Result;
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::{Metric, Registry};

lazy_static! {
    static ref REGISTRY: RwLock<Registry> = RwLock::new(<Registry>::default());
}

pub fn encode_to_string() -> Result<String> {
    let registry_r = REGISTRY.read().unwrap();
    let mut buffer = String::new();
    encode(&mut buffer, &registry_r)?;

    Ok(buffer)
}

pub fn register(name: &str, help: &str, metric: impl Metric) {
    let mut registry_w = REGISTRY.write().unwrap();
    registry_w.register(name, help, metric)
}
