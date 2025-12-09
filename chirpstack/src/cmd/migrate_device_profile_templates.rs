use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::info;

use lrwn::region::{CommonName, MacVersion, Revision};

use crate::codec::Codec;
use crate::storage::{
    self, device_profile, fields, get_async_db_conn, schema::device_profile_template,
};

#[derive(Clone, Queryable, Debug, PartialEq, Eq)]
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
    pub class_b_ping_slot_periodicity: i32,
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

pub async fn run() -> Result<()> {
    storage::setup().await?;

    info!("Migrating device-profile templates to device profiles");

    info!("Querying vendors");
    let vendors: Vec<String> = device_profile_template::table
        .select(device_profile_template::vendor)
        .group_by(device_profile_template::vendor)
        .load(&mut get_async_db_conn().await?)
        .await?;

    for vendor in &vendors {
        info!(vendor = %vendor, "Creating vendor");

        let v = device_profile::upsert_vendor(device_profile::Vendor {
            name: vendor.clone(),
            ..Default::default()
        })
        .await?;

        info!(vendor = %vendor, "Querying vendor devices");
        let devices: Vec<String> = device_profile_template::table
            .select(device_profile_template::name)
            .filter(device_profile_template::vendor.eq(vendor))
            .group_by(device_profile_template::name)
            .load(&mut get_async_db_conn().await?)
            .await?;

        for device in &devices {
            info!(vendor = %vendor, device = %device, "Creating device");

            let d = device_profile::upsert_device(device_profile::Device {
                vendor_id: v.id,
                name: device.clone(),
                ..Default::default()
            })
            .await?;

            info!(vendor = %vendor, device = %device, "Querying device profiles");
            let dps: Vec<DeviceProfileTemplate> = device_profile_template::table
                .filter(
                    device_profile_template::vendor
                        .eq(vendor)
                        .and(device_profile_template::name.eq(device)),
                )
                .load(&mut get_async_db_conn().await?)
                .await?;

            for dp in &dps {
                info!(vendor = %vendor, device = %device, firmware = %dp.firmware, region = %dp.region, "Creating device profile");

                let _ = device_profile::create(device_profile::DeviceProfile {
                    name: dp.name.clone(),
                    region: dp.region,
                    mac_version: dp.mac_version,
                    reg_params_revision: dp.reg_params_revision,
                    adr_algorithm_id: "default".into(),
                    payload_codec_runtime: dp.payload_codec_runtime,
                    uplink_interval: dp.uplink_interval,
                    device_status_req_interval: dp.device_status_req_interval,
                    supports_otaa: dp.supports_otaa,
                    supports_class_b: dp.supports_class_b,
                    supports_class_c: dp.supports_class_c,
                    tags: dp.tags.clone(),
                    payload_codec_script: dp.payload_codec_script.clone(),
                    flush_queue_on_activate: dp.flush_queue_on_activate,
                    description: dp.description.clone(),
                    measurements: dp.measurements.clone(),
                    auto_detect_measurements: dp.auto_detect_measurements,
                    rx1_delay: 0,
                    abp_params: if dp.supports_otaa {
                        Some(fields::AbpParams {
                            rx1_delay: dp.abp_rx1_delay as u8,
                            rx1_dr_offset: dp.abp_rx1_dr_offset as u8,
                            rx2_dr: dp.abp_rx2_dr as u8,
                            rx2_freq: dp.abp_rx2_freq as u32,
                        })
                    } else {
                        None
                    },
                    class_b_params: if dp.supports_class_b {
                        Some(fields::ClassBParams {
                            timeout: dp.class_b_timeout as u16,
                            ping_slot_periodicity: dp.class_b_ping_slot_periodicity as u8,
                            ping_slot_dr: dp.class_b_ping_slot_dr as u8,
                            ping_slot_freq: dp.class_b_ping_slot_freq as u32,
                        })
                    } else {
                        None
                    },
                    class_c_params: if dp.supports_class_c {
                        Some(fields::ClassCParams {
                            timeout: dp.class_c_timeout as u16,
                        })
                    } else {
                        None
                    },
                    relay_params: None,
                    app_layer_params: fields::AppLayerParams::default(),
                    device_id: Some(d.id),
                    firmware_version: dp.firmware.clone(),
                    ..Default::default()
                })
                .await?;
            }
        }
    }

    Ok(())
}
