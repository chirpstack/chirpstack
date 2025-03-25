use anyhow::Result;
use tracing::info;

use crate::gpstime::ToGpsTime;
use crate::storage::fields::device_profile::Ts003Version;
use crate::storage::{device, device_profile, device_queue};
use crate::uplink::helpers;
use chirpstack_api::gw;
use lrwn::applayer::clocksync;

pub async fn handle_uplink(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    data: &[u8],
) -> Result<()> {
    let version = dp
        .app_layer_params
        .ts003_version
        .ok_or_else(|| anyhow!("Device does not support TS003"))?;

    match version {
        Ts003Version::V100 => handle_uplink_v100(dev, dp, rx_info, data).await,
        Ts003Version::V200 => handle_uplink_v200(dev, dp, rx_info, data).await,
    }
}

async fn handle_uplink_v100(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    data: &[u8],
) -> Result<()> {
    let pl = clocksync::v1::Payload::from_slice(true, data)?;

    if let clocksync::v1::Payload::AppTimeReq(pl) = pl {
        handle_v1_app_time_req(dev, dp, rx_info, pl).await?
    }

    Ok(())
}

async fn handle_uplink_v200(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    data: &[u8],
) -> Result<()> {
    let pl = clocksync::v2::Payload::from_slice(true, data)?;

    if let clocksync::v2::Payload::AppTimeReq(pl) = pl {
        handle_v2_app_time_req(dev, dp, rx_info, pl).await?
    }

    Ok(())
}

async fn handle_v1_app_time_req(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    pl: clocksync::v1::AppTimeReqPayload,
) -> Result<()> {
    info!("Handling AppTimeReq");

    let now_time_since_gps = if let Some(t) = helpers::get_time_since_gps_epoch(rx_info) {
        chrono::Duration::from_std(t)?
    } else {
        helpers::get_rx_timestamp_chrono(rx_info).to_gps_time()
    };
    let dev_time_since_gps = chrono::Duration::seconds(pl.device_time.into());

    let time_diff = (now_time_since_gps - dev_time_since_gps).num_seconds();
    let time_correction: i32 = if time_diff < 0 {
        time_diff.try_into().unwrap_or(i32::MIN)
    } else {
        time_diff.try_into().unwrap_or(i32::MAX)
    };

    if time_diff == 0 && !pl.param.ans_required {
        return Ok(());
    }

    info!(
        time_correcrtion = time_correction,
        "Responding with AppTimeAns"
    );

    let ans = clocksync::v1::Payload::AppTimeAns(clocksync::v1::AppTimeAnsPayload {
        time_correction,
        param: clocksync::v1::AppTimeAnsPayloadParam {
            token_ans: pl.param.token_req,
        },
    });

    device_queue::enqueue_item(device_queue::DeviceQueueItem {
        dev_eui: dev.dev_eui,
        f_port: dp.app_layer_params.ts003_f_port.into(),
        data: ans.to_vec()?,
        ..Default::default()
    })
    .await?;

    Ok(())
}

