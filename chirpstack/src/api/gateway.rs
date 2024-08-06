use std::collections::HashSet;
use std::str::FromStr;
use std::time::SystemTime;

use chrono::{DateTime, Duration, Local, Utc};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api::gateway_service_server::GatewayService;
use chirpstack_api::{api, common};
use lrwn::EUI64;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers::{self, FromProto};
use crate::certificate;
use crate::storage::{
    fields,
    gateway::{self, RelayId},
    metrics,
};

pub struct Gateway {
    validator: validator::RequestValidator,
}

impl Gateway {
    pub fn new(validator: validator::RequestValidator) -> Self {
        Gateway { validator }
    }
}

#[tonic::async_trait]
impl GatewayService for Gateway {
    async fn create(
        &self,
        request: Request<api::CreateGatewayRequest>,
    ) -> Result<Response<()>, Status> {
        let req_gw = match &request.get_ref().gateway {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("gateway is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_gw.tenant_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(validator::Flag::Create, tenant_id),
            )
            .await?;

        let (lat, lon, alt) = match &req_gw.location {
            Some(v) => (v.latitude, v.longitude, v.altitude as f32),
            None => (0.0, 0.0, 0.0),
        };

        let gw = gateway::Gateway {
            gateway_id: EUI64::from_str(&req_gw.gateway_id).map_err(|e| e.status())?,
            tenant_id: tenant_id.into(),
            name: req_gw.name.clone(),
            description: req_gw.description.clone(),
            latitude: lat,
            longitude: lon,
            altitude: alt,
            tags: fields::KeyValue::new(req_gw.tags.clone()),
            stats_interval_secs: req_gw.stats_interval as i32,
            ..Default::default()
        };

        let _ = gateway::create(gw).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-gateway_id", req_gw.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn get(
        &self,
        request: Request<api::GetGatewayRequest>,
    ) -> Result<Response<api::GetGatewayResponse>, Status> {
        let req = request.get_ref();
        let gw_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Read, gw_id),
            )
            .await?;

