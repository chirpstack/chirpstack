use std::sync::{Mutex, Once};

use crate::{adr, config, region, storage};

mod assert;
mod class_a_pr_test;
mod class_a_test;
mod class_b_test;
mod class_c_test;
mod multicast_test;
mod otaa_pr_test;
mod otaa_test;

static TRACING_INIT: Once = Once::new();

lazy_static! {
    static ref TEST_MUX: Mutex<()> = Mutex::new(());
}

pub async fn prepare<'a>() -> std::sync::MutexGuard<'a, ()> {
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
    conf.postgresql.dsn =
        "postgres://chirpstack_test:chirpstack_test@postgres/chirpstack_test?sslmode=disable"
            .to_string();
    conf.redis.servers = vec!["redis://redis/".to_string()];
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
