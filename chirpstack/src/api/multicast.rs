use std::collections::HashSet;
use std::str::FromStr;

use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::multicast_group_service_server::MulticastGroupService;
use lrwn::{AES128Key, DevAddr, EUI64};

use super::auth::validator;
use super::error::ToStatus;
use super::helpers::{self, FromProto, ToProto};
use crate::downlink;
use crate::storage::multicast;

pub struct MulticastGroup {
    validator: validator::RequestValidator,
}

impl MulticastGroup {
    pub fn new(validator: validator::RequestValidator) -> Self {
        MulticastGroup { validator }
    }
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
                validator::ValidateMulticastGroupsAccess::new(validator::Flag::Create, app_id),
            )
            .await?;

        let mg = multicast::MulticastGroup {
            application_id: app_id,
            name: req_mg.name.clone(),
            region: req_mg.region().from_proto(),
            mc_addr: DevAddr::from_str(&req_mg.mc_addr).map_err(|e| e.status())?,
            mc_nwk_s_key: AES128Key::from_str(&req_mg.mc_nwk_s_key).map_err(|e| e.status())?,
            mc_app_s_key: AES128Key::from_str(&req_mg.mc_app_s_key).map_err(|e| e.status())?,
            f_cnt: req_mg.f_cnt as i64,
            group_type: match req_mg.group_type() {
                api::MulticastGroupType::ClassB => "B",
                api::MulticastGroupType::ClassC => "C",
            }
            .to_string(),
            dr: req_mg.dr as i16,
            frequency: req_mg.frequency as i64,
            class_b_ping_slot_period: req_mg.class_b_ping_slot_period as i32,
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
            multicast_group: Some(api::MulticastGroup {
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
                class_b_ping_slot_period: mg.class_b_ping_slot_period as u32,
                class_c_scheduling_type: mg.class_c_scheduling_type.to_proto().into(),
            }),
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

        let _ = multicast::update(multicast::MulticastGroup {
            id: mg_id,
            name: req_mg.name.clone(),
            region: req_mg.region().from_proto(),
            mc_addr: DevAddr::from_str(&req_mg.mc_addr).map_err(|e| e.status())?,
            mc_nwk_s_key: AES128Key::from_str(&req_mg.mc_nwk_s_key).map_err(|e| e.status())?,
            mc_app_s_key: AES128Key::from_str(&req_mg.mc_app_s_key).map_err(|e| e.status())?,
            f_cnt: req_mg.f_cnt as i64,
            group_type: match req_mg.group_type() {
                api::MulticastGroupType::ClassB => "B",
                api::MulticastGroupType::ClassC => "C",
            }
            .to_string(),
            dr: req_mg.dr as i16,
            frequency: req_mg.frequency as i64,
            class_b_ping_slot_period: req_mg.class_b_ping_slot_period as i32,
            class_c_scheduling_type: req_mg.class_c_scheduling_type().from_proto(),
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
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateMulticastGroupsAccess::new(validator::Flag::List, app_id),
            )
            .await?;

        let filters = multicast::Filters {
            application_id: Some(app_id),
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
                })
                .collect(),
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

        multicast::add_device(&mg_id, &dev_eui)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );
        resp.metadata_mut()
            .insert("x-log-dev_eui", req.dev_eui.parse().unwrap());

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

        multicast::remove_device(&mg_id, &dev_eui)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut().insert(
            "x-log-multicast_group_id",
            req.multicast_group_id.parse().unwrap(),
        );
        resp.metadata_mut()
            .insert("x-log-dev_eui", req.dev_eui.parse().unwrap());

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
            multicast_group_id: mg_id,
            f_port: req_enq.f_port as i16,
            data: req_enq.data.clone(),
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
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::{
        application, device, device_gateway, device_profile, gateway, multicast, tenant, user,
    };
    use crate::test;
    use chirpstack_api::{common, internal};

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
            tenant_id: t.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create device-profile
        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id.clone(),
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
                    class_b_ping_slot_period: 1,
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
                class_b_ping_slot_period: 1,
                class_c_scheduling_type: api::MulticastGroupSchedulingType::GpsTime.into(),
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
                    class_b_ping_slot_period: 2,
                    class_c_scheduling_type: api::MulticastGroupSchedulingType::Delay.into(),
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
                class_b_ping_slot_period: 2,
                class_c_scheduling_type: api::MulticastGroupSchedulingType::Delay.into(),
            }),
            get_resp.get_ref().multicast_group
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListMulticastGroupsRequest {
                search: "updated".into(),
                application_id: app.id.to_string(),
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

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(user_id.clone()));
        req
    }
}
