use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{dsl, prelude::*};
use diesel_async::RunQueryDsl;
use tracing::info;
use uuid::Uuid;

use lrwn::{DevAddr, EUI64};

use super::schema::{gateway, multicast_group_gateway, relay_gateway, tenant};
use super::{db_transaction, error::Error, fields, get_async_db_conn};

pub type RelayId = DevAddr;

#[derive(Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = gateway)]
pub struct Gateway {
    pub gateway_id: EUI64,
    pub tenant_id: fields::Uuid,
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

impl Default for Gateway {
    fn default() -> Self {
        let now = Utc::now();

        Gateway {
            gateway_id: EUI64::from_be_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            tenant_id: Uuid::nil().into(),
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

#[derive(AsChangeset, Debug, Clone, Default)]
#[diesel(table_name = gateway)]
pub struct GatewayChangeset {
    pub last_seen_at: Option<Option<DateTime<Utc>>>,
    pub properties: Option<fields::KeyValue>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f32>,
    pub tls_certificate: Option<Option<Vec<u8>>>,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct GatewayListItem {
    pub tenant_id: fields::Uuid,
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
    pub tenant_id: fields::Uuid,
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

#[derive(Clone, Debug, Default)]
pub enum OrderBy {
    #[default]
    Name,
    GatewayId,
    LastSeenAt,
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

#[derive(Queryable, Insertable, PartialEq, Debug)]
#[diesel(table_name = relay_gateway)]
pub struct RelayGateway {
    pub tenant_id: fields::Uuid,
    pub relay_id: RelayId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
    pub stats_interval_secs: i32,
    pub region_config_id: String,
}

impl Default for RelayGateway {
    fn default() -> Self {
        let now = Utc::now();

        RelayGateway {
            relay_id: RelayId::from_be_bytes([1, 2, 3, 4]),
            tenant_id: Uuid::nil().into(),
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            name: "".into(),
            description: "".into(),
            stats_interval_secs: 900,
            region_config_id: "".into(),
        }
    }
}

#[derive(Default, Clone)]
pub struct RelayGatewayFilters {
    pub tenant_id: Option<Uuid>,
}

#[derive(Queryable, PartialEq, Debug)]
pub struct RelayGatewayListItem {
    pub relay_id: RelayId,
    pub tenant_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
    pub stats_interval_secs: i32,
    pub region_config_id: String,
}

pub async fn create(gw: Gateway) -> Result<Gateway, Error> {
    gw.validate()?;
    let mut c = get_async_db_conn().await?;
    let gw: Gateway = db_transaction::<Gateway, Error, _>(&mut c, |c| {
        Box::pin(async move {
            let query = tenant::dsl::tenant.find(&gw.tenant_id);
            // use for_update to lock the tenant.
            #[cfg(feature = "postgres")]
            let query = query.for_update();
            let t: super::tenant::Tenant = query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, gw.tenant_id.to_string()))?;

            if !t.can_have_gateways {
                return Err(Error::NotAllowed("Tenant can not have gateways".into()));
            }

            let gw_count: i64 = gateway::dsl::gateway
                .select(dsl::count_star())
                .filter(gateway::dsl::tenant_id.eq(&gw.tenant_id))
                .first(c)
                .await?;

            if t.max_gateway_count != 0 && gw_count as i32 >= t.max_gateway_count {
                return Err(Error::NotAllowed(
                    "Max number of gateways exceeded for tenant".into(),
                ));
            }

            diesel::insert_into(gateway::table)
                .values(&gw)
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, gw.gateway_id.to_string()))
        })
    })
    .await?;
    info!(
        gateway_id = %gw.gateway_id,
        "Gateway created"
    );
    Ok(gw)
}

pub async fn get(gateway_id: &EUI64) -> Result<Gateway, Error> {
    let gw = gateway::dsl::gateway
        .find(&gateway_id)
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, gateway_id.to_string()))?;
    Ok(gw)
}

pub async fn update(gw: Gateway) -> Result<Gateway, Error> {
    gw.validate()?;

    let gw: Gateway = diesel::update(gateway::dsl::gateway.find(&gw.gateway_id))
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
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, gw.gateway_id.to_string()))?;
    info!(
        gateway_id = %gw.gateway_id,
        "Gateway updated"
    );
    Ok(gw)
}

