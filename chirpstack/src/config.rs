use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fs};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use lrwn::region::CommonName;
use lrwn::{AES128Key, NetID, EUI64};

lazy_static! {
    static ref CONFIG: Mutex<Arc<Configuration>> = Mutex::new(Arc::new(Default::default()));
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Configuration {
    pub logging: Logging,
    pub postgresql: Postgresql,
    pub redis: Redis,
    pub api: Api,
    pub gateway: Gateway,
    pub network: Network,
    pub monitoring: Monitoring,
    pub integration: Integration,
    pub codec: Codec,
    pub user_authentication: UserAuthentication,
    pub join_server: JoinServer,
    pub keks: Vec<Kek>,
    pub regions: Vec<Region>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            logging: Default::default(),
            postgresql: Default::default(),
            redis: Default::default(),
            api: Default::default(),
            gateway: Default::default(),
            network: Default::default(),
            monitoring: Default::default(),
            integration: Default::default(),
            codec: Default::default(),
            user_authentication: Default::default(),
            join_server: Default::default(),
            keks: Vec::new(),
            regions: vec![Default::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Logging {
    pub level: String,
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "info".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Postgresql {
    pub dsn: String,
    pub max_open_connections: u32,
    pub min_idle_connections: u32,
}

impl Default for Postgresql {
    fn default() -> Self {
        Postgresql {
            dsn: "postgresql://chirpstack:chirpstack@localhost/chirpstack?sslmode=disable".into(),
            max_open_connections: 10,
            min_idle_connections: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Redis {
    pub servers: Vec<String>,
    pub cluster: bool,
    pub key_prefix: String,
    pub max_open_connections: u32,
    pub min_idle_connections: u32,
}

impl Default for Redis {
    fn default() -> Self {
        Redis {
            servers: vec!["redis://127.0.0.1/".into()],
            cluster: false,
            key_prefix: "".into(),
            max_open_connections: 100,
            min_idle_connections: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Api {
    pub bind: String,
    pub secret: String,
}

impl Default for Api {
    fn default() -> Self {
        Api {
            bind: "0.0.0.0:8080".into(),
            secret: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Gateway {
    #[serde(with = "humantime_serde")]
    pub client_cert_lifetime: Duration,
    pub ca_cert: String,
    pub ca_key: String,
}

impl Default for Gateway {
    fn default() -> Self {
        Gateway {
            client_cert_lifetime: Duration::from_secs(60 * 60 * 24 * 365),
            ca_cert: "".to_string(),
            ca_key: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Network {
    pub net_id: NetID,
    pub enabled_regions: Vec<String>,
    #[serde(with = "humantime_serde")]
    pub device_session_ttl: Duration,
    pub mac_commands_disabled: bool,
    pub adr_plugins: Vec<String>,
    pub scheduler: Scheduler,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            net_id: NetID::from_be_bytes([0x00, 0x00, 0x00]),
            enabled_regions: vec!["eu868".into()],
            device_session_ttl: Duration::from_secs(60 * 60 * 24 * 31),
            mac_commands_disabled: false,
            adr_plugins: vec![],
            scheduler: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Scheduler {
    #[serde(with = "humantime_serde")]
    pub interval: Duration,
    pub batch_size: usize,
    #[serde(with = "humantime_serde")]
    pub class_a_lock_duration: Duration,
    #[serde(with = "humantime_serde")]
    pub class_c_lock_duration: Duration,
    pub multicast_class_c_use_gps_time: bool,
    #[serde(with = "humantime_serde")]
    pub multicast_class_c_margin: Duration,
    #[serde(with = "humantime_serde")]
    pub multicast_class_b_margin: Duration,
}

impl Default for Scheduler {
    fn default() -> Self {
        Scheduler {
            interval: Duration::from_secs(1),
            batch_size: 100,
            class_a_lock_duration: Duration::from_secs(5),
            class_c_lock_duration: Duration::from_secs(5),
            multicast_class_c_use_gps_time: false,
            multicast_class_c_margin: Duration::from_secs(5),
            multicast_class_b_margin: Duration::from_secs(5),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Monitoring {
    pub meta_log_max_history: usize,
    pub gateway_frame_log_max_history: usize,
    pub device_frame_log_max_history: usize,
    pub device_event_log_max_history: usize,
    pub per_gateway_frame_log_max_history: usize,
    #[serde(with = "humantime_serde")]
    pub per_gateway_frame_log_ttl: Duration,
    pub per_device_frame_log_max_history: usize,
    #[serde(with = "humantime_serde")]
    pub per_device_frame_log_ttl: Duration,
    pub per_device_event_log_max_history: usize,
    #[serde(with = "humantime_serde")]
    pub per_device_event_log_ttl: Duration,
}

impl Default for Monitoring {
    fn default() -> Self {
        Monitoring {
            meta_log_max_history: 10,
            gateway_frame_log_max_history: 10,
            device_frame_log_max_history: 10,
            device_event_log_max_history: 10,
            per_gateway_frame_log_max_history: 10,
            per_device_frame_log_max_history: 10,
            per_device_event_log_max_history: 10,
            per_gateway_frame_log_ttl: Duration::from_secs(60 * 60 * 24 * 31), // 31 days
            per_device_frame_log_ttl: Duration::from_secs(60 * 60 * 24 * 31),
            per_device_event_log_ttl: Duration::from_secs(60 * 60 * 24 * 31),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Integration {
    pub enabled: Vec<String>,
    pub mqtt: MqttIntegration,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct MqttIntegration {
    pub client: MqttIntegrationClient,
    pub event_topic: String,
    pub state_topic: String,
    pub json: bool,
    pub server: String,
    pub username: String,
    pub password: String,
    pub qos: usize,
    pub clean_session: bool,
    pub client_id: String,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
}

impl Default for MqttIntegration {
    fn default() -> Self {
        MqttIntegration {
            client: Default::default(),
            event_topic: "application/{{application_id}}/device/{{dev_eui}}/event/{{event}}".into(),
            state_topic: "application/{{application_id}}/device/{{dev_eui}}/state/{{state}}".into(),
            json: true,
            server: "tcp://127.0.0.1:1883/".into(),
            username: "".into(),
            password: "".into(),
            qos: 0,
            clean_session: true,
            client_id: "".into(),
            ca_cert: "".into(),
            tls_cert: "".into(),
            tls_key: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct MqttIntegrationClient {
    #[serde(with = "humantime_serde")]
    pub client_cert_lifetime: Duration,
    pub ca_cert: String,
    pub ca_key: String,
}

impl Default for MqttIntegrationClient {
    fn default() -> Self {
        MqttIntegrationClient {
            client_cert_lifetime: Duration::from_secs(60 * 60 * 24 * 365),
            ca_cert: "".into(),
            ca_key: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Codec {
    pub js: CodecJs,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CodecJs {
    #[serde(with = "humantime_serde")]
    pub max_execution_time: Duration,
}

impl Default for CodecJs {
    fn default() -> Self {
        CodecJs {
            max_execution_time: Duration::from_millis(100),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct UserAuthentication {
    pub openid_connect: OpenIdConnect,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct OpenIdConnect {
    pub enabled: bool,
    pub registration_enabled: bool,
    pub registration_callback_url: String,
    pub provider_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub logout_url: String,
    pub login_label: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct JoinServer {
    pub servers: Vec<JoinServerServer>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct JoinServerServer {
    pub join_eui: EUI64,
    pub server: String,
    pub async_interface: bool,
    #[serde(with = "humantime_serde")]
    pub async_interface_timeout: Duration,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Kek {
    pub label: String,
    pub kek: AES128Key,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Region {
    pub name: String,
    pub common_name: CommonName,
    pub network: RegionNetwork,
    pub gateway: RegionGateway,
}

impl Default for Region {
    fn default() -> Self {
        Region {
            name: "eu868".to_string(),
            common_name: CommonName::EU868,
            network: RegionNetwork {
                installation_margin: 10.0,
                rx1_delay: 1,
                rx2_frequency: 869525000,
                gateway_prefer_min_margin: 10.0,
                downlink_tx_power: -1,
                min_dr: 0,
                max_dr: 5,
                uplink_max_eirp: 16.0,
                class_b: ClassB {
                    ping_slot_dr: 0,
                    ping_slot_frequency: 868100000,
                },
                extra_channels: Vec::new(),
                enabled_uplink_channels: Vec::new(),
                ..Default::default()
            },
            gateway: RegionGateway {
                force_gws_private: false,
                channels: vec![],
                backend: GatewayBackend {
                    enabled: "mqtt".into(),
                    mqtt: GatewayBackendMqtt {
                        event_topic: "eu868/gateway/+/event/+".into(),
                        command_topic: "eu868/gateway/{{ gateway_id }}/command/{{ command }}"
                            .into(),
                        server: "tcp://127.0.0.1:1883".into(),
                        clean_session: true,
                        ..Default::default()
                    },
                },
            },
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RegionNetwork {
    pub installation_margin: f32,
    pub rx_window: u8,
    pub rx1_delay: u8,
    pub rx1_dr_offset: u8,
    pub rx2_dr: u8,
    pub rx2_frequency: u32,
    pub rx2_prefer_on_rx1_dr_lt: u8,
    pub rx2_prefer_on_link_budget: bool,
    pub gateway_prefer_min_margin: f32,
    pub downlink_tx_power: i32,
    pub adr_disabled: bool,
    pub min_dr: u8,
    pub max_dr: u8,
    pub uplink_dwell_time_400ms: bool,
    pub downlink_dwell_time_400ms: bool,
    pub uplink_max_eirp: f32,
    pub rejoin_request: RejoinRequest,
    pub class_b: ClassB,
    pub extra_channels: Vec<ExtraChannel>,
    pub enabled_uplink_channels: Vec<usize>,
    pub repeater_compatible: bool,
    pub dwell_time_400ms: bool,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RejoinRequest {
    pub enabled: bool,
    pub max_count_n: u8,
    pub max_time_n: u8,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ClassB {
    pub ping_slot_dr: u8,
    pub ping_slot_frequency: u32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ExtraChannel {
    pub frequency: u32,
    pub min_dr: u8,
    pub max_dr: u8,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RegionGateway {
    pub force_gws_private: bool,
    pub backend: GatewayBackend,
    pub channels: Vec<GatewayChannel>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GatewayBackend {
    pub enabled: String,
    pub mqtt: GatewayBackendMqtt,
}

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GatewayBackendMqtt {
    pub event_topic: String,
    pub command_topic: String,
    pub server: String,
    pub username: String,
    pub password: String,
    pub qos: usize,
    pub clean_session: bool,
    pub client_id: String,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
}

#[derive(Serialize, Deserialize, Clone, Hash)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum GatewayChannelModulation {
    LORA,
    FSK,
}

#[derive(Serialize, Deserialize, Clone, Hash)]
#[serde(default)]
pub struct GatewayChannel {
    pub frequency: u32,
    pub bandwidth: u32,
    pub modulation: GatewayChannelModulation,
    pub spreading_factors: Vec<u32>,
    pub datarate: u32,
}

impl Default for GatewayChannel {
    fn default() -> Self {
        GatewayChannel {
            frequency: 0,
            bandwidth: 0,
            modulation: GatewayChannelModulation::LORA,
            spreading_factors: vec![],
            datarate: 0,
        }
    }
}

pub fn load(config_dir: &Path) -> Result<()> {
    let mut content: String = String::new();

    let paths = fs::read_dir(config_dir)?;
    for path in paths {
        let path = path.unwrap().path();

        if let Some(ext) = path.extension() {
            if ext == "toml" {
                content.push_str(
                    &fs::read_to_string(&path)
                        .context(format!("Read config file: {}", path.display()))?,
                );
            }
        }
    }

    // substitute environment variables in config file
    for (k, v) in env::vars() {
        content = content.replace(&format!("${}", k), &v);
    }

    let conf: Configuration = toml::from_str(&content)?;
    set(conf);

    Ok(())
}

pub fn set(c: Configuration) {
    let mut conf_mutex = CONFIG.lock().unwrap();
    *conf_mutex = Arc::new(c);
}

pub fn get() -> Arc<Configuration> {
    let conf = CONFIG.lock().unwrap();
    conf.clone()
}

pub fn get_force_gws_private(region_name: &str) -> Result<bool> {
    let conf = get();
    for region in &conf.regions {
        if region.name == region_name {
            return Ok(region.gateway.force_gws_private);
        }
    }

    Err(anyhow!("region_name not found"))
}

pub fn get_region_network(region_name: &str) -> Result<RegionNetwork> {
    let conf = get();
    for region in &conf.regions {
        if region.name == region_name {
            return Ok(region.network.clone());
        }
    }

    Err(anyhow!("region_name not found"))
}

pub fn get_region_gateway(region_name: &str) -> Result<RegionGateway> {
    let conf = get();
    for region in &conf.regions {
        if region.name == region_name {
            return Ok(region.gateway.clone());
        }
    }

    Err(anyhow!("region_name not found"))
}

pub fn get_required_snr_for_sf(sf: u8) -> Result<f32> {
    Ok(match sf {
        6 => -5.0,
        7 => -7.5,
        8 => -10.0,
        9 => -12.5,
        10 => -15.0,
        11 => -17.5,
        12 => -20.0,
        _ => {
            return Err(anyhow!("Unknown sf {} for get_required_snr_for_sf", sf));
        }
    })
}
