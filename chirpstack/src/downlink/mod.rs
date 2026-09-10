use std::sync::LazyLock;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use tracing::info;
use crate::monitoring::prometheus;

pub mod classb;
pub mod data;
pub mod data_fns;
pub mod error;
mod helpers;
pub mod join;
pub mod multicast;
pub mod roaming;
pub mod scheduler;
pub mod tx_ack;

#[derive(Clone, Hash, PartialEq, Eq, EncodeLabelSet, Debug)]
struct DownlinkMacCommandLabels {
    mac_command: String,
}

static DOWNLINK_MAC_COMMAND_COUNTER: LazyLock<Family<DownlinkMacCommandLabels, Counter>> = LazyLock::new(|| {
    let counter = Family::<DownlinkMacCommandLabels, Counter>::default();
    prometheus::register(
        "downlink_mac_command_count",
        "Number of sent mac commands in downlinks",
        counter.clone(),
    );
    counter
});

pub async fn setup() {
    info!("Setting up Class-B/C scheduler loop");
    tokio::spawn(async move {
        scheduler::class_b_c_scheduler_loop().await;
    });

    info!("Setting up multicast scheduler loop");
    tokio::spawn(async move {
        scheduler::multicast_group_queue_scheduler_loop().await;
    });
}

pub fn record_downlink_mac_commands(mac_command_sets: Vec<lrwn::MACCommandSet>) {
    mac_command_sets.iter().for_each(|mac_command_set| {
        mac_command_set.iter().for_each(|mac_command| {
            DOWNLINK_MAC_COMMAND_COUNTER
                .get_or_create(&DownlinkMacCommandLabels {
                    mac_command: mac_command.to_string(),
                })
                .inc();
        })
    })
}
