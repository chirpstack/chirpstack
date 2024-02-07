use std::collections::HashMap;
use std::fmt;
use std::io::Cursor;
use std::str::FromStr;

use anyhow::{Context, Result};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, Utc};
use diesel::{backend::Backend, deserialize, dsl, prelude::*, serialize, sql_types::Text};
use diesel_async::RunQueryDsl;
use prost::Message;
use tracing::info;
use uuid::Uuid;

use chirpstack_api::internal;
use lrwn::{DevAddr, EUI64};

use super::schema::{application, device, device_profile, multicast_group_device, tenant};
use super::{error::Error, fields, get_async_db_conn};
use crate::api::helpers::FromProto;
use crate::config;

pub enum ValidationStatus {
    Ok(u32, internal::DeviceSession),
    Retransmission(u32, internal::DeviceSession),
    Reset(u32, internal::DeviceSession),
}

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
    pub secondary_dev_addr: Option<DevAddr>,
    pub device_session: Option<Vec<u8>>,
}

#[derive(AsChangeset, Debug, Clone, Default)]
#[diesel(table_name = device)]
pub struct DeviceChangeset {
    pub last_seen_at: Option<Option<DateTime<Utc>>>,
    pub dr: Option<Option<i16>>,
    pub dev_addr: Option<Option<DevAddr>>,
    pub enabled_class: Option<DeviceClass>,
    pub join_eui: Option<EUI64>,
    pub secondary_dev_addr: Option<Option<DevAddr>>,
    pub device_session: Option<Option<Vec<u8>>>,
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
            secondary_dev_addr: None,
            device_session: None,
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
    let mut c = get_async_db_conn().await?;
    let d: Device = c
        .build_transaction()
        .run::<Device, Error, _>(|c| {
            Box::pin(async move {
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
                        tenant::dsl::tags,
                    ))
                    .inner_join(application::table)
                    .filter(application::dsl::id.eq(&d.application_id))
                    .for_update()
                    .first(c)
                    .await?;

                let dev_count: i64 = device::dsl::device
                    .select(dsl::count_star())
                    .inner_join(application::table)
                    .filter(application::dsl::tenant_id.eq(&t.id))
                    .first(c)
                    .await?;

                if t.max_device_count != 0 && dev_count as i32 >= t.max_device_count {
                    return Err(Error::NotAllowed(
                        "Max number of devices exceeded for tenant".into(),
                    ));
                }

                diesel::insert_into(device::table)
                    .values(&d)
                    .get_result(c)
                    .await
                    .map_err(|e| Error::from_diesel(e, d.dev_eui.to_string()))
            })
        })
        .await?;
    info!(dev_eui = %d.dev_eui, "Device created");
    Ok(d)
}

pub async fn get(dev_eui: &EUI64) -> Result<Device, Error> {
    let d = device::dsl::device
        .find(&dev_eui)
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
    Ok(d)
}

