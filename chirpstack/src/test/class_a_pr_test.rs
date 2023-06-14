use std::str::FromStr;

use bytes::Bytes;
use chrono::Utc;
use httpmock::prelude::*;
use prost::Message;
use uuid::Uuid;

use crate::api::backend as backend_api;
use crate::backend::{joinserver, roaming};
use crate::gateway::backend as gateway_backend;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_profile, device_queue, device_session, gateway, tenant,
};
use crate::{config, test, uplink};
use chirpstack_api::{common, gw, internal};
use lrwn::{AES128Key, NetID, EUI64};

#[tokio::test]
async fn test_fns_uplink() {
    let _guard = test::prepare().await;

    let sns_mock = MockServer::start();

    let mut conf = (*config::get()).clone();

    // Set NetID.
    conf.network.net_id = NetID::from_str("000202").unwrap();

    // Set roaming agreement.
    conf.roaming.servers.push(config::RoamingServer {
        net_id: NetID::from_str("000505").unwrap(),
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
        location: Some(common::Location {
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
            ..Default::default()
        }),
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

    let data_phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: {
                    let mut d = lrwn::DevAddr::from_be_bytes([0, 0, 0, 0]);
                    d.set_dev_addr_prefix(
                        lrwn::NetID::from_str("000505").unwrap().dev_addr_prefix(),
                    );
                    d
                },
                f_ctrl: Default::default(),
                f_cnt: 1,
                f_opts: lrwn::MACCommandSet::new(vec![]),
            },
            f_port: None,
            frm_payload: None,
        }),
        mic: Some([1, 2, 3, 4]),
    };

    // Setup sns mock.
    let mut sns_pr_start_req_mock = sns_mock.mock(|when, then| {
        when.method(POST)
            .path("/")
            .json_body_obj(&backend::PRStartReqPayload {
                base: backend::BasePayload {
                    sender_id: vec![0, 2, 2],
                    receiver_id: vec![0, 5, 5],
                    message_type: backend::MessageType::PRStartReq,
                    transaction_id: 1234,
                    ..Default::default()
                },
                phy_payload: data_phy.to_vec().unwrap(),
                ul_meta_data: backend::ULMetaData {
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
            ..Default::default()
        })
        .status(200);
    });

    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    // Simulate uplink
    uplink::handle_uplink(
        Uuid::new_v4(),
        gw::UplinkFrameSet {
            phy_payload: data_phy.to_vec().unwrap(),
            tx_info: Some(tx_info),
            rx_info: vec![rx_info],
        },
    )
    .await
    .unwrap();

    sns_pr_start_req_mock.assert();
    sns_pr_start_req_mock.delete();

    joinserver::reset();
    roaming::reset();
}

