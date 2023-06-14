use uuid::Uuid;

use super::assert;
use crate::gpstime::ToGpsTime;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_gateway, device_profile, device_queue, device_session, gateway, reset_redis, tenant,
};
use crate::{
    config, downlink, downlink::classb, gateway::backend as gateway_backend, integration, test,
    uplink,
};
use chirpstack_api::{common, gw, internal};
use lrwn::{DevAddr, EUI64};

struct UplinkTest {
    name: String,
    device_queue_items: Vec<device_queue::DeviceQueueItem>,
    device_session: Option<internal::DeviceSession>,
    tx_info: gw::UplinkTxInfo,
    rx_info: gw::UplinkRxInfo,
    phy_payload: lrwn::PhyPayload,
    assert: Vec<assert::Validator>,
}

struct DownlinkTest {
    name: String,
    device_queue_items: Vec<device_queue::DeviceQueueItem>,
    device_session: Option<internal::DeviceSession>,
    device_gateway_rx_info: Option<internal::DeviceGatewayRxInfo>,
    assert: Vec<assert::Validator>,
}

#[tokio::test]
async fn test_uplink() {
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
        supports_class_b: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let dev = device::create(device::Device {
        name: "device".into(),
        application_id: app.id.clone(),
        device_profile_id: dp.id.clone(),
        dev_eui: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
        enabled_class: DeviceClass::A,
        ..Default::default()
    })
    .await
    .unwrap();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: gw.gateway_id.to_string(),
        ..Default::default()
    };
    rx_info
        .metadata
        .insert("region_config_id".to_string(), "eu868".to_string());
    rx_info
        .metadata
        .insert("region_common_name".to_string(), "EU868".to_string());

    let mut tx_info = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info, 0).unwrap();

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

    let mut conf = (*config::get()).clone();
    conf.regions[0].network.class_b.ping_slot_dr = 2;
    conf.regions[0].network.class_b.ping_slot_frequency = 868300000;
    config::set(conf);

    // trigger beacon locked
    run_uplink_test(&UplinkTest {
        name: "trigger beacon locked".into(),
        device_queue_items: vec![],
        device_session: Some(ds.clone()),
        tx_info: tx_info.clone(),
        rx_info: rx_info.clone(),
        phy_payload: lrwn::PhyPayload {
            mhdr: lrwn::MHDR {
                m_type: lrwn::MType::UnconfirmedDataUp,
                major: lrwn::Major::LoRaWANR1,
            },
            payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                fhdr: lrwn::FHDR {
                    devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                    f_cnt: 8,
                    f_ctrl: lrwn::FCtrl {
                        class_b: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                f_port: None,
                frm_payload: None,
            }),
            mic: Some([241, 100, 207, 79]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 9),
            assert::enabled_class(dev.dev_eui.clone(), DeviceClass::B),
        ],
    })
    .await;

    // trigger beacon unlocked
    run_uplink_test(&UplinkTest {
        name: "trigger beacon locked".into(),
        device_queue_items: vec![],
        device_session: Some(ds.clone()),
        tx_info: tx_info.clone(),
        rx_info: rx_info.clone(),
        phy_payload: lrwn::PhyPayload {
            mhdr: lrwn::MHDR {
                m_type: lrwn::MType::UnconfirmedDataUp,
                major: lrwn::Major::LoRaWANR1,
            },
            payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                fhdr: lrwn::FHDR {
                    devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                    f_cnt: 8,
                    f_ctrl: lrwn::FCtrl {
                        class_b: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                f_port: None,
                frm_payload: None,
            }),
            mic: Some([137, 180, 12, 148]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 9),
            assert::enabled_class(dev.dev_eui.clone(), DeviceClass::A),
        ],
    })
    .await;
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
        supports_class_b: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let dev = device::create(device::Device {
        name: "device".into(),
        application_id: app.id.clone(),
        device_profile_id: dp.id.clone(),
        dev_eui: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
        enabled_class: DeviceClass::B,
        ..Default::default()
    })
    .await
    .unwrap();

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
        class_b_ping_slot_freq: 868300000,
        class_b_ping_slot_dr: 2,
        class_b_ping_slot_nb: 1,
        ..Default::default()
    };

    let device_gateway_rx_info = internal::DeviceGatewayRxInfo {
        dev_eui: dev.dev_eui.to_vec(),
        items: vec![internal::DeviceGatewayRxInfoItem {
            gateway_id: gw.gateway_id.to_vec(),
            ..Default::default()
        }],
        ..Default::default()
    };

    let now_gps_ts = chrono::Utc::now().to_gps_time() + chrono::Duration::seconds(1);
    let ping_slot_ts = classb::get_next_ping_slot_after(
        now_gps_ts,
        &DevAddr::from_slice(&ds.dev_addr).unwrap(),
        ds.class_b_ping_slot_nb as usize,
    )
    .unwrap();

    run_scheduler_test(&DownlinkTest {
        name: "class-b downlink".into(),
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
            assert::f_cnt_up(dev.dev_eui.clone(), 8),
            assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![96, 4, 3, 2, 1, 128, 5, 0, 10, 115, 46, 73, 41, 113, 46, 49],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 868300000,
                        power: 14,
                        modulation: Some(gw::Modulation {
                            parameters: Some(gw::modulation::Parameters::Lora(
                                gw::LoraModulationInfo {
                                    bandwidth: 125000,
                                    spreading_factor: 10,
                                    code_rate: gw::CodeRate::Cr45.into(),
                                    polarization_inversion: true,
                                    ..Default::default()
                                },
                            )),
                        }),
                        timing: Some(gw::Timing {
                            parameters: Some(gw::timing::Parameters::GpsEpoch(
                                gw::GpsEpochTimingInfo {
                                    time_since_gps_epoch: Some(pbjson_types::Duration::from(
                                        ping_slot_ts.to_std().unwrap(),
                                    )),
                                },
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
        name: "class-b downlink with more data".into(),
        device_queue_items: vec![
            device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![1, 2, 3],
                ..Default::default()
            },
            device_queue::DeviceQueueItem {
                id: Uuid::new_v4(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![1, 2, 3, 4],
                ..Default::default()
            },
        ],
        device_session: Some(ds.clone()),
        device_gateway_rx_info: Some(device_gateway_rx_info.clone()),
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 8),
            assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![
                        96, 4, 3, 2, 1, 144, 5, 0, 10, 115, 46, 73, 218, 230, 215, 91,
                    ],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 868300000,
                        power: 14,
                        modulation: Some(gw::Modulation {
                            parameters: Some(gw::modulation::Parameters::Lora(
                                gw::LoraModulationInfo {
                                    bandwidth: 125000,
                                    spreading_factor: 10,
                                    code_rate: gw::CodeRate::Cr45.into(),
                                    polarization_inversion: true,
                                    ..Default::default()
                                },
                            )),
                        }),
                        timing: Some(gw::Timing {
                            parameters: Some(gw::timing::Parameters::GpsEpoch(
                                gw::GpsEpochTimingInfo {
                                    time_since_gps_epoch: Some(pbjson_types::Duration::from(
                                        ping_slot_ts.to_std().unwrap(),
                                    )),
                                },
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
}

async fn run_uplink_test(t: &UplinkTest) {
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

    for qi in &t.device_queue_items {
        let _ = device_queue::enqueue_item(qi.clone()).await.unwrap();
    }

    uplink::handle_uplink(
        Uuid::new_v4(),
        gw::UplinkFrameSet {
            phy_payload: t.phy_payload.to_vec().unwrap(),
            tx_info: Some(t.tx_info.clone()),
            rx_info: vec![t.rx_info.clone()],
        },
    )
    .await
    .unwrap();

    for assert in &t.assert {
        assert().await;
    }
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
