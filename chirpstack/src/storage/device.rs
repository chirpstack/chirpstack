use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use anyhow::{Context, Result};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Utc};
use diesel::{backend::Backend, deserialize, dsl, prelude::*, serialize, sql_types::Text};
use tokio::task;
use tracing::info;
use uuid::Uuid;

use lrwn::{DevAddr, EUI64};

use super::schema::{application, device, device_profile, multicast_group_device, tenant};
use super::{error::Error, fields, get_db_conn, get_redis_conn, redis_key};
use crate::config;

#[derive(Debug, Clone, Copy, Eq, PartialEq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum DeviceClass {
    A,
    B,
    C,
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for DeviceClass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "A" => DeviceClass::A,
            "B" => DeviceClass::B,
            "C" => DeviceClass::C,
            _ => return Err(anyhow!("Unexpected DeviceClass: {}", s)),
        })
    }
}

impl<DB> deserialize::FromSql<Text, DB> for DeviceClass
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(value: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(value)?;
        Ok(DeviceClass::from_str(&string)?)
    }
}

impl serialize::ToSql<Text, diesel::pg::Pg> for DeviceClass
where
    str: serialize::ToSql<Text, diesel::pg::Pg>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> serialize::Result {
        <str as serialize::ToSql<Text, diesel::pg::Pg>>::to_sql(
            &self.to_string(),
            &mut out.reborrow(),
        )
    }
}

#[derive(Queryable, QueryableByName, Insertable, PartialEq, Debug, Clone)]
#[diesel(table_name = device)]
pub struct Device {
    pub dev_eui: EUI64,
    pub application_id: Uuid,
    pub device_profile_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub scheduler_run_after: Option<DateTime<Utc>>,
    pub name: String,
    pub description: String,
    pub external_power_source: bool,
    pub battery_level: Option<BigDecimal>,
    pub margin: Option<i32>,
    pub dr: Option<i16>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f32>,
    pub dev_addr: Option<DevAddr>,
    pub enabled_class: DeviceClass,
    pub skip_fcnt_check: bool,
    pub is_disabled: bool,
    pub tags: fields::KeyValue,
    pub variables: fields::KeyValue,
    pub join_eui: EUI64,
}

impl Device {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }
        Ok(())
    }
}

impl Default for Device {
    fn default() -> Self {
        let now = Utc::now();

        Device {
            dev_eui: EUI64::default(),
            application_id: Uuid::nil(),
            device_profile_id: Uuid::nil(),
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            scheduler_run_after: None,
            name: "".into(),
            description: "".into(),
            external_power_source: false,
            battery_level: None,
            margin: None,
            dr: None,
            latitude: None,
            longitude: None,
            altitude: None,
            dev_addr: None,
            enabled_class: DeviceClass::A,
            skip_fcnt_check: false,
            is_disabled: false,
            tags: fields::KeyValue::new(HashMap::new()),
            variables: fields::KeyValue::new(HashMap::new()),
            join_eui: EUI64::default(),
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct DeviceListItem {
    pub dev_eui: EUI64,
    pub name: String,
    pub description: String,
    pub device_profile_id: Uuid,
    pub device_profile_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub margin: Option<i32>,
    pub external_power_source: bool,
    pub battery_level: Option<BigDecimal>,
}

#[derive(Default, Clone)]
pub struct Filters {
    pub application_id: Option<Uuid>,
    pub multicast_group_id: Option<Uuid>,
    pub search: Option<String>,
}

#[derive(QueryableByName, PartialEq, Eq, Debug)]
pub struct DevicesActiveInactive {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub never_seen_count: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub active_count: i64,
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub inactive_count: i64,
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct DevicesDataRate {
    pub dr: Option<i16>, // as the column is nullable
    pub count: i64,
}

pub async fn create(d: Device) -> Result<Device, Error> {
    d.validate()?;
    let d = task::spawn_blocking({
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            c.transaction::<Device, Error, _>(|c| {
                // use for update to lock the tenant
                let t: super::tenant::Tenant = tenant::dsl::tenant
                    .select((
                        tenant::dsl::id,
                        tenant::dsl::created_at,
                        tenant::dsl::updated_at,
                        tenant::dsl::name,
                        tenant::dsl::description,
                        tenant::dsl::can_have_gateways,
                        tenant::dsl::max_device_count,
                        tenant::dsl::max_gateway_count,
                        tenant::dsl::private_gateways_up,
                        tenant::dsl::private_gateways_down,
                    ))
                    .inner_join(application::table)
                    .filter(application::dsl::id.eq(&d.application_id))
                    .first(c)?;

                let dev_count: i64 = device::dsl::device
                    .select(dsl::count_star())
                    .inner_join(application::table)
                    .filter(application::dsl::tenant_id.eq(&t.id))
                    .first(c)?;

                if t.max_device_count != 0 && dev_count as i32 >= t.max_device_count {
                    return Err(Error::NotAllowed(
                        "Max number of devices exceeded for tenant".into(),
                    ));
                }

                diesel::insert_into(device::table)
                    .values(&d)
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, d.dev_eui.to_string()))
            })
        }
    })
    .await??;
    info!(dev_eui = %d.dev_eui, "Device created");
    Ok(d)
}

pub async fn get(dev_eui: &EUI64) -> Result<Device, Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            let d = device::dsl::device
                .find(&dev_eui)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
            Ok(d)
        }
    })
    .await?
}

