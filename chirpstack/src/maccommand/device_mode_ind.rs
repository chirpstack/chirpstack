use anyhow::Result;

use crate::storage::device::{self, DeviceClass};

pub async fn handle(
    dev: &device::Device,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("Expected DeviceModeInd"))?;
    if let lrwn::MACCommand::DeviceModeInd(pl) = mac {
        device::set_enabled_class(
            &dev.dev_eui,
            match pl.class {
                lrwn::DeviceModeClass::ClassA => DeviceClass::A,
                lrwn::DeviceModeClass::ClassC => DeviceClass::C,
            },
        )
        .await?;

        return Ok(Some(lrwn::MACCommandSet::new(vec![
            lrwn::MACCommand::DeviceModeConf(lrwn::DeviceModeConfPayload { class: pl.class }),
        ])));
    }

    Err(anyhow!("Expected DeviceModeInd payload"))
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;
    use std::str::FromStr;

    use crate::storage::{application, device_profile, tenant};
    use lrwn::EUI64;

    #[tokio::test]
    async fn test_handle() {
        let _guard = test::prepare().await;

        let tenant = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let app = application::create(application::Application {
            tenant_id: tenant.id.clone(),
            name: "test-app".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: tenant.id.clone(),
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let dev = device::create(device::Device {
            application_id: app.id.clone(),
            device_profile_id: dp.id.clone(),
            dev_eui: EUI64::from_str("0102030405060708").unwrap(),
            name: "test-device".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::DeviceModeInd(
            lrwn::DeviceModeIndPayload {
                class: lrwn::DeviceModeClass::ClassC,
            },
        )]);

        let resp = handle(&dev, &block).await.unwrap();

        assert_eq!(
            Some(lrwn::MACCommandSet::new(vec![
                lrwn::MACCommand::DeviceModeConf(lrwn::DeviceModeConfPayload {
                    class: lrwn::DeviceModeClass::ClassC,
                })
            ])),
            resp
        );

        let d = device::get(&dev.dev_eui).await.unwrap();
        assert_eq!(DeviceClass::C, d.enabled_class);
    }
}
