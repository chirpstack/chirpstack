use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use tokio::task;
use tracing::info;
use uuid::Uuid;

use lrwn::EUI64;

use super::schema::{gateway, multicast_group_gateway, tenant};
use super::{error::Error, fields, get_db_conn};

#[derive(Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = gateway)]
pub struct Gateway {
    pub gateway_id: EUI64,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub stats_interval_secs: i32,
    pub tls_certificate: Option<Vec<u8>>,
    pub tags: fields::KeyValue,
    pub properties: fields::KeyValue,
}

impl Gateway {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }
        Ok(())
    }
}

#[derive(Queryable, PartialEq, Debug)]
pub struct GatewayListItem {
    pub tenant_id: Uuid,
    pub gateway_id: EUI64,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub properties: fields::KeyValue,
    pub stats_interval_secs: i32,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct GatewayMeta {
    pub gateway_id: EUI64,
    pub tenant_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub is_private_up: bool,
    pub is_private_down: bool,
}

#[derive(Default, Clone)]
pub struct Filters {
    pub tenant_id: Option<Uuid>,
    pub multicast_group_id: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(QueryableByName, PartialEq, Eq, Debug)]
pub struct GatewayCountsByState {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub never_seen_count: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub online_count: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub offline_count: i64,
}

impl Default for Gateway {
    fn default() -> Self {
        let now = Utc::now();

        Gateway {
            gateway_id: EUI64::from_be_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            tenant_id: Uuid::nil(),
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            name: "".into(),
            description: "".into(),
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            tls_certificate: None,
            stats_interval_secs: 30,
            tags: fields::KeyValue::new(HashMap::new()),
            properties: fields::KeyValue::new(HashMap::new()),
        }
    }
}

pub async fn create(gw: Gateway) -> Result<Gateway, Error> {
    gw.validate()?;
    let gw = task::spawn_blocking({
        move || -> Result<Gateway, Error> {
            let mut c = get_db_conn()?;
            c.transaction::<Gateway, Error, _>(|c| {
                // use for_update to lock the tenant.
                let t: super::tenant::Tenant = tenant::dsl::tenant
                    .find(&gw.tenant_id)
                    .for_update()
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, gw.tenant_id.to_string()))?;

                if !t.can_have_gateways {
                    return Err(Error::NotAllowed("Tenant can not have gateways".into()));
                }

                let gw_count: i64 = gateway::dsl::gateway
                    .select(dsl::count_star())
                    .filter(gateway::dsl::tenant_id.eq(&gw.tenant_id))
                    .first(c)?;

                if t.max_gateway_count != 0 && gw_count as i32 >= t.max_gateway_count {
                    return Err(Error::NotAllowed(
                        "Max number of gateways exceeded for tenant".into(),
                    ));
                }

                diesel::insert_into(gateway::table)
                    .values(&gw)
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, gw.gateway_id.to_string()))
            })
        }
    })
    .await??;
    info!(
        gateway_id = %gw.gateway_id,
        "Gateway created"
    );
    Ok(gw)
}

pub async fn get(gateway_id: &EUI64) -> Result<Gateway, Error> {
    task::spawn_blocking({
        let gateway_id = *gateway_id;
        move || -> Result<Gateway, Error> {
            let mut c = get_db_conn()?;
            let gw = gateway::dsl::gateway
                .find(&gateway_id)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, gateway_id.to_string()))?;
            Ok(gw)
        }
    })
    .await?
}

pub async fn update(gw: Gateway) -> Result<Gateway, Error> {
    gw.validate()?;
    let gw = task::spawn_blocking({
        move || -> Result<Gateway, Error> {
            let mut c = get_db_conn()?;
            diesel::update(gateway::dsl::gateway.find(&gw.gateway_id))
                .set((
                    gateway::updated_at.eq(Utc::now()),
                    gateway::name.eq(&gw.name),
                    gateway::description.eq(&gw.description),
                    gateway::latitude.eq(&gw.latitude),
                    gateway::longitude.eq(&gw.longitude),
                    gateway::altitude.eq(&gw.altitude),
                    gateway::stats_interval_secs.eq(&gw.stats_interval_secs),
                    gateway::tags.eq(&gw.tags),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, gw.gateway_id.to_string()))
        }
    })
    .await??;
    info!(
        gateway_id = %gw.gateway_id,
        "Gateway updated"
    );
    Ok(gw)
}

