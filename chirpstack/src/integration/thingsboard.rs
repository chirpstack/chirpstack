use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Client;
use serde::{Serialize, Serializer};
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::ThingsBoardConfiguration;
use chirpstack_api::integration;

pub struct Integration {
    server: String,
    timeout: Duration,
}

impl Integration {
    pub fn new(conf: &ThingsBoardConfiguration) -> Integration {
        trace!("Initializing ThingsBoard integration");

        Integration {
            timeout: Duration::from_secs(5),
            server: conf.server.clone(),
        }
    }

    async fn send_attributes(
        &self,
        vars: &HashMap<String, String>,
        attributes: &Payload,
    ) -> Result<()> {
        let endpoint = format!(
            "{}/api/v1/{}/attributes",
            self.server,
            vars.get("ThingsBoardAccessToken")
                .cloned()
                .unwrap_or_default()
        );
        let b = serde_json::to_string(&attributes)?;

        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let res = client
            .post(endpoint)
            .body(b)
            .headers(headers)
            .send()
            .await?;
        let _ = res.error_for_status()?;
        Ok(())
    }

    async fn send_telemetry(
        &self,
        vars: &HashMap<String, String>,
        telemetry: &Payload,
    ) -> Result<()> {
        let endpoint = format!(
            "{}/api/v1/{}/telemetry",
            self.server,
            vars.get("ThingsBoardAccessToken")
                .cloned()
                .unwrap_or_default()
        );
        let b = serde_json::to_string(&telemetry)?;
        println!("{}", b);

        let client = Client::builder().timeout(self.timeout).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let res = client
            .post(endpoint)
            .body(b)
            .headers(headers)
            .send()
            .await?;
        let _ = res.error_for_status()?;
        Ok(())
    }
}

#[async_trait]
impl IntegrationTrait for Integration {
    async fn uplink_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::UplinkEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();

        let mut attributes: BTreeMap<String, Value> = di
            .tags
            .iter()
            .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
            .collect();
        attributes.insert(
            "application_name".to_string(),
            Value::String(di.application_name.clone()),
        );
        attributes.insert(
            "application_id".to_string(),
            Value::String(di.application_id.clone()),
        );
        attributes.insert(
            "device_name".to_string(),
            Value::String(di.device_name.clone()),
        );
        attributes.insert("dev_eui".to_string(), Value::String(di.dev_eui.clone()));
        let attributes = Payload(attributes);

        info!(dev_eui = %di.dev_eui, server = %self.server, "Sending device telemetry");
        self.send_attributes(vars, &attributes).await?;

        let mut telemetry: BTreeMap<String, Value> = if let Some(obj) = &pl.object {
            struct_to_telemetry(obj)
        } else {
            BTreeMap::new()
        };

        telemetry.insert("f_port".to_string(), Value::Integer(pl.f_port.into()));
        telemetry.insert("f_cnt".to_string(), Value::Integer(pl.f_cnt.into()));
        telemetry.insert("dr".to_string(), Value::Integer(pl.dr.into()));
        telemetry.insert(
            "rssi".to_string(),
            Value::Integer(pl.rx_info.iter().max_by_key(|x| x.rssi).unwrap().rssi as i64),
        );
        telemetry.insert(
            "snr".to_string(),
            Value::Float(
                pl.rx_info
                    .iter()
                    .max_by(|x, y| {
                        x.snr
                            .partial_cmp(&y.snr)
                            .unwrap_or(std::cmp::Ordering::Less)
                    })
                    .unwrap()
                    .snr
                    .into(),
            ),
        );
        let telemetry = Payload(telemetry);

        info!(dev_eui = %di.dev_eui, server = %self.server, "Sending device telemetry");
        self.send_telemetry(vars, &telemetry).await?;

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
        vars: &HashMap<String, String>,
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();
        let mut telemetry: BTreeMap<String, Value> = BTreeMap::new();
        telemetry.insert(
            "status_margin".to_string(),
            Value::Integer(pl.margin.into()),
        );
        telemetry.insert(
            "status_external_power_source".to_string(),
            Value::Bool(pl.external_power_source),
        );
        telemetry.insert(
            "status_battery_level".to_string(),
            Value::Float(pl.battery_level.into()),
        );
        telemetry.insert(
            "status_battery_level_unavailable".to_string(),
            Value::Bool(pl.battery_level_unavailable),
        );

