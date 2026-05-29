use std::collections::HashSet;
use std::str::FromStr;

use chirpstack_api::api;
use chirpstack_api::api::multicast_group_service_server::MulticastGroupService;
use chirpstack_api::tonic::{self, Request, Response, Status};
use chrono::{DateTime, Utc};
use lrwn::{AES128Key, DevAddr, EUI64};
use uuid::Uuid;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers::{self, FromProto, ToProto};
use crate::aeskey::get_random_aes_key;
use crate::applayer::multicastsetup as app_multicastsetup;
use crate::devaddr::get_random_dev_addr;
use crate::downlink;
use crate::storage::{
    device, device_keys, device_profile, device_queue, error::Error as StorageError, multicast,
    tenant,
};

pub struct MulticastGroup {
    validator: validator::RequestValidator,
}

impl MulticastGroup {
    pub fn new(validator: validator::RequestValidator) -> Self {
        MulticastGroup { validator }
    }
}

fn setup_to_proto(setup: &str) -> api::MulticastGroupSetup {
    match setup {
        "TS005" => api::MulticastGroupSetup::Ts005,
        _ => api::MulticastGroupSetup::OutOfBand,
    }
}

fn setup_from_proto(setup: api::MulticastGroupSetup) -> String {
    match setup {
        api::MulticastGroupSetup::OutOfBand => "OUT_OF_BAND",
        api::MulticastGroupSetup::Ts005 => "TS005",
    }
    .into()
}

fn timestamp_from_proto(
    ts: Option<prost_types::Timestamp>,
) -> Result<Option<DateTime<Utc>>, Status> {
    match ts {
        Some(v) => {
            let v: std::time::SystemTime = v
                .try_into()
                .map_err(|e: prost_types::TimestampError| e.status())?;
            Ok(Some(v.into()))
        }
        None => Ok(None),
    }
}

fn timestamp_seconds_changed(
    current: &Option<DateTime<Utc>>,
    next: &Option<DateTime<Utc>>,
) -> bool {
    current.as_ref().map(|v| v.timestamp()) != next.as_ref().map(|v| v.timestamp())
}

fn validate_ts005_class_support(
    mg: &multicast::MulticastGroup,
    dp: &device_profile::DeviceProfile,
) -> Result<(), Status> {
    match mg.group_type.as_ref() {
        "B" if !dp.supports_class_b => Err(Status::failed_precondition(
            "Device-profile does not support Class-B",
        )),
        "C" if !dp.supports_class_c => Err(Status::failed_precondition(
            "Device-profile does not support Class-C",
        )),
        _ => Ok(()),
    }
}

fn multicast_group_to_proto(mg: &multicast::MulticastGroup) -> Result<api::MulticastGroup, Status> {
    Ok(api::MulticastGroup {
        id: mg.id.to_string(),
        name: mg.name.clone(),
        application_id: mg.application_id.to_string(),
        region: mg.region.to_proto().into(),
        mc_addr: mg.mc_addr.to_string(),
        mc_nwk_s_key: mg.mc_nwk_s_key.to_string(),
        mc_app_s_key: mg.mc_app_s_key.to_string(),
        f_cnt: mg.f_cnt as u32,
        group_type: match mg.group_type.as_ref() {
            "B" => api::MulticastGroupType::ClassB,
            "C" => api::MulticastGroupType::ClassC,
            _ => {
                return Err(Status::invalid_argument("Invalid group_type"));
            }
        }
        .into(),
        dr: mg.dr as u32,
        frequency: mg.frequency as u32,
        class_b_ping_slot_periodicity: mg.class_b_ping_slot_periodicity as u32,
        class_c_scheduling_type: mg.class_c_scheduling_type.to_proto().into(),
        setup: setup_to_proto(&mg.setup).into(),
        mc_session_start: mg
            .mc_session_start
            .as_ref()
            .map(helpers::datetime_to_prost_timestamp),
        mc_session_timeout: mg.mc_session_timeout as u32,
    })
}

