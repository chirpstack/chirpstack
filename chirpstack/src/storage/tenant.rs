use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use tokio::task;
use tracing::info;
use uuid::Uuid;

use super::error::Error;
use super::get_db_conn;
use super::schema::{tenant, tenant_user, user};

#[derive(Queryable, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = tenant)]
pub struct Tenant {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub can_have_gateways: bool,
    pub max_device_count: i32,
    pub max_gateway_count: i32,
    pub private_gateways_up: bool,
    pub private_gateways_down: bool,
}

impl Tenant {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }
        Ok(())
    }
}

impl Default for Tenant {
    fn default() -> Self {
        let now = Utc::now();

        Tenant {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            name: "".into(),
            description: "".into(),
            can_have_gateways: false,
            max_device_count: 0,
            max_gateway_count: 0,
            private_gateways_up: false,
            private_gateways_down: false,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset, PartialEq, Eq, Debug)]
#[diesel(table_name = tenant_user)]
pub struct TenantUser {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
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
            tenant_id: Uuid::nil(),
            user_id: Uuid::nil(),
            created_at: now,
            updated_at: now,
            is_admin: false,
            is_device_admin: false,
            is_gateway_admin: false,
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct TenantUserListItem {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
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
    let t = task::spawn_blocking({
        move || -> Result<Tenant, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(tenant::table)
                .values(&t)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, t.id.to_string()))
        }
    })
    .await??;
    info!(id = %t.id, "Tenant created");
    Ok(t)
}

pub async fn get(id: &Uuid) -> Result<Tenant, Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<Tenant, Error> {
            let mut c = get_db_conn()?;
            let t = tenant::dsl::tenant
                .find(&id)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;
            Ok(t)
        }
    })
    .await?
}

pub async fn update(t: Tenant) -> Result<Tenant, Error> {
    t.validate()?;
    let t = task::spawn_blocking({
        move || -> Result<Tenant, Error> {
            let mut c = get_db_conn()?;
            diesel::update(tenant::dsl::tenant.find(&t.id))
                .set((
                    tenant::updated_at.eq(Utc::now()),
                    tenant::name.eq(&t.name),
                    tenant::description.eq(&t.description),
                    tenant::can_have_gateways.eq(&t.can_have_gateways),
                    tenant::max_device_count.eq(&t.max_device_count),
                    tenant::max_gateway_count.eq(&t.max_gateway_count),
                    tenant::private_gateways_up.eq(&t.private_gateways_up),
                    tenant::private_gateways_down.eq(&t.private_gateways_down),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, t.id.to_string()))
        }
    })
    .await??;
    info!(id = %t.id, "Tenant updated");
    Ok(t)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(tenant::dsl::tenant.find(&id))
                .execute(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;

            if ra == 0 {
                return Err(Error::NotFound(id.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(id = %id, "Tenant deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = tenant::dsl::tenant
                .left_join(tenant_user::table)
                .into_boxed();

            if let Some(user_id) = &filters.user_id {
                q = q.filter(tenant_user::dsl::user_id.eq(user_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(tenant::dsl::name.ilike(format!("%{}%", search)));
            }

            Ok(
                q.select(dsl::sql::<diesel::sql_types::BigInt>("count(distinct id)"))
                    .first(&mut c)?,
            )
        }
    })
    .await?
}

pub async fn list(limit: i64, offset: i64, filters: &Filters) -> Result<Vec<Tenant>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<Tenant>, Error> {
            let mut c = get_db_conn()?;
            let mut q = tenant::dsl::tenant
                .left_join(tenant_user::table)
                .select(tenant::all_columns)
                .group_by(tenant::dsl::id)
                .order_by(tenant::dsl::name)
                .limit(limit)
                .offset(offset)
                .into_boxed();

            if let Some(user_id) = &filters.user_id {
                q = q.filter(tenant_user::dsl::user_id.eq(user_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(tenant::dsl::name.ilike(format!("%{}%", search)));
            }

            let items = q.load(&mut c)?;

            Ok(items)
        }
    })
    .await?
}

pub async fn add_user(tu: TenantUser) -> Result<TenantUser, Error> {
    let tu = task::spawn_blocking({
        move || -> Result<TenantUser, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(tenant_user::table)
                .values(&tu)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))
        }
    })
    .await??;
    info!(
        tenant_id = %tu.tenant_id,
        user_id = %tu.user_id,
        "Tenant user added"
    );
    Ok(tu)
}