pub async fn update(d: Device) -> Result<Device, Error> {
    d.validate()?;
    let d = task::spawn_blocking({
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&d.dev_eui))
                .set((
                    device::updated_at.eq(Utc::now()),
                    device::application_id.eq(&d.application_id),
                    device::device_profile_id.eq(&d.device_profile_id),
                    device::name.eq(&d.name),
                    device::description.eq(&d.description),
                    device::skip_fcnt_check.eq(&d.skip_fcnt_check),
                    device::is_disabled.eq(&d.is_disabled),
                    device::tags.eq(&d.tags),
                    device::variables.eq(&d.variables),
                    device::join_eui.eq(&d.join_eui),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, d.dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %d.dev_eui, "Device updated");
    Ok(d)
}

pub async fn set_enabled_class(dev_eui: &EUI64, mode: DeviceClass) -> Result<Device, Error> {
    let d = task::spawn_blocking({
        let dev_eui = *dev_eui;

        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&dev_eui))
                .set(device::enabled_class.eq(&mode))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, enabled_class = %mode, "Enabled class updated");
    Ok(d)
}

pub async fn set_join_eui(dev_eui: EUI64, join_eui: EUI64) -> Result<Device, Error> {
    let d = task::spawn_blocking({
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&dev_eui))
                .set(device::join_eui.eq(&join_eui))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, join_eui = %join_eui, "Updated JoinEUI");
    Ok(d)
}

pub async fn set_dev_addr(dev_eui: EUI64, dev_addr: DevAddr) -> Result<Device, Error> {
    let d = task::spawn_blocking({
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&dev_eui))
                .set(device::dev_addr.eq(&dev_addr))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, dev_addr = %dev_addr, "Updated DevAddr");
    Ok(d)
}

// In case the current_ts has been updated during the last device get and calling this update
// function, this will return a NotFound error. The purpose of this error is to catch concurrent
// scheduling, e.g. Class-A downlink and Class-B/C downlink. In such case we want to terminate one
// of the downlinks.
pub async fn set_scheduler_run_after(
    dev_eui: &EUI64,
    new_ts: Option<DateTime<Utc>>,
) -> Result<Device, Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&dev_eui))
                .set(device::scheduler_run_after.eq(&new_ts))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await?
}

pub async fn set_last_seen_dr(dev_eui: &EUI64, dr: u8) -> Result<Device, Error> {
    let d = task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&dev_eui))
                .set((
                    device::last_seen_at.eq(Utc::now()),
                    device::dr.eq(dr as i16),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, dr = dr, "Data-rate updated");
    Ok(d)
}

