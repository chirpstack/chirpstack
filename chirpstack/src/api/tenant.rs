use std::str::FromStr;

use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::tenant_service_server::TenantService;

use super::auth::{validator, AuthID};
use super::error::ToStatus;
use super::helpers;
use crate::storage::{tenant, user};

pub struct Tenant {
    validator: validator::RequestValidator,
}

impl Tenant {
    pub fn new(validator: validator::RequestValidator) -> Self {
        Tenant { validator }
    }
}

#[tonic::async_trait]
impl TenantService for Tenant {
    async fn create(
        &self,
        request: Request<api::CreateTenantRequest>,
    ) -> Result<Response<api::CreateTenantResponse>, Status> {
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantsAccess::new(validator::Flag::Create),
            )
            .await?;

        let req_tenant = match &request.get_ref().tenant {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("tenant is missing"));
            }
        };

        let t = tenant::Tenant {
            name: req_tenant.name.clone(),
            description: req_tenant.description.clone(),
            can_have_gateways: req_tenant.can_have_gateways,
            max_device_count: req_tenant.max_device_count as i32,
            max_gateway_count: req_tenant.max_gateway_count as i32,
            private_gateways_up: req_tenant.private_gateways_up,
            private_gateways_down: req_tenant.private_gateways_down,
            ..Default::default()
        };

        let t = tenant::create(t).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::CreateTenantResponse {
            id: t.id.to_string(),
        });
        resp.metadata_mut()
            .insert("x-log-tenant_id", t.id.to_string().parse().unwrap());

        Ok(resp)
    }

    async fn get(
        &self,
        request: Request<api::GetTenantRequest>,
    ) -> Result<Response<api::GetTenantResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantAccess::new(validator::Flag::Read, tenant_id),
            )
            .await?;

        let t = tenant::get(&tenant_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetTenantResponse {
            tenant: Some(api::Tenant {
                id: t.id.to_string(),
                name: t.name,
                description: t.description,
                can_have_gateways: t.can_have_gateways,
                max_gateway_count: t.max_gateway_count as u32,
                max_device_count: t.max_device_count as u32,
                private_gateways_up: t.private_gateways_up,
                private_gateways_down: t.private_gateways_down,
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&t.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&t.updated_at)),
        });
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn update(
        &self,
        request: Request<api::UpdateTenantRequest>,
    ) -> Result<Response<()>, Status> {
        let req_tenant = match &request.get_ref().tenant {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("tenant is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_tenant.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantAccess::new(validator::Flag::Update, tenant_id),
            )
            .await?;

        // update
        let _ = tenant::update(tenant::Tenant {
            id: tenant_id,
            name: req_tenant.name.clone(),
            description: req_tenant.description.clone(),
            can_have_gateways: req_tenant.can_have_gateways,
            max_device_count: req_tenant.max_device_count as i32,
            max_gateway_count: req_tenant.max_gateway_count as i32,
            private_gateways_up: req_tenant.private_gateways_up,
            private_gateways_down: req_tenant.private_gateways_down,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req_tenant.id.parse().unwrap());

        Ok(resp)
    }

    async fn delete(
        &self,
        request: Request<api::DeleteTenantRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantAccess::new(validator::Flag::Delete, tenant_id),
            )
            .await?;

        tenant::delete(&tenant_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn list(
        &self,
        request: Request<api::ListTenantsRequest>,
    ) -> Result<Response<api::ListTenantsResponse>, Status> {
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantsAccess::new(validator::Flag::List),
            )
            .await?;

        let auth_id = request.extensions().get::<AuthID>().unwrap();
        let req = request.get_ref();
        let mut filters = tenant::Filters::default();

        if !req.search.is_empty() {
            filters.search = Some(req.search.clone());
        }

        match auth_id {
            AuthID::User(id) => {
                let u = user::get(id).await.map_err(|e| e.status())?;

                if !u.is_admin {
                    filters.user_id = Some(u.id);
                }
            }
            AuthID::Key(_) => {
                // Nothing else to do as the validator function already validated that the
                // API key must be a global admin key.

                if !req.user_id.is_empty() {
                    let user_id = Uuid::from_str(&req.user_id).map_err(|e| e.status())?;
                    filters.user_id = Some(user_id);
                }
            }
            _ => {
                // this should never happen
                return Err(Status::internal(
                    "request authenticated but no AuthID in extensions",
                ));
            }
        }

        let count = tenant::get_count(&filters).await.map_err(|e| e.status())?;
        let results = tenant::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::ListTenantsResponse {
            total_count: count as u32,
            result: results
                .iter()
                .map(|t| api::TenantListItem {
                    id: t.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&t.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&t.updated_at)),
                    name: t.name.clone(),
                    can_have_gateways: t.can_have_gateways,
                    private_gateways_up: t.private_gateways_up,
                    private_gateways_down: t.private_gateways_down,
                    max_gateway_count: t.max_gateway_count as u32,
                    max_device_count: t.max_device_count as u32,
                })
                .collect(),
        }))
    }

    async fn add_user(
        &self,
        request: Request<api::AddTenantUserRequest>,
    ) -> Result<Response<()>, Status> {
        let req_user = match &request.get_ref().tenant_user {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("tenant_user is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_user.tenant_id).map_err(|e| e.status())?;
        let user_id = user::get_by_email(&req_user.email)
            .await
            .map_err(|e| e.status())?
            .id;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantUsersAccess::new(validator::Flag::Create, tenant_id),
            )
            .await?;

        let _ = tenant::add_user(tenant::TenantUser {
            tenant_id,
            user_id,
            is_admin: req_user.is_admin,
            is_device_admin: req_user.is_device_admin,
            is_gateway_admin: req_user.is_gateway_admin,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req_user.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-user_id", user_id.to_string().parse().unwrap());

        Ok(resp)
    }

    async fn get_user(
        &self,
        request: Request<api::GetTenantUserRequest>,
    ) -> Result<Response<api::GetTenantUserResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;
        let user_id = Uuid::from_str(&req.user_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantUserAccess::new(validator::Flag::Read, tenant_id, user_id),
            )
            .await?;

        let u = user::get(&user_id).await.map_err(|e| e.status())?;
        let tu = tenant::get_user(&tenant_id, &user_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetTenantUserResponse {
            tenant_user: Some(api::TenantUser {
                tenant_id: tenant_id.to_string(),
                user_id: tu.user_id.to_string(),
                email: u.email.clone(),
                is_admin: tu.is_admin,
                is_device_admin: tu.is_device_admin,
                is_gateway_admin: tu.is_gateway_admin,
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&tu.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&tu.updated_at)),
        });
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-user_id", req.user_id.parse().unwrap());

        Ok(resp)
    }

    async fn update_user(
        &self,
        request: Request<api::UpdateTenantUserRequest>,
    ) -> Result<Response<()>, Status> {
        let req_user = match &request.get_ref().tenant_user {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("tenant_user is missing"));
            }
        };
        let tenant_id = Uuid::from_str(&req_user.tenant_id).map_err(|e| e.status())?;
        let user_id = Uuid::from_str(&req_user.user_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantUserAccess::new(
                    validator::Flag::Update,
                    tenant_id,
                    user_id,
                ),
            )
            .await?;

        tenant::update_user(tenant::TenantUser {
            tenant_id,
            user_id,
            is_admin: req_user.is_admin,
            is_device_admin: req_user.is_device_admin,
            is_gateway_admin: req_user.is_gateway_admin,
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req_user.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-user_id", req_user.user_id.parse().unwrap());

        Ok(resp)
    }

    async fn delete_user(
        &self,
        request: Request<api::DeleteTenantUserRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;
        let user_id = Uuid::from_str(&req.user_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantUserAccess::new(
                    validator::Flag::Delete,
                    tenant_id,
                    user_id,
                ),
            )
            .await?;

        let auth_id = request.extensions().get::<AuthID>().unwrap();
        if let AuthID::User(id) = auth_id {
            if id == &user_id {
                return Err(Status::invalid_argument(
                    "you can not delete yourself from the user",
                ));
            }
        }

        tenant::delete_user(&tenant_id, &user_id)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());
        resp.metadata_mut()
            .insert("x-log-user_id", req.user_id.parse().unwrap());

        Ok(resp)
    }

    async fn list_users(
        &self,
        request: Request<api::ListTenantUsersRequest>,
    ) -> Result<Response<api::ListTenantUsersResponse>, Status> {
        let req = request.get_ref();
        let tenant_id = Uuid::from_str(&req.tenant_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateTenantUsersAccess::new(validator::Flag::List, tenant_id),
            )
            .await?;

        let count = tenant::get_user_count(&tenant_id)
            .await
            .map_err(|e| e.status())?;
        let result = tenant::get_users(&tenant_id, req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(api::ListTenantUsersResponse {
            total_count: count as u32,
            result: result
                .iter()
                .map(|tu| api::TenantUserListItem {
                    tenant_id: tenant_id.to_string(),
                    user_id: tu.user_id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&tu.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&tu.updated_at)),
                    email: tu.email.clone(),
                    is_admin: tu.is_admin,
                    is_device_admin: tu.is_device_admin,
                    is_gateway_admin: tu.is_gateway_admin,
                })
                .collect(),
        });
        resp.metadata_mut()
            .insert("x-log-tenant_id", req.tenant_id.parse().unwrap());

        Ok(resp)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::test;

    #[tokio::test]
    async fn test_tenant() {
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

        // setup api
        let service = Tenant::new(RequestValidator::new());

        // create
        let create_req = api::CreateTenantRequest {
            tenant: Some(api::Tenant {
                name: "Test tenant".into(),
                description: "Test description".into(),
                can_have_gateways: true,
                max_device_count: 10,
                max_gateway_count: 3,
                ..Default::default()
            }),
        };
        let mut create_req = Request::new(create_req);
        create_req
            .extensions_mut()
            .insert(AuthID::User(u.id.clone()));
        let create_resp = service.create(create_req).await.unwrap();

        // get
        let get_req = api::GetTenantRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Tenant {
                id: create_resp.get_ref().id.clone(),
                name: "Test tenant".into(),
                description: "Test description".into(),
                can_have_gateways: true,
                max_device_count: 10,
                max_gateway_count: 3,
                ..Default::default()
            }),
            get_resp.get_ref().tenant
        );

        // update
        let up_req = api::UpdateTenantRequest {
            tenant: Some(api::Tenant {
                id: create_resp.get_ref().id.clone(),
                name: "Test tenant updated".into(),
                description: "Test description".into(),
                can_have_gateways: true,
                max_device_count: 10,
                max_gateway_count: 3,
                ..Default::default()
            }),
        };
        let mut up_req = Request::new(up_req);
        up_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.update(up_req).await.unwrap();

        // get
        let get_req = api::GetTenantRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Tenant {
                id: create_resp.get_ref().id.clone(),
                name: "Test tenant updated".into(),
                description: "Test description".into(),
                can_have_gateways: true,
                max_device_count: 10,
                max_gateway_count: 3,
                ..Default::default()
            }),
            get_resp.get_ref().tenant
        );

        // list
        let list_req = api::ListTenantsRequest {
            search: "update".into(),
            offset: 0,
            limit: 10,
            user_id: "".into(),
        };
        let mut list_req = Request::new(list_req);
        list_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let list_resp = service.list(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);
        assert_eq!(1, list_resp.get_ref().result.len());

        // delete
        let del_req = api::DeleteTenantRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.delete(del_req).await.unwrap();

        let del_req = api::DeleteTenantRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }
}
