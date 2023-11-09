use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use prost::Message;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::AwsSnsConfiguration;
use chirpstack_api::api::Encoding;
use chirpstack_api::integration;

pub struct Integration {
    json: bool,
    access_key_id: String,
    secret_access_key: String,
    region: String,
    topic_arn: String,
    client: reqwest::Client,
}

impl Integration {
    pub async fn new(conf: &AwsSnsConfiguration) -> Result<Integration> {
        trace!("Initializing AWS SNS integration");

        Ok(Integration {
            json: match Encoding::try_from(conf.encoding)
                .map_err(|_| anyhow!("Invalid encoding"))?
            {
                Encoding::Json => true,
                Encoding::Protobuf => false,
            },
            topic_arn: conf.topic_arn.clone(),
            access_key_id: conf.access_key_id.clone(),
            secret_access_key: conf.secret_access_key.clone(),
            region: conf.region.clone(),
            client: reqwest::Client::new(),
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

        let hostname = format!("sns.{}.amazonaws.com", self.region);
        let url = format!("https://{}/", hostname);
        let ts = chrono::Utc::now();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("host", hostname.parse()?);
        headers.insert(
            "X-Amz-Date",
            ts.format("%Y%m%dT%H%M%SZ").to_string().parse()?,
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse()?,
        );

        let body = [
            ("Action", "Publish"),
            ("TopicArn", &self.topic_arn),
            ("MessageAttributes.entry.1.Name", "event"),
            ("MessageAttributes.entry.1.Value.DataType", "String"),
            ("MessageAttributes.entry.1.Value.StringValue", event),
            ("MessageAttributes.entry.2.Name", "dev_eui"),
            ("MessageAttributes.entry.2.Value.DataType", "String"),
            ("MessageAttributes.entry.2.Value.StringValue", dev_eui),
            ("MessageAttributes.entry.3.Name", "application_id"),
            ("MessageAttributes.entry.3.Value.DataType", "String"),
            (
                "MessageAttributes.entry.3.Value.StringValue",
                application_id,
            ),
            ("Message", pl),
        ];
        let body = serde_urlencoded::to_string(body)?;

        let s = aws_sign_v4::AwsSign::new(
            "POST",
            &url,
            &ts,
            &headers,
            &self.region,
            &self.access_key_id,
            &self.secret_access_key,
            "sns",
            &body,
        )
        .sign();

        headers.insert(reqwest::header::AUTHORIZATION, s.parse()?);

        self.client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?
            .error_for_status()?;

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