        let telemetry = Payload(telemetry);

        info!(dev_eui = %di.dev_eui, server = %self.server, "Sending device telemetry");
        self.send_telemetry(vars, &telemetry).await
    }

    async fn location_event(
        &self,
        vars: &HashMap<String, String>,
        pl: &integration::LocationEvent,
    ) -> Result<()> {
        if let Some(loc) = &pl.location {
            let di = pl.device_info.as_ref().unwrap();
            let mut telemetry: BTreeMap<String, Value> = BTreeMap::new();
            telemetry.insert("location_latitude".to_string(), Value::Float(loc.latitude));
            telemetry.insert(
                "location_longitude".to_string(),
                Value::Float(loc.longitude),
            );
            telemetry.insert("location_altitude".to_string(), Value::Float(loc.altitude));

            let telemetry = Payload(telemetry);

            info!(dev_eui = %di.dev_eui, server = %self.server, "Sending device telemetry");
            self.send_telemetry(vars, &telemetry).await?;
        }

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

#[derive(Clone)]
enum Value {
    Bool(bool),
    Float(f64),
    String(String),
    Integer(i64),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Float(v) => serializer.serialize_f64(*v),
            Value::String(v) => serializer.serialize_str(v),
            Value::Integer(v) => serializer.serialize_i64(*v),
        }
    }
}

#[derive(Serialize)]
struct Payload(BTreeMap<String, Value>);

fn struct_to_telemetry(s: &pbjson_types::Struct) -> BTreeMap<String, Value> {
    let mut out: BTreeMap<String, Value> = BTreeMap::new();

    for (k, v) in &s.fields {
        out.extend(struct_value_to_telemetry(&format!("data_{}", k), v));
    }

    out
}

