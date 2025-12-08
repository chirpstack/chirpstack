use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use tracing::{Instrument, Level, info, span};
use uuid::Uuid;

use crate::codec::Codec;
use crate::storage::{self, device_profile, fields};
use lrwn::region;

#[derive(Deserialize)]
struct VendorConfig {
    pub vendor: Vendor,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Vendor {
    pub id: Uuid,
    pub name: String,
    pub vendor_id: i32,
    pub ouis: Vec<String>,
    pub devices: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct DeviceConfig {
    pub device: Device,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub firmware: Vec<DeviceFirmware>,
    pub metadata: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct DeviceFirmware {
    pub version: String,
    pub profiles: Vec<String>,
    pub codec: Option<String>,
}

#[derive(Deserialize)]
pub struct ProfileConfig {
    pub profile: Profile,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct Profile {
    pub id: Uuid,
    pub vendor_profile_id: usize,
    pub region: region::CommonName,
    pub mac_version: region::MacVersion,
    pub reg_params_revision: region::Revision,
    pub supports_otaa: bool,
    pub supports_class_b: bool,
    pub supports_class_c: bool,
    pub max_eirp: usize,

    pub abp: ProfileAbp,
    pub class_b: ProfileClassB,
    pub class_c: ProfileClassC,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            vendor_profile_id: 0,
            region: region::CommonName::EU868,
            mac_version: region::MacVersion::LORAWAN_1_0_4,
            reg_params_revision: region::Revision::RP002_1_0_4,
            supports_otaa: false,
            supports_class_b: false,
            supports_class_c: false,
            max_eirp: 0,
            abp: ProfileAbp::default(),
            class_b: ProfileClassB::default(),
            class_c: ProfileClassC::default(),
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ProfileAbp {
    pub rx1_delay: usize,
    pub rx1_dr_offset: usize,
    pub rx2_dr: usize,
    pub rx2_freq: usize,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ProfileClassB {
    pub timeout_secs: usize,
    pub ping_slot_periodicity: usize,
    pub ping_slot_dr: usize,
    pub ping_slot_freq: usize,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct ProfileClassC {
    pub timeout_secs: usize,
}

pub async fn run(dir: &Path) -> Result<()> {
    storage::setup().await?;
    info!(path = ?dir, "Import LoRaWAN device profiles");

    let vendors_dir = dir.join("vendors");
    let vendors = fs::read_dir(vendors_dir)?;

    for vendor in vendors.flatten() {
        if vendor.file_name() == "example-vendor" {
            continue;
        }

        let span = span!(Level::INFO, "", vendor = ?vendor.file_name());

        let vendor_dir = vendor.path();
        if vendor_dir.is_dir() {
            handle_vendor(&vendor_dir).instrument(span).await?;
        }
    }

    Ok(())
}

async fn handle_vendor(dir: &Path) -> Result<()> {
    // vendor_conf.vendor.slug = dir.file_name().unwrap().to_str().unwrap().to_string();
    //
    let vendor_conf = dir.join("vendor.toml");
    info!(path = ?vendor_conf, "Reading vendor configuration");
    let vendor_conf: VendorConfig = toml::from_str(&fs::read_to_string(vendor_conf)?)?;
    info!(vendor_name = %vendor_conf.vendor.name, "Vendor loaded");

    info!(id = %vendor_conf.vendor.id, "Upserting vendor");
    let _ = device_profile::upsert_vendor(device_profile::Vendor {
        id: vendor_conf.vendor.id.into(),
        name: vendor_conf.vendor.name.clone(),
        vendor_id: vendor_conf.vendor.vendor_id,
        ouis: fields::StringVec::new(
            vendor_conf
                .vendor
                .ouis
                .iter()
                .map(|v| Some(v.clone()))
                .collect(),
        ),
        metadata: fields::KeyValue::new(vendor_conf.vendor.metadata.clone()),
        ..Default::default()
    })
    .await?;

    for device in &vendor_conf.vendor.devices {
        let span = span!(Level::INFO, "", device = %device);
        handle_device(dir, &vendor_conf.vendor, device)
            .instrument(span)
            .await?;
    }

    Ok(())
}

async fn handle_device(dir: &Path, vendor: &Vendor, device: &str) -> Result<()> {
    let device_conf = dir.join("devices").join(device);

    info!(path = ?device_conf, "Reading device configuration");
    let device_conf: DeviceConfig = toml::from_str(&fs::read_to_string(device_conf)?)?;
    info!(device_name = %device_conf.device.name, "Device loaded");

    let _ = device_profile::upsert_device(device_profile::Device {
        id: device_conf.device.id.into(),
        vendor_id: vendor.id.into(),
        name: device_conf.device.name.clone(),
        description: device_conf.device.description.clone(),
        metadata: fields::KeyValue::new(device_conf.device.metadata.clone()),
        ..Default::default()
    })
    .await?;

    for firmware in &device_conf.device.firmware {
        let span = span!(Level::INFO, "", firmware = %firmware.version);
        handle_firmware(dir, &device_conf.device, firmware)
            .instrument(span)
            .await?;
    }

    Ok(())
}

async fn handle_firmware(dir: &Path, device: &Device, firmware: &DeviceFirmware) -> Result<()> {
    let codec = if let Some(codec) = &firmware.codec {
        let codec_path = dir.join("codecs").join(codec);
        info!(path = ?codec_path, "Reading codec file");

        Some(fs::read_to_string(codec_path)?)
    } else {
        None
    };

    for profile in &firmware.profiles {
        let span = span!(Level::INFO, "", profile = %profile);
        handle_profile(dir, device, firmware, &codec, profile)
            .instrument(span)
            .await?;
    }

    Ok(())
}

async fn handle_profile(
    dir: &Path,
    device: &Device,
    firmware: &DeviceFirmware,
    codec: &Option<String>,
    profile: &str,
) -> Result<()> {
    let profile_path = dir.join("profiles").join(profile);

    info!(path = ?profile_path, "Reading profile configuration");
    let profile_conf: ProfileConfig = toml::from_str(&fs::read_to_string(profile_path)?)?;

    let mut dp = device_profile::DeviceProfile {
        name: format!(
            "{} (region: {}, firmware: {})",
            device.name, profile_conf.profile.region, firmware.version
        ),
        region: profile_conf.profile.region,
        mac_version: profile_conf.profile.mac_version,
        reg_params_revision: profile_conf.profile.reg_params_revision,
        adr_algorithm_id: "default".into(),
        payload_codec_runtime: match codec {
            Some(_) => Codec::JS,
            None => Codec::NONE,
        },
        uplink_interval: 60 * 60,
        device_status_req_interval: 1,
        supports_otaa: profile_conf.profile.supports_otaa,
        supports_class_b: profile_conf.profile.supports_class_b,
        supports_class_c: profile_conf.profile.supports_class_c,
        payload_codec_script: codec.clone().unwrap_or_default(),
        flush_queue_on_activate: true,
        description: device.description.clone(),
        abp_params: if !profile_conf.profile.supports_otaa {
            Some(fields::AbpParams {
                rx1_delay: profile_conf.profile.abp.rx1_delay as u8,
                rx1_dr_offset: profile_conf.profile.abp.rx1_dr_offset as u8,
                rx2_dr: profile_conf.profile.abp.rx2_dr as u8,
                rx2_freq: profile_conf.profile.abp.rx2_freq as u32,
            })
        } else {
            None
        },
        class_b_params: if profile_conf.profile.supports_class_b {
            Some(fields::ClassBParams {
                timeout: profile_conf.profile.class_b.timeout_secs as u16,
                ping_slot_periodicity: profile_conf.profile.class_b.ping_slot_periodicity as u8,
                ping_slot_dr: profile_conf.profile.class_b.ping_slot_dr as u8,
                ping_slot_freq: profile_conf.profile.class_b.ping_slot_freq as u32,
            })
        } else {
            None
        },
        class_c_params: if profile_conf.profile.supports_class_c {
            Some(fields::ClassCParams {
                timeout: profile_conf.profile.class_c.timeout_secs as u16,
            })
        } else {
            None
        },
        device_id: Some(device.id.into()),
        firmware_version: firmware.version.clone(),
        vendor_profile_id: profile_conf.profile.vendor_profile_id as i32,
        ..Default::default()
    };

    if let Ok(dp_existing) = device_profile::get_for_device_id_region_and_fw(
        device.id,
        profile_conf.profile.region,
        &firmware.version,
    )
    .await
    {
        info!(id = %dp_existing.id, "Updating existing device-profile");
        dp.id = dp_existing.id;
        dp.created_at = dp_existing.created_at;
        device_profile::update(dp).await?;
    } else {
        info!("Creating new device-profile");
        device_profile::create(dp).await?;
    }

    Ok(())
}
