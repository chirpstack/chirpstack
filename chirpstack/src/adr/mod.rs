use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::{info, trace, warn};

use crate::config;
use chirpstack_api::internal;
use lrwn::EUI64;

pub mod default;
pub mod lora_lr_fhss;
pub mod lr_fhss;
pub mod plugin;

lazy_static! {
    static ref ADR_ALGORITHMS: RwLock<HashMap<String, Box<dyn Handler + Sync + Send>>> =
        RwLock::new(HashMap::new());
}

pub async fn setup() -> Result<()> {
    info!("Setting up adr algorithms");
    let mut algos = ADR_ALGORITHMS.write().await;

    trace!("Setting up included algorithms");
    let a = default::Algorithm::new();
    algos.insert(a.get_id(), Box::new(a));

    let a = lr_fhss::Algorithm::new();
    algos.insert(a.get_id(), Box::new(a));

    let a = lora_lr_fhss::Algorithm::new();
    algos.insert(a.get_id(), Box::new(a));

    trace!("Setting up plugins");
    let conf = config::get();
    for file_path in &conf.network.adr_plugins {
        info!(file_path = %file_path, "Setting up ADR plugin");
        let a = plugin::Plugin::new(file_path)?;
        algos.insert(a.get_id(), Box::new(a));
    }

    Ok(())
}

pub async fn get_algorithms() -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();

    let algos = ADR_ALGORITHMS.read().await;
    for (_, v) in algos.iter() {
        out.insert(v.get_id(), v.get_name());
    }

    out
}

pub async fn handle(algo_id: &str, req: &Request) -> Response {
    let algos = ADR_ALGORITHMS.read().await;
    match algos.get(algo_id) {
        Some(v) => match v.handle(req).await {
            Ok(v) => v,
            Err(e) => {
                warn!(algorithm_id = %algo_id, error = %e, "ADR algorithm returned error");
                Response {
                    dr: req.dr,
                    tx_power_index: req.tx_power_index,
                    nb_trans: req.nb_trans,
                }
            }
        },
        None => {
            warn!(algorithm_id = %algo_id, "No ADR algorithm configured with given ID");
            Response {
                dr: req.dr,
                tx_power_index: req.tx_power_index,
                nb_trans: req.nb_trans,
            }
        }
    }
}

#[async_trait]
pub trait Handler {
    // Returns the name.
    fn get_name(&self) -> String;

    // Get the ID.
    fn get_id(&self) -> String;

    // Handle the ADR request.
    async fn handle(&self, req: &Request) -> Result<Response>;
}

#[derive(Clone)]
pub struct Request {
    pub region_config_id: String,
    pub region_common_name: lrwn::region::CommonName,
    pub dev_eui: EUI64,
    pub mac_version: lrwn::region::MacVersion,
    pub reg_params_revision: lrwn::region::Revision,
    pub adr: bool,
    pub dr: u8,
    pub tx_power_index: u8,
    pub nb_trans: u8,
    pub max_tx_power_index: u8,
    pub required_snr_for_dr: f32,
    pub installation_margin: f32,
    pub min_dr: u8,
    pub max_dr: u8,
    pub uplink_history: Vec<internal::UplinkAdrHistory>,
    pub skip_f_cnt_check: bool,
    pub device_variables: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    pub dr: u8,
    pub tx_power_index: u8,
    pub nb_trans: u8,
}
