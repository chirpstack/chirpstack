use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;

use chirpstack_api::integration;

use super::Integration as IntegrationTrait;

lazy_static! {
    static ref UPLINK_EVENTS: RwLock<Vec<integration::UplinkEvent>> = RwLock::new(Vec::new());
    static ref JOIN_EVENTS: RwLock<Vec<integration::JoinEvent>> = RwLock::new(Vec::new());
    static ref ACK_EVENTS: RwLock<Vec<integration::AckEvent>> = RwLock::new(Vec::new());
    static ref TXACK_EVENTS: RwLock<Vec<integration::TxAckEvent>> = RwLock::new(Vec::new());
    static ref LOG_EVENTS: RwLock<Vec<integration::LogEvent>> = RwLock::new(Vec::new());
    static ref STATUS_EVENTS: RwLock<Vec<integration::StatusEvent>> = RwLock::new(Vec::new());
    static ref LOCATION_EVENTS: RwLock<Vec<integration::LocationEvent>> = RwLock::new(Vec::new());
    static ref INTEGRATION_EVENTS: RwLock<Vec<integration::IntegrationEvent>> =
        RwLock::new(Vec::new());
}

pub async fn reset() {
    UPLINK_EVENTS.write().await.drain(..);
    JOIN_EVENTS.write().await.drain(..);
    ACK_EVENTS.write().await.drain(..);
    TXACK_EVENTS.write().await.drain(..);
    LOG_EVENTS.write().await.drain(..);
    STATUS_EVENTS.write().await.drain(..);
    LOCATION_EVENTS.write().await.drain(..);
    INTEGRATION_EVENTS.write().await.drain(..);
}

pub struct Integration {}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        UPLINK_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        JOIN_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        ACK_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        TXACK_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        LOG_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        STATUS_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        LOCATION_EVENTS.write().await.push(pl.clone());
        Ok(())
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        INTEGRATION_EVENTS.write().await.push(pl.clone());
        Ok(())
    }
}

pub async fn get_join_event() -> Option<integration::JoinEvent> {
    if JOIN_EVENTS.read().await.is_empty() {
        return None;
    }

    JOIN_EVENTS
        .write()
        .await
        .drain(0..1)
        .collect::<Vec<integration::JoinEvent>>()
        .first()
        .cloned()
}

pub async fn get_uplink_event() -> Option<integration::UplinkEvent> {
    if UPLINK_EVENTS.read().await.is_empty() {
        return None;
    }

    UPLINK_EVENTS
        .write()
        .await
        .drain(0..1)
        .collect::<Vec<integration::UplinkEvent>>()
        .first()
        .cloned()
}

pub async fn get_ack_event() -> Option<integration::AckEvent> {
    if ACK_EVENTS.read().await.is_empty() {
        return None;
    }

    ACK_EVENTS
        .write()
        .await
        .drain(0..1)
        .collect::<Vec<integration::AckEvent>>()
        .first()
        .cloned()
}

pub async fn get_log_event() -> Option<integration::LogEvent> {
    if LOG_EVENTS.read().await.is_empty() {
        return None;
    }

    LOG_EVENTS
        .write()
        .await
        .drain(0..1)
        .collect::<Vec<integration::LogEvent>>()
        .first()
        .cloned()
}

pub async fn get_join_events() -> Vec<integration::JoinEvent> {
    JOIN_EVENTS.write().await.drain(..).collect()
}

pub async fn get_ack_events() -> Vec<integration::AckEvent> {
    ACK_EVENTS.write().await.drain(..).collect()
}

pub async fn get_txack_events() -> Vec<integration::TxAckEvent> {
    TXACK_EVENTS.write().await.drain(..).collect()
}

pub async fn get_log_events() -> Vec<integration::LogEvent> {
    LOG_EVENTS.write().await.drain(..).collect()
}

pub async fn get_status_events() -> Vec<integration::StatusEvent> {
    STATUS_EVENTS.write().await.drain(..).collect()
}

pub async fn get_location_events() -> Vec<integration::LocationEvent> {
    LOCATION_EVENTS.write().await.drain(..).collect()
}

pub async fn get_integration_events() -> Vec<integration::IntegrationEvent> {
    INTEGRATION_EVENTS.write().await.drain(..).collect()
}
