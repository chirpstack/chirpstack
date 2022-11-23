use std::str::FromStr;

use chrono::Utc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api;
use chirpstack_api::api::user_service_server::UserService;

use super::auth::{validator, AuthID};
use super::error::ToStatus;
use super::helpers;
use crate::storage::{tenant, user};

pub struct User {
    validator: validator::RequestValidator,
    pw_hash_iterations: u32,
}

impl User {
    pub fn new(validator: validator::RequestValidator) -> Self {
        User {
            validator,
            pw_hash_iterations: 10_000,
        }
    }
}

#[tonic::async_trait]
impl UserService for User {
    async fn create(
        &self,
        request: Request<api::CreateUserRequest>,
    ) -> Result<Response<api::CreateUserResponse>, Status> {
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateUsersAccess::new(validator::Flag::Create),
            )
            .await?;

        let req = request.get_ref();
        let req_user = match &req.user {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("user is missing"));
            }
        };

        let mut u = user::User {
            is_admin: req_user.is_admin,
            is_active: req_user.is_active,
            email: req_user.email.clone(),
            note: req_user.note.clone(),
            ..Default::default()
        };

        u.set_password_hash(&req.password, self.pw_hash_iterations)
            .map_err(|e| e.status())?;

        u = user::create(u).await.map_err(|e| e.status())?;

        for tu in &req.tenants {
            let tenant_id = Uuid::from_str(&tu.tenant_id).map_err(|e| e.status())?;

            tenant::add_user(tenant::TenantUser {
                tenant_id,
                user_id: u.id,
                is_admin: tu.is_admin,
                is_device_admin: tu.is_device_admin,
                is_gateway_admin: tu.is_gateway_admin,
                ..Default::default()
            })
            .await
            .map_err(|e| e.status())?;
        }

        let mut resp = Response::new(api::CreateUserResponse {
            id: u.id.to_string(),
        });
        resp.metadata_mut()
            .insert("x-log-user_id", u.id.to_string().parse().unwrap());

        Ok(resp)
    }

    async fn get(
        &self,
        request: Request<api::GetUserRequest>,
    ) -> Result<Response<api::GetUserResponse>, Status> {
        let req = request.get_ref();
        let user_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateUserAccess::new(validator::Flag::Read, user_id),
            )
            .await?;

        let u = user::get(&user_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(api::GetUserResponse {
            user: Some(api::User {
                id: u.id.to_string(),
                is_admin: u.is_admin,
                is_active: u.is_active,
                email: u.email.clone(),
                note: u.note.clone(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&u.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&u.updated_at)),
        });
        resp.metadata_mut()
            .insert("x-log-user_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn update(
        &self,
        request: Request<api::UpdateUserRequest>,
    ) -> Result<Response<()>, Status> {
        let req_user = match &request.get_ref().user {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("user is missing"));
            }
        };
        let user_id = Uuid::from_str(&req_user.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateUserAccess::new(validator::Flag::Update, user_id),
            )
            .await?;

        // update
        let _ = user::update(user::User {
            id: user_id,
            is_admin: req_user.is_admin,
            is_active: req_user.is_active,
            email: req_user.email.clone(),
            email_verified: true,
            note: req_user.note.clone(),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-user_id", req_user.id.parse().unwrap());

        Ok(resp)
    }

    async fn delete(
        &self,
        request: Request<api::DeleteUserRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let user_id = Uuid::from_str(&req.id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateUserAccess::new(validator::Flag::Delete, user_id),
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

        user::delete(&user_id).await.map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-user_id", req.id.parse().unwrap());

        Ok(resp)
    }

    async fn list(
        &self,
        request: Request<api::ListUsersRequest>,
    ) -> Result<Response<api::ListUsersResponse>, Status> {
        let req = request.get_ref();
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateUsersAccess::new(validator::Flag::List),
            )
            .await?;

        let count = user::get_count().await.map_err(|e| e.status())?;
        let items = user::list(req.limit as i64, req.offset as i64)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::ListUsersResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|u| api::UserListItem {
                    id: u.id.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&u.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&u.updated_at)),
                    email: u.email.clone(),
                    is_admin: u.is_admin,
                    is_active: u.is_active,
                })
                .collect(),
        }))
    }

    async fn update_password(
        &self,
        request: Request<api::UpdateUserPasswordRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let user_id = Uuid::from_str(&req.user_id).map_err(|e| e.status())?;
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateUserAccess::new(validator::Flag::UpdateProfile, user_id),
            )
            .await?;

        // get
        let mut u = user::get(&user_id).await.map_err(|e| e.status())?;

        // set password
        u.updated_at = Utc::now();
        u.set_password_hash(&req.password, self.pw_hash_iterations)
            .map_err(|e| e.status())?;

        // update
        let _ = user::set_password_hash(&u.id, &u.password_hash)
            .await
            .map_err(|e| e.status())?;

        let mut resp = Response::new(());
        resp.metadata_mut()
            .insert("x-log-user_id", req.user_id.parse().unwrap());

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
    async fn test_user() {
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
        let service = User::new(RequestValidator::new());

        // create
        let create_req = api::CreateUserRequest {
            password: "secret".into(),
            tenants: vec![],
            user: Some(api::User {
                is_admin: true,
                is_active: true,
                email: "foo@bar".into(),
                note: "test user".into(),
                ..Default::default()
            }),
        };
        let mut create_req = Request::new(create_req);
        create_req
            .extensions_mut()
            .insert(AuthID::User(u.id.clone()));
        let create_resp = service.create(create_req).await.unwrap();

        // get
        let get_req = api::GetUserRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::User {
                id: create_resp.get_ref().id.clone(),
                is_admin: true,
                is_active: true,
                email: "foo@bar".into(),
                note: "test user".into(),
                ..Default::default()
            }),
            get_resp.get_ref().user
        );

        // update
        let up_req = api::UpdateUserRequest {
            user: Some(api::User {
                id: create_resp.get_ref().id.clone(),
                is_admin: false,
                is_active: true,
                email: "foo@bar".into(),
                note: "updated user".into(),
                ..Default::default()
            }),
        };
        let mut up_req = Request::new(up_req);
        up_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.update(up_req).await.unwrap();

        // get
        let get_req = api::GetUserRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut get_req = Request::new(get_req);
        get_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::User {
                id: create_resp.get_ref().id.clone(),
                is_admin: false,
                is_active: true,
                email: "foo@bar".into(),
                note: "updated user".into(),
                ..Default::default()
            }),
            get_resp.get_ref().user
        );

        // update password
        let up_req = api::UpdateUserPasswordRequest {
            user_id: create_resp.get_ref().id.clone(),
            password: "newpassword".into(),
        };
        let mut up_req = Request::new(up_req);
        up_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.update_password(up_req).await.unwrap();

        // list
        let list_req = api::ListUsersRequest {
            offset: 0,
            limit: 10,
        };
        let mut list_req = Request::new(list_req);
        list_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let list_resp = service.list(list_req).await.unwrap();
        // * Admin from migrations
        // * User that we created for auth
        // * User that we created through API
        assert_eq!(3, list_resp.get_ref().total_count);
        assert_eq!(3, list_resp.get_ref().result.len());

        // delete
        let del_req = api::DeleteUserRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let _ = service.delete(del_req).await.unwrap();

        let del_req = api::DeleteUserRequest {
            id: create_resp.get_ref().id.clone(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());

        let del_req = api::DeleteUserRequest {
            id: u.id.to_string(),
        };
        let mut del_req = Request::new(del_req);
        del_req.extensions_mut().insert(AuthID::User(u.id.clone()));
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }
}
