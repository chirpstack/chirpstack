use anyhow::Result;
use tracing::info;

use chirpstack_api::internal;

pub fn set_pending(ds: &mut internal::DeviceSession, set: &lrwn::MACCommandSet) -> Result<()> {
    let cid = set.cid()?;
    let b = set.to_vec()?;
    ds.mac_command_pending.insert(cid.to_u8().into(), b);
    info!(cid = %cid, "Pending mac-command block set");
    Ok(())
}

pub async fn get_pending(
    ds: &mut internal::DeviceSession,
    cid: lrwn::CID,
) -> Result<Option<lrwn::MACCommandSet>> {
    let b = ds
        .mac_command_pending
        .remove(&cid.to_u8().into())
        .unwrap_or_default();

    let out = if !b.is_empty() {
        let mut mac = lrwn::MACCommandSet::from_slice(&b);

        // Per definition, the uplink flag is set to false as this function is intended to retrieve
        // pending mac-commands that were previously sent to the device.
        mac.decode_from_raw(false)?;

        Some(mac)
    } else {
        None
    };

    Ok(out)
}
