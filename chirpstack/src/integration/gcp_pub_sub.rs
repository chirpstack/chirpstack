use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use gcp_auth::{AuthenticationManager, CustomServiceAccount};
use prost::Message;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::Serialize;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::GcpPubSubConfiguration;
use chirpstack_api::api::Encoding;
use chirpstack_api::integration;

pub struct Integration {
    json: bool,
    project_id: String,
    topic_name: String,
    auth_manager: gcp_auth::AuthenticationManager,
    timeout: Duration,
}

#[derive(Serialize)]
struct PublishRequest {
    pub messages: Vec<PubSubMessage>,
}

#[derive(Serialize)]
struct PubSubMessage {
    pub data: String,
    pub attributes: PubSubMessageAttributes,
}

#[derive(Serialize)]
struct PubSubMessageAttributes {
    pub event: String,
    pub dev_eui: String,
    pub application_id: String,
}

impl Integration {
    pub async fn new(conf: &GcpPubSubConfiguration) -> Result<Integration> {
        trace!("Initializing GCP Pub-Sub integration");
        let service_account = CustomServiceAccount::from_json(&conf.credentials_file)?;
        let auth_manager = AuthenticationManager::from(service_account);

        Ok(Integration {
            json: match Encoding::try_from(conf.encoding)
                .map_err(|_| anyhow!("Invalid encoding"))?
            {
                Encoding::Json => true,
                Encoding::Protobuf => false,
            },
            project_id: conf.project_id.clone(),
            topic_name: conf.topic_name.clone(),
            auth_manager,
            timeout: Duration::from_secs(5),
        })
    }

    async fn publish(
        &self,
        event: &str,
        application_id: &str,
        dev_eui: &str,
        pl: &[u8],
    ) -> Result<()> {
        info!(
            event = %event, dev_eui = %dev_eui,
            "Publishing event"
        );

        let topic = format!("projects/{}/topics/{}", self.project_id, self.topic_name);

        let pl = PublishRequest {
            messages: vec![PubSubMessage {
                data: general_purpose::STANDARD.encode(pl),
                attributes: PubSubMessageAttributes {
                    event: event.to_string(),
                    dev_eui: dev_eui.to_string(),
                    application_id: application_id.to_string(),
                },
            }],
        };
        let pl = serde_json::to_string(&pl)?;

        let token = self
            .auth_manager
            .get_token(&["https://www.googleapis.com/auth/pubsub"])
            .await
            .context("Get GCP bearer token")?;

        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", token.as_str()).parse().unwrap(),
        );

        let res = client
            .post(format!(
                "https://pubsub.googleapis.com/v1/{}:publish",
                topic
            ))
            .body(pl)
            .headers(headers)
            .send()
            .await?;
        res.error_for_status()?;

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
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("up", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("join", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("ack", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("txack", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("log", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("status", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("location", &di.application_id, &di.dev_eui, &pl)
            .await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let pl = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish("integration", &di.application_id, &di.dev_eui, &pl)
            .await
    }
}
