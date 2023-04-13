use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use std::time::Duration;

use anyhow::{Context as AnyhowContext, Result};
use futures::Stream;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Client;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::{debug, error, trace};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::internal_service_server::InternalService;

use super::auth::claims;
use super::auth::{validator, AuthID};
use super::error::ToStatus;
use super::helpers::ToProto;
use super::{helpers, oidc};
use crate::storage::{api_key, device, error::Error, gateway, redis_key, search, tenant, user};
use crate::{config, eventlog, framelog, region};
use lrwn::EUI64;

pub struct Internal {
    validator: validator::RequestValidator,
    jwt_secret: String,
}

impl Internal {
    pub fn new(validator: validator::RequestValidator, jwt_secret: String) -> Self {
        Internal {
            validator,
            jwt_secret,
        }
    }

    async fn create_and_provision_user(&self, oidc_user: &oidc::User) -> Result<user::User> {
        let external_id = oidc_user.subject().to_string();
        let email = match oidc_user.email() {
            Some(v) => v.to_string(),
            None => {
                return Err(anyhow!("email is missing"));
            }
        };
        let email_verified = oidc_user.email_verified().unwrap_or_default();

        let u = user::User {
            is_active: true,
            email,
            email_verified,
            external_id: Some(external_id),
            ..Default::default()
        };
        let u = user::create(u).await?;
        if let Err(e) = self.provision_user(&u.id, oidc_user).await {
            error!(error = %e, "Provisioning user failed");
            user::delete(&u.id).await?;
            return Err(e);
        }

        Ok(u)
    }

    async fn provision_user(&self, user_id: &Uuid, oidc_user: &oidc::User) -> Result<()> {
        let conf = config::get();
        if conf
            .user_authentication
            .openid_connect
            .registration_callback_url
            .is_empty()
        {
            return Ok(());
        }

        let client = Client::builder().timeout(Duration::from_secs(5)).build()?;
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let res = client
            .post(
                &conf
                    .user_authentication
                    .openid_connect
                    .registration_callback_url,
            )
            .json(&oidc_user)
            .query(&[("user_id", user_id.to_string())])
            .headers(headers)
            .send()
            .await?;

        match res.error_for_status().context("Provision request error") {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

pub struct DropReceiver<T> {
    inner: ReceiverStream<T>,
    close_chan: mpsc::Sender<()>,
}

impl<T> DropReceiver<T> {
    pub fn new(t: ReceiverStream<T>) -> (Self, mpsc::Receiver<()>) {
        let (tx, rx) = mpsc::channel(1);
        (
            DropReceiver {
                inner: t,
                close_chan: tx,
            },
            rx,
        )
    }
}

impl<T> Stream for DropReceiver<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

impl<T> Drop for DropReceiver<T> {
    fn drop(&mut self) {
        trace!("DropReceiver drop method called");
        let _ = self.close_chan.try_send(());
    }
}

#[tonic::async_trait]
impl InternalService for Internal {
    async fn login(
        &self,
        request: Request<api::LoginRequest>,
    ) -> Result<Response<api::LoginResponse>, Status> {
        let req = request.get_ref();
        let u = user::get_by_email_and_pw(&req.email, &req.password)
            .await
            .map_err(|e| e.status())?;

        let token = claims::AuthClaim::new_for_user(&u.id)
            .encode(self.jwt_secret.as_ref())
            .map_err(|e| e.status())?;

        Ok(Response::new(api::LoginResponse { jwt: token }))
    }

    async fn profile(
        &self,
        request: Request<()>,
    ) -> Result<Response<api::ProfileResponse>, Status> {
        self.validator
            .validate(request.extensions(), validator::ValidateActiveUser::new())
            .await?;

        let auth_id = request.extensions().get::<AuthID>().unwrap();
        let id = match auth_id {
            AuthID::User(id) => id,
            _ => {
                return Err(Status::internal("no user id"));
            }
        };

        let u = user::get(id).await.map_err(|e| e.status())?;
        let items = tenant::get_tenant_users_for_user(id)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::ProfileResponse {
            user: Some(api::User {
                id: u.id.to_string(),
                email: u.email,
                is_active: u.is_active,
                is_admin: u.is_admin,
                note: u.note,
            }),
            tenants: items
                .iter()
                .map(|i| api::UserTenantLink {
                    created_at: Some(helpers::datetime_to_prost_timestamp(&i.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&i.updated_at)),
                    tenant_id: i.tenant_id.to_string(),
                    is_admin: i.is_admin,
                    is_device_admin: i.is_device_admin,
                    is_gateway_admin: i.is_gateway_admin,
                })
                .collect(),
        }))
    }

    async fn global_search(
        &self,
        request: Request<api::GlobalSearchRequest>,
    ) -> Result<Response<api::GlobalSearchResponse>, Status> {
        let req = request.get_ref();
        self.validator
            .validate(request.extensions(), validator::ValidateActiveUser::new())
            .await?;

        let auth_id = request.extensions().get::<AuthID>().unwrap();
        let user_id = match auth_id {
            AuthID::User(id) => id,
            _ => {
                return Err(Status::unauthenticated("no user id"));
            }
        };

        let u = user::get(user_id).await.map_err(|e| e.status())?;

        let items = search::global_search(
            &u.id,
            u.is_admin,
            &req.search,
            req.limit as usize,
            req.offset as usize,
        )
        .await
        .map_err(|e| e.status())?;

        Ok(Response::new(api::GlobalSearchResponse {
            result: items
                .iter()
                .map(|r| api::GlobalSearchResult {
                    kind: r.kind.clone(),
                    score: r.score,
                    tenant_id: match &r.tenant_id {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    },
                    tenant_name: match &r.tenant_name {
                        Some(v) => v.clone(),
                        None => "".to_string(),
                    },
                    application_id: match &r.application_id {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    },
                    application_name: match &r.application_name {
                        Some(v) => v.clone(),
                        None => "".to_string(),
                    },
                    device_dev_eui: match &r.device_dev_eui {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    },
                    device_name: match &r.device_name {
                        Some(v) => v.clone(),
                        None => "".to_string(),
                    },
                    gateway_id: match &r.gateway_id {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    },
                    gateway_name: match &r.gateway_name {
                        Some(v) => v.clone(),
                        None => "".to_string(),
                    },
                })
                .collect(),
        }))
    }

    async fn create_api_key(
        &self,
        request: Request<api::CreateApiKeyRequest>,
    ) -> Result<Response<api::CreateApiKeyResponse>, Status> {
        let req_key = match &request.get_ref().api_key {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("api_key is missing"));
            }
        };