// Return the device-session matching the given PhyPayload. This will fetch all device-session
// associated with the used DevAddr and based on f_cont and mic, decides which one to use.
// This function will increment the uplink frame-counter and will immediately update the
// device-session in the database, to make sure that in case this function is called multiple
// times, at most one will be valid.
// On Ok response, the PhyPayload f_cnt will be set to the full 32bit frame-counter based on the
// device-session context.
pub async fn get_for_phypayload_and_incr_f_cnt_up(
    relayed: bool,
    phy: &mut lrwn::PhyPayload,
    tx_dr: u8,
    tx_ch: u8,
) -> Result<ValidationStatus, Error> {
    let mut dev_addr = lrwn::DevAddr::from_be_bytes([0x00, 0x00, 0x00, 0x00]);
    let mut f_cnt_orig = 0;

    // Get the dev_addr and original f_cnt.
    if let lrwn::Payload::MACPayload(pl) = &phy.payload {
        dev_addr = pl.fhdr.devaddr;
        f_cnt_orig = pl.fhdr.f_cnt;
    } else {
        return Err(Error::InvalidPayload("MacPayload".to_string()));
    }

    let mut c = get_async_db_conn().await?;

    c.build_transaction()
        .run::<ValidationStatus, Error, _>(|c| {
            Box::pin(async move {
                let devices: Vec<(EUI64, Option<Vec<u8>>)> = device::dsl::device
                    .select((device::dev_eui, device::device_session))
                    .filter(
                        device::dsl::dev_addr
                            .eq(&dev_addr)
                            .or(device::dsl::secondary_dev_addr.eq(&dev_addr)),
                    )
                    .for_update()
                    .load(c)
                    .await?;

                for d in &devices {
                    if d.1.is_none() {
                        continue;
                    }

                    let mut ds =
                        internal::DeviceSession::decode(&mut Cursor::new(d.1.as_ref().unwrap()))?;

                    // Get the full 32bit frame-counter.
                    let full_f_cnt = get_full_f_cnt_up(ds.f_cnt_up, f_cnt_orig);
                    let f_nwk_s_int_key = lrwn::AES128Key::from_slice(&ds.f_nwk_s_int_key)?;
                    let s_nwk_s_int_key = lrwn::AES128Key::from_slice(&ds.s_nwk_s_int_key)?;

                    // Check both the full frame-counter and the received frame-counter
                    // truncated to the 16LSB.
                    // The latter is needed in case of a frame-counter reset as the
                    // GetFullFCntUp will think the 16LSB has rolled over and will
                    // increment the 16MSB bit.
                    let mut mic_ok = false;
                    for f_cnt in [full_f_cnt, f_cnt_orig] {
                        // Set the full f_cnt.
                        if let lrwn::Payload::MACPayload(pl) = &mut phy.payload {
                            pl.fhdr.f_cnt = f_cnt;
                        }

                        mic_ok = phy
                            .validate_uplink_data_mic(
                                ds.mac_version().from_proto(),
                                ds.conf_f_cnt,
                                tx_dr,
                                tx_ch,
                                &f_nwk_s_int_key,
                                &s_nwk_s_int_key,
                            )
                            .context("Validate MIC")?;

                        if mic_ok {
                            break;
                        }
                    }

                    if mic_ok {
                        let full_f_cnt = if let lrwn::Payload::MACPayload(pl) = &phy.payload {
                            pl.fhdr.f_cnt
                        } else {
                            0
                        };

                        if let Some(relay) = &ds.relay {
                            if !relayed && relay.ed_relay_only {
                                info!(
                                    dev_eui = %d.0,
                                    "Only communication through relay is allowed"
                                );
                                return Err(Error::NotFound(dev_addr.to_string()));
                            }
                        }

                        if full_f_cnt >= ds.f_cnt_up {
                            // We immediately save the device-session to make sure that concurrent calls for
                            // the same uplink will fail on the frame-counter validation.
                            let ds_f_cnt_up = ds.f_cnt_up;
                            ds.f_cnt_up = full_f_cnt + 1;

                            let _ = diesel::update(device::dsl::device.find(&d.0))
                                .set(device::device_session.eq(&ds.encode_to_vec()))
                                .execute(c)
                                .await?;

                            // We do return the device-session with original frame-counter
                            ds.f_cnt_up = ds_f_cnt_up;

                            return Ok(ValidationStatus::Ok(full_f_cnt, ds));
                        } else if ds.skip_f_cnt_check {
                            // re-transmission or frame-counter reset
                            ds.f_cnt_up = 0;
                            return Ok(ValidationStatus::Ok(full_f_cnt, ds));
                        } else if full_f_cnt == (ds.f_cnt_up - 1) {
                            // re-transmission, the frame-counter did not increment
                            return Ok(ValidationStatus::Retransmission(full_f_cnt, ds));
                        } else {
                            return Ok(ValidationStatus::Reset(full_f_cnt, ds));
                        }
                    }

                    // Restore the original f_cnt.
                    if let lrwn::Payload::MACPayload(pl) = &mut phy.payload {
                        pl.fhdr.f_cnt = f_cnt_orig;
                    }
                }

                Err(Error::InvalidMIC)
            })
        })
        .await
}

pub async fn update(d: Device) -> Result<Device, Error> {
    d.validate()?;

    let d: Device = diesel::update(device::dsl::device.find(&d.dev_eui))
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
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, d.dev_eui.to_string()))?;
    info!(dev_eui = %d.dev_eui, "Device updated");
    Ok(d)
}