fn struct_value_to_telemetry(prefix: &str, v: &pbjson_types::Value) -> BTreeMap<String, Value> {
    let mut out: BTreeMap<String, Value> = BTreeMap::new();

    if let Some(k) = &v.kind {
        match k {
            pbjson_types::value::Kind::NullValue(_) => {}
            pbjson_types::value::Kind::NumberValue(v) => {
                out.insert(prefix.to_string(), Value::Float(*v));
            }
            pbjson_types::value::Kind::StringValue(v) => {
                out.insert(prefix.to_string(), Value::String(v.to_string()));
            }
            pbjson_types::value::Kind::BoolValue(v) => {
                out.insert(prefix.to_string(), Value::Bool(*v));
            }
            pbjson_types::value::Kind::StructValue(sv) => {
                for (k, v) in &sv.fields {
                    let prefix = format!("{}_{}", prefix, k);
                    out.extend(struct_value_to_telemetry(&prefix, v));
                }
            }
            pbjson_types::value::Kind::ListValue(v) => {
                for (i, v) in v.values.iter().enumerate() {
                    let prefix = format!("{}_{}", prefix, i);
                    out.extend(struct_value_to_telemetry(&prefix, v));
                }
            }
        }
    }

    out
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chirpstack_api::{common, gw};
    use httpmock::prelude::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_thingsboard() {
        let server = MockServer::start();

        let i = Integration {
            server: server.url(""),
            timeout: Duration::from_secs(5),
        };

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert(
            "ThingsBoardAccessToken".to_string(),
            "test-token".to_string(),
        );

        // uplink with decoded payload
        let mut mock_attr = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/test-token/attributes")
                .header("Content-Type", "application/json")
                .body(r#"{"application_id":"00000000-0000-0000-0000-000000000000","application_name":"test-app","dev_eui":"0102030405060708","device_name":"test-device","foo":"bar"}"#);

            then.status(200);
        });
        let mut mock_telm = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/test-token/telemetry")
                .header("Content-Type", "application/json")
                .body(r#"{"data_active":true,"data_status":"on","data_temperature":20.5,"dr":2,"f_cnt":20,"f_port":10,"rssi":-55,"snr":2.5}"#);

            then.status(200);
        });

        i.uplink_event(
            &vars,
            &integration::UplinkEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".to_string(),
                    application_id: Uuid::nil().to_string(),
                    device_name: "test-device".to_string(),
                    dev_eui: "0102030405060708".to_string(),
                    tags: [("foo".to_string(), "bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                f_port: 10,
                f_cnt: 20,
                dr: 2,
                rx_info: vec![
                    gw::UplinkRxInfo {
                        rssi: -60,
                        snr: 1.0,
                        ..Default::default()
                    },
                    gw::UplinkRxInfo {
                        rssi: -55,
                        snr: 2.5,
                        ..Default::default()
                    },
                    gw::UplinkRxInfo {
                        rssi: -70,
                        snr: 1.0,
                        ..Default::default()
                    },
                ],
                object: Some(pbjson_types::Struct {
                    fields: [
                        (
                            "temperature".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::NumberValue(20.5)),
                            },
                        ),
                        (
                            "active".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::BoolValue(true)),
                            },
                        ),
                        (
                            "status".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::StringValue("on".into())),
                            },
                        ),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                }),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        mock_attr.assert();
        mock_attr.delete();
        mock_telm.assert();
        mock_telm.delete();

        // uplink without decoded payload
        let mut mock_attr = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/test-token/attributes")
                .header("Content-Type", "application/json")
                .body(r#"{"application_id":"00000000-0000-0000-0000-000000000000","application_name":"test-app","dev_eui":"0102030405060708","device_name":"test-device","foo":"bar"}"#);

            then.status(200);
        });
        let mut mock_telm = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/test-token/telemetry")
                .header("Content-Type", "application/json")
                .body(r#"{"dr":2,"f_cnt":20,"f_port":10,"rssi":-55,"snr":2.5}"#);

            then.status(200);
        });

        i.uplink_event(
            &vars,
            &integration::UplinkEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".to_string(),
                    application_id: Uuid::nil().to_string(),
                    device_name: "test-device".to_string(),
                    dev_eui: "0102030405060708".to_string(),
                    tags: [("foo".to_string(), "bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                f_port: 10,
                f_cnt: 20,
                dr: 2,
                rx_info: vec![
                    gw::UplinkRxInfo {
                        rssi: -60,
                        snr: 1.0,
                        ..Default::default()
                    },
                    gw::UplinkRxInfo {
                        rssi: -55,
                        snr: 2.5,
                        ..Default::default()
                    },
                    gw::UplinkRxInfo {
                        rssi: -70,
                        snr: 1.0,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        )
        .await
        .unwrap();

        mock_attr.assert();
        mock_attr.delete();
        mock_telm.assert();
        mock_telm.delete();

        // location
        let mut mock_telm = server.mock(|when, then| {
            when.method(POST)
                .path("/api/v1/test-token/telemetry")
                .header("Content-Type", "application/json")
                .body(r#"{"location_altitude":3.23,"location_latitude":1.23,"location_longitude":2.23}"#);

            then.status(200);
        });

        i.location_event(
            &vars,
            &integration::LocationEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".to_string(),
                    application_id: Uuid::nil().to_string(),
                    device_name: "test-device".to_string(),
                    dev_eui: "0102030405060708".to_string(),
                    tags: [("foo".to_string(), "bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                location: Some(common::Location {
                    latitude: 1.23,
                    longitude: 2.23,
                    altitude: 3.23,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        mock_telm.assert();
        mock_telm.delete();

        // status
        let mut mock_telm = server.mock(|when, then| {
            when.method(POST).path("/api/v1/test-token/telemetry")
                .header("Content-Type", "application/json")
                .body(r#"{"status_battery_level":75.0,"status_battery_level_unavailable":false,"status_external_power_source":false,"status_margin":10}"#);

            then.status(200);
        });

        i.status_event(
            &vars,
            &integration::StatusEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".to_string(),
                    application_id: Uuid::nil().to_string(),
                    device_name: "test-device".to_string(),
                    dev_eui: "0102030405060708".to_string(),
                    tags: [("foo".to_string(), "bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                battery_level: 75.0,
                margin: 10,
                ..Default::default()
            },
        )
        .await
        .unwrap();

        mock_telm.assert();
        mock_telm.delete();
    }
}
