use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use tracing::{info, span, Instrument, Level};
use uuid::Uuid;

use crate::codec::Codec;
use crate::storage::{self, device_profile_template};
use lrwn::region;

#[derive(Deserialize)]
struct VendorConfig {
    pub vendor: Vendor,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Vendor {
    pub slug: String,
    pub name: String,
    pub id: usize,
    pub ouis: Vec<String>,
    pub devices: Vec<String>,
}

#[derive(Deserialize)]
pub struct DeviceConfig {
    pub device: Device,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Device {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub firmware: Vec<DeviceFirmware>,
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
    let vendor_conf = dir.join("vendor.toml");
    info!(path = ?vendor_conf, "Reading vendor configuration");

    let mut vendor_conf: VendorConfig = toml::from_str(&fs::read_to_string(vendor_conf)?)?;
    vendor_conf.vendor.slug = dir.file_name().unwrap().to_str().unwrap().to_string();
    info!(vendor_name = %vendor_conf.vendor.name, "Vendor loaded");
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

    let mut device_conf: DeviceConfig = toml::from_str(&fs::read_to_string(device_conf)?)?;
    device_conf.device.slug = dir
        .join("devices")
        .join(device)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    info!(device_name = %device_conf.device.name, "Device loaded");

    for firmware in &device_conf.device.firmware {
        let span = span!(Level::INFO, "", firmware = %firmware.version);
        handle_firmware(dir, vendor, &device_conf.device, firmware)
            .instrument(span)
            .await?;
    }

    Ok(())
}

async fn handle_firmware(
    dir: &Path,
    vendor: &Vendor,
    device: &Device,
    firmware: &DeviceFirmware,
) -> Result<()> {
    let codec = if let Some(codec) = &firmware.codec {
        let codec_path = dir.join("codecs").join(codec);
        info!(path = ?codec_path, "Reading codec file");

        Some(fs::read_to_string(codec_path)?)
    } else {
        None
    };

    for profile in &firmware.profiles {
        let span = span!(Level::INFO, "", profile = %profile);
        handle_profile(dir, vendor, device, firmware, &codec, profile)
            .instrument(span)
            .await?;
    }

    Ok(())
}

async fn handle_profile(
    dir: &Path,
    vendor: &Vendor,
    device: &Device,
    firmware: &DeviceFirmware,
    codec: &Option<String>,
    profile: &str,
) -> Result<()> {
    let profile_path = dir.join("profiles").join(profile);
    info!(path = ?profile_path, "Reading profile configuration");

    let profile_conf: ProfileConfig = toml::from_str(&fs::read_to_string(profile_path)?)?;

    let dpt = device_profile_template::DeviceProfileTemplate {
        id: profile_conf.profile.id.to_string(),
        name: device.name.clone(),
        description: device.description.clone(),
        vendor: vendor.name.clone(),
        firmware: firmware.version.clone(),
        region: profile_conf.profile.region,
        mac_version: profile_conf.profile.mac_version,
        reg_params_revision: profile_conf.profile.reg_params_revision,
        adr_algorithm_id: "default".into(),
        payload_codec_runtime: match codec {
            Some(_) => Codec::JS,
            None => Codec::NONE,
        },
        payload_codec_script: match codec {
            Some(v) => v.into(),
            None => "".into(),
        },
        uplink_interval: 60 * 60,
        device_status_req_interval: 1,
        flush_queue_on_activate: true,
        supports_otaa: profile_conf.profile.supports_otaa,
        supports_class_b: profile_conf.profile.supports_class_b,
        supports_class_c: profile_conf.profile.supports_class_c,
        class_b_timeout: profile_conf.profile.class_b.timeout_secs as i32,
        class_b_ping_slot_periodicity: profile_conf.profile.class_b.ping_slot_periodicity as i32,
        class_b_ping_slot_dr: profile_conf.profile.class_b.ping_slot_dr as i16,
        class_b_ping_slot_freq: profile_conf.profile.class_b.ping_slot_freq as i64,
        class_c_timeout: profile_conf.profile.class_c.timeout_secs as i32,
        abp_rx1_delay: profile_conf.profile.abp.rx1_delay as i16,
        abp_rx1_dr_offset: profile_conf.profile.abp.rx1_dr_offset as i16,
        abp_rx2_dr: profile_conf.profile.abp.rx2_dr as i16,
        abp_rx2_freq: profile_conf.profile.abp.rx2_freq as i64,
        ..Default::default()
    };

    info!(id = %dpt.id, "Creating or updating device-profile template");
    device_profile_template::upsert(dpt).await?;

    Ok(())
}
