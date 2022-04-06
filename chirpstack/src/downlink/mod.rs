use tracing::info;

pub mod classb;
pub mod data;
mod helpers;
pub mod join;
pub mod multicast;
pub mod scheduler;
pub mod tx_ack;

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
