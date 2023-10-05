use httpmock::prelude::*;
use uuid::Uuid;

use super::assert;
use crate::storage::{application, device, device_profile, gateway, reset_redis, tenant};
use crate::{
    backend::joinserver, config, gateway::backend as gateway_backend, integration, region, test,
    uplink,
};
use chirpstack_api::{common, gw, integration as integration_pb, internal};
use lrwn::{DevAddr, EUI64};

struct Test {
    name: String,
    tx_info: gw::UplinkTxInfo,
    rx_info: gw::UplinkRxInfo,
    phy_payload: lrwn::PhyPayload,
    js_response: backend::JoinAnsPayload,
    assert: Vec<assert::Validator>,
}

#[tokio::test]
async fn test_js() {
    let _guard = test::prepare().await;

    let t = tenant::create(tenant::Tenant {
        name: "tenant".into(),
        can_have_gateways: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let gw = gateway::create(gateway::Gateway {
        name: "gw".into(),
        tenant_id: t.id.clone(),
        gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let dp = device_profile::create(device_profile::DeviceProfile {
        name: "dp".into(),
        tenant_id: t.id.clone(),
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_3,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
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

    let dev = device::create(device::Device {
        name: "dev".into(),
        application_id: app.id.clone(),
        device_profile_id: dp.id.clone(),
        dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let mut tx_info = gw::UplinkTxInfo {
        frequency: 868100000,
        ..Default::default()
    };
    uplink::helpers::set_uplink_modulation(&"eu868", &mut tx_info, 0).unwrap();

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

    let phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinRequest,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
            join_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dev_eui: dev.dev_eui.clone(),
            dev_nonce: 1,
        }),
        mic: Some([1, 2, 3, 4]),
    };

    let phy_ja = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinAccept,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinAccept(lrwn::JoinAcceptPayload {
            join_nonce: 1,
            home_netid: lrwn::NetID::from_be_bytes([0, 0, 0]),
            devaddr: DevAddr::from_be_bytes([1, 2, 3, 4]),
            dl_settings: lrwn::DLSettings {
                opt_neg: false,
                rx2_dr: 0,
                rx1_dr_offset: 0,
            },
            rx_delay: 1,
            cflist: None,
        }),
        mic: Some([1, 2, 3, 4]),
    };

    let tests = vec![
        Test {
            name: "test plain-text app_s_key".into(),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: phy.clone(),
            js_response: backend::JoinAnsPayload {
                base: backend::BasePayloadResult {
                    base: backend::BasePayload {
                        sender_id: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        receiver_id: vec![0, 0, 0],
                        message_type: backend::MessageType::JoinAns,
                        ..Default::default()
                    },
                    result: backend::ResultPayload {
                        result_code: backend::ResultCode::Success,
                        ..Default::default()
                    },
                },
                phy_payload: phy_ja.to_vec().unwrap(),
                app_s_key: Some(backend::KeyEnvelope {
                    kek_label: "".into(),
                    aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                }),
                nwk_s_key: Some(backend::KeyEnvelope {
                    kek_label: "".into(),
                    aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                }),
                ..Default::default()
            },
            assert: vec![
                assert::device_session(
                    dev.dev_eui.clone(),
                    internal::DeviceSession {
                        dev_addr: vec![1, 2, 3, 4],
                        dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        mac_version: common::MacVersion::Lorawan103.into(),
                        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "".into(),
                            aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
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
                assert::join_event(integration_pb::JoinEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        device_class_enabled: common::DeviceClass::ClassA.into(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    join_server_context: None,
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "test session_key_id".into(),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: phy.clone(),
            js_response: backend::JoinAnsPayload {
                base: backend::BasePayloadResult {
                    base: backend::BasePayload {
                        sender_id: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        receiver_id: vec![0, 0, 0],
                        message_type: backend::MessageType::JoinAns,
                        ..Default::default()
                    },
                    result: backend::ResultPayload {
                        result_code: backend::ResultCode::Success,
                        ..Default::default()
                    },
                },
                phy_payload: phy_ja.to_vec().unwrap(),
                session_key_id: vec![1, 2, 3, 4],
                nwk_s_key: Some(backend::KeyEnvelope {
                    kek_label: "".into(),
                    aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                }),
                ..Default::default()
            },
            assert: vec![
                assert::device_session(
                    dev.dev_eui.clone(),
                    internal::DeviceSession {
                        dev_addr: vec![1, 2, 3, 4],
                        dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        mac_version: common::MacVersion::Lorawan103.into(),
                        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        js_session_key_id: vec![1, 2, 3, 4],
                        rx1_delay: 1,
                        rx2_frequency: 869525000,
                        enabled_uplink_channel_indices: vec![0, 1, 2],
                        nb_trans: 1,
                        region_config_id: "eu868".to_string(),
                        class_b_ping_slot_nb: 1,
                        ..Default::default()
                    },
                ),
                assert::join_event(integration_pb::JoinEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        device_class_enabled: common::DeviceClass::ClassA.into(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    join_server_context: Some(integration_pb::JoinServerContext {
                        session_key_id: "01020304".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            ],
        },
        Test {
            name: "test encrypted app_s_key".into(),
            rx_info: rx_info.clone(),
            tx_info: tx_info.clone(),
            phy_payload: phy.clone(),
            js_response: backend::JoinAnsPayload {
                base: backend::BasePayloadResult {
                    base: backend::BasePayload {
                        sender_id: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        receiver_id: vec![0, 0, 0],
                        message_type: backend::MessageType::JoinAns,
                        ..Default::default()
                    },
                    result: backend::ResultPayload {
                        result_code: backend::ResultCode::Success,
                        ..Default::default()
                    },
                },
                phy_payload: phy_ja.to_vec().unwrap(),
                app_s_key: Some(backend::KeyEnvelope {
                    kek_label: "kek-label".into(),
                    aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                }),
                nwk_s_key: Some(backend::KeyEnvelope {
                    kek_label: "".into(),
                    aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                }),
                ..Default::default()
            },
            assert: vec![
                assert::device_session(
                    dev.dev_eui.clone(),
                    internal::DeviceSession {
                        dev_addr: vec![1, 2, 3, 4],
                        dev_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                        mac_version: common::MacVersion::Lorawan103.into(),
                        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "kek-label".into(),
                            aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
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
                assert::join_event(integration_pb::JoinEvent {
                    device_info: Some(integration_pb::DeviceInfo {
                        tenant_id: t.id.to_string(),
                        tenant_name: t.name.clone(),
                        application_id: app.id.to_string(),
                        application_name: app.name.clone(),
                        device_profile_id: dp.id.to_string(),
                        device_profile_name: dp.name.clone(),
                        device_name: dev.name.clone(),
                        dev_eui: dev.dev_eui.to_string(),
                        device_class_enabled: common::DeviceClass::ClassA.into(),
                        ..Default::default()
                    }),
                    dev_addr: "01020304".into(),
                    join_server_context: Some(integration_pb::JoinServerContext {
                        app_s_key: Some(common::KeyEnvelope {
                            kek_label: "kek-label".into(),
                            aes_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8],
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            ],
        },
    ];

    for tst in &tests {
        run_test(tst).await;
    }
}

async fn run_test(t: &Test) {
    println!("> {}", t.name);

    reset_redis().await.unwrap();

    let server = MockServer::start();
    let mut js_mock = server.mock(|when, then| {
        when.method(POST).path("/");

        then.body(serde_json::to_string(&t.js_response).unwrap());
    });

    let mut conf: config::Configuration = (*config::get()).clone();
    conf.join_server.servers.push(config::JoinServerServer {
        join_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
        server: server.url("/"),
        ..Default::default()
    });
    config::set(conf);
    region::setup().unwrap();
    joinserver::setup().unwrap();

    integration::set_mock().await;
    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    integration::mock::reset().await;
    gateway_backend::mock::reset().await;

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

    js_mock.assert();
    js_mock.delete();

    for assert in &t.assert {
        assert().await;
    }
}
