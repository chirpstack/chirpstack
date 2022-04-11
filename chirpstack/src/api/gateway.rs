use std::str::FromStr;
use std::time::SystemTime;

use chrono::{DateTime, Local};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api::gateway_service_server::GatewayService;
use chirpstack_api::{api, common};
use lrwn::EUI64;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers;
use crate::certificate;
use crate::storage::{fields, gateway, metrics};

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
            tenant_id,
            name: req_gw.name.clone(),
            description: req_gw.description.clone(),
            latitude: lat,
            longitude: lon,
            altitude: alt,
            tags: fields::KeyValue::new(req_gw.tags.clone()),
            ..Default::default()
        };

        let _ = gateway::create(gw).await.map_err(|e| e.status())?;

        Ok(Response::new(()))
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

        Ok(Response::new(api::GetGatewayResponse {
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
                properties: gw.properties.into_hashmap(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&gw.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&gw.updated_at)),
            last_seen_at: gw
                .last_seen_at
                .as_ref()
                .map(helpers::datetime_to_prost_timestamp),
        }))
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
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        Ok(Response::new(()))
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
        Ok(Response::new(()))
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

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewaysAccess::new(
                    validator::Flag::List,
                    tenant_id.unwrap_or(Uuid::nil()),
                ),
            )
            .await?;

        let filters = gateway::Filters {
            tenant_id,
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

        Ok(Response::new(api::ListGatewaysResponse {
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
                })
                .collect(),
        }))
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

        gateway::update_tls_cert(&gw_id, cert.as_bytes())
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(
            api::GenerateGatewayClientCertificateResponse {
                ca_cert,
                tls_cert: cert,
                tls_key: key,
                expires_at: Some(ttl.into()),
            },
        ))
    }

    async fn get_stats(
        &self,
        request: Request<api::GetGatewayStatsRequest>,
    ) -> Result<Response<api::GetGatewayStatsResponse>, Status> {
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
                .ok_or(anyhow!("start is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let end = SystemTime::try_from(
            req.end
                .as_ref()
                .ok_or(anyhow!("end is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let start: DateTime<Local> = start.into();
        let end: DateTime<Local> = end.into();

        let gw_metrics = metrics::get(
            &format!("gw:{}", gateway_id),
            metrics::Aggregation::Day,
            start,
            end,
        )
        .await
        .map_err(|e| e.status())?;

        let mut out: api::GetGatewayStatsResponse = Default::default();

        for m in gw_metrics {
            let ts: SystemTime = m.time.into();
            let ts: prost_types::Timestamp = ts.into();

            let mut item = api::GatewayStats {
                time: Some(ts),
                ..Default::default()
            };

            item.rx_packets = m.metrics.get("rx_count").cloned().unwrap_or(0.0) as u32;
            item.tx_packets = m.metrics.get("tx_count").cloned().unwrap_or(0.0) as u32;

            for (k, v) in m.metrics {
                if k.starts_with("tx_freq_") {
                    let freq: u32 = k
                        .trim_start_matches("tx_freq_")
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.status())?;
                    item.tx_packets_per_frequency.insert(freq, v as u32);
                }

                if k.starts_with("rx_freq_") {
                    let freq: u32 = k
                        .trim_start_matches("rx_freq_")
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.status())?;
                    item.rx_packets_per_frequency.insert(freq, v as u32);
                }

                if k.starts_with("tx_status_") {
                    let code = k.trim_start_matches("tx_status_").to_string();
                    item.tx_packets_per_status.insert(code, v as u32);
                }

                if k.starts_with("tx_dr_") {
                    let dr: u32 = k
                        .trim_start_matches("tx_dr_")
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.status())?;
                    item.tx_packets_per_dr.insert(dr, v as u32);
                }

                if k.starts_with("rx_dr_") {
                    let dr: u32 = k
                        .trim_start_matches("rx_dr_")
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.status())?;
                    item.rx_packets_per_dr.insert(dr, v as u32);
                }
            }

            out.result.push(item);
        }

        Ok(Response::new(out))
    }
}

#[cfg(test)]
pub mod test {
    use chrono::{Datelike, Local, TimeZone};
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
            .insert(AuthID::User(u.id.clone()));
        let _ = service.create(create_req).await.unwrap();

        // get
        let get_req = api::GetGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
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
        up_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.update(up_req).await.unwrap();

        // get
        let get_req = api::GetGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
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
        };
        let mut list_req = Request::new(list_req);
        list_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let list_resp = service.list(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);
        assert_eq!(1, list_resp.get_ref().result.len());

        // delete
        let del_req = api::DeleteGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.delete(del_req).await.unwrap();

        let del_req = api::DeleteGatewayRequest {
            gateway_id: "0102030405060708".into(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
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
            time: now.into(),
            metrics: HashMap::new(),
        };

        m.metrics.insert("rx_count".into(), 10.0);
        m.metrics.insert("rx_freq_868100000".into(), 10.0);
        m.metrics.insert("rx_dr_5".into(), 10.0);

        m.metrics.insert("tx_count".into(), 5.0);
        m.metrics.insert("tx_freq_868200000".into(), 5.0);
        m.metrics.insert("tx_dr_4".into(), 5.0);

        metrics::save("gw:0102030405060708", &m).await.unwrap();

        // setup api
        let service = Gateway::new(RequestValidator::new());

        // request stats
        let now_st: SystemTime = now.into();
        let stats_req = api::GetGatewayStatsRequest {
            gateway_id: "0102030405060708".into(),
            start: Some(now_st.into()),
            end: Some(now_st.into()),
        };
        let mut stats_req = Request::new(stats_req);
        stats_req
            .extensions_mut()
            .insert(AuthID::User(u.id.clone()));
        let stats_resp = service.get_stats(stats_req).await.unwrap();
        let stats_resp = stats_resp.get_ref();
        assert_eq!(1, stats_resp.result.len());
        assert_eq!(
            api::GatewayStats {
                time: Some({
                    let ts = Local
                        .ymd(now.year(), now.month(), now.day())
                        .and_hms(0, 0, 0);
                    let ts: SystemTime = ts.into();
                    ts.into()
                }),
                rx_packets: 10,
                tx_packets: 5,
                rx_packets_per_frequency: [(868100000, 10)].iter().cloned().collect(),
                rx_packets_per_dr: [(5, 10)].iter().cloned().collect(),
                tx_packets_per_frequency: [(868200000, 5)].iter().cloned().collect(),
                tx_packets_per_dr: [(4, 5)].iter().cloned().collect(),
                ..Default::default()
            },
            stats_resp.result[0]
        );
    }
}
