use anyhow::Result;
use tracing::{span, warn, Instrument, Level};

use crate::storage::{device, device_profile};
use chirpstack_api::gw;

pub mod clocksync;

pub async fn handle_uplink(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    f_port: u8,
    data: &[u8],
) {
    if let Err(e) = _handle_uplink(dev, dp, rx_info, f_port, data).await {
        warn!(error = %e, "Handle applayer payload error");
    }
}

async fn _handle_uplink(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    f_port: u8,
    data: &[u8],
) -> Result<()> {
    if dp.app_layer_params.ts003_f_port == f_port {
        let span = span!(Level::INFO, "ts003");
        clocksync::handle_uplink(dev, dp, rx_info, data)
            .instrument(span)
            .await
    } else if dp.app_layer_params.ts004_f_port == f_port {
        unimplemented!()
    } else if dp.app_layer_params.ts005_f_port == f_port {
        unimplemented!()
    } else {
        return Err(anyhow!("Unexpected f_port {}", f_port));
    }
}
