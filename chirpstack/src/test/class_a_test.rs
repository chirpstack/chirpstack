use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use uuid::Uuid;

use super::assert;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_profile, device_queue, device_session, gateway, mac_command, reset_redis, tenant,
};
use crate::{config, gateway::backend as gateway_backend, integration, region, test, uplink};
use chirpstack_api::{api, common, gw, integration as integration_pb, internal};
use lrwn::{AES128Key, EUI64};

type Function = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>;

struct Test {
    name: String,
    device_queue_items: Vec<device_queue::DeviceQueueItem>,
    before_func: Option<Function>,
    after_func: Option<Function>,
    device_session: Option<internal::DeviceSession>,
    tx_info: gw::UplinkTxInfo,
    rx_info: gw::UplinkRxInfo,
    phy_payload: lrwn::PhyPayload,
    assert: Vec<assert::Validator>,
}

#[tokio::test]
async fn test_gateway_filtering() {
    let _guard = test::prepare().await;

    let t_a = tenant::create(tenant::Tenant {
        name: "tenant-a".into(),
        private_gateways_up: true,
        can_have_gateways: true,
        ..Default::default()
    })
    .await
    .unwrap();
    let t_b = tenant::create(tenant::Tenant {
        name: "tenant-b".into(),
        private_gateways_up: true,
        can_have_gateways: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let gw_a = gateway::create(gateway::Gateway {
        name: "gateway-a".into(),
        tenant_id: t_a.id.clone(),
        gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let gw_b = gateway::create(gateway::Gateway {
        name: "gateway-b".into(),
        tenant_id: t_b.id.clone(),
        gateway_id: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let app = application::create(application::Application {
        name: "app".into(),
        tenant_id: t_a.id.clone(),
        ..Default::default()
    })
    .await
    .unwrap();

    let dp = device_profile::create(device_profile::DeviceProfile {
        name: "dp".into(),
        tenant_id: t_a.id.clone(),
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_2,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
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

    let mut rx_info_a = gw::UplinkRxInfo {
        gateway_id: gw_a.gateway_id.to_string(),
        location: Some(Default::default()),
        ..Default::default()
    };
    rx_info_a
        .metadata
        .insert("region_config_id".to_string(), "eu868".to_string());
    rx_info_a
        .metadata
        .insert("region_common_name".to_string(), "EU868".to_string());

    let mut rx_info_b = gw::UplinkRxInfo {
        gateway_id: gw_b.gateway_id.to_string(),
        location: Some(Default::default()),
        ..Default::default()
    };
    rx_info_b
        .metadata
        .insert("region_config_id".to_string(), "eu868".to_string());
    rx_info_b
        .metadata
        .insert("region_common_name".to_string(), "EU868".to_string());

    let mut tx_info = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info, 0).unwrap();

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan102.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 7,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "private gateway of same tenant".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info_a.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 7,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([48, 94, 26, 239]),
            },
            assert: vec![assert::f_cnt_up(dev.dev_eui.clone(), 8)],
        },
        Test {
            name: "private gateway other tenant".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info_b.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 7,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([48, 94, 26, 239]),
            },
            assert: vec![assert::f_cnt_up(dev.dev_eui.clone(), 7)],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_region_config_id_filtering() {
    let _guard = test::prepare().await;

    // We need to configure the eu868_other region.
    let region_conf = lrwn::region::get(lrwn::region::CommonName::EU868, false, false);
    region::set("eu868_other", region_conf);

    let t = tenant::create(tenant::Tenant {
        name: "tenant".into(),
        can_have_gateways: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let gw = gateway::create(gateway::Gateway {
        name: "test-gw".into(),
        tenant_id: t.id,
        gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let app = application::create(application::Application {
        name: "app".into(),
        tenant_id: t.id,
        ..Default::default()
    })
    .await
    .unwrap();

    let dp = device_profile::create(device_profile::DeviceProfile {
        name: "test-dp".into(),
        tenant_id: t.id,
        region: lrwn::region::CommonName::EU868,
        region_config_id: Some("eu868".to_string()),
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_2,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
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

    let mut rx_info_ok = gw::UplinkRxInfo {
        gateway_id: gw.gateway_id.to_string(),
        location: Some(Default::default()),
        ..Default::default()
    };
    rx_info_ok
        .metadata
        .insert("region_config_id".to_string(), "eu868".to_string());
    rx_info_ok
        .metadata
        .insert("region_common_name".to_string(), "EU868".to_string());

    let mut rx_info_invalid = gw::UplinkRxInfo {
        gateway_id: gw.gateway_id.to_string(),
        location: Some(Default::default()),
        ..Default::default()
    };
    rx_info_invalid
        .metadata
        .insert("region_config_id".to_string(), "eu868_other".to_string());
    rx_info_invalid
        .metadata
        .insert("region_common_name".to_string(), "EU868".to_string());

    let mut tx_info = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info, 0).unwrap();

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan102.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 7,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "matching config id".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info_ok.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 7,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([48, 94, 26, 239]),
            },
            assert: vec![assert::f_cnt_up(dev.dev_eui.clone(), 8)],
        },
        Test {
            name: "non-matching configuration id".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info_invalid.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 7,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([48, 94, 26, 239]),
            },
            assert: vec![assert::f_cnt_up(dev.dev_eui.clone(), 7)],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_errors() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_2,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let _dev = device::create(device::Device {
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
        location: Some(Default::default()),
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
        mac_version: common::MacVersion::Lorawan102.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 8,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "invalid frame-counter (did not increment)".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 7,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([48, 94, 26, 239]),
            },
            assert: vec![
                assert::integration_log(vec![
                    "Uplink was flagged as re-transmission / frame-counter did not increment"
                        .to_string(),
                ]),
                assert::no_uplink_event(),
            ],
        },
        Test {
            name: "invalid frame-counter (reset)".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 0,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([0x83, 0x24, 0x53, 0xa3]),
            },
            assert: vec![
                assert::integration_log(vec![
                    "Frame-counter reset or rollover detected".to_string()
                ]),
                assert::no_uplink_event(),
            ],
        },
        Test {
            name: "invalid mic".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([1, 2, 3, 4]),
            },
            assert: vec![
                assert::no_uplink_event(),
                assert::device_uplink_frame_log(api::UplinkFrameLog {
                    phy_payload: vec![64, 4, 3, 2, 1, 0, 8, 0, 1, 1, 2, 3, 4],
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    dev_eui: "0000000000000000".into(),
                    dev_addr: "01020304".into(),
                    m_type: common::MType::UnconfirmedDataUp.into(),
                    ..Default::default()
                }),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_11_errors() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_1_0,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let _dev = device::create(device::Device {
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
        location: Some(Default::default()),
        ..Default::default()
    };
    rx_info
        .metadata
        .insert("region_config_id".to_string(), "eu868".to_string());
    rx_info
        .metadata
        .insert("region_common_name".to_string(), "EU868".to_string());

    let mut tx_info_freq = gw::UplinkTxInfo {
        frequency: 868300000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_freq, 0).unwrap();

    let mut tx_info_dr = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_dr, 3).unwrap();

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan102.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 8,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "invalid frequency (MIC)".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info_freq.clone(),
            rx_info: rx_info.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 160, 195]),
            },
            assert: vec![assert::no_uplink_event()],
        },
        Test {
            name: "invalid frequency (MIC)".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info_dr.clone(),
            rx_info: rx_info.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 160, 195]),
            },
            assert: vec![assert::no_uplink_event()],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_skip_f_cnt() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_2,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
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
        skip_fcnt_check: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: gw.gateway_id.to_string(),
        location: Some(Default::default()),
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
        mac_version: common::MacVersion::Lorawan102.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 8,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        skip_f_cnt_check: true,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "frame-counter is invalid but not 0".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 7,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([48, 94, 26, 239]),
            },
            assert: vec![
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 7,
                    f_port: 1,
                    ..Default::default()
                }),
                assert::f_cnt_up(dev.dev_eui.clone(), 8),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            ],
        },
        Test {
            name: "frame-counter is invalid and 0".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 0,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([131, 36, 83, 163]),
            },
            assert: vec![
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 0,
                    f_port: 1,
                    ..Default::default()
                }),
                assert::f_cnt_up(dev.dev_eui.clone(), 1),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_device_disabled() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_2,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
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
        is_disabled: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: gw.gateway_id.to_string(),
        location: Some(Default::default()),
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
        mac_version: common::MacVersion::Lorawan102.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 7,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![Test {
        name: "uplink ignored".into(),
        device_queue_items: vec![],
        before_func: None,
        after_func: None,
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
                    f_cnt: 7,
                    ..Default::default()
                },
                f_port: Some(1),
                frm_payload: None,
            }),
            mic: Some([48, 94, 26, 239]),
        },
        assert: vec![
            assert::no_uplink_event(),
            assert::f_cnt_up(dev.dev_eui.clone(), 7),
            assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
        ],
    }];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_uplink() {
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
        location: Some(Default::default()),
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

    let mut tx_info_lr_fhss = gw::UplinkTxInfo {
        frequency: 867300000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_lr_fhss, 10).unwrap();

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
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "unconfirmed uplink with payload".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 35, 121]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    data: vec![215, 241, 112, 52],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "unconfirmed uplink with payload using LR-FHSS dr".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                Box::pin(async move {
                    let mut conf = (*config::get()).clone();
                    conf.regions[0]
                        .network
                        .extra_channels
                        .push(config::ExtraChannel {
                            frequency: 867300000,
                            min_dr: 10,
                            max_dr: 11,
                        });
                    config::set(conf);
                    region::setup().unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                Box::pin(async move {
                    let mut conf = (*config::get()).clone();
                    conf.regions[0].network.extra_channels = vec![];
                    config::set(conf);
                    region::setup().unwrap();
                })
            })),
            device_session: Some(ds.clone()),
            tx_info: tx_info_lr_fhss.clone(),
            rx_info: rx_info.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 35, 121]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info_lr_fhss.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 10,
                    data: vec![215, 241, 112, 52],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "unconfirmed uplink with payload + ACK".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 1,
                f_cnt_down: Some(4),
                is_pending: true,
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            ack: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([132, 250, 228, 10]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::ack_event(integration_pb::AckEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    queue_item_id: Uuid::nil().to_string(),
                    acknowledged: true,
                    f_cnt_down: 4,
                    ..Default::default()
                }),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    data: vec![215, 241, 112, 52],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "unconfirmed uplink without payload (just FPort)".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 68, 8]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    data: vec![],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "confirmed uplink with payload".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::ConfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([69, 90, 200, 95]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    confirmed: true,
                    data: vec![215, 241, 112, 52],
                    ..Default::default()
                }),
                assert::downlink_frame(gw::DownlinkFrame {
                    gateway_id: "0102030405060708".into(),
                    items: vec![
                        gw::DownlinkFrameItem {
                            phy_payload: vec![96, 4, 3, 2, 1, 160, 5, 0, 161, 179, 218, 104],
                            tx_info_legacy: None,
                            tx_info: Some(gw::DownlinkTxInfo {
                                frequency: 868100000,
                                power: 14,
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
                                    parameters: Some(gw::timing::Parameters::Delay(
                                        gw::DelayTimingInfo {
                                            delay: Some(Duration::from_secs(1).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                        gw::DownlinkFrameItem {
                            phy_payload: vec![96, 4, 3, 2, 1, 160, 5, 0, 161, 179, 218, 104],
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
                                    parameters: Some(gw::timing::Parameters::Delay(
                                        gw::DelayTimingInfo {
                                            delay: Some(Duration::from_secs(2).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                    ],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "confirmed uplink without payload".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info.clone(),
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    m_type: lrwn::MType::ConfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: lrwn::FHDR {
                        devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([210, 52, 52, 94]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    confirmed: true,
                    ..Default::default()
                }),
                assert::downlink_frame(gw::DownlinkFrame {
                    gateway_id: "0102030405060708".into(),
                    items: vec![
                        gw::DownlinkFrameItem {
                            phy_payload: vec![96, 4, 3, 2, 1, 160, 5, 0, 161, 179, 218, 104],
                            tx_info_legacy: None,
                            tx_info: Some(gw::DownlinkTxInfo {
                                frequency: 868100000,
                                power: 14,
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
                                    parameters: Some(gw::timing::Parameters::Delay(
                                        gw::DelayTimingInfo {
                                            delay: Some(Duration::from_secs(1).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                        gw::DownlinkFrameItem {
                            phy_payload: vec![96, 4, 3, 2, 1, 160, 5, 0, 161, 179, 218, 104],
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
                                    parameters: Some(gw::timing::Parameters::Delay(
                                        gw::DelayTimingInfo {
                                            delay: Some(Duration::from_secs(2).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                    ],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "uplink of class-c device sets lock".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_c = true;
                    device_profile::update(dp.clone()).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_c = false;
                    device_profile::update(dp).await.unwrap();
                })
            })),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 35, 121]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_device_lock(dev.dev_eui.clone()),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_end_to_end_enc() {
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
        location: Some(Default::default()),
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

    let ds_sess_key_id = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan104.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        app_s_key: None,
        js_session_key_id: vec![1, 2, 3],
        f_cnt_up: 8,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let ds_app_s_key = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan104.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: vec![1, 2, 3, 4],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        app_s_key: Some(common::KeyEnvelope {
            kek_label: "kek-label".into(),
            aes_key: vec![1, 2, 3],
        }),
        f_cnt_up: 8,
        n_f_cnt_down: 5,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "end-to-end encryption with session key id".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_sess_key_id.clone()),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 35, 121]),
            },
            assert: vec![assert::uplink_event(integration_pb::UplinkEvent {
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_name: t.name.clone(),
                    tenant_id: t.id.to_string(),
                    application_name: app.name.clone(),
                    application_id: app.id.to_string(),
                    device_profile_name: dp.name.clone(),
                    device_profile_id: dp.id.to_string(),
                    device_name: dev.name.clone(),
                    dev_eui: dev.dev_eui.to_string(),
                    ..Default::default()
                }),
                dev_addr: "01020304".into(),
                tx_info: Some(tx_info.clone()),
                rx_info: vec![rx_info.clone()],
                f_cnt: 10,
                f_port: 1,
                dr: 0,
                data: vec![1, 2, 3, 4],
                join_server_context: Some(integration_pb::JoinServerContext {
                    session_key_id: "010203".into(),
                    ..Default::default()
                }),
                ..Default::default()
            })],
        },
        Test {
            name: "end-to-end encryption with AppSKey".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_app_s_key.clone()),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 35, 121]),
            },
            assert: vec![assert::uplink_event(integration_pb::UplinkEvent {
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_name: t.name.clone(),
                    tenant_id: t.id.to_string(),
                    application_name: app.name.clone(),
                    application_id: app.id.to_string(),
                    device_profile_name: dp.name.clone(),
                    device_profile_id: dp.id.to_string(),
                    device_name: dev.name.clone(),
                    dev_eui: dev.dev_eui.to_string(),
                    ..Default::default()
                }),
                dev_addr: "01020304".into(),
                tx_info: Some(tx_info.clone()),
                rx_info: vec![rx_info.clone()],
                f_cnt: 10,
                f_port: 1,
                dr: 0,
                data: vec![1, 2, 3, 4],
                join_server_context: Some(integration_pb::JoinServerContext {
                    app_s_key: Some(common::KeyEnvelope {
                        kek_label: "kek-label".into(),
                        aes_key: vec![1, 2, 3],
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            })],
        },
        Test {
            name: "end-to-end encryption using AppSkey + encrypted downlink".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 1,
                data: vec![1, 2, 3, 4],
                f_cnt_down: Some(10),
                is_encrypted: true,
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
            device_session: Some(ds_app_s_key.clone()),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 35, 121]),
            },
            assert: vec![
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    data: vec![1, 2, 3, 4],
                    join_server_context: Some(integration_pb::JoinServerContext {
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "kek-label".into(),
                            aes_key: vec![1, 2, 3],
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 10,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(1),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                        }),
                        mic: Some([8, 125, 131, 36]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 10,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(1),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                        }),
                        mic: Some([8, 125, 131, 36]),
                    },
                ]),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_11_uplink() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_1_0,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
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
        location: Some(Default::default()),
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

    let mut tx_info_lr_fhss = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_lr_fhss, 8).unwrap();

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan110.into(),
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
        conf_f_cnt: 4,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "unconfirmed uplink with payload".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([104, 147, 104, 147]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    data: vec![215, 241, 112, 52],
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "unconfirmed uplink with payload + ACK".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 1,
                f_cnt_down: Some(4),
                is_pending: true,
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            ack: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
                }),
                mic: Some([76, 46, 132, 250]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::ack_event(integration_pb::AckEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    queue_item_id: Uuid::nil().to_string(),
                    acknowledged: true,
                    f_cnt_down: 4,
                    ..Default::default()
                }),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    f_cnt: 10,
                    f_port: 1,
                    dr: 0,
                    data: vec![215, 241, 112, 52],
                    ..Default::default()
                }),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_rx_delay() {
    let _guard = test::prepare().await;

    let mut conf = (*config::get()).clone();
    conf.regions[0].network.rx1_delay = 3;
    config::set(conf);

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
        location: Some(Default::default()),
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

    let mut tx_info_lr_fhss = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_lr_fhss, 8).unwrap();

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
        rx1_delay: 3,
        ..Default::default()
    };

    let tests = vec![Test {
        name: "confirmed uplink without payload (rx_delay = 3)".into(),
        device_queue_items: vec![],
        before_func: None,
        after_func: None,
        device_session: Some(ds.clone()),
        tx_info: tx_info.clone(),
        rx_info: rx_info.clone(),
        phy_payload: lrwn::PhyPayload {
            mhdr: lrwn::MHDR {
                m_type: lrwn::MType::ConfirmedDataUp,
                major: lrwn::Major::LoRaWANR1,
            },
            payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                fhdr: lrwn::FHDR {
                    devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                    f_cnt: 10,
                    ..Default::default()
                },
                f_port: Some(1),
                frm_payload: None,
            }),
            mic: Some([210, 52, 52, 94]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 11),
            assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
            assert::uplink_event(integration_pb::UplinkEvent {
                device_info: Some(integration_pb::DeviceInfo {
                    tenant_name: t.name.clone(),
                    tenant_id: t.id.to_string(),
                    application_name: app.name.clone(),
                    application_id: app.id.to_string(),
                    device_profile_name: dp.name.clone(),
                    device_profile_id: dp.id.to_string(),
                    device_name: dev.name.clone(),
                    dev_eui: dev.dev_eui.to_string(),
                    ..Default::default()
                }),
                dev_addr: "01020304".into(),
                tx_info: Some(tx_info.clone()),
                rx_info: vec![rx_info.clone()],
                f_cnt: 10,
                f_port: 1,
                confirmed: true,
                dr: 0,
                ..Default::default()
            }),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![
                    gw::DownlinkFrameItem {
                        phy_payload: vec![96, 4, 3, 2, 1, 160, 5, 0, 161, 179, 218, 104],
                        tx_info_legacy: None,
                        tx_info: Some(gw::DownlinkTxInfo {
                            frequency: 868100000,
                            power: 14,
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
                                parameters: Some(gw::timing::Parameters::Delay(
                                    gw::DelayTimingInfo {
                                        delay: Some(Duration::from_secs(3).into()),
                                    },
                                )),
                            }),
                            ..Default::default()
                        }),
                    },
                    gw::DownlinkFrameItem {
                        phy_payload: vec![96, 4, 3, 2, 1, 160, 5, 0, 161, 179, 218, 104],
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
                                parameters: Some(gw::timing::Parameters::Delay(
                                    gw::DelayTimingInfo {
                                        delay: Some(Duration::from_secs(4).into()),
                                    },
                                )),
                            }),
                            ..Default::default()
                        }),
                    },
                ],
                ..Default::default()
            }),
        ],
    }];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_mac_commands() {
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
        location: Some(Default::default()),
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

    let mut tx_info_lr_fhss = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_lr_fhss, 8).unwrap();

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
        rx1_delay: 1,
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "unconfirmed uplink + device-status request downlink (FOpts)".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.device_status_req_interval = 1;
                    device_profile::update(dp.clone()).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.device_status_req_interval = 0;
                    device_profile::update(dp.clone()).await.unwrap();
                })
            })),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([122, 152, 152, 220]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 1,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::Raw(
                                    vec![6],
                                )]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([250, 240, 150, 219]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 1,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::Raw(
                                    vec![6],
                                )]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([250, 240, 150, 219]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink + device-status request downlink (FOpts) + downlink payload"
                .into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 1,
                data: vec![1, 2, 3, 4],
                ..Default::default()
            }],
            before_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.device_status_req_interval = 1;
                    device_profile::update(dp.clone()).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.device_status_req_interval = 0;
                    device_profile::update(dp.clone()).await.unwrap();
                })
            })),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([122, 152, 152, 220]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 1,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::Raw(
                                    vec![6],
                                )]),
                            },
                            f_port: Some(1),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([21, 204, 213, 101]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 1,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::Raw(
                                    vec![6],
                                )]),
                            },
                            f_port: Some(1),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([21, 204, 213, 101]),
                    },
                ]),
            ],
        },
        Test {
            name: "RxTimingSetupAns is answered with an empty downlink".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 1,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::RxTimingSetupAns]),
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([0xb6, 0x20, 0xd2, 0x14]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0xc1, 0x0a, 0x08, 0xd9]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0xc1, 0x0a, 0x08, 0xd9]),
                    },
                ]),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_11_mac_commands() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_1_0,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
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
        location: Some(Default::default()),
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
        mac_version: common::MacVersion::Lorawan110.into(),
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
        rx1_delay: 1,
        ..Default::default()
    };

    let mut phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                f_cnt: 10,
                f_ctrl: lrwn::FCtrl {
                    f_opts_len: 1,
                    ..Default::default()
                },
                f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkCheckReq]),
                ..Default::default()
            },
            f_port: None,
            frm_payload: None,
        }),
        mic: None,
    };

    phy.encrypt_f_opts(&AES128Key::from_slice(&ds.nwk_s_enc_key).unwrap())
        .unwrap();
    phy.set_uplink_data_mic(
        lrwn::MACVersion::LoRaWAN1_1,
        0,
        0,
        0,
        &AES128Key::from_slice(&ds.f_nwk_s_int_key).unwrap(),
        &AES128Key::from_slice(&ds.s_nwk_s_int_key).unwrap(),
    )
    .unwrap();

    let tests = vec![Test {
        name: "uplink mac-command (encrypted fopts)".into(),
        device_queue_items: vec![],
        before_func: None,
        after_func: None,
        device_session: Some(ds.clone()),
        tx_info: tx_info.clone(),
        rx_info: rx_info.clone(),
        phy_payload: phy,
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 11),
            assert::downlink_phy_payloads(vec![
                lrwn::PhyPayload {
                    mhdr: lrwn::MHDR {
                        m_type: lrwn::MType::UnconfirmedDataDown,
                        major: lrwn::Major::LoRaWANR1,
                    },
                    payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                        fhdr: lrwn::FHDR {
                            devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                            f_cnt: 5,
                            f_ctrl: lrwn::FCtrl {
                                adr: true,
                                f_opts_len: 3,
                                ..Default::default()
                            },
                            f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::Raw(vec![
                                85, 88, 242,
                            ])]),
                            ..Default::default()
                        },
                        f_port: None,
                        frm_payload: None,
                    }),
                    mic: Some([124, 101, 247, 223]),
                },
                lrwn::PhyPayload {
                    mhdr: lrwn::MHDR {
                        m_type: lrwn::MType::UnconfirmedDataDown,
                        major: lrwn::Major::LoRaWANR1,
                    },
                    payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                        fhdr: lrwn::FHDR {
                            devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                            f_cnt: 5,
                            f_ctrl: lrwn::FCtrl {
                                adr: true,
                                f_opts_len: 3,
                                ..Default::default()
                            },
                            f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::Raw(vec![
                                85, 88, 242,
                            ])]),
                            ..Default::default()
                        },
                        f_port: None,
                        frm_payload: None,
                    }),
                    mic: Some([124, 101, 247, 223]),
                },
            ]),
        ],
    }];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_device_queue() {
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
        location: Some(Default::default()),
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
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "unconfirmed uplink + one unconfirmed downlink payload in queue".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![1, 2, 3, 4],
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 68, 8]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([180, 235, 116, 59]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([180, 235, 116, 59]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink + two unconfirmed downlinks payload in queue".into(),
            device_queue_items: vec![
                device_queue::DeviceQueueItem {
                    id: Uuid::new_v4(),
                    dev_eui: dev.dev_eui.clone(),
                    f_port: 10,
                    data: vec![1, 2, 3, 4],
                    ..Default::default()
                },
                device_queue::DeviceQueueItem {
                    id: Uuid::new_v4(),
                    dev_eui: dev.dev_eui.clone(),
                    f_port: 10,
                    data: vec![2, 2, 3, 4],
                    ..Default::default()
                },
            ],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 68, 8]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_pending: true,
                                    class_b: true, // bit shared with f_pending
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([126, 136, 139, 5]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_pending: true,
                                    class_b: true, // bit shared with f_pending
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([126, 136, 139, 5]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink + one confirmed downlink payload in queue".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![1, 2, 3, 4],
                confirmed: true,
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 68, 8]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::ConfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([175, 180, 11, 241]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::ConfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![115, 46, 73, 87])),
                        }),
                        mic: Some([175, 180, 11, 241]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink data + downlink payload which exceeds the max payload size (for dr 0)".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![0; 52],
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 68, 8]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::no_downlink_frame(),
                assert::integration_log(vec!["Device queue-item discarded because it exceeds the max. payload size".into()]),
            ],
        },
        Test {
			name: "unconfirmed uplink data + one unconfirmed downlink payload in queue (exactly max size for dr 0) + one mac command".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![0; 51],
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 1,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkCheckReq]),
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([106, 14, 124, 212]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    class_b: true,
                                    f_pending: true,
                                    f_opts_len: 3,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::Raw(vec![2,20,1]),
                                ]),
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([183, 102, 104, 194]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    class_b: true,
                                    f_pending: true,
                                    f_opts_len: 3,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::Raw(vec![2,20,1]),
                                ]),
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([183, 102, 104, 194]),
                    },
                ]),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_11_device_queue() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_1_0,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
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
        location: Some(Default::default()),
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
        mac_version: common::MacVersion::Lorawan110.into(),
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
        a_f_cnt_down: 3,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx2_frequency: 869525000,
        rx1_delay: 1,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let tests = vec![
        Test {
            name: "unconfirmed uplink + one unconfirmed downlink payload in queue".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![1, 2, 3, 4],
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 160, 195]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 3,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![142, 117, 88, 70])),
                        }),
                        mic: Some([75, 196, 253, 79]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 3,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![142, 117, 88, 70])),
                        }),
                        mic: Some([75, 196, 253, 79]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink + two unconfirmed downlinks payload in queue".into(),
            device_queue_items: vec![
                device_queue::DeviceQueueItem {
                    id: Uuid::new_v4(),
                    dev_eui: dev.dev_eui.clone(),
                    f_port: 10,
                    data: vec![1, 2, 3, 4],
                    ..Default::default()
                },
                device_queue::DeviceQueueItem {
                    id: Uuid::new_v4(),
                    dev_eui: dev.dev_eui.clone(),
                    f_port: 10,
                    data: vec![2, 2, 3, 4],
                    ..Default::default()
                },
            ],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 160, 195]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 3,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_pending: true,
                                    class_b: true, // bit shared with f_pending
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![142, 117, 88, 70])),
                        }),
                        mic: Some([148, 237, 227, 26]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 3,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_pending: true,
                                    class_b: true, // bit shared with f_pending
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![142, 117, 88, 70])),
                        }),
                        mic: Some([148, 237, 227, 26]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink + one confirmed downlink payload in queue".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![1, 2, 3, 4],
                confirmed: true,
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 160, 195]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::ConfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 3,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![142, 117, 88, 70])),
                        }),
                        mic: Some([144, 242, 239, 76]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::ConfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 3,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: Some(10),
                            frm_payload: Some(lrwn::FRMPayload::Raw(vec![142, 117, 88, 70])),
                        }),
                        mic: Some([144, 242, 239, 76]),
                    },
                ]),
            ],
        },
        Test {
            name: "unconfirmed uplink data + downlink payload which exceeds the max payload size (for dr 0)".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![0; 52],
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: Some(1),
                    frm_payload: None,
                }),
                mic: Some([160, 195, 160, 195]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::no_downlink_frame(),
                assert::integration_log(vec!["Device queue-item discarded because it exceeds the max. payload size".into()]),
            ],
        },
        Test {
			name: "unconfirmed uplink data + one unconfirmed downlink payload in queue (exactly max size for dr 0) + one mac command".into(),
            device_queue_items: vec![device_queue::DeviceQueueItem {
                id: Uuid::nil(),
                dev_eui: dev.dev_eui.clone(),
                f_port: 10,
                data: vec![0; 51],
                ..Default::default()
            }],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 1,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![
                            lrwn::MACCommand::Raw(vec![61]), // encrypted LinkCheckReq
                        ]),
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([204, 225, 204, 225]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    class_b: true,
                                    f_pending: true,
                                    f_opts_len: 3,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::Raw(vec![85, 88, 242]),
                                ]),
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([227, 104, 50, 163]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    class_b: true,
                                    f_pending: true,
                                    f_opts_len: 3,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::Raw(vec![85, 88, 242]),
                                ]),
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([227, 104, 50, 163]),
                    },
                ]),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_adr() {
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
        adr_algorithm_id: "default".into(),
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
        location: Some(Default::default()),
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
        rx1_delay: 1,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let ds_7chan = internal::DeviceSession {
        enabled_uplink_channel_indices: vec![0, 1, 2, 3, 4, 5, 6, 7],
        ..ds.clone()
    };

    let ds_backoff = internal::DeviceSession {
        dr: 5,
        tx_power_index: 3,
        uplink_adr_history: vec![internal::UplinkAdrHistory {
            f_cnt: 9,
            max_snr: 3.3,
            max_rssi: -120,
            tx_power_index: 3,
            gateway_count: 3,
        }],
        ..ds.clone()
    };

    let tests = vec![
        Test {
            name: "adr triggered".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            adr: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([187, 243, 244, 117]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads_decoded_f_opts(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 5,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                                        dr: 3,
                                        tx_power: 0,
                                        ch_mask: lrwn::ChMask::new([
                                            true, true, true, false, false, false, false, false,
                                            false, false, false, false, false, false, false, false,
                                        ]),
                                        redundancy: lrwn::Redundancy {
                                            ch_mask_cntl: 0,
                                            nb_rep: 1,
                                        },
                                    }),
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([57, 103, 222, 92]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 5,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                                        dr: 3,
                                        tx_power: 0,
                                        ch_mask: lrwn::ChMask::new([
                                            true, true, true, false, false, false, false, false,
                                            false, false, false, false, false, false, false, false,
                                        ]),
                                        redundancy: lrwn::Redundancy {
                                            ch_mask_cntl: 0,
                                            nb_rep: 1,
                                        },
                                    }),
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([57, 103, 222, 92]),
                    },
                ]),
            ],
        },
        Test {
            name: "device has adr disabled".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            adr: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([122, 152, 152, 220]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::no_downlink_frame(),
            ],
        },
        Test {
            name: "acknowledgement of pending adr request".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    mac_command::set_pending(
                        &dev_eui,
                        lrwn::CID::LinkADRReq,
                        &lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                            lrwn::LinkADRReqPayload {
                                dr: 0,
                                tx_power: 3,
                                ch_mask: lrwn::ChMask::new([
                                    true, true, true, false, false, false, false, false, false,
                                    false, false, false, false, false, false, false,
                                ]),
                                redundancy: lrwn::Redundancy {
                                    ch_mask_cntl: 0,
                                    nb_rep: 1,
                                },
                            },
                        )]),
                    )
                    .await
                    .unwrap();
                })
            })),
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 2,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRAns(
                            lrwn::LinkADRAnsPayload {
                                ch_mask_ack: true,
                                dr_ack: true,
                                tx_power_ack: true,
                            },
                        )]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([235, 224, 96, 3]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::tx_power_index(dev.dev_eui.clone(), 3),
                assert::dr(dev.dev_eui.clone(), 0),
                assert::nb_trans(dev.dev_eui.clone(), 1),
                assert::enabled_uplink_channel_indices(dev.dev_eui.clone(), vec![0, 1, 2]),
            ],
        },
        Test {
            name: "negative acknowledgement of pending adr request".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    mac_command::set_pending(
                        &dev_eui,
                        lrwn::CID::LinkADRReq,
                        &lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                            lrwn::LinkADRReqPayload {
                                dr: 0,
                                tx_power: 3,
                                ch_mask: lrwn::ChMask::new([
                                    true, true, true, false, false, false, false, false, false,
                                    false, false, false, false, false, false, false,
                                ]),
                                redundancy: lrwn::Redundancy {
                                    ch_mask_cntl: 0,
                                    nb_rep: 1,
                                },
                            },
                        )]),
                    )
                    .await
                    .unwrap();
                })
            })),
            after_func: None,
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 2,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRAns(
                            lrwn::LinkADRAnsPayload {
                                ch_mask_ack: false,
                                dr_ack: true,
                                tx_power_ack: true,
                            },
                        )]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([252, 17, 226, 74]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::tx_power_index(dev.dev_eui.clone(), 0),
                assert::dr(dev.dev_eui.clone(), 0),
                assert::nb_trans(dev.dev_eui.clone(), 0),
                assert::enabled_uplink_channel_indices(dev.dev_eui.clone(), vec![0, 1, 2]),
            ],
        },
        Test {
            name: "adr ack requested".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_7chan.clone()), // we want to see the NS to reset channels
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            adr_ack_req: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([73, 26, 32, 42]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::enabled_uplink_channel_indices(dev.dev_eui.clone(), vec![0, 1, 2]),
                assert::downlink_phy_payloads(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0xc1, 0xa, 0x8, 0xd9]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0xc1, 0xa, 0x8, 0xd9]),
                    },
                ]),
            ],
        },
        Test {
            name: "channel re-configuration triggered".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_7chan.clone()),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([122, 152, 152, 220]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::downlink_phy_payloads_decoded_f_opts(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 5,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                                        ch_mask: lrwn::ChMask::from_slice(&[true, true, true])
                                            .unwrap(),
                                        dr: 0,
                                        tx_power: 0,
                                        redundancy: lrwn::Redundancy {
                                            ch_mask_cntl: 0,
                                            nb_rep: 0,
                                        },
                                    }),
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0x8, 0xee, 0xdd, 0x34]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 5,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                                        ch_mask: lrwn::ChMask::from_slice(&[true, true, true])
                                            .unwrap(),
                                        dr: 0,
                                        tx_power: 0,
                                        redundancy: lrwn::Redundancy {
                                            ch_mask_cntl: 0,
                                            nb_rep: 0,
                                        },
                                    }),
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0x8, 0xee, 0xdd, 0x34]),
                    },
                ]),
                assert::enabled_uplink_channel_indices(
                    dev.dev_eui.clone(),
                    vec![0, 1, 2, 3, 4, 5, 6, 7],
                ),
            ],
        },
        Test {
            name: "new channel re-configuration ack-ed".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    mac_command::set_pending(
                        &dev_eui,
                        lrwn::CID::LinkADRReq,
                        &lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                            lrwn::LinkADRReqPayload {
                                dr: 0,
                                tx_power: 1,
                                ch_mask: lrwn::ChMask::new([
                                    true, true, true, false, false, false, false, false, false,
                                    false, false, false, false, false, false, false,
                                ]),
                                redundancy: lrwn::Redundancy {
                                    ch_mask_cntl: 0,
                                    nb_rep: 0,
                                },
                            },
                        )]),
                    )
                    .await
                    .unwrap();
                })
            })),
            after_func: None,
            device_session: Some(ds_7chan.clone()),
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 2,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRAns(
                            lrwn::LinkADRAnsPayload {
                                ch_mask_ack: true,
                                dr_ack: true,
                                tx_power_ack: true,
                            },
                        )]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([235, 224, 96, 3]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::no_downlink_frame(),
                assert::enabled_uplink_channel_indices(dev.dev_eui.clone(), vec![0, 1, 2]),
            ],
        },
        Test {
            name: "new channel re-configuration not ack-ed".into(),
            device_queue_items: vec![],
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    mac_command::set_pending(
                        &dev_eui,
                        lrwn::CID::LinkADRReq,
                        &lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRReq(
                            lrwn::LinkADRReqPayload {
                                dr: 0,
                                tx_power: 1,
                                ch_mask: lrwn::ChMask::new([
                                    true, true, true, false, false, false, false, false, false,
                                    false, false, false, false, false, false, false,
                                ]),
                                redundancy: lrwn::Redundancy {
                                    ch_mask_cntl: 0,
                                    nb_rep: 0,
                                },
                            },
                        )]),
                    )
                    .await
                    .unwrap();
                })
            })),
            after_func: None,
            device_session: Some(ds_7chan.clone()),
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 2,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkADRAns(
                            lrwn::LinkADRAnsPayload {
                                ch_mask_ack: false,
                                dr_ack: true,
                                tx_power_ack: true,
                            },
                        )]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([252, 17, 226, 74]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::enabled_uplink_channel_indices(
                    dev.dev_eui.clone(),
                    vec![0, 1, 2, 3, 4, 5, 6, 7],
                ),
                assert::mac_command_error_count(dev.dev_eui.clone(), lrwn::CID::LinkADRReq, 1),
            ],
        },
        Test {
            name: "channel re-configuration and adr triggered".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_7chan.clone()),
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            adr: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([187, 243, 244, 117]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::n_f_cnt_down(dev.dev_eui.clone(), 5),
                assert::enabled_uplink_channel_indices(
                    dev.dev_eui.clone(),
                    vec![0, 1, 2, 3, 4, 5, 6, 7],
                ),
                assert::downlink_phy_payloads_decoded_f_opts(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 5,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                                        dr: 3,
                                        tx_power: 0,
                                        ch_mask: lrwn::ChMask::from_slice(&[true, true, true])
                                            .unwrap(),
                                        redundancy: lrwn::Redundancy {
                                            ch_mask_cntl: 0,
                                            nb_rep: 1,
                                        },
                                    }),
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([57, 103, 222, 92]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 5,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::LinkADRReq(lrwn::LinkADRReqPayload {
                                        dr: 3,
                                        tx_power: 0,
                                        ch_mask: lrwn::ChMask::from_slice(&[true, true, true])
                                            .unwrap(),
                                        redundancy: lrwn::Redundancy {
                                            ch_mask_cntl: 0,
                                            nb_rep: 1,
                                        },
                                    }),
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([57, 103, 222, 92]),
                    },
                ]),
            ],
        },
        // adr backoff triggered
        Test {
            name: "adr backoff triggered".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_backoff.clone()),
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            adr: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([187, 243, 244, 117]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::dr(dev.dev_eui.clone(), 0),
                assert::tx_power_index(dev.dev_eui.clone(), 0),
                assert::uplink_adr_history(
                    dev.dev_eui.clone(),
                    vec![internal::UplinkAdrHistory {
                        f_cnt: 10,
                        max_snr: 0.0,
                        max_rssi: 0,
                        tx_power_index: 0,
                        gateway_count: 1,
                    }],
                ),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10_device_status_request() {
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
        device_status_req_interval: 24,
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
        location: Some(Default::default()),
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
        rx1_delay: 1,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let ds_not_yet_expired = internal::DeviceSession {
        last_device_status_request: Some(chrono::Utc::now().into()),
        ..ds.clone()
    };

    let tests = vec![
        Test {
            name: "must request device-status".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([122, 152, 152, 220]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::downlink_phy_payloads_decoded_f_opts(vec![
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 1,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::DevStatusReq,
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0xfa, 0xf0, 0x96, 0xdb]),
                    },
                    lrwn::PhyPayload {
                        mhdr: lrwn::MHDR {
                            m_type: lrwn::MType::UnconfirmedDataDown,
                            major: lrwn::Major::LoRaWANR1,
                        },
                        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                            fhdr: lrwn::FHDR {
                                devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
                                f_cnt: 5,
                                f_ctrl: lrwn::FCtrl {
                                    adr: true,
                                    f_opts_len: 1,
                                    ..Default::default()
                                },
                                f_opts: lrwn::MACCommandSet::new(vec![
                                    lrwn::MACCommand::DevStatusReq,
                                ]),
                            },
                            f_port: None,
                            frm_payload: None,
                        }),
                        mic: Some([0xfa, 0xf0, 0x96, 0xdb]),
                    },
                ]),
            ],
        },
        Test {
            name: "interval has not yet expired".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_not_yet_expired.clone()),
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
                        f_cnt: 10,
                        ..Default::default()
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([122, 152, 152, 220]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::no_downlink_frame(),
            ],
        },
        // reporting device-status
        Test {
            name: "reporting device-status".into(),
            device_queue_items: vec![],
            before_func: None,
            after_func: None,
            device_session: Some(ds_not_yet_expired.clone()),
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
                        f_cnt: 10,
                        f_ctrl: lrwn::FCtrl {
                            f_opts_len: 3,
                            ..Default::default()
                        },
                        f_opts: lrwn::MACCommandSet::new(vec![lrwn::MACCommand::DevStatusAns(
                            lrwn::DevStatusAnsPayload {
                                battery: 128,
                                margin: 10,
                            },
                        )]),
                    },
                    f_port: None,
                    frm_payload: None,
                }),
                mic: Some([29, 141, 54, 155]),
            },
            assert: vec![
                assert::f_cnt_up(dev.dev_eui.clone(), 11),
                assert::status_event(integration_pb::StatusEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_name: t.name.clone(),
                        tenant_id: t.id.to_string(),
                        application_name: app.name.clone(),
                        application_id: app.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    margin: 10,
                    battery_level: 50.3937,
                    ..Default::default()
                }),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_11_receive_window_selection() {
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
        mac_version: lrwn::region::MacVersion::LORAWAN_1_1_0,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
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
        location: Some(Default::default()),
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

    let mut tx_info_lr_fhss = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info_lr_fhss, 8).unwrap();

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan110.into(),
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
        conf_f_cnt: 4,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx2_frequency: 869525000,
        rx1_delay: 1,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let mut conf = (*config::get()).clone();
    conf.regions[0].network.rx_window = 1;
    config::set(conf);

    run_test(&Test {
        name: "unconfirmed uplink with payload (rx1)".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 1,
            data: vec![1],
            ..Default::default()
        }],
        before_func: None,
        after_func: None,
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
                    f_cnt: 10,
                    ..Default::default()
                },
                f_port: Some(1),
                frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
            }),
            mic: Some([104, 147, 104, 147]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 11),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![96, 4, 3, 2, 1, 128, 0, 0, 1, 240, 144, 155, 230, 40],
                    tx_info_legacy: None,
                    tx_info: Some(gw::DownlinkTxInfo {
                        frequency: 868100000,
                        power: 14,
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
                            parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                                delay: Some(Duration::from_secs(1).into()),
                            })),
                        }),
                        ..Default::default()
                    }),
                }],
                ..Default::default()
            }),
        ],
    })
    .await;

    let mut conf = (*config::get()).clone();
    conf.regions[0].network.rx_window = 2;
    config::set(conf);

    run_test(&Test {
        name: "unconfirmed uplink with payload (rx2)".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 1,
            data: vec![1],
            ..Default::default()
        }],
        before_func: None,
        after_func: None,
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
                    f_cnt: 10,
                    ..Default::default()
                },
                f_port: Some(1),
                frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
            }),
            mic: Some([104, 147, 104, 147]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 11),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![gw::DownlinkFrameItem {
                    phy_payload: vec![96, 4, 3, 2, 1, 128, 0, 0, 1, 240, 144, 155, 230, 40],
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
                            parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                                delay: Some(Duration::from_secs(2).into()),
                            })),
                        }),
                        ..Default::default()
                    }),
                }],
                ..Default::default()
            }),
        ],
    })
    .await;

    let mut conf = (*config::get()).clone();
    conf.regions[0].network.rx_window = 0;
    config::set(conf);

    run_test(&Test {
        name: "unconfirmed uplink with payload (rx1 + rx2)".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 1,
            data: vec![1],
            ..Default::default()
        }],
        before_func: None,
        after_func: None,
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
                    f_cnt: 10,
                    ..Default::default()
                },
                f_port: Some(1),
                frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
            }),
            mic: Some([104, 147, 104, 147]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 11),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![
                    gw::DownlinkFrameItem {
                        phy_payload: vec![96, 4, 3, 2, 1, 128, 0, 0, 1, 240, 144, 155, 230, 40],
                        tx_info_legacy: None,
                        tx_info: Some(gw::DownlinkTxInfo {
                            frequency: 868100000,
                            power: 14,
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
                                parameters: Some(gw::timing::Parameters::Delay(
                                    gw::DelayTimingInfo {
                                        delay: Some(Duration::from_secs(1).into()),
                                    },
                                )),
                            }),
                            ..Default::default()
                        }),
                    },
                    gw::DownlinkFrameItem {
                        phy_payload: vec![96, 4, 3, 2, 1, 128, 0, 0, 1, 240, 144, 155, 230, 40],
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
                                parameters: Some(gw::timing::Parameters::Delay(
                                    gw::DelayTimingInfo {
                                        delay: Some(Duration::from_secs(2).into()),
                                    },
                                )),
                            }),
                            ..Default::default()
                        }),
                    },
                ],
                ..Default::default()
            }),
        ],
    })
    .await;

    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info, 5).unwrap();

    run_test(&Test {
        name: "unconfirmed uplink with payload (rx1, payload exceeds rx2 limit)".into(),
        device_queue_items: vec![device_queue::DeviceQueueItem {
            id: Uuid::nil(),
            dev_eui: dev.dev_eui.clone(),
            f_port: 1,
            data: vec![0; 100],
            ..Default::default()
        }],
        before_func: None,
        after_func: None,
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
                    f_cnt: 10,
                    ..Default::default()
                },
                f_port: Some(1),
                frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
            }),
            mic: Some([0xd4, 0x59, 0x68, 0x93]),
        },
        assert: vec![
            assert::f_cnt_up(dev.dev_eui.clone(), 11),
            assert::downlink_frame(gw::DownlinkFrame {
                gateway_id: "0102030405060708".into(),
                items: vec![
                    gw::DownlinkFrameItem {
                        phy_payload: vec![
                            96, 4, 3, 2, 1, 128, 0, 0, 1, 241, 182, 107, 217, 100, 233, 112, 234,
                            103, 130, 120, 149, 93, 124, 5, 214, 1, 4, 6, 231, 84, 91, 241, 75,
                            121, 151, 52, 245, 144, 78, 149, 186, 84, 236, 188, 0, 218, 131, 120,
                            151, 163, 8, 184, 104, 212, 205, 194, 222, 92, 137, 144, 60, 251, 141,
                            71, 87, 8, 192, 155, 215, 169, 180, 156, 204, 162, 159, 71, 200, 32,
                            15, 39, 173, 94, 78, 204, 43, 254, 9, 12, 132, 91, 8, 2, 93, 149, 184,
                            152, 187, 95, 49, 125, 43, 91, 149, 177, 152, 82, 34, 124, 17, 64, 200,
                            9, 227,
                        ],
                        tx_info_legacy: None,
                        tx_info: Some(gw::DownlinkTxInfo {
                            frequency: 868100000,
                            power: 14,
                            modulation: Some(gw::Modulation {
                                parameters: Some(gw::modulation::Parameters::Lora(
                                    gw::LoraModulationInfo {
                                        bandwidth: 125000,
                                        spreading_factor: 7,
                                        code_rate: gw::CodeRate::Cr45.into(),
                                        polarization_inversion: true,
                                        ..Default::default()
                                    },
                                )),
                            }),
                            timing: Some(gw::Timing {
                                parameters: Some(gw::timing::Parameters::Delay(
                                    gw::DelayTimingInfo {
                                        delay: Some(Duration::from_secs(1).into()),
                                    },
                                )),
                            }),
                            ..Default::default()
                        }),
                    },
                    gw::DownlinkFrameItem {
                        phy_payload: vec![96, 4, 3, 2, 1, 144, 5, 0, 88, 62, 73, 197], // f_pending = true
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
                                parameters: Some(gw::timing::Parameters::Delay(
                                    gw::DelayTimingInfo {
                                        delay: Some(Duration::from_secs(2).into()),
                                    },
                                )),
                            }),
                            ..Default::default()
                        }),
                    },
                ],
                ..Default::default()
            }),
        ],
    })
    .await;
}

async fn run_test(t: &Test) {
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

    if let Some(f) = &t.before_func {
        f().await;
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

    if let Some(f) = &t.after_func {
        f().await;
    }
}