pub async fn set_status(
    dev_eui: &EUI64,
    margin: i32,
    external_power_source: bool,
    battery_level: Option<BigDecimal>,
) -> Result<Device, Error> {
    let d = task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<Device, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device::dsl::device.find(&dev_eui))
                .set((
                    device::margin.eq(Some(margin)),
                    device::external_power_source.eq(external_power_source),
                    device::battery_level.eq(battery_level),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, "Device status updated");
    Ok(d)
}

pub async fn delete(dev_eui: &EUI64) -> Result<(), Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(device::dsl::device.find(&dev_eui)).execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(dev_eui.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, "Device deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = device::dsl::device
                .select(dsl::count_star())
                .distinct()
                .left_join(multicast_group_device::table)
                .into_boxed();

            if let Some(application_id) = &filters.application_id {
                q = q.filter(device::dsl::application_id.eq(application_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(device::dsl::name.ilike(format!("%{}%", search)));
            }

            if let Some(multicast_group_id) = &filters.multicast_group_id {
                q = q
                    .filter(multicast_group_device::dsl::multicast_group_id.eq(multicast_group_id));
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
) -> Result<Vec<DeviceListItem>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<DeviceListItem>, Error> {
            let mut c = get_db_conn()?;
            let mut q = device::dsl::device
                .inner_join(device_profile::table)
                .left_join(multicast_group_device::table)
                .select((
                    device::dev_eui,
                    device::name,
                    device::description,
                    device_profile::id,
                    device_profile::name,
                    device::created_at,
                    device::updated_at,
                    device::last_seen_at,
                    device::margin,
                    device::external_power_source,
                    device::battery_level,
                ))
                .distinct()
                .into_boxed();

            if let Some(application_id) = &filters.application_id {
                q = q.filter(device::dsl::application_id.eq(application_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(device::dsl::name.ilike(format!("%{}%", search)));
            }

            if let Some(multicast_group_id) = &filters.multicast_group_id {
                q = q
                    .filter(multicast_group_device::dsl::multicast_group_id.eq(multicast_group_id));
            }

            q.order_by(device::dsl::name)
                .limit(limit)
                .offset(offset)
                .load(&mut c)
                .map_err(|e| Error::from_diesel(e, "".into()))
        }
    })
    .await?
}

pub async fn get_active_inactive(tenant_id: &Option<Uuid>) -> Result<DevicesActiveInactive, Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        move || -> Result<DevicesActiveInactive, Error> {
            let mut c = get_db_conn()?;
            diesel::sql_query(r#"
                with device_active_inactive as (
                    select
                        make_interval(secs => dp.uplink_interval) * 1.5 as uplink_interval,
                        d.last_seen_at as last_seen_at
                    from
                        device d
                    inner join device_profile dp
                        on d.device_profile_id = dp.id
                    where
                        $1 is null or dp.tenant_id = $1
                )
                select
                    coalesce(sum(case when last_seen_at is null then 1 end), 0) as never_seen_count,
                    coalesce(sum(case when (now() - uplink_interval) > last_seen_at then 1 end), 0) as inactive_count,
                    coalesce(sum(case when (now() - uplink_interval) <= last_seen_at then 1 end), 0) as active_count
                from
                    device_active_inactive
            "#)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Uuid>, _>(tenant_id)
            .get_result(&mut c)
            .map_err(|e| Error::from_diesel(e, "".into()))
        }
    })
    .await?
}

pub async fn get_data_rates(tenant_id: &Option<Uuid>) -> Result<Vec<DevicesDataRate>, Error> {
    task::spawn_blocking({
        let tenant_id = *tenant_id;
        move || -> Result<Vec<DevicesDataRate>, Error> {
            let mut c = get_db_conn()?;
            let mut q = device::dsl::device
                .inner_join(device_profile::table)
                //.select((device::dr, dsl::count_star()))
                .select((
                    device::dr,
                    diesel::dsl::sql::<diesel::sql_types::BigInt>("count(1)"),
                ))
                .group_by(device::dr)
                .filter(device::dsl::dr.is_not_null())
                .into_boxed();

            if let Some(id) = &tenant_id {
                q = q.filter(device_profile::dsl::tenant_id.eq(id));
            }

            q.load(&mut c).map_err(|e| Error::from_diesel(e, "".into()))
        }
    })
    .await?
}

