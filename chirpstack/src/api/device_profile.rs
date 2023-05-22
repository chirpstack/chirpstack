use std::str::FromStr;

use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::device_profile_service_server::DeviceProfileService;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers;
use super::helpers::{FromProto, ToProto};
use crate::adr;
use crate::storage::{device_profile, fields};

pub struct DeviceProfile {
    validator: validator::RequestValidator,
}

impl DeviceProfile {
    pub fn new(validator: validator::RequestValidator) -> Self {
        DeviceProfile { validator }
    }
}

#[tonic::async_trait]
impl DeviceProfileService for DeviceProfile {
    async fn create(
        &self,
        request: Request<api::CreateDeviceProfileRequest>,
    ) -> Result<Response<api::CreateDeviceProfileResponse>, Status> {
        let req_dp = match &request.get_ref().device_profile {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device_profile is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_dp.tenant_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfilesAccess::new(validator::Flag::Create, tenant_id),
            )
            .await?;

        let mut dp = device_profile::DeviceProfile {
            tenant_id,
            name: req_dp.name.clone(),
            description: req_dp.description.clone(),
            region: req_dp.region().from_proto(),
            mac_version: req_dp.mac_version().from_proto(),
            reg_params_revision: req_dp.reg_params_revision().from_proto(),
            adr_algorithm_id: req_dp.adr_algorithm_id.clone(),
            payload_codec_runtime: req_dp.payload_codec_runtime().from_proto(),
            payload_codec_script: req_dp.payload_codec_script.clone(),
            flush_queue_on_activate: req_dp.flush_queue_on_activate,
            uplink_interval: req_dp.uplink_interval as i32,
            device_status_req_interval: req_dp.device_status_req_interval as i32,
            supports_otaa: req_dp.supports_otaa,
            supports_class_b: req_dp.supports_class_b,
            supports_class_c: req_dp.supports_class_c,
            class_b_timeout: req_dp.class_b_timeout as i32,
            class_b_ping_slot_nb_k: req_dp.class_b_ping_slot_nb_k as i32,
            class_b_ping_slot_dr: req_dp.class_b_ping_slot_dr as i16,
            class_b_ping_slot_freq: req_dp.class_b_ping_slot_freq as i64,
            class_c_timeout: req_dp.class_c_timeout as i32,
            abp_rx1_delay: req_dp.abp_rx1_delay as i16,
            abp_rx1_dr_offset: req_dp.abp_rx1_dr_offset as i16,
            abp_rx2_dr: req_dp.abp_rx2_dr as i16,
            abp_rx2_freq: req_dp.abp_rx2_freq as i64,
            tags: fields::KeyValue::new(req_dp.tags.clone()),
            measurements: fields::Measurements::new(
                req_dp
                    .measurements
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.to_string(),
                            fields::Measurement {
                                name: v.name.clone(),
                                kind: v.kind().from_proto(),
                            },
                        )
                    })
                    .collect(),
            ),
            auto_detect_measurements: req_dp.auto_detect_measurements,
            region_config_id: (!req_dp.region_config_id.is_empty())
                .then(|| req_dp.region_config_id.clone()),
            is_relay: req_dp.is_relay,
            is_relay_ed: req_dp.is_relay_ed,
            relay_ed_relay_only: req_dp.relay_ed_relay_only,
            relay_enabled: req_dp.relay_enabled,
            relay_cad_periodicity: req_dp.relay_cad_periodicity as i16,
            relay_default_channel_index: req_dp.relay_default_channel_index as i16,
            relay_second_channel_freq: req_dp.relay_second_channel_freq as i64,
            relay_second_channel_dr: req_dp.relay_second_channel_dr as i16,
            relay_second_channel_ack_offset: req_dp.relay_second_channel_ack_offset as i16,
            relay_ed_activation_mode: req_dp.relay_ed_activation_mode().from_proto(),
            relay_ed_smart_enable_level: req_dp.relay_ed_smart_enable_level as i16,
            relay_ed_back_off: req_dp.relay_ed_back_off as i16,
            relay_ed_uplink_limit_bucket_size: req_dp.relay_ed_uplink_limit_bucket_size as i16,
            relay_ed_uplink_limit_reload_rate: req_dp.relay_ed_uplink_limit_reload_rate as i16,
            relay_join_req_limit_reload_rate: req_dp.relay_join_req_limit_reload_rate as i16,
            relay_notify_limit_reload_rate: req_dp.relay_notify_limit_reload_rate as i16,
            relay_global_uplink_limit_reload_rate: req_dp.relay_global_uplink_limit_reload_rate
                as i16,
            relay_overall_limit_reload_rate: req_dp.relay_overall_limit_reload_rate as i16,
            relay_join_req_limit_bucket_size: req_dp.relay_join_req_limit_bucket_size as i16,
            relay_notify_limit_bucket_size: req_dp.relay_notify_limit_bucket_size as i16,
            relay_global_uplink_limit_bucket_size: req_dp.relay_global_uplink_limit_bucket_size
                as i16,
            relay_overall_limit_bucket_size: req_dp.relay_overall_limit_bucket_size as i16,
            ..Default::default()
        };

        dp = device_profile::create(dp).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::CreateDeviceProfileResponse {
            id: dp.id.to_string(),
        });
        resp.metadata_mut().insert(
            "x-log-device_profile_id",
            dp.id.to_string().parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get(
        &self,
        request: Request<api::GetDeviceProfileRequest>,
    ) -> Result<Response<api::GetDeviceProfileResponse>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileAccess::new(validator::Flag::Read, dp_id),
            )
            .await?;

        let dp = device_profile::get(&dp_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetDeviceProfileResponse {
            device_profile: Some(api::DeviceProfile {
                id: dp.id.to_string(),
                tenant_id: dp.tenant_id.to_string(),
                name: dp.name,
                description: dp.description,
                region: dp.region.to_proto().into(),
                mac_version: dp.mac_version.to_proto().into(),
                reg_params_revision: dp.reg_params_revision.to_proto().into(),
                adr_algorithm_id: dp.adr_algorithm_id,
                payload_codec_runtime: dp.payload_codec_runtime.to_proto().into(),
                payload_codec_script: dp.payload_codec_script,
                flush_queue_on_activate: dp.flush_queue_on_activate,
                uplink_interval: dp.uplink_interval as u32,
                device_status_req_interval: dp.device_status_req_interval as u32,
                supports_otaa: dp.supports_otaa,
                supports_class_b: dp.supports_class_b,
                supports_class_c: dp.supports_class_c,
                class_b_timeout: dp.class_b_timeout as u32,
                class_b_ping_slot_nb_k: dp.class_b_ping_slot_nb_k as u32,
                class_b_ping_slot_dr: dp.class_b_ping_slot_dr as u32,
                class_b_ping_slot_freq: dp.class_b_ping_slot_freq as u32,
                class_c_timeout: dp.class_c_timeout as u32,
                abp_rx1_delay: dp.abp_rx1_delay as u32,
                abp_rx1_dr_offset: dp.abp_rx1_dr_offset as u32,
                abp_rx2_dr: dp.abp_rx2_dr as u32,
                abp_rx2_freq: dp.abp_rx2_freq as u32,
                tags: dp.tags.into_hashmap(),
                measurements: dp
                    .measurements
                    .into_hashmap()
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.to_string(),
                            api::Measurement {
                                name: v.name.clone(),
                                kind: v.kind.to_proto().into(),
                            },
                        )
                    })
                    .collect(),
                auto_detect_measurements: dp.auto_detect_measurements,
                region_config_id: dp.region_config_id.clone().unwrap_or_default(),
                is_relay: dp.is_relay,
                is_relay_ed: dp.is_relay_ed,
                relay_ed_relay_only: dp.relay_ed_relay_only,
                relay_enabled: dp.relay_enabled,
                relay_cad_periodicity: dp.relay_cad_periodicity as i32,
                relay_default_channel_index: dp.relay_default_channel_index as u32,
                relay_second_channel_freq: dp.relay_second_channel_freq as u32,
                relay_second_channel_dr: dp.relay_second_channel_dr as u32,
                relay_second_channel_ack_offset: dp.relay_second_channel_ack_offset as i32,
                relay_ed_activation_mode: dp.relay_ed_activation_mode.to_proto().into(),
                relay_ed_smart_enable_level: dp.relay_ed_smart_enable_level as u32,
                relay_ed_back_off: dp.relay_ed_back_off as u32,
                relay_ed_uplink_limit_bucket_size: dp.relay_ed_uplink_limit_bucket_size as u32,
                relay_ed_uplink_limit_reload_rate: dp.relay_ed_uplink_limit_reload_rate as u32,
                relay_join_req_limit_reload_rate: dp.relay_join_req_limit_reload_rate as u32,
                relay_notify_limit_reload_rate: dp.relay_notify_limit_reload_rate as u32,
                relay_global_uplink_limit_reload_rate: dp.relay_global_uplink_limit_reload_rate
                    as u32,
                relay_overall_limit_reload_rate: dp.relay_overall_limit_reload_rate as u32,
                relay_join_req_limit_bucket_size: dp.relay_join_req_limit_bucket_size as u32,
                relay_notify_limit_bucket_size: dp.relay_notify_limit_bucket_size as u32,
                relay_global_uplink_limit_bucket_size: dp.relay_global_uplink_limit_bucket_size
                    as u32,
                relay_overall_limit_bucket_size: dp.relay_overall_limit_bucket_size as u32,
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&dp.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&dp.updated_at)),
        });
        resp.metadata_mut()
            .insert("x-log-device_profile_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn update(
        &self,
        request: Request<api::UpdateDeviceProfileRequest>,
    ) -> Result<Response<()>, Status> {
        let req_dp = match &request.get_ref().device_profile {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device_profile is missing"));
            }
        };
        let dp_id = Uuid::from_str(&req_dp.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileAccess::new(validator::Flag::Update, dp_id),
            )
            .await?;

        // update
        let _ = device_profile::update(device_profile::DeviceProfile {
            id: dp_id,
            name: req_dp.name.clone(),
            description: req_dp.description.clone(),
            region: req_dp.region().from_proto(),
            mac_version: req_dp.mac_version().from_proto(),
            reg_params_revision: req_dp.reg_params_revision().from_proto(),
            adr_algorithm_id: req_dp.adr_algorithm_id.clone(),
            payload_codec_runtime: req_dp.payload_codec_runtime().from_proto(),
            payload_codec_script: req_dp.payload_codec_script.clone(),
            flush_queue_on_activate: req_dp.flush_queue_on_activate,
            uplink_interval: req_dp.uplink_interval as i32,
            device_status_req_interval: req_dp.device_status_req_interval as i32,
            supports_otaa: req_dp.supports_otaa,
            supports_class_b: req_dp.supports_class_b,
            supports_class_c: req_dp.supports_class_c,
            class_b_timeout: req_dp.class_b_timeout as i32,
            class_b_ping_slot_nb_k: req_dp.class_b_ping_slot_nb_k as i32,
            class_b_ping_slot_dr: req_dp.class_b_ping_slot_dr as i16,
            class_b_ping_slot_freq: req_dp.class_b_ping_slot_freq as i64,
            class_c_timeout: req_dp.class_c_timeout as i32,
            abp_rx1_delay: req_dp.abp_rx1_delay as i16,
            abp_rx1_dr_offset: req_dp.abp_rx1_dr_offset as i16,
            abp_rx2_dr: req_dp.abp_rx2_dr as i16,
            abp_rx2_freq: req_dp.abp_rx2_freq as i64,
            tags: fields::KeyValue::new(req_dp.tags.clone()),
            measurements: fields::Measurements::new(
                req_dp
                    .measurements
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.to_string(),
                            fields::Measurement {
                                name: v.name.clone(),
                                kind: v.kind().from_proto(),
                            },
                        )
                    })
                    .collect(),
            ),
            auto_detect_measurements: req_dp.auto_detect_measurements,
            region_config_id: (!req_dp.region_config_id.is_empty())
                .then(|| req_dp.region_config_id.clone()),
            is_relay: req_dp.is_relay,
            is_relay_ed: req_dp.is_relay_ed,
            relay_ed_relay_only: req_dp.relay_ed_relay_only,
            relay_enabled: req_dp.relay_enabled,
            relay_cad_periodicity: req_dp.relay_cad_periodicity as i16,
            relay_default_channel_index: req_dp.relay_default_channel_index as i16,
            relay_second_channel_freq: req_dp.relay_second_channel_freq as i64,
            relay_second_channel_dr: req_dp.relay_second_channel_dr as i16,
            relay_second_channel_ack_offset: req_dp.relay_second_channel_ack_offset as i16,
            relay_ed_activation_mode: req_dp.relay_ed_activation_mode().from_proto(),
            relay_ed_smart_enable_level: req_dp.relay_ed_smart_enable_level as i16,
            relay_ed_back_off: req_dp.relay_ed_back_off as i16,
            relay_ed_uplink_limit_bucket_size: req_dp.relay_ed_uplink_limit_bucket_size as i16,
            relay_ed_uplink_limit_reload_rate: req_dp.relay_ed_uplink_limit_reload_rate as i16,
            relay_join_req_limit_reload_rate: req_dp.relay_join_req_limit_reload_rate as i16,
            relay_notify_limit_reload_rate: req_dp.relay_notify_limit_reload_rate as i16,
            relay_global_uplink_limit_reload_rate: req_dp.relay_global_uplink_limit_reload_rate
                as i16,
            relay_overall_limit_reload_rate: req_dp.relay_overall_limit_reload_rate as i16,
            relay_join_req_limit_bucket_size: req_dp.relay_join_req_limit_bucket_size as i16,
            relay_notify_limit_bucket_size: req_dp.relay_notify_limit_bucket_size as i16,
            relay_global_uplink_limit_bucket_size: req_dp.relay_global_uplink_limit_bucket_size
                as i16,
            relay_overall_limit_bucket_size: req_dp.relay_overall_limit_bucket_size as i16,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-device_profile_id", req_dp.id.parse().unwrap());

        Ok(resp)
    }

    async fn delete(
        &self,
        request: Request<api::DeleteDeviceProfileRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dp_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileAccess::new(validator::Flag::Delete, dp_id),
            )
            .await?;

        device_profile::delete(&dp_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-device_profile_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn list(
        &self,
        request: Request<api::ListDeviceProfilesRequest>,
    ) -> Result<Response<api::ListDeviceProfilesResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfilesAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        let filters = device_profile::Filters {
            tenant_id: Some(tenant_id),
            search: if req.search.is_empty() {
                None
            } else {
                Some(req.search.to_string())
            },
        };

        let count = device_profile::get_count(&filters)
            .await
            .map_err(|e| e.status())?;
        let items = device_profile::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListDeviceProfilesResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|dp| api::DeviceProfileListItem {
                    id: dp.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&dp.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&dp.updated_at)),
                    name: dp.name.clone(),
                    region: dp.region.to_proto().into(),
                    mac_version: dp.mac_version.to_proto().into(),
                    reg_params_revision: dp.reg_params_revision.to_proto().into(),
                    supports_otaa: dp.supports_otaa,
                    supports_class_b: dp.supports_class_b,
                    supports_class_c: dp.supports_class_c,
                })
                .collect(),
        });
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());

        Ok(resp)
    }

    async fn list_adr_algorithms(
        &self,
        request: Request<()>,
    ) -> Result<Response<api::ListDeviceProfileAdrAlgorithmsResponse>, Status> {
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateActiveUserOrKey::new(),
            )
            .await?;

        let items = adr::get_algorithms().await;
        let mut result: Vec<api::AdrAlgorithmListItem> = items
            .iter()
            .map(|(k, v)| api::AdrAlgorithmListItem {
                id: k.clone(),
                name: v.clone(),
            })
            .collect();
        result.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Response::new(api::ListDeviceProfileAdrAlgorithmsResponse {
            total_count: items.len() as u32,
            result,
        }))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::{tenant, user};
    use crate::test;
    use chirpstack_api::common;

    #[tokio::test]
    async fn test_device_profile() {
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

        // setup the api
        let service = DeviceProfile::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateDeviceProfileRequest {
                device_profile: Some(api::DeviceProfile {
                    tenant_id: t.id.to_string(),
                    name: "test-dp".into(),
                    region: common::Region::Eu868.into(),
                    mac_version: common::MacVersion::Lorawan103.into(),
                    reg_params_revision: common::RegParamsRevision::A.into(),
                    adr_algorithm_id: "default".into(),
                    ..Default::default()
                }),
            },
        );
        let create_resp = service.create(create_req).await.unwrap();
        let dp_id = Uuid::from_str(&create_resp.get_ref().id).unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetDeviceProfileRequest {
                id: dp_id.to_string(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceProfile {
                id: dp_id.to_string(),
                tenant_id: t.id.to_string(),
                name: "test-dp".into(),
                region: common::Region::Eu868.into(),
                mac_version: common::MacVersion::Lorawan103.into(),
                reg_params_revision: common::RegParamsRevision::A.into(),
                adr_algorithm_id: "default".into(),
                ..Default::default()
            }),
            get_resp.get_ref().device_profile
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateDeviceProfileRequest {
                device_profile: Some(api::DeviceProfile {
                    id: dp_id.to_string(),
                    tenant_id: t.id.to_string(),
                    name: "test-dp-updated".into(),
                    region: common::Region::Us915.into(),
                    mac_version: common::MacVersion::Lorawan103.into(),
                    reg_params_revision: common::RegParamsRevision::A.into(),
                    adr_algorithm_id: "default".into(),
                    ..Default::default()
                }),
            },
        );
        let _ = service.update(update_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetDeviceProfileRequest {
                id: dp_id.to_string(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceProfile {
                id: dp_id.to_string(),
                tenant_id: t.id.to_string(),
                name: "test-dp-updated".into(),
                region: common::Region::Us915.into(),
                mac_version: common::MacVersion::Lorawan103.into(),
                reg_params_revision: common::RegParamsRevision::A.into(),
                adr_algorithm_id: "default".into(),
                ..Default::default()
            }),
            get_resp.get_ref().device_profile
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListDeviceProfilesRequest {
                tenant_id: t.id.to_string(),
                limit: 10,
                search: "update".into(),
                ..Default::default()
            },
        );
        let list_resp = service.list(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(1, list_resp.total_count);
        assert_eq!(1, list_resp.result.len());
        assert_eq!(dp_id.to_string(), list_resp.result[0].id);

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteDeviceProfileRequest {
                id: dp_id.to_string(),
            },
        );
        let _ = service.delete(del_req).await.unwrap();
        let del_req = get_request(
            &u.id,
            api::DeleteDeviceProfileRequest {
                id: dp_id.to_string(),
            },
        );
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());

        // list adr algorithms
        let list_adr_algs_req = get_request(&u.id, ());
        let list_adr_algs_resp = service
            .list_adr_algorithms(list_adr_algs_req)
            .await
            .unwrap();
        let list_adr_algs_resp = list_adr_algs_resp.get_ref();
        assert_eq!(3, list_adr_algs_resp.total_count);
        assert_eq!(3, list_adr_algs_resp.result.len());
        assert_eq!("default", list_adr_algs_resp.result[0].id);
        assert_eq!("lr_fhss", list_adr_algs_resp.result[1].id);
        assert_eq!("lora_lr_fhss", list_adr_algs_resp.result[2].id);
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(user_id.clone()));
        req
    }
}
