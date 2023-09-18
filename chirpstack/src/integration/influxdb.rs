use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use tracing::{info, trace};

use super::Integration as IntegrationTrait;
use crate::storage::application::InfluxDbConfiguration;
use chirpstack_api::api::{InfluxDbPrecision, InfluxDbVersion};
use chirpstack_api::integration;

pub struct Integration {
    timeout: Duration,
    endpoint: String,
    version: InfluxDbVersion,

    // v1
    db: String,
    username: String,
    password: String,
    retention_policy_name: String,
    precision: String,

    // v2
    token: String,
    organization: String,
    bucket: String,
}

impl Integration {
    pub fn new(conf: &InfluxDbConfiguration) -> Result<Integration> {
        trace!("Initializing InfluxDB integration");

        Ok(Integration {
            timeout: Duration::from_secs(5),
            endpoint: conf.endpoint.clone(),
            version: InfluxDbVersion::try_from(conf.version)
                .map_err(|_| anyhow!("Invalid version"))?,
            db: conf.db.clone(),
            username: conf.username.clone(),
            password: conf.password.clone(),
            retention_policy_name: conf.retention_policy_name.clone(),
            precision: match InfluxDbPrecision::try_from(conf.precision)
                .map_err(|_| anyhow!("Invalid precision"))?
            {
                InfluxDbPrecision::Ns => "ns",
                InfluxDbPrecision::U => "u",
                InfluxDbPrecision::Ms => "ms",
                InfluxDbPrecision::S => "s",
                InfluxDbPrecision::M => "m",
                InfluxDbPrecision::H => "h",
            }
            .to_string(),
            token: conf.token.clone(),
            organization: conf.organization.clone(),
            bucket: conf.bucket.clone(),
        })
    }

    async fn publish(&self, measurements: &[Measurement]) -> Result<()> {
        let mut measurements: Vec<String> = measurements.iter().map(|m| m.to_string()).collect();
        measurements.sort();
        let body = measurements.join("\n");

        let client = Client::builder().timeout(self.timeout).build()?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());
        if self.version == InfluxDbVersion::Influxdb2 {
            headers.insert(AUTHORIZATION, format!("Token {}", self.token).parse()?);
        }

        let mut query: Vec<(String, String)> = Vec::new();
        match self.version {
            InfluxDbVersion::Influxdb1 => {
                query.push(("db".into(), self.db.clone()));
                query.push(("precision".into(), self.precision.clone()));
                query.push(("rp".into(), self.retention_policy_name.clone()));
            }
            InfluxDbVersion::Influxdb2 => {
                query.push(("org".into(), self.organization.clone()));
                query.push(("bucket".into(), self.bucket.clone()));
            }
        }

        let mut req = client
            .post(&self.endpoint)
            .body(body)
            .query(&query)
            .headers(headers);

        if self.version == InfluxDbVersion::Influxdb1
            && !self.username.is_empty()
            && !self.password.is_empty()
        {
            req = req.basic_auth(self.username.clone(), Some(self.password.clone()));
        }

        let res = req.send().await?;
        let _ = res.error_for_status()?;

        Ok(())
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

        let mut tags = di.tags.clone();
        tags.insert("application_name".into(), di.application_name.clone());
        tags.insert("device_name".into(), di.device_name.clone());
        tags.insert("dev_eui".into(), di.dev_eui.clone());

