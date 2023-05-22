use std::str::FromStr;

use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::relay_service_server::RelayService;
use lrwn::EUI64;

use super::auth::validator;
use super::error::ToStatus;
use super::helpers;

use crate::storage::relay;

pub struct Relay {
    validator: validator::RequestValidator,
}

impl Relay {
    pub fn new(validator: validator::RequestValidator) -> Self {
        Relay { validator }
    }
}

#[tonic::async_trait]
impl RelayService for Relay {
    async fn list(
        &self,
        request: Request<api::ListRelaysRequest>,
    ) -> Result<Response<api::ListRelaysResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDevicesAccess::new(validator::Flag::List, app_id),
            )
            .await?;

        let filters = relay::RelayFilters {
            application_id: Some(app_id),
        };

        let count = relay::get_relay_count(&filters)
            .await
            .map_err(|e| e.status())?;
        let items = relay::list_relays(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListRelaysResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|r| api::RelayListItem {
                    dev_eui: r.dev_eui.to_string(),
                    name: r.name.clone(),
                })
                .collect(),
        });

        resp.metadata_mut()
            .insert("x-log-application_id", req.application_id.parse().unwrap());
        Ok(resp)
    }

    async fn add_device(
        &self,
        request: Request<api::AddRelayDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let relay_dev_eui = EUI64::from_str(&req.relay_dev_eui).map_err(|e| e.status())?;
        let device_dev_eui = EUI64::from_str(&req.device_dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, relay_dev_eui),
            )
            .await?;
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, device_dev_eui),
            )
            .await?;

        relay::add_device(relay_dev_eui, device_dev_eui)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-relay_dev_eui", req.relay_dev_eui.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-device_dev_eui", req.device_dev_eui.parse().unwrap());

        Ok(resp)
    }

    async fn remove_device(
        &self,
        request: Request<api::RemoveRelayDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let relay_dev_eui = EUI64::from_str(&req.relay_dev_eui).map_err(|e| e.status())?;
        let device_dev_eui = EUI64::from_str(&req.device_dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, relay_dev_eui),
            )
            .await?;
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, device_dev_eui),
            )
            .await?;

        relay::remove_device(relay_dev_eui, device_dev_eui)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-relay_dev_eui", req.relay_dev_eui.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-device_dev_eui", req.device_dev_eui.parse().unwrap());

        Ok(resp)
    }

    async fn list_devices(
        &self,
        request: Request<api::ListRelayDevicesRequest>,
    ) -> Result<Response<api::ListRelayDevicesResponse>, Status> {
        let req = request.get_ref();
        let relay_dev_eui = EUI64::from_str(&req.relay_dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, relay_dev_eui),
            )
            .await?;

        let filters = relay::DeviceFilters {
            relay_dev_eui: Some(relay_dev_eui),
        };

        let count = relay::get_device_count(&filters)
            .await
            .map_err(|e| e.status())?;
        let items = relay::list_devices(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListRelayDevicesResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|d| api::RelayDeviceListItem {
                    dev_eui: d.dev_eui.to_string(),
                    name: d.name.clone(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&d.created_at)),
                })
                .collect(),
        });

        resp.metadata_mut()
            .insert("x-log-relay_dev_eui", req.relay_dev_eui.parse().unwrap());
        Ok(resp)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::{application, device, device_profile, tenant, user};
    use crate::test;

    #[tokio::test]
    async fn test_relay() {
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
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // create relay device-profile
        let dp_relay = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id,
            is_relay: true,
            ..Default::default()
        })
        .await
        .unwrap();

        // create devices
        let d_relay = device::create(device::Device {
            name: "relay-device".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            device_profile_id: dp_relay.id,
            application_id: app.id,
            ..Default::default()
        })
        .await
        .unwrap();
        let d = device::create(device::Device {
            name: "device".into(),
            dev_eui: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
            device_profile_id: dp.id,
            application_id: app.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // setup the api
        let service = Relay::new(RequestValidator::new());

        // list relays
        let list_req = get_request(
            &u.id,
            api::ListRelaysRequest {
                application_id: app.id.to_string(),
                limit: 10,
                ..Default::default()
            },
        );
        let list_resp = service.list(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);

        // add device
        let add_req = get_request(
            &u.id,
            api::AddRelayDeviceRequest {
                relay_dev_eui: d_relay.dev_eui.to_string(),
                device_dev_eui: d.dev_eui.to_string(),
            },
        );
        let _ = service.add_device(add_req).await.unwrap();

        // list devices
        let list_req = get_request(
            &u.id,
            api::ListRelayDevicesRequest {
                relay_dev_eui: d_relay.dev_eui.to_string(),
                limit: 10,
                ..Default::default()
            },
        );
        let list_resp = service.list_devices(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);
        assert_eq!(1, list_resp.get_ref().result.len());
        assert_eq!(d.dev_eui.to_string(), list_resp.get_ref().result[0].dev_eui);

        // remove device
        let remove_req = get_request(
            &u.id,
            api::RemoveRelayDeviceRequest {
                relay_dev_eui: d_relay.dev_eui.to_string(),
                device_dev_eui: d.dev_eui.to_string(),
            },
        );
        let _ = service.remove_device(remove_req).await.unwrap();
        let remove_req = get_request(
            &u.id,
            api::RemoveRelayDeviceRequest {
                relay_dev_eui: d_relay.dev_eui.to_string(),
                device_dev_eui: d.dev_eui.to_string(),
            },
        );
        assert!(service.remove_device(remove_req).await.is_err());
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(user_id.clone()));
        req
    }
}
