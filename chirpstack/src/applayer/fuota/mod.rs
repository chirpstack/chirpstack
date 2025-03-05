use tracing::info;

pub mod flow;
pub mod scheduler;

pub async fn setup() {
    info!("Setting up FUOTA scheduler loop");
    tokio::spawn(scheduler::scheduler_loop());
}
