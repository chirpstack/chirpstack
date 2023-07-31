use tonic::{Request, Response, Status};

use chirpstack_api::api;
use chirpstack_api::api::device_profile_template_service_server::DeviceProfileTemplateService;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers;
use super::helpers::{FromProto, ToProto};
use crate::storage::{device_profile_template, fields};

pub struct DeviceProfileTemplate {
    validator: validator::RequestValidator,
}

impl DeviceProfileTemplate {
    pub fn new(validator: validator::RequestValidator) -> Self {
        DeviceProfileTemplate { validator }
    }
}

#[tonic::async_trait]
impl DeviceProfileTemplateService for DeviceProfileTemplate {
    async fn create(
        &self,
        request: Request<api::CreateDeviceProfileTemplateRequest>,
    ) -> Result<Response<()>, Status> {
        let req_dp = match &request.get_ref().device_profile_template {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument(
                    "device_profile_template is missing",
                ));
            }
        };

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileTemplatesAccess::new(validator::Flag::Create),
            )
            .await?;

        let dp = device_profile_template::DeviceProfileTemplate {
            id: req_dp.id.clone(),
            name: req_dp.name.clone(),
            description: req_dp.description.clone(),
            vendor: req_dp.vendor.clone(),
            firmware: req_dp.firmware.clone(),
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
            ..Default::default()
        };

        device_profile_template::create(dp)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn get(
        &self,
        request: Request<api::GetDeviceProfileTemplateRequest>,
    ) -> Result<Response<api::GetDeviceProfileTemplateResponse>, Status> {
        let req = request.get_ref();

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileTemplateAccess::new(validator::Flag::Read),
            )
            .await?;

        let dp = device_profile_template::get(&req.id)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::GetDeviceProfileTemplateResponse {
            device_profile_template: Some(api::DeviceProfileTemplate {
                id: dp.id.to_string(),
                name: dp.name,
                description: dp.description,
                vendor: dp.vendor,
                firmware: dp.firmware,
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
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&dp.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&dp.updated_at)),
        }))
    }

    async fn update(
        &self,
        request: Request<api::UpdateDeviceProfileTemplateRequest>,
    ) -> Result<Response<()>, Status> {
        let req_dp = match &request.get_ref().device_profile_template {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument(
                    "device_profile_template is missing",
                ));
            }
        };

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileTemplateAccess::new(validator::Flag::Update),
            )
            .await?;

        device_profile_template::update(device_profile_template::DeviceProfileTemplate {
            id: req_dp.id.clone(),
            name: req_dp.name.clone(),
            description: req_dp.description.clone(),
            vendor: req_dp.vendor.clone(),
            firmware: req_dp.firmware.clone(),
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
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn delete(
        &self,
        request: Request<api::DeleteDeviceProfileTemplateRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileTemplateAccess::new(validator::Flag::Delete),
            )
            .await?;

        device_profile_template::delete(&req.id)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn list(
        &self,
        request: Request<api::ListDeviceProfileTemplatesRequest>,
    ) -> Result<Response<api::ListDeviceProfileTemplatesResponse>, Status> {
        let req = request.get_ref();

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileTemplatesAccess::new(validator::Flag::List),
            )
            .await?;

        let count = device_profile_template::get_count()
            .await
            .map_err(|e| e.status())?;

        let items = device_profile_template::list(req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::ListDeviceProfileTemplatesResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|dp| api::DeviceProfileTemplateListItem {
                    id: dp.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&dp.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&dp.updated_at)),
                    name: dp.name.clone(),
                    vendor: dp.vendor.clone(),
                    firmware: dp.firmware.clone(),
                    region: dp.region.to_proto().into(),
                    mac_version: dp.mac_version.to_proto().into(),
                    reg_params_revision: dp.reg_params_revision.to_proto().into(),
                    supports_otaa: dp.supports_otaa,
                    supports_class_b: dp.supports_class_b,
                    supports_class_c: dp.supports_class_c,
                })
                .collect(),
        }))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::user;
    use crate::test;
    use chirpstack_api::common;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_device_profile_template() {
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

        // setup the api
        let service = DeviceProfileTemplate::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateDeviceProfileTemplateRequest {
                device_profile_template: Some(api::DeviceProfileTemplate {
                    id: "test-id".into(),
                    name: "test-template".into(),
                    vendor: "Test Vendor".into(),
                    firmware: "1.2.3".into(),
                    region: common::Region::Eu868.into(),
                    mac_version: common::MacVersion::Lorawan103.into(),
                    reg_params_revision: common::RegParamsRevision::A.into(),
                    adr_algorithm_id: "default".into(),
                    ..Default::default()
                }),
            },
        );
        let _ = service.create(create_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetDeviceProfileTemplateRequest {
                id: "test-id".into(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceProfileTemplate {
                id: "test-id".into(),
                name: "test-template".into(),
                vendor: "Test Vendor".into(),
                firmware: "1.2.3".into(),
                region: common::Region::Eu868.into(),
                mac_version: common::MacVersion::Lorawan103.into(),
                reg_params_revision: common::RegParamsRevision::A.into(),
                adr_algorithm_id: "default".into(),
                ..Default::default()
            }),
            get_resp.get_ref().device_profile_template
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateDeviceProfileTemplateRequest {
                device_profile_template: Some(api::DeviceProfileTemplate {
                    id: "test-id".into(),
                    name: "test-template-updated".into(),
                    vendor: "Test Vendor".into(),
                    firmware: "1.2.3".into(),
                    region: common::Region::Eu868.into(),
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
            api::GetDeviceProfileTemplateRequest {
                id: "test-id".into(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceProfileTemplate {
                id: "test-id".into(),
                name: "test-template-updated".into(),
                vendor: "Test Vendor".into(),
                firmware: "1.2.3".into(),
                region: common::Region::Eu868.into(),
                mac_version: common::MacVersion::Lorawan103.into(),
                reg_params_revision: common::RegParamsRevision::A.into(),
                adr_algorithm_id: "default".into(),
                ..Default::default()
            }),
            get_resp.get_ref().device_profile_template
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListDeviceProfileTemplatesRequest {
                limit: 10,
                offset: 0,
                ..Default::default()
            },
        );
        let list_resp = service.list(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(1, list_resp.total_count);
        assert_eq!(1, list_resp.result.len());
        assert_eq!("test-id".to_string(), list_resp.result[0].id);

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteDeviceProfileTemplateRequest {
                id: "test-id".into(),
            },
        );
        let _ = service.delete(del_req).await.unwrap();
        let del_req = get_request(
            &u.id,
            api::DeleteDeviceProfileTemplateRequest {
                id: "test-id".into(),
            },
        );
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(user_id.clone()));
        req
    }
}
