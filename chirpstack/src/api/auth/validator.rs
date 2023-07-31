use async_trait::async_trait;
use diesel::dsl;
use diesel::prelude::*;
use tokio::task;
use tonic::{Extensions, Status};
use tracing::error;
use uuid::Uuid;

use lrwn::EUI64;

use super::error::Error;
use crate::api::auth::AuthID;
use crate::storage::get_db_conn;
use crate::storage::schema::{
    api_key, application, device, device_profile, gateway, multicast_group, tenant, tenant_user,
    user,
};

#[derive(Copy, Clone)]
pub enum Flag {
    Create,
    Read,
    Update,
    UpdateProfile,
    Delete,
    List,
}

pub struct RequestValidator {}

impl RequestValidator {
    pub fn new() -> Self {
        RequestValidator {}
    }

    pub async fn validate(
        &self,
        ext: &Extensions,
        auth_validator: impl Validator + Sync,
    ) -> Result<(), Status> {
        let id = ext.get::<AuthID>().unwrap();
        auth_validator.validate(id).await?;

        Ok(())
    }
}

#[async_trait]
pub trait Validator {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error>;
    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error>;
    async fn validate(&self, id: &AuthID) -> Result<(), Status> {
        let res = match id {
            AuthID::User(id) => self.validate_user(id).await,
            AuthID::Key(id) => self.validate_key(id).await,
            AuthID::None => {
                return Err(Status::unauthenticated("no authorization provided"));
            }
        };

        match res {
            Ok(v) => {
                if v > 0 {
                    return Ok(());
                }

                Err(Status::unauthenticated(""))
            }
            Err(e) => {
                error!(
                    error = %e,
                    "Validator function error"
                );
                Err(Status::internal(""))
            }
        }
    }
}

pub struct ValidateActiveUser {}

impl ValidateActiveUser {
    pub fn new() -> Self {
        ValidateActiveUser {}
    }
}

#[async_trait]
impl Validator for ValidateActiveUser {
    async fn validate_key(&self, _: &Uuid) -> Result<i64, Error> {
        Ok(0)
    }

    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let count = user::dsl::user
                    .select(dsl::count_star())
                    .find(id)
                    .filter(user::dsl::is_active.eq(true))
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }
}

pub struct ValidateIsAdmin {}

impl ValidateIsAdmin {
    pub fn new() -> Self {
        ValidateIsAdmin {}
    }
}

#[async_trait]
impl Validator for ValidateIsAdmin {
    async fn validate_key(&self, _: &Uuid) -> Result<i64, Error> {
        Ok(0)
    }

    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let count = user::dsl::user
                    .select(dsl::count_star())
                    .find(id)
                    .filter(
                        user::dsl::is_active
                            .eq(true)
                            .and(user::dsl::is_admin.eq(true)),
                    )
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }
}

pub struct ValidateActiveUserOrKey {}

impl ValidateActiveUserOrKey {
    pub fn new() -> Self {
        ValidateActiveUserOrKey {}
    }
}

#[async_trait]
impl Validator for ValidateActiveUserOrKey {
    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let count = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }

    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let count = user::dsl::user
                    .select(dsl::count_star())
                    .find(id)
                    .filter(user::dsl::is_active.eq(true))
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }
}

pub struct ValidateUsersAccess {
    flag: Flag,
}

impl ValidateUsersAccess {
    pub fn new(flag: Flag) -> Self {
        ValidateUsersAccess { flag }
    }
}

#[async_trait]
impl Validator for ValidateUsersAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .find(&id)
                    .filter(user::dsl::is_active.eq(true))
                    .into_boxed();

                match flag {
                    // admin user
                    Flag::Create | Flag::List => {
                        q = q.filter(user::dsl::is_admin.eq(true));
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                // admin api key
                let count = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .filter(api_key::dsl::is_admin.eq(true))
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }
}

pub struct ValidateUserAccess {
    flag: Flag,
    user_id: Uuid,
}

impl ValidateUserAccess {
    pub fn new(flag: Flag, user_id: Uuid) -> Self {
        ValidateUserAccess { flag, user_id }
    }
}

#[async_trait]
impl Validator for ValidateUserAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let user_id = self.user_id;
            let flag = self.flag;

            move || {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .find(&id)
                    .filter(user::dsl::is_active.eq(true))
                    .into_boxed();

                match flag {
                    // admin user
                    // user itself
                    Flag::Read | Flag::UpdateProfile => {
                        q = q.filter(user::dsl::is_admin.eq(true).or(user::dsl::id.eq(&user_id)));
                    }
                    // admin user
                    Flag::Update | Flag::Delete => {
                        q = q.filter(user::dsl::is_admin.eq(true));
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                // admin api key
                let count = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .filter(api_key::dsl::is_admin.eq(true))
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }
}

pub struct ValidateApiKeysAccess {
    flag: Flag,
    tenant_id: Uuid,
}

impl ValidateApiKeysAccess {
    pub fn new(flag: Flag, tenant_id: Option<Uuid>) -> Self {
        ValidateApiKeysAccess {
            flag,
            tenant_id: match tenant_id {
                Some(v) => v,
                None => Uuid::nil(),
            },
        }
    }
}

#[async_trait]
impl Validator for ValidateApiKeysAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let tenant_id = self.tenant_id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;

                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table.left_join(tenant::table))
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin.eq(true).or(tenant_user::dsl::is_admin
                                .eq(true)
                                .and(tenant::dsl::id.eq(&tenant_id))),
                        );
                    }
                    // admin user
                    // tenant user (api_key filtered by tenant_id in api)
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, _id: &Uuid) -> Result<i64, Error> {
        Ok(0)
    }
}

pub struct ValidateApiKeyAccess {
    flag: Flag,
    id: Uuid,
}

impl ValidateApiKeyAccess {
    pub fn new(flag: Flag, id: Uuid) -> Self {
        ValidateApiKeyAccess { flag, id }
    }
}

