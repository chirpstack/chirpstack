use anyhow::Result;
use tokio::time::sleep;
use tracing::{error, span, trace, Instrument, Level};

use crate::applayer::fuota::flow;
use crate::config;
use crate::storage::fuota;

pub async fn scheduler_loop() {
    let conf = config::get();

    loop {
        trace!("Starting fuota scheduler_loop run");
        if let Err(err) = schedule_batch(conf.network.scheduler.batch_size).await {
            error!(error = %err, "Scheduling FUOTA batch error");
        } else {
            trace!("schedule_batch completed without error");
        }
        sleep(conf.network.scheduler.interval).await;
    }
}

async fn schedule_batch(size: usize) -> Result<()> {
    trace!("Get schedulable fuota jobs");
    let jobs = fuota::get_schedulable_jobs(size).await?;
    trace!(job_count = jobs.len(), "Got this number of fuota jobs");

    let mut handles = vec![];

    for job in jobs {
        // Spawn the batch as async tasks.
        let handle = tokio::spawn(async move {
            let span = span!(Level::INFO, "job", fuota_deployment_id = %job.fuota_deployment_id, job = %job.job);

            if let Err(e) = flow::Flow::handle_job(job).instrument(span).await {
                error!(error = %e, "Handle FUOTA job error");
            }
        });
        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    Ok(())
}
