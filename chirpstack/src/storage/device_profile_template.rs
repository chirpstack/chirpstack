use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use regex::Regex;
use tokio::task;
use tracing::info;

use lrwn::region::{CommonName, MacVersion, Revision};

use super::error::Error;
use super::schema::device_profile_template;
use super::{error, fields, get_db_conn};
use crate::codec::Codec;

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = device_profile_template)]
pub struct DeviceProfileTemplate {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub vendor: String,
    pub firmware: String,
    pub region: CommonName,
    pub mac_version: MacVersion,
    pub reg_params_revision: Revision,
    pub adr_algorithm_id: String,
    pub payload_codec_runtime: Codec,
    pub payload_codec_script: String,
    pub uplink_interval: i32,
    pub device_status_req_interval: i32,
    pub flush_queue_on_activate: bool,
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
    pub measurements: fields::Measurements,
    pub auto_detect_measurements: bool,
}

impl DeviceProfileTemplate {
    fn validate(&self) -> Result<(), Error> {
        let id_regex = Regex::new(r"^[\w-]+$").unwrap();
        if !id_regex.is_match(&self.id) {
            return Err(Error::Validation(
                "id can only contain alphanumeric chars or dashes".into(),
            ));
        }

        if self.name.is_empty() {
            return Err(Error::Validation("name is not set".into()));
        }

        if self.vendor.is_empty() {
            return Err(Error::Validation("vendor is not set".into()));
        }

        if self.firmware.is_empty() {
            return Err(Error::Validation("firmware is not set".into()));
        }

        Ok(())
    }
}

