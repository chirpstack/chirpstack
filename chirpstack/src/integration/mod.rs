use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::future::join_all;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

use crate::storage::{application, device, device_profile, device_queue};
use crate::{codec, config};
use chirpstack_api::integration;
use lrwn::EUI64;

mod amqp;
mod aws_sns;
mod azure_service_bus;
mod gcp_pub_sub;
mod http;
mod ifttt;
mod influxdb;
mod kafka;
mod loracloud;
#[cfg(test)]
pub mod mock;
mod mqtt;
mod mydevices;
mod pilot_things;
mod postgresql;
mod redis;
mod thingsboard;

lazy_static! {
    static ref GLOBAL_INTEGRATIONS: RwLock<Vec<Box<dyn Integration + Sync + Send>>> =
        RwLock::new(Vec::new());
    static ref MOCK_INTEGRATION: RwLock<bool> = RwLock::new(false);
}

pub async fn setup() -> Result<()> {
    info!("Setting up global integrations");
    let conf = config::get();
    let mut integrations = GLOBAL_INTEGRATIONS.write().await;

    integrations.push(Box::new(redis::Integration::new()));

    for name in &conf.integration.enabled {
        match name.as_ref() {
            "mqtt" => {
                integrations.push(Box::new(
                    mqtt::Integration::new(&conf.integration.mqtt)
                        .await
                        .context("Setup MQTT integration")?,
                ));
            }
            "postgresql" => integrations.push(Box::new(
                postgresql::Integration::new(&conf.integration.postgresql)
                    .context("Setup PostgreSQL integration")?,
            )),
            "amqp" => integrations.push(Box::new(
                amqp::Integration::new(&conf.integration.amqp)
                    .await
                    .context("Setup AMQP integration")?,
            )),
            "kafka" => integrations.push(Box::new(
                kafka::Integration::new(&conf.integration.kafka)
                    .context("Setup Kafka integration")?,
            )),
            _ => {
                return Err(anyhow!("Unexpected integration: {}", name));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
pub async fn set_mock() {
    let mut m = MOCK_INTEGRATION.write().await;
    *m = true;
}

#[async_trait]
pub trait Integration {
    async fn uplink_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()>;

    async fn join_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::JoinEvent,
    ) -> Result<()>;

    async fn ack_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::AckEvent,
    ) -> Result<()>;

    async fn txack_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::TxAckEvent,
    ) -> Result<()>;

    async fn log_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::LogEvent,
    ) -> Result<()>;

    async fn status_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()>;

    async fn location_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()>;

    async fn integration_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::IntegrationEvent,
    ) -> Result<()>;
}

// Returns a Vec of integrations for the given Application ID.
async fn for_application_id(id: Uuid) -> Result<Vec<Box<dyn Integration + Sync + Send>>> {
    #[cfg(test)]
    {
        let m = MOCK_INTEGRATION.read().await;
        if *m {
            return Ok(vec![Box::new(mock::Integration {})]);
        }
    }

    let mut out: Vec<Box<dyn Integration + Sync + Send>> = Vec::new();
    let integrations = application::get_integrations_for_application(&id).await?;

    for app_i in &integrations {
        out.push(match &app_i.configuration {
            application::IntegrationConfiguration::AwsSns(conf) => {
                Box::new(aws_sns::Integration::new(conf).await?)
            }
            application::IntegrationConfiguration::AzureServiceBus(conf) => {
                Box::new(azure_service_bus::Integration::new(conf)?)
            }
            application::IntegrationConfiguration::GcpPubSub(conf) => {
                Box::new(gcp_pub_sub::Integration::new(conf).await?)
            }
            application::IntegrationConfiguration::Http(conf) => {
                Box::new(http::Integration::new(conf))
            }
            application::IntegrationConfiguration::InfluxDb(conf) => {
                Box::new(influxdb::Integration::new(conf)?)
            }
            application::IntegrationConfiguration::LoraCloud(conf) => {
                Box::new(loracloud::Integration::new(conf))
            }
            application::IntegrationConfiguration::MyDevices(conf) => {
                Box::new(mydevices::Integration::new(conf))
            }
            application::IntegrationConfiguration::PilotThings(conf) => {
                Box::new(pilot_things::Integration::new(conf))
            }
            application::IntegrationConfiguration::ThingsBoard(conf) => {
                Box::new(thingsboard::Integration::new(conf))
            }
            application::IntegrationConfiguration::Ifttt(conf) => {
                Box::new(ifttt::Integration::new(conf))
            }
            _ => {
                continue;
            }
        })
    }

    Ok(out)
}

