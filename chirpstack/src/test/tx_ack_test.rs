use std::str::FromStr;

use chrono::Duration;

use crate::downlink::tx_ack;
use crate::gpstime::ToGpsTime;
use crate::storage::{
    application, device, device_profile, device_queue, downlink_frame, fields, tenant,
};
use crate::test;

use chirpstack_api::{gw, internal};
use lrwn::{AES128Key, EUI64};

#[tokio::test]
async fn test_class_b() {
    let _guard = test::prepare().await;

    let t = tenant::create(tenant::Tenant {
        name: "tenant".into(),
        can_have_gateways: true,
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
        name: "class-b".into(),
        tenant_id: Some(t.id),
        region: lrwn::region::CommonName::EU868,
        mac_version: lrwn::region::MacVersion::LORAWAN_1_0_4,
        reg_params_revision: lrwn::region::Revision::RP002_1_0_3,
        supports_otaa: true,
        class_b_params: Some(fields::ClassBParams {
            timeout: 60,
            ping_slot_periodicity: 1,
            ping_slot_dr: 0,
            ping_slot_freq: 868100000,
            ..Default::default()
        }),
        ..Default::default()
    })
    .await
    .unwrap();

    let dev = device::create(device::Device {
        dev_eui: EUI64::from_str("0102030405060708").unwrap(),
        application_id: app.id,
        device_profile_id: dp.id,
        name: "test-dev".into(),
        enabled_class: device::DeviceClass::B,
        device_session: Some(fields::DeviceSession::new(Default::default())),
        ..Default::default()
    })
    .await
    .unwrap();

    let qi = device_queue::enqueue_item(device_queue::DeviceQueueItem {
        f_port: 10,
        data: vec![0x01, 0x02, 0x03],
        dev_eui: dev.dev_eui,
        confirmed: true,
        f_cnt_down: Some(10),
        ..Default::default()
    })
    .await
    .unwrap();

    let phy = lrwn::PhyPayload {
        mhdr: lrwn::MHDR {
            f_type: lrwn::FType::ConfirmedDataDown,
            major: lrwn::Major::LoRaWANR1,
        },
        payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
            fhdr: lrwn::FHDR {
                ..Default::default()
            },
            f_port: Some(10),
            ..Default::default()
        }),
        mic: Some([1, 2, 3, 4]),
    };

    let _ = downlink_frame::save(&internal::DownlinkFrame {
        downlink_id: 1234,
        dev_eui: dev.dev_eui.to_vec(),
        device_queue_item_id: qi.id.as_bytes().to_vec(),
        nwk_s_enc_key: AES128Key::null().to_vec(),
        downlink_frame: Some(gw::DownlinkFrame {
            downlink_id: 1234,
            gateway_id: "0807060504030201".into(),
            items: vec![gw::DownlinkFrameItem {
                phy_payload: phy.to_vec().unwrap(),
                tx_info: Some(gw::DownlinkTxInfo {
                    timing: Some(gw::Timing {
                        parameters: Some(gw::timing::Parameters::GpsEpoch(
                            gw::GpsEpochTimingInfo {
                                time_since_gps_epoch: Some(pbjson_types::Duration {
                                    seconds: 120,
                                    nanos: 0,
                                }),
                            },
                        )),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    })
    .await
    .unwrap();

    tx_ack::TxAck::handle(gw::DownlinkTxAck {
        downlink_id: 1234,
        items: vec![gw::DownlinkTxAckItem {
            status: gw::TxAckStatus::Ok.into(),
        }],
        ..Default::default()
    })
    .await;

    // validate that the queue-item has an expires at that is 60 seconds after the ping-lsot
    let qi = device_queue::get_item(&qi.id).await.unwrap();
    assert!(qi.timeout_after.is_some());

    let gps_ts = qi.timeout_after.unwrap().to_gps_time();
    assert_eq!(Duration::seconds(120 + 60), gps_ts);
}
