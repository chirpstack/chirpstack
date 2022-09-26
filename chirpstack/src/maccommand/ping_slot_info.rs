use anyhow::Result;
use tracing::info;

use crate::storage::device;
use chirpstack_api::internal;

pub fn handle(
    dev: &device::Device,
    ds: &mut internal::DeviceSession,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;

    let pl = if let lrwn::MACCommand::PingSlotInfoReq(pl) = &mac {
        pl
    } else {
        return Err(anyhow!("PingSlotInfoReq expected"));
    };

    ds.class_b_ping_slot_nb = 1 << (7 - pl.periodicity);

    info!(dev_eui = %dev.dev_eui, periodicity = pl.periodicity, ping_slot_nb = ds.class_b_ping_slot_nb, "PingSlotInfoReq received");

    Ok(Some(lrwn::MACCommandSet::new(vec![
        lrwn::MACCommand::PingSlotInfoAns,
    ])))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_handle() {
        let mut ds: internal::DeviceSession = Default::default();
        let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::PingSlotInfoReq(
            lrwn::PingSlotInfoReqPayload { periodicity: 3 },
        )]);
        let res = handle(
            &device::Device {
                ..Default::default()
            },
            &mut ds,
            &block,
        )
        .unwrap();
        assert_eq!(16, ds.class_b_ping_slot_nb);
        assert_eq!(
            Some(lrwn::MACCommandSet::new(vec![
                lrwn::MACCommand::PingSlotInfoAns,
            ])),
            res
        );
    }
}
