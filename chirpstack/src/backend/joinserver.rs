use std::sync::{Arc, RwLock};

use anyhow::Result;
use tracing::info;

use crate::{config, stream};
use backend::{Client, ClientConfig};
use lrwn::{EUI64Prefix, EUI64};

lazy_static! {
    static ref CLIENTS: RwLock<Vec<(EUI64Prefix, Arc<Client>)>> = RwLock::new(vec![]);
}

pub fn setup() -> Result<()> {
    info!("Setting up Join Server clients");
    let conf = config::get();

    let mut clients_w = CLIENTS.write().unwrap();
    *clients_w = vec![];

    for js in &conf.join_server.servers {
        info!(join_eui_prefix = %js.join_eui_prefix, "Configuring Join Server");

        let c = Client::new(ClientConfig {
            sender_id: conf.network.net_id.to_vec(),
            server: js.server.clone(),
            ca_cert: js.ca_cert.clone(),
            tls_cert: js.tls_cert.clone(),
            tls_key: js.tls_key.clone(),
            async_timeout: js.async_timeout,
            request_log_fn: Some(Box::new(move |log| {
                Box::pin(async move { stream::backend_interfaces::log_request(log).await })
            })),
            ..Default::default()
        })?;

        clients_w.push((js.join_eui_prefix, Arc::new(c)));
    }

    Ok(())
}

pub fn get(join_eui: EUI64) -> Result<Arc<Client>> {
    let clients_r = CLIENTS.read().unwrap();
    for client in clients_r.iter() {
        if client.0.matches(join_eui) {
            return Ok(client.1.clone());
        }
    }

    Err(anyhow!(
        "Join Server client for join_eui {} does not exist",
        join_eui
    ))
}

#[cfg(test)]
pub fn reset() {
    let mut clients_w = CLIENTS.write().unwrap();
    *clients_w = vec![];
}
