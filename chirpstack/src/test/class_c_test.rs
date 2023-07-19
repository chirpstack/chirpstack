use uuid::Uuid;

use super::assert;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_gateway, device_profile, device_queue, device_session, gateway, reset_redis, tenant,
};
use crate::{downlink, gateway::backend as gateway_backend, integration, test};
use chirpstack_api::{common, gw, internal};
use lrwn::EUI64;

struct DownlinkTest {
    name: String,
    device_queue_items: Vec<device_queue::DeviceQueueItem>,
    device_session: Option<internal::DeviceSession>,
    device_gateway_rx_info: Option<internal::DeviceGatewayRxInfo>,
    assert: Vec<assert::Validator>,
}

#[tokio::test]
async fn test_uplink() {
    // TODO: implement changing from Class A -> C?
}

#[tokio::test]
async fn test_downlink_scheduler() {
    let _guard = test::prepare().await;

    let t = tenant::create(tenant::Tenant {
        name: "tenant".into(),
        can_have_gateways: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let gw = gateway::create(gateway::Gateway {
        name: "gateway".into(),
        tenant_id: t.id.clone(),
        gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let app = application::create(application::Application {
        name: "app".into(),
        tenant_id: t.id.clone(),
        ..Default::default()
    })
    .await
    .unwrap();

    let dp = device_profile::create(device_profile::DeviceProfile {
        name: "dp".into(),
        tenant_id: t.id.clone(),
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_4,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
        supports_class_c: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let dev = device::create(device::Device {
        name: "device".into(),
        application_id: app.id.clone(),
        device_profile_id: dp.id.clone(),
        dev_eui: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
        enabled_class: DeviceClass::C,
        ..Default::default()
    })
    .await
    .unwrap();

    let device_gateway_rx_info = internal::DeviceGatewayRxInfo {
        dev_eui: dev.dev_eui.to_vec(),
        items: vec![internal::DeviceGatewayRxInfoItem {
            gateway_id: gw.gateway_id.to_vec(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan104.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        app_s_key: Some(common::KeyEnvelope {
            kek_label: "".into(),
            aes_key: vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        }),
        f_cnt_up: 8,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let ds_no_uplink = internal::DeviceSession {
        f_cnt_up: 0,
        ..ds.clone()
    };

    run_scheduler_test(&DownlinkTest {
        name: "device has not yet sent an uplink".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 10,
            data: vec![1, 2, 3],
            ..Default::default()
        }],
        device_session: Some(ds_no_uplink.clone()),
        device_gateway_rx_info: Some(device_gateway_rx_info.clone()),
        assert: vec![assert::no_downlink_frame()],
    })
    .await;

    // remove the schedule run after
    device::set_scheduler_run_after(&dev.dev_eui.clone(), None)
        .await
        .unwrap();

    run_scheduler_test(&DownlinkTest {
        name: "unconfirmed data".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 10,
            data: vec![1, 2, 3],
            ..Default::default()
        }],
        device_session: Some(ds.clone()),
        device_gateway_rx_info: Some(device_gateway_rx_info.clone()),
        assert: vec![
            assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![96, 4, 3, 2, 1, 128, 5, 0, 10, 115, 46, 73, 41, 113, 46, 49],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 869525000,
                        power: 27,
                        modulation: Some(gw::Modulation {
                            parameters: Some(gw::modulation::Parameters::Lora(
                                gw::LoraModulationInfo {
                                    bandwidth: 125000,
                                    spreading_factor: 12,
                                    code_rate: gw::CodeRate::Cr45.into(),
                                    polarization_inversion: true,
                                    ..Default::default()
                                },
                            )),
                        }),
                        timing: Some(gw::Timing {
                            parameters: Some(gw::timing::Parameters::Immediately(
                                gw::ImmediatelyTimingInfo {},
                            )),
                        }),
                        ..Default::default()
                    }),
                }],
                ..Default::default()
            }),
        ],
    })
    .await;

    run_scheduler_test(&DownlinkTest {
        name: "scheduler_run_after has not yet expired".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 10,
            data: vec![1, 2, 3],
            ..Default::default()
        }],
        device_session: Some(ds.clone()),
        device_gateway_rx_info: Some(device_gateway_rx_info.clone()),
        assert: vec![assert::no_downlink_frame()],
    })
    .await;

    // remove the schedule run after
    device::set_scheduler_run_after(&dev.dev_eui.clone(), None)
        .await
        .unwrap();

    run_scheduler_test(&DownlinkTest {
        name: "unconfirmed data".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 10,
            data: vec![1, 2, 3],
            confirmed: true,
            ..Default::default()
        }],
        device_session: Some(ds.clone()),
        device_gateway_rx_info: Some(device_gateway_rx_info.clone()),
        assert: vec![
            assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![
                        160, 4, 3, 2, 1, 128, 5, 0, 10, 115, 46, 73, 138, 39, 53, 228,
                    ],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 869525000,
                        power: 27,
                        modulation: Some(gw::Modulation {
                            parameters: Some(gw::modulation::Parameters::Lora(
                                gw::LoraModulationInfo {
                                    bandwidth: 125000,
                                    spreading_factor: 12,
                                    code_rate: gw::CodeRate::Cr45.into(),
                                    polarization_inversion: true,
                                    ..Default::default()
                                },
                            )),
                        }),
                        timing: Some(gw::Timing {
                            parameters: Some(gw::timing::Parameters::Immediately(
                                gw::ImmediatelyTimingInfo {},
                            )),
                        }),
                        ..Default::default()
                    }),
                }],
                ..Default::default()
            }),
        ],
    })
    .await;

    // remove the schedule run after
    device::set_scheduler_run_after(&dev.dev_eui.clone(), None)
        .await
        .unwrap();

    run_scheduler_test(&DownlinkTest {
        name: "unconfirmed data".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 10,
            data: vec![0; 300],
            ..Default::default()
        }],
        device_session: Some(ds.clone()),
        device_gateway_rx_info: Some(device_gateway_rx_info.clone()),
        assert: vec![assert::no_downlink_frame()],
    })
    .await;
}

async fn run_scheduler_test(t: &DownlinkTest) {
    println!("> {}", t.name);

    reset_redis().await.unwrap();

    integration::set_mock().await;
    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    integration::mock::reset().await;
    gateway_backend::mock::reset().await;

    if let Some(ds) = &t.device_session {
        let _ = device_session::save(&ds).await.unwrap();

        let dev_eui = EUI64::from_slice(&ds.dev_eui).unwrap();
        device_queue::flush_for_dev_eui(&dev_eui).await.unwrap();
    }

    if let Some(rx_info) = &t.device_gateway_rx_info {
        let _ = device_gateway::save_rx_info(rx_info).await.unwrap();
    }

    for qi in &t.device_queue_items {
        let _ = device_queue::enqueue_item(qi.clone()).await.unwrap();
    }

    downlink::scheduler::schedule_device_queue_batch(1)
        .await
        .unwrap();

    for assert in &t.assert {
        assert().await;
    }
}
