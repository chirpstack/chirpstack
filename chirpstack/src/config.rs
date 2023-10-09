use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, fs};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use lrwn::region::CommonName;
use lrwn::{AES128Key, DevAddrPrefix, NetID, EUI64};

lazy_static! {
    static ref CONFIG: Mutex<Arc<Configuration>> = Mutex::new(Arc::new(Default::default()));
}

#[derive(Default, Serialize, Deserialize, Clone)]
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
    pub backend_interfaces: BackendInterfaces,
    pub roaming: Roaming,
    pub keks: Vec<Kek>,
    pub regions: Vec<Region>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Logging {
    pub level: String,
    pub json: bool,
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "info".into(),
            json: false,
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
    pub allow_unknown_gateways: bool,
}

impl Default for Gateway {
    fn default() -> Self {
        Gateway {
            client_cert_lifetime: Duration::from_secs(60 * 60 * 24 * 365),
            ca_cert: "".to_string(),
            ca_key: "".to_string(),
            allow_unknown_gateways: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Network {
    pub net_id: NetID,
    pub secondary_net_ids: Vec<NetID>,
    pub dev_addr_prefixes: Vec<DevAddrPrefix>,
    pub enabled_regions: Vec<String>,
    #[serde(with = "humantime_serde")]
    pub device_session_ttl: Duration,
    #[serde(with = "humantime_serde")]
    pub deduplication_delay: Duration,
    #[serde(with = "humantime_serde")]
    pub get_downlink_data_delay: Duration,
    pub mac_commands_disabled: bool,
    pub adr_plugins: Vec<String>,
    pub scheduler: Scheduler,
}

impl Default for Network {
    fn default() -> Self {
        Network {
            net_id: NetID::from_be_bytes([0x00, 0x00, 0x00]),
            secondary_net_ids: vec![],
            dev_addr_prefixes: vec![],
            enabled_regions: vec![],
            device_session_ttl: Duration::from_secs(60 * 60 * 24 * 31),
            deduplication_delay: Duration::from_millis(200),
            get_downlink_data_delay: Duration::from_millis(100),
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
            multicast_class_c_margin: Duration::from_secs(5),
            multicast_class_b_margin: Duration::from_secs(5),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Monitoring {
    pub bind: String,
    pub api_request_log_max_history: usize,
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
            bind: "".to_string(),
            api_request_log_max_history: 10,
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
    pub postgresql: PostgresqlIntegration,
    pub amqp: AmqpIntegration,
    pub kafka: KafkaIntegration,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct MqttIntegration {
    pub client: MqttIntegrationClient,
    pub event_topic: String,
    pub command_topic: String,
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
    #[serde(with = "humantime_serde")]
    pub keep_alive_interval: Duration,
}

impl Default for MqttIntegration {
    fn default() -> Self {
        MqttIntegration {
            client: Default::default(),
            event_topic: "application/{{application_id}}/device/{{dev_eui}}/event/{{event}}".into(),
            command_topic: "application/{{application_id}}/device/{{dev_eui}}/command/{{command}}"
                .into(),
            json: true,
            server: "tcp://127.0.0.1:1883/".into(),
            username: "".into(),
            password: "".into(),
            qos: 0,
            clean_session: false,
            client_id: "".into(),
            ca_cert: "".into(),
            tls_cert: "".into(),
            tls_key: "".into(),
            keep_alive_interval: Duration::from_secs(30),
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct PostgresqlIntegration {
    pub dsn: String,
    pub max_open_connections: u32,
    pub min_idle_connections: u32,
}

impl Default for PostgresqlIntegration {
    fn default() -> Self {
        PostgresqlIntegration {
            dsn: "postgresql://chirpstack_integration:chirpstack_integration@localhost/chirpstack_integration?sslmode=disable".into(),
            max_open_connections: 10,
            min_idle_connections: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AmqpIntegration {
    pub url: String,
    pub json: bool,
    pub event_routing_key: String,
}

impl Default for AmqpIntegration {
    fn default() -> Self {
        AmqpIntegration {
            url: "amqp://guest:guest@localhost:5672".to_string(),
            json: true,
            event_routing_key: "application.{{application_id}}.device.{{dev_eui}}.event.{{event}}"
                .to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct KafkaIntegration {
    pub brokers: Vec<String>,
    pub tls: bool,
    pub topic: String,
    pub event_key: String,
    pub username: String,
    pub password: String,
    pub mechanism: String,
    pub json: bool,
}

impl Default for KafkaIntegration {
    fn default() -> Self {
        KafkaIntegration {
            brokers: vec!["localhost:9092".to_string()],
            tls: false,
            topic: "chirpstack".to_string(),
            event_key: "application.{{application_id}}.device.{{dev_eui}}.event.{{event}}"
                .to_string(),
            username: "".to_string(),
            password: "".to_string(),
            mechanism: "PLAIN".to_string(),
            json: true,
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
    #[serde(with = "humantime_serde")]
    pub async_timeout: Duration,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Roaming {
    pub resolve_net_id_domain_suffix: String,
    pub servers: Vec<RoamingServer>,
    pub default: RoamingServerDefault,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct BackendInterfaces {
    pub bind: String,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct RoamingServer {
    pub net_id: NetID,
    #[serde(with = "humantime_serde")]
    pub async_timeout: Duration,
    #[serde(with = "humantime_serde")]
    pub passive_roaming_lifetime: Duration,
    pub passive_roaming_kek_label: String,
    pub server: String,
    pub use_target_role_suffix: bool,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
    pub authorization_header: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct RoamingServerDefault {
    pub enabled: bool,
    #[serde(with = "humantime_serde")]
    pub async_timeout: Duration,
    #[serde(with = "humantime_serde")]
    pub passive_roaming_lifetime: Duration,
    pub passive_roaming_kek_label: String,
    pub server: String,
    pub use_target_role_suffix: bool,
    pub ca_cert: String,
    pub tls_cert: String,
    pub tls_key: String,
    pub authorization_header: String,
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
    #[serde(alias = "name")]
    pub id: String,
    pub description: String,
    pub common_name: CommonName,
    pub user_info: String,
    pub network: RegionNetwork,
    pub gateway: RegionGateway,
}

impl Default for Region {
    fn default() -> Self {
        Region {
            id: "".to_string(),
            description: "".to_string(),
            common_name: CommonName::EU868,
            user_info: "".into(),
            network: RegionNetwork::default(),
            gateway: RegionGateway::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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

impl Default for RegionNetwork {
    fn default() -> Self {
        RegionNetwork {
            installation_margin: 10.0,
            rx_window: 0,
            rx1_delay: 1,
            rx1_dr_offset: 0,
            rx2_dr: 0,
            rx2_frequency: 0,
            rx2_prefer_on_rx1_dr_lt: 0,
            rx2_prefer_on_link_budget: false,
            gateway_prefer_min_margin: 10.0,
            downlink_tx_power: -1,
            adr_disabled: false,
            min_dr: 0,
            max_dr: 0,
            uplink_dwell_time_400ms: false,
            downlink_dwell_time_400ms: false,
            uplink_max_eirp: 0.0,
            rejoin_request: RejoinRequest::default(),
            class_b: ClassB::default(),
            extra_channels: vec![],
            enabled_uplink_channels: vec![],
            repeater_compatible: false,
            dwell_time_400ms: false,
        }
    }
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GatewayBackendMqtt {
    pub topic_prefix: String,
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
    #[serde(with = "humantime_serde")]
    pub keep_alive_interval: Duration,
    pub v4_migrate: bool,
}

impl Default for GatewayBackendMqtt {
    fn default() -> Self {
        GatewayBackendMqtt {
            topic_prefix: "".into(),
            event_topic: "".into(),
            command_topic: "".into(),
            server: "tcp://127.0.0.1:1883/".into(),
            username: "".into(),
            password: "".into(),
            qos: 0,
            clean_session: false,
            client_id: "".into(),
            ca_cert: "".into(),
            tls_cert: "".into(),
            tls_key: "".into(),
            keep_alive_interval: Duration::from_secs(30),
            v4_migrate: true,
        }
    }
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

pub fn get_force_gws_private(region_id: &str) -> Result<bool> {
    let conf = get();
    for region in &conf.regions {
        if region.id == region_id {
            return Ok(region.gateway.force_gws_private);
        }
    }

    Err(anyhow!("region_config_id not found"))
}

pub fn get_region_network(region_id: &str) -> Result<RegionNetwork> {
    let conf = get();
    for region in &conf.regions {
        if region.id == region_id {
            return Ok(region.network.clone());
        }
    }

    Err(anyhow!("Region ID '{}' not found", region_id))
}

pub fn get_region_gateway(region_id: &str) -> Result<RegionGateway> {
    let conf = get();
    for region in &conf.regions {
        if region.id == region_id {
            return Ok(region.gateway.clone());
        }
    }

    Err(anyhow!("Region ID '{}' not found", region_id))
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
