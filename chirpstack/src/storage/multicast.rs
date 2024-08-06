use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use diesel::{dsl, prelude::*};
use diesel_async::RunQueryDsl;
use tracing::info;
use uuid::Uuid;

use lrwn::region::CommonName;
use lrwn::{AES128Key, DevAddr, EUI64};

use super::error::Error;
use super::schema::{
    application, device, gateway, multicast_group, multicast_group_device, multicast_group_gateway,
    multicast_group_queue_item,
};
use super::{db_transaction, fields, get_async_db_conn};
use crate::downlink::classb;
use crate::{config, gpstime::ToDateTime, gpstime::ToGpsTime};

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = multicast_group)]
pub struct MulticastGroup {
    pub id: fields::Uuid,
    pub application_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub region: CommonName,
    pub mc_addr: DevAddr,
    pub mc_nwk_s_key: AES128Key,
    pub mc_app_s_key: AES128Key,
    pub f_cnt: i64,
    pub group_type: String,
    pub dr: i16,
    pub frequency: i64,
    pub class_b_ping_slot_nb_k: i16,
    pub class_c_scheduling_type: fields::MulticastGroupSchedulingType,
}

impl MulticastGroup {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }
        Ok(())
    }
}

impl Default for MulticastGroup {
    fn default() -> Self {
        let now = Utc::now();

        MulticastGroup {
            id: Uuid::new_v4().into(),
            application_id: Uuid::nil().into(),
            created_at: now,
            updated_at: now,
            name: "".into(),
            region: CommonName::EU868,
            mc_addr: DevAddr::default(),
            mc_nwk_s_key: AES128Key::default(),
            mc_app_s_key: AES128Key::default(),
            f_cnt: 0,
            group_type: "".into(),
            dr: 0,
            frequency: 0,
            class_b_ping_slot_nb_k: 0,
            class_c_scheduling_type: fields::MulticastGroupSchedulingType::DELAY,
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct MulticastGroupListItem {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub region: CommonName,
    pub group_type: String,
}

#[derive(Default, Clone)]
pub struct Filters {
    pub application_id: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(Clone, Queryable, QueryableByName, Insertable, AsChangeset, Debug, PartialEq, Eq)]
#[diesel(table_name = multicast_group_queue_item)]
pub struct MulticastGroupQueueItem {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub scheduler_run_after: DateTime<Utc>,
    pub multicast_group_id: fields::Uuid,
    pub gateway_id: EUI64,
    pub f_cnt: i64,
    pub f_port: i16,
    pub data: Vec<u8>,
    pub emit_at_time_since_gps_epoch: Option<i64>,
}

impl MulticastGroupQueueItem {
    fn validate(&self) -> Result<(), Error> {
        if self.f_port == 0 || self.f_port > 255 {
            return Err(Error::Validation(
                "FPort must be between 1 - 255".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for MulticastGroupQueueItem {
    fn default() -> Self {
        let now = Utc::now();

        MulticastGroupQueueItem {
            id: Uuid::new_v4().into(),
            created_at: now,
            scheduler_run_after: now,
            multicast_group_id: Uuid::nil().into(),
            gateway_id: Default::default(),
            f_cnt: 0,
            f_port: 0,
            data: vec![],
            emit_at_time_since_gps_epoch: None,
        }
    }
}

pub async fn create(mg: MulticastGroup) -> Result<MulticastGroup, Error> {
    mg.validate()?;

    let mg: MulticastGroup = diesel::insert_into(multicast_group::table)
        .values(&mg)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, mg.id.to_string()))?;
    info!(id = %mg.id, "Multicast-group created");
    Ok(mg)
}

pub async fn get(id: &Uuid) -> Result<MulticastGroup, Error> {
    multicast_group::dsl::multicast_group
        .find(&fields::Uuid::from(id))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, id.to_string()))
}

pub async fn update(mg: MulticastGroup) -> Result<MulticastGroup, Error> {
    mg.validate()?;

    let mg: MulticastGroup = diesel::update(multicast_group::dsl::multicast_group.find(&mg.id))
        .set((
            multicast_group::updated_at.eq(Utc::now()),
            multicast_group::name.eq(&mg.name),
            multicast_group::region.eq(&mg.region),
            multicast_group::mc_addr.eq(&mg.mc_addr),
            multicast_group::mc_nwk_s_key.eq(&mg.mc_nwk_s_key),
            multicast_group::mc_app_s_key.eq(&mg.mc_app_s_key),
            multicast_group::f_cnt.eq(&mg.f_cnt),
            multicast_group::group_type.eq(&mg.group_type),
            multicast_group::dr.eq(&mg.dr),
            multicast_group::frequency.eq(&mg.frequency),
            multicast_group::class_b_ping_slot_nb_k.eq(&mg.class_b_ping_slot_nb_k),
            multicast_group::class_c_scheduling_type.eq(&mg.class_c_scheduling_type),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, mg.id.to_string()))?;
    info!(id = %mg.id, "Multicast-group updated");
    Ok(mg)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    let ra = diesel::delete(multicast_group::dsl::multicast_group.find(&fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await?;
    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }
    info!(id = %id, "Multicast-group deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    let mut q = multicast_group::dsl::multicast_group
        .select(dsl::count_star())
        .into_boxed();

    if let Some(application_id) = &filters.application_id {
        q = q.filter(multicast_group::dsl::application_id.eq(fields::Uuid::from(application_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(multicast_group::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(multicast_group::dsl::name.like(format!("%{}%", search)));
        }
    }

    q.first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
) -> Result<Vec<MulticastGroupListItem>, Error> {
    let mut q = multicast_group::dsl::multicast_group
        .select((
            multicast_group::id,
            multicast_group::created_at,
            multicast_group::updated_at,
            multicast_group::name,
            multicast_group::region,
            multicast_group::group_type,
        ))
        .into_boxed();

    if let Some(application_id) = &filters.application_id {
        q = q.filter(multicast_group::dsl::application_id.eq(fields::Uuid::from(application_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(multicast_group::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(multicast_group::dsl::name.like(format!("%{}%", search)));
        }
    }

    q.order_by(multicast_group::dsl::name)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn add_device(group_id: &Uuid, dev_eui: &EUI64) -> Result<(), Error> {
    let mut c = get_async_db_conn().await?;
    db_transaction::<(), Error, _>(&mut c, |c| {
        Box::pin(async move {
            let device_query = device::dsl::device.find(&dev_eui);
            #[cfg(feature = "postgres")]
            let device_query = device_query.for_update();
            let d: super::device::Device = device_query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;

            let fields_group_id = fields::Uuid::from(group_id);

            let multicast_group_query =
                multicast_group::dsl::multicast_group.find(&fields_group_id);
            #[cfg(feature = "postgres")]
            let multicast_group_query = multicast_group_query.for_update();
            let mg: MulticastGroup = multicast_group_query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, group_id.to_string()))?;

            if d.application_id != mg.application_id {
                // Device not found within the same application.
                return Err(Error::NotFound(dev_eui.to_string()));
            }

            let _ = diesel::insert_into(multicast_group_device::table)
                .values((
                    multicast_group_device::multicast_group_id.eq(&fields_group_id),
                    multicast_group_device::dev_eui.eq(&dev_eui),
                    multicast_group_device::created_at.eq(Utc::now()),
                ))
                .execute(c)
                .await
                .map_err(|e| Error::from_diesel(e, "".into()))?;
            Ok(())
        })
    })
    .await?;
    info!(multicast_group_id = %group_id, dev_eui = %dev_eui, "Device added to multicast-group");
    Ok(())
}

pub async fn remove_device(group_id: &Uuid, dev_eui: &EUI64) -> Result<(), Error> {
    let ra = diesel::delete(
        multicast_group_device::dsl::multicast_group_device
            .filter(multicast_group_device::multicast_group_id.eq(&fields::Uuid::from(group_id)))
            .filter(multicast_group_device::dev_eui.eq(&dev_eui)),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;
    if ra == 0 {
        return Err(Error::NotFound(format!(
            "multicast-group: {}, device: {}",
            group_id, dev_eui
        )));
    }
    info!(multicast_group_id = %group_id, dev_eui = %dev_eui, "Device removed from multicast-group");
    Ok(())
}

pub async fn add_gateway(group_id: &Uuid, gateway_id: &EUI64) -> Result<(), Error> {
    let mut c = get_async_db_conn().await?;
    db_transaction::<(), Error, _>(&mut c, |c| {
        Box::pin(async move {
            let gateway_query = gateway::dsl::gateway.find(&gateway_id);
            #[cfg(feature = "postgres")]
            let gateway_query = gateway_query.for_update();
            let gw: super::gateway::Gateway = gateway_query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, gateway_id.to_string()))?;

            let fields_group_id = fields::Uuid::from(group_id);

            let multicast_group_query =
                multicast_group::dsl::multicast_group.find(&fields_group_id);
            #[cfg(feature = "postgres")]
            let multicast_group_query = multicast_group_query.for_update();
            let mg: MulticastGroup = multicast_group_query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, group_id.to_string()))?;

            let application_query = application::dsl::application.find(&mg.application_id);
            #[cfg(feature = "postgres")]
            let application_query = application_query.for_update();
            let a: super::application::Application = application_query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, mg.application_id.to_string()))?;

            if a.tenant_id != gw.tenant_id {
                // Gateway and multicast-group are not under same tenant.
                return Err(Error::NotFound(gateway_id.to_string()));
            }

            let _ = diesel::insert_into(multicast_group_gateway::table)
                .values((
                    multicast_group_gateway::multicast_group_id.eq(&fields_group_id),
                    multicast_group_gateway::gateway_id.eq(&gateway_id),
                    multicast_group_gateway::created_at.eq(Utc::now()),
                ))
                .execute(c)
                .await
                .map_err(|e| Error::from_diesel(e, "".into()))?;
            Ok(())
        })
    })
    .await?;
    info!(multicast_group_id = %group_id, gateway_id = %gateway_id, "Gateway added to multicast-group");
    Ok(())
}

pub async fn remove_gateway(group_id: &Uuid, gateway_id: &EUI64) -> Result<(), Error> {
    let ra = diesel::delete(
        multicast_group_gateway::dsl::multicast_group_gateway
            .filter(multicast_group_gateway::multicast_group_id.eq(&fields::Uuid::from(group_id)))
            .filter(multicast_group_gateway::gateway_id.eq(&gateway_id)),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;
    if ra == 0 {
        return Err(Error::NotFound(format!(
            "multicast-group: {}, gateway: {}",
            group_id, gateway_id
        )));
    }
    info!(multicast_group_id = %group_id, gateway_id = %gateway_id, "Gateway removed from multicast-group");
    Ok(())
}

pub async fn get_dev_euis(group_id: &Uuid) -> Result<Vec<EUI64>, Error> {
    multicast_group_device::dsl::multicast_group_device
        .select(multicast_group_device::dev_eui)
        .filter(multicast_group_device::dsl::multicast_group_id.eq(&fields::Uuid::from(group_id)))
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, group_id.to_string()))
}

pub async fn get_gateway_ids(group_id: &Uuid) -> Result<Vec<EUI64>, Error> {
    multicast_group_gateway::dsl::multicast_group_gateway
        .select(multicast_group_gateway::gateway_id)
        .filter(multicast_group_gateway::dsl::multicast_group_id.eq(&fields::Uuid::from(group_id)))
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, group_id.to_string()))
}

// This enqueues a multicast-group queue item for the given gateways and returns the frame-counter
// of the multicast downlink.
// This function locks the multicast-group to avoid race-conditions with scheduling time and
// frame-counters.
pub async fn enqueue(
    qi: MulticastGroupQueueItem,
    gateway_ids: &[EUI64],
) -> Result<(Vec<Uuid>, u32), Error> {
    qi.validate()?;
    let mut c = get_async_db_conn().await?;
    let conf = config::get();
    let (ids, f_cnt) = db_transaction::<(Vec<Uuid>, u32), Error, _>(&mut c, |c| {
        Box::pin(async move {
            let mut ids: Vec<Uuid> = Vec::new();
            let query = multicast_group::dsl::multicast_group.find(&qi.multicast_group_id);
            #[cfg(feature = "postgres")]
            let query = query.for_update();
            let mg: MulticastGroup = query
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, qi.multicast_group_id.to_string()))?;

            match mg.group_type.as_ref() {
                "B" => {
                    // get ping nb
                    let ping_nb = 1 << mg.class_b_ping_slot_nb_k as usize;

                    // get max. gps epoch time.
                    let res: Option<i64> =
                        multicast_group_queue_item::dsl::multicast_group_queue_item
                            .select(dsl::max(
                                multicast_group_queue_item::dsl::emit_at_time_since_gps_epoch,
                            ))
                            .filter(
                                multicast_group_queue_item::dsl::multicast_group_id
                                    .eq(&qi.multicast_group_id),
                            )
                            .first(c)
                            .await?;

                    // Get timestamp after which we must generate the next ping-slot.
                    let ping_slot_after_gps_time = match res {
                        Some(v) => Duration::try_milliseconds(v).unwrap_or_default(),
                        None => (Utc::now()
                            + Duration::from_std(conf.network.scheduler.multicast_class_b_margin)
                                .unwrap())
                        .to_gps_time(),
                    };

                    let emit_at_time_since_gps_epoch = classb::get_next_ping_slot_after(
                        ping_slot_after_gps_time,
                        &mg.mc_addr,
                        ping_nb,
                    )?;

                    let scheduler_run_after_ts = emit_at_time_since_gps_epoch.to_date_time()
                        - Duration::from_std(2 * conf.network.scheduler.interval).unwrap();

                    for gateway_id in gateway_ids {
                        let qi = MulticastGroupQueueItem {
                            scheduler_run_after: scheduler_run_after_ts,
                            multicast_group_id: mg.id.into(),
                            gateway_id: *gateway_id,
                            f_cnt: mg.f_cnt,
                            f_port: qi.f_port,
                            data: qi.data.clone(),
                            emit_at_time_since_gps_epoch: Some(
                                emit_at_time_since_gps_epoch.num_milliseconds(),
                            ),
                            ..Default::default()
                        };

                        let qi: MulticastGroupQueueItem =
                            diesel::insert_into(multicast_group_queue_item::table)
                                .values(&qi)
                                .get_result(c)
                                .await
                                .map_err(|e| Error::from_diesel(e, mg.id.to_string()))?;
                        ids.push(qi.id.into());
                    }
                }
                "C" => {
                    // Get max. scheduler_run_after timestamp.

                    #[cfg(feature = "postgres")]
                    let res: Option<DateTime<Utc>> =
                        multicast_group_queue_item::dsl::multicast_group_queue_item
                            .select(dsl::max(
                                multicast_group_queue_item::dsl::scheduler_run_after,
                            ))
                            .filter(
                                multicast_group_queue_item::dsl::multicast_group_id
                                    .eq(&qi.multicast_group_id),
                            )
                            .first(c)
                            .await?;

                    #[cfg(feature = "sqlite")]
                    let res: Option<DateTime<Utc>> =
                        multicast_group_queue_item::dsl::multicast_group_queue_item
                            .select(multicast_group_queue_item::dsl::scheduler_run_after)
                            .filter(
                                multicast_group_queue_item::dsl::multicast_group_id
                                    .eq(&qi.multicast_group_id),
                            )
                            .get_results(c)
                            .await?
                            .into_iter()
                            // fallback on code max instead of DB builtin
                            .max();

                    let mut scheduler_run_after_ts = match res {
                        Some(v) => {
                            v + Duration::from_std(conf.network.scheduler.multicast_class_c_margin)
                                .unwrap()
                        }
                        None => Utc::now(),
                    };

                    let emit_at_time_since_gps_epoch = if mg.class_c_scheduling_type
                        == fields::MulticastGroupSchedulingType::GPS_TIME
                    {
                        // Increment with margin as requesting the gateway to send the
                        // downlink 'now' will result in a too late error from the gateway.
                        scheduler_run_after_ts +=
                            Duration::from_std(conf.network.scheduler.multicast_class_c_margin)
                                .unwrap();
                        Some(scheduler_run_after_ts.to_gps_time().num_milliseconds())
                    } else {
                        None
                    };

                    for gateway_id in gateway_ids {
                        let qi = MulticastGroupQueueItem {
                            scheduler_run_after: scheduler_run_after_ts,
                            multicast_group_id: mg.id.into(),
                            gateway_id: *gateway_id,
                            f_cnt: mg.f_cnt,
                            f_port: qi.f_port,
                            data: qi.data.clone(),
                            emit_at_time_since_gps_epoch,
                            ..Default::default()
                        };

                        let qi: MulticastGroupQueueItem =
                            diesel::insert_into(multicast_group_queue_item::table)
                                .values(&qi)
                                .get_result(c)
                                .await
                                .map_err(|e| Error::from_diesel(e, mg.id.to_string()))?;
                        ids.push(qi.id.into());

                        if mg.class_c_scheduling_type == fields::MulticastGroupSchedulingType::DELAY
                        {
                            // Increment timing for each gateway to avoid colissions.
                            scheduler_run_after_ts +=
                                Duration::from_std(conf.network.scheduler.multicast_class_c_margin)
                                    .unwrap();
                        }
                    }
                }
                _ => {
                    return Err(Error::Anyhow(anyhow!(
                        "Invalid multicast group_type: {}",
                        mg.group_type
                    )));
                }
            }

            diesel::update(multicast_group::dsl::multicast_group.find(&qi.multicast_group_id))
                .set(multicast_group::f_cnt.eq(mg.f_cnt + 1))
                .execute(c)
                .await
                .map_err(|e| Error::from_diesel(e, qi.multicast_group_id.to_string()))?;

            // Return value before it was incremented
            Ok((ids, mg.f_cnt as u32))
        })
    })
    .await?;
    info!(multicast_group_id = %qi.multicast_group_id, f_cnt = f_cnt, "Multicast-group queue item created");
    Ok((ids, f_cnt))
}

pub async fn delete_queue_item(id: &Uuid) -> Result<(), Error> {
    let ra = diesel::delete(
        multicast_group_queue_item::dsl::multicast_group_queue_item.find(&fields::Uuid::from(id)),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;
    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }
    info!(id = %id, "Multicast-group queue item deleted");
    Ok(())
}

pub async fn flush_queue(multicast_group_id: &Uuid) -> Result<(), Error> {
    let _ = diesel::delete(
        multicast_group_queue_item::dsl::multicast_group_queue_item.filter(
            multicast_group_queue_item::multicast_group_id
                .eq(&fields::Uuid::from(multicast_group_id)),
        ),
    )
    .execute(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, multicast_group_id.to_string()))?;
    info!(multicast_group_id = %multicast_group_id, "Multicast-group queue flushed");
    Ok(())
}

