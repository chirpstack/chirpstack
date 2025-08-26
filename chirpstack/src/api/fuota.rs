use std::str::FromStr;

use chirpstack_api::api;
use chirpstack_api::api::fuota_service_server::FuotaService;
use chirpstack_api::tonic::{self, Request, Response, Status};
use chrono::Utc;
use lrwn::EUI64;
use uuid::Uuid;

use crate::aeskey::get_random_aes_key;
use crate::api::auth::validator;
use crate::api::error::ToStatus;
use crate::api::helpers::{self, FromProto, ToProto};
use crate::devaddr::get_random_dev_addr;
use crate::storage::{fields, fuota};

pub struct Fuota {
    validator: validator::RequestValidator,
}

impl Fuota {
    pub fn new(validator: validator::RequestValidator) -> Self {
        Fuota { validator }
    }
}

#[tonic::async_trait]
impl FuotaService for Fuota {
    async fn create_deployment(
        &self,
        request: Request<api::CreateFuotaDeploymentRequest>,
    ) -> Result<Response<api::CreateFuotaDeploymentResponse>, Status> {
        let req_dp = match &request.get_ref().deployment {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("deployment is missing"));
            }
        };

        let app_id = Uuid::from_str(&req_dp.application_id).map_err(|e| e.status())?;
        let dp_id = Uuid::from_str(&req_dp.device_profile_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentsAccess::new(validator::Flag::Create, app_id),
            )
            .await?;

        let mut dp = fuota::FuotaDeployment {
            name: req_dp.name.clone(),
            application_id: app_id.into(),
            device_profile_id: dp_id.into(),
            multicast_addr: get_random_dev_addr(),
            multicast_key: get_random_aes_key(),
            multicast_group_type: match req_dp.multicast_group_type() {
                api::MulticastGroupType::ClassB => "B",
                api::MulticastGroupType::ClassC => "C",
            }
            .to_string(),
            multicast_class_c_scheduling_type: req_dp
                .multicast_class_c_scheduling_type()
                .from_proto(),
            multicast_dr: req_dp.multicast_dr as i16,
            multicast_class_b_ping_slot_periodicity: req_dp.multicast_class_b_ping_slot_periodicity
                as i16,
            multicast_frequency: req_dp.multicast_frequency as i64,
            multicast_timeout: req_dp.multicast_timeout as i16,
            unicast_max_retry_count: req_dp.unicast_max_retry_count as i16,
            fragmentation_fragment_size: req_dp.fragmentation_fragment_size as i16,
            fragmentation_redundancy_percentage: req_dp.fragmentation_redundancy_percentage as i16,
            fragmentation_session_index: req_dp.fragmentation_session_index as i16,
            fragmentation_matrix: req_dp.fragmentation_matrix as i16,
            fragmentation_block_ack_delay: req_dp.fragmentation_block_ack_delay as i16,
            fragmentation_descriptor: req_dp.fragmentation_descriptor.clone(),
            request_fragmentation_session_status: req_dp
                .request_fragmentation_session_status()
                .from_proto(),
            payload: req_dp.payload.clone(),
            on_complete_set_device_tags: fields::KeyValue::new(
                req_dp.on_complete_set_device_tags.clone(),
            ),
            ..Default::default()
        };
        if req_dp.calculate_fragmentation_fragment_size {
            dp.fragmentation_fragment_size = fuota::get_max_fragment_size(&dp)
                .await
                .map_err(|e| e.status())? as i16;
        }
        if req_dp.calculate_multicast_timeout {
            dp.multicast_timeout =
                fuota::get_multicast_timeout(&dp).map_err(|e| e.status())? as i16;
        }

        let dp = fuota::create_deployment(dp).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::CreateFuotaDeploymentResponse {
            id: dp.id.to_string(),
        });
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            dp.id.to_string().parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get_deployment(
        &self,
        request: Request<api::GetFuotaDeploymentRequest>,
    ) -> Result<Response<api::GetFuotaDeploymentResponse>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Read, dp_id),
            )
            .await?;

        let dp = fuota::get_deployment(dp_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetFuotaDeploymentResponse {
            deployment: Some(api::FuotaDeployment {
                id: dp.id.to_string(),
                application_id: dp.application_id.to_string(),
                device_profile_id: dp.device_profile_id.to_string(),
                name: dp.name.clone(),
                multicast_group_type: match dp.multicast_group_type.as_ref() {
                    "B" => api::MulticastGroupType::ClassB,
                    "C" => api::MulticastGroupType::ClassC,
                    _ => return Err(Status::invalid_argument("Invalid multicast_group_type")),
                }
                .into(),
                multicast_class_c_scheduling_type: dp
                    .multicast_class_c_scheduling_type
                    .to_proto()
                    .into(),
                multicast_dr: dp.multicast_dr as u32,
                multicast_class_b_ping_slot_periodicity: dp.multicast_class_b_ping_slot_periodicity
                    as u32,
                multicast_frequency: dp.multicast_frequency as u32,
                multicast_timeout: dp.multicast_timeout as u32,
                unicast_max_retry_count: dp.unicast_max_retry_count as u32,
                fragmentation_fragment_size: dp.fragmentation_fragment_size as u32,
                fragmentation_redundancy_percentage: dp.fragmentation_redundancy_percentage as u32,
                fragmentation_session_index: dp.fragmentation_session_index as u32,
                fragmentation_matrix: dp.fragmentation_matrix as u32,
                fragmentation_block_ack_delay: dp.fragmentation_block_ack_delay as u32,
                fragmentation_descriptor: dp.fragmentation_descriptor.clone(),
                request_fragmentation_session_status: dp
                    .request_fragmentation_session_status
                    .to_proto()
                    .into(),
                payload: dp.payload.clone(),
                calculate_multicast_timeout: false,
                calculate_fragmentation_fragment_size: false,
                on_complete_set_device_tags: dp.on_complete_set_device_tags.into_hashmap(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&dp.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&dp.updated_at)),
            started_at: dp
                .started_at
                .as_ref()
                .map(helpers::datetime_to_prost_timestamp),
            completed_at: dp
                .completed_at
                .as_ref()
                .map(helpers::datetime_to_prost_timestamp),
        });
        resp.metadata_mut()
            .insert("x-log-fuota_deployment_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn update_deployment(
        &self,
        request: Request<api::UpdateFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req_dp = match &request.get_ref().deployment {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("deployment is missing"));
            }
        };

        let id = Uuid::from_str(&req_dp.id).map_err(|e| e.status())?;
        let app_id = Uuid::from_str(&req_dp.application_id).map_err(|e| e.status())?;
        let dp_id = Uuid::from_str(&req_dp.device_profile_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Update, dp_id),
            )
            .await?;

        let d = fuota::get_deployment(id).await.map_err(|e| e.status())?;
        if d.started_at.is_some() {
            return Err(Status::failed_precondition(
                "FUOTA deployment has already started",
            ));
        }

        let mut dp = fuota::FuotaDeployment {
            id: id.into(),
            name: req_dp.name.clone(),
            application_id: app_id.into(),
            device_profile_id: dp_id.into(),
            multicast_group_type: match req_dp.multicast_group_type() {
                api::MulticastGroupType::ClassB => "B",
                api::MulticastGroupType::ClassC => "C",
            }
            .to_string(),
            multicast_class_c_scheduling_type: req_dp
                .multicast_class_c_scheduling_type()
                .from_proto(),
            multicast_dr: req_dp.multicast_dr as i16,
            multicast_class_b_ping_slot_periodicity: req_dp.multicast_class_b_ping_slot_periodicity
                as i16,
            multicast_frequency: req_dp.multicast_frequency as i64,
            multicast_timeout: req_dp.multicast_timeout as i16,
            unicast_max_retry_count: req_dp.unicast_max_retry_count as i16,
            fragmentation_fragment_size: req_dp.fragmentation_fragment_size as i16,
            fragmentation_redundancy_percentage: req_dp.fragmentation_redundancy_percentage as i16,
            fragmentation_session_index: req_dp.fragmentation_session_index as i16,
            fragmentation_matrix: req_dp.fragmentation_matrix as i16,
            fragmentation_block_ack_delay: req_dp.fragmentation_block_ack_delay as i16,
            fragmentation_descriptor: req_dp.fragmentation_descriptor.clone(),
            request_fragmentation_session_status: req_dp
                .request_fragmentation_session_status()
                .from_proto(),
            payload: req_dp.payload.clone(),
            on_complete_set_device_tags: fields::KeyValue::new(
                req_dp.on_complete_set_device_tags.clone(),
            ),
            ..Default::default()
        };
        if req_dp.calculate_fragmentation_fragment_size {
            dp.fragmentation_fragment_size = fuota::get_max_fragment_size(&dp)
                .await
                .map_err(|e| e.status())? as i16;
        }
        if req_dp.calculate_multicast_timeout {
            dp.multicast_timeout =
                fuota::get_multicast_timeout(&dp).map_err(|e| e.status())? as i16;
        }

        let _ = fuota::update_deployment(dp).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-fuota_deployment_id", req_dp.id.parse().unwrap());
        Ok(resp)
    }

    async fn delete_deployment(
        &self,
        request: Request<api::DeleteFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Delete, id),
            )
            .await?;

        let _ = fuota::delete_deployment(id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-fuota_deployment_id", req.id.parse().unwrap());
        Ok(resp)
    }

    async fn start_deployment(
        &self,
        request: Request<api::StartFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Update, id),
            )
            .await?;

        let mut d = fuota::get_deployment(id).await.map_err(|e| e.status())?;
        if d.started_at.is_some() {
            return Err(Status::failed_precondition(
                "FUOTA deployment has already started",
            ));
        }

        d.started_at = Some(Utc::now());
        let d = fuota::update_deployment(d).await.map_err(|e| e.status())?;

        fuota::create_job(fuota::FuotaDeploymentJob {
            fuota_deployment_id: d.id,
            job: fields::FuotaJob::CreateMcGroup,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-fuota_deployment_id", req.id.parse().unwrap());
        Ok(resp)
    }

    async fn list_deployments(
        &self,
        request: Request<api::ListFuotaDeploymentsRequest>,
    ) -> Result<Response<api::ListFuotaDeploymentsResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentsAccess::new(validator::Flag::List, app_id),
            )
            .await?;

        let count = fuota::get_deployment_count(app_id)
            .await
            .map_err(|e| e.status())?;
        let items = fuota::list_deployments(app_id, req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListFuotaDeploymentsResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|d| api::FuotaDeploymentListItem {
                    id: d.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&d.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&d.created_at)),
                    started_at: d
                        .started_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    completed_at: d
                        .completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    name: d.name.clone(),
                })
                .collect(),
        });
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn add_devices(
        &self,
        request: Request<api::AddDevicesToFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Update, dp_id),
            )
            .await?;

        let d = fuota::get_deployment(dp_id).await.map_err(|e| e.status())?;
        if d.started_at.is_some() {
            return Err(Status::failed_precondition(
                "FUOTA deployment has already started",
            ));
        }

        let mut dev_euis = Vec::with_capacity(req.dev_euis.len());
        for dev_eui in &req.dev_euis {
            dev_euis.push(EUI64::from_str(dev_eui).map_err(|e| e.status())?);
        }

        fuota::add_devices(dp_id, dev_euis)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );
        Ok(resp)
    }

    async fn remove_devices(
        &self,
        request: Request<api::RemoveDevicesFromFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Update, dp_id),
            )
            .await?;

        let mut dev_euis = Vec::with_capacity(req.dev_euis.len());
        for dev_eui in &req.dev_euis {
            dev_euis.push(EUI64::from_str(dev_eui).map_err(|e| e.status())?);
        }

        fuota::remove_devices(dp_id, dev_euis)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );
        Ok(resp)
    }

    async fn list_devices(
        &self,
        request: Request<api::ListFuotaDeploymentDevicesRequest>,
    ) -> Result<Response<api::ListFuotaDeploymentDevicesResponse>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Read, dp_id),
            )
            .await?;

        let count = fuota::get_device_count(dp_id)
            .await
            .map_err(|e| e.status())?;
        let items = fuota::get_devices(dp_id, req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListFuotaDeploymentDevicesResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|d| api::FuotaDeploymentDeviceListItem {
                    fuota_deployment_id: d.fuota_deployment_id.to_string(),
                    dev_eui: d.dev_eui.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&d.created_at)),
                    completed_at: d
                        .completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    mc_group_setup_completed_at: d
                        .mc_group_setup_completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    mc_session_completed_at: d
                        .mc_session_completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    frag_session_setup_completed_at: d
                        .frag_session_setup_completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    frag_status_completed_at: d
                        .frag_status_completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    error_msg: d.error_msg.clone(),
                })
                .collect(),
        });
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn add_gateways(
        &self,
        request: Request<api::AddGatewaysToFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Update, dp_id),
            )
            .await?;

        let d = fuota::get_deployment(dp_id).await.map_err(|e| e.status())?;
        if d.started_at.is_some() {
            return Err(Status::failed_precondition(
                "FUOTA deployment has already started",
            ));
        }

        let mut gateway_ids = Vec::with_capacity(req.gateway_ids.len());
        for gateway_id in &req.gateway_ids {
            gateway_ids.push(EUI64::from_str(gateway_id).map_err(|e| e.status())?);
        }

        fuota::add_gateways(dp_id, gateway_ids)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );
        Ok(resp)
    }

    async fn remove_gateways(
        &self,
        request: Request<api::RemoveGatewaysFromFuotaDeploymentRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Update, dp_id),
            )
            .await?;

        let mut gateway_ids = Vec::with_capacity(req.gateway_ids.len());
        for gateway_id in &req.gateway_ids {
            gateway_ids.push(EUI64::from_str(gateway_id).map_err(|e| e.status())?);
        }

        fuota::remove_gateways(dp_id, gateway_ids)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );
        Ok(resp)
    }

    async fn list_gateways(
        &self,
        request: Request<api::ListFuotaDeploymentGatewaysRequest>,
    ) -> Result<Response<api::ListFuotaDeploymentGatewaysResponse>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Read, dp_id),
            )
            .await?;

        let count = fuota::get_gateway_count(dp_id)
            .await
            .map_err(|e| e.status())?;
        let items = fuota::get_gateways(dp_id, req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListFuotaDeploymentGatewaysResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|gw| api::FuotaDeploymentGatewayListItem {
                    fuota_deployment_id: gw.fuota_deployment_id.to_string(),
                    gateway_id: gw.gateway_id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&gw.created_at)),
                })
                .collect(),
        });
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );
        Ok(resp)
    }

    async fn list_jobs(
        &self,
        request: Request<api::ListFuotaDeploymentJobsRequest>,
    ) -> Result<Response<api::ListFuotaDeploymentJobsResponse>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.fuota_deployment_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateFuotaDeploymentAccess::new(validator::Flag::Read, dp_id),
            )
            .await?;

        let jobs = fuota::list_jobs(dp_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListFuotaDeploymentJobsResponse {
            jobs: jobs
                .iter()
                .map(|j| api::FuotaDeploymentJob {
                    job: j.job.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&j.created_at)),
                    completed_at: j
                        .completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    max_retry_count: j.max_retry_count as u32,
                    attempt_count: j.attempt_count as u32,
                    scheduler_run_after: Some(helpers::datetime_to_prost_timestamp(
                        &j.scheduler_run_after,
                    )),
                    warning_msg: j.warning_msg.clone(),
                    error_msg: j.error_msg.clone(),
                })
                .collect(),
        });
        resp.metadata_mut().insert(
            "x-log-fuota_deployment_id",
            req.fuota_deployment_id.parse().unwrap(),
        );
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::{application, device, device_profile, gateway, tenant, user};
    use crate::test;

    #[tokio::test]
    async fn test_fuota() {
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
            ..Default::default()
        })
        .await
        .unwrap();

        // create app
        let app = application::create(application::Application {
            tenant_id: t.id,
            name: "test-app".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create dp
        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create device
        let dev = device::create(device::Device {
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // create gateway
        let gw = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            tenant_id: t.id,
            name: "test-gw".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // setup api
        let service = Fuota::new(RequestValidator::new());

        // create deployment
        let create_req = get_request(
            &u.id,
            api::CreateFuotaDeploymentRequest {
                deployment: Some(api::FuotaDeployment {
                    application_id: app.id.to_string(),
                    device_profile_id: dp.id.to_string(),
                    name: "test-fuota".into(),
                    ..Default::default()
                }),
            },
        );
        let create_resp = service.create_deployment(create_req).await.unwrap();
        let create_resp = create_resp.get_ref();

        // get deployment
        let get_req = get_request(
            &u.id,
            api::GetFuotaDeploymentRequest {
                id: create_resp.id.clone(),
            },
        );
        let get_resp = service.get_deployment(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::FuotaDeployment {
                id: create_resp.id.clone(),
                application_id: app.id.to_string(),
                device_profile_id: dp.id.to_string(),
                name: "test-fuota".into(),
                ..Default::default()
            }),
            get_resp.deployment
        );

        // update deployment
        let update_req = get_request(
            &u.id,
            api::UpdateFuotaDeploymentRequest {
                deployment: Some(api::FuotaDeployment {
                    id: create_resp.id.clone(),
                    application_id: app.id.to_string(),
                    device_profile_id: dp.id.to_string(),
                    name: "updated-test-fuota".into(),
                    ..Default::default()
                }),
            },
        );
        service.update_deployment(update_req).await.unwrap();
        let get_req = get_request(
            &u.id,
            api::GetFuotaDeploymentRequest {
                id: create_resp.id.clone(),
            },
        );
        let get_resp = service.get_deployment(get_req).await.unwrap();
        let get_resp = get_resp.get_ref();
        assert_eq!(
            Some(api::FuotaDeployment {
                id: create_resp.id.clone(),
                application_id: app.id.to_string(),
                device_profile_id: dp.id.to_string(),
                name: "updated-test-fuota".into(),
                ..Default::default()
            }),
            get_resp.deployment
        );

        // list deployments
        let list_req = get_request(
            &u.id,
            api::ListFuotaDeploymentsRequest {
                application_id: app.id.to_string(),
                limit: 10,
                offset: 0,
            },
        );
        let list_resp = service.list_deployments(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(1, list_resp.total_count);
        assert_eq!(1, list_resp.result.len());
        assert_eq!(create_resp.id, list_resp.result[0].id);

        // add device
        let add_dev_req = get_request(
            &u.id,
            api::AddDevicesToFuotaDeploymentRequest {
                fuota_deployment_id: create_resp.id.clone(),
                dev_euis: vec![dev.dev_eui.to_string()],
            },
        );
        service.add_devices(add_dev_req).await.unwrap();

        // list devices
        let list_devs_req = get_request(
            &u.id,
            api::ListFuotaDeploymentDevicesRequest {
                fuota_deployment_id: create_resp.id.clone(),
                limit: 10,
                offset: 0,
            },
        );
        let list_devs_resp = service.list_devices(list_devs_req).await.unwrap();
        let list_devs_resp = list_devs_resp.get_ref();
        assert_eq!(1, list_devs_resp.total_count);
        assert_eq!(1, list_devs_resp.result.len());
        assert_eq!(dev.dev_eui.to_string(), list_devs_resp.result[0].dev_eui);

        // remove devices
        let remove_devs_req = get_request(
            &u.id,
            api::RemoveDevicesFromFuotaDeploymentRequest {
                fuota_deployment_id: create_resp.id.clone(),
                dev_euis: vec![dev.dev_eui.to_string()],
            },
        );
        service.remove_devices(remove_devs_req).await.unwrap();
        let list_devs_req = get_request(
            &u.id,
            api::ListFuotaDeploymentDevicesRequest {
                fuota_deployment_id: create_resp.id.clone(),
                limit: 10,
                offset: 0,
            },
        );
        let list_devs_resp = service.list_devices(list_devs_req).await.unwrap();
        let list_devs_resp = list_devs_resp.get_ref();
        assert_eq!(0, list_devs_resp.total_count);
        assert_eq!(0, list_devs_resp.result.len());

        // add gateway
        let add_gws_req = get_request(
            &u.id,
            api::AddGatewaysToFuotaDeploymentRequest {
                fuota_deployment_id: create_resp.id.clone(),
                gateway_ids: vec![gw.gateway_id.to_string()],
            },
        );
        service.add_gateways(add_gws_req).await.unwrap();

        // list gateways
        let list_gws_req = get_request(
            &u.id,
            api::ListFuotaDeploymentGatewaysRequest {
                fuota_deployment_id: create_resp.id.clone(),
                limit: 10,
                offset: 0,
            },
        );
        let list_gws_resp = service.list_gateways(list_gws_req).await.unwrap();
        let list_gws_resp = list_gws_resp.get_ref();
        assert_eq!(1, list_gws_resp.total_count);
        assert_eq!(1, list_gws_resp.result.len());
        assert_eq!(
            gw.gateway_id.to_string(),
            list_gws_resp.result[0].gateway_id
        );

        // remove gateways
        let remove_gws_req = get_request(
            &u.id,
            api::RemoveGatewaysFromFuotaDeploymentRequest {
                fuota_deployment_id: create_resp.id.clone(),
                gateway_ids: vec![gw.gateway_id.to_string()],
            },
        );
        service.remove_gateways(remove_gws_req).await.unwrap();
        let list_gws_req = get_request(
            &u.id,
            api::ListFuotaDeploymentGatewaysRequest {
                fuota_deployment_id: create_resp.id.clone(),
                limit: 10,
                offset: 0,
            },
        );
        let list_gws_resp = service.list_gateways(list_gws_req).await.unwrap();
        let list_gws_resp = list_gws_resp.get_ref();
        assert_eq!(0, list_gws_resp.total_count);
        assert_eq!(0, list_gws_resp.result.len());

        // start deployment
        let start_req = get_request(
            &u.id,
            api::StartFuotaDeploymentRequest {
                id: create_resp.id.clone(),
            },
        );
        service.start_deployment(start_req).await.unwrap();
        let jobs = fuota::list_jobs(Uuid::from_str(&create_resp.id).unwrap())
            .await
            .unwrap();
        assert_eq!(1, jobs.len());
        assert_eq!(create_resp.id, jobs[0].fuota_deployment_id.to_string());
        assert_eq!(fields::FuotaJob::CreateMcGroup, jobs[0].job);

        // delete deployment
        let delete_req = get_request(
            &u.id,
            api::DeleteFuotaDeploymentRequest {
                id: create_resp.id.clone(),
            },
        );
        service.delete_deployment(delete_req).await.unwrap();
        let delete_req = get_request(
            &u.id,
            api::DeleteFuotaDeploymentRequest {
                id: create_resp.id.clone(),
            },
        );
        assert!(service.delete_deployment(delete_req).await.is_err());
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(*user_id));
        req
    }
}
