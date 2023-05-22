use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use tokio::task;
use tracing::info;
use uuid::Uuid;

use lrwn::region::{CommonName, MacVersion, Revision};

use super::error::Error;
use super::schema::device_profile;
use super::{error, fields, get_db_conn};
use crate::api::helpers::ToProto;
use crate::codec::Codec;
use chirpstack_api::internal;

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = device_profile)]
pub struct DeviceProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub class_b_timeout: i32,
    pub class_b_ping_slot_nb_k: i32,
    pub class_b_ping_slot_dr: i16,
    pub class_b_ping_slot_freq: i64,
    pub class_c_timeout: i32,
    pub abp_rx1_delay: i16,
    pub abp_rx1_dr_offset: i16,
    pub abp_rx2_dr: i16,
    pub abp_rx2_freq: i64,
    pub tags: fields::KeyValue,
    pub payload_codec_script: String,
    pub flush_queue_on_activate: bool,
    pub description: String,
    pub measurements: fields::Measurements,
    pub auto_detect_measurements: bool,
    pub region_config_id: Option<String>,
    pub is_relay: bool,
    pub is_relay_ed: bool,
    pub relay_ed_relay_only: bool,
    pub relay_enabled: bool,
    pub relay_cad_periodicity: i16,
    pub relay_default_channel_index: i16,
    pub relay_second_channel_freq: i64,
    pub relay_second_channel_dr: i16,
    pub relay_second_channel_ack_offset: i16,
    pub relay_ed_activation_mode: lrwn::RelayModeActivation,
    pub relay_ed_smart_enable_level: i16,
    pub relay_ed_back_off: i16,
    pub relay_ed_uplink_limit_bucket_size: i16,
    pub relay_ed_uplink_limit_reload_rate: i16,
    pub relay_join_req_limit_reload_rate: i16,
    pub relay_notify_limit_reload_rate: i16,
    pub relay_global_uplink_limit_reload_rate: i16,
    pub relay_overall_limit_reload_rate: i16,
    pub relay_join_req_limit_bucket_size: i16,
    pub relay_notify_limit_bucket_size: i16,
    pub relay_global_uplink_limit_bucket_size: i16,
    pub relay_overall_limit_bucket_size: i16,
}

impl DeviceProfile {
    fn validate(&self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }
        Ok(())
    }
}

impl Default for DeviceProfile {
    fn default() -> Self {
        let now = Utc::now();

        DeviceProfile {
            id: Uuid::new_v4(),
            tenant_id: Uuid::nil(),
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
            class_b_timeout: 0,
            class_b_ping_slot_nb_k: 0,
            class_b_ping_slot_dr: 0,
            class_b_ping_slot_freq: 0,
            class_c_timeout: 0,
            abp_rx1_delay: 0,
            abp_rx1_dr_offset: 0,
            abp_rx2_dr: 0,
            abp_rx2_freq: 0,
            tags: fields::KeyValue::new(HashMap::new()),
            measurements: fields::Measurements::new(HashMap::new()),
            auto_detect_measurements: false,
            region_config_id: None,
            is_relay: false,
            is_relay_ed: false,
            relay_ed_relay_only: false,
            relay_enabled: false,
            relay_cad_periodicity: 0,
            relay_default_channel_index: 0,
            relay_second_channel_freq: 0,
            relay_second_channel_dr: 0,
            relay_second_channel_ack_offset: 0,
            relay_ed_activation_mode: lrwn::RelayModeActivation::DisableRelayMode,
            relay_ed_smart_enable_level: 0,
            relay_ed_back_off: 0,
            relay_ed_uplink_limit_bucket_size: 0,
            relay_ed_uplink_limit_reload_rate: 0,
            relay_join_req_limit_reload_rate: 0,
            relay_notify_limit_reload_rate: 0,
            relay_global_uplink_limit_reload_rate: 0,
            relay_overall_limit_reload_rate: 0,
            relay_join_req_limit_bucket_size: 0,
            relay_notify_limit_bucket_size: 0,
            relay_global_uplink_limit_bucket_size: 0,
            relay_overall_limit_bucket_size: 0,
        }
    }
}

