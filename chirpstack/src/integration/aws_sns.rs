use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use aws_credential_types::provider::{future, ProvideCredentials, Result as CredentialsResult};
use aws_credential_types::Credentials;
use aws_sdk_sns::types::MessageAttributeValue;
use aws_types::region::Region;
use base64::{engine::general_purpose, Engine as _};
use prost::Message;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::AwsSnsConfiguration;
use chirpstack_api::api::Encoding;
use chirpstack_api::integration;

#[derive(Debug)]
struct StaticCredentials {
    aws_access_key_id: String,
    aws_secret_access_key: String,
}

impl StaticCredentials {
    fn new(key: &str, secret: &str) -> Self {
        StaticCredentials {
            aws_access_key_id: key.to_string(),
            aws_secret_access_key: secret.to_string(),
        }
    }

    fn credentials(&self) -> CredentialsResult {
        Ok(Credentials::new(
            self.aws_access_key_id.clone(),
            self.aws_secret_access_key.clone(),
            None,
            None,
            "StaticProvider",
        ))
    }
}

impl ProvideCredentials for StaticCredentials {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::ready(self.credentials())
    }
}

pub struct Integration {
    json: bool,
    client: aws_sdk_sns::Client,
    topic_arn: String,
}

impl Integration {
    pub async fn new(conf: &AwsSnsConfiguration) -> Result<Integration> {
        trace!("Initializing AWS SNS integration");

        let credentials = StaticCredentials::new(&conf.access_key_id, &conf.secret_access_key);
        let config = aws_config::ConfigLoader::default()
            .credentials_provider(credentials)
            .region(Region::new(conf.region.clone()))
            .load()
            .await;
        let client = aws_sdk_sns::Client::new(&config);

        Ok(Integration {
            json: match Encoding::try_from(conf.encoding)
                .map_err(|_| anyhow!("Invalid encoding"))?
            {
                Encoding::Json => true,
                Encoding::Protobuf => false,
            },
            topic_arn: conf.topic_arn.clone(),
            client,
        })
    }

    async fn publish(
        &self,
        event: &str,
        application_id: &str,
        dev_eui: &str,
        pl: &str,
    ) -> Result<()> {
        info!(event = %event, dev_eui = %dev_eui, "Publishing event");
        self.client
            .publish()
            .topic_arn(self.topic_arn.clone())
            .message_attributes(
                "event",
                MessageAttributeValue::builder()
                    .data_type("String")
                    .string_value(event)
                    .build(),
            )
            .message_attributes(
                "dev_eui",
                MessageAttributeValue::builder()
                    .data_type("String")
                    .string_value(dev_eui)
                    .build(),
            )
            .message_attributes(
                "application_id",
                MessageAttributeValue::builder()
                    .data_type("String")
                    .string_value(application_id)
                    .build(),
            )
            .message(pl)
            .send()
            .await?;

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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
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
            true => serde_json::to_string(&pl)?,
            false => general_purpose::STANDARD.encode(pl.encode_to_vec()),
        };

        self.publish("integration", &di.application_id, &di.dev_eui, &pl)
            .await
    }
}
