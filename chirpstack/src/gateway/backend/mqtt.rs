use std::collections::HashMap;
use std::io::Cursor;
use std::sync::RwLock;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use handlebars::Handlebars;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prost::Message;
use rand::Rng;
use rumqttc::tokio_rustls::rustls;
use rumqttc::v5::mqttbytes::v5::{ConnectReturnCode, Publish};
use rumqttc::v5::{mqttbytes::QoS, AsyncClient, Event, Incoming, MqttOptions};
use rumqttc::Transport;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{error, info, trace};

use super::GatewayBackend;
use crate::config::GatewayBackendMqtt;
use crate::helpers::tls::{get_root_certs, load_cert, load_key};
use crate::monitoring::prometheus;
use crate::{downlink, uplink};
use lrwn::region::CommonName;

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
struct EventLabels {
    event: String,
}

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
struct CommandLabels {
    command: String,
}

lazy_static! {
    static ref EVENT_COUNTER: Family<EventLabels, Counter> = {
        let counter = Family::<EventLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_events",
            "Number of events received",
            counter.clone(),
        );
        counter
    };
    static ref COMMAND_COUNTER: Family<CommandLabels, Counter> = {
        let counter = Family::<CommandLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_commands",
            "Number of commands sent",
            counter.clone(),
        );
        counter
    };
    static ref GATEWAY_JSON: RwLock<HashMap<String, bool>> = RwLock::new(HashMap::new());
}

pub struct MqttBackend<'a> {
    client: AsyncClient,
    templates: handlebars::Handlebars<'a>,
    qos: QoS,
    v4_migrate: bool,
    region_config_id: String,
}

#[derive(Serialize)]
struct CommandTopicContext {
    pub gateway_id: String,
    pub command: String,
}

