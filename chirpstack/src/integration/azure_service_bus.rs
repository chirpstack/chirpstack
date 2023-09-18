use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use prost::Message;
use reqwest::header::{HeaderMap, HeaderName, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use sha2::Sha256;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::AzureServiceBusConfiguration;
use chirpstack_api::api::Encoding;
use chirpstack_api::integration;

pub struct Integration {
    timeout: Duration,
    json: bool,
    uri: String,
    key_name: String,
    key: String,
}

impl Integration {
    pub fn new(conf: &AzureServiceBusConfiguration) -> Result<Integration> {
        trace!("Initializing Azure Service-Bus integration");

        let kv = parse_connection_string(&conf.connection_string);

        Ok(Integration {
            timeout: Duration::from_secs(5),
            json: match Encoding::try_from(conf.encoding)
                .map_err(|_| anyhow!("Invalid encoding"))?
            {
                Encoding::Json => true,
                Encoding::Protobuf => false,
            },
            uri: format!(
                "https://{}{}",
                kv.get("Endpoint")
                    .cloned()
                    .unwrap_or_default()
                    .replace("sb://", ""),
                conf.publish_name
            ),
            key_name: kv.get("SharedAccessKeyName").cloned().unwrap_or_default(),
            key: kv.get("SharedAccessKey").cloned().unwrap_or_default(),
        })
    }

    async fn publish(
        &self,
        event: &str,
        application_id: &str,
        dev_eui: &str,
        pl: &str,
    ) -> Result<()> {
        let token = create_sas_token(
            &self.uri,
            &self.key_name,
            &self.key,
            &(SystemTime::now() + Duration::from_secs(60 * 5)),
        )?;

        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, token.parse()?);
        if self.json {
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        } else {
            headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
        }

        headers.insert(
            HeaderName::try_from("event").unwrap(),
            format!("\"{}\"", event).parse()?,
        );
        headers.insert(
            HeaderName::try_from("application_id").unwrap(),
            format!("\"{}\"", application_id).parse()?,
        );
        headers.insert(
            HeaderName::try_from("dev_eui").unwrap(),
            format!("\"{}\"", dev_eui).parse()?,
        );

        info!(event = %event, dev_eui = %dev_eui, "Publishing event");
        let res = client
            .post(format!("{}/messages", self.uri))
            .body(pl.to_string())
            .headers(headers)
            .send()
            .await?;

        let _ = res.error_for_status()?;
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

type HmacSha256 = Hmac<Sha256>;

fn create_sas_token(
    uri: &str,
    key_name: &str,
    key: &str,
    expiration: &SystemTime,
) -> Result<String> {
    let encoded_url = urlencoding::encode(uri);
    let exp = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();

    let sig = format!("{}\n{}", encoded_url, exp);
    let mut m = HmacSha256::new_from_slice(key.as_bytes())?;
    m.update(sig.as_bytes());
    let result = general_purpose::STANDARD.encode(m.finalize().into_bytes());

    let hash = urlencoding::encode(&result);

    Ok(format!(
        "SharedAccessSignature sig={}&se={}&skn={}&sr={}",
        hash, exp, key_name, encoded_url
    ))
}

fn parse_connection_string(s: &str) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();

    for pair in s.split(';') {
        let kv: Vec<&str> = pair.splitn(2, '=').collect();
        if kv.len() != 2 {
            continue;
        }

        out.insert(kv[0].to_string(), kv[1].to_string());
    }

    out
}

#[cfg(test)]
pub mod test {
    use super::*;
    use httpmock::prelude::*;
    use std::time::Duration;
    use uuid::Uuid;

    #[test]
    fn test_create_sas_token() {
        let exp = UNIX_EPOCH + Duration::from_secs(10);
        let token = create_sas_token(
            "https://chirpstack-tst.servicebus.windows.net/",
            "MyKey",
            "AQID",
            &exp,
        )
        .unwrap();

        assert_eq!("SharedAccessSignature sig=VPMESaZwz0wSdvzJXET0DgZMBpKh95yjP988pUt6Qo4%3D&se=10&skn=MyKey&sr=https%3A%2F%2Fchirpstack-tst.servicebus.windows.net%2F", token);
    }

    #[test]
    fn test_parse_connection_string() {
        let kv = parse_connection_string("Endpoint=sb://chirpstack-tst.servicebus.windows.net/;SharedAccessKeyName=TestKeyName;SharedAccessKey=TestKey");
        let expected: HashMap<String, String> = [
            (
                "Endpoint".to_string(),
                "sb://chirpstack-tst.servicebus.windows.net/".to_string(),
            ),
            ("SharedAccessKeyName".to_string(), "TestKeyName".to_string()),
            ("SharedAccessKey".to_string(), "TestKey".to_string()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(expected, kv);
    }

    #[tokio::test]
    async fn test_influxdb() {
        let server = MockServer::start();

        let i = Integration {
            timeout: Duration::from_secs(5),
            json: true,
            uri: server.url(""),
            key_name: "key-name".to_string(),
            key: "foo-key".to_string(),
        };

        // uplink
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"up\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // join
        let pl = integration::JoinEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"join\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.join_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // ack
        let pl = integration::AckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"ack\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.ack_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // txack
        let pl = integration::TxAckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"txack\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.txack_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // log
        let pl = integration::LogEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"log\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.log_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // status
        let pl = integration::StatusEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"status\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.status_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // location
        let pl = integration::LocationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"location\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.location_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();

        // integration
        let pl = integration::IntegrationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/messages")
                .header_exists("Authorization")
                .header("Content-Type", "application/json")
                .header("event", "\"integration\"")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.integration_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();
    }
}
