use std::str::FromStr;

use bytes::Bytes;
use chrono::Utc;
use httpmock::prelude::*;
use prost::Message;
use uuid::Uuid;

use super::assert;
use crate::api::backend as backend_api;
use crate::backend::{joinserver, roaming};
use crate::gateway::backend as gateway_backend;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_keys, device_profile, gateway, tenant,
};
use crate::{config, test, uplink};
use chirpstack_api::gw;
use lrwn::{AES128Key, NetID, EUI64};

#[tokio::test]
async fn test_fns() {
    let _guard = test::prepare().await;

    let js_mock = MockServer::start();
    let sns_mock = MockServer::start();

    let mut conf = (*config::get()).clone();

    // Set NetID.
    conf.network.net_id = NetID::from_str("010203").unwrap();

    // Set Join Server.
    conf.join_server.servers.push(config::JoinServerServer {
        join_eui: EUI64::from_str("0102030405060708").unwrap(),
        server: js_mock.url("/"),
        ..Default::default()
    });

    // Set roaming agreement.
    conf.roaming.servers.push(config::RoamingServer {
        net_id: NetID::from_str("030201").unwrap(),
        server: sns_mock.url("/"),
        ..Default::default()
    });

    config::set(conf);
    joinserver::setup().unwrap();
    roaming::setup().unwrap();

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
        gateway_id: EUI64::from_str("0102030405060708").unwrap(),
        ..Default::default()
    })
    .await
    .unwrap();

    let recv_time = Utc::now();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: gw.gateway_id.to_string(),
        time: Some(recv_time.into()),
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
    uplink::helpers::set_uplink_modulation("eu868", &mut tx_info, 0).unwrap();

    let mut jr_phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinRequest,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
            join_eui: EUI64::from_str("0102030405060708").unwrap(),
            dev_eui: EUI64::from_str("0807060504030201").unwrap(),
            dev_nonce: 123,
        }),
        mic: None,
    };
    jr_phy
        .set_join_request_mic(&AES128Key::from_str("01020304050607080102030405060708").unwrap())
        .unwrap();

    // Setup JS mock (HomeNSReq).
    let mut js_join_request_mock = js_mock.mock(|when, then| {
        when.method(POST)
            .path("/")
            .json_body_obj(&backend::HomeNSReqPayload {
                base: backend::BasePayload {
                    sender_id: vec![1, 2, 3],
                    receiver_id: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    message_type: backend::MessageType::HomeNSReq,
                    transaction_id: 1234,
                    ..Default::default()
                },
                dev_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
            });

        then.json_body_obj(&backend::HomeNSAnsPayload {
            base: backend::BasePayloadResult {
                base: backend::BasePayload {
                    receiver_id: vec![1, 2, 3],
                    sender_id: vec![1, 2, 3, 4, 5, 6, 7, 8],
                    message_type: backend::MessageType::HomeNSAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: backend::ResultPayload {
                    result_code: backend::ResultCode::Success,
                    ..Default::default()
                },
            },
            h_net_id: vec![3, 2, 1],
        })
        .status(200);
    });

    // Setup SNS mock (PRStartReq).
    let mut sns_pr_start_req_mock = sns_mock.mock(|when, then| {
        when.method(POST)
            .path("/")
            .json_body_obj(&backend::PRStartReqPayload {
                base: backend::BasePayload {
                    sender_id: vec![1, 2, 3],
                    receiver_id: vec![3, 2, 1],
                    message_type: backend::MessageType::PRStartReq,
                    transaction_id: 1234,
                    ..Default::default()
                },
                phy_payload: jr_phy.to_vec().unwrap(),
                ul_meta_data: backend::ULMetaData {
                    dev_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
                    ul_freq: Some(868.1),
                    data_rate: Some(0),
                    recv_time: recv_time,
                    rf_region: "EU868".to_string(),
                    gw_cnt: Some(1),
                    gw_info: roaming::rx_info_to_gw_info(&[rx_info.clone()]).unwrap(),
                    ..Default::default()
                },
            });

        then.json_body_obj(&backend::PRStartAnsPayload {
            base: backend::BasePayloadResult {
                base: backend::BasePayload {
                    receiver_id: vec![1, 2, 3],
                    sender_id: vec![3, 2, 1],
                    message_type: backend::MessageType::PRStartAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: backend::ResultPayload {
                    result_code: backend::ResultCode::Success,
                    ..Default::default()
                },
            },
            phy_payload: vec![1, 2, 3, 4],
            dl_meta_data: Some(backend::DLMetaData {
                class_mode: Some("A".to_string()),
                dl_freq_1: Some(868.1),
                data_rate_1: Some(0),
                rx_delay_1: Some(5),
                ..Default::default()
            }),
            ..Default::default()
        })
        .status(200);
    });

    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;
    gateway_backend::mock::reset().await;

    // Simulate uplink
    uplink::handle_uplink(
        Uuid::new_v4(),
        gw::UplinkFrameSet {
            phy_payload: jr_phy.to_vec().unwrap(),
            tx_info: Some(tx_info),
            rx_info: vec![rx_info],
        },
    )
    .await
    .unwrap();

    js_join_request_mock.assert();
    js_join_request_mock.delete();

    sns_pr_start_req_mock.assert();
    sns_pr_start_req_mock.delete();

    assert::downlink_frame(gw::DownlinkFrame {
        gateway_id: "0102030405060708".into(),
        items: vec![gw::DownlinkFrameItem {
            phy_payload: vec![1, 2, 3, 4],
            tx_info: Some(gw::DownlinkTxInfo {
                frequency: 868100000,
                power: 14,
                modulation: Some(gw::Modulation {
                    parameters: Some(gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                        bandwidth: 125000,
                        spreading_factor: 12,
                        code_rate: gw::CodeRate::Cr45.into(),
                        polarization_inversion: true,
                        code_rate_legacy: "".to_string(),
                    })),
                }),
                board: 0,
                antenna: 0,
                timing: Some(gw::Timing {
                    parameters: Some(gw::timing::Parameters::Delay(gw::DelayTimingInfo {
                        delay: Some(pbjson_types::Duration {
                            seconds: 5,
                            nanos: 0,
                        }),
                    })),
                }),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    })()
    .await;

    joinserver::reset();
    roaming::reset();
}

#[tokio::test]
async fn test_sns() {
    let _guard = test::prepare().await;

    let fns_mock = MockServer::start();

    let mut conf = (*config::get()).clone();

    // Set NetID.
    conf.network.net_id = NetID::from_str("010203").unwrap();

    // Set roaming agreement.
    conf.roaming.servers.push(config::RoamingServer {
        net_id: NetID::from_str("030201").unwrap(),
        server: fns_mock.url("/"),
        ..Default::default()
    });

    config::set(conf);
    joinserver::setup().unwrap();
    roaming::setup().unwrap();

    let t = tenant::create(tenant::Tenant {
        name: "tenant".into(),
        can_have_gateways: true,
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
        dev_nonces: vec![],
        ..Default::default()
    })
    .await
    .unwrap();

    let mut jr_phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinRequest,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinRequest(lrwn::JoinRequestPayload {
            join_eui: EUI64::from_str("0000000000000000").unwrap(),
            dev_eui: dev.dev_eui,
            dev_nonce: 1,
        }),
        mic: None,
    };
    jr_phy.set_join_request_mic(&dk.nwk_key).unwrap();

    let recv_time = Utc::now();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: "0302030405060708".to_string(),
        time: Some(recv_time.into()),
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
    uplink::helpers::set_uplink_modulation("eu868", &mut tx_info, 0).unwrap();

    let pr_start_req = backend::PRStartReqPayload {
        base: backend::BasePayload {
            sender_id: vec![3, 2, 1],
            receiver_id: vec![1, 2, 3],
            message_type: backend::MessageType::PRStartReq,
            transaction_id: 1234,
            ..Default::default()
        },
        phy_payload: jr_phy.to_vec().unwrap(),
        ul_meta_data: backend::ULMetaData {
            dev_eui: dev.dev_eui.to_vec(),
            ul_freq: Some(868.1),
            data_rate: Some(0),
            recv_time: recv_time,
            rf_region: "EU868".to_string(),
            gw_cnt: Some(1),
            gw_info: roaming::rx_info_to_gw_info(&[rx_info.clone()]).unwrap(),
            ..Default::default()
        },
    };

    let resp =
        backend_api::handle_request(Bytes::from(serde_json::to_string(&pr_start_req).unwrap()))
            .await;
    let resp_b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let pr_start_ans: backend::PRStartAnsPayload = serde_json::from_slice(&resp_b).unwrap();

    assert_eq!(
        backend::PRStartAnsPayload {
            base: backend::BasePayloadResult {
                base: backend::BasePayload {
                    sender_id: vec![1, 2, 3],
                    receiver_id: vec![3, 2, 1],
                    message_type: backend::MessageType::PRStartAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: backend::ResultPayload {
                    result_code: backend::ResultCode::Success,
                    ..Default::default()
                },
                ..Default::default()
            },
            phy_payload: vec![
                32, 62, 206, 177, 148, 31, 33, 193, 200, 4, 185, 248, 156, 108, 64, 97, 1
            ],
            dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
            nwk_s_key: Some(backend::KeyEnvelope {
                kek_label: "".to_string(),
                aes_key: vec![
                    136, 91, 1, 94, 61, 245, 54, 151, 185, 147, 143, 76, 248, 79, 192, 28
                ],
            }),
            f_cnt_up: Some(0),
            dl_meta_data: Some(backend::DLMetaData {
                dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
                dl_freq_1: Some(868.1),
                dl_freq_2: Some(869.525),
                rx_delay_1: Some(5),
                class_mode: Some("A".to_string()),
                data_rate_1: Some(0),
                data_rate_2: Some(0),
                gw_info: vec![backend::GWInfoElement {
                    ul_token: rx_info.encode_to_vec(),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            dev_addr: vec![7, 2, 3, 4],
            ..Default::default()
        },
        pr_start_ans
    );

    joinserver::reset();
    roaming::reset();
}
