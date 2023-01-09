use super::assert;
use crate::storage::{
    application, device, device_gateway, device_profile, fields, gateway, multicast, tenant,
};
use crate::{downlink, gateway::backend as gateway_backend, integration, test};
use chirpstack_api::{gw, internal};
use lrwn::{AES128Key, DevAddr, EUI64};

struct MulticastTest {
    name: String,
    multicast_group: multicast::MulticastGroup,
    multicast_group_queue_items: Vec<multicast::MulticastGroupQueueItem>,
    assert: Vec<assert::Validator>,
}

#[tokio::test]
async fn test_multicast() {
    let _guard = test::prepare().await;

    // tenant
    let t = tenant::create(tenant::Tenant {
        name: "test-tenant".into(),
        can_have_gateways: true,
        ..Default::default()
    })
    .await
    .unwrap();

    // gateway
    let gw = gateway::create(gateway::Gateway {
        tenant_id: t.id,
        gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        name: "test-gw".into(),
        properties: fields::KeyValue::new(
            [("region_config_id".to_string(), "eu868".to_string())]
                .iter()
                .cloned()
                .collect(),
        ),
        ..Default::default()
    })
    .await
    .unwrap();

    // application
    let app = application::create(application::Application {
        name: "test-app".into(),
        tenant_id: t.id,
        ..Default::default()
    })
    .await
    .unwrap();

    // device-profile
    let dp = device_profile::create(device_profile::DeviceProfile {
        name: "test-dp".into(),
        tenant_id: t.id.clone(),
        ..Default::default()
    })
    .await
    .unwrap();

    // device
    let d = device::create(device::Device {
        name: "test-dev".into(),
        application_id: app.id,
        device_profile_id: dp.id,
        dev_eui: EUI64::from_be_bytes([8, 7, 6, 5, 4, 3, 2, 1]),
        ..Default::default()
    })
    .await
    .unwrap();

    // multicast group
    let mg = multicast::create(multicast::MulticastGroup {
        application_id: app.id,
        name: "test-mg".into(),
        mc_addr: DevAddr::from_be_bytes([1, 2, 3, 4]),
        mc_nwk_s_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
        mc_app_s_key: AES128Key::from_bytes([2, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
        f_cnt: 10,
        group_type: "C".into(),
        dr: 3,
        frequency: 868300000,
        class_b_ping_slot_period: 32,
        ..Default::default()
    })
    .await
    .unwrap();
    multicast::add_device(&mg.id, &d.dev_eui).await.unwrap();

    // device <> gateway
    device_gateway::save_rx_info(&internal::DeviceGatewayRxInfo {
        dev_eui: d.dev_eui.to_vec(),
        items: vec![internal::DeviceGatewayRxInfoItem {
            gateway_id: gw.gateway_id.to_vec(),
            ..Default::default()
        }],
        ..Default::default()
    })
    .await
    .unwrap();

    let tests = vec![
        MulticastTest {
            name: "nothing in queue".into(),
            multicast_group: mg.clone(),
            multicast_group_queue_items: vec![],
            assert: vec![assert::no_downlink_frame()],
        },
        MulticastTest {
            name: "one item in queue".into(),
            multicast_group: mg.clone(),
            multicast_group_queue_items: vec![multicast::MulticastGroupQueueItem {
                multicast_group_id: mg.id,
                f_port: 5,
                data: vec![1, 2, 3],
                ..Default::default()
            }],
            assert: vec![assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: gw.gateway_id.to_string(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![
                        96, 4, 3, 2, 1, 0, 10, 0, 5, 161, 250, 255, 42, 110, 141, 200,
                    ],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 868300000,
                        power: 14,
                        modulation: Some(gw::Modulation {
                            parameters: Some(gw::modulation::Parameters::Lora(
                                gw::LoraModulationInfo {
                                    bandwidth: 125000,
                                    spreading_factor: 9,
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
            })],
        },
        MulticastTest {
            name: "two items in queue".into(),
            multicast_group: mg.clone(),
            multicast_group_queue_items: vec![
                multicast::MulticastGroupQueueItem {
                    multicast_group_id: mg.id,
                    f_port: 5,
                    data: vec![1, 2, 3],
                    ..Default::default()
                },
                multicast::MulticastGroupQueueItem {
                    multicast_group_id: mg.id,
                    f_port: 6,
                    data: vec![1, 2, 3],
                    ..Default::default()
                },
            ],
            assert: vec![assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: gw.gateway_id.to_string(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![
                        96, 4, 3, 2, 1, 0, 10, 0, 5, 161, 250, 255, 42, 110, 141, 200,
                    ],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 868300000,
                        power: 14,
                        modulation: Some(gw::Modulation {
                            parameters: Some(gw::modulation::Parameters::Lora(
                                gw::LoraModulationInfo {
                                    bandwidth: 125000,
                                    spreading_factor: 9,
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
            })],
        },
        MulticastTest {
            name: "item discarded because of payload size".into(),
            multicast_group: mg.clone(),
            multicast_group_queue_items: vec![
                multicast::MulticastGroupQueueItem {
                    multicast_group_id: mg.id,
                    f_port: 5,
                    data: vec![2; 300],
                    ..Default::default()
                },
                multicast::MulticastGroupQueueItem {
                    multicast_group_id: mg.id,
                    f_port: 6,
                    data: vec![1, 2, 3],
                    ..Default::default()
                },
            ],
            assert: vec![assert::no_downlink_frame()],
        },
    ];

    for tst in &tests {
        run_scheduler_test(tst).await;
    }
}

async fn run_scheduler_test(t: &MulticastTest) {
    println!("> {}", t.name);

    integration::set_mock().await;
    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    // overwrite multicast-group to deal with frame-counter increments
    multicast::update(t.multicast_group.clone()).await.unwrap();

    // set multicast-group queue
    multicast::flush_queue(&t.multicast_group.id).await.unwrap();
    for qi in &t.multicast_group_queue_items {
        let _ = downlink::multicast::enqueue(qi.clone()).await.unwrap();
    }

    downlink::scheduler::schedule_multicast_group_queue_batch(1)
        .await
        .unwrap();

    for assert in &t.assert {
        assert().await;
    }
}
