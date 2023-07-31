use std::time::Duration;

use uuid::Uuid;

use super::assert;
use crate::storage::{
    application,
    device::{self, DeviceClass},
    device_keys, device_profile, device_session, gateway, tenant,
};
use crate::{gateway::backend as gateway_backend, integration, test, uplink};
use chirpstack_api::{common, gw, internal};
use lrwn::{AES128Key, EUI64};

#[tokio::test]
async fn test_lorawan_10() {
    let _guard = test::prepare().await;
    integration::set_mock().await;
    gateway_backend::set_backend(&"eu868", Box::new(gateway_backend::mock::Backend {})).await;

    integration::mock::reset().await;
    gateway_backend::mock::reset().await;

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

    let dp_relay = device_profile::create(device_profile::DeviceProfile {
        name: "dp".into(),
        tenant_id: t.id.clone(),
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_2,
        reg_params_revision: lrwn::region::Revision::A,
        supports_otaa: true,
        is_relay: true,
        ..Default::default()
    })
    .await
    .unwrap();

    let dev = device::create(device::Device {
        name: "device".into(),
        application_id: app.id.clone(),
        device_profile_id: dp.id.clone(),
        dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 1]),
        enabled_class: DeviceClass::A,
        ..Default::default()
    })
    .await
    .unwrap();

    let dk_dev = device_keys::create(device_keys::DeviceKeys {
        dev_eui: dev.dev_eui.clone(),
        nwk_key: AES128Key::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]),
        ..Default::default()
    })
    .await
    .unwrap();

    let dev_relay = device::create(device::Device {
        name: "relay-device".into(),
        application_id: app.id.clone(),
        device_profile_id: dp_relay.id.clone(),
        dev_eui: EUI64::from_be_bytes([1, 1, 1, 1, 1, 1, 1, 2]),
        enabled_class: DeviceClass::A,
        ..Default::default()
    })
    .await
    .unwrap();

    let ds_relay = internal::DeviceSession {
        dev_eui: dev_relay.dev_eui.to_vec(),
        mac_version: common::MacVersion::Lorawan102.into(),
        dev_addr: vec![4, 3, 2, 1],
        f_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        s_nwk_s_int_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        nwk_s_enc_key: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        f_cnt_up: 10,
        n_f_cnt_down: 5,
        rx1_delay: 1,
        rx2_frequency: 869525000,
        region_config_id: "eu868".into(),
        ..Default::default()
    };
    device_session::save(&ds_relay).await.unwrap();

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
            dev_nonce: 1,
        }),
        mic: None,
    };
    jr_pl.set_join_request_mic(&dk_dev.nwk_key).unwrap();

    let mut ja_pl = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::JoinAccept,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::JoinAccept(lrwn::JoinAcceptPayload {
            home_netid: lrwn::NetID::from_be_bytes([0, 0, 0]),
            devaddr: lrwn::DevAddr::from_be_bytes([1, 2, 3, 4]),
            dl_settings: lrwn::DLSettings {
                rx2_dr: 0,
                rx1_dr_offset: 0,
                opt_neg: false,
            },
            rx_delay: 1,
            join_nonce: 0,
            cflist: None,
        }),
        mic: None,
    };
    ja_pl
        .set_join_accept_mic(
            lrwn::JoinType::Join,
            &EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            1,
            &dk_dev.nwk_key,
        )
        .unwrap();
    ja_pl.encrypt_join_accept_payload(&dk_dev.nwk_key).unwrap();

    let mut phy_relay_jr = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataUp,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_be_bytes([4, 3, 2, 1]),
                f_cnt: 10,
                ..Default::default()
            },
            f_port: Some(226),
            frm_payload: Some(lrwn::FRMPayload::ForwardUplinkReq(lrwn::ForwardUplinkReq {
                metadata: lrwn::UplinkMetadata {
                    dr: 5,
                    snr: 10,
                    rssi: -120,
                    wor_channel: 0,
                },
                frequency: 868100000,
                payload: Box::new(jr_pl),
            })),
        }),
        mic: None,
    };
    phy_relay_jr
        .encrypt_frm_payload(&AES128Key::from_slice(&ds_relay.nwk_s_enc_key).unwrap())
        .unwrap();
    phy_relay_jr
        .set_uplink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            0,
            0,
            0,
            &AES128Key::from_slice(&ds_relay.f_nwk_s_int_key).unwrap(),
            &AES128Key::from_slice(&ds_relay.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    uplink::handle_uplink(
        Uuid::new_v4(),
        gw::UplinkFrameSet {
            phy_payload: phy_relay_jr.to_vec().unwrap(),
            tx_info: Some(tx_info),
            rx_info: vec![rx_info],
        },
    )
    .await
    .unwrap();

    let mut phy_relay_ja = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            m_type: lrwn::MType::UnconfirmedDataDown,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                devaddr: lrwn::DevAddr::from_be_bytes([4, 3, 2, 1]),
                f_cnt: 5,
                f_ctrl: lrwn::FCtrl {
                    adr: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            f_port: Some(226),
            frm_payload: Some(lrwn::FRMPayload::ForwardDownlinkReq(
                lrwn::ForwardDownlinkReq {
                    payload: Box::new(ja_pl),
                },
            )),
        }),
        mic: None,
    };
    phy_relay_ja
        .encrypt_frm_payload(&AES128Key::from_slice(&ds_relay.nwk_s_enc_key).unwrap())
        .unwrap();
    phy_relay_ja
        .set_downlink_data_mic(
            lrwn::MACVersion::LoRaWAN1_0,
            10,
            &AES128Key::from_slice(&ds_relay.s_nwk_s_int_key).unwrap(),
        )
        .unwrap();

    let assertions = vec![
        assert::device_session(
            dev.dev_eui.clone(),
            internal::DeviceSession {
                dev_addr: vec![1, 2, 3, 4],
                dev_eui: vec![1, 1, 1, 1, 1, 1, 1, 1],
                join_eui: vec![1, 2, 3, 4, 5, 6, 7, 8],
                mac_version: common::MacVersion::Lorawan102.into(),
                f_nwk_s_int_key: vec![
                    146, 184, 94, 251, 180, 89, 48, 96, 236, 112, 106, 181, 94, 25, 215, 162,
                ],
                s_nwk_s_int_key: vec![
                    146, 184, 94, 251, 180, 89, 48, 96, 236, 112, 106, 181, 94, 25, 215, 162,
                ],
                nwk_s_enc_key: vec![
                    146, 184, 94, 251, 180, 89, 48, 96, 236, 112, 106, 181, 94, 25, 215, 162,
                ],
                app_s_key: Some(common::KeyEnvelope {
                    kek_label: "".to_string(),
                    aes_key: vec![
                        181, 55, 181, 113, 220, 19, 233, 58, 156, 54, 209, 48, 209, 201, 73, 33,
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
                    phy_payload: phy_relay_ja.to_vec().unwrap(),
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
                },
                gw::DownlinkFrameItem {
                    phy_payload: phy_relay_ja.to_vec().unwrap(),
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
                },
            ],
            gateway_id: "0102030405060708".to_string(),
            ..Default::default()
        }),
    ];

    for assert in &assertions {
        assert().await;
    }
}
