use std::collections::HashMap;
use std::io::Cursor;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use handlebars::Handlebars;
use prost::Message;
use rand::Rng;
use regex::Regex;
use rumqttc::tokio_rustls::rustls;
use rumqttc::v5::mqttbytes::v5::{ConnectReturnCode, Publish};
use rumqttc::v5::{mqttbytes::QoS, AsyncClient, Event, Incoming, MqttOptions};
use rumqttc::Transport;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{error, info, trace, warn};

use super::Integration as IntegrationTrait;
use crate::config::MqttIntegration as Config;
use crate::helpers::tls::{get_root_certs, load_cert, load_key};
use chirpstack_api::integration;

pub struct Integration<'a> {
    client: AsyncClient,
    templates: Handlebars<'a>,
    json: bool,
    qos: QoS,
    command_regex: Regex,
}

#[derive(Serialize)]
struct EventTopicContext {
    pub application_id: String,
    pub dev_eui: String,
    pub event: String,
}

#[derive(Serialize)]
struct CommandTopicContext {
    pub application_id: String,
    pub dev_eui: String,
    pub command: String,
}

impl<'a> Integration<'a> {
    pub async fn new(conf: &Config) -> Result<Integration<'a>> {
        info!("Initializing MQTT integration");

        // topic templates
        let mut templates = Handlebars::new();
        templates.register_escape_fn(handlebars::no_escape);
        templates.register_template_string("event_topic", &conf.event_topic)?;
        templates.register_template_string("command_topic", &conf.command_topic)?;

        let command_topic = templates.render(
            "command_topic",
            &CommandTopicContext {
                application_id: "+".into(),
                dev_eui: "+".into(),
                command: "+".into(),
            },
        )?;

        // get client id, this will generate a random client_id when no client_id has been
        // configured.
        let client_id = if conf.client_id.is_empty() {
            let mut rnd = rand::thread_rng();
            let client_id: u64 = rnd.gen();
            format!("{:x}", client_id)
        } else {
            conf.client_id.clone()
        };

        // Get QoS
        let qos = match conf.qos {
            0 => QoS::AtMostOnce,
            1 => QoS::AtLeastOnce,
            2 => QoS::ExactlyOnce,
            _ => return Err(anyhow!("Invalid QoS: {}", conf.qos)),
        };

        // Create connect channel
        // We need to re-subscribe on (re)connect to be sure we have a subscription. Even
        // in case of a persistent MQTT session, there is no guarantee that the MQTT persisted the
        // session and that a re-connect would recover the subscription.
        let (connect_tx, mut connect_rx) = mpsc::channel(10);

        // Create client
        let mut mqtt_opts =
            MqttOptions::parse_url(format!("{}?client_id={}", conf.server, client_id))?;
        mqtt_opts.set_clean_start(conf.clean_session);
        mqtt_opts.set_keep_alive(conf.keep_alive_interval);
        if !conf.username.is_empty() || !conf.password.is_empty() {
            mqtt_opts.set_credentials(&conf.username, &conf.password);
        }

        if !conf.ca_cert.is_empty() || !conf.tls_cert.is_empty() || !conf.tls_key.is_empty() {
            info!(
                "Configuring client with TLS certificate, ca_cert: {}, tls_cert: {}, tls_key: {}",
                conf.ca_cert, conf.tls_cert, conf.tls_key
            );

            let root_certs = get_root_certs(if conf.ca_cert.is_empty() {
                None
            } else {
                Some(conf.ca_cert.clone())
            })?;

            let client_conf = if conf.tls_cert.is_empty() && conf.tls_key.is_empty() {
                rustls::ClientConfig::builder()
                    .with_root_certificates(root_certs.clone())
                    .with_no_client_auth()
            } else {
                rustls::ClientConfig::builder()
                    .with_root_certificates(root_certs.clone())
                    .with_client_auth_cert(
                        load_cert(&conf.tls_cert).await?,
                        load_key(&conf.tls_key).await?,
                    )?
            };

            mqtt_opts.set_transport(Transport::tls_with_config(client_conf.into()));
        }

        let (client, mut eventloop) = AsyncClient::new(mqtt_opts, 100);

        let i = Integration {
            command_regex: Regex::new(&templates.render(
                "command_topic",
                &CommandTopicContext {
                    application_id: r"(?P<application_id>[\w-]+)".to_string(),
                    dev_eui: r"(?P<dev_eui>[\w]+)".to_string(),
                    command: r"(?P<command>[\w]+)".to_string(),
                },
            )?)?,
            qos,
            json: conf.json,
            client,
            templates,
        };