pub async fn partial_update(dev_eui: EUI64, d: &DeviceChangeset) -> Result<Device, Error> {
    let d = diesel::update(device::dsl::device.find(&dev_eui))
        .set(d)
        .get_result::<Device>(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;

    info!(dev_eui = %dev_eui, "Device partially updated");
    Ok(d)
}

pub async fn set_enabled_class(dev_eui: &EUI64, mode: DeviceClass) -> Result<Device, Error> {
    let d: Device = diesel::update(device::dsl::device.find(&dev_eui))
        .set(device::enabled_class.eq(&mode))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
    info!(dev_eui = %dev_eui, enabled_class = %mode, "Enabled class updated");
    Ok(d)
}

pub async fn set_join_eui(dev_eui: EUI64, join_eui: EUI64) -> Result<Device, Error> {
    let d: Device = diesel::update(device::dsl::device.find(&dev_eui))
        .set(device::join_eui.eq(&join_eui))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
    info!(dev_eui = %dev_eui, join_eui = %join_eui, "Updated JoinEUI");
    Ok(d)
}

pub async fn set_dev_addr(dev_eui: EUI64, dev_addr: DevAddr) -> Result<Device, Error> {
    let d: Device = diesel::update(device::dsl::device.find(&dev_eui))
        .set(device::dev_addr.eq(&dev_addr))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
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
    diesel::update(device::dsl::device.find(&dev_eui))
        .set(device::scheduler_run_after.eq(&new_ts))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
}

pub async fn set_last_seen_dr(dev_eui: &EUI64, dr: u8) -> Result<Device, Error> {
    let d: Device = diesel::update(device::dsl::device.find(&dev_eui))
        .set((
            device::last_seen_at.eq(Utc::now()),
            device::dr.eq(dr as i16),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
    info!(dev_eui = %dev_eui, dr = dr, "Data-rate updated");
    Ok(d)
}

pub async fn set_status(
    dev_eui: &EUI64,
    margin: i32,
    external_power_source: bool,
    battery_level: Option<BigDecimal>,
) -> Result<Device, Error> {
    let d: Device = diesel::update(device::dsl::device.find(&dev_eui))
        .set((
            device::margin.eq(Some(margin)),
            device::external_power_source.eq(external_power_source),
            device::battery_level.eq(battery_level),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
    info!(dev_eui = %dev_eui, "Device status updated");
    Ok(d)
}

pub async fn delete(dev_eui: &EUI64) -> Result<(), Error> {
    let ra = diesel::delete(device::dsl::device.find(&dev_eui))
        .execute(&mut get_async_db_conn().await?)
        .await?;
    if ra == 0 {
        return Err(Error::NotFound(dev_eui.to_string()));
    }
    info!(dev_eui = %dev_eui, "Device deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
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
        q = q.filter(multicast_group_device::dsl::multicast_group_id.eq(multicast_group_id));
    }

    Ok(q.first(&mut get_async_db_conn().await?).await?)
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
) -> Result<Vec<DeviceListItem>, Error> {
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
        q = q.filter(multicast_group_device::dsl::multicast_group_id.eq(multicast_group_id));
    }

    q.order_by(device::dsl::name)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn get_active_inactive(tenant_id: &Option<Uuid>) -> Result<DevicesActiveInactive, Error> {
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
    .get_result(&mut get_async_db_conn().await?).await
    .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn get_data_rates(tenant_id: &Option<Uuid>) -> Result<Vec<DevicesDataRate>, Error> {
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

    q.load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn get_with_class_b_c_queue_items(limit: usize) -> Result<Vec<Device>> {
    let mut c = get_async_db_conn().await?;
    c.build_transaction()
        .run::<Vec<Device>, Error, _>(|c| {
            Box::pin(async {
                let conf = config::get();

                // This query will:
                //  * Select the devices for which a Class-B or Class-C downlink can be scheduled.
                //  * Lock the device records for update with skip locked such that other
                //    ChirpStack instances are able to do the same for the remaining devices.
                //  * Update the scheduler_run_after for these devices to now() + 2 * scheduler
                //    interval to avoid concurrency issues (other ChirpStack instance scheduling
                //    the same queue items).
                //
                // This way, we do not have to keep the device records locked until the scheduler
                // finishes its batch as the same set of devices will not be returned until after
                // the updated scheduler_run_after. Only if the scheduler takes more time than 2x the
                // interval (the scheduler is still working on processing the batch after 2 x interval)
                // this might cause issues.
                // The alternative would be to keep the transaction open for a long time + keep
                // the device records locked during this time which could case issues as well.
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
                            for update skip locked
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
                .await
                .map_err(|e| Error::from_diesel(e, "".into()))
            })
        })
        .await
        .context("Get with Class B/C queue-items transaction")
}

// GetFullFCntUp returns the full 32bit frame-counter, given the fCntUp which
// has been truncated to the last 16 LSB.
// Notes:
// * After a succesful validation of the FCntUp and the MIC, don't forget
//   to synchronize the device FCntUp with the packet FCnt.
// * In case of a frame-counter rollover, the returned values will be less
//   than the given DeviceSession FCntUp. This must be validated outside this
//   function!
// * In case of a re-transmission, the returned frame-counter equals
//   DeviceSession.FCntUp - 1, as the FCntUp value holds the next expected
//   frame-counter, not the FCntUp which was last seen.
fn get_full_f_cnt_up(next_expected_full_fcnt: u32, truncated_f_cnt: u32) -> u32 {
    // Handle re-transmission.
    if truncated_f_cnt == (((next_expected_full_fcnt % (1 << 16)) as u16).wrapping_sub(1)) as u32 {
        return next_expected_full_fcnt - 1;
    }

    let gap = ((truncated_f_cnt as u16).wrapping_sub((next_expected_full_fcnt % (1 << 16)) as u16))
        as u32;

    next_expected_full_fcnt.wrapping_add(gap)
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
}
