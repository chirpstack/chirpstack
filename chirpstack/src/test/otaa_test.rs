use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use uuid::Uuid;

use super::assert;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_keys, device_profile, gateway, reset_redis, tenant,
};
use crate::{config, gateway::backend as gateway_backend, integration, region, test, uplink};
use chirpstack_api::{common, gw, internal, meta};
use lrwn::keys::get_js_int_key;
use lrwn::{AES128Key, EUI64};

type Function = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>;

struct Test {
    name: String,
    before_func: Option<Function>,
    after_func: Option<Function>,
    tx_info: gw::UplinkTxInfo,
    rx_info: gw::UplinkRxInfo,
    phy_payload: lrwn::PhyPayload,
    extra_uplink_channels: Vec<u32>,
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

    let dk = device_keys::create(device_keys::DeviceKeys {
        dev_eui: dev.dev_eui.clone(),
        nwk_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        dev_nonces: vec![Some(258)],
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

    let mut jr_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinRequest,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
            join_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dev_eui: dev.dev_eui.clone(),
            dev_nonce: 258,
        }),
        mic: None,
    };
    jr_pl.set_join_request_mic(&dk.nwk_key).unwrap();

    let tests = vec![
        Test {
            name: "private gateway of same tenant".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();
                })
            })),
            after_func: None,
            rx_info: rx_info_a.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![assert::device_session(
                dev.dev_eui.clone(),
                internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                    join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    mac_version: common::MacVersion::Lorawan102.into(),
                    f_nwk_s_int_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    s_nwk_s_int_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    nwk_s_enc_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    app_s_key: Some(common::KeyEnvelope {
                        kek_label: "".into(),
                        aes_key: vec![
                            5, 211, 222, 240, 51, 52, 23, 15, 218, 155, 237, 228, 198, 37, 200, 117,
                        ],
                    }),
                    rx1_delay: 1,
                    rx2_frequency: 869525000,
                    enabled_uplink_channel_indices: vec![0, 1, 2],
                    nb_trans: 1,
                    region_config_id: "eu868".to_string(),
                    class_b_ping_slot_nb: 1,
                    ..Default::default()
                },
            )],
        },
        Test {
            name: "private gateway other tenant".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();
                })
            })),
            after_func: None,
            rx_info: rx_info_b.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![assert::no_device_session(dev.dev_eui.clone())],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_10() {
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
        enabled_class: DeviceClass::B,
        ..Default::default()
    })
    .await
    .unwrap();

    let dk = device_keys::create(device_keys::DeviceKeys {
        dev_eui: dev.dev_eui.clone(),
        nwk_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        dev_nonces: vec![Some(258)],
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

    let mut jr_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinRequest,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
            join_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dev_eui: dev.dev_eui.clone(),
            dev_nonce: 258,
        }),
        mic: None,
    };
    jr_pl.set_join_request_mic(&dk.nwk_key).unwrap();

    let mut ja_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinAccept,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinAccept(lrwn::JoinAcceptPayload {
            join_nonce: 0,
            home_netid: lrwn::NetID::from_be_bytes([0, 0, 0]),
            devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
            dl_settings: lrwn::DLSettings {
                rx2_dr: 0,
                rx1_dr_offset: 0,
                opt_neg: false,
            },
            rx_delay: 1,
            cflist: None,
        }),
        mic: None,
    };
    ja_pl
        .set_join_accept_mic(
            lrwn::JoinType::Join,
            &EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            258,
            &dk.nwk_key,
        )
        .unwrap();
    ja_pl.encrypt_join_accept_payload(&dk.nwk_key).unwrap();

    let mut ja_cflist_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinAccept,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinAccept(lrwn::JoinAcceptPayload {
            join_nonce: 0,
            home_netid: lrwn::NetID::from_be_bytes([0, 0, 0]),
            devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
            dl_settings: lrwn::DLSettings {
                rx2_dr: 0,
                rx1_dr_offset: 0,
                opt_neg: false,
            },
            rx_delay: 1,
            cflist: Some(lrwn::CFList::Channels(lrwn::CFListChannels::new([
                867100000, 867300000, 867500000, 867700000, 867900000,
            ]))),
        }),
        mic: None,
    };
    ja_cflist_pl
        .set_join_accept_mic(
            lrwn::JoinType::Join,
            &EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            258,
            &dk.nwk_key,
        )
        .unwrap();
    ja_cflist_pl
        .encrypt_join_accept_payload(&dk.nwk_key)
        .unwrap();

    let tests = vec![
        Test {
            name: "dev-nonce already used".into(),
            before_func: None,
            after_func: None,
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![assert::integration_log(vec![
                "DevNonce has already been used".to_string(),
            ])],
        },
        Test {
            name: "join-request accepted".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();
                })
            })),
            after_func: None,
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![
                assert::device_join_eui(
                    dev.dev_eui,
                    EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
                ),
                assert::device_session(
                    dev.dev_eui.clone(),
                    internal::DeviceSession {
                        dev_addr: vec![1, 2, 3, 4],
                        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                        join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        mac_version: common::MacVersion::Lorawan102.into(),
                        f_nwk_s_int_key: vec![
                            128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                        ],
                        s_nwk_s_int_key: vec![
                            128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                        ],
                        nwk_s_enc_key: vec![
                            128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                        ],
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "".into(),
                            aes_key: vec![
                                5, 211, 222, 240, 51, 52, 23, 15, 218, 155, 237, 228, 198, 37, 200,
                                117,
                            ],
                        }),
                        rx1_delay: 1,
                        rx2_frequency: 869525000,
                        enabled_uplink_channel_indices: vec![0, 1, 2],
                        nb_trans: 1,
                        region_config_id: "eu868".to_string(),
                        class_b_ping_slot_nb: 1,
                        ..Default::default()
                    },
                ),
                assert::downlink_frame(gw::DownlinkFrame {
                    items: vec![
                        gw::DownlinkFrameItem {
                            phy_payload: ja_pl.to_vec().unwrap(),
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
                                            delay: Some(Duration::from_secs(5).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                        gw::DownlinkFrameItem {
                            phy_payload: ja_pl.to_vec().unwrap(),
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
                                            delay: Some(Duration::from_secs(6).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                    ],
                    gateway_id: "0102030405060708".to_string(),
                    ..Default::default()
                }),
                assert::downlink_frame_saved(internal::DownlinkFrame {
                    dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                    nwk_s_enc_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    downlink_frame: Some(gw::DownlinkFrame {
                        items: vec![
                            gw::DownlinkFrameItem {
                                phy_payload: ja_pl.to_vec().unwrap(),
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
                                                delay: Some(Duration::from_secs(5).into()),
                                            },
                                        )),
                                    }),
                                    ..Default::default()
                                }),
                            },
                            gw::DownlinkFrameItem {
                                phy_payload: ja_pl.to_vec().unwrap(),
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
                                                delay: Some(Duration::from_secs(6).into()),
                                            },
                                        )),
                                    }),
                                    ..Default::default()
                                }),
                            },
                        ],
                        gateway_id: "0102030405060708".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                assert::enabled_class(dev.dev_eui.clone(), DeviceClass::A),
                assert::device_queue_items(dev.dev_eui.clone(), vec![]),
                assert::uplink_meta_log(meta::UplinkMeta {
                    dev_eui: dev.dev_eui.to_string(),
                    tx_info: Some(tx_info.clone()),
                    rx_info: vec![rx_info.clone()],
                    phy_payload_byte_count: 23,
                    message_type: common::MType::JoinRequest.into(),
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "join-request accepted + skip fcnt check".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();

                    let mut dev = device::get(&dev_eui).await.unwrap();
                    dev.skip_fcnt_check = true;
                    let _ = device::update(dev).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    let mut dev = device::get(&dev_eui).await.unwrap();
                    dev.skip_fcnt_check = false;
                    let _ = device::update(dev).await.unwrap();
                })
            })),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![assert::device_session(
                dev.dev_eui.clone(),
                internal::DeviceSession {
                    dev_addr: vec![1, 2, 3, 4],
                    dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                    join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    mac_version: common::MacVersion::Lorawan102.into(),
                    f_nwk_s_int_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    s_nwk_s_int_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    nwk_s_enc_key: vec![
                        128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                    ],
                    app_s_key: Some(common::KeyEnvelope {
                        kek_label: "".into(),
                        aes_key: vec![
                            5, 211, 222, 240, 51, 52, 23, 15, 218, 155, 237, 228, 198, 37, 200, 117,
                        ],
                    }),
                    rx1_delay: 1,
                    rx2_frequency: 869525000,
                    enabled_uplink_channel_indices: vec![0, 1, 2],
                    nb_trans: 1,
                    region_config_id: "eu868".to_string(),
                    skip_f_cnt_check: true,
                    class_b_ping_slot_nb: 1,
                    ..Default::default()
                },
            )],
        },
        Test {
            name: "join-request accepted + cflist".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();
                })
            })),
            after_func: None,
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![867100000, 867300000, 867500000, 867700000, 867900000],
            assert: vec![
                assert::device_session(
                    dev.dev_eui.clone(),
                    internal::DeviceSession {
                        dev_addr: vec![1, 2, 3, 4],
                        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                        join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        mac_version: common::MacVersion::Lorawan102.into(),
                        f_nwk_s_int_key: vec![
                            128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                        ],
                        s_nwk_s_int_key: vec![
                            128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                        ],
                        nwk_s_enc_key: vec![
                            128, 47, 168, 41, 62, 215, 212, 79, 19, 83, 183, 201, 43, 169, 125, 200,
                        ],
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "".into(),
                            aes_key: vec![
                                5, 211, 222, 240, 51, 52, 23, 15, 218, 155, 237, 228, 198, 37, 200,
                                117,
                            ],
                        }),
                        rx1_delay: 1,
                        rx2_frequency: 869525000,
                        enabled_uplink_channel_indices: vec![0, 1, 2, 3, 4, 5, 6, 7],
                        extra_uplink_channels: [
                            (
                                3,
                                internal::DeviceSessionChannel {
                                    frequency: 867100000,
                                    min_dr: 0,
                                    max_dr: 5,
                                },
                            ),
                            (
                                4,
                                internal::DeviceSessionChannel {
                                    frequency: 867300000,
                                    min_dr: 0,
                                    max_dr: 5,
                                },
                            ),
                            (
                                5,
                                internal::DeviceSessionChannel {
                                    frequency: 867500000,
                                    min_dr: 0,
                                    max_dr: 5,
                                },
                            ),
                            (
                                6,
                                internal::DeviceSessionChannel {
                                    frequency: 867700000,
                                    min_dr: 0,
                                    max_dr: 5,
                                },
                            ),
                            (
                                7,
                                internal::DeviceSessionChannel {
                                    frequency: 867900000,
                                    min_dr: 0,
                                    max_dr: 5,
                                },
                            ),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                        nb_trans: 1,
                        region_config_id: "eu868".to_string(),
                        class_b_ping_slot_nb: 1,
                        ..Default::default()
                    },
                ),
                assert::downlink_frame(gw::DownlinkFrame {
                    items: vec![
                        gw::DownlinkFrameItem {
                            phy_payload: ja_cflist_pl.to_vec().unwrap(),
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
                                            delay: Some(Duration::from_secs(5).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                        gw::DownlinkFrameItem {
                            phy_payload: ja_cflist_pl.to_vec().unwrap(),
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
                                            delay: Some(Duration::from_secs(6).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                    ],
                    gateway_id: "0102030405060708".into(),
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "join-request accepted + class-b supported".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();

                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_b = true;
                    let _ = device_profile::update(dp).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_b = false;
                    let _ = device_profile::update(dp).await.unwrap();
                })
            })),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: Vec::new(),
            assert: vec![assert::enabled_class(dev.dev_eui.clone(), DeviceClass::A)],
        },
        Test {
            name: "join-request accepted + class-c supported".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();

                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_c = true;
                    let _ = device_profile::update(dp).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_c = false;
                    let _ = device_profile::update(dp).await.unwrap();
                })
            })),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: Vec::new(),
            assert: vec![assert::enabled_class(dev.dev_eui.clone(), DeviceClass::C)],
        },
        Test {
            name: "device disabled".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();

                    let mut dev = device::get(&dev_eui).await.unwrap();
                    dev.is_disabled = true;
                    let _ = device::update(dev).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    let mut dev = device::get(&dev_eui).await.unwrap();
                    dev.is_disabled = false;
                    let _ = device::update(dev).await.unwrap();
                })
            })),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: Vec::new(),
            assert: vec![assert::no_downlink_frame()],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