pub async fn update_state(id: &EUI64, props: &HashMap<String, String>) -> Result<Gateway, Error> {
    let gw = task::spawn_blocking({
        let id = *id;
        let props = fields::KeyValue::new(props.clone());
        move || -> Result<Gateway, Error> {
            let mut c = get_db_conn()?;
            let gw: Gateway = diesel::update(gateway::dsl::gateway.find(&id))
                .set((
                    gateway::last_seen_at.eq(Some(Utc::now())),
                    gateway::properties.eq(props),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;

            Ok(gw)
        }
    })
    .await??;

    info!(
        gateway_id = %id,
        "Gateway state updated"
    );

    Ok(gw)
}

pub async fn update_state_and_loc(
    id: &EUI64,
    lat: f64,
    lon: f64,
    alt: f32,
    props: &HashMap<String, String>,
) -> Result<Gateway, Error> {
    let gw = task::spawn_blocking({
        let id = *id;
        let props = fields::KeyValue::new(props.clone());
        move || -> Result<Gateway, Error> {
            let mut c = get_db_conn()?;
            let gw: Gateway = diesel::update(gateway::dsl::gateway.find(&id))
                .set((
                    gateway::last_seen_at.eq(Some(Utc::now())),
                    gateway::latitude.eq(lat),
                    gateway::longitude.eq(lon),
                    gateway::altitude.eq(alt),
                    gateway::properties.eq(props),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;

            Ok(gw)
        }
    })
    .await??;

    info!(
        gateway_id = %id,
        "Gateway state and location updated"
    );

    Ok(gw)
}

pub async fn update_tls_cert(id: &EUI64, cert: &[u8]) -> Result<Gateway, Error> {
    let gw = task::spawn_blocking({
        let id = *id;
        let cert = cert.to_vec();
        move || -> Result<Gateway, Error> {
            let mut c = get_db_conn()?;
            let gw: Gateway = diesel::update(gateway::dsl::gateway.find(&id))
                .set(gateway::tls_certificate.eq(cert))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;
            Ok(gw)
        }
    })
    .await??;

    info!(
        gateway_id = %id,
        "Gateway tls certificate updated"
    );

    Ok(gw)
}

pub async fn delete(gateway_id: &EUI64) -> Result<(), Error> {
    task::spawn_blocking({
        let gateway_id = *gateway_id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(gateway::dsl::gateway.find(&gateway_id)).execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(gateway_id.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(
        gateway_id = %gateway_id,
        "Gateway deleted"
    );
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = gateway::dsl::gateway
                .select(dsl::count_star())
                .distinct()
                .left_join(multicast_group_gateway::table)
                .into_boxed();

            if let Some(tenant_id) = &filters.tenant_id {
                q = q.filter(gateway::dsl::tenant_id.eq(tenant_id));
            }

            if let Some(multicast_group_id) = &filters.multicast_group_id {
                q = q.filter(
                    multicast_group_gateway::dsl::multicast_group_id.eq(multicast_group_id),
                );
            }

            if let Some(search) = &filters.search {
                q = q.filter(gateway::dsl::name.ilike(format!("%{}%", search)));
            }

            Ok(q.first(&mut c)?)
        }
    })
    .await?
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
) -> Result<Vec<GatewayListItem>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<GatewayListItem>, Error> {
            let mut c = get_db_conn()?;
            let mut q = gateway::dsl::gateway
                .left_join(multicast_group_gateway::table)
                .select((
                    gateway::tenant_id,
                    gateway::gateway_id,
                    gateway::name,
                    gateway::description,
                    gateway::created_at,
                    gateway::updated_at,
                    gateway::last_seen_at,
                    gateway::latitude,
                    gateway::longitude,
                    gateway::altitude,
                    gateway::properties,
                    gateway::stats_interval_secs,
                ))
                .distinct()
                .into_boxed();

            if let Some(tenant_id) = &filters.tenant_id {
                q = q.filter(gateway::dsl::tenant_id.eq(tenant_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(gateway::dsl::name.ilike(format!("%{}%", search)));
            }

            if let Some(multicast_group_id) = &filters.multicast_group_id {
                q = q.filter(
                    multicast_group_gateway::dsl::multicast_group_id.eq(multicast_group_id),
                );
            }

            let items = q
                .order_by(gateway::dsl::name)
                .limit(limit)
                .offset(offset)
                .load(&mut c)?;
            Ok(items)
        }
    })
    .await?
}

pub async fn get_meta(gateway_id: &EUI64) -> Result<GatewayMeta, Error> {
    task::spawn_blocking({
        let gateway_id = *gateway_id;
        move || -> Result<GatewayMeta, Error> {
            let mut c = get_db_conn()?;
            let meta = gateway::dsl::gateway
                .inner_join(tenant::table)
                .select((
                    gateway::gateway_id,
                    gateway::tenant_id,
                    gateway::latitude,
                    gateway::longitude,
                    gateway::altitude,
                    tenant::private_gateways_up,
                    tenant::private_gateways_down,
                ))
                .filter(gateway::dsl::gateway_id.eq(&gateway_id))
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, gateway_id.to_string()))?;

            Ok(meta)
        }
    })
    .await?
}

pub async fn get_counts_by_state(tenant_id: &Option<Uuid>) -> Result<GatewayCountsByState, Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        move || -> Result<GatewayCountsByState, Error> {
            let mut c = get_db_conn()?;
            let counts: GatewayCountsByState = diesel::sql_query(r#"
                select
                    coalesce(sum(case when last_seen_at is null then 1 end), 0) as never_seen_count,
                    coalesce(sum(case when (now() - make_interval(secs => stats_interval_secs * 2)) > last_seen_at then 1 end), 0) as offline_count,
                    coalesce(sum(case when (now() - make_interval(secs => stats_interval_secs * 2)) <= last_seen_at then 1 end), 0) as online_count
                from
                    gateway
                where
                    $1 is null or tenant_id = $1
            "#).bind::<diesel::sql_types::Nullable<diesel::sql_types::Uuid>, _>(tenant_id).get_result(&mut c)?;
            Ok(counts)
        }
    }).await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::{storage, test};
    use lrwn::region::CommonName;
    use lrwn::{AES128Key, DevAddr};

    struct FilterTest<'a> {
        filters: Filters,
        gws: Vec<&'a Gateway>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    pub async fn create_gateway(id: EUI64) -> Gateway {
        let tenant_id = {
            let t = storage::tenant::test::create_tenant().await;
            t.id
        };

        let gw = Gateway {
            gateway_id: id,
            tenant_id: tenant_id,
            name: "gw".into(),
            ..Default::default()
        };

        create(gw).await.unwrap()
    }

    #[tokio::test]
    async fn test_gateway() {
        let _guard = test::prepare().await;
        let mut gw = create_gateway(EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8])).await;

        let app = storage::application::create(storage::application::Application {
            tenant_id: gw.tenant_id,
            name: "test-app".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let mg = storage::multicast::create(storage::multicast::MulticastGroup {
            application_id: app.id,
            name: "test-mg".into(),
            region: CommonName::EU868,
            mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
            mc_nwk_s_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
            f_cnt: 10,
            group_type: "C".into(),
            dr: 1,
            frequency: 868100000,
            class_b_ping_slot_period: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        storage::multicast::add_gateway(&mg.id, &gw.gateway_id)
            .await
            .unwrap();

        // get
        let gw_get = get(&gw.gateway_id).await.unwrap();
        assert_eq!(gw, gw_get);

        // update
        gw.name = "updated-name".into();
        gw = update(gw).await.unwrap();
        let gw_get = get(&gw.gateway_id).await.unwrap();
        assert_eq!(gw, gw_get);

        // get count and list
        let tests = vec![
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    multicast_group_id: None,
                    search: None,
                },
                gws: vec![&gw],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    multicast_group_id: None,
                    search: Some("uup".into()),
                },
                gws: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    multicast_group_id: None,
                    search: Some("upd".into()),
                },
                gws: vec![&gw],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(gw.tenant_id),
                    multicast_group_id: None,
                    search: None,
                },
                gws: vec![&gw],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(Uuid::new_v4()),
                    multicast_group_id: None,
                    search: None,
                },
                gws: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    multicast_group_id: Some(mg.id),
                    search: None,
                },
                gws: vec![&gw],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    multicast_group_id: Some(Uuid::new_v4()),
                    search: None,
                },
                gws: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
        ];

        for tst in tests {
            let count = get_count(&tst.filters).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(tst.limit, tst.offset, &tst.filters).await.unwrap();
            assert_eq!(
                tst.gws
                    .iter()
                    .map(|gw| gw.gateway_id.to_string())
                    .collect::<String>(),
                items
                    .iter()
                    .map(|gw| gw.gateway_id.to_string())
                    .collect::<String>(),
            );
        }

        // delete
        delete(&gw.gateway_id).await.unwrap();
        assert_eq!(true, delete(&gw.gateway_id).await.is_err());
    }
}