pub async fn partial_update(gateway_id: EUI64, gw: &GatewayChangeset) -> Result<Gateway, Error> {
    let gw = diesel::update(gateway::dsl::gateway.find(&gateway_id))
        .set(gw)
        .get_result::<Gateway>(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, gateway_id.to_string()))?;

    info!(gateway_id = %gateway_id, "Gateway partially updated");
    Ok(gw)
}

pub async fn delete(gateway_id: &EUI64) -> Result<(), Error> {
    let ra = diesel::delete(gateway::dsl::gateway.find(&gateway_id))
        .execute(&mut get_async_db_conn().await?)
        .await?;
    if ra == 0 {
        return Err(Error::NotFound(gateway_id.to_string()));
    }
    info!(
        gateway_id = %gateway_id,
        "Gateway deleted"
    );
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    let mut q = gateway::dsl::gateway
        .select(dsl::count_star())
        .distinct()
        .left_join(multicast_group_gateway::table)
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(gateway::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    if let Some(multicast_group_id) = &filters.multicast_group_id {
        q = q.filter(
            multicast_group_gateway::dsl::multicast_group_id
                .eq(fields::Uuid::from(multicast_group_id)),
        );
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(gateway::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(gateway::dsl::name.like(format!("%{}%", search)));
        }
    }

    Ok(q.first(&mut get_async_db_conn().await?).await?)
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
    order_by: OrderBy,
    order_by_desc: bool,
) -> Result<Vec<GatewayListItem>, Error> {
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
        q = q.filter(gateway::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(gateway::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(gateway::dsl::name.like(format!("%{}%", search)));
        }
    }

    if let Some(multicast_group_id) = &filters.multicast_group_id {
        q = q.filter(
            multicast_group_gateway::dsl::multicast_group_id
                .eq(fields::Uuid::from(multicast_group_id)),
        );
    }

    q = match order_by_desc {
        true => match order_by {
            OrderBy::Name => q.order_by(gateway::dsl::name.desc()),
            OrderBy::GatewayId => q.order_by(gateway::dsl::gateway_id.desc()),
            OrderBy::LastSeenAt => {
                #[cfg(feature = "postgres")]
                {
                    q.order_by(gateway::dsl::last_seen_at.desc().nulls_last())
                        .then_order_by(gateway::dsl::name)
                }

                #[cfg(feature = "sqlite")]
                {
                    q.order_by(gateway::dsl::last_seen_at.desc())
                        .then_order_by(gateway::dsl::name)
                }
            }
        },
        false => match order_by {
            OrderBy::Name => q.order_by(gateway::dsl::name),
            OrderBy::GatewayId => q.order_by(gateway::dsl::gateway_id),
            OrderBy::LastSeenAt => {
                #[cfg(feature = "postgres")]
                {
                    q.order_by(gateway::dsl::last_seen_at.asc().nulls_first())
                        .then_order_by(gateway::dsl::name)
                }

                #[cfg(feature = "sqlite")]
                {
                    q.order_by(gateway::dsl::last_seen_at.asc())
                        .then_order_by(gateway::dsl::name)
                }
            }
        },
    };

    let items = q
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await?;
    Ok(items)
}

pub async fn get_meta(gateway_id: &EUI64) -> Result<GatewayMeta, Error> {
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
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, gateway_id.to_string()))?;
    Ok(meta)
}

