use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{Context, Result};
use async_trait::async_trait;
use base64::prelude::*;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::Serialize;
use tracing::{info, trace};
use url::Url;

use super::Integration as IntegrationTrait;
use crate::storage::application::BlynkConfiguration;
use chirpstack_api::integration;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> Client {
    CLIENT
        .get_or_init(|| {
            Client::builder()
                .timeout(Duration::from_secs(5))
                .use_rustls_tls()
                .build()
                .unwrap()
        })
        .clone()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
struct DeviceInfo {
    pub tenant_id: String,
    pub tenant_name: String,
    pub application_id: String,
    pub application_name: String,
    pub device_profile_id: String,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub device_class_enabled: String,
    pub tags: HashMap<String, String>,
}

impl From<integration::DeviceInfo> for DeviceInfo {
    fn from(value: integration::DeviceInfo) -> Self {
        DeviceInfo {
            tenant_id: value.tenant_id.clone(),
            tenant_name: value.tenant_name.clone(),
            application_id: value.application_id.clone(),
            application_name: value.application_name.clone(),
            device_profile_id: value.device_profile_id.clone(),
            device_profile_name: value.device_profile_name.clone(),
            device_name: value.device_name.clone(),
            dev_eui: value.dev_eui.clone(),
            device_class_enabled: value.device_class_enabled().as_str_name().into(),
            tags: value.tags.clone(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
struct UplinkEvent {
    pub device_info: Option<DeviceInfo>,
    pub time: Option<pbjson_types::Timestamp>,
    pub object: Option<pbjson_types::Struct>,
}

impl From<integration::UplinkEvent> for UplinkEvent {
    fn from(value: integration::UplinkEvent) -> Self {
        UplinkEvent {
            device_info: value.device_info.map(|v| v.into()),
            time: value.time.clone(),
            object: value.object.clone(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
struct JoinEvent {
    pub device_info: Option<DeviceInfo>,
    pub time: Option<pbjson_types::Timestamp>,
}

impl From<integration::JoinEvent> for JoinEvent {
    fn from(value: integration::JoinEvent) -> Self {
        JoinEvent {
            device_info: value.device_info.map(|v| v.into()),
            time: value.time.clone(),
        }
    }
}

pub struct Integration {
    token: String,
}

impl Integration {
    pub fn new(conf: &BlynkConfiguration) -> Integration {
        trace!("Initializing Blynk integration");

        Integration {
            token: conf.token.clone(),
        }
    }

    fn parse_token(&self) -> Result<(String, String)> {
        let token_b = BASE64_STANDARD.decode(&self.token).context("Parse token")?;
        let token_str = String::from_utf8(token_b).context("Parse token")?;
        let token_url = Url::parse(&token_str).context("Parse token")?;

        let integration_url = format!(
            "{}://{}{}{}",
            token_url.scheme(),
            token_url.host_str().unwrap_or_default(),
            token_url
                .port()
                .map(|v| format!(":{}", v))
                .unwrap_or_default(),
            token_url.path()
        );
        let params: HashMap<String, String> = token_url.query_pairs().into_owned().collect();
        Ok((
            integration_url,
            params.get("token").cloned().unwrap_or_default(),
        ))
    }

    async fn post_event<T>(&self, event: &str, pl: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let (integration_url, integration_token) = self.parse_token()?;
        let b = serde_json::to_vec(pl)?;

        info!(event = %event, url = %integration_url, "Posting event");

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", integration_token).parse().unwrap(),
        );

        get_client()
            .post(&integration_url)
            .body(b.to_vec())
            .query(&[("event", event)])
            .headers(headers)
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
        self.post_event("up", &UplinkEvent::from(pl.clone())).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        self.post_event("join", &JoinEvent::from(pl.clone())).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::AckEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::TxAckEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::LogEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::StatusEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::LocationEvent,
    ) -> Result<()> {
        Ok(())
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_blynk() {
        let server = MockServer::start();
        let mut token = String::new();
        BASE64_STANDARD.encode_string(
            format!("{}?token=my-secret-token", server.url("/")),
            &mut token,
        );

        let i = Integration { token };

        // uplink event
        let pl: integration::UplinkEvent = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: "app_id".into(),
                application_name: "app_name".into(),
                dev_eui: "0102030405060708".into(),
                device_name: "dev_name".into(),
                device_profile_id: "dp_id".into(),
                device_profile_name: "dp_name".into(),
                tenant_id: "t_id".into(),
                tenant_name: "t_name".into(),
                device_class_enabled: 0,
                tags: HashMap::new(),
            }),
            object: Some(pbjson_types::Struct::default()),
            time: Some(pbjson_types::Timestamp::default()),
            ..Default::default()
        };

        let pl_expected = UplinkEvent {
            device_info: Some(DeviceInfo {
                application_id: "app_id".into(),
                application_name: "app_name".into(),
                dev_eui: "0102030405060708".into(),
                device_name: "dev_name".into(),
                device_profile_id: "dp_id".into(),
                device_profile_name: "dp_name".into(),
                tenant_id: "t_id".into(),
                tenant_name: "t_name".into(),
                device_class_enabled: "CLASS_A".into(),
                tags: HashMap::new(),
            }),
            object: Some(pbjson_types::Struct::default()),
            time: Some(pbjson_types::Timestamp::default()),
        };

        let pl_test: UplinkEvent = pl.clone().into();
        assert_eq!(pl_expected, pl_test);

        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "up")
                .header("Authorization", "Bearer my-secret-token")
                .body(serde_json::to_string(&pl_expected).unwrap());

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();

        mock.assert();
        mock.delete();

        // join event
        let pl: integration::JoinEvent = integration::JoinEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: "app_id".into(),
                application_name: "app_name".into(),
                dev_eui: "0102030405060708".into(),
                device_name: "dev_name".into(),
                device_profile_id: "dp_id".into(),
                device_profile_name: "dp_name".into(),
                tenant_id: "t_id".into(),
                tenant_name: "t_name".into(),
                device_class_enabled: 0,
                tags: HashMap::new(),
            }),
            time: Some(pbjson_types::Timestamp::default()),
            ..Default::default()
        };

        let pl_expected = JoinEvent {
            device_info: Some(DeviceInfo {
                application_id: "app_id".into(),
                application_name: "app_name".into(),
                dev_eui: "0102030405060708".into(),
                device_name: "dev_name".into(),
                device_profile_id: "dp_id".into(),
                device_profile_name: "dp_name".into(),
                tenant_id: "t_id".into(),
                tenant_name: "t_name".into(),
                device_class_enabled: "CLASS_A".into(),
                tags: HashMap::new(),
            }),
            time: Some(pbjson_types::Timestamp::default()),
        };

        let pl_test: JoinEvent = pl.clone().into();
        assert_eq!(pl_expected, pl_test);

        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .query_param("event", "join")
                .header("Authorization", "Bearer my-secret-token")
                .body(serde_json::to_string(&pl_expected).unwrap());

            then.status(200);
        });

        i.join_event(&HashMap::new(), &pl).await.unwrap();

        mock.assert();
        mock.delete();
    }
}
