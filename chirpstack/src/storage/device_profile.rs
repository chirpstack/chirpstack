use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{dsl, prelude::*};
use diesel_async::RunQueryDsl;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use chirpstack_api::internal;
use lrwn::region::{CommonName, MacVersion, Revision};

use super::error::Error;
use super::schema::{device_profile, device_profile_device, device_profile_vendor};
use super::{error, fields, get_async_db_conn};
use crate::api::helpers::ToProto;
use crate::codec::Codec;

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = device_profile_vendor)]
pub struct Vendor {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub vendor_id: i32,
    pub ouis: Vec<Option<String>>,
    pub metadata: fields::KeyValue,
}

impl Default for Vendor {
    fn default() -> Self {
        Vendor {
            id: Uuid::new_v4().into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: "".into(),
            vendor_id: 0,
            ouis: vec![],
            metadata: fields::KeyValue::new(HashMap::new()),
        }
    }
}

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = device_profile_device)]
pub struct Device {
    pub id: fields::Uuid,
    pub vendor_id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub metadata: fields::KeyValue,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            id: Uuid::new_v4().into(),
            vendor_id: Uuid::nil().into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: "".into(),
            description: "".into(),
            metadata: fields::KeyValue::new(HashMap::new()),
        }
    }
}

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq, Validate)]
#[diesel(table_name = device_profile)]
pub struct DeviceProfile {
    pub id: fields::Uuid,
    pub tenant_id: Option<fields::Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[validate(length(min = 2))]
    pub name: String,
    pub region: CommonName,
    pub mac_version: MacVersion,
    pub reg_params_revision: Revision,
    pub adr_algorithm_id: String,
    pub payload_codec_runtime: Codec,
    pub uplink_interval: i32,
    pub device_status_req_interval: i32,
    pub supports_otaa: bool,
    pub supports_class_b: bool,
    pub supports_class_c: bool,
    pub tags: fields::KeyValue,
    pub payload_codec_script: String,
    pub flush_queue_on_activate: bool,
    pub description: String,
    pub measurements: fields::Measurements,
    pub auto_detect_measurements: bool,
    pub region_config_id: Option<String>,
    pub allow_roaming: bool,
    #[validate(range(min = 0, max = 15))]
    pub rx1_delay: i16,
    pub abp_params: Option<fields::AbpParams>,
    pub class_b_params: Option<fields::ClassBParams>,
    pub class_c_params: Option<fields::ClassCParams>,
    pub relay_params: Option<fields::RelayParams>,
    pub app_layer_params: fields::AppLayerParams,
    pub device_id: Option<fields::Uuid>,
    pub firmware_version: String,
    pub vendor_profile_id: i32,
}

impl Default for DeviceProfile {
    fn default() -> Self {
        let now = Utc::now();

        DeviceProfile {
            id: Uuid::new_v4().into(),
            tenant_id: None,
            created_at: now,
            updated_at: now,
            name: "".into(),
            description: "".into(),
            region: CommonName::EU868,
            mac_version: MacVersion::LORAWAN_1_0_0,
            reg_params_revision: Revision::A,
            adr_algorithm_id: "".into(),
            payload_codec_runtime: Codec::NONE,
            payload_codec_script: "".into(),
            flush_queue_on_activate: false,
            uplink_interval: 0,
            device_status_req_interval: 0,
            supports_otaa: false,
            supports_class_b: false,
            supports_class_c: false,
            tags: fields::KeyValue::new(HashMap::new()),
            measurements: fields::Measurements::new(HashMap::new()),
            auto_detect_measurements: false,
            region_config_id: None,
            allow_roaming: false,
            rx1_delay: 0,
            abp_params: None,
            class_b_params: None,
            class_c_params: None,
            relay_params: None,
            app_layer_params: fields::AppLayerParams::default(),
            device_id: None,
            firmware_version: "".into(),
            vendor_profile_id: 0,
        }
    }
}

