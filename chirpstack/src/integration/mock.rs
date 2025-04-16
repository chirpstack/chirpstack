use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;

use chirpstack_api::integration;

use super::Integration as IntegrationTrait;

static UPLINK_EVENTS: LazyLock<RwLock<Vec<integration::UplinkEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static JOIN_EVENTS: LazyLock<RwLock<Vec<integration::JoinEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static ACK_EVENTS: LazyLock<RwLock<Vec<integration::AckEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static TXACK_EVENTS: LazyLock<RwLock<Vec<integration::TxAckEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static LOG_EVENTS: LazyLock<RwLock<Vec<integration::LogEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static STATUS_EVENTS: LazyLock<RwLock<Vec<integration::StatusEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static LOCATION_EVENTS: LazyLock<RwLock<Vec<integration::LocationEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));
static INTEGRATION_EVENTS: LazyLock<RwLock<Vec<integration::IntegrationEvent>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

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