#[tokio::test]
async fn test_lorawan_11() {
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
        enabled_class: DeviceClass::B,
        ..Default::default()
    })
    .await
    .unwrap();

    let dk = device_keys::create(device_keys::DeviceKeys {
        dev_eui: dev.dev_eui.clone(),
        nwk_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
        app_key: AES128Key::from_bytes([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
        dev_nonces: vec![Some(258)],
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

    let mut jr_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinRequest,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
            join_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dev_eui: dev.dev_eui.clone(),
            dev_nonce: 258,
        }),
        mic: None,
    };
    jr_pl.set_join_request_mic(&dk.nwk_key).unwrap();

    let mut ja_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinAccept,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinAccept(lrwn::JoinAcceptPayload {
            join_nonce: 0,
            home_netid: lrwn::NetID::from_be_bytes([0, 0, 0]),
            devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
            dl_settings: lrwn::DLSettings {
                rx2_dr: 0,
                rx1_dr_offset: 0,
                opt_neg: true,
            },
            rx_delay: 1,
            cflist: None,
        }),
        mic: None,
    };
    ja_pl
        .set_join_accept_mic(
            lrwn::JoinType::Join,
            &EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            258,
            &get_js_int_key(&dev.dev_eui, &dk.nwk_key).unwrap(),
        )
        .unwrap();
    ja_pl.encrypt_join_accept_payload(&dk.nwk_key).unwrap();

    let tests = vec![
        Test {
            name: "dev-nonce already used".into(),
            before_func: None,
            after_func: None,
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![assert::integration_log(vec![
                "DevNonce has already been used".to_string(),
            ])],
        },
        Test {
            name: "join-request accepted".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();
                })
            })),
            after_func: None,
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: vec![],
            assert: vec![
                assert::device_session(
                    dev.dev_eui.clone(),
                    internal::DeviceSession {
                        dev_addr: vec![1, 2, 3, 4],
                        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                        join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        mac_version: common::MacVersion::Lorawan110.into(),
                        f_nwk_s_int_key: vec![
                            98, 222, 198, 158, 98, 155, 205, 235, 143, 171, 203, 19, 221, 9, 1, 231,
                        ],
                        s_nwk_s_int_key: vec![
                            8, 16, 172, 220, 92, 121, 168, 210, 224, 162, 133, 180, 191, 167, 33,
                            73,
                        ],
                        nwk_s_enc_key: vec![
                            151, 120, 115, 101, 67, 122, 194, 153, 113, 209, 134, 158, 149, 189,
                            192, 175,
                        ],
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "".into(),
                            aes_key: vec![
                                27, 30, 215, 60, 144, 234, 251, 130, 186, 67, 197, 148, 250, 49,
                                106, 77,
                            ],
                        }),
                        rx1_delay: 1,
                        rx2_frequency: 869525000,
                        enabled_uplink_channel_indices: vec![0, 1, 2],
                        nb_trans: 1,
                        region_config_id: "eu868".to_string(),
                        class_b_ping_slot_nb: 1,
                        ..Default::default()
                    },
                ),
                assert::downlink_frame(gw::DownlinkFrame {
                    items: vec![
                        gw::DownlinkFrameItem {
                            phy_payload: ja_pl.to_vec().unwrap(),
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
                                            delay: Some(Duration::from_secs(5).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                        gw::DownlinkFrameItem {
                            phy_payload: ja_pl.to_vec().unwrap(),
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
                                            delay: Some(Duration::from_secs(6).into()),
                                        },
                                    )),
                                }),
                                ..Default::default()
                            }),
                        },
                    ],
                    gateway_id: "0102030405060708".to_string(),
                    ..Default::default()
                }),
                assert::downlink_frame_saved(internal::DownlinkFrame {
                    dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                    nwk_s_enc_key: vec![
                        151, 120, 115, 101, 67, 122, 194, 153, 113, 209, 134, 158, 149, 189, 192,
                        175,
                    ],
                    downlink_frame: Some(gw::DownlinkFrame {
                        items: vec![
                            gw::DownlinkFrameItem {
                                phy_payload: ja_pl.to_vec().unwrap(),
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
                                                delay: Some(Duration::from_secs(5).into()),
                                            },
                                        )),
                                    }),
                                    ..Default::default()
                                }),
                            },
                            gw::DownlinkFrameItem {
                                phy_payload: ja_pl.to_vec().unwrap(),
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
                                                delay: Some(Duration::from_secs(6).into()),
                                            },
                                        )),
                                    }),
                                    ..Default::default()
                                }),
                            },
                        ],
                        gateway_id: "0102030405060708".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                assert::enabled_class(dev.dev_eui.clone(), DeviceClass::A),
                assert::device_queue_items(dev.dev_eui.clone(), vec![]),
            ],
        },
        Test {
            name: "join-request accepted + class-c supported".into(),
            before_func: Some(Box::new(move || {
                let dev_eui = dev.dev_eui.clone();
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    device_keys::test::reset_nonces(&dev_eui).await.unwrap();

                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_c = true;
                    let _ = device_profile::update(dp).await.unwrap();
                })
            })),
            after_func: Some(Box::new(move || {
                let dp_id = dp.id.clone();
                Box::pin(async move {
                    let mut dp = device_profile::get(&dp_id).await.unwrap();
                    dp.supports_class_c = false;
                    let _ = device_profile::update(dp).await.unwrap();
                })
            })),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: jr_pl.clone(),
            extra_uplink_channels: Vec::new(),
            assert: vec![assert::enabled_class(dev.dev_eui.clone(), DeviceClass::A)],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

async fn run_test(t: &Test) {
    println!("> {}", t.name);

    reset_redis().await.unwrap();

    let mut conf: config::Configuration = (*config::get()).clone();
    for f in &t.extra_uplink_channels {
        conf.regions[0]
            .network
            .extra_channels
            .push(config::ExtraChannel {
                frequency: *f,
                min_dr: 0,
                max_dr: 5,
            });
    }
    config::set(conf);
    region::setup().unwrap();

    integration::set_mock().await;
    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    integration::mock::reset().await;
    gateway_backend::mock::reset().await;

    if let Some(f) = &t.before_func {
        f().await;
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