pub async fn get_with_class_b_c_queue_items(limit: usize) -> Result<Vec<Device>> {
    task::spawn_blocking(move || -> Result<Vec<Device>> {
        let mut c = get_db_conn()?;
        c.transaction::<Vec<Device>, Error, _>(|c| {
            let conf = config::get();
            diesel::sql_query(
                r#"
                    update
                        device
                    set
                        scheduler_run_after = $3
                    where
                        dev_eui in (
                            select 
                                d.dev_eui
                            from
                                device d
                            where
                                d.enabled_class in ('B', 'C')
                                and (d.scheduler_run_after is null or d.scheduler_run_after < $2)
                                and exists (
                                    select
                                        1
                                    from
                                        device_queue_item dq
                                    where
                                        dq.dev_eui = d.dev_eui
                                        and not (
                                            -- pending queue-item with timeout_after in the future
                                            (dq.is_pending = true and dq.timeout_after > $2)
                                        )
                                )
                            order by d.dev_eui
                            limit $1
                        )
                    returning *
                "#,
            )
            .bind::<diesel::sql_types::Integer, _>(limit as i32)
            .bind::<diesel::sql_types::Timestamptz, _>(Utc::now())
            .bind::<diesel::sql_types::Timestamptz, _>(
                Utc::now() + Duration::from_std(2 * conf.network.scheduler.interval).unwrap(),
            )
            .load(c)
            .map_err(|e| Error::from_diesel(e, "".into()))
        })
        .context("Get with Class B/C queue-items transaction")
    })
    .await?
}

// This sets the lock. In case a lock was already set, it will be overwritten with the new TTL
// value.
pub async fn set_lock(dev_eui: &EUI64, ttl: Duration) -> Result<()> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<()> {
            info!(dev_eui = %dev_eui, "Setting device lock");
            let key = redis_key(format!("device:{{{}}}:lock", dev_eui));
            let mut c = get_redis_conn()?;

            redis::cmd("PSETEX")
                .arg(key)
                .arg(ttl.num_milliseconds())
                .arg("lock")
                .query(&mut *c)?;

            Ok(())
        }
    })
    .await?
}

