use std::time::Duration;

use uuid::Uuid;

use super::assert;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_profile, device_queue, device_session, gateway, reset_redis, tenant,
};
use crate::{gateway::backend as gateway_backend, integration, test, uplink};
use chirpstack_api::{common, gw, integration as integration_pb, internal};
use lrwn::{AES128Key, EUI64};

struct Test {
    name: String,
    device_queue_items_relay_ed: Vec<device_queue::DeviceQueueItem>,
    device_session_relay: Option<internal::DeviceSession>,
    device_session_relay_ed: Option<internal::DeviceSession>,
    tx_info: gw::UplinkTxInfo,
    rx_info: gw::UplinkRxInfo,
    phy_payload: lrwn::PhyPayload,
    assert: Vec<assert::Validator>,
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

    let dp_relay = device_profile::create(device_profile::DeviceProfile {
        name: "dp-relay".into(),
        tenant_id: t.id,
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_4,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
        is_relay: true,
        relay_enabled: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let dp_relay_ed = device_profile::create(device_profile::DeviceProfile {
        name: "dp-relay-ed".into(),
        tenant_id: t.id,
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_4,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
        is_relay_ed: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let dev_relay = device::create(device::Device {
        name: "dev-relay".into(),
        application_id: app.id,
        device_profile_id: dp_relay.id,
        dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
        enabled_class: DeviceClass::A,
        ..Default::default()
    })
    .await
    .unwrap();

    let dev_relay_ed = device::create(device::Device {
        name: "dev-relay-ed".into(),
        application_id: app.id,
        device_profile_id: dp_relay_ed.id,
        dev_eui: EUI64::from_be_bytes([2, 2, 2, 2, 2, 2, 2, 2]),
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
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info, 5).unwrap();

    let ds_relay = internal::DeviceSession {
        dev_eui: dev_relay.dev_eui.to_vec(),
        mac_version: common::MacVersion::Lorawan104.into(),
        dev_addr: vec![1, 1, 1, 1],
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

    let ds_relay_ed = internal::DeviceSession {
        dev_eui: dev_relay_ed.dev_eui.to_vec(),
        mac_version: common::MacVersion::Lorawan104.into(),
        dev_addr: vec![2, 2, 2, 2],
        f_nwk_s_int_key: vec![2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        app_s_key: Some(common::KeyEnvelope {
            kek_label: "".into(),
            aes_key: vec![16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 2],
        }),
        f_cnt_up: 88,
        n_f_cnt_down: 55,
        enabled_uplink_channel_indices: vec![0, 1, 2],
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };

    let mut phy_relay_ed_unconfirmed_up = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_slice(&ds_relay_ed.dev_addr).unwrap(),
                f_cnt: 88,
                ..Default::default()
            },
            f_port: Some(1),
            frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
        }),
        mic: None,
    };
    phy_relay_ed_unconfirmed_up
        .encrypt_frm_payload(
            &AES128Key::from_slice(&ds_relay_ed.app_s_key.as_ref().unwrap().aes_key).unwrap(),
        )
        .unwrap();
    phy_relay_ed_unconfirmed_up
        .set_uplink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            0,
            0,
            &AES128Key::from_slice(&ds_relay_ed.f_nwk_s_int_key).unwrap(),
            &AES128Key::from_slice(&ds_relay_ed.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let mut phy_relay_ed_confirmed_up = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::ConfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_slice(&ds_relay_ed.dev_addr).unwrap(),
                f_cnt: 88,
                ..Default::default()
            },
            f_port: Some(1),
            frm_payload: Some(lrwn::FRMPayload::Raw(vec![1, 2, 3, 4])),
        }),
        mic: None,
    };
    phy_relay_ed_confirmed_up
        .encrypt_frm_payload(
            &AES128Key::from_slice(&ds_relay_ed.app_s_key.as_ref().unwrap().aes_key).unwrap(),
        )
        .unwrap();
    phy_relay_ed_confirmed_up
        .set_uplink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            0,
            0,
            &AES128Key::from_slice(&ds_relay_ed.f_nwk_s_int_key).unwrap(),
            &AES128Key::from_slice(&ds_relay_ed.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let mut phy_relay_unconfirmed_up = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_slice(&ds_relay.dev_addr).unwrap(),
                f_cnt: 8,
                ..Default::default()
            },
            f_port: Some(lrwn::LA_FPORT_RELAY),
            frm_payload: Some(lrwn::FRMPayload::ForwardUplinkReq(lrwn::ForwardUplinkReq {
                metadata: lrwn::UplinkMetadata {
                    dr: 5,
                    snr: 10,
                    rssi: -100,
                    wor_channel: 0,
                },
                frequency: 868100000,
                payload: Box::new(phy_relay_ed_unconfirmed_up),
            })),
        }),
        mic: None,
    };
    phy_relay_unconfirmed_up
        .encrypt_frm_payload(&AES128Key::from_slice(&ds_relay.nwk_s_enc_key).unwrap())
        .unwrap();
    phy_relay_unconfirmed_up
        .set_uplink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            0,
            0,
            &AES128Key::from_slice(&ds_relay.f_nwk_s_int_key).unwrap(),
            &AES128Key::from_slice(&ds_relay.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let mut phy_relay_confirmed_up = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_slice(&ds_relay.dev_addr).unwrap(),
                f_cnt: 8,
                ..Default::default()
            },
            f_port: Some(lrwn::LA_FPORT_RELAY),
            frm_payload: Some(lrwn::FRMPayload::ForwardUplinkReq(lrwn::ForwardUplinkReq {
                metadata: lrwn::UplinkMetadata {
                    dr: 5,
                    snr: 10,
                    rssi: -100,
                    wor_channel: 0,
                },
                frequency: 868100000,
                payload: Box::new(phy_relay_ed_confirmed_up),
            })),
        }),
        mic: None,
    };
    phy_relay_confirmed_up
        .encrypt_frm_payload(&AES128Key::from_slice(&ds_relay.nwk_s_enc_key).unwrap())
        .unwrap();
    phy_relay_confirmed_up
        .set_uplink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            0,
            0,
            &AES128Key::from_slice(&ds_relay.f_nwk_s_int_key).unwrap(),
            &AES128Key::from_slice(&ds_relay.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let mut phy_relay_ed_unconfirmed_down_ack = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataDown,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_slice(&ds_relay_ed.dev_addr).unwrap(),
                f_cnt: 55,
                f_ctrl: lrwn::FCtrl {
                    ack: true,
                    adr: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            f_port: None,
            frm_payload: None,
        }),
        mic: None,
    };
    phy_relay_ed_unconfirmed_down_ack
        .set_downlink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            &AES128Key::from_slice(&ds_relay_ed.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let mut phy_relay_unconfirmed_down_ack = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataDown,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_slice(&ds_relay.dev_addr).unwrap(),
                f_cnt: 5,
                f_ctrl: lrwn::FCtrl {
                    adr: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            f_port: Some(lrwn::LA_FPORT_RELAY),
            frm_payload: Some(lrwn::FRMPayload::ForwardDownlinkReq(
                lrwn::ForwardDownlinkReq {
                    payload: Box::new(phy_relay_ed_unconfirmed_down_ack),
                },
            )),
        }),
        mic: None,
    };
    phy_relay_unconfirmed_down_ack
        .encrypt_frm_payload(&AES128Key::from_slice(&ds_relay.nwk_s_enc_key).unwrap())
        .unwrap();
    phy_relay_unconfirmed_down_ack
        .set_downlink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            &AES128Key::from_slice(&ds_relay.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let tests = vec![
        Test {
            name: "relayed unconfirmed uplink".into(),
            device_queue_items_relay_ed: vec![],
            device_session_relay: Some(ds_relay.clone()),
            device_session_relay_ed: Some(ds_relay_ed.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info.clone(),
            phy_payload: phy_relay_unconfirmed_up,
            assert: vec![
                assert::f_cnt_up(dev_relay.dev_eui, 9),
                assert::f_cnt_up(dev_relay_ed.dev_eui, 89),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp_relay.id.to_string(),
                        device_profile_name: dp_relay.name.clone(),
                        device_name: dev_relay.name.clone(),
                        dev_eui: dev_relay.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01010101".to_string(),
                    dr: 5,
                    f_cnt: 8,
                    f_port: 226,
                    data: vec![],
                    rx_info: vec![rx_info.clone()],
                    tx_info: Some(tx_info.clone()),
                    ..Default::default()
                }),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp_relay_ed.id.to_string(),
                        device_profile_name: dp_relay_ed.name.clone(),
                        device_name: dev_relay_ed.name.clone(),
                        dev_eui: dev_relay_ed.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "02020202".to_string(),
                    dr: 5,
                    f_cnt: 88,
                    f_port: 1,
                    data: vec![1, 2, 3, 4],
                    rx_info: vec![rx_info.clone()],
                    tx_info: Some(tx_info.clone()),
                    relay_rx_info: Some(integration_pb::UplinkRelayRxInfo {
                        dev_eui: "0101010101010101".into(),
                        frequency: 868100000,
                        dr: 5,
                        snr: 10,
                        rssi: -100,
                        wor_channel: 0,
                    }),
                    ..Default::default()
                }),
                assert::no_downlink_frame(),
            ],
        },
        Test {
            name: "relayed confirmed uplink".into(),
            device_queue_items_relay_ed: vec![],
            device_session_relay: Some(ds_relay.clone()),
            device_session_relay_ed: Some(ds_relay_ed.clone()),
            tx_info: tx_info.clone(),
            rx_info: rx_info.clone(),
            phy_payload: phy_relay_confirmed_up,
            assert: vec![
                assert::f_cnt_up(dev_relay.dev_eui, 9),
                assert::f_cnt_up(dev_relay_ed.dev_eui, 89),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp_relay.id.to_string(),
                        device_profile_name: dp_relay.name.clone(),
                        device_name: dev_relay.name.clone(),
                        dev_eui: dev_relay.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "01010101".to_string(),
                    dr: 5,
                    f_cnt: 8,
                    f_port: 226,
                    data: vec![],
                    rx_info: vec![rx_info.clone()],
                    tx_info: Some(tx_info.clone()),
                    ..Default::default()
                }),
                assert::uplink_event(integration_pb::UplinkEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp_relay_ed.id.to_string(),
                        device_profile_name: dp_relay_ed.name.clone(),
                        device_name: dev_relay_ed.name.clone(),
                        dev_eui: dev_relay_ed.dev_eui.to_string(),
                        ..Default::default()
                    }),
                    dev_addr: "02020202".to_string(),
                    dr: 5,
                    confirmed: true,
                    f_cnt: 88,
                    f_port: 1,
                    data: vec![1, 2, 3, 4],
                    rx_info: vec![rx_info.clone()],
                    tx_info: Some(tx_info.clone()),
                    relay_rx_info: Some(integration_pb::UplinkRelayRxInfo {
                        dev_eui: "0101010101010101".into(),
                        frequency: 868100000,
                        dr: 5,
                        snr: 10,
                        rssi: -100,
                        wor_channel: 0,
                    }),
                    ..Default::default()
                }),
                assert::downlink_frame(gw::DownlinkFrame {
                    gateway_id: gw.gateway_id.to_string(),
                    items: vec![
                        gw::DownlinkFrameItem {
                            phy_payload: phy_relay_unconfirmed_down_ack.to_vec().unwrap(),
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
                            ..Default::default()
                        },
                        gw::DownlinkFrameItem {
                            phy_payload: phy_relay_unconfirmed_down_ack.to_vec().unwrap(),
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
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
            ],
        },
    ];

    for test in &tests {
        run_test(test).await;
    }
}

async fn run_test(t: &Test) {
    println!("> {}", t.name);

    reset_redis().await.unwrap();

    integration::set_mock().await;
    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    integration::mock::reset().await;
    gateway_backend::mock::reset().await;

    if let Some(ds) = &t.device_session_relay {
        let _ = device_session::save(&ds).await.unwrap();

        let dev_eui = EUI64::from_slice(&ds.dev_eui).unwrap();
        device_queue::flush_for_dev_eui(&dev_eui).await.unwrap();
    }

    if let Some(ds) = &t.device_session_relay_ed {
        let _ = device_session::save(&ds).await.unwrap();

        let dev_eui = EUI64::from_slice(&ds.dev_eui).unwrap();
        device_queue::flush_for_dev_eui(&dev_eui).await.unwrap();
    }

    for qi in &t.device_queue_items_relay_ed {
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
