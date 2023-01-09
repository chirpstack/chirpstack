use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::{Context, Result};
use tracing::{info, span, trace, warn, Level};

use crate::config;
use lrwn::region;

lazy_static! {
    static ref REGIONS: RwLock<HashMap<String, Arc<Box<dyn region::Region + Sync + Send>>>> =
        RwLock::new(HashMap::new());
}

pub fn setup() -> Result<()> {
    info!("Setting up regions");
    let conf = config::get();

    reset();

    for r in &conf.regions {
        let span = span!(Level::INFO, "setup", common_name = %r.common_name, region_id = %r.id);
        let _guard = span.enter();

        if !conf.network.enabled_regions.contains(&r.id) {
            warn!("Config exists, but region is not enabled. To enable it, add '{}' to 'network.enabled_regions'", r.id);
            continue;
        }

        info!("Configuring region");

        let mut region_conf = region::get(
            r.common_name,
            r.network.repeater_compatible,
            r.network.dwell_time_400ms,
        );

        for ec in &r.network.extra_channels {
            trace!(
                frequency = ec.frequency,
                min_dr = ec.min_dr,
                max_dr = ec.max_dr,
                "Adding extra channel"
            );
            region_conf
                .add_channel(ec.frequency, ec.min_dr, ec.max_dr)
                .context("Add channel")?;
        }

        if !r.network.enabled_uplink_channels.is_empty() {
            trace!("Disabling all channels first");
            for i in region_conf.get_enabled_uplink_channel_indices() {
                region_conf.disable_uplink_channel_index(i)?;
            }

            trace!(channels = ?r.network.enabled_uplink_channels, "Enabling channels");
            for i in &r.network.enabled_uplink_channels {
                region_conf.enable_uplink_channel_index(*i)?;
            }
        }

        set(&r.id, region_conf);
    }

    Ok(())
}

fn reset() {
    let mut regions_w = REGIONS.write().unwrap();
    regions_w.clear();
}

pub fn set(region_config_id: &str, r: Box<dyn region::Region + Sync + Send>) {
    let mut regions_w = REGIONS.write().unwrap();
    regions_w.insert(region_config_id.to_string(), Arc::new(r));
}

pub fn get(region_config_id: &str) -> Result<Arc<Box<dyn region::Region + Sync + Send>>> {
    let regions_r = REGIONS.read().unwrap();
    Ok(regions_r
        .get(region_config_id)
        .ok_or_else(|| {
            anyhow!(
                "region_config_id {} does not exist in REGIONS",
                region_config_id
            )
        })?
        .clone())
}

/// This returns the (first) region-name, based on the given common-name.
/// This function is used for roaming, as within the context of roaming, only
/// the common-name is given by the other party.
pub fn get_region_config_id(common_name: region::CommonName) -> Result<String> {
    let regions_r = REGIONS.read().unwrap();
    for (k, v) in &*regions_r {
        if v.get_name() == common_name {
            return Ok(k.clone());
        }
    }

    Err(anyhow!(
        "No region configured with common-name: {}",
        common_name
    ))
}
