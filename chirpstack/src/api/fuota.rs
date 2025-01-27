use std::str::FromStr;

use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::fuota_service_server::FuotaService;
use lrwn::EUI64;

use crate::api::auth::validator;
use crate::api::error::ToStatus;
use crate::api::helpers::{self, FromProto, ToProto};
use crate::storage::fuota;

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

        let dp = fuota::FuotaDeployment {
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
            multicast_class_b_ping_slot_nb_k: req_dp.multicast_class_b_ping_slot_nb_k as i16,
            multicast_frequency: req_dp.multicast_frequency as i64,
            multicast_timeout: req_dp.multicast_timeout as i16,
            unicast_attempt_count: req_dp.unicast_attempt_count as i16,
            fragmentation_fragment_size: req_dp.fragmentation_fragment_size as i16,
            fragmentation_redundancy: req_dp.fragmentation_redundancy as i16,
            fragmentation_session_index: req_dp.fragmentation_session_index as i16,
            fragmentation_matrix: req_dp.fragmentation_matrix as i16,
            fragmentation_block_ack_delay: req_dp.fragmentation_block_ack_delay as i16,
            fragmentation_descriptor: req_dp.fragmentation_descriptor.clone(),
            request_fragmentation_session_status: req_dp
                .request_fragmentation_session_status()
                .from_proto(),
            payload: req_dp.payload.clone(),
            ..Default::default()
        };
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
                multicast_class_b_ping_slot_nb_k: dp.multicast_class_b_ping_slot_nb_k as u32,
                multicast_frequency: dp.multicast_frequency as u32,
                multicast_timeout: dp.multicast_timeout as u32,
                unicast_attempt_count: dp.unicast_attempt_count as u32,
                fragmentation_fragment_size: dp.fragmentation_fragment_size as u32,
                fragmentation_redundancy: dp.fragmentation_redundancy as u32,
                fragmentation_session_index: dp.fragmentation_session_index as u32,
                fragmentation_matrix: dp.fragmentation_matrix as u32,
                fragmentation_block_ack_delay: dp.fragmentation_block_ack_delay as u32,
                fragmentation_descriptor: dp.fragmentation_descriptor.clone(),
                request_fragmentation_session_status: dp
                    .request_fragmentation_session_status
                    .to_proto()
                    .into(),
                payload: dp.payload.clone(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&dp.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&dp.updated_at)),
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

        let _ = fuota::update_deployment(fuota::FuotaDeployment {
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
            multicast_class_b_ping_slot_nb_k: req_dp.multicast_class_b_ping_slot_nb_k as i16,
            multicast_frequency: req_dp.multicast_frequency as i64,
            multicast_timeout: req_dp.multicast_timeout as i16,
            unicast_attempt_count: req_dp.unicast_attempt_count as i16,
            fragmentation_fragment_size: req_dp.fragmentation_fragment_size as i16,
            fragmentation_redundancy: req_dp.fragmentation_redundancy as i16,
            fragmentation_session_index: req_dp.fragmentation_session_index as i16,
            fragmentation_matrix: req_dp.fragmentation_matrix as i16,
            fragmentation_block_ack_delay: req_dp.fragmentation_block_ack_delay as i16,
            fragmentation_descriptor: req_dp.fragmentation_descriptor.clone(),
            request_fragmentation_session_status: req_dp
                .request_fragmentation_session_status()
                .from_proto(),
            payload: req_dp.payload.clone(),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

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
                        .map(|ts| helpers::datetime_to_prost_timestamp(ts)),
                    completed_at: d
                        .completed_at
                        .as_ref()
                        .map(|ts| helpers::datetime_to_prost_timestamp(ts)),
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
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&d.updated_at)),
                    mc_group_setup_completed_at: d
                        .mc_group_setup_completed_at
                        .as_ref()
                        .map(|ts| helpers::datetime_to_prost_timestamp(ts)),
                    mc_session_completed_at: d
                        .mc_session_completed_at
                        .as_ref()
                        .map(|ts| helpers::datetime_to_prost_timestamp(ts)),
                    frag_session_setup_completed_at: d
                        .frag_session_setup_completed_at
                        .as_ref()
                        .map(|ts| helpers::datetime_to_prost_timestamp(ts)),
                    frag_status_completed_at: d
                        .frag_status_completed_at
                        .as_ref()
                        .map(|ts| helpers::datetime_to_prost_timestamp(ts)),
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
        let items = fuota::get_gateway(dp_id, req.limit as i64, req.offset as i64)
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
}