pub async fn uplink_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::UplinkEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _uplink_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Uplink event error");
            }
        }
    });
}

async fn _uplink_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::UplinkEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].uplink_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].uplink_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn join_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::JoinEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _join_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Join event error");
            }
        }
    });
}

async fn _join_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::JoinEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].join_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].join_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn ack_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::AckEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _ack_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Ack event error");
            }
        }
    });
}

async fn _ack_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::AckEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].ack_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].ack_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn txack_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::TxAckEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _txack_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Txack event error");
            }
        }
    });
}

async fn _txack_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::TxAckEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].txack_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].txack_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn log_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::LogEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _log_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Log event error");
            }
        }
    });
}

async fn _log_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::LogEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].log_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].log_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn status_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::StatusEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _status_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Status event error");
            }
        }
    });
}

async fn _status_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::StatusEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].status_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].status_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn location_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::LocationEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _location_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Location event error");
            }
        }
    });
}

async fn _location_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::LocationEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].location_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].location_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

pub async fn integration_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::IntegrationEvent,
) {
    tokio::spawn({
        let vars = vars.clone();
        let pl = pl.clone();

        async move {
            if let Err(err) = _integration_event(application_id, &vars, &pl).await {
                error!(application_id = %application_id, error = %err, "Location event error");
            }
        }
    });
}

async fn _integration_event(
    application_id: Uuid,
    vars: &HashMap<String, String>,
    pl: &integration::IntegrationEvent,
) -> Result<()> {
    let app_ints = for_application_id(application_id)
        .await
        .context("Get integrations for application")?;
    let global_ints = GLOBAL_INTEGRATIONS.read().await;
    let mut futures = Vec::new();

    for (i, _) in app_ints.iter().enumerate() {
        futures.push(app_ints[i].integration_event(vars, pl));
    }
    for (i, _) in global_ints.iter().enumerate() {
        futures.push(global_ints[i].integration_event(vars, pl));
    }

    for e in join_all(futures).await {
        e?;
    }

    Ok(())
}

async fn handle_down_command(application_id: String, pl: integration::DownlinkCommand) {
    let err = async {
        info!(dev_eui = %pl.dev_eui, "Handling downlink command for device");
        let dev_eui = EUI64::from_str(&pl.dev_eui)?;
        let app_id = Uuid::from_str(&application_id)?;

        // Validate that the application_id from the topic is indeed the application ID to which
        // the device belongs.
        let dev = device::get(&dev_eui).await?;
        if dev.application_id != app_id {
            return Err(anyhow!(
                "Application ID from topic does not match application ID from device"
            ));
        }

        let mut data = pl.data.clone();
        if let Some(obj) = &pl.object {
            let dp = device_profile::get(&dev.device_profile_id).await?;

            data = codec::struct_to_binary(
                dp.payload_codec_runtime,
                pl.f_port as u8,
                &dev.variables,
                &dp.payload_codec_script,
                &codec::convert::pb_json_to_prost(obj),
            )
            .await?;
        }

        let qi = device_queue::DeviceQueueItem {
            id: match pl.id.is_empty() {
                true => Uuid::new_v4(),
                false => Uuid::from_str(&pl.id)?,
            },
            f_port: pl.f_port as i16,
            confirmed: pl.confirmed,
            data,
            dev_eui,
            ..Default::default()
        };

        device_queue::enqueue_item(qi).await?;

        Ok(())
    }
    .await
    .err();

    if err.is_some() {
        error!(dev_eui = %pl.dev_eui, error = %err.as_ref().unwrap(), "Handling downlink command error");
    }
}