impl<'a> MqttBackend<'a> {
    pub async fn new(
        region_config_id: &str,
        region_common_name: CommonName,
        conf: &GatewayBackendMqtt,
    ) -> Result<MqttBackend<'a>> {
        // topic templates
        let mut templates = Handlebars::new();
        templates.register_template_string(
            "command_topic",
            if conf.command_topic.is_empty() {
                let command_topic = "gateway/{{ gateway_id }}/command/{{ command }}".to_string();
                if conf.topic_prefix.is_empty() {
                    command_topic
                } else {
                    format!("{}/{}", conf.topic_prefix, command_topic)
                }
            } else {
                conf.command_topic.clone()
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

        let b = MqttBackend {
            client,
            qos,
            templates,
            v4_migrate: conf.v4_migrate,
            region_config_id: region_config_id.to_string(),
        };

        // connect
        info!(region_id = %region_config_id, server_uri = %conf.server, clean_session = conf.clean_session, client_id = %client_id, "Connecting to MQTT broker");

        // (Re)subscribe loop
        tokio::spawn({
            let client = b.client.clone();
            let qos = b.qos;
            let region_config_id = region_config_id.to_string();
            let event_topic = if conf.event_topic.is_empty() {
                let event_topic = "gateway/+/event/+".to_string();
                if conf.topic_prefix.is_empty() {
                    event_topic
                } else {
                    format!("{}/{}", conf.topic_prefix, event_topic)
                }
            } else {
                conf.event_topic.clone()
            };
            let share_name = conf.share_name.clone();

            async move {
                while let Some(shared_sub_support) = connect_rx.recv().await {
                    let event_topic = if shared_sub_support {
                        format!("$share/{}/{}", share_name, event_topic)
                    } else {
                        event_topic.clone()
                    };

                    info!(region_id = %region_config_id, event_topic = %event_topic, "Subscribing to gateway event topic");
                    if let Err(e) = client.subscribe(&event_topic, qos).await {
                        error!(region_id = %region_config_id, event_topic = %event_topic, error = %e, "MQTT subscribe error");
                    }
                }
            }
        });

        // Eventloop
        tokio::spawn({
            let region_config_id = region_config_id.to_string();
            let v4_migrate = conf.v4_migrate;

            async move {
                info!("Starting MQTT event loop");

                loop {
                    match eventloop.poll().await {
                        Ok(v) => {
                            trace!(event = ?v, "MQTT event");

                            match v {
                                Event::Incoming(Incoming::Publish(p)) => {
                                    message_callback(
                                        v4_migrate,
                                        &region_config_id,
                                        region_common_name,
                                        p,
                                    )
                                    .await
                                }
                                Event::Incoming(Incoming::ConnAck(v)) => {
                                    if v.code == ConnectReturnCode::Success {
                                        // Per specification:
                                        // A value of 1 means Shared Subscriptions are supported. If not present, then Shared Subscriptions are supported.
                                        let shared_sub_support = v
                                            .properties
                                            .map(|v| {
                                                v.shared_subscription_available
                                                    .map(|v| v == 1)
                                                    .unwrap_or(true)
                                            })
                                            .unwrap_or(true);

                                        if let Err(e) = connect_tx.try_send(shared_sub_support) {
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

        // return backend
        Ok(b)
    }

    fn get_command_topic(&self, gateway_id: &str, command: &str) -> Result<String> {
        Ok(self.templates.render(
            "command_topic",
            &CommandTopicContext {
                gateway_id: gateway_id.to_string(),
                command: command.to_string(),
            },
        )?)
    }
}

#[async_trait]
impl GatewayBackend for MqttBackend<'_> {
    async fn send_downlink(&self, df: &chirpstack_api::gw::DownlinkFrame) -> Result<()> {
        COMMAND_COUNTER
            .get_or_create(&CommandLabels {
                command: "down".to_string(),
            })
            .inc();
        let topic = self.get_command_topic(&df.gateway_id, "down")?;
        let mut df = df.clone();

        if self.v4_migrate {
            df.v4_migrate();
        }

        let json = gateway_is_json(&df.gateway_id);
        let b = match json {
            true => serde_json::to_vec(&df)?,
            false => df.encode_to_vec(),
        };

        info!(region_id = %self.region_config_id, gateway_id = %df.gateway_id, topic = %topic, json = json, "Sending downlink frame");
        self.client.publish(topic, self.qos, false, b).await?;
        trace!("Message published");

        Ok(())
    }

    async fn send_configuration(
        &self,
        gw_conf: &chirpstack_api::gw::GatewayConfiguration,
    ) -> Result<()> {
        COMMAND_COUNTER
            .get_or_create(&CommandLabels {
                command: "config".to_string(),
            })
            .inc();
        let topic = self.get_command_topic(&gw_conf.gateway_id, "config")?;
        let json = gateway_is_json(&gw_conf.gateway_id);
        let b = match json {
            true => serde_json::to_vec(&gw_conf)?,
            false => gw_conf.encode_to_vec(),
        };

        info!(region_id = %self.region_config_id, gateway_id = %gw_conf.gateway_id, topic = %topic, json = json, "Sending gateway configuration");
        self.client.publish(topic, self.qos, false, b).await?;
        trace!("Message published");

        Ok(())
    }
}

async fn message_callback(
    v4_migrate: bool,
    region_config_id: &str,
    region_common_name: CommonName,
    p: Publish,
) {
    let topic = String::from_utf8_lossy(&p.topic);

    let err = || -> Result<()> {
        let json = payload_is_json(&p.payload);

        info!(
            region_id = region_config_id,
            topic = %topic,
            qos = ?p.qos,
            json = json,
            "Message received from gateway"
        );

        if topic.ends_with("/up") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "up".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(&p.payload)?,
                false => chirpstack_api::gw::UplinkFrame::decode(&mut Cursor::new(&p.payload))?,
            };

            if v4_migrate {
                event.v4_migrate();
            }

            if let Some(rx_info) = &mut event.rx_info {
                set_gateway_json(&rx_info.gateway_id, json);
                rx_info.ns_time = Some(Utc::now().into());
                rx_info
                    .metadata
                    .insert("region_config_id".to_string(), region_config_id.to_string());
                rx_info.metadata.insert(
                    "region_common_name".to_string(),
                    region_common_name.to_string(),
                );
            }

            tokio::spawn(uplink::deduplicate_uplink(event));
        } else if topic.ends_with("/stats") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "stats".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(&p.payload)?,
                false => chirpstack_api::gw::GatewayStats::decode(&mut Cursor::new(&p.payload))?,
            };

            if v4_migrate {
                event.v4_migrate();
            }

            event
                .metadata
                .insert("region_config_id".to_string(), region_config_id.to_string());
            event.metadata.insert(
                "region_common_name".to_string(),
                region_common_name.to_string(),
            );
            set_gateway_json(&event.gateway_id, json);
            tokio::spawn(uplink::stats::Stats::handle(event));
        } else if topic.ends_with("/ack") {
            EVENT_COUNTER
                .get_or_create(&EventLabels {
                    event: "ack".to_string(),
                })
                .inc();
            let mut event = match json {
                true => serde_json::from_slice(&p.payload)?,
                false => chirpstack_api::gw::DownlinkTxAck::decode(&mut Cursor::new(&p.payload))?,
            };

            if v4_migrate {
                event.v4_migrate();
            }

            set_gateway_json(&event.gateway_id, json);
            tokio::spawn(downlink::tx_ack::TxAck::handle(event));
        } else {
            return Err(anyhow!("Unknown event type"));
        }

        Ok(())
    }()
    .err();

    if err.is_some() {
        error!(
            region_id = %region_config_id,
            topic = %topic,
            qos = ?p.qos,
            "Processing gateway event error: {}",
            err.as_ref().unwrap()
        );
    }
}

fn gateway_is_json(gateway_id: &str) -> bool {
    let gw_json_r = GATEWAY_JSON.read().unwrap();
    gw_json_r.get(gateway_id).cloned().unwrap_or(false)
}

fn set_gateway_json(gateway_id: &str, is_json: bool) {
    let mut gw_json_w = GATEWAY_JSON.write().unwrap();
    gw_json_w.insert(gateway_id.to_string(), is_json);
}

fn payload_is_json(b: &[u8]) -> bool {
    String::from_utf8_lossy(b).contains("gatewayId")
}
