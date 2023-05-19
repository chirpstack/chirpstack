use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info, trace};

use super::Integration as IntegrationTrait;
use crate::codec;
use crate::storage::application::IftttConfiguration;
use chirpstack_api::integration;

#[derive(Serialize, Deserialize)]
struct Values {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value1: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value2: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value3: String,
}

pub struct Integration {
    key: String,
    uplink_values: [String; 2],
    server: String,
    arbitrary_json: bool,
    event_prefix: String,
}

impl Integration {
    pub fn new(conf: &IftttConfiguration) -> Integration {
        trace!("Initializing ifttt integration");

        Integration {
            key: conf.key.clone(),
            uplink_values: conf.uplink_values.clone(),
            server: "https://maker.ifttt.com".to_string(),
            arbitrary_json: conf.arbitrary_json,
            event_prefix: conf.event_prefix.clone(),
        }
    }

    async fn post<T>(&self, event: &str, v: &T) -> Result<()>
    where
        T: Serialize,
    {
        let event = if self.event_prefix.is_empty() {
            event.to_string()
        } else {
            format!("{}_{}", self.event_prefix, event)
        };

        let url = if self.arbitrary_json {
            format!(
                "{}/trigger/{}/json/with/key/{}",
                self.server, event, self.key
            )
        } else {
            format!("{}/trigger/{}/with/key/{}", self.server, event, self.key)
        };

        let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        info!(event = %event, "Sending event to IFTTT");
        let res = client.post(url).json(&v).headers(headers).send().await?;
        match res.error_for_status() {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(event = %event, error = %e, "Sending event to IFTTT failed");
                Err(anyhow::Error::new(e))
            }
        }
    }
}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        if self.arbitrary_json {
            return self.post("up", pl).await;
        }

        let data_measurements: HashMap<String, pbjson_types::value::Kind> = match &pl.object {
            None => {
                trace!("object is None, nothing to send");
                return Ok(());
            }
            Some(v) => codec::get_measurements(v),
        };

        let v = Values {
            value1: pl.device_info.as_ref().unwrap().dev_eui.clone(),
            value2: match data_measurements.get(&self.uplink_values[0]) {
                Some(v) => kind_to_string(v),
                None => "".to_string(),
            },
            value3: match data_measurements.get(&self.uplink_values[1]) {
                Some(v) => kind_to_string(v),
                None => "".to_string(),
            },
        };

        // Nothing to do.
        if v.value1.is_empty() && v.value2.is_empty() && v.value3.is_empty() {
            trace!("All values are empty, nothing to send");
            return Ok(());
        }

        self.post("up", &v).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::JoinEvent,
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

fn kind_to_string(k: &pbjson_types::value::Kind) -> String {
    match k {
        pbjson_types::value::Kind::NullValue(_) => "".to_string(),
        pbjson_types::value::Kind::NumberValue(v) => v.to_string(),
        pbjson_types::value::Kind::StringValue(v) => v.clone(),
        pbjson_types::value::Kind::BoolValue(v) => v.to_string(),
        pbjson_types::value::Kind::StructValue(_) | pbjson_types::value::Kind::ListValue(_) => {
            // this should not happen
            "".to_string()
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_ifttt_no_prefix() {
        let server = MockServer::start();

        let i = Integration {
            key: "verysecret".into(),
            uplink_values: ["temp".to_string(), "door_open".to_string()],
            server: server.url(""),
            arbitrary_json: false,
            event_prefix: "".into(),
        };

        // uplink event
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            object: Some(pbjson_types::Struct {
                fields: [
                    (
                        "temp".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::NumberValue(23.5)),
                        },
                    ),
                    (
                        "door_open".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue(
                                "closed".to_string(),
                            )),
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/trigger/up/with/key/verysecret")
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&Values {
                        value1: "0102030405060708".to_string(),
                        value2: "23.5".to_string(),
                        value3: "closed".to_string(),
                    })
                    .unwrap(),
                );

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();
    }

    #[tokio::test]
    async fn test_ifttt_prefix() {
        let server = MockServer::start();

        let i = Integration {
            key: "verysecret".into(),
            uplink_values: ["temp".to_string(), "door_open".to_string()],
            server: server.url(""),
            arbitrary_json: false,
            event_prefix: "weatherstation".into(),
        };

        // uplink event
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            object: Some(pbjson_types::Struct {
                fields: [
                    (
                        "temp".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::NumberValue(23.5)),
                        },
                    ),
                    (
                        "door_open".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue(
                                "closed".to_string(),
                            )),
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/trigger/weatherstation_up/with/key/verysecret")
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&Values {
                        value1: "0102030405060708".to_string(),
                        value2: "23.5".to_string(),
                        value3: "closed".to_string(),
                    })
                    .unwrap(),
                );

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();
    }

    #[tokio::test]
    async fn test_ifttt_arbitrary_json() {
        let server = MockServer::start();

        let i = Integration {
            key: "verysecret".into(),
            uplink_values: ["temp".to_string(), "door_open".to_string()],
            server: server.url(""),
            arbitrary_json: true,
            event_prefix: "".into(),
        };

        // uplink event
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            object: Some(pbjson_types::Struct {
                fields: [
                    (
                        "temp".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::NumberValue(23.5)),
                        },
                    ),
                    (
                        "door_open".to_string(),
                        pbjson_types::Value {
                            kind: Some(pbjson_types::value::Kind::StringValue(
                                "closed".to_string(),
                            )),
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            }),
            ..Default::default()
        };
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/trigger/up/json/with/key/verysecret")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&pl).unwrap());

            then.status(200);
        });

        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        mock.assert();
        mock.delete();
    }
}