        // connect
        info!(server_uri = %conf.server, client_id = %client_id, clean_session = conf.clean_session, "Connecting to MQTT broker");

        // (Re)subscribe loop
        tokio::spawn({
            let client = i.client.clone();
            let qos = i.qos;

            async move {
                while connect_rx.recv().await.is_some() {
                    info!(command_topic = %command_topic, "Subscribing to command topic");
                    if let Err(e) = client.subscribe(&command_topic, qos).await {
                        error!(error = %e, "Subscribe to command topic error");
                    }
                }
            }
        });

        // Eventloop
        tokio::spawn({
            let command_regex = i.command_regex.clone();
            let json = i.json;

            async move {
                info!("Starting MQTT event loop");

                loop {
                    match eventloop.poll().await {
                        Ok(v) => {
                            trace!(event = ?v, "MQTT event");

                            match v {
                                Event::Incoming(Incoming::Publish(p)) => {
                                    let topic = String::from_utf8_lossy(&p.topic);
                                    let caps = match command_regex.captures(&topic) {
                                        Some(v) => v,
                                        None => {
                                            warn!(topic = %topic, "Error parsing command topic (regex captures returned None");
                                            continue;
                                        }
                                    };

                                    if caps.len() != 4 {
                                        warn!(topic = %topic, "Parsing command topic returned invalid match count");
                                        continue;
                                    }

                                    message_callback(
                                        caps.get(1).map_or("", |m| m.as_str()).to_string(),
                                        caps.get(2).map_or("", |m| m.as_str()).to_string(),
                                        caps.get(3).map_or("", |m| m.as_str()).to_string(),
                                        json,
                                        p,
                                    )
                                    .await;
                                }
                                Event::Incoming(Incoming::ConnAck(v)) => {
                                    if v.code == ConnectReturnCode::Success {
                                        if let Err(e) = connect_tx.try_send(()) {
                                            error!(error = %e, "Send to subscribe channel error");
                                        }
                                    } else {
                                        error!(code = ?v.code, "Connection error");
                                        sleep(Duration::from_secs(1)).await
                                    }
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            error!(error = %e, "MQTT error");
                            sleep(Duration::from_secs(1)).await
                        }
                    }
                }
            }
        });

        // Return integration.
        Ok(i)
    }

    fn get_event_topic(&self, application_id: &str, dev_eui: &str, event: &str) -> Result<String> {
        Ok(self.templates.render(
            "event_topic",
            &EventTopicContext {
                application_id: application_id.to_string(),
                dev_eui: dev_eui.to_string(),
                event: event.to_string(),
            },
        )?)
    }

    async fn publish_event(&self, topic: &str, b: Vec<u8>) -> Result<()> {
        info!(topic = %topic, "Publishing event");
        self.client.publish(topic, self.qos, false, b).await?;
        Ok(())
    }
}

#[async_trait]
impl IntegrationTrait for Integration<'_> {
    async fn uplink_event(
        &self,
        _vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let dev_info = pl
            .device_info
            .as_ref()
            .ok_or_else(|| anyhow!("device_info is None"))?;

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "up")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "join")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "ack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "txack")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "log")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic = self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "status")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic =
            self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "location")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
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

        let topic =
            self.get_event_topic(&dev_info.application_id, &dev_info.dev_eui, "integration")?;
        let b = match self.json {
            true => serde_json::to_vec(&pl)?,
            false => pl.encode_to_vec(),
        };

        self.publish_event(&topic, b).await
    }
}

async fn message_callback(
    application_id: String,
    dev_eui: String,
    command: String,
    json: bool,
    p: Publish,
) {
    let topic = String::from_utf8_lossy(&p.topic);

    info!(topic = %topic, qos = ?p.qos, "Command received for device");

    let err = || -> Result<()> {
        match command.as_ref() {
            "down" => {
                let cmd: integration::DownlinkCommand = match json {
                    true => serde_json::from_slice(&p.payload)?,
                    false => integration::DownlinkCommand::decode(&mut Cursor::new(&p.payload))?,
                };
                if dev_eui != cmd.dev_eui {
                    return Err(anyhow!(
                        "Payload dev_eui {} does not match topic dev_eui {}",
                        cmd.dev_eui,
                        dev_eui
                    ));
                }
                tokio::spawn(super::handle_down_command(application_id, cmd));
            }
            _ => {
                return Err(anyhow!("Unknown command type"));
            }
        }

        Ok(())
    }()
    .err();

    if err.is_some() {
        warn!(
            topic = %topic,
            qos = ?p.qos,
            "Processing command error: {}",
            err.as_ref().unwrap()
        );
    }
}