impl DeviceProfile {
    pub fn reset_session_to_boot_params(&self, ds: &mut internal::DeviceSession) {
        ds.mac_version = self.mac_version.to_proto().into();
        ds.class_b_ping_slot_dr = self.class_b_ping_slot_dr as u32;
        ds.class_b_ping_slot_freq = self.class_b_ping_slot_freq as u32;
        ds.class_b_ping_slot_nb = 1 << self.class_b_ping_slot_nb_k as u32;
        ds.nb_trans = 1;

        if self.is_relay_ed {
            ds.relay = Some(internal::Relay {
                ed_relay_only: self.relay_ed_relay_only,
                ..Default::default()
            });
        }

        if !self.supports_otaa {
            ds.tx_power_index = 0;
            ds.min_supported_tx_power_index = 0;
            ds.max_supported_tx_power_index = 0;
            ds.extra_uplink_channels = HashMap::new();
            ds.rx1_delay = self.abp_rx1_delay as u32;
            ds.rx1_dr_offset = self.abp_rx1_dr_offset as u32;
            ds.rx2_dr = self.abp_rx2_dr as u32;
            ds.rx2_frequency = self.abp_rx2_freq as u32;
            ds.enabled_uplink_channel_indices = Vec::new();
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct DeviceProfileListItem {
    pub id: Uuid,
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
}

pub async fn create(dp: DeviceProfile) -> Result<DeviceProfile, Error> {
    dp.validate()?;
    let dp = task::spawn_blocking({
        move || -> Result<DeviceProfile, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(device_profile::table)
                .values(&dp)
                .get_result(&mut c)
                .map_err(|e| error::Error::from_diesel(e, dp.id.to_string()))
        }
    })
    .await??;
    info!(id = %dp.id, "Device-profile created");
    Ok(dp)
}

pub async fn get(id: &Uuid) -> Result<DeviceProfile, Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<DeviceProfile, Error> {
            let mut c = get_db_conn()?;
            let dp = device_profile::dsl::device_profile
                .find(&id)
                .first(&mut c)
                .map_err(|e| error::Error::from_diesel(e, id.to_string()))?;
            Ok(dp)
        }
    })
    .await?
}