        let gw = gateway::get(&gw_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetGatewayResponse {
            gateway: Some(api::Gateway {
                gateway_id: gw.gateway_id.to_string(),
                name: gw.name,
                description: gw.description,
                location: Some(common::Location {
                    latitude: gw.latitude,
                    longitude: gw.longitude,
                    altitude: gw.altitude as f64,
                    ..Default::default()
                }),
                tenant_id: gw.tenant_id.to_string(),
                tags: gw.tags.into_hashmap(),
                metadata: gw.properties.into_hashmap(),
                stats_interval: gw.stats_interval_secs as u32,
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&gw.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&gw.updated_at)),
            last_seen_at: gw
                .last_seen_at
                .as_ref()
                .map(helpers::datetime_to_prost_timestamp),
        });
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn update(
        &self,
        request: Request<api::UpdateGatewayRequest>,
    ) -> Result<Response<()>, Status> {
        let req_gw = match &request.get_ref().gateway {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("gateway is missing"));
            }
        };
        let gw_id = EUI64::from_str(&req_gw.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Update, gw_id),
            )
            .await?;

        let (lat, lon, alt) = match &req_gw.location {
            Some(v) => (v.latitude, v.longitude, v.altitude as f32),
            None => (0.0, 0.0, 0.0),
        };

        // update
        let _ = gateway::update(gateway::Gateway {
            gateway_id: gw_id,
            name: req_gw.name.clone(),
            description: req_gw.description.clone(),
            latitude: lat,
            longitude: lon,
            altitude: alt,
            tags: fields::KeyValue::new(req_gw.tags.clone()),
            stats_interval_secs: req_gw.stats_interval as i32,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-gateway_id", req_gw.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn delete(
        &self,
        request: Request<api::DeleteGatewayRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let gw_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Delete, gw_id),
            )
            .await?;

        gateway::delete(&gw_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn list(
        &self,
        request: Request<api::ListGatewaysRequest>,
    ) -> Result<Response<api::ListGatewaysResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = if req.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?)
        };
        let mg_id: Option<Uuid> = if req.multicast_group_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?)
        };

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(
                    validator::Flag::List,
                    tenant_id.unwrap_or(Uuid::nil()),
                ),
            )
            .await?;

        if let Some(mg_id) = mg_id {
            self.validator
                .validate(
                    request.extensions(),
                    validator::ValidateMulticastGroupAccess::new(validator::Flag::Read, mg_id),
                )
                .await?;
        }

        let filters = gateway::Filters {
            tenant_id,
            multicast_group_id: mg_id,
            search: if req.search.is_empty() {
                None
            } else {
                Some(req.search.to_string())
            },
        };

        let count = gateway::get_count(&filters).await.map_err(|e| e.status())?;
        let result = gateway::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListGatewaysResponse {
            total_count: count as u32,
            result: result
                .iter()
                .map(|gw| api::GatewayListItem {
                    tenant_id: gw.tenant_id.to_string(),
                    gateway_id: gw.gateway_id.to_string(),
                    name: gw.name.clone(),
                    description: gw.description.clone(),
                    location: Some(common::Location {
                        latitude: gw.latitude,
                        longitude: gw.longitude,
                        altitude: gw.altitude as f64,
                        ..Default::default()
                    }),
                    properties: gw.properties.into_hashmap(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&gw.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&gw.updated_at)),
                    last_seen_at: gw
                        .last_seen_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    state: {
                        if let Some(ts) = gw.last_seen_at {
                            if (Utc::now() - ts)
                                > Duration::try_seconds((gw.stats_interval_secs * 2).into())
                                    .unwrap_or_default()
                            {
                                api::GatewayState::Offline
                            } else {
                                api::GatewayState::Online
                            }
                        } else {
                            api::GatewayState::NeverSeen
                        }
                    }
                    .into(),
                })
                .collect(),
        });
        if !req.tenant_id.is_empty() {
            resp.metadata_mut()
                .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());
        }

        Ok(resp)
    }

    async fn generate_client_certificate(
        &self,
        request: Request<api::GenerateGatewayClientCertificateRequest>,
    ) -> Result<Response<api::GenerateGatewayClientCertificateResponse>, Status> {
        let req = request.get_ref();
        let gw_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Update, gw_id),
            )
            .await?;

        let (ttl, ca_cert, cert, key) = certificate::client_cert_for_gateway_id(&gw_id)
            .await
            .map_err(|e| e.status())?;

        gateway::partial_update(
            gw_id,
            &gateway::GatewayChangeset {
                tls_certificate: Some(Some(cert.as_bytes().to_vec())),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(api::GenerateGatewayClientCertificateResponse {
            ca_cert,
            tls_cert: cert,
            tls_key: key,
            expires_at: Some(ttl.into()),
        });
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn get_metrics(
        &self,
        request: Request<api::GetGatewayMetricsRequest>,
    ) -> Result<Response<api::GetGatewayMetricsResponse>, Status> {
        let req = request.get_ref();
        let gateway_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Read, gateway_id),
            )
            .await?;

        let start = SystemTime::try_from(
            req.start
                .as_ref()
                .ok_or_else(|| anyhow!("start is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let end = SystemTime::try_from(
            req.end
                .as_ref()
                .ok_or_else(|| anyhow!("end is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let start: DateTime<Local> = start.into();
        let end: DateTime<Local> = end.into();
        let aggregation = req.aggregation().from_proto();

        let gw_metrics = metrics::get(
            &format!("gw:{}", gateway_id),
            metrics::Kind::ABSOLUTE,
            aggregation,
            start,
            end,
        )
        .await
        .map_err(|e| e.status())?;

        let out = api::GetGatewayMetricsResponse {
            rx_packets: Some(common::Metric {
                name: "Received".to_string(),
                timestamps: gw_metrics
                    .iter()
                    .map(|row| {
                        let ts: DateTime<Utc> = row.time.into();
                        let ts: pbjson_types::Timestamp = ts.into();
                        ts
                    })
                    .collect(),
                datasets: vec![common::MetricDataset {
                    label: "rx_count".to_string(),
                    data: gw_metrics
                        .iter()
                        .map(|row| row.metrics.get("rx_count").cloned().unwrap_or(0.0) as f32)
                        .collect(),
                }],
                kind: common::MetricKind::Absolute.into(),
            }),
            tx_packets: Some(common::Metric {
                name: "Transmitted".to_string(),
                timestamps: gw_metrics
                    .iter()
                    .map(|row| {
                        let ts: DateTime<Utc> = row.time.into();
                        let ts: pbjson_types::Timestamp = ts.into();
                        ts
                    })
                    .collect(),
                datasets: vec![common::MetricDataset {
                    label: "tx_count".to_string(),
                    data: gw_metrics
                        .iter()
                        .map(|row| row.metrics.get("tx_count").cloned().unwrap_or(0.0) as f32)
                        .collect(),
                }],
                kind: common::MetricKind::Absolute.into(),
            }),
            tx_packets_per_freq: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &gw_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("tx_freq_") {
                            datasets.insert(k.trim_start_matches("tx_freq_").to_string());
                        }
                    }
                }

                common::Metric {
                    name: "Transmitted / frequency".to_string(),
                    timestamps: gw_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|label| common::MetricDataset {
                            label: label.to_string(),
                            data: gw_metrics
                                .iter()
                                .map(|row| {
                                    row.metrics
                                        .get(&format!("tx_freq_{}", label))
                                        .cloned()
                                        .unwrap_or(0.0) as f32
                                })
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
            rx_packets_per_freq: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &gw_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("rx_freq_") {
                            datasets.insert(k.trim_start_matches("rx_freq_").to_string());
                        }
                    }
                }

                common::Metric {
                    name: "Received / frequency".to_string(),
                    timestamps: gw_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|label| common::MetricDataset {
                            label: label.to_string(),
                            data: gw_metrics
                                .iter()
                                .map(|row| {
                                    row.metrics
                                        .get(&format!("rx_freq_{}", label))
                                        .cloned()
                                        .unwrap_or(0.0) as f32
                                })
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
            rx_packets_per_dr: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &gw_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("rx_dr_") {
                            datasets.insert(k.trim_start_matches("rx_dr_").to_string());
                        }
                    }
                }

                common::Metric {
                    name: "Received / DR".to_string(),
                    timestamps: gw_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|label| common::MetricDataset {
                            label: label.to_string(),
                            data: gw_metrics
                                .iter()
                                .map(|row| {
                                    row.metrics
                                        .get(&format!("rx_dr_{}", label))
                                        .cloned()
                                        .unwrap_or(0.0) as f32
                                })
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
            tx_packets_per_dr: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &gw_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("tx_dr_") {
                            datasets.insert(k.trim_start_matches("tx_dr_").to_string());
                        }
                    }
                }

                common::Metric {
                    name: "Transmitted / DR".to_string(),
                    timestamps: gw_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|label| common::MetricDataset {
                            label: label.to_string(),
                            data: gw_metrics
                                .iter()
                                .map(|row| {
                                    row.metrics
                                        .get(&format!("tx_dr_{}", label))
                                        .cloned()
                                        .unwrap_or(0.0) as f32
                                })
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
            tx_packets_per_status: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &gw_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("tx_status_") {
                            datasets.insert(k.trim_start_matches("tx_status_").to_string());
                        }
                    }
                }

                common::Metric {
                    name: "TX packets / status".to_string(),
                    timestamps: gw_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|label| common::MetricDataset {
                            label: label.to_string(),
                            data: gw_metrics
                                .iter()
                                .map(|row| {
                                    row.metrics
                                        .get(&format!("tx_status_{}", label))
                                        .cloned()
                                        .unwrap_or(0.0) as f32
                                })
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
        };

        let mut resp = Response::new(out);
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn get_duty_cycle_metrics(
        &self,
        request: Request<api::GetGatewayDutyCycleMetricsRequest>,
    ) -> Result<Response<api::GetGatewayDutyCycleMetricsResponse>, Status> {
        let req = request.get_ref();
        let gateway_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Read, gateway_id),
            )
            .await?;

        let start = SystemTime::try_from(
            req.start
                .as_ref()
                .ok_or_else(|| anyhow!("start is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let end = SystemTime::try_from(
            req.end
                .as_ref()
                .ok_or_else(|| anyhow!("end is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let start: DateTime<Local> = start.into();
        let end: DateTime<Local> = end.into();

        let dc_metrics = metrics::get(
            &format!("gw:dc:{}", gateway_id),
            metrics::Kind::COUNTER,
            metrics::Aggregation::MINUTE,
            start,
            end,
        )
        .await
        .map_err(|e| e.status())?;

        let out = api::GetGatewayDutyCycleMetricsResponse {
            max_load_percentage: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &dc_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("max_load_perc_") {
                            datasets.insert(k.to_string());
                        }
                    }
                }

                common::Metric {
                    name: "Percentage of max tx duty-cycle".into(),
                    timestamps: dc_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|key| common::MetricDataset {
                            label: {
                                let s = key.strip_prefix("max_load_perc_").unwrap_or_default();
                                let s: Vec<&str> = s.split('_').collect();

                                format!(
                                    "{} ({:.2}MHz - {:.2}MHz: {:.2}%)",
                                    s.first().unwrap_or(&""),
                                    s.get(1)
                                        .unwrap_or(&"")
                                        .parse::<f64>()
                                        .map(|v| v / 1_000_000.0)
                                        .unwrap_or(0.0),
                                    s.get(2)
                                        .unwrap_or(&"")
                                        .parse::<f64>()
                                        .map(|v| v / 1_000_000.0)
                                        .unwrap_or(0.0),
                                    s.get(3)
                                        .unwrap_or(&"")
                                        .parse::<f64>()
                                        .map(|v| v / 10.0)
                                        .unwrap_or(0.0),
                                )
                            },
                            data: dc_metrics
                                .iter()
                                .map(|row| row.metrics.get(key).cloned().unwrap_or(0.0) as f32)
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
            window_percentage: Some({
                // discover all data-sets
                let mut datasets: HashSet<String> = HashSet::new();
                for m in &dc_metrics {
                    for k in m.metrics.keys() {
                        if k.starts_with("window_perc_") {
                            datasets.insert(k.to_string());
                        }
                    }
                }

                common::Metric {
                    name: "Tx duty-cycle".into(),
                    timestamps: dc_metrics
                        .iter()
                        .map(|row| {
                            let ts: DateTime<Utc> = row.time.into();
                            let ts: pbjson_types::Timestamp = ts.into();
                            ts
                        })
                        .collect(),
                    datasets: datasets
                        .iter()
                        .map(|key| common::MetricDataset {
                            label: {
                                let s = key.strip_prefix("window_perc_").unwrap_or_default();
                                let s: Vec<&str> = s.split('_').collect();

                                format!(
                                    "{} ({:.2}MHz - {:.2}MHz: {:.2}%)",
                                    s.first().unwrap_or(&""),
                                    s.get(1)
                                        .unwrap_or(&"")
                                        .parse::<f64>()
                                        .map(|v| v / 1_000_000.0)
                                        .unwrap_or(0.0),
                                    s.get(2)
                                        .unwrap_or(&"")
                                        .parse::<f64>()
                                        .map(|v| v / 1_000_000.0)
                                        .unwrap_or(0.0),
                                    s.get(3)
                                        .unwrap_or(&"")
                                        .parse::<f64>()
                                        .map(|v| v / 10.0)
                                        .unwrap_or(0.0),
                                )
                            },
                            data: dc_metrics
                                .iter()
                                .map(|row| row.metrics.get(key).cloned().unwrap_or(0.0) as f32)
                                .collect(),
                        })
                        .collect(),
                    kind: common::MetricKind::Absolute.into(),
                }
            }),
        };

        let mut resp = Response::new(out);
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn get_relay_gateway(
        &self,
        request: Request<api::GetRelayGatewayRequest>,
    ) -> Result<Response<api::GetRelayGatewayResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;
        let relay_id = RelayId::from_str(&req.relay_id).map_err(|e| e.status())?;

        // The tenant_id is part of the relay PK.
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        let relay = gateway::get_relay_gateway(tenant_id, relay_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetRelayGatewayResponse {
            relay_gateway: Some(api::RelayGateway {
                tenant_id: relay.tenant_id.to_string(),
                relay_id: relay.relay_id.to_string(),
                name: relay.name,
                description: relay.description,
                stats_interval: relay.stats_interval_secs as u32,
                region_config_id: relay.region_config_id.to_string(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&relay.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&relay.updated_at)),
            last_seen_at: relay
                .last_seen_at
                .as_ref()
                .map(helpers::datetime_to_prost_timestamp),
        });

        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-relay_id", req.relay_id.parse().unwrap());

        Ok(resp)
    }

    async fn update_relay_gateway(
        &self,
        request: Request<api::UpdateRelayGatewayRequest>,
    ) -> Result<Response<()>, Status> {
        let req_relay = match &request.get_ref().relay_gateway {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("relay_gateway is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_relay.tenant_id).map_err(|e| e.status())?;
        let relay_id = RelayId::from_str(&req_relay.relay_id).map_err(|e| e.status())?;

        // The tenant_id is part of the relay PK.
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        let _ = gateway::update_relay_gateway(gateway::RelayGateway {
            relay_id,
            tenant_id: tenant_id.into(),
            name: req_relay.name.clone(),
            description: req_relay.description.clone(),
            stats_interval_secs: req_relay.stats_interval as i32,
            region_config_id: req_relay.region_config_id.clone(),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req_relay.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-relay_id", req_relay.relay_id.parse().unwrap());

        Ok(resp)
    }

    async fn delete_relay_gateway(
        &self,
        request: Request<api::DeleteRelayGatewayRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;
        let relay_id = RelayId::from_str(&req.relay_id).map_err(|e| e.status())?;

        // The tenant_id is part of the relay PK.
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        gateway::delete_relay_gateway(tenant_id, relay_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-relay_id", req.relay_id.parse().unwrap());

        Ok(resp)
    }

    async fn list_relay_gateways(
        &self,
        request: Request<api::ListRelayGatewaysRequest>,
    ) -> Result<Response<api::ListRelayGatewaysResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = if req.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?)
        };

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(
                    validator::Flag::List,
                    tenant_id.unwrap_or(Uuid::nil()),
                ),
            )
            .await?;

        let filters = gateway::RelayGatewayFilters { tenant_id };

        let count = gateway::get_relay_gateway_count(&filters)
            .await
            .map_err(|e| e.status())?;
        let result = gateway::list_relay_gateways(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListRelayGatewaysResponse {
            total_count: count as u32,
            result: result
                .iter()
                .map(|r| api::RelayGatewayListItem {
                    tenant_id: r.tenant_id.to_string(),
                    relay_id: r.relay_id.to_string(),
                    name: r.name.clone(),
                    description: r.description.clone(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&r.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&r.updated_at)),
                    last_seen_at: r
                        .last_seen_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    state: {
                        if let Some(ts) = r.last_seen_at {
                            if (Utc::now() - ts)
                                > Duration::try_seconds((r.stats_interval_secs * 2).into())
                                    .unwrap_or_default()
                            {
                                api::GatewayState::Offline
                            } else {
                                api::GatewayState::Online
                            }
                        } else {
                            api::GatewayState::NeverSeen
                        }
                    }
                    .into(),
                    region_config_id: r.region_config_id.to_string(),
                })
                .collect(),
        });
        if !req.tenant_id.is_empty() {
            resp.metadata_mut()
                .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());
        }

        Ok(resp)
    }
}

#[cfg(test)]
pub mod test {
    use chrono::{Datelike, Local, TimeZone, Timelike};
    use std::collections::HashMap;

    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::metrics;
    use crate::storage::{tenant, user};
    use crate::test;

    #[tokio::test]
    async fn test_gateway() {
        let _guard = test::prepare().await;

        // setup admin user
        let u = user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        };
        let u = user::create(u).await.unwrap();

        // create tenant
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            max_gateway_count: 10,
            ..Default::default()
        })
        .await
        .unwrap();

        // setup api
        let service = Gateway::new(RequestValidator::new());

        // create
        let create_req = api::CreateGatewayRequest {
            gateway: Some(api::Gateway {
                gateway_id: "0102030405060708".into(),
                tenant_id: t.id.to_string(),
                name: "test-gw".into(),
                location: Some(common::Location {
                    latitude: 1.1,
                    longitude: 1.2,
                    altitude: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };
        let mut create_req = Request::new(create_req);
        create_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let _ = service.create(create_req).await.unwrap();

        // get
        let get_req = api::GetGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut get_req = Request::new(get_req);
        get_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Gateway {
                gateway_id: "0102030405060708".into(),
                tenant_id: t.id.to_string(),
                name: "test-gw".into(),
                location: Some(common::Location {
                    latitude: 1.1,
                    longitude: 1.2,
                    altitude: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            get_resp.get_ref().gateway
        );

        // update
        let up_req = api::UpdateGatewayRequest {
            gateway: Some(api::Gateway {
                gateway_id: "0102030405060708".into(),
                tenant_id: t.id.to_string(),
                name: "updated-gw".into(),
                location: Some(common::Location {
                    latitude: 2.1,
                    longitude: 2.2,
                    altitude: 2.0,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        };
        let mut up_req = Request::new(up_req);
        up_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let _ = service.update(up_req).await.unwrap();

        // get
        let get_req = api::GetGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut get_req = Request::new(get_req);
        get_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Gateway {
                gateway_id: "0102030405060708".into(),
                tenant_id: t.id.to_string(),
                name: "updated-gw".into(),
                location: Some(common::Location {
                    latitude: 2.1,
                    longitude: 2.2,
                    altitude: 2.0,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            get_resp.get_ref().gateway
        );

        // list
        let list_req = api::ListGatewaysRequest {
            search: "updated".into(),
            tenant_id: t.id.to_string(),
            limit: 10,
            offset: 0,
            ..Default::default()
        };
        let mut list_req = Request::new(list_req);
        list_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let list_resp = service.list(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);
        assert_eq!(1, list_resp.get_ref().result.len());

        // delete
        let del_req = api::DeleteGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut del_req = Request::new(del_req);
        del_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let _ = service.delete(del_req).await.unwrap();

        let del_req = api::DeleteGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut del_req = Request::new(del_req);
        del_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }

    #[tokio::test]
    async fn test_gateway_stats() {
        let _guard = test::prepare().await;

        // setup admin user
        let u = user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        };
        let u = user::create(u).await.unwrap();

        // create tenant
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            max_gateway_count: 10,
            ..Default::default()
        })
        .await
        .unwrap();

        // create gateway
        let _ = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            tenant_id: t.id.clone(),
            name: "test-gw".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let now = Local::now();

        // insert stats
        let mut m = metrics::Record {
            kind: metrics::Kind::ABSOLUTE,
            time: now.into(),
            metrics: HashMap::new(),
        };

        m.metrics.insert("rx_count".into(), 10.0);
        m.metrics.insert("rx_freq_868100000".into(), 10.0);
        m.metrics.insert("rx_dr_5".into(), 10.0);

        m.metrics.insert("tx_count".into(), 5.0);
        m.metrics.insert("tx_freq_868200000".into(), 5.0);
        m.metrics.insert("tx_dr_4".into(), 5.0);

        metrics::save(
            "gw:0102030405060708",
            &m,
            &metrics::Aggregation::default_aggregations(),
        )
        .await
        .unwrap();

        // setup api
        let service = Gateway::new(RequestValidator::new());

        // request stats
        let now_st: SystemTime = now.into();
        let stats_req = api::GetGatewayMetricsRequest {
            gateway_id: "0102030405060708".into(),
            start: Some(now_st.into()),
            end: Some(now_st.into()),
            aggregation: common::Aggregation::Day.into(),
        };
        let mut stats_req = Request::new(stats_req);
        stats_req
            .extensions_mut()
            .insert(AuthID::User(Into::<uuid::Uuid>::into(u.id).clone()));
        let stats_resp = service.get_metrics(stats_req).await.unwrap();
        let stats_resp = stats_resp.get_ref();
        assert_eq!(
            Some(common::Metric {
                name: "Received".to_string(),
                timestamps: vec![{
                    let ts = Local
                        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
                        .unwrap();
                    //let ts: SystemTime = ts.into();
                    let ts: DateTime<Utc> = ts.into();
                    ts.into()
                }],
                datasets: vec![common::MetricDataset {
                    label: "rx_count".to_string(),
                    data: vec![10.0],
                }],
                kind: common::MetricKind::Absolute.into(),
            }),
            stats_resp.rx_packets
        );
    }

    #[tokio::test]
    async fn test_gateway_duty_cycle_stats() {
        let _guard = test::prepare().await;

        // setup admin user
        let u = user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        };
        let u = user::create(u).await.unwrap();

        // create tenant
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            max_gateway_count: 10,
            ..Default::default()
        })
        .await
        .unwrap();

        // create gateway
        let _ = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            tenant_id: t.id.clone(),
            name: "test-gw".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let now = Local::now();

        // insert stats
        let mut m = metrics::Record {
            kind: metrics::Kind::COUNTER,
            time: now.into(),
            metrics: HashMap::new(),
        };

        m.metrics
            .insert("window_perc_M_868000000_868600000_10".into(), 0.5);
        m.metrics
            .insert("max_load_perc_L_865000000_868000000_10".into(), 5.0);

        metrics::save(
            "gw:dc:0102030405060708",
            &m,
            &[metrics::Aggregation::MINUTE],
        )
        .await
        .unwrap();

        // setup api
        let service = Gateway::new(RequestValidator::new());

        // request stats
        let now_st: SystemTime = now.into();
        let stats_req = api::GetGatewayDutyCycleMetricsRequest {
            gateway_id: "0102030405060708".into(),
            start: Some(now_st.into()),
            end: Some(now_st.into()),
        };
        let mut stats_req = Request::new(stats_req);
        stats_req.extensions_mut().insert(AuthID::User(u.id.into()));
        let stats_resp = service.get_duty_cycle_metrics(stats_req).await.unwrap();
        let stats_resp = stats_resp.get_ref();
        assert_eq!(
            Some(common::Metric {
                name: "Percentage of max tx duty-cycle".into(),
                timestamps: vec![{
                    let ts = Local
                        .with_ymd_and_hms(
                            now.year(),
                            now.month(),
                            now.day(),
                            now.hour(),
                            now.minute(),
                            0,
                        )
                        .unwrap();
                    //let ts: SystemTime = ts.into();
                    let ts: DateTime<Utc> = ts.into();
                    ts.into()
                }],
                datasets: vec![common::MetricDataset {
                    label: "L (865.00MHz - 868.00MHz: 1.00%)".into(),
                    data: vec![5.0],
                }],
                kind: common::MetricKind::Absolute.into(),
            }),
            stats_resp.max_load_percentage
        );
        assert_eq!(
            Some(common::Metric {
                name: "Tx duty-cycle".into(),
                timestamps: vec![{
                    let ts = Local
                        .with_ymd_and_hms(
                            now.year(),
                            now.month(),
                            now.day(),
                            now.hour(),
                            now.minute(),
                            0,
                        )
                        .unwrap();
                    //let ts: SystemTime = ts.into();
                    let ts: DateTime<Utc> = ts.into();
                    ts.into()
                }],
                datasets: vec![common::MetricDataset {
                    label: "M (868.00MHz - 868.60MHz: 1.00%)".into(),
                    data: vec![0.5],
                }],
                kind: common::MetricKind::Absolute.into(),
            }),
            stats_resp.window_percentage
        );

        // create relay gateway
        let _ = gateway::create_relay_gateway(gateway::RelayGateway {
            tenant_id: t.id,
            relay_id: gateway::RelayId::from_be_bytes([1, 2, 3, 4]),
            name: "test-relay".into(),
            description: "test relay".into(),
            region_config_id: "eu868".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // get relay gateway
        let get_relay_req = api::GetRelayGatewayRequest {
            tenant_id: t.id.to_string(),
            relay_id: "01020304".into(),
        };
        let mut get_relay_req = Request::new(get_relay_req);
        get_relay_req
            .extensions_mut()
            .insert(AuthID::User(u.id.into()));
        let get_relay_resp = service.get_relay_gateway(get_relay_req).await.unwrap();
        assert_eq!(
            Some(api::RelayGateway {
                tenant_id: t.id.to_string(),
                relay_id: "01020304".into(),
                name: "test-relay".into(),
                description: "test relay".into(),
                stats_interval: 900,
                region_config_id: "eu868".into(),
            }),
            get_relay_resp.get_ref().relay_gateway
        );

        // update
        let up_relay_req = api::UpdateRelayGatewayRequest {
            relay_gateway: Some(api::RelayGateway {
                tenant_id: t.id.to_string(),
                relay_id: "01020304".into(),
                name: "updated-relay".into(),
                description: "updated relay".into(),
                stats_interval: 600,
                region_config_id: "us915_0".into(),
            }),
        };
        let mut up_relay_req = Request::new(up_relay_req);
        up_relay_req
            .extensions_mut()
            .insert(AuthID::User(u.id.into()));
        let _ = service.update_relay_gateway(up_relay_req).await.unwrap();

        // get relay gateway
        let get_relay_req = api::GetRelayGatewayRequest {
            tenant_id: t.id.to_string(),
            relay_id: "01020304".into(),
        };
        let mut get_relay_req = Request::new(get_relay_req);
        get_relay_req
            .extensions_mut()
            .insert(AuthID::User(u.id.into()));
        let get_relay_resp = service.get_relay_gateway(get_relay_req).await.unwrap();
        assert_eq!(
            Some(api::RelayGateway {
                tenant_id: t.id.to_string(),
                relay_id: "01020304".into(),
                name: "updated-relay".into(),
                description: "updated relay".into(),
                stats_interval: 600,
                region_config_id: "us915_0".into(),
            }),
            get_relay_resp.get_ref().relay_gateway
        );

        // list
        let list_relay_req = api::ListRelayGatewaysRequest {
            tenant_id: t.id.to_string(),
            limit: 10,
            offset: 0,
        };
        let mut list_relay_req = Request::new(list_relay_req);
        list_relay_req
            .extensions_mut()
            .insert(AuthID::User(u.id.into()));
        let list_relay_resp = service.list_relay_gateways(list_relay_req).await.unwrap();
        assert_eq!(1, list_relay_resp.get_ref().total_count);
        assert_eq!(1, list_relay_resp.get_ref().result.len());

        // delete
        let del_relay_req = api::DeleteRelayGatewayRequest {
            tenant_id: t.id.to_string(),
            relay_id: "01020304".into(),
        };
        let mut del_relay_req = Request::new(del_relay_req);
        del_relay_req
            .extensions_mut()
            .insert(AuthID::User(u.id.into()));
        let del_relay_resp = service.delete_relay_gateway(del_relay_req).await;
        assert!(del_relay_resp.is_ok());

        let del_relay_req = api::DeleteRelayGatewayRequest {
            tenant_id: t.id.to_string(),
            relay_id: "01020304".into(),
        };
        let mut del_relay_req = Request::new(del_relay_req);
        del_relay_req
            .extensions_mut()
            .insert(AuthID::User(u.id.into()));
        let del_relay_resp = service.delete_relay_gateway(del_relay_req).await;
        assert!(del_relay_resp.is_err());
    }
}
