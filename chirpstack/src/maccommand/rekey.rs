use anyhow::Result;
use tracing::info;

use crate::storage::device;

const SERV_LORAWAN_VERSION: lrwn::Version = lrwn::Version::LoRaWAN1_1;

pub fn handle(
    dev: &device::Device,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let block_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let req_pl = if let lrwn::MACCommand::RekeyInd(pl) = block_mac {
        pl
    } else {
        return Err(anyhow!("RekeyInd expected"));
    };

    info!(dev = %dev.dev_eui, dev_lorawan_version_minor = %req_pl.dev_lorawan_version, serv_lorawan_version = %SERV_LORAWAN_VERSION, "RekeyInd received");

    Ok(Some(lrwn::MACCommandSet::new(vec![
        lrwn::MACCommand::RekeyConf(lrwn::RekeyConfPayload {
            serv_lorawan_version: if SERV_LORAWAN_VERSION.to_u8()
                > req_pl.dev_lorawan_version.to_u8()
            {
                req_pl.dev_lorawan_version
            } else {
                SERV_LORAWAN_VERSION
            },
        }),
    ])))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_handle() {
        let resp = handle(
            &device::Device {
                ..Default::default()
            },
            &lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RekeyInd(lrwn::RekeyIndPayload {
                dev_lorawan_version: lrwn::Version::LoRaWAN1_1,
            })]),
        )
        .unwrap();

        assert_eq!(
            Some(lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RekeyConf(
                lrwn::RekeyConfPayload {
                    serv_lorawan_version: lrwn::Version::LoRaWAN1_1,
                }
            )])),
            resp
        );
    }
}
