use anyhow::Result;
use chrono::Utc;
use tracing::info;

use crate::api::helpers::ToProto;
use crate::integration;
use crate::storage::{application, device, device_profile, tenant};
use chirpstack_api::integration as integration_pb;

pub async fn handle(
    t: &tenant::Tenant,
    dp: &device_profile::DeviceProfile,
    app: &application::Application,
    dev: &device::Device,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let req_mac = (**block)
        .first()
        .ok_or_else(|| anyhow!("MACCommandSet is empty"))?;
    let req_pl = if let lrwn::MACCommand::NotifyNewEndDeviceReq(pl) = req_mac {
        pl
    } else {
        return Err(anyhow!("NotifyNewEndDeviceReq is expected"));
    };

    info!(dev_eui = %dev.dev_eui, ed_dev_addr = %req_pl.dev_addr, "NotifyNewEndDeviceReq received");

    let device_info = integration_pb::DeviceInfo {
        tenant_id: t.id.to_string(),
        tenant_name: t.name.clone(),
        application_id: app.id.to_string(),
        application_name: app.name.to_string(),
        device_profile_id: dp.id.to_string(),
        device_profile_name: dp.name.clone(),
        device_name: dev.name.clone(),
        device_class_enabled: dev.enabled_class.to_proto().into(),
        dev_eui: dev.dev_eui.to_string(),
        tags: {
            let mut tags = (*dp.tags).clone();
            tags.extend((*dev.tags).clone());
            tags
        },
    };

    let log_event = integration_pb::LogEvent {
        time: Some(Utc::now().into()),
        device_info: Some(device_info),
        level: integration_pb::LogLevel::Info.into(),
        code: integration_pb::LogCode::RelayNewEndDevice.into(),
        description: "NotifyNewEndDevice received from relay".into(),
        context: [
            ("dev_addr".to_string(), req_pl.dev_addr.to_string()),
            (
                "wor_snr".to_string(),
                req_pl.power_level.wor_snr.to_string(),
            ),
            (
                "wor_rssi".to_string(),
                req_pl.power_level.wor_rssi.to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    integration::log_event(app.id, &dev.variables, &log_event).await;

    Ok(None)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;

    use chirpstack_api::common;

    use crate::storage::fields;
    use crate::test;

    #[tokio::test]
    async fn test_handle() {
        let _guard = test::prepare().await;
        integration::set_mock().await;

        let t = tenant::Tenant {
            id: Uuid::new_v4(),
            name: "tenant".to_string(),
            ..Default::default()
        };

        let app = application::Application {
            id: Uuid::new_v4(),
            name: "app".to_string(),
            ..Default::default()
        };

        let dp = device_profile::DeviceProfile {
            id: Uuid::new_v4(),
            name: "dp".to_string(),
            tags: fields::KeyValue::new(
                [("dp_tag".to_string(), "dp_value".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ..Default::default()
        };

        let dev = device::Device {
            dev_eui: lrwn::EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            name: "dev".to_string(),
            tags: fields::KeyValue::new(
                [("dev_tag".to_string(), "dev_value".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            ..Default::default()
        };

        let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::NotifyNewEndDeviceReq(
            lrwn::NotifyNewEndDeviceReqPayload {
                dev_addr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                power_level: lrwn::PowerLevel {
                    wor_snr: 10,
                    wor_rssi: 20,
                },
            },
        )]);

        handle(&t, &dp, &app, &dev, &block).await.unwrap();
        sleep(Duration::from_millis(100)).await;
        let mock_events = integration::mock::get_log_events().await;
        assert_eq!(1, mock_events.len());

        let mut mock_event = mock_events[0].clone();
        assert!(mock_event.time.is_some());
        mock_event.time = None;

        assert_eq!(
            integration_pb::LogEvent {
                time: None,
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_id: t.id.to_string(),
                    tenant_name: "tenant".to_string(),
                    application_id: app.id.to_string(),
                    application_name: "app".to_string(),
                    device_profile_id: dp.id.to_string(),
                    device_profile_name: "dp".to_string(),
                    device_name: "dev".to_string(),
                    device_class_enabled: common::DeviceClass::ClassA.into(),
                    dev_eui: dev.dev_eui.to_string(),
                    tags: [
                        ("dp_tag".to_string(), "dp_value".to_string()),
                        ("dev_tag".to_string(), "dev_value".to_string()),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                }),
                level: integration_pb::LogLevel::Info.into(),
                code: integration_pb::LogCode::RelayNewEndDevice.into(),
                description: "NotifyNewEndDevice received from relay".into(),
                context: [
                    ("dev_addr".to_string(), "01020304".to_string()),
                    ("wor_snr".to_string(), "10".to_string()),
                    ("wor_rssi".to_string(), "20".to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
            mock_event
        );
    }
}
