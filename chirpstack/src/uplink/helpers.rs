use std::convert::TryInto;
use std::time::{Duration, SystemTime};

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::gpstime::ToDateTime;
use crate::region;
use chirpstack_api::{common, gw};

pub fn get_uplink_dr(region_name: &str, tx_info: &chirpstack_api::gw::UplinkTxInfo) -> Result<u8> {
    let region_conf = region::get(region_name)?;
    let mod_info = tx_info
        .modulation_info
        .as_ref()
        .ok_or(anyhow!("modulation_info must not be None"))?;

    let dr_modulation = match &mod_info {
        chirpstack_api::gw::uplink_tx_info::ModulationInfo::LoraModulationInfo(v) => {
            lrwn::region::DataRateModulation::Lora(lrwn::region::LoraDataRate {
                spreading_factor: v.spreading_factor as u8,
                bandwidth: v.bandwidth * 1000, // kHz to Hz
            })
        }
        chirpstack_api::gw::uplink_tx_info::ModulationInfo::FskModulationInfo(v) => {
            lrwn::region::DataRateModulation::Fsk(lrwn::region::FskDataRate {
                bitrate: v.datarate,
            })
        }
        chirpstack_api::gw::uplink_tx_info::ModulationInfo::LrFhssModulationInfo(v) => {
            lrwn::region::DataRateModulation::LrFhss(lrwn::region::LrFhssDataRate {
                coding_rate: v.code_rate.clone(),
                occupied_channel_width: v.operating_channel_width,
            })
        }
    };

    region_conf.get_data_rate_index(true, &dr_modulation)
}

pub fn set_uplink_modulation(
    region_name: &str,
    tx_info: &mut chirpstack_api::gw::UplinkTxInfo,
    dr: u8,
) -> Result<()> {
    let region_conf = region::get(region_name)?;
    let params = region_conf.get_data_rate(dr)?;

    match params {
        lrwn::region::DataRateModulation::Lora(v) => {
            tx_info.set_modulation(common::Modulation::Lora);
            tx_info.modulation_info = Some(gw::uplink_tx_info::ModulationInfo::LoraModulationInfo(
                gw::LoRaModulationInfo {
                    bandwidth: v.bandwidth / 1000,
                    spreading_factor: v.spreading_factor as u32,
                    code_rate: "4/5".to_string(),
                    polarization_inversion: true,
                },
            ));
        }
        lrwn::region::DataRateModulation::Fsk(v) => {
            tx_info.set_modulation(common::Modulation::Fsk);
            tx_info.modulation_info = Some(gw::uplink_tx_info::ModulationInfo::FskModulationInfo(
                gw::FskModulationInfo {
                    datarate: v.bitrate,
                    ..Default::default()
                },
            ));
        }
        lrwn::region::DataRateModulation::LrFhss(v) => {
            tx_info.set_modulation(common::Modulation::LrFhss);
            tx_info.modulation_info =
                Some(gw::uplink_tx_info::ModulationInfo::LrFhssModulationInfo(
                    gw::LrfhssModulationInfo {
                        operating_channel_width: v.occupied_channel_width,
                        code_rate: v.coding_rate,
                        // GridSteps: this value can't be derived from a DR?
                        ..Default::default()
                    },
                ));
        }
    }

    Ok(())
}

pub fn get_uplink_ch(region_name: &str, frequency: u32, dr: u8) -> Result<usize> {
    let region_conf = region::get(region_name)?;
    region_conf.get_uplink_channel_index_for_freq_dr(frequency, dr)
}

pub fn get_rx_timestamp(rx_info: &[gw::UplinkRxInfo]) -> SystemTime {
    // First search for time_since_gps_epoch.
    for rxi in rx_info {
        if let Some(gps_time) = &rxi.time_since_gps_epoch {
            if let Ok(ts) = chrono::Duration::from_std(Duration::new(
                gps_time.seconds as u64,
                gps_time.nanos as u32,
            )) {
                return ts.to_date_time().into();
            }
        }
    }

    // Then search for time.
    for rxi in rx_info {
        if let Some(ts) = &rxi.time {
            let ts: core::result::Result<DateTime<Utc>, core::num::TryFromIntError> =
                ts.clone().try_into();
            if let Ok(ts) = ts {
                return ts.into();
            }
        }
    }

    // last resort use systemtime of NS
    SystemTime::now()
}

pub fn get_rx_timestamp_chrono(rx_info: &[gw::UplinkRxInfo]) -> DateTime<Utc> {
    // First search for time_since_gps_epoch.
    for rxi in rx_info {
        if let Some(gps_time) = &rxi.time_since_gps_epoch {
            if let Ok(ts) = chrono::Duration::from_std(Duration::new(
                gps_time.seconds as u64,
                gps_time.nanos as u32,
            )) {
                return ts.to_date_time();
            }
        }
    }

    // Then search for time.
    for rxi in rx_info {
        if let Some(ts) = &rxi.time {
            let ts: core::result::Result<DateTime<Utc>, core::num::TryFromIntError> =
                ts.clone().try_into();
            if let Ok(ts) = ts {
                return ts;
            }
        }
    }

    // last resort use systemtime of NS
    Utc::now()
}

pub fn get_time_since_gps_epoch(rx_info: &[gw::UplinkRxInfo]) -> Option<Duration> {
    for rxi in rx_info {
        if let Some(gps_time) = &rxi.time_since_gps_epoch {
            return Some(Duration::new(
                gps_time.seconds as u64,
                gps_time.nanos as u32,
            ));
        }
    }

    None
}

pub fn get_time_since_gps_epoch_chrono(rx_info: &[gw::UplinkRxInfo]) -> Option<chrono::Duration> {
    for rxi in rx_info {
        if let Some(gps_time) = &rxi.time_since_gps_epoch {
            return Some(
                chrono::Duration::seconds(gps_time.seconds)
                    + chrono::Duration::nanoseconds(gps_time.nanos as i64),
            );
        }
    }

    None
}

pub fn get_start_location(rx_info: &[gw::UplinkRxInfo]) -> Option<common::Location> {
    let mut with_loc: Vec<gw::UplinkRxInfo> = rx_info
        .iter()
        .cloned()
        .filter(|i| i.location.is_some())
        .collect();
    with_loc.sort_by(|a, b| a.lora_snr.partial_cmp(&b.lora_snr).unwrap());
    with_loc
        .first()
        .map(|i| i.location.as_ref().unwrap().clone())
}
