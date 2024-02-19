use std::io::Cursor;

use anyhow::{Context, Result};
use prost::Message;

use crate::storage;
use crate::storage::device;
use chirpstack_api::internal;
use lrwn::EUI64;

pub async fn run(dev_eui: &EUI64) -> Result<()> {
    storage::setup().await.context("Setup storage")?;

    let d = device::get(dev_eui).await.context("Get device")?;
    let ds = match d.device_session {
        Some(v) => internal::DeviceSession::decode(&mut Cursor::new(&v))
            .context("Decode device-session")?,
        None => return Err(anyhow!("No device-session")),
    };

    let json = serde_json::to_string_pretty(&ds)?;
    println!("{}", json);

    Ok(())
}