        let mut measurements: Vec<Measurement> = Vec::new();
        measurements.push(Measurement {
            name: "device_uplink".into(),
            tags: {
                let mut tags = tags.clone();
                tags.insert(
                    "frequency".into(),
                    format!("{}", pl.tx_info.as_ref().unwrap().frequency),
                );
                tags.insert("dr".into(), format!("{}", pl.dr));
                tags
            },
            values: {
                let mut v: HashMap<String, Value> = HashMap::new();
                v.insert("value".into(), Value::Integer(1));
                v.insert("f_cnt".into(), Value::Integer(pl.f_cnt.into()));
                v.insert(
                    "rssi".into(),
                    Value::Integer(pl.rx_info.iter().max_by_key(|x| x.rssi).unwrap().rssi as i64),
                );
                v.insert(
                    "snr".into(),
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
                v
            },
        });

        tags.insert("f_port".into(), format!("{}", pl.f_port));

        if let Some(obj) = &pl.object {
            measurements.append(&mut struct_to_measurements(&tags, obj));
        }

        self.publish(&measurements).await?;

        info!(dev_eui = %di.dev_eui, "Uplink measurements sent to InfluxDB");

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
        pl: &integration::StatusEvent,
    ) -> Result<()> {
        let di = pl.device_info.as_ref().unwrap();

        let mut tags = di.tags.clone();
        tags.insert("application_name".into(), di.application_name.clone());
        tags.insert("device_name".into(), di.device_name.clone());
        tags.insert("dev_eui".into(), di.dev_eui.clone());

        let mut measurements: Vec<Measurement> = Vec::new();
        if !pl.external_power_source && !pl.battery_level_unavailable {
            measurements.push(Measurement {
                name: "device_status_battery_level".into(),
                tags: tags.clone(),
                values: {
                    let mut v: HashMap<String, Value> = HashMap::new();
                    v.insert("value".into(), Value::Float(pl.battery_level as f64));
                    v
                },
            });
        }

        measurements.push(Measurement {
            name: "device_status_margin".into(),
            tags: tags.clone(),
            values: {
                let mut v: HashMap<String, Value> = HashMap::new();
                v.insert("value".into(), Value::Integer(pl.margin as i64));
                v
            },
        });

        self.publish(&measurements).await?;

        info!(dev_eui = %di.dev_eui, "Status measurements sent to InfluxDB");

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

#[derive(Clone)]
enum Value {
    Bool(bool),
    Float(f64),
    String(String),
    Integer(i64),
}

impl Value {
    fn to_string(&self, quote: bool) -> String {
        match self {
            Value::Bool(v) => format!("{}", v),
            Value::Float(v) => format!("{:.6}", v),
            Value::String(v) => match quote {
                false => v.to_string(),
                true => format!("\"{}\"", v.replace('"', "\\\"")),
            },
            Value::Integer(v) => format!("{}i", v),
        }
    }
}

struct Measurement {
    name: String,
    tags: HashMap<String, String>,
    values: HashMap<String, Value>,
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();

        for (k, v) in &self.tags {
            tags.push(format!("{}={}", escape_influx_tag(k), escape_influx_tag(v)));
        }

        for (k, v) in &self.values {
            values.push(format!("{}={}", k, v.to_string(true)));
        }

        tags.sort();
        values.sort();

        write!(f, "{},{} {}", self.name, tags.join(","), values.join(","))
    }
}

// see https://docs.influxdata.com/influxdb/v1.7/write_protocols/line_protocol_tutorial/#special-characters
fn escape_influx_tag(s: &str) -> String {
    let mut s = s.to_string();

    let replace: HashMap<String, String> = [
        (",".to_string(), "\\,".to_string()),
        ("=".to_string(), "\\=".to_string()),
        (" ".to_string(), "\\ ".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    for (k, v) in &replace {
        s = s.replace(k, v);
    }

    s
}

fn struct_to_measurements(
    tags: &HashMap<String, String>,
    s: &pbjson_types::Struct,
) -> Vec<Measurement> {
    let mut out: Vec<Measurement> = Vec::new();

    out.append(&mut struct_values_to_location(
        tags,
        "device_frmpayload_data",
        &s.fields,
    ));

    for (k, v) in &s.fields {
        if k == "latitude" || k == "longitude" {
            continue;
        }
        out.append(&mut struct_value_to_measurements(
            tags,
            &format!("device_frmpayload_data_{}", k),
            v,
        ))
    }

    out
}

fn struct_values_to_location(
    tags: &HashMap<String, String>,
    prefix: &str,
    v: &HashMap<String, pbjson_types::Value>,
) -> Vec<Measurement> {
    let mut out: Vec<Measurement> = Vec::new();
    let mut latitude: Option<f64> = None;
    let mut longitude: Option<f64> = None;

    if let Some(v) = v.get("latitude") {
        if let Some(pbjson_types::value::Kind::NumberValue(n)) = &v.kind {
            latitude = Some(*n);
        }
    }

    if let Some(v) = v.get("longitude") {
        if let Some(pbjson_types::value::Kind::NumberValue(n)) = &v.kind {
            longitude = Some(*n);
        }
    }

    if let (Some(lat), Some(lon)) = (latitude, longitude) {
        out.push(Measurement {
            name: format!("{}_location", prefix),
            tags: tags.clone(),
            values: [
                ("latitude".to_string(), Value::Float(lat)),
                ("longitude".to_string(), Value::Float(lon)),
                (
                    "geohash".to_string(),
                    Value::String(
                        geohash::encode(geohash::Coord { x: lon, y: lat }, 12).unwrap_or_default(),
                    ),
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        });
    }

    out
}

fn struct_value_to_measurements(
    tags: &HashMap<String, String>,
    prefix: &str,
    v: &pbjson_types::Value,
) -> Vec<Measurement> {
    let mut out: Vec<Measurement> = Vec::new();

    if let Some(k) = &v.kind {
        match k {
            pbjson_types::value::Kind::NullValue(_) => {}
            pbjson_types::value::Kind::NumberValue(v) => {
                out.push(Measurement {
                    name: prefix.to_string(),
                    tags: tags.clone(),
                    values: [("value".to_string(), Value::Float(*v))]
                        .iter()
                        .cloned()
                        .collect(),
                });
            }
            pbjson_types::value::Kind::StringValue(v) => {
                out.push(Measurement {
                    name: prefix.to_string(),
                    tags: tags.clone(),
                    values: [("value".to_string(), Value::String(v.to_string()))]
                        .iter()
                        .cloned()
                        .collect(),
                });
            }
            pbjson_types::value::Kind::BoolValue(v) => {
                out.push(Measurement {
                    name: prefix.to_string(),
                    tags: tags.clone(),
                    values: [("value".to_string(), Value::Bool(*v))]
                        .iter()
                        .cloned()
                        .collect(),
                });
            }
            pbjson_types::value::Kind::StructValue(sv) => {
                out.append(&mut struct_values_to_location(tags, prefix, &sv.fields));

                for (k, v) in &sv.fields {
                    if k == "latitude" || k == "longitude" {
                        continue;
                    }

                    let prefix = format!("{}_{}", prefix, k);
                    out.append(&mut struct_value_to_measurements(tags, &prefix, v));
                }
            }
            pbjson_types::value::Kind::ListValue(v) => {
                for (i, v) in v.values.iter().enumerate() {
                    let prefix = format!("{}_{}", prefix, i);
                    out.append(&mut struct_value_to_measurements(tags, &prefix, v));
                }
            }
        }
    }

    out
}

#[cfg(test)]
pub mod test {
    use super::*;
    use chirpstack_api::gw;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_v1() {
        let server = MockServer::start();

        let i = Integration {
            timeout: Duration::from_secs(5),
            endpoint: server.url("/write"),
            version: InfluxDbVersion::Influxdb1,
            db: "testdb".into(),
            username: "foo".into(),
            password: "bar".into(),
            retention_policy_name: "DEFAULT".into(),
            precision: "s".into(),
            token: "".into(),
            organization: "".into(),
            bucket: "".into(),
        };

        // status
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/write")
                .query_param("db", "testdb")
                .query_param("precision", "s")
                .query_param("rp", "DEFAULT")
                .header_exists("Authorization")
                .body(r#"device_status_battery_level,application_name=test-app,dev_eui=0102030405060708,device_name=test-device,foo=bar value=48.430000
device_status_margin,application_name=test-app,dev_eui=0102030405060708,device_name=test-device,foo=bar value=10i"#);
            then.status(200);
        });
        i.status_event(
            &HashMap::new(),
            &integration::StatusEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".into(),
                    device_name: "test-device".into(),
                    dev_eui: "0102030405060708".into(),
                    tags: [("foo".to_string(), "bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                battery_level: 48.43,
                margin: 10,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        mock.assert();
        mock.delete();

        // one level depth
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/write")
                .query_param("db", "testdb")
                .query_param("precision", "s")
                .query_param("rp", "DEFAULT")
                .header_exists("Authorization")
                .body(r#"device_frmpayload_data_active,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=true
device_frmpayload_data_humidity,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=20.000000
device_frmpayload_data_status,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value="on"
device_frmpayload_data_temperature,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=25.400000
device_uplink,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,dr=2,fo\ o=ba\,r,frequency=868100000 f_cnt=10i,rssi=-55i,snr=2.500000,value=1i"#);
            then.status(200);
        });
        i.uplink_event(
            &HashMap::new(),
            &integration::UplinkEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".into(),
                    device_name: "test-dev".into(),
                    dev_eui: "0102030405060708".into(),
                    tags: [("fo o".to_string(), "ba,r".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                tx_info: Some(gw::UplinkTxInfo {
                    frequency: 868100000,
                    ..Default::default()
                }),
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
                dr: 2,
                f_cnt: 10,
                f_port: 20,
                object: Some(pbjson_types::Struct {
                    fields: [
                        (
                            "temperature".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::NumberValue(25.4)),
                            },
                        ),
                        (
                            "humidity".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::NumberValue(20.0)),
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
        mock.assert();
        mock.delete();

        // multiple levels
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/write")
                .query_param("db", "testdb")
                .query_param("precision", "s")
                .query_param("rp", "DEFAULT")
                .header_exists("Authorization")
                .body(r#"device_frmpayload_data_active,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=true
device_frmpayload_data_humidity,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=20.000000
device_frmpayload_data_status,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value="on"
device_frmpayload_data_temperature_a,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=20.500000
device_frmpayload_data_temperature_b,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=33.300000
device_uplink,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,dr=2,fo\ o=ba\,r,frequency=868100000 f_cnt=10i,rssi=-55i,snr=2.500000,value=1i"#);
            then.status(200);
        });
        i.uplink_event(
            &HashMap::new(),
            &integration::UplinkEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".into(),
                    device_name: "test-dev".into(),
                    dev_eui: "0102030405060708".into(),
                    tags: [("fo o".to_string(), "ba,r".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                tx_info: Some(gw::UplinkTxInfo {
                    frequency: 868100000,
                    ..Default::default()
                }),
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
                dr: 2,
                f_cnt: 10,
                f_port: 20,
                object: Some(pbjson_types::Struct {
                    fields: [
                        (
                            "temperature".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::StructValue(
                                    pbjson_types::Struct {
                                        fields: [
                                            (
                                                "a".to_string(),
                                                pbjson_types::Value {
                                                    kind: Some(
                                                        pbjson_types::value::Kind::NumberValue(
                                                            20.5,
                                                        ),
                                                    ),
                                                },
                                            ),
                                            (
                                                "b".to_string(),
                                                pbjson_types::Value {
                                                    kind: Some(
                                                        pbjson_types::value::Kind::NumberValue(
                                                            33.3,
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ]
                                        .iter()
                                        .cloned()
                                        .collect(),
                                    },
                                )),
                            },
                        ),
                        (
                            "humidity".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::NumberValue(20.0)),
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
        mock.assert();
        mock.delete();

        // latitude and longitude
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/write")
                .query_param("db", "testdb")
                .query_param("precision", "s")
                .query_param("rp", "DEFAULT")
                .header_exists("Authorization")
                .body(r#"device_frmpayload_data_active,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value=true
device_frmpayload_data_location,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r geohash="s01w2k3vvqre",latitude=1.123000,longitude=2.123000
device_frmpayload_data_status,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,f_port=20,fo\ o=ba\,r value="on"
device_uplink,application_name=test-app,dev_eui=0102030405060708,device_name=test-dev,dr=2,fo\ o=ba\,r,frequency=868100000 f_cnt=10i,rssi=-55i,snr=2.500000,value=1i"#);
            then.status(200);
        });
        i.uplink_event(
            &HashMap::new(),
            &integration::UplinkEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".into(),
                    device_name: "test-dev".into(),
                    dev_eui: "0102030405060708".into(),
                    tags: [("fo o".to_string(), "ba,r".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                tx_info: Some(gw::UplinkTxInfo {
                    frequency: 868100000,
                    ..Default::default()
                }),
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
                dr: 2,
                f_cnt: 10,
                f_port: 20,
                object: Some(pbjson_types::Struct {
                    fields: [
                        (
                            "latitude".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::NumberValue(1.123)),
                            },
                        ),
                        (
                            "longitude".to_string(),
                            pbjson_types::Value {
                                kind: Some(pbjson_types::value::Kind::NumberValue(2.123)),
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
        mock.assert();
        mock.delete();
    }

    #[tokio::test]
    async fn test_v2() {
        let server = MockServer::start();

        let i = Integration {
            timeout: Duration::from_secs(5),
            endpoint: server.url("/write"),
            version: InfluxDbVersion::Influxdb2,
            db: "".into(),
            username: "".into(),
            password: "".into(),
            retention_policy_name: "".into(),
            precision: "".into(),
            token: "testtoken".into(),
            organization: "testorg".into(),
            bucket: "testbucket".into(),
        };

        // status
        let mut mock = server.mock(|when, then| {
            when.method(POST)
                .path("/write")
                .query_param("org", "testorg")
                .query_param("bucket", "testbucket")
                .header("Authorization", "Token testtoken")
                .body(r#"device_status_battery_level,application_name=test-app,dev_eui=0102030405060708,device_name=test-device,foo=bar value=48.430000
device_status_margin,application_name=test-app,dev_eui=0102030405060708,device_name=test-device,foo=bar value=10i"#);
            then.status(200);
        });
        i.status_event(
            &HashMap::new(),
            &integration::StatusEvent {
                device_info: Some(integration::DeviceInfo {
                    application_name: "test-app".into(),
                    device_name: "test-device".into(),
                    dev_eui: "0102030405060708".into(),
                    tags: [("foo".to_string(), "bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                    ..Default::default()
                }),
                battery_level: 48.43,
                margin: 10,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        mock.assert();
        mock.delete();
    }
}
