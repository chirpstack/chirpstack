use std::sync::{Arc, LazyLock};

use anyhow::Result;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::{config, stream};
use backend::{Client, ClientConfig};
use lrwn::{EUI64, EUI64Prefix};

type EuiClientList = Vec<(EUI64Prefix, Arc<Client>)>;

static CLIENTS: LazyLock<RwLock<EuiClientList>> = LazyLock::new(|| RwLock::new(vec![]));

pub async fn setup() -> Result<()> {
    info!("Setting up Join Server clients");
    let conf = config::get();

    let mut clients_w = CLIENTS.write().await;
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
            request_log_sender: stream::backend_interfaces::get_log_sender().await,
            authorization: if js.authorization_header.is_empty() {
                None
            } else {
                Some(js.authorization_header.clone())
            },
            ..Default::default()
        })?;

        clients_w.push((js.join_eui_prefix, Arc::new(c)));
    }

    Ok(())
}

pub async fn get(join_eui: EUI64) -> Result<Arc<Client>> {
    for client in CLIENTS.read().await.iter() {
        if client.0.matches(join_eui) {
            return Ok(client.1.clone());
        }
    }

    let conf = config::get();
    if conf.join_server.default.enabled {
        debug!(join_eui = %join_eui, "Configuring default join-server client");

        let server = if conf.join_server.default.server.is_empty() {
            let join_eui: String = join_eui
                .to_string()
                .chars()
                .rev()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(".");

            format!(
                "https://{}{}",
                join_eui, conf.join_server.resolve_join_eui_domain_suffix
            )
        } else {
            conf.join_server.default.server.clone()
        };

        let c = Client::new(ClientConfig {
            sender_id: conf.network.net_id.to_vec(),
            server,
            ca_cert: conf.join_server.default.ca_cert.clone(),
            tls_cert: conf.join_server.default.tls_cert.clone(),
            tls_key: conf.join_server.default.tls_key.clone(),
            async_timeout: conf.join_server.default.async_timeout,
            request_log_sender: stream::backend_interfaces::get_log_sender().await,
            authorization: if conf.join_server.default.authorization_header.is_empty() {
                None
            } else {
                Some(conf.join_server.default.authorization_header.clone())
            },
            ..Default::default()
        })?;

        let c = Arc::new(c);
        let c_out = c.clone();

        CLIENTS
            .write()
            .await
            .push((EUI64Prefix::new(join_eui.to_be_bytes(), 64), c));

        return Ok(c_out);
    }

    Err(anyhow!(
        "Join Server client for join_eui {} does not exist",
        join_eui
    ))
}

#[cfg(test)]
pub async fn reset() {
    let mut clients_w = CLIENTS.write().await;
    *clients_w = vec![];
}