impl Default for DeviceProfileTemplate {
    fn default() -> Self {
        let now = Utc::now();

        DeviceProfileTemplate {
            id: "".into(),
            created_at: now,
            updated_at: now,
            name: "".into(),
            description: "".into(),
            vendor: "".into(),
            firmware: "".into(),
            region: CommonName::EU868,
            mac_version: MacVersion::LORAWAN_1_0_0,
            reg_params_revision: Revision::A,
            adr_algorithm_id: "".into(),
            payload_codec_runtime: Codec::NONE,
            payload_codec_script: "".into(),
            uplink_interval: 0,
            device_status_req_interval: 0,
            flush_queue_on_activate: false,
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
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct DeviceProfileTemplateListItem {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub vendor: String,
    pub firmware: String,
    pub region: CommonName,
    pub mac_version: MacVersion,
    pub reg_params_revision: Revision,
    pub supports_otaa: bool,
    pub supports_class_b: bool,
    pub supports_class_c: bool,
}

pub async fn create(dp: DeviceProfileTemplate) -> Result<DeviceProfileTemplate, Error> {
    dp.validate()?;
    let dp = task::spawn_blocking({
        move || -> Result<DeviceProfileTemplate, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(device_profile_template::table)
                .values(&dp)
                .get_result(&mut c)
                .map_err(|e| error::Error::from_diesel(e, dp.id.to_string()))
        }
    })
    .await??;
    info!(id = %dp.id, "Device-profile template created");
    Ok(dp)
}

pub async fn upsert(dp: DeviceProfileTemplate) -> Result<DeviceProfileTemplate, Error> {
    dp.validate()?;
    let dp = task::spawn_blocking({
        move || -> Result<DeviceProfileTemplate, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(device_profile_template::table)
                .values(&dp)
                .on_conflict(device_profile_template::id)
                .do_update()
                .set((
                    device_profile_template::updated_at.eq(Utc::now()),
                    device_profile_template::name.eq(&dp.name),
                    device_profile_template::description.eq(&dp.description),
                    device_profile_template::vendor.eq(&dp.vendor),
                    device_profile_template::firmware.eq(&dp.firmware),
                    device_profile_template::region.eq(&dp.region),
                    device_profile_template::mac_version.eq(&dp.mac_version),
                    device_profile_template::reg_params_revision.eq(&dp.reg_params_revision),
                    device_profile_template::adr_algorithm_id.eq(&dp.adr_algorithm_id),
                    device_profile_template::payload_codec_runtime.eq(&dp.payload_codec_runtime),
                    device_profile_template::payload_codec_script.eq(&dp.payload_codec_script),
                    device_profile_template::uplink_interval.eq(&dp.uplink_interval),
                    device_profile_template::device_status_req_interval
                        .eq(&dp.device_status_req_interval),
                    device_profile_template::flush_queue_on_activate
                        .eq(&dp.flush_queue_on_activate),
                    device_profile_template::supports_otaa.eq(&dp.supports_otaa),
                    device_profile_template::supports_class_b.eq(&dp.supports_class_b),
                    device_profile_template::supports_class_c.eq(&dp.supports_class_c),
                    device_profile_template::class_b_timeout.eq(&dp.class_b_timeout),
                    device_profile_template::class_b_ping_slot_nb_k.eq(&dp.class_b_ping_slot_nb_k),
                    device_profile_template::class_b_ping_slot_dr.eq(&dp.class_b_ping_slot_dr),
                    device_profile_template::class_b_ping_slot_freq.eq(&dp.class_b_ping_slot_freq),
                    device_profile_template::class_c_timeout.eq(&dp.class_c_timeout),
                    device_profile_template::abp_rx1_delay.eq(&dp.abp_rx1_delay),
                    device_profile_template::abp_rx1_dr_offset.eq(&dp.abp_rx1_dr_offset),
                    device_profile_template::abp_rx2_dr.eq(&dp.abp_rx2_dr),
                    device_profile_template::abp_rx2_freq.eq(&dp.abp_rx2_freq),
                    device_profile_template::tags.eq(&dp.tags),
                    device_profile_template::measurements.eq(&dp.measurements),
                    device_profile_template::auto_detect_measurements
                        .eq(&dp.auto_detect_measurements),
                ))
                .get_result(&mut c)
                .map_err(|e| error::Error::from_diesel(e, dp.id.to_string()))
        }
    })
    .await??;
    info!(id = %dp.id, "Device-profile template upserted");
    Ok(dp)
}

pub async fn get(id: &str) -> Result<DeviceProfileTemplate, Error> {
    task::spawn_blocking({
        let id = id.to_string();
        move || -> Result<DeviceProfileTemplate, Error> {
            let mut c = get_db_conn()?;
            let dp = device_profile_template::dsl::device_profile_template
                .find(&id)
                .first(&mut c)
                .map_err(|e| error::Error::from_diesel(e, id.clone()))?;
            Ok(dp)
        }
    })
    .await?
}

pub async fn update(dp: DeviceProfileTemplate) -> Result<DeviceProfileTemplate, Error> {
    dp.validate()?;
    let dp = task::spawn_blocking({
        move || -> Result<DeviceProfileTemplate, Error> {
            let mut c = get_db_conn()?;

            diesel::update(device_profile_template::dsl::device_profile_template.find(&dp.id))
                .set((
                    device_profile_template::updated_at.eq(Utc::now()),
                    device_profile_template::name.eq(&dp.name),
                    device_profile_template::description.eq(&dp.description),
                    device_profile_template::vendor.eq(&dp.vendor),
                    device_profile_template::firmware.eq(&dp.firmware),
                    device_profile_template::region.eq(&dp.region),
                    device_profile_template::mac_version.eq(&dp.mac_version),
                    device_profile_template::reg_params_revision.eq(&dp.reg_params_revision),
                    device_profile_template::adr_algorithm_id.eq(&dp.adr_algorithm_id),
                    device_profile_template::payload_codec_runtime.eq(&dp.payload_codec_runtime),
                    device_profile_template::payload_codec_script.eq(&dp.payload_codec_script),
                    device_profile_template::uplink_interval.eq(&dp.uplink_interval),
                    device_profile_template::device_status_req_interval
                        .eq(&dp.device_status_req_interval),
                    device_profile_template::flush_queue_on_activate
                        .eq(&dp.flush_queue_on_activate),
                    device_profile_template::supports_otaa.eq(&dp.supports_otaa),
                    device_profile_template::supports_class_b.eq(&dp.supports_class_b),
                    device_profile_template::supports_class_c.eq(&dp.supports_class_c),
                    device_profile_template::class_b_timeout.eq(&dp.class_b_timeout),
                    device_profile_template::class_b_ping_slot_nb_k.eq(&dp.class_b_ping_slot_nb_k),
                    device_profile_template::class_b_ping_slot_dr.eq(&dp.class_b_ping_slot_dr),
                    device_profile_template::class_b_ping_slot_freq.eq(&dp.class_b_ping_slot_freq),
                    device_profile_template::class_c_timeout.eq(&dp.class_c_timeout),
                    device_profile_template::abp_rx1_delay.eq(&dp.abp_rx1_delay),
                    device_profile_template::abp_rx1_dr_offset.eq(&dp.abp_rx1_dr_offset),
                    device_profile_template::abp_rx2_dr.eq(&dp.abp_rx2_dr),
                    device_profile_template::abp_rx2_freq.eq(&dp.abp_rx2_freq),
                    device_profile_template::tags.eq(&dp.tags),
                ))
                .get_result(&mut c)
                .map_err(|e| error::Error::from_diesel(e, dp.id.clone()))
        }
    })
    .await??;
    info!(id = %dp.id, "Device-profile template updated");
    Ok(dp)
}

pub async fn delete(id: &str) -> Result<(), Error> {
    task::spawn_blocking({
        let id = id.to_string();
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra =
                diesel::delete(device_profile_template::dsl::device_profile_template.find(&id))
                    .execute(&mut c)?;
            if ra == 0 {
                return Err(error::Error::NotFound(id));
            }
            Ok(())
        }
    })
    .await??;
    info!(id = %id, "Device-profile template deleted");
    Ok(())
}