#[async_trait]
impl Validator for ValidateApiKeyAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let self_id = self.id;
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;

                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(tenant::table.left_join(api_key::table)),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant admin
                    Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin.eq(true).or(tenant_user::dsl::is_admin
                                .eq(true)
                                .and(api_key::dsl::id.eq(&self_id))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, _id: &Uuid) -> Result<i64, Error> {
        Ok(0)
    }
}

pub struct ValidateTenantsAccess {
    flag: Flag,
}

impl ValidateTenantsAccess {
    pub fn new(flag: Flag) -> Self {
        ValidateTenantsAccess { flag }
    }
}

#[async_trait]
impl Validator for ValidateTenantsAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;

                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .find(&id)
                    .filter(user::dsl::is_active.eq(true))
                    .into_boxed();

                match flag {
                    // admin user
                    Flag::Create => {
                        q = q.filter(user::dsl::is_admin.eq(true));
                    }
                    // any active user (results are filtered by the storage function)
                    Flag::List => {}
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                // admin api key
                let count = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .filter(api_key::dsl::is_admin.eq(true))
                    .first(&mut c)?;
                Ok(count)
            }
        })
        .await?
    }
}

pub struct ValidateTenantAccess {
    flag: Flag,
    tenant_id: Uuid,
}

impl ValidateTenantAccess {
    pub fn new(flag: Flag, tenant_id: Uuid) -> Self {
        ValidateTenantAccess { flag, tenant_id }
    }
}

#[async_trait]
impl Validator for ValidateTenantAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;

                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table.left_join(tenant::table))
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant user
                    Flag::Read => {
                        q = q.filter(user::is_admin.eq(true).or(tenant::dsl::id.eq(&tenant_id)));
                    }

                    // global admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(user::is_admin.eq(true));
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;

                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    // admin api key
                    Flag::Update | Flag::Delete => {
                        q = q.filter(api_key::dsl::is_admin.eq(true));
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateTenantUsersAccess {
    flag: Flag,
    tenant_id: Uuid,
}

impl ValidateTenantUsersAccess {
    pub fn new(flag: Flag, tenant_id: Uuid) -> Self {
        ValidateTenantUsersAccess { flag, tenant_id }
    }
}

#[async_trait]
impl Validator for ValidateTenantUsersAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table.left_join(tenant::table))
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin.eq(true).or(tenant::dsl::id
                                .eq(&tenant_id)
                                .and(tenant_user::dsl::is_admin.eq(true))),
                        );
                    }
                    // global admin
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateTenantUserAccess {
    flag: Flag,
    tenant_id: Uuid,
    user_id: Uuid,
}

impl ValidateTenantUserAccess {
    pub fn new(flag: Flag, tenant_id: Uuid, user_id: Uuid) -> Self {
        ValidateTenantUserAccess {
            flag,
            tenant_id,
            user_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateTenantUserAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;
            let user_id = self.user_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table.left_join(tenant::table))
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant admin
                    // user itself
                    Flag::Read => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(tenant::dsl::id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::user_id.eq(&user_id))),
                        );
                    }
                    // admin user
                    // tenant admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin.eq(true).or(tenant::dsl::id
                                .eq(&tenant_id)
                                .and(tenant_user::dsl::is_admin.eq(true))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read | Flag::Update | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateApplicationsAccess {
    flag: Flag,
    tenant_id: Uuid,
}

impl ValidateApplicationsAccess {
    pub fn new(flag: Flag, tenant_id: Uuid) -> Self {
        ValidateApplicationsAccess { flag, tenant_id }
    }
}

#[async_trait]
impl Validator for ValidateApplicationsAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table.left_join(tenant::table))
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant admin
                    // tenant device admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(tenant::dsl::id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    // global admin
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    // admin api key
                    // tenant api key (api will do filtering)
                    Flag::List => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateApplicationAccess {
    flag: Flag,
    application_id: Uuid,
}