#[tonic::async_trait]
impl MulticastGroupService for MulticastGroup {
    async fn create(
        &self,
        request: Request<api::CreateMulticastGroupRequest>,
    ) -> Result<Response<api::CreateMulticastGroupResponse>, Status> {
        let req_mg = match &request.get_ref().multicast_group {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("multicast_group is missing"));
            }
        };

        let app_id = Uuid::from_str(&req_mg.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupsAccess::new(
                    validator::Flag::Create,
                    None,
                    Some(app_id),
                ),
            )
            .await?;

        let setup = setup_from_proto(req_mg.setup());
        let mc_addr = DevAddr::from_str(&req_mg.mc_addr).map_err(|e| e.status())?;
        let (mc_nwk_s_key, mc_app_s_key, mc_key, mc_session_start, mc_session_timeout) =
            if setup == "TS005" {
                if req_mg.mc_session_timeout > 15 {
                    return Err(Status::invalid_argument(
                        "mc_session_timeout must be between 0 - 15",
                    ));
                }

                let mc_session_start = timestamp_from_proto(req_mg.mc_session_start)?;
                if mc_session_start.is_none() {
                    return Err(Status::invalid_argument(
                        "mc_session_start is required for TS005 multicast setup",
                    ));
                }

                let mc_key = get_random_aes_key();
                let (mc_app_s_key, mc_nwk_s_key) =
                    app_multicastsetup::derive_mc_keys(mc_key, mc_addr).map_err(|e| e.status())?;

                (
                    mc_nwk_s_key,
                    mc_app_s_key,
                    Some(mc_key),
                    mc_session_start,
                    req_mg.mc_session_timeout as i16,
                )
            } else {
                (
                    AES128Key::from_str(&req_mg.mc_nwk_s_key).map_err(|e| e.status())?,
                    AES128Key::from_str(&req_mg.mc_app_s_key).map_err(|e| e.status())?,
                    None,
                    None,
                    0,
                )
            };

        let mg = multicast::MulticastGroup {
            application_id: app_id.into(),
            name: req_mg.name.clone(),
            region: req_mg.region().from_proto(),
            mc_addr,
            mc_nwk_s_key,
            mc_app_s_key,
            setup,
            mc_key,
            mc_session_start,
            mc_session_timeout,
            f_cnt: req_mg.f_cnt as i64,
            group_type: match req_mg.group_type() {
                api::MulticastGroupType::ClassB => "B",
                api::MulticastGroupType::ClassC => "C",
            }
            .to_string(),
            dr: req_mg.dr as i16,
            frequency: req_mg.frequency as i64,
            class_b_ping_slot_periodicity: req_mg.class_b_ping_slot_periodicity as i16,
            class_c_scheduling_type: req_mg.class_c_scheduling_type().from_proto(),
            ..Default::default()
        };
        let mg = multicast::create(mg).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::CreateMulticastGroupResponse {
            id: mg.id.to_string(),
        });
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            mg.id.to_string().parse().unwrap(),
        );

        Ok(resp)
    }

    async fn get(
        &self,
        request: Request<api::GetMulticastGroupRequest>,
    ) -> Result<Response<api::GetMulticastGroupResponse>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Read, mg_id),
            )
            .await?;

        let mg = multicast::get(&mg_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetMulticastGroupResponse {
            multicast_group: Some(multicast_group_to_proto(&mg)?),
            created_at: Some(helpers::datetime_to_prost_timestamp(&mg.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&mg.updated_at)),
        });
        resp.metadata_mut()
            .insert("x-log-multicast_group_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn update(
        &self,
        request: Request<api::UpdateMulticastGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let req_mg = match &request.get_ref().multicast_group {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("multicast_group is missing"));
            }
        };
        let mg_id = Uuid::from_str(&req_mg.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Update, mg_id),
            )
            .await?;

        let current_mg = multicast::get(&mg_id).await.map_err(|e| e.status())?;
        let setup = setup_from_proto(req_mg.setup());
        let region = req_mg.region().from_proto();
        let mc_addr = DevAddr::from_str(&req_mg.mc_addr).map_err(|e| e.status())?;
        let group_type = match req_mg.group_type() {
            api::MulticastGroupType::ClassB => "B",
            api::MulticastGroupType::ClassC => "C",
        }
        .to_string();
        let class_c_scheduling_type = req_mg.class_c_scheduling_type().from_proto();
        let device_count = multicast::get_device_count(&mg_id)
            .await
            .map_err(|e| e.status())?;

        if device_count > 0 && current_mg.setup != setup {
            return Err(Status::failed_precondition(
                "Remove devices from the multicast-group before changing multicast setup",
            ));
        }

        let (mc_nwk_s_key, mc_app_s_key, mc_key, mc_session_start, mc_session_timeout) =
            if setup == "TS005" {
                if req_mg.mc_session_timeout > 15 {
                    return Err(Status::invalid_argument(
                        "mc_session_timeout must be between 0 - 15",
                    ));
                }

                let mc_session_start = timestamp_from_proto(req_mg.mc_session_start)?;
                if mc_session_start.is_none() {
                    return Err(Status::invalid_argument(
                        "mc_session_start is required for TS005 multicast setup",
                    ));
                }

                let mc_key = current_mg.mc_key.unwrap_or_else(get_random_aes_key);
                let (mc_app_s_key, mc_nwk_s_key) =
                    app_multicastsetup::derive_mc_keys(mc_key, mc_addr).map_err(|e| e.status())?;

                (
                    mc_nwk_s_key,
                    mc_app_s_key,
                    Some(mc_key),
                    mc_session_start,
                    req_mg.mc_session_timeout as i16,
                )
            } else {
                (
                    AES128Key::from_str(&req_mg.mc_nwk_s_key).map_err(|e| e.status())?,
                    AES128Key::from_str(&req_mg.mc_app_s_key).map_err(|e| e.status())?,
                    None,
                    None,
                    0,
                )
            };

        if device_count > 0
            && current_mg.setup == "TS005"
            && (current_mg.region != region
                || current_mg.mc_addr != mc_addr
                || current_mg.f_cnt != req_mg.f_cnt as i64
                || current_mg.group_type != group_type
                || current_mg.dr != req_mg.dr as i16
                || current_mg.frequency != req_mg.frequency as i64
                || current_mg.class_b_ping_slot_periodicity
                    != req_mg.class_b_ping_slot_periodicity as i16
                || current_mg.class_c_scheduling_type != class_c_scheduling_type
                || timestamp_seconds_changed(&current_mg.mc_session_start, &mc_session_start)
                || current_mg.mc_session_timeout != mc_session_timeout)
        {
            return Err(Status::failed_precondition(
                "Remove devices from the multicast-group before changing TS005 multicast setup",
            ));
        }

        let _ = multicast::update(multicast::MulticastGroup {
            id: mg_id.into(),
            application_id: current_mg.application_id,
            name: req_mg.name.clone(),
            region,
            mc_addr,
            mc_nwk_s_key,
            mc_app_s_key,
            setup,
            mc_key,
            mc_session_start,
            mc_session_timeout,
            f_cnt: req_mg.f_cnt as i64,
            group_type,
            dr: req_mg.dr as i16,
            frequency: req_mg.frequency as i64,
            class_b_ping_slot_periodicity: req_mg.class_b_ping_slot_periodicity as i16,
            class_c_scheduling_type,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-multicast_group_id", req_mg.id.parse().unwrap());

        Ok(resp)
    }

    async fn delete(
        &self,
        request: Request<api::DeleteMulticastGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Delete, mg_id),
            )
            .await?;

        let mg = multicast::get(&mg_id).await.map_err(|e| e.status())?;
        if mg.setup == "TS005" {
            let device_count = multicast::get_device_count(&mg_id)
                .await
                .map_err(|e| e.status())?;
            if device_count > 0 {
                return Err(Status::failed_precondition(
                    "Remove TS005 devices from the multicast-group before deleting it",
                ));
            }
        }

        multicast::delete(&mg_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-multicast_group_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn list(
        &self,
        request: Request<api::ListMulticastGroupsRequest>,
    ) -> Result<Response<api::ListMulticastGroupsResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = if req.tenant_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?)
        };
        let app_id = if req.application_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.application_id).map_err(|e| e.status())?)
        };
        let dev_eui = if req.dev_eui.is_empty() {
            None
        } else {
            Some(EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?)
        };

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupsAccess::new(
                    validator::Flag::List,
                    tenant_id,
                    app_id,
                ),
            )
            .await?;

        let filters = multicast::Filters {
            tenant_id: None,
            application_id: app_id,
            dev_eui,
            search: if req.search.is_empty() {
                None
            } else {
                Some(req.search.to_string())
            },
        };

        let count = multicast::get_count(&filters)
            .await
            .map_err(|e| e.status())?;
        let items = multicast::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListMulticastGroupsResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|mg| api::MulticastGroupListItem {
                    id: mg.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&mg.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&mg.updated_at)),
                    name: mg.name.clone(),
                    region: mg.region.to_proto().into(),
                    group_type: match mg.group_type.as_ref() {
                        "B" => api::MulticastGroupType::ClassB,
                        "C" => api::MulticastGroupType::ClassC,
                        _ => api::MulticastGroupType::ClassC,
                    }
                    .into(),
                    application_id: mg.application_id.to_string(),
                    application_name: mg.application_name.clone(),
                })
                .collect(),
        });
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn get_random_dev_addr(
        &self,
        request: Request<api::GetRandomMulticastGroupDevAddrRequest>,
    ) -> Result<Response<api::GetRandomMulticastGroupDevAddrResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupsAccess::new(
                    validator::Flag::Create,
                    None,
                    Some(app_id),
                ),
            )
            .await?;

        let t = tenant::get_for_application_id(app_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetRandomMulticastGroupDevAddrResponse {
            dev_addr: get_random_dev_addr(&t.get_dev_addr_prefixes()).to_string(),
        });
        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());

        Ok(resp)
    }

    async fn add_device(
        &self,
        request: Request<api::AddDeviceToMulticastGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Update, mg_id),
            )
            .await?;

        let mg = multicast::get(&mg_id).await.map_err(|e| e.status())?;

        if mg.setup == "TS005" {
            let dev = device::get(&dev_eui).await.map_err(|e| e.status())?;
            if dev.application_id != mg.application_id {
                return Err(StorageError::NotFound(dev_eui.to_string()).status());
            }

            let dp = device_profile::get(&dev.device_profile_id)
                .await
                .map_err(|e| e.status())?;
            let ts005_version = dp.app_layer_params.ts005_version.ok_or_else(|| {
                Status::failed_precondition("Device-profile does not support TS005")
            })?;
            let mc_key = mg
                .mc_key
                .ok_or_else(|| Status::internal("TS005 multicast-group is missing mc_key"))?;

            if mg.mc_session_start.is_none() {
                return Err(Status::failed_precondition(
                    "TS005 multicast-group is missing mc_session_start",
                ));
            }

            validate_ts005_class_support(&mg, &dp)?;

            let mgd = multicast::add_ts005_device(&mg_id, &dev_eui)
                .await
                .map_err(|e| e.status())?;
            let mc_group_id = mgd
                .mc_group_id
                .ok_or_else(|| Status::internal("TS005 mc_group_id was not allocated"))?;
            let dev_keys = match device_keys::get(&dev_eui).await {
                Ok(v) => v,
                Err(e) => {
                    let _ = multicast::remove_device(&mg_id, &dev_eui).await;
                    return Err(e.status());
                }
            };

            let pl = match app_multicastsetup::build_mc_group_setup_req(
                ts005_version,
                dp.mac_version,
                mc_group_id as u8,
                mg.mc_addr,
                mc_key,
                &dev_keys,
            ) {
                Ok(v) => v,
                Err(e) => {
                    let _ = multicast::remove_device(&mg_id, &dev_eui).await;
                    return Err(e.status());
                }
            };

            if let Err(e) = device_queue::enqueue_item(device_queue::DeviceQueueItem {
                dev_eui,
                f_port: dp.app_layer_params.ts005_f_port.into(),
                data: pl,
                ..Default::default()
            })
            .await
            {
                let _ = multicast::remove_device(&mg_id, &dev_eui).await;
                return Err(e.status());
            }
        } else {
            multicast::add_device(&mg_id, &dev_eui)
                .await
                .map_err(|e| e.status())?;
        }

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );
        resp.metadata_mut()
            .insert("x-log-dev_eui", req.dev_eui.parse().unwrap());

        Ok(resp)
    }

    async fn list_devices(
        &self,
        request: Request<api::ListMulticastGroupDevicesRequest>,
    ) -> Result<Response<api::ListMulticastGroupDevicesResponse>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Read, mg_id),
            )
            .await?;

        let count = multicast::get_device_count(&mg_id)
            .await
            .map_err(|e| e.status())?;
        let devices = multicast::list_devices(&mg_id, req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListMulticastGroupDevicesResponse {
            total_count: count as u32,
            result: devices
                .iter()
                .map(|d| api::MulticastGroupDeviceListItem {
                    dev_eui: d.dev_eui.to_string(),
                    device_name: d.device_name.clone(),
                    mc_group_id: d.mc_group_id.unwrap_or_default() as u32,
                    mc_group_setup_completed_at: d
                        .mc_group_setup_completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    mc_session_completed_at: d
                        .mc_session_completed_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    error_msg: d.error_msg.clone(),
                    pending_delete: d.pending_delete,
                })
                .collect(),
        });
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn remove_device(
        &self,
        request: Request<api::RemoveDeviceFromMulticastGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Update, mg_id),
            )
            .await?;

        let mg = multicast::get(&mg_id).await.map_err(|e| e.status())?;
        let mut mgd = multicast::get_device(&mg_id, &dev_eui)
            .await
            .map_err(|e| e.status())?;

        if mg.setup == "TS005" {
            let mc_group_id = mgd
                .mc_group_id
                .ok_or_else(|| Status::internal("TS005 mc_group_id is not set"))?;
            let dev = device::get(&dev_eui).await.map_err(|e| e.status())?;
            let dp = device_profile::get(&dev.device_profile_id)
                .await
                .map_err(|e| e.status())?;
            let ts005_version = dp.app_layer_params.ts005_version.ok_or_else(|| {
                Status::failed_precondition(
                    "Device-profile must keep TS005 enabled while TS005 devices are assigned to the multicast-group",
                )
            })?;

            if !mgd.pending_delete {
                mgd.pending_delete = true;
                mgd = multicast::update_device(mgd)
                    .await
                    .map_err(|e| e.status())?;
            }

            let pl = match app_multicastsetup::build_mc_group_delete_req(
                ts005_version,
                mc_group_id as u8,
            ) {
                Ok(v) => v,
                Err(e) => {
                    mgd.pending_delete = false;
                    let _ = multicast::update_device(mgd).await;
                    return Err(e.status());
                }
            };

            if let Err(e) = device_queue::enqueue_item(device_queue::DeviceQueueItem {
                dev_eui,
                f_port: dp.app_layer_params.ts005_f_port.into(),
                data: pl,
                ..Default::default()
            })
            .await
            {
                mgd.pending_delete = false;
                let _ = multicast::update_device(mgd).await;
                return Err(e.status());
            }
        } else {
            multicast::remove_device(&mg_id, &dev_eui)
                .await
                .map_err(|e| e.status())?;
        }

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );
        resp.metadata_mut()
            .insert("x-log-dev_eui", req.dev_eui.parse().unwrap());

        Ok(resp)
    }

    async fn sync_ts005_session(
        &self,
        request: Request<api::SyncMulticastGroupTs005SessionRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Update, mg_id),
            )
            .await?;

        let mut mg = multicast::get(&mg_id).await.map_err(|e| e.status())?;
        if mg.setup != "TS005" {
            return Err(Status::failed_precondition(
                "Multicast-group is not configured for TS005",
            ));
        }

        if req.mc_session_start.is_some() {
            if req.mc_session_timeout > 15 {
                return Err(Status::invalid_argument(
                    "mc_session_timeout must be between 0 - 15",
                ));
            }

            mg.mc_session_start = timestamp_from_proto(req.mc_session_start)?;
            mg.mc_session_timeout = req.mc_session_timeout as i16;
            mg = multicast::update(mg).await.map_err(|e| e.status())?;
        }

        if mg.mc_session_start.is_none() {
            return Err(Status::failed_precondition(
                "TS005 multicast-group is missing mc_session_start",
            ));
        }

        let dev_euis = multicast::get_dev_euis(&mg_id)
            .await
            .map_err(|e| e.status())?;
        let mut candidates = Vec::new();

        for dev_eui in dev_euis {
            let mgd = multicast::get_device(&mg_id, &dev_eui)
                .await
                .map_err(|e| e.status())?;
            if mgd.pending_delete || mgd.mc_group_setup_completed_at.is_none() {
                continue;
            }

            let mc_group_id = mgd
                .mc_group_id
                .ok_or_else(|| Status::internal("TS005 mc_group_id is not set"))?;
            let dev = device::get(&dev_eui).await.map_err(|e| e.status())?;
            if dev.application_id != mg.application_id {
                return Err(StorageError::NotFound(dev_eui.to_string()).status());
            }

            let dp = device_profile::get(&dev.device_profile_id)
                .await
                .map_err(|e| e.status())?;
            let ts005_version = dp.app_layer_params.ts005_version.ok_or_else(|| {
                Status::failed_precondition(
                    "Device-profile must keep TS005 enabled while TS005 devices are assigned to the multicast-group",
                )
            })?;
            validate_ts005_class_support(&mg, &dp)?;

            candidates.push((dev, dp, mgd, ts005_version, mc_group_id));
        }

        if candidates.is_empty() {
            return Err(Status::failed_precondition(
                "No devices with completed TS005 multicast setup",
            ));
        }

        for (dev, dp, mut mgd, ts005_version, _) in candidates {
            mgd.mc_session_completed_at = None;
            mgd.error_msg = "".into();
            let mgd = multicast::update_device(mgd)
                .await
                .map_err(|e| e.status())?;

            app_multicastsetup::enqueue_mc_session_req(&dev, &dp, ts005_version, &mgd, &mg)
                .await
                .map_err(|e| e.status())?;
        }

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn add_gateway(
        &self,
        request: Request<api::AddGatewayToMulticastGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;
        let gateway_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Update, mg_id),
            )
            .await?;

        multicast::add_gateway(&mg_id, &gateway_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn remove_gateway(
        &self,
        request: Request<api::RemoveGatewayFromMulticastGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;
        let gateway_id = EUI64::from_str(&req.gateway_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupAccess::new(validator::Flag::Update, mg_id),
            )
            .await?;

        multicast::remove_gateway(&mg_id, &gateway_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );
        resp.metadata_mut()
            .insert("x-log-gateway_id", req.gateway_id.parse().unwrap());

        Ok(resp)
    }

    async fn enqueue(
        &self,
        request: Request<api::EnqueueMulticastGroupQueueItemRequest>,
    ) -> Result<Response<api::EnqueueMulticastGroupQueueItemResponse>, Status> {
        let req_enq = match &request.get_ref().queue_item {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("queue_item is missing"));
            }
        };

        let mg_id = Uuid::from_str(&req_enq.multicast_group_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupQueueAccess::new(validator::Flag::Create, mg_id),
            )
            .await?;

        let f_cnt = downlink::multicast::enqueue(multicast::MulticastGroupQueueItem {
            multicast_group_id: mg_id.into(),
            f_port: req_enq.f_port as i16,
            data: req_enq.data.clone(),
            expires_at: if let Some(expires_at) = req_enq.expires_at {
                let expires_at: std::time::SystemTime = expires_at
                    .try_into()
                    .map_err(|e: prost_types::TimestampError| e.status())?;
                Some(expires_at.into())
            } else {
                None
            },
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(api::EnqueueMulticastGroupQueueItemResponse { f_cnt });
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req_enq.multicast_group_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn flush_queue(
        &self,
        request: Request<api::FlushMulticastGroupQueueRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupQueueAccess::new(validator::Flag::Delete, mg_id),
            )
            .await?;

        multicast::flush_queue(&mg_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );

        Ok(resp)
    }

    async fn list_queue(
        &self,
        request: Request<api::ListMulticastGroupQueueRequest>,
    ) -> Result<Response<api::ListMulticastGroupQueueResponse>, Status> {
        let req = request.get_ref();
        let mg_id = Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupQueueAccess::new(validator::Flag::List, mg_id),
            )
            .await?;

        let items = multicast::get_queue(&mg_id).await.map_err(|e| e.status())?;

        let mut f_cnts: HashSet<i64> = HashSet::new();
        let mut deduped_items: Vec<api::MulticastGroupQueueItem> = Vec::new();

        for qi in items {
            if f_cnts.insert(qi.f_cnt) {
                deduped_items.push(api::MulticastGroupQueueItem {
                    multicast_group_id: qi.multicast_group_id.to_string(),
                    f_cnt: qi.f_cnt as u32,
                    f_port: qi.f_port as u32,
                    data: qi.data.clone(),
                    expires_at: qi.expires_at.map(|v| {
                        let v: std::time::SystemTime = v.into();
                        v.into()
                    }),
                });
            }
        }

        let mut resp = Response::new(api::ListMulticastGroupQueueResponse {
            items: deduped_items,
        });
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );

        Ok(resp)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::AuthID;
    use crate::api::auth::validator::RequestValidator;
    use crate::gpstime::ToGpsTime;
    use crate::storage::{
        application, device, device_gateway, device_keys, device_profile, device_queue, fields,
        gateway, multicast, tenant, user,
    };
    use crate::test;
    use chirpstack_api::{common, internal};
    use chrono::{Duration, Utc};
    use lrwn::applayer::multicastsetup;

    #[tokio::test]
    async fn test_multicast_group() {
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

        // create gws
        let gw1 = gateway::create(gateway::Gateway {
            name: "test-gw-1".into(),
            tenant_id: t.id,
            gateway_id: EUI64::from_be_bytes([8, 7, 6, 54, 4, 3, 2, 1]),
            ..Default::default()
        })
        .await
        .unwrap();
        let gw2 = gateway::create(gateway::Gateway {
            name: "test-gw-2".into(),
            tenant_id: t.id,
            gateway_id: EUI64::from_be_bytes([8, 7, 6, 54, 4, 3, 2, 2]),
            ..Default::default()
        })
        .await
        .unwrap();

        // create application
        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // create device-profile
        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: Some(t.id),
            app_layer_params: fields::AppLayerParams {
                ts005_version: Some(fields::device_profile::Ts005Version::V100),
                ts005_f_port: 200,
                ..Default::default()
            },
            ..Default::default()
        })
        .await
        .unwrap();

        // create device
        let d = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp.id,
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            name: "test-dev".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // setup api
        let service = MulticastGroup::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateMulticastGroupRequest {
                multicast_group: Some(api::MulticastGroup {
                    name: "test-mg".into(),
                    application_id: app.id.to_string(),
                    region: common::Region::Eu868.into(),
                    mc_addr: "01020304".into(),
                    mc_nwk_s_key: "01020304050607080102030405060708".into(),
                    mc_app_s_key: "02020304050607080102030405060708".into(),
                    f_cnt: 20,
                    group_type: api::MulticastGroupType::ClassC.into(),
                    dr: 3,
                    frequency: 868300000,
                    class_b_ping_slot_periodicity: 1,
                    class_c_scheduling_type: api::MulticastGroupSchedulingType::GpsTime.into(),
                    ..Default::default()
                }),
            },
        );
        let create_resp = service.create(create_req).await.unwrap();
        let create_resp = create_resp.get_ref();

        // get
        let get_req = get_request(
            &u.id,
            api::GetMulticastGroupRequest {
                id: create_resp.id.clone(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::MulticastGroup {
                id: create_resp.id.clone(),
                name: "test-mg".into(),
                application_id: app.id.to_string(),
                region: common::Region::Eu868.into(),
                mc_addr: "01020304".into(),
                mc_nwk_s_key: "01020304050607080102030405060708".into(),
                mc_app_s_key: "02020304050607080102030405060708".into(),
                f_cnt: 20,
                group_type: api::MulticastGroupType::ClassC.into(),
                dr: 3,
                frequency: 868300000,
                class_b_ping_slot_periodicity: 1,
                class_c_scheduling_type: api::MulticastGroupSchedulingType::GpsTime.into(),
                setup: api::MulticastGroupSetup::OutOfBand.into(),
                mc_session_start: None,
                mc_session_timeout: 0,
            }),
            get_resp.get_ref().multicast_group
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateMulticastGroupRequest {
                multicast_group: Some(api::MulticastGroup {
                    id: create_resp.id.clone(),
                    name: "test-mg-updated".into(),
                    application_id: app.id.to_string(),
                    region: common::Region::Eu868.into(),
                    mc_addr: "02020304".into(),
                    mc_nwk_s_key: "02020304050607080102030405060708".into(),
                    mc_app_s_key: "03020304050607080102030405060708".into(),
                    f_cnt: 30,
                    group_type: api::MulticastGroupType::ClassB.into(),
                    dr: 2,
                    frequency: 868200000,
                    class_b_ping_slot_periodicity: 2,
                    class_c_scheduling_type: api::MulticastGroupSchedulingType::Delay.into(),
                    setup: api::MulticastGroupSetup::OutOfBand.into(),
                    mc_session_start: None,
                    mc_session_timeout: 0,
                }),
            },
        );
        let _ = service.update(update_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetMulticastGroupRequest {
                id: create_resp.id.clone(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::MulticastGroup {
                id: create_resp.id.clone(),
                name: "test-mg-updated".into(),
                application_id: app.id.to_string(),
                region: common::Region::Eu868.into(),
                mc_addr: "02020304".into(),
                mc_nwk_s_key: "02020304050607080102030405060708".into(),
                mc_app_s_key: "03020304050607080102030405060708".into(),
                f_cnt: 30,
                group_type: api::MulticastGroupType::ClassB.into(),
                dr: 2,
                frequency: 868200000,
                class_b_ping_slot_periodicity: 2,
                class_c_scheduling_type: api::MulticastGroupSchedulingType::Delay.into(),
                setup: api::MulticastGroupSetup::OutOfBand.into(),
                mc_session_start: None,
                mc_session_timeout: 0,
            }),
            get_resp.get_ref().multicast_group
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListMulticastGroupsRequest {
                search: "updated".into(),
                tenant_id: "".into(),
                application_id: app.id.to_string(),
                dev_eui: "".to_string(),
                limit: 10,
                offset: 0,
            },
        );
        let list_resp = service.list(list_req).await.unwrap();
        let list_resp = list_resp.get_ref();
        assert_eq!(1, list_resp.total_count);
        assert_eq!(1, list_resp.result.len());

        // add device
        let add_dev_req = get_request(
            &u.id,
            api::AddDeviceToMulticastGroupRequest {
                dev_eui: d.dev_eui.to_string(),
                multicast_group_id: create_resp.id.clone(),
            },
        );
        let _ = service.add_device(add_dev_req).await.unwrap();
        let device_queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert!(device_queue_items.is_empty());

        // enqueue (no uplink path set between dev <> gateway)
        let enqueue_req = get_request(
            &u.id,
            api::EnqueueMulticastGroupQueueItemRequest {
                queue_item: Some(api::MulticastGroupQueueItem {
                    multicast_group_id: create_resp.id.clone(),
                    f_port: 10,
                    data: vec![1, 2, 3],
                    ..Default::default()
                }),
            },
        );
        let enqueue_resp = service.enqueue(enqueue_req).await.unwrap();
        let enqueue_resp = enqueue_resp.get_ref();
        assert_eq!(30, enqueue_resp.f_cnt);

        // therefore queue is empty
        let list_queue_req = get_request(
            &u.id,
            api::ListMulticastGroupQueueRequest {
                multicast_group_id: create_resp.id.clone(),
            },
        );
        let list_queue_resp = service.list_queue(list_queue_req).await.unwrap();
        assert_eq!(0, list_queue_resp.get_ref().items.len());

        // set uplink device <> gateway path
        device_gateway::save_rx_info(&internal::DeviceGatewayRxInfo {
            dev_eui: d.dev_eui.to_vec(),
            items: vec![internal::DeviceGatewayRxInfoItem {
                gateway_id: gw1.gateway_id.to_vec(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await
        .unwrap();

        // enqueue
        let enqueue_req = get_request(
            &u.id,
            api::EnqueueMulticastGroupQueueItemRequest {
                queue_item: Some(api::MulticastGroupQueueItem {
                    multicast_group_id: create_resp.id.clone(),
                    f_port: 10,
                    data: vec![1, 2, 3],
                    ..Default::default()
                }),
            },
        );
        let enqueue_resp = service.enqueue(enqueue_req).await.unwrap();
        let enqueue_resp = enqueue_resp.get_ref();
        assert_eq!(31, enqueue_resp.f_cnt);

        // list queue
        let list_queue_req = get_request(
            &u.id,
            api::ListMulticastGroupQueueRequest {
                multicast_group_id: create_resp.id.clone(),
            },
        );
        let list_queue_resp = service.list_queue(list_queue_req).await.unwrap();
        let list_queue_resp = list_queue_resp.get_ref();
        assert_eq!(1, list_queue_resp.items.len());
        assert_eq!(
            api::MulticastGroupQueueItem {
                multicast_group_id: create_resp.id.clone(),
                f_cnt: 31,
                f_port: 10,
                data: vec![1, 2, 3],
                expires_at: None,
            },
            list_queue_resp.items[0]
        );

        // flush queue
        let flush_queue_req = get_request(
            &u.id,
            api::FlushMulticastGroupQueueRequest {
                multicast_group_id: create_resp.id.clone(),
            },
        );
        service.flush_queue(flush_queue_req).await.unwrap();

        // add gateways
        let add_gw_req = get_request(
            &u.id,
            api::AddGatewayToMulticastGroupRequest {
                multicast_group_id: create_resp.id.clone(),
                gateway_id: gw1.gateway_id.to_string(),
            },
        );
        let _ = service.add_gateway(add_gw_req).await.unwrap();
        let add_gw_req = get_request(
            &u.id,
            api::AddGatewayToMulticastGroupRequest {
                multicast_group_id: create_resp.id.clone(),
                gateway_id: gw2.gateway_id.to_string(),
            },
        );
        let _ = service.add_gateway(add_gw_req).await.unwrap();

        // enqueue (the two multicast-group gateways will be used)
        let enqueue_req = get_request(
            &u.id,
            api::EnqueueMulticastGroupQueueItemRequest {
                queue_item: Some(api::MulticastGroupQueueItem {
                    multicast_group_id: create_resp.id.clone(),
                    f_port: 10,
                    data: vec![1, 2, 3],
                    ..Default::default()
                }),
            },
        );
        let enqueue_resp = service.enqueue(enqueue_req).await.unwrap();
        let enqueue_resp = enqueue_resp.get_ref();
        assert_eq!(32, enqueue_resp.f_cnt);

        // we expect two queue items (for each gateway one)
        let queue_items = multicast::get_queue(&Uuid::from_str(&create_resp.id).unwrap())
            .await
            .unwrap();
        assert_eq!(2, queue_items.len());

        // remove device
        let remove_dev_req = get_request(
            &u.id,
            api::RemoveDeviceFromMulticastGroupRequest {
                dev_eui: d.dev_eui.to_string(),
                multicast_group_id: create_resp.id.clone(),
            },
        );
        let _ = service.remove_device(remove_dev_req).await.unwrap();

        // remove gateway
        let remove_gw_req = get_request(
            &u.id,
            api::RemoveGatewayFromMulticastGroupRequest {
                multicast_group_id: create_resp.id.clone(),
                gateway_id: gw1.gateway_id.to_string(),
            },
        );
        let _ = service.remove_gateway(remove_gw_req).await.unwrap();

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteMulticastGroupRequest {
                id: create_resp.id.clone(),
            },
        );
        let _ = service.delete(del_req).await.unwrap();
        let del_req = get_request(
            &u.id,
            api::DeleteMulticastGroupRequest {
                id: create_resp.id.clone(),
            },
        );
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }

    #[tokio::test]
    async fn test_ts005_multicast_group() {
        let _guard = test::prepare().await;

        let u = user::create(user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            max_gateway_count: 10,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: Some(t.id),
            supports_class_c: true,
            class_c_params: Some(fields::ClassCParams { timeout: 5 }),
            app_layer_params: fields::AppLayerParams {
                ts005_version: Some(fields::device_profile::Ts005Version::V100),
                ts005_f_port: 200,
                ..Default::default()
            },
            ..Default::default()
        })
        .await
        .unwrap();

        let d = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp.id,
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            name: "test-dev".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let _ = device_keys::create(device_keys::DeviceKeys {
            dev_eui: d.dev_eui,
            nwk_key: AES128Key::from_bytes([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            app_key: AES128Key::from_bytes([2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]),
            gen_app_key: AES128Key::from_bytes([3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]),
            ..Default::default()
        })
        .await
        .unwrap();

        let service = MulticastGroup::new(RequestValidator::new());
        let random_dev_addr_resp = service
            .get_random_dev_addr(get_request(
                &u.id,
                api::GetRandomMulticastGroupDevAddrRequest {
                    application_id: app.id.to_string(),
                },
            ))
            .await
            .unwrap()
            .into_inner();
        let _ = DevAddr::from_str(&random_dev_addr_resp.dev_addr).unwrap();

        let session_start = Utc::now() + Duration::hours(1);

        let create_resp = service
            .create(get_request(
                &u.id,
                api::CreateMulticastGroupRequest {
                    multicast_group: Some(api::MulticastGroup {
                        name: "test-ts005-mg".into(),
                        application_id: app.id.to_string(),
                        region: common::Region::Eu868.into(),
                        mc_addr: "01020304".into(),
                        mc_nwk_s_key: "".into(),
                        mc_app_s_key: "".into(),
                        f_cnt: 0,
                        group_type: api::MulticastGroupType::ClassC.into(),
                        dr: 3,
                        frequency: 868300000,
                        class_c_scheduling_type: api::MulticastGroupSchedulingType::GpsTime.into(),
                        setup: api::MulticastGroupSetup::Ts005.into(),
                        mc_session_start: Some(helpers::datetime_to_prost_timestamp(
                            &session_start,
                        )),
                        mc_session_timeout: 5,
                        ..Default::default()
                    }),
                },
            ))
            .await
            .unwrap()
            .into_inner();

        let mg_id = Uuid::from_str(&create_resp.id).unwrap();
        let mg = multicast::get(&mg_id).await.unwrap();
        assert_eq!("TS005", mg.setup);
        assert!(mg.mc_key.is_some());
        assert_eq!(
            Some(session_start.timestamp()),
            mg.mc_session_start.map(|v| v.timestamp())
        );
        assert_eq!(5, mg.mc_session_timeout);

        let other_app = application::create(application::Application {
            name: "test-other-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let other_dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-other-dp".into(),
            tenant_id: Some(t.id),
            ..Default::default()
        })
        .await
        .unwrap();

        let other_d = device::create(device::Device {
            application_id: other_app.id,
            device_profile_id: other_dp.id,
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 9]),
            name: "test-other-dev".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let add_other_resp = service
            .add_device(get_request(
                &u.id,
                api::AddDeviceToMulticastGroupRequest {
                    multicast_group_id: create_resp.id.clone(),
                    dev_eui: other_d.dev_eui.to_string(),
                },
            ))
            .await;
        assert_eq!(tonic::Code::NotFound, add_other_resp.err().unwrap().code());
        assert!(
            multicast::get_device(&mg_id, &other_d.dev_eui)
                .await
                .is_err()
        );

        let _ = service
            .add_device(get_request(
                &u.id,
                api::AddDeviceToMulticastGroupRequest {
                    multicast_group_id: create_resp.id.clone(),
                    dev_eui: d.dev_eui.to_string(),
                },
            ))
            .await
            .unwrap();

        let queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(1, queue_items.len());
        assert_eq!(200, queue_items[0].f_port);

        match multicastsetup::v1::Payload::from_slice(false, &queue_items[0].data).unwrap() {
            multicastsetup::v1::Payload::McGroupSetupReq(pl) => {
                assert_eq!(1, pl.mc_group_id_header.mc_group_id);
                assert_eq!(DevAddr::from_str("01020304").unwrap(), pl.mc_addr);
            }
            _ => panic!("expected McGroupSetupReq"),
        }

        let update_resp = service
            .update(get_request(
                &u.id,
                api::UpdateMulticastGroupRequest {
                    multicast_group: Some(api::MulticastGroup {
                        id: create_resp.id.clone(),
                        name: "test-ts005-mg-updated".into(),
                        application_id: app.id.to_string(),
                        region: common::Region::Eu868.into(),
                        mc_addr: "01020304".into(),
                        f_cnt: 0,
                        group_type: api::MulticastGroupType::ClassC.into(),
                        dr: 3,
                        frequency: 868500000,
                        class_c_scheduling_type: api::MulticastGroupSchedulingType::GpsTime.into(),
                        setup: api::MulticastGroupSetup::Ts005.into(),
                        mc_session_start: Some(helpers::datetime_to_prost_timestamp(
                            &(session_start + Duration::hours(1)),
                        )),
                        mc_session_timeout: 5,
                        ..Default::default()
                    }),
                },
            ))
            .await;
        assert_eq!(
            tonic::Code::FailedPrecondition,
            update_resp.err().unwrap().code()
        );

        let list_devices_resp = service
            .list_devices(get_request(
                &u.id,
                api::ListMulticastGroupDevicesRequest {
                    multicast_group_id: create_resp.id.clone(),
                    limit: 10,
                    offset: 0,
                },
            ))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(1, list_devices_resp.total_count);
        assert_eq!("test-dev", list_devices_resp.result[0].device_name);
        assert_eq!(d.dev_eui.to_string(), list_devices_resp.result[0].dev_eui);
        assert_eq!(1, list_devices_resp.result[0].mc_group_id);
        assert!(
            list_devices_resp.result[0]
                .mc_group_setup_completed_at
                .is_none()
        );
        assert!(
            list_devices_resp.result[0]
                .mc_session_completed_at
                .is_none()
        );
        assert!(!list_devices_resp.result[0].pending_delete);
        assert_eq!("", list_devices_resp.result[0].error_msg);

        device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();

        let ans = multicastsetup::v1::Payload::McGroupSetupAns(
            multicastsetup::v1::McGroupSetupAnsPayload {
                mc_group_id_header: multicastsetup::v1::McGroupSetupAnsPayloadMcGroupIdHeader {
                    mc_group_id: 1,
                    id_error: false,
                },
            },
        )
        .to_vec()
        .unwrap();

        crate::applayer::multicastsetup::handle_uplink(&d, &dp, &ans)
            .await
            .unwrap();

        let mgd = multicast::get_device(&mg_id, &d.dev_eui).await.unwrap();
        assert!(mgd.mc_group_setup_completed_at.is_some());
        assert!(mgd.mc_session_completed_at.is_none());
        assert_eq!("", mgd.error_msg);

        let queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(1, queue_items.len());

        match multicastsetup::v1::Payload::from_slice(false, &queue_items[0].data).unwrap() {
            multicastsetup::v1::Payload::McClassCSessionReq(pl) => {
                assert_eq!(1, pl.mc_group_id_header.mc_group_id);
                assert_eq!(
                    session_start.to_gps_time().num_seconds() as u32,
                    pl.session_time
                );
                assert_eq!(5, pl.session_time_out.time_out);
                assert_eq!(868300000, pl.dl_frequ);
                assert_eq!(3, pl.dr);
            }
            _ => panic!("expected McClassCSessionReq"),
        }

        device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();

        let ans = multicastsetup::v1::Payload::McClassCSessionAns(
            multicastsetup::v1::McClassCSessionAnsPayload {
                status_and_mc_group_id:
                    multicastsetup::v1::McClassCSessionAnsPayloadStatusAnsMcGroupId {
                        mc_group_id: 1,
                        dr_error: false,
                        freq_error: false,
                        mc_group_undefined: false,
                    },
                time_to_start: Some(60),
            },
        )
        .to_vec()
        .unwrap();

        crate::applayer::multicastsetup::handle_uplink(&d, &dp, &ans)
            .await
            .unwrap();

        let mgd = multicast::get_device(&mg_id, &d.dev_eui).await.unwrap();
        assert!(mgd.mc_session_completed_at.is_some());
        assert_eq!("", mgd.error_msg);

        let list_devices_resp = service
            .list_devices(get_request(
                &u.id,
                api::ListMulticastGroupDevicesRequest {
                    multicast_group_id: create_resp.id.clone(),
                    limit: 10,
                    offset: 0,
                },
            ))
            .await
            .unwrap()
            .into_inner();
        assert!(
            list_devices_resp.result[0]
                .mc_session_completed_at
                .is_some()
        );

        device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();
        let sync_session_start = session_start + Duration::hours(2);
        service
            .sync_ts005_session(get_request(
                &u.id,
                api::SyncMulticastGroupTs005SessionRequest {
                    multicast_group_id: create_resp.id.clone(),
                    mc_session_start: Some(helpers::datetime_to_prost_timestamp(
                        &sync_session_start,
                    )),
                    mc_session_timeout: 4,
                },
            ))
            .await
            .unwrap();

        let mgd = multicast::get_device(&mg_id, &d.dev_eui).await.unwrap();
        assert!(mgd.mc_session_completed_at.is_none());
        assert_eq!("", mgd.error_msg);
        let mg = multicast::get(&mg_id).await.unwrap();
        assert_eq!(
            Some(sync_session_start.timestamp()),
            mg.mc_session_start.map(|v| v.timestamp())
        );
        assert_eq!(4, mg.mc_session_timeout);

        let queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(1, queue_items.len());
        match multicastsetup::v1::Payload::from_slice(false, &queue_items[0].data).unwrap() {
            multicastsetup::v1::Payload::McClassCSessionReq(pl) => {
                assert_eq!(1, pl.mc_group_id_header.mc_group_id);
                assert_eq!(
                    sync_session_start.to_gps_time().num_seconds() as u32,
                    pl.session_time
                );
                assert_eq!(4, pl.session_time_out.time_out);
            }
            _ => panic!("expected McClassCSessionReq"),
        }

        let delete_resp = service
            .delete(get_request(
                &u.id,
                api::DeleteMulticastGroupRequest {
                    id: create_resp.id.clone(),
                },
            ))
            .await;
        assert_eq!(
            tonic::Code::FailedPrecondition,
            delete_resp.err().unwrap().code()
        );

        device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();

        let _ = service
            .remove_device(get_request(
                &u.id,
                api::RemoveDeviceFromMulticastGroupRequest {
                    multicast_group_id: create_resp.id.clone(),
                    dev_eui: d.dev_eui.to_string(),
                },
            ))
            .await
            .unwrap();

        let mgd = multicast::get_device(&mg_id, &d.dev_eui).await.unwrap();
        assert!(mgd.pending_delete);

        let queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(1, queue_items.len());
        match multicastsetup::v1::Payload::from_slice(false, &queue_items[0].data).unwrap() {
            multicastsetup::v1::Payload::McGroupDeleteReq(pl) => {
                assert_eq!(1, pl.mc_group_id_header.mc_group_id);
            }
            _ => panic!("expected McGroupDeleteReq"),
        }

        let list_devices_resp = service
            .list_devices(get_request(
                &u.id,
                api::ListMulticastGroupDevicesRequest {
                    multicast_group_id: create_resp.id.clone(),
                    limit: 10,
                    offset: 0,
                },
            ))
            .await
            .unwrap()
            .into_inner();
        assert!(list_devices_resp.result[0].pending_delete);

        let ans = multicastsetup::v1::Payload::McGroupDeleteAns(
            multicastsetup::v1::McGroupDeleteAnsPayload {
                mc_group_id_header: multicastsetup::v1::McGroupDeleteAnsPayloadMcGroupIdHeader {
                    mc_group_id: 1,
                    mc_group_undefined: false,
                },
            },
        )
        .to_vec()
        .unwrap();

        crate::applayer::multicastsetup::handle_uplink(&d, &dp, &ans)
            .await
            .unwrap();

        assert!(multicast::get_device(&mg_id, &d.dev_eui).await.is_err());

        let _ = service
            .add_device(get_request(
                &u.id,
                api::AddDeviceToMulticastGroupRequest {
                    multicast_group_id: create_resp.id.clone(),
                    dev_eui: d.dev_eui.to_string(),
                },
            ))
            .await
            .unwrap();

        let mgd = multicast::get_device(&mg_id, &d.dev_eui).await.unwrap();
        assert_eq!(Some(1), mgd.mc_group_id);
        assert!(!mgd.pending_delete);

        crate::applayer::multicastsetup::handle_uplink(&d, &dp, &ans)
            .await
            .unwrap();

        let mgd = multicast::get_device(&mg_id, &d.dev_eui).await.unwrap();
        assert_eq!(Some(1), mgd.mc_group_id);
        assert!(!mgd.pending_delete);
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(*user_id));
        req
    }
}