pub async fn get_queue(multicast_group_id: &Uuid) -> Result<Vec<MulticastGroupQueueItem>, Error> {
    multicast_group_queue_item::dsl::multicast_group_queue_item
        .filter(
            multicast_group_queue_item::dsl::multicast_group_id
                .eq(&fields::Uuid::from(multicast_group_id)),
        )
        .order_by(multicast_group_queue_item::created_at)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, multicast_group_id.to_string()))
}

pub async fn get_schedulable_queue_items(limit: usize) -> Result<Vec<MulticastGroupQueueItem>> {
    let mut c = get_async_db_conn().await?;
    db_transaction::<Vec<MulticastGroupQueueItem>, Error, _>(&mut c, |c| {
            Box::pin(async move {
                let conf = config::get();
                diesel::sql_query(if cfg!(feature = "sqlite") {
                    r#"
                        update
                            multicast_group_queue_item
                        set
                            scheduler_run_after = ?3
                        where
                            id in (
                                select
                                    id
                                from
                                    multicast_group_queue_item
                                where
                                    scheduler_run_after <= ?2
                                order by
                                    created_at
                                limit ?1
                            )
                        returning *
                    "#
                } else {
                    r#"
                        update
                            multicast_group_queue_item
                        set
                            scheduler_run_after = $3
                        where
                            id in (
                                select
                                    qi.id
                                from
                                    multicast_group_queue_item qi
                                inner join gateway g
                                    on g.gateway_id = qi.gateway_id
                                where
                                    qi.scheduler_run_after <= $2
                                    and now() - make_interval(secs => g.stats_interval_secs * 2) <= g.last_seen_at
                                order by
                                    qi.created_at
                                limit $1
                            )
                        returning *
                    "#
                })
                .bind::<diesel::sql_types::Integer, _>(limit as i32)
                .bind::<fields::sql_types::Timestamptz, _>(Utc::now())
                .bind::<fields::sql_types::Timestamptz, _>(
                    Utc::now() + Duration::from_std(2 * conf.network.scheduler.interval).unwrap(),
                )
                .load(c)
                .await
                .map_err(|e| Error::from_diesel(e, "".into()))
            })
        })
        .await
        .context("Get schedulable multicast-group queue items")
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage::{application, device, device_profile, gateway, tenant};
    use crate::test;

    pub async fn get_queue_item(id: &Uuid) -> Result<MulticastGroupQueueItem, Error> {
        multicast_group_queue_item::dsl::multicast_group_queue_item
            .find(&fields::Uuid::from(id))
            .first(&mut get_async_db_conn().await?)
            .await
            .map_err(|e| Error::from_diesel(e, id.to_string()))
    }

    struct FilterTest<'a> {
        filters: Filters,
        groups: Vec<&'a MulticastGroup>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    #[tokio::test]
    async fn test_multicast_group() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // create
        let mut mg = create(MulticastGroup {
            application_id: app.id,
            name: "test-mg".into(),
            region: CommonName::EU868,
            mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
            mc_nwk_s_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
            mc_app_s_key: AES128Key::from_bytes([2, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
            f_cnt: 10,
            group_type: "C".into(),
            dr: 1,
            frequency: 868100000,
            class_b_ping_slot_nb_k: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        // get
        let mg_get = get(&mg.id.into()).await.unwrap();
        assert_eq!(mg, mg_get);

        // update
        mg.name = "test-mg-updated".into();
        mg.group_type = "B".into();
        mg.class_b_ping_slot_nb_k = 4;
        mg = update(mg).await.unwrap();
        let mg_get = get(&mg.id.into()).await.unwrap();
        assert_eq!(mg, mg_get);

        // get count and list
        let tests = vec![
            FilterTest {
                filters: Filters {
                    application_id: None,
                    search: None,
                },
                groups: vec![&mg],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: None,
                    search: Some("teest".into()),
                },
                groups: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: None,
                    search: Some("upd".into()),
                },
                groups: vec![&mg],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: Some(app.id.into()),
                    search: None,
                },
                groups: vec![&mg],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: Some(Uuid::new_v4()),
                    search: None,
                },
                groups: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
        ];

        for tst in &tests {
            let count = get_count(&tst.filters).await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(tst.limit, tst.offset, &tst.filters).await.unwrap();
            assert_eq!(
                tst.groups
                    .iter()
                    .map(|mg| mg.id.to_string())
                    .collect::<String>(),
                items.iter().map(|mg| mg.id.to_string()).collect::<String>()
            );
        }

        // delete
        delete(&mg.id.into()).await.unwrap();
        assert!(delete(&mg.id.into()).await.is_err());
    }

    #[tokio::test]
    async fn test_device() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let d = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-device".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            ..Default::default()
        })
        .await
        .unwrap();

        let mg = create(MulticastGroup {
            application_id: app.id,
            name: "test-mg".into(),
            region: CommonName::EU868,
            mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
            mc_nwk_s_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
            f_cnt: 10,
            group_type: "C".into(),
            dr: 1,
            frequency: 868100000,
            class_b_ping_slot_nb_k: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        // add device
        add_device(&mg.id.into(), &d.dev_eui).await.unwrap();

        // get group deveuis
        let dev_euis = get_dev_euis(&mg.id.into()).await.unwrap();
        assert_eq!(vec![d.dev_eui], dev_euis);

        // remove device
        remove_device(&mg.id.into(), &d.dev_eui).await.unwrap();
        let dev_euis = get_dev_euis(&mg.id.into()).await.unwrap();
        assert!(dev_euis.is_empty());
    }

    #[tokio::test]
    async fn test_gateway() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let gw = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            tenant_id: t.id,
            name: "test-gw".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let mg = create(MulticastGroup {
            application_id: app.id,
            name: "test-mg".into(),
            region: CommonName::EU868,
            mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
            mc_nwk_s_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
            f_cnt: 10,
            group_type: "C".into(),
            dr: 1,
            frequency: 868100000,
            class_b_ping_slot_nb_k: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        // add gateway
        add_gateway(&mg.id.into(), &gw.gateway_id).await.unwrap();

        // get gateway ids
        let gw_ids = get_gateway_ids(&mg.id.into()).await.unwrap();
        assert_eq!(vec![gw.gateway_id], gw_ids);

        // remove gateway
        remove_gateway(&mg.id.into(), &gw.gateway_id).await.unwrap();
        let gw_ids = get_gateway_ids(&mg.id.into()).await.unwrap();
        assert!(gw_ids.is_empty());
    }

    #[tokio::test]
    async fn test_queue() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let gw = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            name: "test-gw".into(),
            tenant_id: t.id,
            stats_interval_secs: 30,
            last_seen_at: Some(Utc::now()),
            ..Default::default()
        })
        .await
        .unwrap();

        let mut mg = create(MulticastGroup {
            application_id: app.id,
            name: "test-mg".into(),
            region: CommonName::EU868,
            mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
            mc_nwk_s_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
            f_cnt: 10,
            group_type: "C".into(),
            dr: 1,
            frequency: 868100000,
            class_b_ping_slot_nb_k: 1,
            class_c_scheduling_type: fields::MulticastGroupSchedulingType::DELAY,
            ..Default::default()
        })
        .await
        .unwrap();

        // invalid f_port
        assert!(enqueue(
            MulticastGroupQueueItem {
                multicast_group_id: mg.id.into(),
                gateway_id: gw.gateway_id,
                f_cnt: 1,
                f_port: 0,
                data: vec![3, 2, 1],
                ..Default::default()
            },
            &[gw.gateway_id],
        )
        .await
        .is_err());

        assert!(enqueue(
            MulticastGroupQueueItem {
                multicast_group_id: mg.id.into(),
                gateway_id: gw.gateway_id,
                f_cnt: 1,
                f_port: 256,
                data: vec![3, 2, 1],
                ..Default::default()
            },
            &[gw.gateway_id],
        )
        .await
        .is_err());

        // Enqueue (Class-C) (delay)
        let (ids, f_cnt) = enqueue(
            MulticastGroupQueueItem {
                multicast_group_id: mg.id.into(),
                gateway_id: gw.gateway_id,
                f_cnt: 1,
                f_port: 2,
                data: vec![3, 2, 1],
                ..Default::default()
            },
            &[gw.gateway_id],
        )
        .await
        .unwrap();
        assert_eq!(1, ids.len());
        assert_eq!(10, f_cnt);

        // get
        let qi_get = get_queue_item(&ids[0]).await.unwrap();
        assert!((Utc::now() - qi_get.scheduler_run_after) < Duration::try_seconds(1).unwrap()); // ~ Utc::now()
        assert!(qi_get.emit_at_time_since_gps_epoch.is_none());
        assert_eq!(10, qi_get.f_cnt);
        assert_eq!(vec![3, 2, 1], qi_get.data);

        // delete
        delete_queue_item(&ids[0]).await.unwrap();
        assert!(delete_queue_item(&ids[0]).await.is_err());

        // Enqueue (Class-C) (GPS time)
        mg.class_c_scheduling_type = fields::MulticastGroupSchedulingType::GPS_TIME;
        let mut mg = update(mg).await.unwrap();
        let (ids, f_cnt) = enqueue(
            MulticastGroupQueueItem {
                multicast_group_id: mg.id.into(),
                gateway_id: gw.gateway_id,
                f_cnt: 1,
                f_port: 2,
                data: vec![3, 2, 1],
                ..Default::default()
            },
            &[gw.gateway_id],
        )
        .await
        .unwrap();
        assert_eq!(1, ids.len());
        assert_eq!(10, f_cnt);

        // get
        let qi_get = get_queue_item(&ids[0]).await.unwrap();
        assert!(qi_get.emit_at_time_since_gps_epoch.is_some());

        // delete
        delete_queue_item(&ids[0]).await.unwrap();
        assert!(delete_queue_item(&ids[0]).await.is_err());

        // Enqueue (Class-B)
        mg.group_type = "B".into();
        let mg = update(mg).await.unwrap();
        let (ids, f_cnt) = enqueue(
            MulticastGroupQueueItem {
                multicast_group_id: mg.id.into(),
                gateway_id: gw.gateway_id,
                f_cnt: 1,
                f_port: 2,
                data: vec![3, 2, 1],
                ..Default::default()
            },
            &[gw.gateway_id],
        )
        .await
        .unwrap();
        assert_eq!(1, ids.len());
        assert_eq!(10, f_cnt);

        // get
        let qi_get = get_queue_item(&ids[0]).await.unwrap();
        assert!(Utc::now() < qi_get.scheduler_run_after); // in the future because of margin + next ping slot calculation
        assert!(qi_get.emit_at_time_since_gps_epoch.is_some());
        assert_eq!(10, qi_get.f_cnt);
        assert_eq!(vec![3, 2, 1], qi_get.data);

        // flush queue
        flush_queue(&mg.id.into()).await.unwrap();
        assert!(delete_queue_item(&ids[0]).await.is_err());
    }
}
