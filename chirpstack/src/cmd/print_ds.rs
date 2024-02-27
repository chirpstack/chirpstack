use anyhow::{Context, Result};

use crate::storage;
use crate::storage::device;
use lrwn::EUI64;

pub async fn run(dev_eui: &EUI64) -> Result<()> {
    storage::setup().await.context("Setup storage")?;

    let d = device::get(dev_eui).await.context("Get device")?;
    let ds = d.get_device_session()?;
    let json = serde_json::to_string_pretty(&ds)?;
    println!("{}", json);

    Ok(())
}
