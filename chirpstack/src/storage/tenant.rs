use std::collections::HashMap;
use std::ops::Deref;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{dsl, prelude::*};
use diesel_async::{AsyncConnection, RunQueryDsl};
use tracing::info;
use uuid::Uuid;

use super::error::Error;
use super::schema::{
    application, device, device_profile, tenant, tenant_user, tenant_user_application,
    tenant_user_device_profile, user,
};
use super::{fields, get_async_db_conn};
use crate::{config, storage};
use lrwn::EUI64;

#[derive(Queryable, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = tenant)]
pub struct Tenant {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub can_have_gateways: bool,
    pub max_device_count: i32,
    pub max_gateway_count: i32,
    pub private_gateways_up: bool,
    pub private_gateways_down: bool,
    pub tags: fields::KeyValue,
    pub dev_addr_prefixes: fields::DevAddrPrefixVec,
}

impl Tenant {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }

        let nw_prefixes = config::get().network.get_dev_addr_prefixes();
        for prefix in self.dev_addr_prefixes.deref().iter().flatten() {
            let mut is_valid = false;

            for nw_prefix in &nw_prefixes {
                if prefix.is_subset_of(nw_prefix) {
                    is_valid = true;
                }
            }

            if !is_valid {
                return Err(Error::Validation(format!(
                    "DevAddr prefix {} is not a subset of configured network DevAddr space",
                    prefix
                )));
            }
        }

        Ok(())
    }

    pub fn get_dev_addr_prefixes(&self) -> Vec<lrwn::DevAddrPrefix> {
        let prefixes: Vec<lrwn::DevAddrPrefix> = (*self.dev_addr_prefixes)
            .iter()
            .cloned()
            .flatten()
            .collect();
        if prefixes.is_empty() {
            config::get().network.get_dev_addr_prefixes()
        } else {
            prefixes
        }
    }
}

impl Default for Tenant {
    fn default() -> Self {
        let now = Utc::now();

        Tenant {
            id: Uuid::new_v4().into(),
            created_at: now,
            updated_at: now,
            name: "".into(),
            description: "".into(),
            can_have_gateways: false,
            max_device_count: 0,
            max_gateway_count: 0,
            private_gateways_up: false,
            private_gateways_down: false,
            tags: fields::KeyValue::new(HashMap::new()),
            dev_addr_prefixes: fields::DevAddrPrefixVec::new(vec![]),
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset, PartialEq, Eq, Debug)]
#[diesel(table_name = tenant_user)]
pub struct TenantUser {
    pub tenant_id: fields::Uuid,
    pub user_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_admin: bool,
    pub is_device_admin: bool,
    pub is_gateway_admin: bool,
}

impl Default for TenantUser {
    fn default() -> Self {
        let now = Utc::now();

        TenantUser {
            tenant_id: Uuid::nil().into(),
            user_id: Uuid::nil().into(),
            created_at: now,
            updated_at: now,
            is_admin: false,
            is_device_admin: false,
            is_gateway_admin: false,
        }
    }
}

#[derive(Queryable, Insertable, PartialEq, Eq, Debug)]
#[diesel(table_name = tenant_user_device_profile)]
pub struct TenantUserDeviceProfile {
    pub user_id: fields::Uuid,
    pub device_profile_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
}

impl Default for TenantUserDeviceProfile {
    fn default() -> Self {
        TenantUserDeviceProfile {
            user_id: Uuid::nil().into(),
            device_profile_id: Uuid::nil().into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Queryable, Insertable, PartialEq, Eq, Debug)]
#[diesel(table_name = tenant_user_application)]
pub struct TenantUserApplication {
    pub user_id: fields::Uuid,
    pub application_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub is_read_only: bool,
}

impl Default for TenantUserApplication {
    fn default() -> Self {
        TenantUserApplication {
            user_id: Uuid::nil().into(),
            application_id: Uuid::nil().into(),
            created_at: Utc::now(),
            is_read_only: false,
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct TenantUserListItem {
    pub tenant_id: fields::Uuid,
    pub user_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub is_admin: bool,
    pub is_device_admin: bool,
    pub is_gateway_admin: bool,
}

#[derive(Default, Clone)]
pub struct Filters {
    pub user_id: Option<Uuid>,
    pub search: Option<String>,
}

pub async fn create(t: Tenant) -> Result<Tenant, Error> {
    t.validate()?;

    let t: Tenant = diesel::insert_into(tenant::table)
        .values(&t)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, t.id.to_string()))?;
    info!(id = %t.id, "Tenant created");
    Ok(t)
}

pub async fn get(id: &Uuid) -> Result<Tenant, Error> {
    let t = tenant::dsl::tenant
        .find(&fields::Uuid::from(id))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, id.to_string()))?;
    Ok(t)
}

pub async fn get_for_dev_eui(dev_eui: EUI64) -> Result<Tenant, Error> {
    let t = tenant::dsl::tenant
        .inner_join(application::table.on(application::tenant_id.eq(tenant::id)))
        .inner_join(device::table.on(device::application_id.eq(application::id)))
        .select(tenant::all_columns)
        .filter(device::dev_eui.eq(dev_eui))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;

    Ok(t)
}

pub async fn get_for_application_id(app_id: Uuid) -> Result<Tenant, Error> {
    let t = tenant::dsl::tenant
        .inner_join(application::table.on(application::tenant_id.eq(tenant::id)))
        .select(tenant::all_columns)
        .filter(application::id.eq(fields::Uuid::from(app_id)))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, app_id.to_string()))?;

    Ok(t)
}