#[cfg(feature = "postgres")]
pub async fn get_counts_by_state(tenant_id: &Option<Uuid>) -> Result<GatewayCountsByState, Error> {
    let counts: GatewayCountsByState = diesel::sql_query(r#"
        select
            coalesce(sum(case when last_seen_at is null then 1 end), 0) as never_seen_count,
            coalesce(sum(case when (now() - make_interval(secs => stats_interval_secs * 2)) > last_seen_at then 1 end), 0) as offline_count,
            coalesce(sum(case when (now() - make_interval(secs => stats_interval_secs * 2)) <= last_seen_at then 1 end), 0) as online_count
        from
            gateway
        where
            $1 is null or tenant_id = $1
    "#).bind::<diesel::sql_types::Nullable<fields::sql_types::Uuid>, _>(tenant_id.map(fields::Uuid::from)).get_result(&mut get_async_db_conn().await?).await?;
    Ok(counts)
}

#[cfg(feature = "sqlite")]
pub async fn get_counts_by_state(tenant_id: &Option<Uuid>) -> Result<GatewayCountsByState, Error> {
    let counts: GatewayCountsByState = diesel::sql_query(r#"
        select
            coalesce(sum(case when last_seen_at is null then 1 end), 0) as never_seen_count,
            coalesce(sum(case when (unixepoch('now') - unixepoch(last_seen_at)) > (stats_interval_secs * 2) then 1 end), 0) as offline_count,
            coalesce(sum(case when (unixepoch('now') - unixepoch(last_seen_at)) <= (stats_interval_secs * 2) then 1 end), 0) as online_count
        from
            gateway
        where
            ?1 is null or tenant_id = ?1
    "#).bind::<diesel::sql_types::Nullable<fields::sql_types::Uuid>, _>(tenant_id.map(|u| fields::Uuid::from(u))).get_result(&mut get_async_db_conn().await?).await?;
    Ok(counts)
}

pub async fn create_relay_gateway(relay: RelayGateway) -> Result<RelayGateway, Error> {
    let relay: RelayGateway = diesel::insert_into(relay_gateway::table)
        .values(&relay)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, relay.relay_id.to_string()))?;

    info!(relay_id = %relay.relay_id, "Relay Gateway created");

    Ok(relay)
}

pub async fn get_relay_gateway(tenant_id: Uuid, relay_id: RelayId) -> Result<RelayGateway, Error> {
    let relay = relay_gateway::dsl::relay_gateway
        .find((fields::Uuid::from(tenant_id), &relay_id))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, relay_id.to_string()))?;
    Ok(relay)
}

pub async fn update_relay_gateway(relay: RelayGateway) -> Result<RelayGateway, Error> {
    let relay: RelayGateway =
        diesel::update(relay_gateway::dsl::relay_gateway.find((&relay.tenant_id, &relay.relay_id)))
            .set((
                relay_gateway::updated_at.eq(&relay.updated_at),
                relay_gateway::last_seen_at.eq(&relay.last_seen_at),
                relay_gateway::name.eq(&relay.name),
                relay_gateway::description.eq(&relay.description),
                relay_gateway::stats_interval_secs.eq(&relay.stats_interval_secs),
                relay_gateway::region_config_id.eq(&relay.region_config_id),
            ))
            .get_result(&mut get_async_db_conn().await?)
            .await
            .map_err(|e| Error::from_diesel(e, relay.relay_id.to_string()))?;

    info!(relay_id = %relay.relay_id, "Relay Gateway updated");

    Ok(relay)
}