pub async fn get_count() -> Result<i64, Error> {
    task::spawn_blocking({
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            Ok(device_profile_template::dsl::device_profile_template
                .select(dsl::count_star())
                .first(&mut c)?)
        }
    })
    .await?
}

pub async fn list(limit: i64, offset: i64) -> Result<Vec<DeviceProfileTemplateListItem>, Error> {
    task::spawn_blocking({
        move || -> Result<Vec<DeviceProfileTemplateListItem>, Error> {
            let mut c = get_db_conn()?;
            let items = device_profile_template::dsl::device_profile_template
                .select((
                    device_profile_template::id,
                    device_profile_template::created_at,
                    device_profile_template::updated_at,
                    device_profile_template::name,
                    device_profile_template::vendor,
                    device_profile_template::firmware,
                    device_profile_template::region,
                    device_profile_template::mac_version,
                    device_profile_template::reg_params_revision,
                    device_profile_template::supports_otaa,
                    device_profile_template::supports_class_b,
                    device_profile_template::supports_class_c,
                ))
                .order_by((
                    device_profile_template::dsl::vendor,
                    device_profile_template::dsl::name,
                    device_profile_template::dsl::firmware,
                    device_profile_template::dsl::region,
                ))
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
    use crate::test;

    struct FilterTest<'a> {
        dps: Vec<&'a DeviceProfileTemplate>,
        count: usize,
        limit: i64,
        offset: i64,
    }

    #[tokio::test]
    async fn test_device_profile_test() {
        let _guard = test::prepare().await;
        let dp = DeviceProfileTemplate {
            id: "test-dp".into(),
            name: "test-template".into(),
            vendor: "Test Vendor".into(),
            firmware: "1.2.3".into(),
            ..Default::default()
        };

        // create
        let mut dp = create(dp).await.unwrap();

        // get
        let dp_get = get(&dp.id).await.unwrap();
        assert_eq!(dp, dp_get);

        // update
        dp.name = "test-template-updated".into();
        dp = update(dp).await.unwrap();
        let dp_get = get(&dp.id).await.unwrap();
        assert_eq!(dp, dp_get);

        // upsert
        dp.name = "test-template-upsert".into();
        dp = upsert(dp).await.unwrap();
        let dp_get = get(&dp.id).await.unwrap();
        assert_eq!(dp, dp_get);

        // get count and list
        let tests = vec![
            FilterTest {
                dps: vec![&dp],
                count: 1,
                limit: 10,
                offset: 0,
            },
            FilterTest {
                dps: vec![],
                count: 1,
                limit: 0,
                offset: 0,
            },
        ];

        for tst in tests {
            let count = get_count().await.unwrap() as usize;
            assert_eq!(tst.count, count);

            let items = list(tst.limit, tst.offset).await.unwrap();
            assert_eq!(
                tst.dps.iter().map(|dp| dp.id.clone()).collect::<String>(),
                items.iter().map(|dp| dp.id.clone()).collect::<String>()
            );
        }

        // delete
        delete(&dp.id).await.unwrap();
        assert_eq!(true, delete(&dp.id).await.is_err());
    }
}
