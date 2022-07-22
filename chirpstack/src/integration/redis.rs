use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use prost::Message;
use tracing::info;

use super::Integration as IntegrationTrait;
use crate::eventlog;
use chirpstack_api::integration;

pub struct Integration {}

impl Integration {
    pub fn new() -> Integration {
        info!("Initializing Redis integration");
        Integration {}
    }
}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("up", &dev_info.dev_eui, &b).await
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("join", &dev_info.dev_eui, &b).await
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("ack", &dev_info.dev_eui, &b).await
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("txack", &dev_info.dev_eui, &b).await
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("log", &dev_info.dev_eui, &b).await
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("status", &dev_info.dev_eui, &b).await
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("location", &dev_info.dev_eui, &b).await
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;
        let b = pl.encode_to_vec();
        eventlog::log_event_for_device("integration", &dev_info.dev_eui, &b).await
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage::get_redis_conn;
    use crate::test;
    use chirpstack_api::integration;
    use redis::streams::StreamReadReply;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_redis() {
        let _guard = test::prepare().await;
        let i = Integration::new();
        let mut last_id = "0".to_string();

        // uplink
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "up", &pl.encode_to_vec());

        // join
        let pl = integration::JoinEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.join_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "join", &pl.encode_to_vec());

        // ack
        let pl = integration::AckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.ack_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "ack", &pl.encode_to_vec());

        // txack
        let pl = integration::TxAckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.txack_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "txack", &pl.encode_to_vec());

        // log
        let pl = integration::LogEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.log_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "log", &pl.encode_to_vec());

        // status
        let pl = integration::StatusEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.status_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "status", &pl.encode_to_vec());

        // location
        let pl = integration::LocationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.location_event(&HashMap::new(), &pl).await.unwrap();
        last_id = assert_reply(&last_id, "location", &pl.encode_to_vec());

        // integration
        let pl = integration::IntegrationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.integration_event(&HashMap::new(), &pl).await.unwrap();
        let _ = assert_reply(&last_id, "integration", &pl.encode_to_vec());
    }

    fn assert_reply(last_id: &str, event: &str, b: &[u8]) -> String {
        let mut c = get_redis_conn().unwrap();
        let srr: StreamReadReply = redis::cmd("XREAD")
            .arg("COUNT")
            .arg(1 as usize)
            .arg("STREAMS")
            .arg("device:stream:event")
            .arg(&last_id)
            .query(&mut *c)
            .unwrap();
        assert_eq!(1, srr.keys.len());

        let stream_key = &srr.keys[0];
        assert_eq!(1, stream_key.ids.len());
        let stream_id = &stream_key.ids[0];

        let v = stream_id.map.get(event).unwrap();
        assert_eq!(&redis::Value::Data(b.to_vec()), v);

        stream_id.id.clone()
    }
}