pub async fn get_relay_gateway_count(filters: &RelayGatewayFilters) -> Result<i64, Error> {
    let mut q = relay_gateway::dsl::relay_gateway
        .select(dsl::count_star())
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(relay_gateway::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    Ok(q.first(&mut get_async_db_conn().await?).await?)
}

pub async fn delete_relay_gateway(tenant_id: Uuid, relay_id: RelayId) -> Result<(), Error> {
    let ra = diesel::delete(
        relay_gateway::dsl::relay_gateway.find((fields::Uuid::from(tenant_id), &relay_id)),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;
    if ra == 0 {
        return Err(Error::NotFound(relay_id.to_string()));
    }

    info!(relay_id = %relay_id, "Relay Gateway deleted");

    Ok(())
}

pub async fn list_relay_gateways(
    limit: i64,
    offset: i64,
    filters: &RelayGatewayFilters,
) -> Result<Vec<RelayGatewayListItem>, Error> {
    let mut q = relay_gateway::dsl::relay_gateway
        .select((
            relay_gateway::relay_id,
            relay_gateway::tenant_id,
            relay_gateway::created_at,
            relay_gateway::updated_at,
            relay_gateway::last_seen_at,
            relay_gateway::name,
            relay_gateway::description,
            relay_gateway::stats_interval_secs,
            relay_gateway::region_config_id,
        ))
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(relay_gateway::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    let items = q
        .order_by(relay_gateway::dsl::name)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await?;
    Ok(items)
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
        order: OrderBy,
        order_by_desc: bool,
    }

    struct RelayGatewayFilterTest<'a> {
        filters: RelayGatewayFilters,
        relay_gateways: Vec<&'a RelayGateway>,
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
            tenant_id,
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
            class_b_ping_slot_periodicity: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        storage::multicast::add_gateway(&mg.id.into(), &gw.gateway_id)
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
                order: OrderBy::Name,
                order_by_desc: false,
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
                order: OrderBy::Name,
                order_by_desc: false,
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
                order: OrderBy::Name,
                order_by_desc: false,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(gw.tenant_id.into()),
                    multicast_group_id: None,
                    search: None,
                },
                gws: vec![&gw],
                count: 1,
                limit: 10,
                offset: 0,
                order: OrderBy::Name,
                order_by_desc: false,
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
                order: OrderBy::Name,
                order_by_desc: false,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    multicast_group_id: Some(mg.id.into()),
                    search: None,
                },
                gws: vec![&gw],
                count: 1,
                limit: 10,
                offset: 0,
                order: OrderBy::Name,
                order_by_desc: false,
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
                order: OrderBy::Name,
                order_by_desc: false,
            },
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
                order: OrderBy::Name,
                order_by_desc: false,
            },
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
                order: OrderBy::Name,
                order_by_desc: false,
            },
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
                order: OrderBy::Name,
                order_by_desc: true,
            },
        ];

        for tst in tests {
            let count = get_count(&tst.filters).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(
                tst.limit,
                tst.offset,
                &tst.filters,
                tst.order,
                tst.order_by_desc,
            )
            .await
            .unwrap();
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
        assert!(delete(&gw.gateway_id).await.is_err());
    }

    #[tokio::test]
    async fn test_relay_gateway() {
        let _guard = test::prepare().await;
        let gw = create_gateway(EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8])).await;

        // create
        let mut relay = create_relay_gateway(RelayGateway {
            relay_id: RelayId::from_be_bytes([1, 2, 3, 4]),
            tenant_id: gw.tenant_id,
            name: "test-relay".into(),
            description: "test relay".into(),
            region_config_id: "eu868".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // get
        let relay_get = get_relay_gateway(relay.tenant_id.into(), relay.relay_id)
            .await
            .unwrap();
        assert_eq!(relay, relay_get);

        // update
        relay.name = "updated-relay".into();
        relay.region_config_id = "us915_0".into();
        relay = update_relay_gateway(relay).await.unwrap();
        let relay_get = get_relay_gateway(relay.tenant_id.into(), relay.relay_id)
            .await
            .unwrap();
        assert_eq!(relay, relay_get);

        // test count and list
        let tests = vec![
            RelayGatewayFilterTest {
                filters: RelayGatewayFilters { tenant_id: None },
                relay_gateways: vec![&relay],
                count: 1,
                limit: 10,
                offset: 0,
            },
            RelayGatewayFilterTest {
                filters: RelayGatewayFilters {
                    tenant_id: Some(gw.tenant_id.into()),
                },
                relay_gateways: vec![&relay],
                count: 1,
                limit: 10,
                offset: 0,
            },
            RelayGatewayFilterTest {
                filters: RelayGatewayFilters {
                    tenant_id: Some(gw.tenant_id.into()),
                },
                relay_gateways: vec![&relay],
                count: 1,
                limit: 10,
                offset: 0,
            },
        ];

        for tst in tests {
            let count = get_relay_gateway_count(&tst.filters).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list_relay_gateways(tst.limit, tst.offset, &tst.filters)
                .await
                .unwrap();
            assert_eq!(
                tst.relay_gateways
                    .iter()
                    .map(|r| r.relay_id.to_string())
                    .collect::<String>(),
                items
                    .iter()
                    .map(|r| r.relay_id.to_string())
                    .collect::<String>(),
            );
        }

        // delete
        delete_relay_gateway(relay.tenant_id.into(), relay.relay_id)
            .await
            .unwrap();
        assert!(delete_relay_gateway(relay.tenant_id.into(), relay.relay_id)
            .await
            .is_err());
    }
}
