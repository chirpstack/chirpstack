use anyhow::Result;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use tracing::info;

use crate::api::helpers::ToProto;
use crate::integration;
use crate::storage::{application, device, device_profile, tenant};
use crate::uplink::{helpers, UplinkFrameSet};
use chirpstack_api::integration as integration_pb;

pub async fn handle(
    uplink_frame_set: &UplinkFrameSet,
    tenant: &tenant::Tenant,
    app: &application::Application,
    dp: &device_profile::DeviceProfile,
    dev: &device::Device,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("Expected DevStatusAns"))?;
    if let lrwn::MACCommand::DevStatusAns(pl) = mac {
        info!(dev_eui = %dev.dev_eui, battery = pl.battery, margin = pl.margin, "DevStatusAns received");

        device::set_status(
            &dev.dev_eui,
            pl.margin as i32,
            pl.battery == 0,
            if pl.battery > 0 && pl.battery < 255 {
                let v: BigDecimal = ((pl.battery as f32) / 254.0 * 100.0).try_into()?;
                Some(v.with_scale(2))
            } else {
                None
            },
        )
        .await?;

        let mut tags = (*dp.tags).clone();
        tags.clone_from(&*dev.tags);

        let rx_time: DateTime<Utc> =
            helpers::get_rx_timestamp(&uplink_frame_set.rx_info_set).into();

        integration::status_event(
            app.id,
            &dev.variables,
            &integration_pb::StatusEvent {
                deduplication_id: uplink_frame_set.uplink_set_id.to_string(),
                time: Some(rx_time.into()),
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_id: tenant.id.to_string(),
                    tenant_name: tenant.name.clone(),
                    application_id: app.id.to_string(),
                    application_name: app.name.to_string(),
                    device_profile_id: dp.id.to_string(),
                    device_profile_name: dp.name.clone(),
                    device_name: dev.name.clone(),
                    device_class_enabled: dev.enabled_class.to_proto().into(),
                    dev_eui: dev.dev_eui.to_string(),
                    tags,
                }),
                margin: pl.margin as i32,
                external_power_source: pl.battery == 0,
                battery_level_unavailable: pl.battery == 255,
                battery_level: if pl.battery > 0 && pl.battery < 255 {
                    (pl.battery as f32) / 254.0 * 100.0
                } else {
                    0.0
                },
            },
        )
        .await;
    }

    Ok(None)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::integration::mock;
    use crate::{integration, test};
    use chirpstack_api::gw;
    use chrono::Utc;
    use lrwn::EUI64;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_handle() {
        let _guard = test::prepare().await;
        integration::set_mock().await;

        let rx_time = Utc::now();

        let ufs = UplinkFrameSet {
            uplink_set_id: Uuid::new_v4(),
            dr: 0,
            ch: 0,
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: Default::default(),
                    f_port: None,
                    frm_payload: None,
                }),
                mic: None,
            },
            tx_info: Default::default(),
            rx_info_set: vec![gw::UplinkRxInfo {
                time: Some(rx_time.into()),
                ..Default::default()
            }],
            gateway_private_up_map: HashMap::new(),
            gateway_private_down_map: HashMap::new(),
            gateway_tenant_id_map: HashMap::new(),
            region_common_name: lrwn::region::CommonName::EU868,
            region_config_id: "eu868".into(),
            roaming_meta_data: None,
        };

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

        let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::DevStatusAns(
            lrwn::DevStatusAnsPayload {
                battery: 254,
                margin: 10,
            },
        )]);

        let resp = handle(&ufs, &tenant, &app, &dp, &dev, &block)
            .await
            .unwrap();
        assert_eq!(true, resp.is_none());

        // Integration events are handled async.
        sleep(Duration::from_millis(100)).await;

        let status_events = mock::get_status_events().await;
        assert_eq!(
            vec![integration_pb::StatusEvent {
                deduplication_id: ufs.uplink_set_id.to_string(),
                time: Some(rx_time.into()),
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_id: tenant.id.to_string(),
                    tenant_name: tenant.name.clone(),
                    application_id: app.id.to_string(),
                    application_name: app.name.clone(),
                    device_profile_id: dp.id.to_string(),
                    device_profile_name: dp.name.clone(),
                    dev_eui: dev.dev_eui.to_string(),
                    device_name: dev.name.clone(),
                    ..Default::default()
                }),
                margin: 10,
                external_power_source: false,
                battery_level_unavailable: false,
                battery_level: 100.0,
            }],
            status_events
        );

        let d = device::get(&dev.dev_eui).await.unwrap();
        assert_eq!(Some(10), d.margin);
        assert_eq!(false, d.external_power_source);
        assert_eq!(
            Some(BigDecimal::from_str("100.00").unwrap()),
            d.battery_level
        );
    }
}