// This sets the lock. In case a lock was already set, this function will return an error.
pub async fn get_lock(dev_eui: &EUI64, ttl: Duration) -> Result<(), Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<(), Error> {
            info!(dev_eui = %dev_eui, "Aquiring device lock");
            let key = redis_key(format!("device:{{{}}}:lock", dev_eui));
            let mut c = get_redis_conn()?;

            let set: bool = redis::cmd("SET")
                .arg(&key)
                .arg("lock")
                .arg("PX")
                .arg(ttl.num_milliseconds() as usize)
                .arg("NX")
                .query(&mut *c)?;

            if !set {
                return Err(Error::AlreadyExists(key));
            }

            Ok(())
        }
    })
    .await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage;
    use crate::storage::device_queue;
    use crate::test;

    struct FilterTest<'a> {
        filters: Filters,
        devs: Vec<&'a Device>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    pub async fn create_device(
        dev_eui: EUI64,
        device_profile_id: Uuid,
        application_id: Option<Uuid>,
    ) -> Device {
        let tenant_id = {
            let dp = storage::device_profile::get(&device_profile_id)
                .await
                .unwrap();
            dp.tenant_id
        };

        let application_id = match application_id {
            Some(v) => v,
            None => {
                let a = storage::application::test::create_application(Some(tenant_id)).await;
                a.id
            }
        };

        let d = Device {
            name: "test-dev".into(),
            dev_eui: dev_eui,
            application_id: application_id,
            device_profile_id: device_profile_id,
            ..Default::default()
        };

        create(d).await.unwrap()
    }

    #[tokio::test]
    async fn test_device() {
        let _guard = test::prepare().await;
        let dp = storage::device_profile::test::create_device_profile(None).await;
        let mut d =
            create_device(EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]), dp.id, None).await;

        // get
        let d_get = get(&d.dev_eui).await.unwrap();
        assert_eq!(d, d_get);

        // update
        d.name = "updated".into();
        d = update(d).await.unwrap();
        let d_get = get(&d.dev_eui).await.unwrap();
        assert_eq!(d, d_get);

        // get count and list
        let tests = vec![
            FilterTest {
                filters: Filters {
                    application_id: None,
                    multicast_group_id: None,
                    search: None,
                },
                devs: vec![&d],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: None,
                    multicast_group_id: None,
                    search: Some("uup".into()),
                },
                devs: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: None,
                    multicast_group_id: None,
                    search: Some("upd".into()),
                },
                devs: vec![&d],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: Some(d.application_id),
                    multicast_group_id: None,
                    search: None,
                },
                devs: vec![&d],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    application_id: Some(Uuid::new_v4()),
                    multicast_group_id: None,
                    search: None,
                },
                devs: vec![],
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
                tst.devs
                    .iter()
                    .map(|d| d.dev_eui.to_string())
                    .collect::<String>(),
                items
                    .iter()
                    .map(|d| d.dev_eui.to_string())
                    .collect::<String>()
            );
        }

        // delete
        delete(&d.dev_eui).await.unwrap();
        assert_eq!(true, delete(&d.dev_eui).await.is_err());
    }

    #[tokio::test]
    async fn test_get_with_class_b_c_queue_items() {
        let _guard = test::prepare().await;
        let dp = storage::device_profile::test::create_device_profile(None).await;
        let d = create_device(EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]), dp.id, None).await;

        // nothing in the queue
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        assert_eq!(0, res.len());

        // something in the queue but Class-A mode
        let mut qi = device_queue::enqueue_item(device_queue::DeviceQueueItem {
            dev_eui: d.dev_eui,
            f_port: 1,
            ..Default::default()
        })
        .await
        .unwrap();
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        assert_eq!(0, res.len());

        // device in Class-B.
        let d = set_enabled_class(&d.dev_eui, DeviceClass::B).await.unwrap();
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        let d = set_scheduler_run_after(&d.dev_eui, None).await.unwrap();
        assert_eq!(1, res.len());

        // device in Class-C
        let d = set_enabled_class(&d.dev_eui, DeviceClass::C).await.unwrap();
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        assert_eq!(1, res.len());

        // device in Class-C / scheduler_run_after is set (because of previous
        // get_with_class_b_c_queue_items run).
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        assert_eq!(0, res.len());

        // device in class C / downlink is pending.
        let _ = set_scheduler_run_after(&d.dev_eui, None).await.unwrap();
        qi.is_pending = true;
        qi.timeout_after = Some(Utc::now() + Duration::seconds(10));
        qi = device_queue::update_item(qi).await.unwrap();
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        assert_eq!(0, res.len());

        // device in class C / downlink is pending but has expired.
        qi.is_pending = true;
        qi.timeout_after = Some(Utc::now() - Duration::seconds(10));
        let _ = device_queue::update_item(qi).await.unwrap();
        let res = get_with_class_b_c_queue_items(10).await.unwrap();
        assert_eq!(1, res.len());
    }

    #[tokio::test]
    async fn test_get_set_lock() {
        let _guard = test::prepare().await;

        // This is okay, as we are overwriting the lock
        set_lock(
            &EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            Duration::seconds(1),
        )
        .await
        .unwrap();
        set_lock(
            &EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            Duration::seconds(1),
        )
        .await
        .unwrap();

        // This should fail as we are trying to aquire a lock,
        // but there is already a lock set.
        let res = get_lock(
            &EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
            Duration::seconds(1),
        )
        .await;
        assert!(res.is_err());

        get_lock(
            &EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2]),
            Duration::seconds(1),
        )
        .await
        .unwrap();
        let res = get_lock(
            &EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2]),
            Duration::seconds(1),
        )
        .await;
        assert!(res.is_err());
    }
}
