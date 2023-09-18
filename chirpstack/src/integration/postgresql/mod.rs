use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::task;
use tracing::info;
use uuid::Uuid;

use super::Integration as IntegrationTrait;
use crate::config::PostgresqlIntegration as Config;
use chirpstack_api::integration;
use schema::{
    event_ack, event_integration, event_join, event_location, event_log, event_status,
    event_tx_ack, event_up,
};

mod schema;

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./src/integration/postgresql/migrations");

type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Insertable)]
#[diesel(table_name = event_up)]
struct EventUp {
    pub deduplication_id: Uuid,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub dev_addr: String,
    pub adr: bool,
    pub dr: i16,
    pub f_cnt: i64,
    pub f_port: i16,
    pub confirmed: bool,
    pub data: Vec<u8>,
    pub object: serde_json::Value,
    pub rx_info: serde_json::Value,
    pub tx_info: serde_json::Value,
}

#[derive(Insertable)]
#[diesel(table_name = event_join)]
struct EventJoin {
    pub deduplication_id: Uuid,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub dev_addr: String,
}

#[derive(Insertable)]
#[diesel(table_name = event_ack)]
struct EventAck {
    pub queue_item_id: Uuid,
    pub deduplication_id: Uuid,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub acknowledged: bool,
    pub f_cnt_down: i64,
}

#[derive(Insertable)]
#[diesel(table_name = event_tx_ack)]
struct EventTxAck {
    pub queue_item_id: Uuid,
    pub downlink_id: i64,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub f_cnt_down: i64,
    pub gateway_id: String,
    pub tx_info: serde_json::Value,
}

#[derive(Insertable)]
#[diesel(table_name = event_log)]
struct EventLog {
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub level: String,
    pub code: String,
    pub description: String,
    pub context: serde_json::Value,
}

#[derive(Insertable)]
#[diesel(table_name = event_status)]
struct EventStatus {
    pub deduplication_id: Uuid,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub margin: i16,
    pub external_power_source: bool,
    pub battery_level_unavailable: bool,
    pub battery_level: f32,
}

#[derive(Insertable)]
#[diesel(table_name = event_location)]
struct EventLocation {
    pub deduplication_id: Uuid,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub source: String,
    pub accuracy: f32,
}

#[derive(Insertable)]
#[diesel(table_name = event_integration)]
struct EventIntegration {
    pub deduplication_id: Uuid,
    pub time: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub application_id: Uuid,
    pub application_name: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub device_name: String,
    pub dev_eui: String,
    pub tags: serde_json::Value,
    pub integration_name: String,
    pub event_type: String,
    pub object: serde_json::Value,
}

pub struct Integration {
    pg_pool: PgPool,
}

impl Integration {
    pub fn new(conf: &Config) -> Result<Integration> {
        info!("Initializing PostgreSQL integration");

        let pg_pool = PgPool::builder()
            .max_size(conf.max_open_connections)
            .min_idle(match conf.min_idle_connections {
                0 => None,
                _ => Some(conf.min_idle_connections),
            })
            .build(ConnectionManager::new(&conf.dsn))
            .context("Setup PostgreSQL connection pool error")?;
        let mut db_conn = pg_pool.get()?;

        info!("Applying schema migrations");
        db_conn
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("{}", e))?;

        Ok(Integration { pg_pool })
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
        info!(dev_eui = %di.dev_eui, event =  "up", "Inserting event");

        let e = EventUp {
            deduplication_id: Uuid::from_str(&pl.deduplication_id)?,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            dev_addr: pl.dev_addr.clone(),
            adr: pl.adr,
            dr: pl.dr as i16,
            f_cnt: pl.f_cnt as i64,
            f_port: pl.f_port as i16,
            confirmed: pl.confirmed,
            data: pl.data.clone(),
            object: serde_json::to_value(&pl.object)?,
            rx_info: serde_json::to_value(&pl.rx_info)?,
            tx_info: serde_json::to_value(&pl.tx_info)?,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_up::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn join_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "join", "Inserting event");

        let e = EventJoin {
            deduplication_id: Uuid::from_str(&pl.deduplication_id)?,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            dev_addr: pl.dev_addr.clone(),
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_join::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn ack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "ack", "Inserting event");

        let e = EventAck {
            queue_item_id: Uuid::from_str(&pl.queue_item_id)?,
            deduplication_id: Uuid::from_str(&pl.deduplication_id)?,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            acknowledged: pl.acknowledged,
            f_cnt_down: pl.f_cnt_down as i64,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_ack::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn txack_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "txack", "Inserting event");

        let e = EventTxAck {
            queue_item_id: Uuid::from_str(&pl.queue_item_id)?,
            downlink_id: pl.downlink_id as i64,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            f_cnt_down: pl.f_cnt_down as i64,
            gateway_id: pl.gateway_id.clone(),
            tx_info: serde_json::to_value(&pl.tx_info)?,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_tx_ack::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn log_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "log", "Inserting event");

        let e = EventLog {
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            level: pl.level.to_string(),
            code: pl.code.to_string(),
            description: pl.description.clone(),
            context: serde_json::to_value(&pl.context)?,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_log::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn status_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "status", "Inserting event");

        let e = EventStatus {
            deduplication_id: Uuid::from_str(&pl.deduplication_id)?,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            margin: pl.margin as i16,
            external_power_source: pl.external_power_source,
            battery_level_unavailable: pl.battery_level_unavailable,
            battery_level: pl.battery_level,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_status::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;
        Ok(())
    }

    async fn location_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let loc = pl.location.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "location", "Inserting event");

        let e = EventLocation {
            deduplication_id: Uuid::from_str(&pl.deduplication_id)?,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            latitude: loc.latitude,
            longitude: loc.longitude,
            altitude: loc.altitude,
            source: loc.source.to_string(),
            accuracy: loc.accuracy,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_location::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn integration_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        info!(dev_eui = %di.dev_eui, event = "integration", "Inserting event");

        let e = EventIntegration {
            deduplication_id: Uuid::from_str(&pl.deduplication_id)?,
            time: pl
                .time
                .as_ref()
                .unwrap()
                .clone()
                .try_into()
                .map_err(anyhow::Error::msg)?,
            tenant_id: Uuid::from_str(&di.tenant_id)?,
            tenant_name: di.tenant_name.clone(),
            application_id: Uuid::from_str(&di.application_id)?,
            application_name: di.application_name.clone(),
            device_profile_id: Uuid::from_str(&di.device_profile_id)?,
            device_profile_name: di.device_profile_name.clone(),
            device_name: di.device_name.clone(),
            dev_eui: di.dev_eui.clone(),
            tags: serde_json::to_value(&di.tags)?,
            integration_name: pl.integration_name.clone(),
            event_type: pl.event_type.clone(),
            object: serde_json::to_value(&pl.object)?,
        };
        let mut c = self.pg_pool.get()?;

        task::spawn_blocking(move || -> Result<()> {
            diesel::insert_into(event_integration::table)
                .values(&e)
                .execute(&mut c)?;
            Ok(())
        })
        .await??;

        Ok(())
    }
}