pub async fn update(t: Tenant) -> Result<Tenant, Error> {
    t.validate()?;

    let t: Tenant = diesel::update(tenant::dsl::tenant.find(&t.id))
        .set((
            tenant::updated_at.eq(Utc::now()),
            tenant::name.eq(&t.name),
            tenant::description.eq(&t.description),
            tenant::can_have_gateways.eq(&t.can_have_gateways),
            tenant::max_device_count.eq(&t.max_device_count),
            tenant::max_gateway_count.eq(&t.max_gateway_count),
            tenant::private_gateways_up.eq(&t.private_gateways_up),
            tenant::private_gateways_down.eq(&t.private_gateways_down),
            tenant::tags.eq(&t.tags),
            tenant::dev_addr_prefixes.eq(&t.dev_addr_prefixes),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, t.id.to_string()))?;
    info!(id = %t.id, "Tenant updated");
    Ok(t)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    let ra = diesel::delete(tenant::dsl::tenant.find(&fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, id.to_string()))?;

    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }
    info!(id = %id, "Tenant deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    let mut q = tenant::dsl::tenant
        .left_join(tenant_user::table)
        .into_boxed();

    if let Some(user_id) = &filters.user_id {
        q = q.filter(tenant_user::dsl::user_id.eq(fields::Uuid::from(user_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(tenant::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(tenant::dsl::name.like(format!("%{}%", search)));
        }
    }

    Ok(
        q.select(dsl::sql::<diesel::sql_types::BigInt>("count(distinct id)"))
            .first(&mut get_async_db_conn().await?)
            .await?,
    )
}

pub async fn list(limit: i64, offset: i64, filters: &Filters) -> Result<Vec<Tenant>, Error> {
    let mut q = tenant::dsl::tenant
        .left_join(tenant_user::table)
        .select(tenant::all_columns)
        .group_by(tenant::dsl::id)
        .order_by(tenant::dsl::name)
        .limit(limit)
        .offset(offset)
        .into_boxed();

    if let Some(user_id) = &filters.user_id {
        q = q.filter(tenant_user::dsl::user_id.eq(fields::Uuid::from(user_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(tenant::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(tenant::dsl::name.like(format!("%{}%", search)));
        }
    }

    let items = q.load(&mut get_async_db_conn().await?).await?;
    Ok(items)
}

pub async fn list_by_dev_addr_prefix_overlap(
    dev_addr_prefix: lrwn::DevAddrPrefix,
) -> Result<Vec<Tenant>, Error> {
    let tenants: Vec<Tenant> = tenant::table
        .order_by(tenant::name)
        .load(&mut get_async_db_conn().await?)
        .await?;

    Ok(tenants
        .into_iter()
        .filter(|t| {
            let prefixes: Vec<lrwn::DevAddrPrefix> =
                (*t.dev_addr_prefixes).iter().cloned().flatten().collect();

            for p in prefixes {
                if dev_addr_prefix.is_subset_of(&p) || p.is_subset_of(&dev_addr_prefix) {
                    return true;
                }
            }

            false
        })
        .collect())
}

pub async fn add_user(
    tu: TenantUser,
    device_profiles: &[TenantUserDeviceProfile],
    applications: &[TenantUserApplication],
) -> Result<TenantUser, Error> {
    let mut c = get_async_db_conn().await?;
    let tu: TenantUser = c
        .transaction::<TenantUser, Error, _>(async |c| {
            let tu: TenantUser = diesel::insert_into(tenant_user::table)
                .values(&tu)
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;

            for dp in device_profiles {
                // make sure dp exists under same tenant
                let _: storage::device_profile::DeviceProfile = device_profile::table
                    .filter(
                        device_profile::id
                            .eq(dp.device_profile_id)
                            .and(device_profile::tenant_id.eq(&tu.tenant_id)),
                    )
                    .first(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, dp.device_profile_id.to_string()))?;

                diesel::insert_into(tenant_user_device_profile::table)
                    .values(dp)
                    .execute(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;
            }

            for app in applications {
                // make sure dp exists under same tenant
                let _: storage::application::Application = application::table
                    .filter(
                        application::id
                            .eq(app.application_id)
                            .and(application::tenant_id.eq(&tu.tenant_id)),
                    )
                    .first(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, app.application_id.to_string()))?;

                diesel::insert_into(tenant_user_application::table)
                    .values(app)
                    .execute(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;
            }

            Ok(tu)
        })
        .await?;

    info!(
        tenant_id = %tu.tenant_id,
        user_id = %tu.user_id,
        "Tenant user added"
    );

    Ok(tu)
}

pub async fn update_user(
    tu: TenantUser,
    device_profiles: &[TenantUserDeviceProfile],
    applications: &[TenantUserApplication],
) -> Result<TenantUser, Error> {
    let mut c = get_async_db_conn().await?;
    let tu: TenantUser = c
        .transaction::<TenantUser, Error, _>(async |c| {
            let tu: TenantUser = diesel::update(
                tenant_user::dsl::tenant_user
                    .filter(tenant_user::dsl::tenant_id.eq(&tu.tenant_id))
                    .filter(tenant_user::dsl::user_id.eq(&tu.user_id)),
            )
            .set(&tu)
            .get_result(c)
            .await
            .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;

            // delete device-profile links
            diesel::delete(
                tenant_user_device_profile::table.filter(
                    tenant_user_device_profile::user_id.eq(&tu.user_id).and(
                        tenant_user_device_profile::device_profile_id.eq_any(
                            device_profile::table
                                .filter(device_profile::tenant_id.eq(&tu.tenant_id))
                                .select(device_profile::id),
                        ),
                    ),
                ),
            )
            .execute(c)
            .await
            .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;

            // delete application links
            diesel::delete(
                tenant_user_application::table.filter(
                    tenant_user_application::user_id.eq(&tu.user_id).and(
                        tenant_user_application::application_id.eq_any(
                            application::table
                                .filter(application::tenant_id.eq(&tu.tenant_id))
                                .select(application::id),
                        ),
                    ),
                ),
            )
            .execute(c)
            .await
            .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;

            // re-create device-profiles
            for dp in device_profiles {
                // make sure dp exists under same tenant
                let _: storage::device_profile::DeviceProfile = device_profile::table
                    .filter(
                        device_profile::id
                            .eq(dp.device_profile_id)
                            .and(device_profile::tenant_id.eq(&tu.tenant_id)),
                    )
                    .first(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, dp.device_profile_id.to_string()))?;

                diesel::insert_into(tenant_user_device_profile::table)
                    .values(dp)
                    .execute(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;
            }

            // re-create applications
            for app in applications {
                // make sure dp exists under same tenant
                let _: storage::application::Application = application::table
                    .filter(
                        application::id
                            .eq(app.application_id)
                            .and(application::tenant_id.eq(&tu.tenant_id)),
                    )
                    .first(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, app.application_id.to_string()))?;

                diesel::insert_into(tenant_user_application::table)
                    .values(app)
                    .execute(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))?;
            }

            Ok(tu)
        })
        .await?;

    info!(
        tenant_id = %tu.tenant_id,
        user_id = %tu.user_id,
        "Tenant user updated"
    );
    Ok(tu)
}

pub async fn get_user(
    tenant_id: &Uuid,
    user_id: &Uuid,
) -> Result<
    (
        TenantUser,
        Vec<TenantUserDeviceProfile>,
        Vec<TenantUserApplication>,
    ),
    Error,
> {
    let tu: TenantUser = tenant_user::table
        .filter(tenant_user::tenant_id.eq(&fields::Uuid::from(tenant_id)))
        .filter(tenant_user::user_id.eq(&fields::Uuid::from(user_id)))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, user_id.to_string()))?;

    let dps: Vec<TenantUserDeviceProfile> = tenant_user_device_profile::table
        .select(tenant_user_device_profile::all_columns)
        .filter(
            tenant_user_device_profile::user_id
                .eq(&fields::Uuid::from(user_id))
                .and(
                    tenant_user_device_profile::device_profile_id.eq_any(
                        device_profile::table
                            .filter(device_profile::tenant_id.eq(&fields::Uuid::from(tenant_id)))
                            .select(device_profile::id),
                    ),
                ),
        )
        .load(&mut get_async_db_conn().await?)
        .await?;

    let apps: Vec<TenantUserApplication> = tenant_user_application::table
        .select(tenant_user_application::all_columns)
        .filter(
            tenant_user_application::application_id.eq_any(
                application::table
                    .filter(application::tenant_id.eq(&fields::Uuid::from(tenant_id)))
                    .select(application::id),
            ),
        )
        .load(&mut get_async_db_conn().await?)
        .await?;

    Ok((tu, dps, apps))
}

pub async fn get_user_count(tenant_id: &Uuid) -> Result<i64, Error> {
    let count = tenant_user::table
        .select(dsl::count_star())
        .filter(tenant_user::tenant_id.eq(fields::Uuid::from(tenant_id)))
        .first(&mut get_async_db_conn().await?)
        .await?;
    Ok(count)
}

pub async fn get_users(
    tenant_id: &Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TenantUserListItem>, Error> {
    let items = tenant_user::dsl::tenant_user
        .inner_join(user::table)
        .select((
            tenant_user::dsl::tenant_id,
            tenant_user::dsl::user_id,
            tenant_user::dsl::created_at,
            tenant_user::dsl::updated_at,
            user::dsl::email,
            tenant_user::dsl::is_admin,
            tenant_user::dsl::is_device_admin,
            tenant_user::dsl::is_gateway_admin,
        ))
        .filter(tenant_user::dsl::tenant_id.eq(&fields::Uuid::from(tenant_id)))
        .order_by(user::dsl::email)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await?;

    Ok(items)
}

pub async fn delete_user(tenant_id: &Uuid, user_id: &Uuid) -> Result<(), Error> {
    // delete device-profile admin references
    diesel::delete(
        tenant_user_device_profile::table
            .filter(tenant_user_device_profile::user_id.eq(&fields::Uuid::from(user_id)))
            .filter(
                tenant_user_device_profile::device_profile_id.eq_any(
                    device_profile::table
                        .filter(device_profile::tenant_id.eq(&fields::Uuid::from(tenant_id)))
                        .select(device_profile::id),
                ),
            ),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;

    // delete application admin references
    diesel::delete(
        tenant_user_application::table
            .filter(tenant_user_application::user_id.eq(&fields::Uuid::from(user_id)))
            .filter(
                tenant_user_application::application_id.eq_any(
                    application::table
                        .filter(application::tenant_id.eq(&fields::Uuid::from(tenant_id)))
                        .select(application::id),
                ),
            ),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;

    let ra = diesel::delete(
        tenant_user::dsl::tenant_user
            .filter(tenant_user::dsl::tenant_id.eq(&fields::Uuid::from(tenant_id)))
            .filter(tenant_user::dsl::user_id.eq(&fields::Uuid::from(user_id))),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;
    if ra == 0 {
        return Err(Error::NotFound(user_id.to_string()));
    }
    info!(
        tenant_id = %tenant_id,
        user_id = %user_id,
        "Tenant user deleted"
    );
    Ok(())
}

pub async fn get_tenant_users_for_user(user_id: &Uuid) -> Result<Vec<TenantUser>, Error> {
    let items = tenant_user::dsl::tenant_user
        .filter(tenant_user::dsl::user_id.eq(&fields::Uuid::from(user_id)))
        .load(&mut get_async_db_conn().await?)
        .await?;
    Ok(items)
}

pub async fn get_tenant_user_device_profiles_for_user(
    user_id: Uuid,
) -> Result<Vec<TenantUserDeviceProfile>, Error> {
    let items = tenant_user_device_profile::table
        .filter(tenant_user_device_profile::user_id.eq(&fields::Uuid::from(user_id)))
        .load(&mut get_async_db_conn().await?)
        .await?;

    Ok(items)
}

pub async fn get_tenant_user_applications_for_user(
    user_id: Uuid,
) -> Result<Vec<TenantUserApplication>, Error> {
    let items = tenant_user_application::table
        .filter(tenant_user_application::user_id.eq(&fields::Uuid::from(user_id)))
        .load(&mut get_async_db_conn().await?)
        .await?;

    Ok(items)
}

#[cfg(test)]
pub mod test {
    use std::str::FromStr;

    use super::*;
    use crate::storage::fields::DevAddrPrefixVec;
    use crate::storage::user::test::create_user;
    use crate::test;
    use chrono::SubsecRound;
    use uuid::Uuid;

    struct FilterTest<'a> {
        filter: Filters,
        ts: Vec<&'a Tenant>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    pub async fn create_tenant() -> Tenant {
        let t = Tenant {
            id: Uuid::new_v4().into(),
            created_at: Utc::now().round_subsecs(1),
            updated_at: Utc::now().round_subsecs(1),
            name: "test t".into(),
            description: "test description".into(),
            can_have_gateways: true,
            max_device_count: 20,
            max_gateway_count: 10,
            private_gateways_up: true,
            private_gateways_down: true,
            tags: fields::KeyValue::new(HashMap::new()),
            dev_addr_prefixes: fields::DevAddrPrefixVec::new(vec![]),
        };
        create(t).await.unwrap()
    }

    #[tokio::test]
    async fn test_tenant() {
        let _guard = test::prepare().await;

        // delete existing tenants.
        let tenants = list(
            10,
            0,
            &Filters {
                ..Default::default()
            },
        )
        .await
        .unwrap();
        for t in &tenants {
            delete(&t.id).await.unwrap();
        }

        let mut t = create_tenant().await;

        // get
        let t_get = get(&t.id).await.unwrap();
        assert_eq!(t, t_get);

        // update
        t.name = "new t".into();
        t = update(t).await.unwrap();
        let t_get = get(&t.id).await.unwrap();
        assert_eq!(t, t_get);

        // add tenant user for filter by user_id test
        let user = create_user().await;

        let tu = TenantUser {
            tenant_id: t.id,
            user_id: user.id,
            is_admin: true,
            ..Default::default()
        };

        add_user(tu, &[], &[]).await.unwrap();

        // get_count and list
        let tests = vec![
            FilterTest {
                filter: Filters {
                    search: None,
                    user_id: None,
                },
                ts: vec![&t],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filter: Filters {
                    search: Some("bt".into()),
                    user_id: None,
                },
                ts: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filter: Filters {
                    search: Some("t".into()),
                    user_id: None,
                },
                ts: vec![&t],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filter: Filters {
                    search: Some("t".into()),
                    user_id: None,
                },
                ts: vec![],
                count: 1,
                limit: 0,
                offset: 0,
            },
            FilterTest {
                filter: Filters {
                    search: Some("t".into()),
                    user_id: None,
                },
                ts: vec![],
                count: 1,
                limit: 10,
                offset: 10,
            },
            FilterTest {
                filter: Filters {
                    user_id: Some(user.id.into()),
                    search: None,
                },
                ts: vec![&t],
                count: 1,
                limit: 10,
                offset: 0,
            },
        ];
        for tst in tests {
            let count = get_count(&tst.filter).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(tst.limit, tst.offset, &tst.filter).await.unwrap();
            assert_eq!(
                tst.ts
                    .iter()
                    .map(|t| { t.id.to_string() })
                    .collect::<String>(),
                items
                    .iter()
                    .map(|t| { t.id.to_string() })
                    .collect::<String>()
            );
        }

        // delete
        delete(&t.id).await.unwrap();
        assert!(delete(&t.id).await.is_err());
    }

    #[tokio::test]
    async fn test_tenant_list_by_dev_addr_prefix_overlap() {
        let _guard = test::prepare().await;

        create(Tenant {
            name: "tenant-1".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        assert_eq!(
            0,
            list_by_dev_addr_prefix_overlap(lrwn::DevAddrPrefix::from_str("00010000/16").unwrap())
                .await
                .unwrap()
                .len()
        );

        create(Tenant {
            name: "tenant-1".into(),
            dev_addr_prefixes: DevAddrPrefixVec::new(vec![Some(
                lrwn::DevAddrPrefix::from_str("00030000/16").unwrap(),
            )]),
            ..Default::default()
        })
        .await
        .unwrap();

        assert_eq!(
            1,
            list_by_dev_addr_prefix_overlap(lrwn::DevAddrPrefix::from_str("00030000/16").unwrap())
                .await
                .unwrap()
                .len()
        );

        assert_eq!(
            0,
            list_by_dev_addr_prefix_overlap(lrwn::DevAddrPrefix::from_str("00010000/16").unwrap())
                .await
                .unwrap()
                .len()
        );

        assert_eq!(
            1,
            list_by_dev_addr_prefix_overlap(lrwn::DevAddrPrefix::from_str("00020000/15").unwrap())
                .await
                .unwrap()
                .len()
        );
    }

    #[tokio::test]
    async fn test_tenant_user() {
        let _guard = test::prepare().await;

        let t = create_tenant().await;
        let user = create_user().await;

        let tu = TenantUser {
            tenant_id: t.id,
            user_id: user.id,
            is_admin: true,
            ..Default::default()
        };

        // add user
        let tu = add_user(tu, &[], &[]).await.unwrap();

        // get
        let (tu_get, _, _) = get_user(&t.id, &user.id).await.unwrap();
        assert_eq!(tu, tu_get);

        // get count and list
        let count = get_user_count(&t.id).await.unwrap();
        assert_eq!(1, count);

        // get users
        let users = get_users(&t.id, 10, 0).await.unwrap();
        assert_eq!(user.id, users[0].user_id);

        // delete
        delete_user(&t.id, &user.id).await.unwrap();
    }

    #[test]
    fn test_validate_name() {
        assert!(
            Tenant {
                ..Default::default()
            }
            .validate()
            .is_err()
        );
        assert!(
            Tenant {
                name: "test-tenant".into(),
                ..Default::default()
            }
            .validate()
            .is_ok()
        );
    }

    #[test]
    fn test_validate_dev_addr_prefix() {
        assert!(
            Tenant {
                name: "test-tenant".into(),
                dev_addr_prefixes: fields::DevAddrPrefixVec::new(vec![Some(
                    lrwn::DevAddrPrefix::from_str("00000000/7").unwrap()
                ),]),
                ..Default::default()
            }
            .validate()
            .is_ok()
        );

        assert!(
            Tenant {
                name: "test-tenant".into(),
                dev_addr_prefixes: fields::DevAddrPrefixVec::new(vec![Some(
                    lrwn::DevAddrPrefix::from_str("00000000/6").unwrap()
                ),]),
                ..Default::default()
            }
            .validate()
            .is_err(),
            "00000000/6 should not fit within default 00000000/7 range"
        );
    }

    #[test]
    fn test_get_dev_addr_prefixes() {
        assert_eq!(
            vec![lrwn::DevAddrPrefix::from_str("00000000/7").unwrap()],
            Tenant {
                ..Default::default()
            }
            .get_dev_addr_prefixes()
        );

        assert_eq!(
            vec![lrwn::DevAddrPrefix::from_str("00000000/8").unwrap()],
            Tenant {
                dev_addr_prefixes: DevAddrPrefixVec::new(vec![Some(
                    lrwn::DevAddrPrefix::from_str("00000000/8").unwrap()
                ),]),
                ..Default::default()
            }
            .get_dev_addr_prefixes()
        );
    }
}
