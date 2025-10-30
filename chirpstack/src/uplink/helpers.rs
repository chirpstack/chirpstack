#[cfg(test)]
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use anyhow::Result;
use chrono::{DateTime, TimeDelta, Utc};

use chirpstack_api::gw;

use crate::{config, gpstime::ToDateTime, region};

pub fn get_uplink_dr(
    region_config_id: &str,
    tx_info: &chirpstack_api::gw::UplinkTxInfo,
) -> Result<u8> {
    let region_conf = region::get(region_config_id)?;
    let mod_info = tx_info
        .modulation
        .as_ref()
        .ok_or_else(|| anyhow!("modulation must not be None"))?;

    let mod_params = mod_info
        .parameters
        .as_ref()
        .ok_or_else(|| anyhow!("parameters must not be None"))?;

    let dr_modulation = match &mod_params {
        chirpstack_api::gw::modulation::Parameters::Lora(v) => {
            lrwn::region::DataRateModulation::Lora(lrwn::region::LoraDataRate {
                spreading_factor: v.spreading_factor as u8,
                bandwidth: v.bandwidth,
                coding_rate: v.code_rate().into(),
            })
        }
        chirpstack_api::gw::modulation::Parameters::Fsk(v) => {
            lrwn::region::DataRateModulation::Fsk(lrwn::region::FskDataRate {
                bitrate: v.datarate,
            })
        }
        chirpstack_api::gw::modulation::Parameters::LrFhss(v) => {
            lrwn::region::DataRateModulation::LrFhss(lrwn::region::LrFhssDataRate {
                coding_rate: v.code_rate().into(),
                occupied_channel_width: v.operating_channel_width,
            })
        }
    };

    region_conf.get_data_rate_index(true, &dr_modulation)
}

pub fn get_uplink_ch(region_config_id: &str, frequency: u32, dr: u8) -> Result<usize> {
    let region_conf = region::get(region_config_id)?;
    region_conf.get_uplink_channel_index_for_freq_dr(frequency, dr)
}

pub fn get_rx_timestamp(rx_info: &[gw::UplinkRxInfo]) -> SystemTime {
    let conf = config::get();
    let rx_timestamp_max_drift = conf.gateway.rx_timestamp_max_drift;

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
        if let Some(ts) = &rxi.gw_time {
            let ts: Result<DateTime<Utc>> = (*ts).try_into().map_err(anyhow::Error::msg);
            if let Ok(ts) = ts {
                let mut delta = Utc::now() - ts;
                if delta < TimeDelta::default() {
                    delta = -delta;
                }
                let delta = delta.to_std().unwrap_or_default();
                if delta < rx_timestamp_max_drift {
                    return ts.into();
                }
            }
        }
    }

    // last resort use systemtime of NS
    SystemTime::now()
}

pub fn get_rx_timestamp_chrono(rx_info: &[gw::UplinkRxInfo]) -> DateTime<Utc> {
    let conf = config::get();
    let rx_timestamp_max_drift = conf.gateway.rx_timestamp_max_drift;

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
        if let Some(ts) = &rxi.gw_time {
            let ts: Result<DateTime<Utc>> = (*ts).try_into().map_err(anyhow::Error::msg);
            if let Ok(ts) = ts {
                let mut delta = Utc::now() - ts;
                if delta < TimeDelta::default() {
                    delta = -delta;
                }
                let delta = delta.to_std().unwrap_or_default();
                if delta < rx_timestamp_max_drift {
                    return ts;
                }
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

#[cfg(test)]
pub fn set_uplink_modulation(
    region_config_id: &str,
    tx_info: &mut chirpstack_api::gw::UplinkTxInfo,
    dr: u8,
) -> Result<()> {
    let region_conf = region::get(region_config_id)?;
    let params = region_conf.get_data_rate(true, dr)?;

    tx_info.modulation = Some(gw::Modulation {
        parameters: Some(match params {
            lrwn::region::DataRateModulation::Lora(v) => {
                gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                    bandwidth: v.bandwidth,
                    spreading_factor: v.spreading_factor as u32,
                    code_rate: gw::CodeRate::from_str(&v.coding_rate)
                        .map_err(|e| anyhow!("{}", e))?
                        .into(),
                    code_rate_legacy: "".into(),
                    polarization_inversion: true,
                    no_crc: false,
                    preamble: 0,
                })
            }
            lrwn::region::DataRateModulation::Fsk(v) => {
                gw::modulation::Parameters::Fsk(gw::FskModulationInfo {
                    datarate: v.bitrate,
                    ..Default::default()
                })
            }
            lrwn::region::DataRateModulation::LrFhss(v) => {
                gw::modulation::Parameters::LrFhss(gw::LrFhssModulationInfo {
                    operating_channel_width: v.occupied_channel_width,
                    code_rate: gw::CodeRate::from_str(&v.coding_rate)
                        .map_err(|e| anyhow!("{}", e))?
                        .into(),
                    // GridSteps: this value can't be derived from a DR?
                    ..Default::default()
                })
            }
        }),
    });

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_rx_timestamp_no_drift() {
        let now = Utc::now();
        let rx_info = gw::UplinkRxInfo {
            gw_time: Some(now.try_into().unwrap()),
            ..Default::default()
        };

        let res: DateTime<Utc> = get_rx_timestamp(&[rx_info]).into();
        assert_eq!(res, now);
    }

    #[test]
    fn test_get_rx_timestamp_drift() {
        let now = Utc::now() - chrono::Duration::seconds(60);
        let rx_info = gw::UplinkRxInfo {
            gw_time: Some(now.try_into().unwrap()),
            ..Default::default()
        };

        let res: DateTime<Utc> = get_rx_timestamp(&[rx_info]).into();
        assert_ne!(res, now);

        let now = Utc::now() + chrono::Duration::seconds(60);
        let rx_info = gw::UplinkRxInfo {
            gw_time: Some(now.try_into().unwrap()),
            ..Default::default()
        };

        let res: DateTime<Utc> = get_rx_timestamp(&[rx_info]).into();
        assert_ne!(res, now);
    }

    #[test]
    fn test_get_rx_timestamp_chrono_no_drift() {
        let now = Utc::now();
        let rx_info = gw::UplinkRxInfo {
            gw_time: Some(now.try_into().unwrap()),
            ..Default::default()
        };

        let res = get_rx_timestamp_chrono(&[rx_info]);
        assert_eq!(res, now);
    }

    #[test]
    fn test_get_rx_timestamp_chrono_drift() {
        let now = Utc::now() - chrono::Duration::seconds(60);
        let rx_info = gw::UplinkRxInfo {
            gw_time: Some(now.try_into().unwrap()),
            ..Default::default()
        };

        let res = get_rx_timestamp_chrono(&[rx_info]);
        assert_ne!(res, now);

        let now = Utc::now() + chrono::Duration::seconds(60);
        let rx_info = gw::UplinkRxInfo {
            gw_time: Some(now.try_into().unwrap()),
            ..Default::default()
        };

        let res = get_rx_timestamp_chrono(&[rx_info]);
        assert_ne!(res, now);
    }
}