#[cfg(all(test, feature = "test-integration-mqtt"))]
pub mod test {
    use std::env;

    use super::*;
    use crate::config::MqttIntegration;
    use crate::storage::{application, device, device_profile, device_queue, tenant};
    use crate::test;
    use lrwn::EUI64;
    use tokio::sync::mpsc;
    use tokio::time::{sleep, Duration};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mqtt() {
        let _guard = test::prepare().await;

        // setup base objects
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();
        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();
        let dev = device::create(device::Device {
            name: "test-device".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // setup of integration and MQTT client
        let conf = MqttIntegration {
            event_topic: "application/{{application_id}}/device/{{dev_eui}}/event/{{event}}".into(),
            json: true,
            server: env::var("TEST_MOSQUITTO_SERVER").unwrap(),
            clean_session: true,
            ..Default::default()
        };
        let i = Integration::new(&conf).await.unwrap();

        let mut mqtt_opts =
            MqttOptions::parse_url(format!("{}?client_id=chirpstack_test", &conf.server)).unwrap();
        mqtt_opts.set_clean_start(true);
        let (client, mut eventloop) = AsyncClient::new(mqtt_opts, 100);
        let (mqtt_tx, mut mqtt_rx) = mpsc::channel(100);

        tokio::spawn({
            async move {
                loop {
                    match eventloop.poll().await {
                        Ok(v) => match v {
                            Event::Incoming(Incoming::Publish(p)) => mqtt_tx.send(p).await.unwrap(),
                            _ => {}
                        },
                        Err(_) => {
                            break;
                        }
                    }
                }
            }
        });

        client
            .subscribe(
                "application/00000000-0000-0000-0000-000000000000/device/+/event/+",
                QoS::AtLeastOnce,
            )
            .await
            .unwrap();

        sleep(Duration::from_millis(100)).await;

        // uplink event
        let pl = integration::UplinkEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.uplink_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/up",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // join event
        let pl = integration::JoinEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.join_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/join",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // ack event
        let pl = integration::AckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.ack_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/ack",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // txack event
        let pl = integration::TxAckEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.txack_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/txack",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // log event
        let pl = integration::LogEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.log_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/log",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // status event
        let pl = integration::StatusEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.status_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/status",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // location event
        let pl = integration::LocationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.location_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/location",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // integration event
        let pl = integration::IntegrationEvent {
            device_info: Some(integration::DeviceInfo {
                application_id: Uuid::nil().to_string(),
                dev_eui: "0102030405060708".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        i.integration_event(&HashMap::new(), &pl).await.unwrap();
        let msg = mqtt_rx.recv().await.unwrap();
        assert_eq!(
            "application/00000000-0000-0000-0000-000000000000/device/0102030405060708/event/integration",
            String::from_utf8(msg.topic.to_vec()).unwrap()
        );
        assert_eq!(
            serde_json::to_string(&pl).unwrap(),
            String::from_utf8(msg.payload.to_vec()).unwrap()
        );

        // downlink command
        let down_cmd = integration::DownlinkCommand {
            id: Uuid::new_v4().to_string(),
            dev_eui: dev.dev_eui.to_string(),
            confirmed: false,
            f_port: 10,
            data: vec![1, 2, 3],
            object: None,
        };
        let down_cmd_json = serde_json::to_string(&down_cmd).unwrap();
        client
            .publish(
                format!("application/{}/device/{}/command/down", app.id, dev.dev_eui),
                QoS::AtLeastOnce,
                false,
                down_cmd_json,
            )
            .await
            .unwrap();

        // give the async consumer some time to process
        sleep(Duration::from_millis(200)).await;

        let queue_items = device_queue::get_for_dev_eui(&dev.dev_eui).await.unwrap();
        assert_eq!(1, queue_items.len());
        assert_eq!(down_cmd.id, queue_items[0].id.to_string());
        assert_eq!(dev.dev_eui, queue_items[0].dev_eui);
        assert_eq!(10, queue_items[0].f_port);
        assert_eq!(vec![1, 2, 3], queue_items[0].data);
    }
}