pub async fn update(dp: DeviceProfile) -> Result<DeviceProfile, Error> {
    dp.validate()?;
    let dp = task::spawn_blocking({
        move || -> Result<DeviceProfile, Error> {
            let mut c = get_db_conn()?;

            diesel::update(device_profile::dsl::device_profile.find(&dp.id))
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
                    device_profile::class_b_timeout.eq(&dp.class_b_timeout),
                    device_profile::class_b_ping_slot_nb_k.eq(&dp.class_b_ping_slot_nb_k),
                    device_profile::class_b_ping_slot_dr.eq(&dp.class_b_ping_slot_dr),
                    device_profile::class_b_ping_slot_freq.eq(&dp.class_b_ping_slot_freq),
                    device_profile::class_c_timeout.eq(&dp.class_c_timeout),
                    device_profile::abp_rx1_delay.eq(&dp.abp_rx1_delay),
                    device_profile::abp_rx1_dr_offset.eq(&dp.abp_rx1_dr_offset),
                    device_profile::abp_rx2_dr.eq(&dp.abp_rx2_dr),
                    device_profile::abp_rx2_freq.eq(&dp.abp_rx2_freq),
                    device_profile::tags.eq(&dp.tags),
                    device_profile::measurements.eq(&dp.measurements),
                    device_profile::auto_detect_measurements.eq(&dp.auto_detect_measurements),
                    device_profile::region_config_id.eq(&dp.region_config_id),
                    device_profile::is_relay.eq(&dp.is_relay),
                    device_profile::is_relay_ed.eq(&dp.is_relay_ed),
                    device_profile::relay_ed_relay_only.eq(&dp.relay_ed_relay_only),
                    device_profile::relay_enabled.eq(&dp.relay_enabled),
                    device_profile::relay_cad_periodicity.eq(&dp.relay_cad_periodicity),
                    device_profile::relay_default_channel_index.eq(&dp.relay_default_channel_index),
                    device_profile::relay_second_channel_freq.eq(&dp.relay_second_channel_freq),
                    device_profile::relay_second_channel_dr.eq(&dp.relay_second_channel_dr),
                    device_profile::relay_second_channel_ack_offset
                        .eq(&dp.relay_second_channel_ack_offset),
                    device_profile::relay_ed_activation_mode.eq(&dp.relay_ed_activation_mode),
                    device_profile::relay_ed_smart_enable_level.eq(&dp.relay_ed_smart_enable_level),
                    device_profile::relay_ed_back_off.eq(&dp.relay_ed_back_off),
                    device_profile::relay_ed_uplink_limit_bucket_size
                        .eq(&dp.relay_ed_uplink_limit_bucket_size),
                    device_profile::relay_ed_uplink_limit_reload_rate
                        .eq(&dp.relay_ed_uplink_limit_reload_rate),
                    device_profile::relay_join_req_limit_reload_rate
                        .eq(&dp.relay_join_req_limit_reload_rate),
                    device_profile::relay_notify_limit_reload_rate
                        .eq(&dp.relay_notify_limit_reload_rate),
                    device_profile::relay_global_uplink_limit_reload_rate
                        .eq(&dp.relay_global_uplink_limit_reload_rate),
                    device_profile::relay_overall_limit_reload_rate
                        .eq(&dp.relay_overall_limit_reload_rate),
                    device_profile::relay_join_req_limit_bucket_size
                        .eq(&dp.relay_join_req_limit_bucket_size),
                    device_profile::relay_notify_limit_bucket_size
                        .eq(&dp.relay_notify_limit_bucket_size),
                    device_profile::relay_global_uplink_limit_bucket_size
                        .eq(&dp.relay_global_uplink_limit_bucket_size),
                    device_profile::relay_overall_limit_bucket_size
                        .eq(&dp.relay_overall_limit_bucket_size),
                ))
                .get_result(&mut c)
                .map_err(|e| error::Error::from_diesel(e, dp.id.to_string()))
        }
    })
    .await??;
    info!(id = %dp.id, "Device-profile updated");
    Ok(dp)
}

pub async fn set_measurements(id: Uuid, m: &fields::Measurements) -> Result<DeviceProfile, Error> {
    let dp = task::spawn_blocking({
        let m = m.clone();
        move || -> Result<DeviceProfile, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device_profile::dsl::device_profile.find(&id))
                .set(device_profile::measurements.eq(m))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))
        }
    })
    .await??;
    info!(id = %id, "Device-profile measurements updated");
    Ok(dp)
}

pub async fn delete(id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra =
                diesel::delete(device_profile::dsl::device_profile.find(&id)).execute(&mut c)?;
            if ra == 0 {
                return Err(error::Error::NotFound(id.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(id = %id, "Device-profile deleted");
    Ok(())
}

pub async fn get_count(filters: &Filters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = device_profile::dsl::device_profile
                .select(dsl::count_star())
                .into_boxed();

            if let Some(tenant_id) = &filters.tenant_id {
                q = q.filter(device_profile::dsl::tenant_id.eq(tenant_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(device_profile::dsl::name.ilike(format!("%{}%", search)));
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
) -> Result<Vec<DeviceProfileListItem>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<DeviceProfileListItem>, Error> {
            let mut c = get_db_conn()?;
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
                q = q.filter(device_profile::dsl::tenant_id.eq(tenant_id));
            }

            if let Some(search) = &filters.search {
                q = q.filter(device_profile::dsl::name.ilike(format!("%{}%", search)));
            }

            let items = q
                .order_by(device_profile::dsl::name)
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
            Some(v) => v,
            None => {
                let t = storage::tenant::test::create_tenant().await;
                t.id
            }
        };

        let mut kv = HashMap::new();
        kv.insert("foo".into(), "bar".into());

        let dp = DeviceProfile {
            tenant_id: tenant_id,
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
                },
                dps: vec![&dp],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                filters: Filters {
                    tenant_id: Some(dp.tenant_id),
                    search: None,
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
        assert_eq!(true, delete(&dp.id).await.is_err());
    }
}