        let tenant_id = if req_key.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req_key.tenant_id).map_err(|e| e.status())?)
        };

        if req_key.is_admin && tenant_id.is_some() {
            return Err(Status::invalid_argument(
                "tenant_id can not be set with is_admin set to true",
            ));
        }

        if !req_key.is_admin && tenant_id.is_none() {
            return Err(Status::invalid_argument(
                "either is_admin or tenant_id must be set",
            ));
        }

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApiKeysAccess::new(validator::Flag::Create, tenant_id),
            )
            .await?;

        let ak = api_key::ApiKey {
            name: req_key.name.clone(),
            is_admin: req_key.is_admin,
            tenant_id,
            ..Default::default()
        };

        let ak = api_key::create(ak).await.map_err(|e| e.status())?;
        let token = claims::AuthClaim::new_for_api_key(&ak.id)
            .encode(self.jwt_secret.as_ref())
            .map_err(|e| e.status())?;

        Ok(Response::new(api::CreateApiKeyResponse {
            id: ak.id.to_string(),
            token,
        }))
    }

    async fn delete_api_key(
        &self,
        request: Request<api::DeleteApiKeyRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let api_key_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApiKeyAccess::new(validator::Flag::Delete, api_key_id),
            )
            .await?;

        api_key::delete(&api_key_id).await.map_err(|e| e.status())?;
        Ok(Response::new(()))
    }

    async fn list_api_keys(
        &self,
        request: Request<api::ListApiKeysRequest>,
    ) -> Result<Response<api::ListApiKeysResponse>, Status> {
        let req = request.get_ref();

        let tenant_id = if req.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?)
        };

        if req.is_admin && tenant_id.is_some() {
            return Err(Status::invalid_argument(
                "tenant_id can not be set with is_admin set to true",
            ));
        }

        if !req.is_admin && tenant_id.is_none() {
            return Err(Status::invalid_argument(
                "either is_admin or tenant_id must be set",
            ));
        }

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApiKeysAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        let filters = api_key::Filters {
            is_admin: req.is_admin,
            tenant_id,
        };

        let count = api_key::get_count(&filters).await.map_err(|e| e.status())?;
        let results = api_key::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::ListApiKeysResponse {
            total_count: count as u32,
            result: results
                .iter()
                .map(|ak| api::ApiKey {
                    id: ak.id.to_string(),
                    name: ak.name.clone(),
                    is_admin: ak.is_admin,
                    tenant_id: match ak.tenant_id {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    },
                })
                .collect(),
        }))
    }

    async fn settings(
        &self,
        _request: Request<()>,
    ) -> Result<Response<api::SettingsResponse>, Status> {
        let conf = config::get();

        Ok(Response::new(api::SettingsResponse {
            openid_connect: Some(api::OpenIdConnect {
                enabled: conf.user_authentication.openid_connect.enabled,
                login_url: "/auth/oidc/login".into(),
                login_label: conf.user_authentication.openid_connect.login_label.clone(),
                logout_url: conf.user_authentication.openid_connect.logout_url.clone(),
            }),
        }))
    }

    async fn open_id_connect_login(
        &self,
        request: Request<api::OpenIdConnectLoginRequest>,
    ) -> Result<Response<api::OpenIdConnectLoginResponse>, Status> {
        let req = request.get_ref();
        let conf = config::get();
        let oidc_user = oidc::get_user(&req.code, &req.state)
            .await
            .map_err(|e| e.status())?;

        let external_id = oidc_user.subject().to_string();
        let email = match oidc_user.email() {
            Some(v) => v.to_string(),
            None => {
                return Err(Status::invalid_argument("email is missing"));
            }
        };
        let email_verified = oidc_user.email_verified().unwrap_or_default();

        if !email_verified {
            return Err(Status::failed_precondition(
                "email address must be verified before you can login",
            ));
        }

        // try to get user by external id
        let mut u: Option<user::User> = match user::get_by_external_id(&external_id).await {
            Ok(v) => Some(v),
            Err(e) => match e {
                Error::NotFound(_) => None,
                _ => {
                    return Err(e.status());
                }
            },
        };

        // try to get user by email and set external id
        if u.is_none() {
            u = match user::get_by_email(&email).await {
                Ok(mut v) => {
                    v.external_id = Some(external_id);
                    Some(v)
                }
                Err(e) => match e {
                    Error::NotFound(_) => None,
                    _ => {
                        return Err(e.status());
                    }
                },
            };
        }

        // register the user (if enabled)
        if u.is_none() && conf.user_authentication.openid_connect.registration_enabled {
            u = Some(
                self.create_and_provision_user(&oidc_user)
                    .await
                    .map_err(|e| e.status())?,
            );
        }

        let mut u = match u {
            Some(v) => v,
            None => {
                return Err(Status::not_found("User does not exist"));
            }
        };

        // update the user
        // in case it was fetched using the external id, this will make sure we sync with any
        // possible email change.
        u.email = email;
        u.email_verified = email_verified;
        let u = user::update(u).await.map_err(|e| e.status())?;

        let token = claims::AuthClaim::new_for_user(&u.id)
            .encode(self.jwt_secret.as_ref())
            .map_err(|e| e.status())?;
        Ok(Response::new(api::OpenIdConnectLoginResponse { token }))
    }

    async fn get_devices_summary(
        &self,
        request: Request<api::GetDevicesSummaryRequest>,
    ) -> Result<Response<api::GetDevicesSummaryResponse>, Status> {
        let req = request.get_ref();

        let tenant_id = if req.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?)
        };

        if tenant_id.is_none() {
            self.validator
                .validate(request.extensions(), validator::ValidateIsAdmin::new())
                .await?;
        } else {
            self.validator
                .validate(
                    request.extensions(),
                    validator::ValidateTenantAccess::new(
                        validator::Flag::Read,
                        *tenant_id.as_ref().unwrap(),
                    ),
                )
                .await?;
        }

        let active_inactive = device::get_active_inactive(&tenant_id)
            .await
            .map_err(|e| e.status())?;

        let dr_count = device::get_data_rates(&tenant_id)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::GetDevicesSummaryResponse {
            active_count: active_inactive.active_count as u32,
            inactive_count: active_inactive.inactive_count as u32,
            never_seen_count: active_inactive.never_seen_count as u32,
            dr_count: dr_count
                .iter()
                .map(|i| (i.dr.unwrap() as u32, i.count as u32))
                .collect(),
        }))
    }

    async fn get_gateways_summary(
        &self,
        request: Request<api::GetGatewaysSummaryRequest>,
    ) -> Result<Response<api::GetGatewaysSummaryResponse>, Status> {
        let req = request.get_ref();

        let tenant_id = if req.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?)
        };

        if tenant_id.is_none() {
            self.validator
                .validate(request.extensions(), validator::ValidateIsAdmin::new())
                .await?;
        } else {
            self.validator
                .validate(
                    request.extensions(),
                    validator::ValidateTenantAccess::new(
                        validator::Flag::Read,
                        *tenant_id.as_ref().unwrap(),
                    ),
                )
                .await?;
        }

        let counts = gateway::get_counts_by_state(&tenant_id)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::GetGatewaysSummaryResponse {
            online_count: counts.online_count as u32,
            offline_count: counts.offline_count as u32,
            never_seen_count: counts.never_seen_count as u32,
        }))
    }

    type StreamGatewayFramesStream = DropReceiver<Result<api::LogItem, Status>>;

    async fn stream_gateway_frames(
        &self,
        request: Request<api::StreamGatewayFramesRequest>,
    ) -> Result<Response<Self::StreamGatewayFramesStream>, Status> {
        let req = request.get_ref();
        let gw_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateGatewayAccess::new(validator::Flag::Read, gw_id),
            )
            .await?;

        let key = redis_key(format!("gw:{{{}}}:stream:frame", req.gateway_id));
        let (redis_tx, mut redis_rx) = mpsc::channel(1);
        let (stream_tx, stream_rx) = mpsc::channel(1);

        let mut framelog_future = Box::pin(framelog::get_frame_logs(key, 10, redis_tx));
        let (drop_receiver, mut close_rx) = DropReceiver::new(ReceiverStream::new(stream_rx));

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // detect client disconnect
                    _ = close_rx.recv() => {
                        debug!("Client disconnected");
                        break;
                    }
                    // detect get_frame_logs function return
                    res = &mut framelog_future => {
                        match res {
                            Ok(_) => {
                                trace!("get_frame_logs returned");
                            },
                            Err(e) => {
                                error!("Reading frame-log returned error: {}", e);
                                stream_tx.send(Err(e.status())).await.unwrap();
                            },
                        }
                        break;
                    }
                    // detect stream message
                    msg = redis_rx.recv() => {
                        match msg {
                            None => {
                                trace!("Redis Stream channel has been closed");
                                break;
                            },
                            Some(msg) => {
                                trace!("Message received from Redis Stream channel");
                                if  stream_tx.send(Ok(msg)).await.is_err() {
                                    error!("Sending message to gRPC channel error");
                                    break;
                                };
                            },
                        }
                    }
                }
            }
        });

        Ok(Response::new(drop_receiver))
    }

    type StreamDeviceFramesStream = DropReceiver<Result<api::LogItem, Status>>;

    async fn stream_device_frames(
        &self,
        request: Request<api::StreamDeviceFramesRequest>,
    ) -> Result<Response<Self::StreamDeviceFramesStream>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, dev_eui),
            )
            .await?;

        let key = redis_key(format!("device:{{{}}}:stream:frame", req.dev_eui));
        let (redis_tx, mut redis_rx) = mpsc::channel(1);
        let (stream_tx, stream_rx) = mpsc::channel(1);

        let mut framelog_future = Box::pin(framelog::get_frame_logs(key, 10, redis_tx));
        let (drop_receiver, mut close_rx) = DropReceiver::new(ReceiverStream::new(stream_rx));

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // detect client disconnect
                    _ = close_rx.recv() => {
                        debug!("Client disconnected");
                        redis_rx.close();
                        break;
                    }
                    // detect get_frame_logs function return
                    res = &mut framelog_future => {
                        match res {
                            Ok(_) => {
                                trace!("get_frame_logs returned");
                            },
                            Err(e) => {
                                error!("Reading frame-log returned error: {}", e);
                                stream_tx.send(Err(e.status())).await.unwrap();
                            },
                        }
                        break;
                    }
                    // detect stream message
                    msg = redis_rx.recv() => {
                        match msg {
                            None => {
                                trace!("Redis Stream channel has been closed");
                                break;
                            },
                            Some(msg) => {
                                trace!("Message received from Redis Stream channel");
                                if stream_tx.send(Ok(msg)).await.is_err() {
                                    error!("Sending message to gRPC channel error");
                                    break;
                                };
                            },
                        }
                    }
                }
            }
        });

        Ok(Response::new(drop_receiver))
    }

    type StreamDeviceEventsStream = DropReceiver<Result<api::LogItem, Status>>;

    async fn stream_device_events(
        &self,
        request: Request<api::StreamDeviceEventsRequest>,
    ) -> Result<Response<Self::StreamDeviceEventsStream>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, dev_eui),
            )
            .await?;

        let key = redis_key(format!("device:{{{}}}:stream:event", req.dev_eui));
        let (redis_tx, mut redis_rx) = mpsc::channel(1);
        let (stream_tx, stream_rx) = mpsc::channel(1);

        let mut eventlog_future = Box::pin(eventlog::get_event_logs(key, 10, redis_tx));
        let (drop_receiver, mut close_rx) = DropReceiver::new(ReceiverStream::new(stream_rx));

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // detect client disconnect
                    _ = close_rx.recv() => {
                        debug!("Client disconnected");
                        redis_rx.close();
                        break;
                    },
                    // detect get_event_logs function return
                    res = &mut eventlog_future => {
                        match res {
                            Ok(_) => {
                                trace!("get_event_logs returned");
                            },
                            Err(e) => {
                                error!("Reading event-log returned error: {}", e);
                                stream_tx.send(Err(e.status())).await.unwrap();
                            },
                        }
                        break;
                    }
                    // detect stream message
                    msg = redis_rx.recv() => {
                        match msg {
                            None => {
                                trace!("Redis Stream channel has been closed");
                                break;
                            },
                            Some(msg) => {
                                trace!("Message received from Redis Stream channel");
                                if stream_tx.send(Ok(msg)).await.is_err() {
                                    error!("Sending message to gRPC channel error");
                                    break;
                                };
                            },
                        }
                    }
                }
            }
        });

        Ok(Response::new(drop_receiver))
    }

    async fn list_regions(
        &self,
        request: Request<()>,
    ) -> Result<Response<api::ListRegionsResponse>, Status> {
        self.validator
            .validate(request.extensions(), validator::ValidateActiveUser::new())
            .await?;

        let conf = config::get();

        let mut out: api::ListRegionsResponse = Default::default();

        for region_config in &conf.regions {
            // Check if region is enabled.
            if !conf.network.enabled_regions.contains(&region_config.id) {
                continue;
            }

            out.regions.push(api::RegionListItem {
                id: region_config.id.clone(),
                description: if region_config.description.is_empty() {
                    region_config.id.clone()
                } else {
                    region_config.description.clone()
                },
                region: region_config.common_name.to_proto().into(),
            });
        }

        out.regions.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Response::new(out))
    }

    async fn get_region(
        &self,
        request: Request<api::GetRegionRequest>,
    ) -> Result<Response<api::GetRegionResponse>, Status> {
        let req = request.get_ref();
        self.validator
            .validate(request.extensions(), validator::ValidateActiveUser::new())
            .await?;

        let conf = config::get();
        let reg = region::get(&req.id).map_err(|e| e.status())?;

        let mut out = api::GetRegionResponse {
            ..Default::default()
        };

        for region_conf in &conf.regions {
            if req.id == region_conf.id {
                out.id = region_conf.id.clone();
                out.description = if region_conf.description.is_empty() {
                    region_conf.id.clone()
                } else {
                    region_conf.description.clone()
                };
                out.region = region_conf.common_name.to_proto().into();
                out.user_info = region_conf.user_info.clone();
                out.rx1_delay = region_conf.network.rx1_delay as u32;
                out.rx1_dr_offset = region_conf.network.rx1_dr_offset as u32;
                out.rx2_dr = region_conf.network.rx2_dr as u32;
                out.rx2_frequency = region_conf.network.rx2_frequency;
                out.class_b_ping_slot_dr = region_conf.network.class_b.ping_slot_dr as u32;
                out.class_b_ping_slot_frequency = region_conf.network.class_b.ping_slot_frequency;
            }
        }

        let enabled_channels = reg.get_enabled_uplink_channel_indices();

        for i in enabled_channels {
            let ch = reg.get_uplink_channel(i).map_err(|e| e.status())?;
            out.uplink_channels.push(api::RegionChannel {
                frequency: ch.frequency,
                dr_min: ch.min_dr as u32,
                dr_max: ch.max_dr as u32,
            });
        }

        Ok(Response::new(out))
    }
}
