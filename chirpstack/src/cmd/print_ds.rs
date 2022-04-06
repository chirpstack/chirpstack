use anyhow::{Context, Result};

use crate::storage;
use crate::storage::device_session;
use lrwn::EUI64;

pub async fn run(dev_eui: &EUI64) -> Result<()> {
    storage::setup().await.context("Setup storage")?;
    let ds = device_session::get(dev_eui)
        .await
        .context("Get device-session")?;
    let json = serde_json::to_string_pretty(&ds)?;
    println!("{}", json);

    Ok(())
}
