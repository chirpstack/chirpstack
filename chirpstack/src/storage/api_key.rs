use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use tokio::task;
use tracing::info;
use uuid::Uuid;

use super::error::Error;
use super::schema::api_key;
use super::{error, get_db_conn};

#[derive(Queryable, Insertable, PartialEq, Eq, Debug)]
#[diesel(table_name = api_key)]
pub struct ApiKey {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub is_admin: bool,
    pub tenant_id: Option<Uuid>,
}

impl ApiKey {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }

        Ok(())
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        ApiKey {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: "".into(),
            is_admin: false,
            tenant_id: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct Filters {
    pub tenant_id: Option<Uuid>,
    pub is_admin: bool,
}

pub async fn create(ak: ApiKey) -> Result<ApiKey, Error> {
    ak.validate()?;

    let ak = task::spawn_blocking(move || -> Result<ApiKey, Error> {
        let mut c = get_db_conn()?;
        diesel::insert_into(api_key::table)
            .values(&ak)
            .get_result(&mut c)
            .map_err(|e| error::Error::from_diesel(e, ak.id.to_string()))
    })
    .await??;
    info!(id = %ak.id, "Api-key created");
    Ok(ak)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let id = *id;

        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(api_key::dsl::api_key.find(&id)).execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(id.to_string()));
            }
            info!(id = %id, "Api-key deleted");
            Ok(())
        }
    })
    .await?
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();

        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;

            let mut q = api_key::dsl::api_key
                .select(dsl::count_star())
                .filter(api_key::dsl::is_admin.eq(filters.is_admin))
                .into_boxed();

            if let Some(tenant_id) = &filters.tenant_id {
                q = q.filter(api_key::dsl::tenant_id.eq(tenant_id));
            }

            Ok(q.first(&mut c)?)
        }
    })
    .await?
}

pub async fn list(limit: i64, offset: i64, filters: &Filters) -> Result<Vec<ApiKey>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();

        move || -> Result<Vec<ApiKey>, Error> {
            let mut c = get_db_conn()?;

            let mut q = api_key::dsl::api_key
                .filter(api_key::dsl::is_admin.eq(filters.is_admin))
                .into_boxed();

            if let Some(tenant_id) = &filters.tenant_id {
                q = q.filter(api_key::dsl::tenant_id.eq(tenant_id));
            }

            let items = q
                .order_by(api_key::dsl::name)
                .limit(limit)
                .offset(offset)
                .load(&mut c)?;
            Ok(items)
        }
    })
    .await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage::tenant;
    use crate::test;

    struct FilterTest<'a> {
        filters: Filters,
        keys: Vec<&'a ApiKey>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    pub async fn get(id: &Uuid) -> Result<ApiKey, Error> {
        task::spawn_blocking({
            let id = *id;

            move || -> Result<ApiKey, Error> {
                let mut c = get_db_conn()?;
                api_key::dsl::api_key
                    .find(&id)
                    .first(&mut c)
                    .map_err(|e| error::Error::from_diesel(e, id.to_string()))
            }
        })
        .await?
    }

    pub async fn create_api_key(is_admin: bool, is_tenant: bool) -> ApiKey {
        let ak = ApiKey {
            name: "test api key".into(),
            is_admin: is_admin,
            tenant_id: match is_tenant {
                false => None,
                true => Some(tenant::test::create_tenant().await.id),
            },
            ..Default::default()
        };

        create(ak).await.unwrap()
    }

    #[tokio::test]
    async fn api_key() {
        let _guard = test::prepare().await;
        let ak_admin = create_api_key(true, false).await;
        let ak_tenant = create_api_key(false, true).await;

        // get
        let ak_get = get(&ak_admin.id).await.unwrap();
        assert_eq!(ak_admin, ak_get);

        // get count and list
        let tests = vec![
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    is_admin: true,
                },
                keys: vec![&ak_admin],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: ak_tenant.tenant_id,
                    is_admin: false,
                },
                keys: vec![&ak_tenant],
                count: 1,
                limit: 10,
                offset: 0,
            },
        ];

        for tst in tests {
            let count = get_count(&tst.filters).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(tst.limit, tst.offset, &tst.filters).await.unwrap();
            assert_eq!(
                tst.keys
                    .iter()
                    .map(|k| { k.id.to_string() })
                    .collect::<String>(),
                items
                    .iter()
                    .map(|k| { k.id.to_string() })
                    .collect::<String>()
            );
        }

        // delete
        delete(&ak_admin.id).await.unwrap();
        assert_eq!(true, delete(&ak_admin.id).await.is_err());
    }
}