impl DeviceProfile {
    pub fn reset_session_to_boot_params(&self, ds: &mut internal::DeviceSession) {
        ds.mac_version = self.mac_version.to_proto().into();
        ds.nb_trans = 1;

        if let Some(class_b_params) = &self.class_b_params {
            ds.class_b_ping_slot_dr = class_b_params.ping_slot_dr as u32;
            ds.class_b_ping_slot_freq = class_b_params.ping_slot_freq;
            ds.class_b_ping_slot_nb = 1 << (7 - class_b_params.ping_slot_periodicity) as u32;
        }

        if let Some(relay_params) = &self.relay_params {
            if relay_params.is_relay_ed {
                ds.relay = Some(internal::Relay {
                    ed_relay_only: relay_params.ed_relay_only,
                    ..Default::default()
                });
            }
        }

        if !self.supports_otaa {
            ds.tx_power_index = 0;
            ds.min_supported_tx_power_index = 0;
            ds.max_supported_tx_power_index = 0;
            ds.extra_uplink_channels = HashMap::new();
            ds.enabled_uplink_channel_indices = Vec::new();

            if let Some(abp_params) = &self.abp_params {
                ds.rx1_delay = abp_params.rx1_delay as u32;
                ds.rx1_dr_offset = abp_params.rx1_dr_offset as u32;
                ds.rx2_dr = abp_params.rx2_dr as u32;
                ds.rx2_frequency = abp_params.rx2_freq;
            }
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct DeviceProfileListItem {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub region: CommonName,
    pub mac_version: MacVersion,
    pub reg_params_revision: Revision,
    pub supports_otaa: bool,
    pub supports_class_b: bool,
    pub supports_class_c: bool,
}

#[derive(Default, Clone)]
pub struct Filters {
    pub tenant_id: Option<Uuid>,
    pub search: Option<String>,
    pub device_id: Option<Uuid>,
    pub global_only: bool,
}

pub async fn create(dp: DeviceProfile) -> Result<DeviceProfile, Error> {
    dp.validate()
        .map_err(|e| Error::Validation(e.to_string()))?;

    let dp: DeviceProfile = diesel::insert_into(device_profile::table)
        .values(&dp)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| error::Error::from_diesel(e, dp.id.to_string()))?;
    info!(id = %dp.id, "Device-profile created");
    Ok(dp)
}

pub async fn get(id: &Uuid) -> Result<DeviceProfile, Error> {
    let dp = device_profile::dsl::device_profile
        .find(&fields::Uuid::from(id))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| error::Error::from_diesel(e, id.to_string()))?;
    Ok(dp)
}

pub async fn upsert(dp: DeviceProfile) -> Result<DeviceProfile, Error> {
    dp.validate()
        .map_err(|e| Error::Validation(e.to_string()))?;

    let dp: DeviceProfile = diesel::insert_into(device_profile::table)
        .values(&dp)
        .on_conflict(device_profile::id)
        .do_update()
        .set((
            device_profile::updated_at.eq(Utc::now()),
            device_profile::name.eq(&dp.name),
            device_profile::description.eq(&dp.description),
            device_profile::region.eq(&dp.region),
            device_profile::mac_version.eq(&dp.mac_version),
            device_profile::reg_params_revision.eq(&dp.reg_params_revision),
            device_profile::adr_algorithm_id.eq(&dp.adr_algorithm_id),
            device_profile::payload_codec_runtime.eq(&dp.payload_codec_runtime),
            device_profile::payload_codec_script.eq(&dp.payload_codec_script),
            device_profile::flush_queue_on_activate.eq(&dp.flush_queue_on_activate),
            device_profile::uplink_interval.eq(&dp.uplink_interval),
            device_profile::device_status_req_interval.eq(&dp.device_status_req_interval),
            device_profile::supports_otaa.eq(&dp.supports_otaa),
            device_profile::supports_class_b.eq(&dp.supports_class_b),
            device_profile::supports_class_c.eq(&dp.supports_class_c),
            device_profile::tags.eq(&dp.tags),
            device_profile::measurements.eq(&dp.measurements),
            device_profile::auto_detect_measurements.eq(&dp.auto_detect_measurements),
            device_profile::region_config_id.eq(&dp.region_config_id),
            device_profile::allow_roaming.eq(&dp.allow_roaming),
            device_profile::rx1_delay.eq(&dp.rx1_delay),
            device_profile::abp_params.eq(&dp.abp_params),
            device_profile::class_b_params.eq(&dp.class_b_params),
            device_profile::class_c_params.eq(&dp.class_c_params),
            device_profile::relay_params.eq(&dp.relay_params),
            device_profile::app_layer_params.eq(&dp.app_layer_params),
            device_profile::device_id.eq(&dp.device_id),
            device_profile::firmware_version.eq(&dp.firmware_version),
            device_profile::vendor_profile_id.eq(&dp.vendor_profile_id),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dp.id.to_string()))?;

    info!(id = %dp.id, "Device-profile upserted");

    Ok(dp)
}

pub async fn update(dp: DeviceProfile) -> Result<DeviceProfile, Error> {
    dp.validate()
        .map_err(|e| Error::Validation(e.to_string()))?;

    let dp: DeviceProfile = diesel::update(device_profile::dsl::device_profile.find(&dp.id))
        .set((
            device_profile::updated_at.eq(Utc::now()),
            device_profile::name.eq(&dp.name),
            device_profile::description.eq(&dp.description),
            device_profile::region.eq(&dp.region),
            device_profile::mac_version.eq(&dp.mac_version),
            device_profile::reg_params_revision.eq(&dp.reg_params_revision),
            device_profile::adr_algorithm_id.eq(&dp.adr_algorithm_id),
            device_profile::payload_codec_runtime.eq(&dp.payload_codec_runtime),
            device_profile::payload_codec_script.eq(&dp.payload_codec_script),
            device_profile::flush_queue_on_activate.eq(&dp.flush_queue_on_activate),
            device_profile::uplink_interval.eq(&dp.uplink_interval),
            device_profile::device_status_req_interval.eq(&dp.device_status_req_interval),
            device_profile::supports_otaa.eq(&dp.supports_otaa),
            device_profile::supports_class_b.eq(&dp.supports_class_b),
            device_profile::supports_class_c.eq(&dp.supports_class_c),
            device_profile::tags.eq(&dp.tags),
            device_profile::measurements.eq(&dp.measurements),
            device_profile::auto_detect_measurements.eq(&dp.auto_detect_measurements),
            device_profile::region_config_id.eq(&dp.region_config_id),
            device_profile::allow_roaming.eq(&dp.allow_roaming),
            device_profile::rx1_delay.eq(&dp.rx1_delay),
            device_profile::abp_params.eq(&dp.abp_params),
            device_profile::class_b_params.eq(&dp.class_b_params),
            device_profile::class_c_params.eq(&dp.class_c_params),
            device_profile::relay_params.eq(&dp.relay_params),
            device_profile::app_layer_params.eq(&dp.app_layer_params),
            device_profile::device_id.eq(&dp.device_id),
            device_profile::firmware_version.eq(&dp.firmware_version),
            device_profile::vendor_profile_id.eq(&dp.vendor_profile_id),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| error::Error::from_diesel(e, dp.id.to_string()))?;

    info!(id = %dp.id, "Device-profile updated");
    Ok(dp)
}

pub async fn set_measurements(id: Uuid, m: &fields::Measurements) -> Result<DeviceProfile, Error> {
    let dp: DeviceProfile =
        diesel::update(device_profile::dsl::device_profile.find(&fields::Uuid::from(id)))
            .set(device_profile::measurements.eq(m))
            .get_result(&mut get_async_db_conn().await?)
            .await
            .map_err(|e| Error::from_diesel(e, id.to_string()))?;
    info!(id = %id, "Device-profile measurements updated");
    Ok(dp)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    let ra = diesel::delete(device_profile::dsl::device_profile.find(&fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await?;
    if ra == 0 {
        return Err(error::Error::NotFound(id.to_string()));
    }
    info!(id = %id, "Device-profile deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    let mut q = device_profile::dsl::device_profile
        .select(dsl::count_star())
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(device_profile::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    if let Some(device_id) = &filters.device_id {
        q = q.filter(device_profile::dsl::device_id.eq(fields::Uuid::from(device_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(device_profile::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(device_profile::dsl::name.like(format!("%{}%", search)));
        }
    }

    Ok(q.first(&mut get_async_db_conn().await?).await?)
}

pub async fn list(
    limit: i64,
    offset: i64,
    filters: &Filters,
) -> Result<Vec<DeviceProfileListItem>, Error> {
    let mut q = device_profile::dsl::device_profile
        .select((
            device_profile::id,
            device_profile::created_at,
            device_profile::updated_at,
            device_profile::name,
            device_profile::region,
            device_profile::mac_version,
            device_profile::reg_params_revision,
            device_profile::supports_otaa,
            device_profile::supports_class_b,
            device_profile::supports_class_c,
        ))
        .into_boxed();

    if let Some(tenant_id) = &filters.tenant_id {
        q = q.filter(device_profile::dsl::tenant_id.eq(fields::Uuid::from(tenant_id)));
    }

    if let Some(device_id) = &filters.device_id {
        q = q.filter(device_profile::dsl::device_id.eq(fields::Uuid::from(device_id)));
    }

    if let Some(search) = &filters.search {
        #[cfg(feature = "postgres")]
        {
            q = q.filter(device_profile::dsl::name.ilike(format!("%{}%", search)));
        }
        #[cfg(feature = "sqlite")]
        {
            q = q.filter(device_profile::dsl::name.like(format!("%{}%", search)));
        }
    }

    let items = q
        .order_by(device_profile::dsl::name)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await?;
    Ok(items)
}

pub async fn upsert_vendor(v: Vendor) -> Result<Vendor, Error> {
    let v: Vendor = diesel::insert_into(device_profile_vendor::table)
        .values(&v)
        .on_conflict(device_profile_vendor::id)
        .do_update()
        .set((
            device_profile_vendor::updated_at.eq(Utc::now()),
            device_profile_vendor::name.eq(&v.name),
            device_profile_vendor::vendor_id.eq(&v.vendor_id),
            device_profile_vendor::ouis.eq(&v.ouis),
            device_profile_vendor::metadata.eq(&v.metadata),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| error::Error::from_diesel(e, v.id.to_string()))?;

    info!(id = %v.id, "Device-profile vendor upserted");

    Ok(v)
}

pub async fn list_vendors() -> Result<Vec<Vendor>, Error> {
    Ok(device_profile_vendor::table
        .order_by(device_profile_vendor::name)
        .load(&mut get_async_db_conn().await?)
        .await?)
}

pub async fn list_devices(vendor_id: Uuid) -> Result<Vec<Device>, Error> {
    Ok(device_profile_device::table
        .filter(device_profile_device::vendor_id.eq(fields::Uuid::from(vendor_id)))
        .order_by(device_profile_device::name)
        .load(&mut get_async_db_conn().await?)
        .await?)
}

pub async fn delete_vendor(id: uuid::Uuid) -> Result<(), Error> {
    let ra = diesel::delete(device_profile_vendor::table.find(&fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await?;

    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }

    info!(id = %id, "Device-profile vendor deleted");
    Ok(())
}

pub async fn upsert_device(d: Device) -> Result<Device, Error> {
    let d: Device = diesel::insert_into(device_profile_device::table)
        .values(&d)
        .on_conflict(device_profile_device::id)
        .do_update()
        .set((
            device_profile_device::updated_at.eq(Utc::now()),
            device_profile_device::name.eq(&d.name),
            device_profile_device::description.eq(&d.name),
            device_profile_device::metadata.eq(&d.metadata),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, d.id.to_string()))?;

    info!(id = %d.id, "Device-profile device upserted");

    Ok(d)
}

pub async fn delete_device(id: uuid::Uuid) -> Result<(), Error> {
    let ra = diesel::delete(device_profile_device::table.find(&fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await?;

    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }

    info!(id = %id, "Device-profile device deleted");
    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage;
    use crate::test;

    struct FilterTest<'a> {
        filters: Filters,
        dps: Vec<&'a DeviceProfile>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    pub async fn create_device_profile(tenant_id: Option<Uuid>) -> DeviceProfile {
        let tenant_id = match tenant_id {
            Some(v) => v.into(),
            None => {
                let t = storage::tenant::test::create_tenant().await;
                t.id
            }
        };

        let mut kv = HashMap::new();
        kv.insert("foo".into(), "bar".into());

        let dp = DeviceProfile {
            tenant_id: Some(tenant_id),
            name: "test device-profile".into(),
            region: CommonName::EU868,
            mac_version: MacVersion::LORAWAN_1_0_2,
            reg_params_revision: Revision::B,
            adr_algorithm_id: "default".into(),
            payload_codec_runtime: Codec::JS,
            uplink_interval: 60,
            supports_otaa: true,
            tags: fields::KeyValue::new(kv),
            ..Default::default()
        };

        create(dp).await.unwrap()
    }

    #[tokio::test]
    async fn test_device_profile() {
        let _guard = test::prepare().await;
        let mut dp = create_device_profile(None).await;

        // get
        let dp_get = get(&dp.id).await.unwrap();
        assert_eq!(dp, dp_get);

        // update
        dp.name = "update device-profile".into();
        dp = update(dp).await.unwrap();
        let dp_get = get(&dp.id).await.unwrap();
        assert_eq!(dp, dp_get);

        // get count and list
        let tests = vec![
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: None,
                    device_id: None,
                    global_only: false,
                },
                dps: vec![&dp],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: Some("proof".into()),
                    device_id: None,
                    global_only: false,
                },
                dps: vec![],
                count: 0,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: None,
                    search: Some("prof".into()),
                    device_id: None,
                    global_only: false,
                },
                dps: vec![&dp],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(dp.tenant_id.unwrap().into()),
                    search: None,
                    device_id: None,
                    global_only: false,
                },
                dps: vec![&dp],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(Uuid::new_v4()),
                    search: None,
                    device_id: None,
                    global_only: false,
                },
                dps: vec![],
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
                tst.dps
                    .iter()
                    .map(|dp| { dp.id.to_string() })
                    .collect::<String>(),
                items
                    .iter()
                    .map(|dp| { dp.id.to_string() })
                    .collect::<String>()
            );
        }

        // delete
        delete(&dp.id).await.unwrap();
        assert!(delete(&dp.id).await.is_err());
    }
}
