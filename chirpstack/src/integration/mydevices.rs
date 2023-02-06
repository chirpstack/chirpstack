use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Client;
use serde::Serialize;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::MyDevicesConfiguration;
use chirpstack_api::integration;

#[derive(Serialize)]
struct UplinkPayload {
    #[serde(rename = "correlationID")]
    pub correlation_id: String,
    #[serde(rename = "devEUI")]
    pub dev_eui: String,
    pub data: String, // base64
    #[serde(rename = "fCnt")]
    pub f_cnt: u32,
    #[serde(rename = "fPort")]
    pub f_port: u32,
    #[serde(rename = "rxInfo")]
    rx_info: Vec<RxInfo>,
    #[serde(rename = "txInfo")]
    tx_info: TxInfo,
}

impl UplinkPayload {
    fn from_uplink_event(pl: &integration::UplinkEvent) -> Self {
        let di = pl.device_info.as_ref().unwrap();

        UplinkPayload {
            correlation_id: pl.deduplication_id.clone(),
            dev_eui: di.dev_eui.clone(),
            data: general_purpose::STANDARD.encode(&pl.data),
            f_cnt: pl.f_cnt,
            f_port: pl.f_port,
            rx_info: pl
                .rx_info
                .iter()
                .map(|i| RxInfo {
                    gateway_id: i.gateway_id.clone(),
                    rssi: i.rssi,
                    lora_snr: i.snr,
                    location: match &i.location {
                        Some(v) => Location {
                            latitude: v.latitude,
                            longitude: v.longitude,
                        },
                        None => Location {
                            latitude: 0.0,
                            longitude: 0.0,
                        },
                    },
                })
                .collect(),
            tx_info: TxInfo {
                frequency: pl.tx_info.as_ref().unwrap().frequency,
            },
        }
    }
}

#[derive(Serialize)]
struct RxInfo {
    #[serde(rename = "gatewayID")]
    pub gateway_id: String,
    pub rssi: i32,
    #[serde(rename = "loRaSNR")]
    pub lora_snr: f32,
    pub location: Location,
}

#[derive(Serialize)]
struct TxInfo {
    pub frequency: u32,
}

#[derive(Serialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

pub struct Integration {
    timeout: Duration,
    endpoint: String,
}

impl Integration {
    pub fn new(conf: &MyDevicesConfiguration) -> Integration {
        trace!("Initializing myDevices integration");
        Integration {
            timeout: Duration::from_secs(5),
            endpoint: conf.endpoint.clone(),
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
        if pl.f_port == 0 {
            return Ok(());
        }

        let di = pl.device_info.as_ref().unwrap();

        info!(dev_eui = %di.dev_eui, event = "up", endpoint = %self.endpoint, "Publishing event");

        let pl = UplinkPayload::from_uplink_event(pl);
        let b = serde_json::to_string(&pl)?;

        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let req = client
            .post(&self.endpoint)
            .body(b)
            .headers(headers)
            .send()
            .await?;

        let _ = req.error_for_status()?;
        Ok(())
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        _pl: &integration::JoinEvent,
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
pub mod test {
    use super::*;
    use chirpstack_api::{common, gw};
    use httpmock::prelude::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mydevices() {
        let server = MockServer::start();

        let i = Integration {
            timeout: Duration::from_secs(5),
            endpoint: server.url("/"),
        };

        // uplink
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/")
                .header("Content-Type", "application/json")
                .body(r#"{"correlationID":"00000000-0000-0000-0000-000000000000","devEUI":"0102030405060708","data":"AQID","fCnt":10,"fPort":20,"rxInfo":[{"gatewayID":"0807060504030201","rssi":20,"loRaSNR":5.0,"location":{"latitude":1.123,"longitude":2.123}}],"txInfo":{"frequency":868100000}}"#);

            then.status(200);
        });

        i.uplink_event(
            &HashMap::new(),
            &integration::UplinkEvent {
                deduplication_id: Uuid::nil().to_string(),
                device_info: Some(integration::DeviceInfo {
                    dev_eui: "0102030405060708".into(),
                    ..Default::default()
                }),
                data: vec![0x01, 0x02, 0x03],
                f_cnt: 10,
                f_port: 20,
                tx_info: Some(gw::UplinkTxInfo {
                    frequency: 868100000,
                    ..Default::default()
                }),
                rx_info: vec![gw::UplinkRxInfo {
                    gateway_id: "0807060504030201".to_string(),
                    rssi: 20,
                    snr: 5.0,
                    location: Some(common::Location {
                        latitude: 1.123,
                        longitude: 2.123,
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
        .unwrap();

        mock.assert();
        mock.delete();
    }
}
