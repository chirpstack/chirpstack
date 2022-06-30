use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use tracing::{info, span, Level};

use crate::config;
use backend::{Client, ClientConfig};
use lrwn::EUI64;

lazy_static! {
    static ref CLIENTS: RwLock<HashMap<EUI64, Arc<Client>>> = RwLock::new(HashMap::new());
}

pub fn setup() -> Result<()> {
    info!("Setting up Join Server clients");
    let conf = config::get();

    for js in &conf.join_server.servers {
        let span = span!(Level::INFO, "setup", join_eui = %js.join_eui);
        let _guard = span.enter();

        info!("Configuring Join Server");

        let c = Client::new(ClientConfig {
            sender_id: conf.network.net_id.to_vec(),
            receiver_id: js.join_eui.to_vec(),
            server: js.server.clone(),
            ca_cert: js.ca_cert.clone(),
            tls_cert: js.tls_cert.clone(),
            tls_key: js.tls_key.clone(),
            async_timeout: js.async_timeout,
            ..Default::default()
        })?;

        set(&js.join_eui, c);
    }

    Ok(())
}

pub fn set(join_eui: &EUI64, c: Client) {
    let mut clients_w = CLIENTS.write().unwrap();
    clients_w.insert(*join_eui, Arc::new(c));
}

pub fn get(join_eui: &EUI64) -> Result<Arc<Client>> {
    let clients_r = CLIENTS.read().unwrap();
    Ok(clients_r
        .get(join_eui)
        .ok_or_else(|| {
            anyhow!(
                "Join Server client for join_eui {} does not exist",
                join_eui
            )
        })?
        .clone())
}

#[cfg(test)]
pub fn reset() {
    let mut clients_w = CLIENTS.write().unwrap();
    *clients_w = HashMap::new();
}
