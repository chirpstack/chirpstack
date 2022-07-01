use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Client;
use serde::Serialize;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::PilotThingsConfiguration;
use chirpstack_api::integration;

pub struct Integration {
    timeout: Duration,
    server: String,
    token: String,
}

impl Integration {
    pub fn new(conf: &PilotThingsConfiguration) -> Integration {
        trace!("Initializing Pilot Things integration");

        Integration {
            timeout: Duration::from_secs(5),
            server: conf.server.clone(),
            token: conf.token.clone(),
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
        let endpoint = format!("{}/om2m/ipe-loraserver/up-link", self.server);

        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "up", endpoint = %endpoint, "Sending uplink event");

        let pl = UplinkPayload::from_uplink_event(pl);
        let b = serde_json::to_string(&pl)?;

        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let res = client
            .post(endpoint)
            .body(b)
            .query(&[("token", self.token.clone())])
            .headers(headers)
            .send()
            .await?;

        let _ = res.error_for_status()?;
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

#[derive(Serialize)]
struct UplinkPayload {
    #[serde(rename = "deviceName")]
    pub device_name: String,
    pub data: String,
    #[serde(rename = "devEUI")]
    pub dev_eui: String,
    #[serde(rename = "fPort")]
    pub f_port: u32,
    #[serde(rename = "devAddr")]
    pub dev_addr: String,
    #[serde(rename = "fcnt")]
    pub f_cnt: u32,
    pub metadata: Vec<UplinkMetadata>,
}

#[derive(Serialize)]
struct UplinkMetadata {
    pub rssi: i32,
    #[serde(rename = "lorasnr")]
    pub lora_snr: f32,
    #[serde(rename = "rfchain")]
    pub rf_chain: u32,
    pub antenna: u32,
    pub board: u32,
}

impl UplinkPayload {
    fn from_uplink_event(pl: &integration::UplinkEvent) -> Self {
        let di = pl.device_info.as_ref().unwrap();

        UplinkPayload {
            device_name: di.device_name.clone(),
            data: hex::encode(&pl.data),
            dev_eui: di.dev_eui.clone(),
            f_port: pl.f_port,
            dev_addr: pl.dev_addr.clone(),
            f_cnt: pl.f_cnt,
            metadata: pl
                .rx_info
                .iter()
                .map(|i| UplinkMetadata {
                    rssi: i.rssi,
                    lora_snr: i.snr,
                    rf_chain: 0,
                    antenna: i.antenna,
                    board: i.board,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chirpstack_api::gw;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_pilot_things() {
        let server = MockServer::start();

        let i = Integration {
            timeout: Duration::from_secs(5),
            server: server.url(""),
            token: "foo-token".into(),
        };

        // uplink
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/om2m/ipe-loraserver/up-link")
                .query_param("token", "foo-token")
                .header("Content-Type", "application/json")
                .body(r#"{"deviceName":"test-device","data":"01020304","devEUI":"0102030405060708","fPort":10,"devAddr":"04030201","fcnt":20,"metadata":[{"rssi":-10,"lorasnr":3.5,"rfchain":0,"antenna":1,"board":2}]}"#);

            then.status(200);
        });

        i.uplink_event(
            &HashMap::new(),
            &integration::UplinkEvent {
                device_info: Some(integration::DeviceInfo {
                    device_name: "test-device".into(),
                    dev_eui: "0102030405060708".into(),
                    ..Default::default()
                }),
                data: vec![1, 2, 3, 4],
                f_port: 10,
                f_cnt: 20,
                dev_addr: "04030201".into(),
                rx_info: vec![gw::UplinkRxInfo {
                    rssi: -10,
                    snr: 3.5,
                    antenna: 1,
                    board: 2,
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