pub async fn update_user(tu: TenantUser) -> Result<TenantUser, Error> {
    let tu = task::spawn_blocking({
        move || -> Result<TenantUser, Error> {
            let mut c = get_db_conn()?;
            diesel::update(
                tenant_user::dsl::tenant_user
                    .filter(tenant_user::dsl::tenant_id.eq(&tu.tenant_id))
                    .filter(tenant_user::dsl::user_id.eq(&tu.user_id)),
            )
            .set(&tu)
            .get_result(&mut c)
            .map_err(|e| Error::from_diesel(e, tu.user_id.to_string()))
        }
    })
    .await??;
    info!(
        tenant_id = %tu.tenant_id,
        user_id = %tu.user_id,
        "Tenant user updated"
    );
    Ok(tu)
}

pub async fn get_user(tenant_id: &Uuid, user_id: &Uuid) -> Result<TenantUser, Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        let user_id = *user_id;
        move || -> Result<TenantUser, Error> {
            let mut c = get_db_conn()?;
            let tu: TenantUser = tenant_user::dsl::tenant_user
                .filter(tenant_user::dsl::tenant_id.eq(&tenant_id))
                .filter(tenant_user::dsl::user_id.eq(&user_id))
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, user_id.to_string()))?;
            Ok(tu)
        }
    })
    .await?
}

pub async fn get_user_count(tenant_id: &Uuid) -> Result<i64, Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let count = tenant_user::dsl::tenant_user
                .select(dsl::count_star())
                .filter(tenant_user::dsl::tenant_id.eq(&tenant_id))
                .first(&mut c)?;
            Ok(count)
        }
    })
    .await?
}

pub async fn get_users(
    tenant_id: &Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TenantUserListItem>, Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        move || -> Result<Vec<TenantUserListItem>, Error> {
            let mut c = get_db_conn()?;
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
                .filter(tenant_user::dsl::tenant_id.eq(&tenant_id))
                .order_by(user::dsl::email)
                .limit(limit)
                .offset(offset)
                .load(&mut c)?;

            Ok(items)
        }
    })
    .await?
}

pub async fn delete_user(tenant_id: &Uuid, user_id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        let user_id = *user_id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(
                tenant_user::dsl::tenant_user
                    .filter(tenant_user::dsl::tenant_id.eq(&tenant_id))
                    .filter(tenant_user::dsl::user_id.eq(&user_id)),
            )
            .execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(user_id.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(
        tenant_id = %tenant_id,
        user_id = %user_id,
        "Tenant user deleted"
    );
    Ok(())
}

pub async fn get_tenant_users_for_user(user_id: &Uuid) -> Result<Vec<TenantUser>, Error> {
    task::spawn_blocking({
        let user_id = *user_id;
        move || -> Result<Vec<TenantUser>, Error> {
            let mut c = get_db_conn()?;
            let items = tenant_user::dsl::tenant_user
                .filter(tenant_user::dsl::user_id.eq(&user_id))
                .load(&mut c)?;
            Ok(items)
        }
    })
    .await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage::user::test::create_user;
    use crate::test;
    use chrono::SubsecRound;
    use std::str::FromStr;
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
            id: Uuid::new_v4(),
            created_at: Utc::now().round_subsecs(1),
            updated_at: Utc::now().round_subsecs(1),
            name: "test t".into(),
            description: "test description".into(),
            can_have_gateways: true,
            max_device_count: 20,
            max_gateway_count: 10,
            private_gateways_up: true,
            private_gateways_down: true,
        };
        create(t).await.unwrap()
    }

    #[tokio::test]
    async fn test_tenant() {
        let _guard = test::prepare().await;

        // delete default tenant
        let _ = delete(&Uuid::from_str("52f14cd4-c6f1-4fbd-8f87-4025e1d49242").unwrap())
            .await
            .unwrap();

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

        add_user(tu).await.unwrap();

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
                    user_id: Some(user.id),
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
        assert_eq!(true, delete(&t.id).await.is_err());
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
        let tu = add_user(tu).await.unwrap();

        // get
        let tu_get = get_user(&t.id, &user.id).await.unwrap();
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
}