async fn handle_v2_app_time_req(
    dev: &device::Device,
    dp: &device_profile::DeviceProfile,
    rx_info: &[gw::UplinkRxInfo],
    pl: clocksync::v2::AppTimeReqPayload,
) -> Result<()> {
    info!("Handling AppTimeReq");

    let now_time_since_gps = if let Some(t) = helpers::get_time_since_gps_epoch(rx_info) {
        chrono::Duration::from_std(t)?
    } else {
        helpers::get_rx_timestamp_chrono(rx_info).to_gps_time()
    };
    let dev_time_since_gps = chrono::Duration::seconds(pl.device_time.into());

    let time_diff = (now_time_since_gps - dev_time_since_gps).num_seconds();
    let time_correction: i32 = if time_diff < 0 {
        time_diff.try_into().unwrap_or(i32::MIN)
    } else {
        time_diff.try_into().unwrap_or(i32::MAX)
    };

    if time_diff == 0 && !pl.param.ans_required {
        return Ok(());
    }

    info!(
        time_correcrtion = time_correction,
        "Responding with AppTimeAns"
    );

    let ans = clocksync::v2::Payload::AppTimeAns(clocksync::v2::AppTimeAnsPayload {
        time_correction,
        param: clocksync::v2::AppTimeAnsPayloadParam {
            token_ans: pl.param.token_req,
        },
    });

    device_queue::enqueue_item(device_queue::DeviceQueueItem {
        dev_eui: dev.dev_eui,
        f_port: dp.app_layer_params.ts003_f_port.into(),
        data: ans.to_vec()?,
        ..Default::default()
    })
    .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::applayer::handle_uplink;
    use crate::storage::{application, device_queue, fields, tenant};
    use crate::test;
    use lrwn::EUI64;
    use std::time::Duration;

    #[tokio::test]
    async fn test_handle_v1_app_time_req() {
        struct Test {
            name: String,
            rx_info: gw::UplinkRxInfo,
            req: clocksync::v1::AppTimeReqPayload,
            expected: Option<clocksync::v1::AppTimeAnsPayload>,
        }

        let tests = vec![
            Test {
                name: "device synced".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1234).into()),
                    ..Default::default()
                },
                req: clocksync::v1::AppTimeReqPayload {
                    device_time: 1234,
                    param: clocksync::v1::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: false,
                    },
                },
                expected: None,
            },
            Test {
                name: "device synced - ans required".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1234).into()),
                    ..Default::default()
                },
                req: clocksync::v1::AppTimeReqPayload {
                    device_time: 1234,
                    param: clocksync::v1::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: true,
                    },
                },
                expected: Some(clocksync::v1::AppTimeAnsPayload {
                    time_correction: 0,
                    param: clocksync::v1::AppTimeAnsPayloadParam { token_ans: 8 },
                }),
            },
            Test {
                name: "device not synced (positive correction)".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1234).into()),
                    ..Default::default()
                },
                req: clocksync::v1::AppTimeReqPayload {
                    device_time: 1200,
                    param: clocksync::v1::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: false,
                    },
                },
                expected: Some(clocksync::v1::AppTimeAnsPayload {
                    time_correction: 34,
                    param: clocksync::v1::AppTimeAnsPayloadParam { token_ans: 8 },
                }),
            },
            Test {
                name: "device not synced (negative correction)".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1200).into()),
                    ..Default::default()
                },
                req: clocksync::v1::AppTimeReqPayload {
                    device_time: 1234,
                    param: clocksync::v1::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: false,
                    },
                },
                expected: Some(clocksync::v1::AppTimeAnsPayload {
                    time_correction: -34,
                    param: clocksync::v1::AppTimeAnsPayloadParam { token_ans: 8 },
                }),
            },
        ];

        let _guard = test::prepare().await;
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id,
            app_layer_params: fields::AppLayerParams {
                ts003_version: Some(Ts003Version::V100),
                ..Default::default()
            },
            ..Default::default()
        })
        .await
        .unwrap();

        let d = device::create(device::Device {
            name: "test-dev".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        for tst in &tests {
            println!("> {}", tst.name);
            device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();
            let pl = clocksync::v1::Payload::AppTimeReq(tst.req.clone());

            handle_uplink(
                &d,
                &dp,
                &[tst.rx_info.clone()],
                dp.app_layer_params.ts003_f_port,
                &pl.to_vec().unwrap(),
            )
            .await;

            let queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
            if let Some(expected_pl) = &tst.expected {
                assert_eq!(1, queue_items.len());
                let qi = queue_items.first().unwrap();
                assert_eq!(dp.app_layer_params.ts003_f_port as i16, qi.f_port);

                let qi_pl = clocksync::v1::Payload::from_slice(false, &qi.data).unwrap();
                let expected_pl = clocksync::v1::Payload::AppTimeAns(expected_pl.clone());

                assert_eq!(expected_pl, qi_pl);
            } else {
                assert!(queue_items.is_empty());
            }
        }
    }

    #[tokio::test]
    async fn test_handle_v2_app_time_req() {
        struct Test {
            name: String,
            rx_info: gw::UplinkRxInfo,
            req: clocksync::v2::AppTimeReqPayload,
            expected: Option<clocksync::v2::AppTimeAnsPayload>,
        }

        let tests = vec![
            Test {
                name: "device synced".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1234).into()),
                    ..Default::default()
                },
                req: clocksync::v2::AppTimeReqPayload {
                    device_time: 1234,
                    param: clocksync::v2::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: false,
                    },
                },
                expected: None,
            },
            Test {
                name: "device synced - ans required".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1234).into()),
                    ..Default::default()
                },
                req: clocksync::v2::AppTimeReqPayload {
                    device_time: 1234,
                    param: clocksync::v2::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: true,
                    },
                },
                expected: Some(clocksync::v2::AppTimeAnsPayload {
                    time_correction: 0,
                    param: clocksync::v2::AppTimeAnsPayloadParam { token_ans: 8 },
                }),
            },
            Test {
                name: "device not synced (positive correction)".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1234).into()),
                    ..Default::default()
                },
                req: clocksync::v2::AppTimeReqPayload {
                    device_time: 1200,
                    param: clocksync::v2::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: false,
                    },
                },
                expected: Some(clocksync::v2::AppTimeAnsPayload {
                    time_correction: 34,
                    param: clocksync::v2::AppTimeAnsPayloadParam { token_ans: 8 },
                }),
            },
            Test {
                name: "device not synced (negative correction)".into(),
                rx_info: gw::UplinkRxInfo {
                    time_since_gps_epoch: Some(Duration::from_secs(1200).into()),
                    ..Default::default()
                },
                req: clocksync::v2::AppTimeReqPayload {
                    device_time: 1234,
                    param: clocksync::v2::AppTimeReqPayloadParam {
                        token_req: 8,
                        ans_required: false,
                    },
                },
                expected: Some(clocksync::v2::AppTimeAnsPayload {
                    time_correction: -34,
                    param: clocksync::v2::AppTimeAnsPayloadParam { token_ans: 8 },
                }),
            },
        ];

        let _guard = test::prepare().await;
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id,
            app_layer_params: fields::AppLayerParams {
                ts003_version: Some(Ts003Version::V200),
                ..Default::default()
            },
            ..Default::default()
        })
        .await
        .unwrap();

        let d = device::create(device::Device {
            name: "test-dev".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            application_id: app.id,
            device_profile_id: dp.id,
            ..Default::default()
        })
        .await
        .unwrap();

        for tst in &tests {
            println!("> {}", tst.name);
            device_queue::flush_for_dev_eui(&d.dev_eui).await.unwrap();
            let pl = clocksync::v2::Payload::AppTimeReq(tst.req.clone());

            handle_uplink(
                &d,
                &dp,
                &[tst.rx_info.clone()],
                dp.app_layer_params.ts003_f_port,
                &pl.to_vec().unwrap(),
            )
            .await;

            let queue_items = device_queue::get_for_dev_eui(&d.dev_eui).await.unwrap();
            if let Some(expected_pl) = &tst.expected {
                assert_eq!(1, queue_items.len());
                let qi = queue_items.first().unwrap();
                assert_eq!(dp.app_layer_params.ts003_f_port as i16, qi.f_port);

                let qi_pl = clocksync::v2::Payload::from_slice(false, &qi.data).unwrap();
                let expected_pl = clocksync::v2::Payload::AppTimeAns(expected_pl.clone());

                assert_eq!(expected_pl, qi_pl);
            } else {
                assert!(queue_items.is_empty());
            }
        }
    }
}