#[tokio::test]
async fn test_sns_uplink() {
    let _guard = test::prepare().await;
    let fns_mock = MockServer::start();
    let mut conf = (*config::get()).clone();

    // Set NetID.
    conf.network.net_id = NetID::from_str("000505").unwrap();

    // Set roaming agreement.
    conf.roaming.servers.push(config::RoamingServer {
        net_id: NetID::from_str("000202").unwrap(),
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

    device_queue::enqueue_item(device_queue::DeviceQueueItem {
        dev_eui: dev.dev_eui,
        f_port: 10,
        data: vec![1, 2, 3, 4],
        ..Default::default()
    })
    .await
    .unwrap();

    let mut dev_addr = lrwn::DevAddr::from_be_bytes([0, 0, 0, 0]);
    dev_addr.set_dev_addr_prefix(lrwn::NetID::from_str("000505").unwrap().dev_addr_prefix());

    let ds = internal::DeviceSession {
        dev_eui: vec![2, 2, 3, 4, 5, 6, 7, 8],
        mac_version: common::MacVersion::Lorawan104.into(),
        join_eui: vec![8, 7, 6, 5, 4, 3, 2, 1],
        dev_addr: dev_addr.to_vec(),
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
    device_session::save(&ds).await.unwrap();

    let mut data_phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: dev_addr,
                f_ctrl: Default::default(),
                f_cnt: 8,
                f_opts: lrwn::MACCommandSet::new(vec![]),
            },
            f_port: None,
            frm_payload: None,
        }),
        mic: None,
    };
    data_phy
        .set_uplink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            0,
            0,
            &AES128Key::from_slice(&ds.f_nwk_s_int_key).unwrap(),
            &AES128Key::from_slice(&ds.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let recv_time = Utc::now();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: "0302030405060708".to_string(),
        time: Some(recv_time.into()),
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
            sender_id: vec![0, 2, 2],
            receiver_id: vec![0, 5, 5],
            message_type: backend::MessageType::PRStartReq,
            transaction_id: 1234,
            ..Default::default()
        },
        phy_payload: data_phy.to_vec().unwrap(),
        ul_meta_data: backend::ULMetaData {
            ul_freq: Some(868.1),
            data_rate: Some(0),
            recv_time: recv_time,
            rf_region: "EU868".to_string(),
            gw_cnt: Some(1),
            gw_info: roaming::rx_info_to_gw_info(&[rx_info.clone()]).unwrap(),
            ..Default::default()
        },
    };

    // Setup downlink xmit mock.
    let mut fns_xmit_data_req_mock = fns_mock.mock(|when, then| {
        when.method(POST)
            .path("/")
            .json_body_obj(&backend::XmitDataReqPayload {
                base: backend::BasePayload {
                    receiver_id: vec![0, 2, 2],
                    sender_id: vec![0, 5, 5],
                    message_type: backend::MessageType::XmitDataReq,
                    transaction_id: 1234,
                    ..Default::default()
                },
                phy_payload: hex::decode("600000000a8005000a54972baa8b983cd1").unwrap(),
                dl_meta_data: Some(backend::DLMetaData {
                    dev_eui: ds.dev_eui.clone(),
                    dl_freq_1: Some(868.1),
                    dl_freq_2: Some(869.525),
                    rx_delay_1: Some(1),
                    class_mode: Some("A".to_string()),
                    data_rate_1: Some(0),
                    data_rate_2: Some(0),
                    gw_info: vec![backend::GWInfoElement {
                        ul_token: rx_info.encode_to_vec(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            });

        then.json_body_obj(&backend::XmitDataAnsPayload {
            base: backend::BasePayloadResult {
                base: backend::BasePayload {
                    receiver_id: vec![0, 5, 5],
                    sender_id: vec![0, 2, 2],
                    message_type: backend::MessageType::XmitDataAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: backend::ResultPayload {
                    result_code: backend::ResultCode::Success,
                    ..Default::default()
                },
            },
        })
        .status(200);
    });

    let resp =
        backend_api::handle_request(Bytes::from(serde_json::to_string(&pr_start_req).unwrap()))
            .await;
    let resp_b = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let pr_start_ans: backend::PRStartAnsPayload = serde_json::from_slice(&resp_b).unwrap();

    assert_eq!(
        backend::PRStartAnsPayload {
            base: backend::BasePayloadResult {
                base: backend::BasePayload {
                    sender_id: vec![0, 5, 5],
                    receiver_id: vec![0, 2, 2],
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
            dev_eui: ds.dev_eui.clone(),
            nwk_s_key: Some(backend::KeyEnvelope {
                kek_label: "".to_string(),
                aes_key: ds.nwk_s_enc_key.clone(),
            }),
            f_cnt_up: Some(8),
            ..Default::default()
        },
        pr_start_ans
    );

    fns_xmit_data_req_mock.assert();
    fns_xmit_data_req_mock.delete();
}

#[tokio::test]
async fn test_sns_dev_not_found() {
    let _guard = test::prepare().await;
    let fns_mock = MockServer::start();

    let mut conf = (*config::get()).clone();

    // Set NetID.
    conf.network.net_id = NetID::from_str("000505").unwrap();

    // Set roaming agreement.
    conf.roaming.servers.push(config::RoamingServer {
        net_id: NetID::from_str("000202").unwrap(),
        server: fns_mock.url("/"),
        ..Default::default()
    });

    config::set(conf);
    joinserver::setup().unwrap();
    roaming::setup().unwrap();

    let mut dev_addr = lrwn::DevAddr::from_be_bytes([0, 0, 0, 0]);
    dev_addr.set_dev_addr_prefix(lrwn::NetID::from_str("000505").unwrap().dev_addr_prefix());

    let data_phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: dev_addr,
                f_ctrl: Default::default(),
                f_cnt: 8,
                f_opts: lrwn::MACCommandSet::new(vec![]),
            },
            f_port: None,
            frm_payload: None,
        }),
        mic: Some([1, 2, 3, 4]),
    };

    let recv_time = Utc::now();

    let mut rx_info = gw::UplinkRxInfo {
        gateway_id: "0302030405060708".to_string(),
        time: Some(recv_time.into()),
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
            sender_id: vec![0, 2, 2],
            receiver_id: vec![0, 5, 5],
            message_type: backend::MessageType::PRStartReq,
            transaction_id: 1234,
            ..Default::default()
        },
        phy_payload: data_phy.to_vec().unwrap(),
        ul_meta_data: backend::ULMetaData {
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
                    sender_id: vec![0, 5, 5],
                    receiver_id: vec![0, 2, 2],
                    message_type: backend::MessageType::PRStartAns,
                    transaction_id: 1234,
                    ..Default::default()
                },
                result: backend::ResultPayload {
                    result_code: backend::ResultCode::UnknownDevAddr,
                    description: format!("Object does not exist (id: {})", dev_addr),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        pr_start_ans
    );
}
