use std::env;
use std::sync::{Mutex, Once};

use crate::{adr, config, region, storage};

mod assert;
mod class_a_pr_test;
mod class_a_test;
mod class_b_test;
mod class_c_test;
mod multicast_test;
mod otaa_js_test;
mod otaa_pr_test;
mod otaa_test;
mod relay_class_a_test;
mod relay_otaa_test;

static TRACING_INIT: Once = Once::new();

lazy_static! {
    static ref TEST_MUX: Mutex<()> = Mutex::new(());
}

pub async fn prepare<'a>() -> std::sync::MutexGuard<'a, ()> {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.local").ok();

    // Set a mutex lock to make sure database dependent tests are not overlapping. At the end of
    // the function the guard is returned, so that the mutex guard can be kept during the lifetime
    // of the function running the tests.
    let guard = TEST_MUX.lock().unwrap();

    // Set logger
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });

    // set test config
    let mut conf: config::Configuration = Default::default();
    conf.postgresql.dsn = env::var("TEST_POSTGRESQL_DSN").unwrap();
    conf.redis.servers = vec![env::var("TEST_REDIS_URL").unwrap()];
    conf.network.enabled_regions = vec!["eu868".to_string()];
    conf.regions = vec![config::Region {
        id: "eu868".to_string(),
        description: "EU868".to_string(),
        common_name: lrwn::region::CommonName::EU868,
        user_info: "".into(),
        network: config::RegionNetwork {
            installation_margin: 10.0,
            rx1_delay: 1,
            rx2_frequency: 869525000,
            gateway_prefer_min_margin: 10.0,
            downlink_tx_power: -1,
            min_dr: 0,
            max_dr: 5,
            uplink_max_eirp: 16.0,
            class_b: config::ClassB {
                ping_slot_dr: 0,
                ping_slot_frequency: 868100000,
            },
            extra_channels: Vec::new(),
            enabled_uplink_channels: Vec::new(),
            ..Default::default()
        },
        gateway: config::RegionGateway {
            force_gws_private: false,
            channels: vec![],
            backend: config::GatewayBackend {
                enabled: "mqtt".into(),
                mqtt: config::GatewayBackendMqtt {
                    topic_prefix: "eu868".into(),
                    ..Default::default()
                },
            },
        },
    }];
    config::set(conf);

    // setup storage
    storage::setup().await.unwrap();

    // reset db
    storage::reset_db().unwrap();

    // flush redis db
    storage::reset_redis().await.unwrap();

    // setup region
    region::setup().unwrap();

    // setup adr
    adr::setup().await.unwrap();

    return guard;
}