impl ValidateApplicationAccess {
    pub fn new(flag: Flag, app_id: Uuid) -> Self {
        ValidateApplicationAccess {
            flag,
            application_id: app_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateApplicationAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let application_id = self.application_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant user
                    Flag::Read => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id.eq(&application_id)),
                        );
                    }
                    // global admin
                    // tenant admin
                    // tenant device admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id
                                    .eq(&application_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(application::dsl::id
                                    .eq(&application_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let application_id = self.application_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read | Flag::Update | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id.eq(&application_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDeviceProfileTemplatesAccess {
    flag: Flag,
}

impl ValidateDeviceProfileTemplatesAccess {
    pub fn new(flag: Flag) -> Self {
        ValidateDeviceProfileTemplatesAccess { flag }
    }
}

#[async_trait]
impl Validator for ValidateDeviceProfileTemplatesAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    Flag::Create => {
                        q = q.filter(user::dsl::is_admin.eq(true));
                    }
                    // any active user
                    Flag::List => {}
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    Flag::Create => {
                        q = q.filter(api_key::dsl::is_admin.eq(true));
                    }
                    // any api key
                    Flag::List => {}
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDeviceProfileTemplateAccess {
    flag: Flag,
}

impl ValidateDeviceProfileTemplateAccess {
    pub fn new(flag: Flag) -> Self {
        ValidateDeviceProfileTemplateAccess { flag }
    }
}

#[async_trait]
impl Validator for ValidateDeviceProfileTemplateAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // any active user
                    Flag::Read => {}
                    // global admin user
                    Flag::Update | Flag::Delete => {
                        q = q.filter(user::dsl::is_admin.eq(true));
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // any api key
                    Flag::Read => {}
                    // admin api key
                    Flag::Update | Flag::Delete => {
                        q = q.filter(api_key::dsl::is_admin.eq(true));
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDeviceProfilesAccess {
    flag: Flag,
    tenant_id: Uuid,
}

impl ValidateDeviceProfilesAccess {
    pub fn new(flag: Flag, tenant_id: Uuid) -> Self {
        ValidateDeviceProfilesAccess { flag, tenant_id }
    }
}

#[async_trait]
impl Validator for ValidateDeviceProfilesAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table)
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant admin
                    // tenant device admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant_user::dsl::tenant_id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(tenant_user::dsl::tenant_id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    // global admin
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant_user::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDeviceProfileAccess {
    flag: Flag,
    device_profile_id: Uuid,
}

impl ValidateDeviceProfileAccess {
    pub fn new(flag: Flag, dp_id: Uuid) -> Self {
        ValidateDeviceProfileAccess {
            flag,
            device_profile_id: dp_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateDeviceProfileAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let device_profile_id = self.device_profile_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table
                            .left_join(device_profile::table.on(
                                tenant_user::dsl::tenant_id.eq(device_profile::dsl::tenant_id),
                            )),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant user
                    Flag::Read => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(device_profile::dsl::id.eq(&device_profile_id)),
                        );
                    }
                    // global admin
                    // tenant admin user
                    // tenant device admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(device_profile::dsl::id
                                    .eq(&device_profile_id)
                                    .and(device_profile::dsl::tenant_id.is_not_null())
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(device_profile::dsl::id
                                    .eq(&device_profile_id)
                                    .and(device_profile::dsl::tenant_id.is_not_null())
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let device_profile_id = self.device_profile_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q =
                    api_key::dsl::api_key
                        .select(dsl::count_star())
                        .left_join(device_profile::table.on(
                            api_key::dsl::tenant_id.eq(device_profile::dsl::tenant_id.nullable()),
                        ))
                        .filter(api_key::dsl::id.eq(&id))
                        .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read | Flag::Update | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(device_profile::dsl::id.eq(&device_profile_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                };

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDevicesAccess {
    flag: Flag,
    application_id: Uuid,
}

impl ValidateDevicesAccess {
    pub fn new(flag: Flag, app_id: Uuid) -> Self {
        ValidateDevicesAccess {
            flag,
            application_id: app_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateDevicesAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let application_id = self.application_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant admin
                    // tenant device admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id
                                    .eq(&application_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(application::dsl::id
                                    .eq(&application_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    // admin user
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id.eq(&application_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let application_id = self.application_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id.eq(&application_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDeviceAccess {
    flag: Flag,
    dev_eui: EUI64,
}

impl ValidateDeviceAccess {
    pub fn new(flag: Flag, dev_eui: EUI64) -> Self {
        ValidateDeviceAccess { flag, dev_eui }
    }
}

#[async_trait]
impl Validator for ValidateDeviceAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let dev_eui = self.dev_eui;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .left_join(
                        device::table.on(application::dsl::id.eq(device::dsl::application_id)),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant user
                    Flag::Read => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(device::dsl::dev_eui.eq(&dev_eui)),
                        );
                    }
                    // admin user
                    // tenant admin
                    // tenant device admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(device::dsl::dev_eui
                                    .eq(&dev_eui)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(device::dsl::dev_eui
                                    .eq(&dev_eui)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let dev_eui = self.dev_eui;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .left_join(
                        device::table.on(application::dsl::id.eq(device::dsl::application_id)),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read | Flag::Update | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(device::dsl::dev_eui.eq(&dev_eui)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateDeviceQueueAccess {
    flag: Flag,
    dev_eui: EUI64,
}

impl ValidateDeviceQueueAccess {
    pub fn new(flag: Flag, dev_eui: EUI64) -> Self {
        ValidateDeviceQueueAccess { flag, dev_eui }
    }
}

#[async_trait]
impl Validator for ValidateDeviceQueueAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let dev_eui = self.dev_eui;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .left_join(
                        device::table.on(application::dsl::id.eq(device::dsl::application_id)),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant user
                    Flag::Create | Flag::List | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(device::dsl::dev_eui.eq(&dev_eui)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let dev_eui = self.dev_eui;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .left_join(
                        device::table.on(application::dsl::id.eq(device::dsl::application_id)),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(device::dsl::dev_eui.eq(&dev_eui)),
                        )
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateGatewaysAccess {
    flag: Flag,
    tenant_id: Uuid,
}

impl ValidateGatewaysAccess {
    pub fn new(flag: Flag, tenant_id: Uuid) -> Self {
        ValidateGatewaysAccess { flag, tenant_id }
    }
}

#[async_trait]
impl Validator for ValidateGatewaysAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(tenant_user::table.left_join(tenant::table))
                    .filter(user::dsl::id.eq(id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // global admin
                    // tenant admin
                    // gateway admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(tenant::dsl::id
                                    .eq(&tenant_id)
                                    .and(tenant_user::dsl::is_gateway_admin.eq(true))),
                        );
                    }
                    // global admin
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(tenant::dsl::id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let tenant_id = self.tenant_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .find(&id)
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(api_key::dsl::tenant_id.eq(&tenant_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateGatewayAccess {
    flag: Flag,
    gateway_id: EUI64,
}

impl ValidateGatewayAccess {
    pub fn new(flag: Flag, gateway_id: EUI64) -> Self {
        ValidateGatewayAccess { flag, gateway_id }
    }
}

#[async_trait]
impl Validator for ValidateGatewayAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let gateway_id = self.gateway_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(tenant::table.left_join(gateway::table)),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant user
                    Flag::Read => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(gateway::dsl::gateway_id.eq(&gateway_id)),
                        );
                    }
                    // admin user
                    // tenant admin
                    // gateway admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(gateway::dsl::gateway_id
                                    .eq(&gateway_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(gateway::dsl::gateway_id
                                    .eq(&gateway_id)
                                    .and(tenant_user::dsl::is_gateway_admin.eq(true))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let gateway_id = self.gateway_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        gateway::table
                            .on(api_key::dsl::tenant_id.eq(gateway::dsl::tenant_id.nullable())),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read | Flag::Update | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(gateway::dsl::gateway_id.eq(&gateway_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateMulticastGroupsAccess {
    flag: Flag,
    application_id: Uuid,
}

impl ValidateMulticastGroupsAccess {
    pub fn new(flag: Flag, application_id: Uuid) -> Self {
        ValidateMulticastGroupsAccess {
            flag,
            application_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateMulticastGroupsAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let application_id = self.application_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant admin
                    // tenant device admin
                    Flag::Create => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id
                                    .eq(&application_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(application::dsl::id
                                    .eq(&application_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    // admin user
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id.eq(&application_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let application_id = self.application_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(application::dsl::id.eq(&application_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateMulticastGroupAccess {
    flag: Flag,
    multicast_group_id: Uuid,
}

impl ValidateMulticastGroupAccess {
    pub fn new(flag: Flag, multicast_group_id: Uuid) -> Self {
        ValidateMulticastGroupAccess {
            flag,
            multicast_group_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateMulticastGroupAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let multicast_group_id = self.multicast_group_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .left_join(
                        multicast_group::table
                            .on(application::dsl::id.eq(multicast_group::dsl::application_id)),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant user
                    Flag::Read => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(multicast_group::dsl::id.eq(&multicast_group_id)),
                        );
                    }
                    // admin user
                    // tenant admin
                    // tenant device admin
                    Flag::Update | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(multicast_group::dsl::id
                                    .eq(&multicast_group_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(multicast_group::dsl::id
                                    .eq(&multicast_group_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let multicast_group_id = self.multicast_group_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .left_join(
                        multicast_group::table
                            .on(application::dsl::id.eq(multicast_group::dsl::application_id)),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Read | Flag::Update | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(multicast_group::dsl::id.eq(&multicast_group_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

pub struct ValidateMulticastGroupQueueAccess {
    flag: Flag,
    multicast_group_id: Uuid,
}

impl ValidateMulticastGroupQueueAccess {
    pub fn new(flag: Flag, multicast_group_id: Uuid) -> Self {
        ValidateMulticastGroupQueueAccess {
            flag,
            multicast_group_id,
        }
    }
}

#[async_trait]
impl Validator for ValidateMulticastGroupQueueAccess {
    async fn validate_user(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let multicast_group_id = self.multicast_group_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = user::dsl::user
                    .select(dsl::count_star())
                    .left_join(
                        tenant_user::table.left_join(
                            application::table
                                .on(tenant_user::dsl::tenant_id.eq(application::dsl::tenant_id)),
                        ),
                    )
                    .left_join(
                        multicast_group::table
                            .on(application::dsl::id.eq(multicast_group::dsl::application_id)),
                    )
                    .filter(user::dsl::id.eq(&id).and(user::dsl::is_active.eq(true)))
                    .into_boxed();

                match flag {
                    // admin user
                    // tenant admin
                    // tenant device admin
                    Flag::Create | Flag::Delete => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(multicast_group::dsl::id
                                    .eq(&multicast_group_id)
                                    .and(tenant_user::dsl::is_admin.eq(true)))
                                .or(multicast_group::dsl::id
                                    .eq(&multicast_group_id)
                                    .and(tenant_user::dsl::is_device_admin.eq(true))),
                        );
                    }
                    // admin user
                    // tenant user
                    Flag::List => {
                        q = q.filter(
                            user::dsl::is_admin
                                .eq(true)
                                .or(multicast_group::dsl::id.eq(&multicast_group_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }

    async fn validate_key(&self, id: &Uuid) -> Result<i64, Error> {
        task::spawn_blocking({
            let id = *id;
            let flag = self.flag;
            let multicast_group_id = self.multicast_group_id;

            move || -> Result<i64, Error> {
                let mut c = get_db_conn()?;
                let mut q = api_key::dsl::api_key
                    .select(dsl::count_star())
                    .left_join(
                        application::table
                            .on(api_key::dsl::tenant_id.eq(application::dsl::tenant_id.nullable())),
                    )
                    .left_join(
                        multicast_group::table
                            .on(application::dsl::id.eq(multicast_group::dsl::application_id)),
                    )
                    .filter(api_key::dsl::id.eq(&id))
                    .into_boxed();

                match flag {
                    // admin api key
                    // tenant api key
                    Flag::Create | Flag::List | Flag::Delete => {
                        q = q.filter(
                            api_key::dsl::is_admin
                                .eq(true)
                                .or(multicast_group::dsl::id.eq(&multicast_group_id)),
                        );
                    }
                    _ => {
                        return Ok(0);
                    }
                }

                Ok(q.first(&mut c)?)
            }
        })
        .await?
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage::{
        api_key, application, device, device_profile, gateway, multicast, tenant, user,
    };
    use crate::test;
    use std::str::FromStr;

    struct ValidatorTest<V>
    where
        V: Validator + Sync,
    {
        validators: Vec<V>,
        id: AuthID,
        ok: bool,
    }

    async fn run_tests<V>(tests: Vec<ValidatorTest<V>>)
    where
        V: Validator + Sync,
    {
        for tst in tests {
            for v in tst.validators {
                assert_eq!(tst.ok, v.validate(&tst.id).await.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn validate_is_admin() {
        let _guard = test::prepare().await;
        let users = vec![
            user::User {
                email: "admin@user".into(),
                is_active: true,
                is_admin: true,
                ..Default::default()
            },
            user::User {
                email: "inactive@user".into(),
                is_active: false,
                is_admin: false,
                ..Default::default()
            },
            user::User {
                email: "normal@user".into(),
                is_active: true,
                is_admin: false,
                ..Default::default()
            },
        ];
        for u in &users {
            user::create(u.clone()).await.unwrap();
        }

        let tests = vec![
            // admin user
            ValidatorTest {
                validators: vec![ValidateIsAdmin::new()],
                id: AuthID::User(users[0].id),
                ok: true,
            },
            // inactive
            ValidatorTest {
                validators: vec![ValidateIsAdmin::new()],
                id: AuthID::User(users[1].id),
                ok: false,
            },
            // active regular user
            ValidatorTest {
                validators: vec![ValidateIsAdmin::new()],
                id: AuthID::User(users[2].id),
                ok: false,
            },
        ];

        run_tests(tests).await;
    }

    #[tokio::test]
    async fn validate_active_user() {
        let _guard = test::prepare().await;
        let users = vec![
            user::User {
                email: "active@user".into(),
                is_active: true,
                is_admin: false,
                ..Default::default()
            },
            user::User {
                email: "inactive@user".into(),
                is_active: false,
                is_admin: false,
                ..Default::default()
            },
        ];
        for u in &users {
            user::create(u.clone()).await.unwrap();
        }

        let api_key = api_key::test::create_api_key(true, false).await;

        let tests = vec![
            // active user
            ValidatorTest {
                validators: vec![ValidateActiveUser::new()],
                id: AuthID::User(users[0].id),
                ok: true,
            },
            // inactive user
            ValidatorTest {
                validators: vec![ValidateActiveUser::new()],
                id: AuthID::User(users[1].id),
                ok: false,
            },
            // api key
            ValidatorTest {
                validators: vec![ValidateActiveUser::new()],
                id: AuthID::Key(api_key.id),
                ok: false,
            },
        ];

        run_tests(tests).await;
    }

    #[tokio::test]
    async fn validate_active_user_or_key() {
        let _guard = test::prepare().await;

        let users = vec![
            user::User {
                email: "active@user".into(),
                is_active: true,
                is_admin: false,
                ..Default::default()
            },
            user::User {
                email: "inactive@user".into(),
                is_active: false,
                is_admin: false,
                ..Default::default()
            },
        ];
        for u in &users {
            user::create(u.clone()).await.unwrap();
        }

        let api_key = api_key::test::create_api_key(false, true).await;

        let tests = vec![
            // active user
            ValidatorTest {
                validators: vec![ValidateActiveUserOrKey::new()],
                id: AuthID::User(users[0].id),
                ok: true,
            },
            // inactive user
            ValidatorTest {
                validators: vec![ValidateActiveUserOrKey::new()],
                id: AuthID::User(users[1].id),
                ok: false,
            },
            // api key
            ValidatorTest {
                validators: vec![ValidateActiveUserOrKey::new()],
                id: AuthID::Key(api_key.id),
                ok: true,
            },
            // non-existing key
            ValidatorTest {
                validators: vec![ValidateActiveUserOrKey::new()],
                id: AuthID::Key(Uuid::new_v4()),
                ok: false,
            },
        ];

        run_tests(tests).await;
    }

    #[tokio::test]
    async fn validate_tenant() {
        let _guard = test::prepare().await;
        let user = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_inactive = user::User {
            email: "inactive@user".into(),
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };

        for u in vec![
            &user,
            &tenant_user,
            &tenant_admin,
            &user_inactive,
            &user_admin,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let tenant_a = tenant::test::create_tenant().await;

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;

        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();

        // tenants with user id
        let tests = vec![
            // global admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateTenantsAccess::new(Flag::Create),
                    ValidateTenantsAccess::new(Flag::List),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateTenantsAccess::new(Flag::List)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // normal user can list
            ValidatorTest {
                validators: vec![ValidateTenantsAccess::new(Flag::List)],
                id: AuthID::User(user.id),
                ok: true,
            },
            // tenant user can not create
            ValidatorTest {
                validators: vec![ValidateTenantsAccess::new(Flag::Create)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // normal user can not create
            ValidatorTest {
                validators: vec![ValidateTenantsAccess::new(Flag::Create)],
                id: AuthID::User(user.id),
                ok: false,
            },
            // inactive user can not list
            ValidatorTest {
                validators: vec![ValidateTenantsAccess::new(Flag::Create)],
                id: AuthID::User(user_inactive.id),
                ok: false,
            },
        ];

        run_tests(tests).await;

        // tenants with api key
        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateTenantsAccess::new(Flag::Create),
                    ValidateTenantsAccess::new(Flag::List),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api can not create or list
            ValidatorTest {
                validators: vec![
                    ValidateTenantsAccess::new(Flag::Create),
                    ValidateTenantsAccess::new(Flag::List),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];

        run_tests(tests).await;

        // tenant with user
        let tests = vec![
            // global admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantAccess::new(Flag::Read, tenant_a.id),
                    ValidateTenantAccess::new(Flag::Update, tenant_a.id),
                    ValidateTenantAccess::new(Flag::Delete, tenant_a.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can read
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Read, tenant_a.id)],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant user can read
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Read, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant admin can not update
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Update, tenant_a.id)],
                id: AuthID::User(tenant_admin.id),
                ok: false,
            },
            // tenant admin can not delete
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Delete, tenant_a.id)],
                id: AuthID::User(tenant_admin.id),
                ok: false,
            },
            // tenant user can not update
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Update, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // tenant user can not delete
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Delete, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // normal user can not read
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Read, tenant_a.id)],
                id: AuthID::User(user.id),
                ok: false,
            },
            // normal user can not update
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Update, tenant_a.id)],
                id: AuthID::User(user.id),
                ok: false,
            },
            // normal user can not delete
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(Flag::Delete, tenant_a.id)],
                id: AuthID::User(user.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // tenant with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantAccess::new(Flag::Read, tenant_a.id),
                    ValidateTenantAccess::new(Flag::Update, tenant_a.id),
                    ValidateTenantAccess::new(Flag::Delete, tenant_a.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(
                    Flag::Read,
                    api_key_tenant.tenant_id.unwrap(),
                )],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not update
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(
                    Flag::Update,
                    api_key_tenant.tenant_id.unwrap(),
                )],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
            // tenant api key can not delete
            ValidatorTest {
                validators: vec![ValidateTenantAccess::new(
                    Flag::Delete,
                    api_key_tenant.tenant_id.unwrap(),
                )],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn tenant_user() {
        let _guard = test::prepare().await;

        let user = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user_other = user::User {
            email: "tenant-user-other@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![
            &user,
            &user_admin,
            &tenant_admin,
            &tenant_user,
            &tenant_user_other,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let tenant_a = tenant::test::create_tenant().await;

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;

        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_user_other.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // tenant users with user id
        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateTenantUsersAccess::new(Flag::Create, tenant_a.id),
                    ValidateTenantUsersAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateTenantUsersAccess::new(Flag::Create, tenant_a.id),
                    ValidateTenantUsersAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateTenantUsersAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not create
            ValidatorTest {
                validators: vec![ValidateTenantUsersAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // normal user can not create
            ValidatorTest {
                validators: vec![ValidateTenantUsersAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(user.id),
                ok: false,
            },
            // normal user can not list
            ValidatorTest {
                validators: vec![ValidateTenantUsersAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(user.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // tenant users with api key
        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateTenantUsersAccess::new(Flag::Create, tenant_a.id),
                    ValidateTenantUsersAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateTenantUsersAccess::new(Flag::Create, api_key_tenant.tenant_id.unwrap()),
                    ValidateTenantUsersAccess::new(Flag::List, api_key_tenant.tenant_id.unwrap()),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key for different tenant can not create or list
            ValidatorTest {
                validators: vec![
                    ValidateTenantUsersAccess::new(Flag::Create, tenant_a.id),
                    ValidateTenantUsersAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // tenant user with user
        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(Flag::Read, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Update, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Delete, tenant_a.id, tenant_user.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(Flag::Read, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Update, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Delete, tenant_a.id, tenant_user.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant user can read own user
            ValidatorTest {
                validators: vec![ValidateTenantUserAccess::new(
                    Flag::Read,
                    tenant_a.id,
                    tenant_user.id,
                )],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not read other user
            ValidatorTest {
                validators: vec![ValidateTenantUserAccess::new(
                    Flag::Read,
                    tenant_a.id,
                    tenant_user_other.id,
                )],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // tenant user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(Flag::Update, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Delete, tenant_a.id, tenant_user.id),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // normal user can not read, update or delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(Flag::Read, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Update, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Delete, tenant_a.id, tenant_user.id),
                ],
                id: AuthID::User(user.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // tenant user with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(Flag::Read, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Update, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Delete, tenant_a.id, tenant_user.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(
                        Flag::Read,
                        api_key_tenant.tenant_id.unwrap(),
                        tenant_user.id,
                    ),
                    ValidateTenantUserAccess::new(
                        Flag::Update,
                        api_key_tenant.tenant_id.unwrap(),
                        tenant_user.id,
                    ),
                    ValidateTenantUserAccess::new(
                        Flag::Delete,
                        api_key_tenant.tenant_id.unwrap(),
                        tenant_user.id,
                    ),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not read, update or delete for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateTenantUserAccess::new(Flag::Read, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Update, tenant_a.id, tenant_user.id),
                    ValidateTenantUserAccess::new(Flag::Delete, tenant_a.id, tenant_user.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn application() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_device_admin = user::User {
            email: "tenant-device-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_gateway_admin = user::User {
            email: "tenant-gateway-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![
            &user_active,
            &user_admin,
            &tenant_admin,
            &tenant_device_admin,
            &tenant_gateway_admin,
            &tenant_user,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let tenant_a = tenant::test::create_tenant().await;

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;

        let app = application::test::create_application(Some(tenant_a.id)).await;
        let app_api_key_tenant =
            application::test::create_application(Some(api_key_tenant.tenant_id.unwrap())).await;

        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_device_admin.id,
            is_device_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_gateway_admin.id,
            is_gateway_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // applications with user
        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(Flag::Create, tenant_a.id),
                    ValidateApplicationsAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(Flag::Create, tenant_a.id),
                    ValidateApplicationsAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(Flag::Create, tenant_a.id),
                    ValidateApplicationsAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant gateway admin can list
            ValidatorTest {
                validators: vec![ValidateApplicationsAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateApplicationsAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant gateway admin can not create
            ValidatorTest {
                validators: vec![ValidateApplicationsAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: false,
            },
            // tenant user can not create
            ValidatorTest {
                validators: vec![ValidateApplicationsAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // normal user can not create or list
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(Flag::Create, tenant_a.id),
                    ValidateApplicationsAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // applications with api key
        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(Flag::Create, tenant_a.id),
                    ValidateApplicationsAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(
                        Flag::Create,
                        api_key_tenant.tenant_id.unwrap(),
                    ),
                    ValidateApplicationsAccess::new(Flag::List, api_key_tenant.tenant_id.unwrap()),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not create or list for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateApplicationsAccess::new(Flag::Create, tenant_a.id),
                    ValidateApplicationsAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // application with user
        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app.id),
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app.id),
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app.id),
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant user can read
            ValidatorTest {
                validators: vec![ValidateApplicationAccess::new(Flag::Read, app.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // user can not read, update or delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app.id),
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
            // tenant user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // application with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app.id),
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read update and delete
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app_api_key_tenant.id),
                    ValidateApplicationAccess::new(Flag::Update, app_api_key_tenant.id),
                    ValidateApplicationAccess::new(Flag::Delete, app_api_key_tenant.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not read, update or delete app from other tentant
            ValidatorTest {
                validators: vec![
                    ValidateApplicationAccess::new(Flag::Read, app.id),
                    ValidateApplicationAccess::new(Flag::Update, app.id),
                    ValidateApplicationAccess::new(Flag::Delete, app.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn device_profile_test() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };

        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };

        for u in vec![&user_active, &user_admin] {
            user::create(u.clone()).await.unwrap();
        }

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;

        // device-profile templates with user
        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileTemplatesAccess::new(Flag::Create),
                    ValidateDeviceProfileTemplatesAccess::new(Flag::List),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // user can list
            ValidatorTest {
                validators: vec![ValidateDeviceProfileTemplatesAccess::new(Flag::List)],
                id: AuthID::User(user_active.id),
                ok: true,
            },
            // user can not create
            ValidatorTest {
                validators: vec![ValidateDeviceProfileTemplatesAccess::new(Flag::Create)],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // device-profile templates with api key
        let tests = vec![
            // admin api can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileTemplatesAccess::new(Flag::Create),
                    ValidateDeviceProfileTemplatesAccess::new(Flag::List),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can list
            ValidatorTest {
                validators: vec![ValidateDeviceProfileTemplatesAccess::new(Flag::List)],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api can not create
            ValidatorTest {
                validators: vec![ValidateDeviceProfileTemplatesAccess::new(Flag::Create)],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // device-profile template with user
        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileTemplateAccess::new(Flag::Read),
                    ValidateDeviceProfileTemplateAccess::new(Flag::Update),
                    ValidateDeviceProfileTemplateAccess::new(Flag::Delete),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // user can read
            ValidatorTest {
                validators: vec![ValidateDeviceProfileTemplateAccess::new(Flag::Read)],
                id: AuthID::User(user_active.id),
                ok: true,
            },
            // user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileTemplateAccess::new(Flag::Update),
                    ValidateDeviceProfileTemplateAccess::new(Flag::Delete),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // device-profile template with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileTemplateAccess::new(Flag::Read),
                    ValidateDeviceProfileTemplateAccess::new(Flag::Update),
                    ValidateDeviceProfileTemplateAccess::new(Flag::Delete),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read
            ValidatorTest {
                validators: vec![ValidateDeviceProfileTemplateAccess::new(Flag::Read)],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileTemplateAccess::new(Flag::Update),
                    ValidateDeviceProfileTemplateAccess::new(Flag::Delete),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn device_profile() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_device_admin = user::User {
            email: "tenant-device-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_gateway_admin = user::User {
            email: "tenant-gateway-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![
            &user_active,
            &user_admin,
            &tenant_admin,
            &tenant_gateway_admin,
            &tenant_device_admin,
            &tenant_user,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let tenant_a = tenant::test::create_tenant().await;

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;

        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: tenant_a.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();
        let dp_api_key_tenant = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp-tenant".into(),
            tenant_id: api_key_tenant.tenant_id.unwrap().clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_device_admin.id,
            is_device_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_gateway_admin.id,
            is_gateway_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // device profiles with user
        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id),
                    ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id),
                    ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id),
                    ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant gateway admin can list
            ValidatorTest {
                validators: vec![ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: true,
            },
            // tenant users can list
            ValidatorTest {
                validators: vec![ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant users can not create
            ValidatorTest {
                validators: vec![ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // tenant gateway admin can not create
            ValidatorTest {
                validators: vec![ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: false,
            },
            // non-tenant users can not list or create
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id),
                    ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // device profiles with api key
        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id),
                    ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(
                        Flag::Create,
                        api_key_tenant.tenant_id.unwrap(),
                    ),
                    ValidateDeviceProfilesAccess::new(
                        Flag::List,
                        api_key_tenant.tenant_id.unwrap(),
                    ),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not create or list for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfilesAccess::new(Flag::Create, tenant_a.id),
                    ValidateDeviceProfilesAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // device profile with user
        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Read, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Read, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Read, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant gateway admin can read
            ValidatorTest {
                validators: vec![ValidateDeviceProfileAccess::new(Flag::Read, dp.id)],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: true,
            },
            // tenant user can read
            ValidatorTest {
                validators: vec![ValidateDeviceProfileAccess::new(Flag::Read, dp.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant gateway admin can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: false,
            },
            // tenant user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // device profile with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Read, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Read, dp_api_key_tenant.id),
                    ValidateDeviceProfileAccess::new(Flag::Update, dp_api_key_tenant.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp_api_key_tenant.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not read, update or delete for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateDeviceProfileAccess::new(Flag::Read, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Update, dp.id),
                    ValidateDeviceProfileAccess::new(Flag::Delete, dp.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn device() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_device_admin = user::User {
            email: "tenant-device-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_gateway_admin = user::User {
            email: "tenant-gateway-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![
            &user_active,
            &user_admin,
            &tenant_admin,
            &tenant_gateway_admin,
            &tenant_device_admin,
            &tenant_user,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;
        let api_key_other_tenant = api_key::test::create_api_key(false, true).await;

        let app =
            application::test::create_application(Some(api_key_tenant.tenant_id.unwrap())).await;

        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_device_admin.id,
            is_device_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_gateway_admin.id,
            is_gateway_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateDevicesAccess::new(Flag::List, app.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not create
            ValidatorTest {
                validators: vec![ValidateDevicesAccess::new(Flag::Create, app.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // other users can not create or list
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not create or list for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateDevicesAccess::new(Flag::Create, app.id),
                    ValidateDevicesAccess::new(Flag::List, app.id),
                ],
                id: AuthID::Key(api_key_other_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        let dp =
            device_profile::test::create_device_profile(Some(api_key_tenant.tenant_id.unwrap()))
                .await;
        let dev = device::test::create_device(
            EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            Some(app.id),
        )
        .await;

        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant user can read
            ValidatorTest {
                validators: vec![ValidateDeviceAccess::new(Flag::Read, dev.dev_eui)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // other user can not read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // other api key can not read, update or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceAccess::new(Flag::Read, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Update, dev.dev_eui),
                    ValidateDeviceAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::Key(api_key_other_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn device_queue() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![&user_active, &user_admin, &tenant_user] {
            user::create(u.clone()).await.unwrap();
        }

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;
        let api_key_other_tenant = api_key::test::create_api_key(false, true).await;

        let app =
            application::test::create_application(Some(api_key_tenant.tenant_id.unwrap())).await;

        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp =
            device_profile::test::create_device_profile(Some(api_key_tenant.tenant_id.unwrap()))
                .await;
        let dev = device::test::create_device(
            EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            Some(app.id),
        )
        .await;

        let tests = vec![
            // admin user can create list and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceQueueAccess::new(Flag::Create, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::List, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant user can create list and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceQueueAccess::new(Flag::Create, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::List, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // other user can not create, list or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceQueueAccess::new(Flag::Create, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::List, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        let tests = vec![
            // admin api key can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceQueueAccess::new(Flag::Create, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::List, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceQueueAccess::new(Flag::Create, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::List, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // api key for other tenant cna not create, list or delete
            ValidatorTest {
                validators: vec![
                    ValidateDeviceQueueAccess::new(Flag::Create, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::List, dev.dev_eui),
                    ValidateDeviceQueueAccess::new(Flag::Delete, dev.dev_eui),
                ],
                id: AuthID::Key(api_key_other_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn gateway() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_gateway_admin = user::User {
            email: "tenant-gateway-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![
            &user_active,
            &user_admin,
            &tenant_admin,
            &tenant_gateway_admin,
            &tenant_user,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let tenant_a = tenant::test::create_tenant().await;

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;

        let gw = gateway::create(gateway::Gateway {
            name: "test-gw".into(),
            gateway_id: EUI64::from_str("0102030405060708").unwrap(),
            tenant_id: tenant_a.id,
            ..Default::default()
        })
        .await
        .unwrap();
        let gw_api_key_tenant = gateway::create(gateway::Gateway {
            name: "test-gw-tenant".into(),
            gateway_id: EUI64::from_str("0202030405060708").unwrap(),
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            ..Default::default()
        })
        .await
        .unwrap();

        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_gateway_admin.id,
            is_gateway_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: tenant_a.id,
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // gateways with user
        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, tenant_a.id),
                    ValidateGatewaysAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, tenant_a.id),
                    ValidateGatewaysAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant gateway admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, tenant_a.id),
                    ValidateGatewaysAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateGatewaysAccess::new(Flag::List, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not create
            ValidatorTest {
                validators: vec![ValidateGatewaysAccess::new(Flag::Create, tenant_a.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // other users can not create or list
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, tenant_a.id),
                    ValidateGatewaysAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // gateways with api key
        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, tenant_a.id),
                    ValidateGatewaysAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, api_key_tenant.tenant_id.unwrap()),
                    ValidateGatewaysAccess::new(Flag::List, api_key_tenant.tenant_id.unwrap()),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not create or list for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateGatewaysAccess::new(Flag::Create, tenant_a.id),
                    ValidateGatewaysAccess::new(Flag::List, tenant_a.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // gateway with user
        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant gateway admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::User(tenant_gateway_admin.id),
                ok: true,
            },
            // tenant user can read
            ValidatorTest {
                validators: vec![ValidateGatewayAccess::new(Flag::Read, gw.gateway_id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // other user can not read, update or delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // gateway with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw_api_key_tenant.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw_api_key_tenant.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw_api_key_tenant.gateway_id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not read, update or delete gw from other tenant
            ValidatorTest {
                validators: vec![
                    ValidateGatewayAccess::new(Flag::Read, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Update, gw.gateway_id),
                    ValidateGatewayAccess::new(Flag::Delete, gw.gateway_id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }

    #[tokio::test]
    async fn multicast_group() {
        let _guard = test::prepare().await;

        let user_active = user::User {
            email: "user@user".into(),
            is_active: true,
            ..Default::default()
        };
        let user_admin = user::User {
            email: "admin@user".into(),
            is_active: true,
            is_admin: true,
            ..Default::default()
        };
        let tenant_admin = user::User {
            email: "tenant-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_device_admin = user::User {
            email: "tenant-device-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_gateway_admin = user::User {
            email: "tenant-gateway-admin@user".into(),
            is_active: true,
            ..Default::default()
        };
        let tenant_user = user::User {
            email: "tenant-user@user".into(),
            is_active: true,
            ..Default::default()
        };

        for u in vec![
            &user_active,
            &user_admin,
            &tenant_admin,
            &tenant_gateway_admin,
            &tenant_device_admin,
            &tenant_user,
        ] {
            user::create(u.clone()).await.unwrap();
        }

        let api_key_admin = api_key::test::create_api_key(true, false).await;
        let api_key_tenant = api_key::test::create_api_key(false, true).await;
        let api_key_other_tenant = api_key::test::create_api_key(false, true).await;

        let app =
            application::test::create_application(Some(api_key_tenant.tenant_id.unwrap())).await;

        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_admin.id,
            is_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_device_admin.id,
            is_device_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_gateway_admin.id,
            is_gateway_admin: true,
            ..Default::default()
        })
        .await
        .unwrap();
        tenant::add_user(tenant::TenantUser {
            tenant_id: api_key_tenant.tenant_id.unwrap(),
            user_id: tenant_user.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // multicast-groups with user
        let tests = vec![
            // admin user can create and list
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can create and list
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateMulticastGroupsAccess::new(Flag::List, app.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not create
            ValidatorTest {
                validators: vec![ValidateMulticastGroupsAccess::new(Flag::Create, app.id)],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // other user can not create or list
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // multicast-groups with api key
        let tests = vec![
            // admin api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create and list
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // tenant api key can not create or list for other tenant
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupsAccess::new(Flag::Create, app.id),
                    ValidateMulticastGroupsAccess::new(Flag::List, app.id),
                ],
                id: AuthID::Key(api_key_other_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        let mg = multicast::create(multicast::MulticastGroup {
            name: "test-mg".into(),
            application_id: app.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // multicast-group with user
        let tests = vec![
            // admin user can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant user can read
            ValidatorTest {
                validators: vec![ValidateMulticastGroupAccess::new(Flag::Read, mg.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not update or delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // other user can not read, update or delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // multicast-group with api key
        let tests = vec![
            // admin api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can read, update and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // other api key can not read, update or delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupAccess::new(Flag::Read, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Update, mg.id),
                    ValidateMulticastGroupAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::Key(api_key_other_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // multicast-group queue with user
        let tests = vec![
            // admin user can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(user_admin.id),
                ok: true,
            },
            // tenant admin can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(tenant_admin.id),
                ok: true,
            },
            // tenant device admin can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(tenant_device_admin.id),
                ok: true,
            },
            // tenant user can list
            ValidatorTest {
                validators: vec![ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id)],
                id: AuthID::User(tenant_user.id),
                ok: true,
            },
            // tenant user can not create or delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(tenant_user.id),
                ok: false,
            },
            // uther user can not create, list or delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::User(user_active.id),
                ok: false,
            },
        ];
        run_tests(tests).await;

        // multicast-group queue with api key
        let tests = vec![
            // admin api key can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::Key(api_key_admin.id),
                ok: true,
            },
            // tenant api key can create, list and delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::Key(api_key_tenant.id),
                ok: true,
            },
            // other api key can not create, list or delete
            ValidatorTest {
                validators: vec![
                    ValidateMulticastGroupQueueAccess::new(Flag::Create, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::List, mg.id),
                    ValidateMulticastGroupQueueAccess::new(Flag::Delete, mg.id),
                ],
                id: AuthID::Key(api_key_other_tenant.id),
                ok: false,
            },
        ];
        run_tests(tests).await;
    }
}
