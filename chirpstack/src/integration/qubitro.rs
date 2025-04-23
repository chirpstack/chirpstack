use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use reqwest::Client;
use tracing::{info, trace, warn};

use super::Integration as IntegrationTrait;
use crate::storage::application::QubitroConfiguration;
use chirpstack_api::integration;

static CLIENT: OnceLock<Client> = OnceLock::new();
static QUBITRO_ENDPOINT: &str = "https://webhook.qubitro.com/integrations/chirpstack";

fn get_client() -> Client {
    CLIENT
        .get_or_init(|| {
            Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap()
        })
        .clone()
}

pub struct Integration {
    project_id: String,
    webhook_signing_key: String,
}

impl Integration {
    pub fn new(conf: &QubitroConfiguration) -> Integration {
        trace!("Initializing Qubitro integration");

        Integration {
            project_id: conf.project_id.clone(),
            webhook_signing_key: conf.webhook_signing_key.clone(),
        }
    }

    async fn post_event(&self, event: &str, b: Vec<u8>) -> Result<()> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            HeaderName::from_static("x-qubitro-project-id"),
            self.project_id.parse()?,
        );
        headers.insert(
            HeaderName::from_static("x-qubitro-signing-key"),
            self.webhook_signing_key.parse()?,
        );

        info!(event = %event, "Posting event to Qubitro");
        let res = get_client()
            .post(QUBITRO_ENDPOINT)
            .body(b)
            .query(&[("event", event)])
            .headers(headers)
            .send()
            .await;

        match res {
            Ok(res) => match res.error_for_status() {
                Ok(_) => {}
                Err(e) => {
                    warn!(event = %event, error = %e, "Posting event to Qubitro failed");
                }
            },
            Err(e) => {
                warn!(event = %event, error = %e, "Posting event to Qubitro failed");
            }
        }

        Ok(())
    }
}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("up", b).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("join", b).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("ack", b).await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("txack", b).await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("log", b).await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("status", b).await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("location", b).await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let b = serde_json::to_vec(&pl)?;
        self.post_event("integration", b).await
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_qubitro() {
        let server = MockServer::start();

        let i = Integration {
            project_id: "test-project".to_string(),
            webhook_signing_key: "test-key".to_string(),
        };

        // uplink event
        let pl: integration::UplinkEvent = Default::default();
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .header("x-qubitro-project-id", "test-project")
                .header("x-qubitro-signing-key", "test-key")
                .header("content-type", "application/json")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();
    }
} 