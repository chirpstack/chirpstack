use anyhow::Result;
use chrono::{DateTime, Utc};
use tracing::{info, warn};

use crate::api::helpers::ToProto;
use crate::integration;
use crate::storage::{application, device, device_profile, tenant};
use crate::uplink::{UplinkFrameSet, helpers};
use crate::config;
use chirpstack_api::gw;
use chirpstack_api::integration as integration_pb;

pub async fn handle(
    ufs: &UplinkFrameSet,
    tenant: &tenant::Tenant,
    app: &application::Application,
    dp: &device_profile::DeviceProfile,
    dev: &device::Device,
    block: &lrwn::MACCommandSet,
) -> Result<Option<lrwn::MACCommandSet>> {
    let _ = (**block)
        .first()
        .ok_or_else(|| anyhow!("Expected LinkCheckReq"));

    info!(dev_eui = %dev.dev_eui, "Received LinkCheckReq");

    let mod_info = ufs
        .tx_info
        .modulation
        .as_ref()
        .ok_or_else(|| anyhow!("modulation can not be None"))?;
    let mod_params = mod_info
        .parameters
        .as_ref()
        .ok_or_else(|| anyhow!("parameters can not be None"))?;

    let required_snr: Option<f32> = match mod_params {
        gw::modulation::Parameters::Lora(pl) => {
            Some(config::get_required_snr_for_sf(pl.spreading_factor as u8)?)
        }
        _ => {
            warn!("Modulation does not provide margin to LinkCheckReq");
            None
        }
    };

    let gw_rx_info: Vec<integration_pb::LinkCheckGwRxInfo> = ufs
        .rx_info_set
        .iter()
        .map(|rx| {
            let gw_margin = required_snr
                .map(|snr| {
                    let m = rx.snr - snr;
                    if m < 0.0 { 0.0 } else { m }
                })
                .unwrap_or(0.0);
            integration_pb::LinkCheckGwRxInfo {
                gateway_id: rx.gateway_id.clone(),
                rssi: rx.rssi,
                snr: rx.snr,
                margin: gw_margin as u32,
            }
        })
        .collect();

    let best_margin = gw_rx_info.iter().map(|g| g.margin).max().unwrap_or(0);

    let mut tags = (*app.tags).clone();
    tags.extend((*dp.tags).clone());
    tags.extend((*dev.tags).clone());

    let rx_time: DateTime<Utc> = helpers::get_rx_timestamp(&ufs.rx_info_set).into();

    integration::link_check_event(
        app.id.into(),
        &dev.variables,
        &integration_pb::LinkCheckEvent {
            deduplication_id: ufs.uplink_set_id.to_string(),
            time: Some(rx_time.into()),
            device_info: Some(integration_pb::DeviceInfo {
                tenant_id: tenant.id.to_string(),
                tenant_name: tenant.name.clone(),
                application_id: app.id.to_string(),
                application_name: app.name.to_string(),
                device_profile_id: dp.id.to_string(),
                device_profile_name: dp.name.clone(),
                device_name: dev.name.clone(),
                device_class_enabled: dev.enabled_class.to_proto().into(),
                dev_eui: dev.dev_eui.to_string(),
                tags,
            }),
            margin: best_margin,
            gw_cnt: ufs.rx_info_set.len() as u32,
            gw_rx_info,
        },
    )
    .await;

    Ok(Some(lrwn::MACCommandSet::new(vec![
        lrwn::MACCommand::LinkCheckAns(lrwn::LinkCheckAnsPayload {
            margin: best_margin as u8,
            gw_cnt: ufs.rx_info_set.len() as u8,
        }),
    ])))
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::integration::mock;
    use crate::{integration, test};
    use chirpstack_api::gw;
    use lrwn::EUI64;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::time::Duration;
    use tokio::time::sleep;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_handle() {
        let _guard = test::prepare().await;
        integration::set_mock().await;

        let ufs = UplinkFrameSet {
            uplink_set_id: Uuid::new_v4(),
            dr: 0,
            ch: 0,
            phy_payload: lrwn::PhyPayload {
                mhdr: lrwn::MHDR {
                    f_type: lrwn::FType::UnconfirmedDataUp,
                    major: lrwn::Major::LoRaWANR1,
                },
                payload: lrwn::Payload::MACPayload(lrwn::MACPayload {
                    fhdr: Default::default(),
                    f_port: None,
                    frm_payload: None,
                }),
                mic: None,
            },
            tx_info: gw::UplinkTxInfo {
                modulation: Some(gw::Modulation {
                    parameters: Some(gw::modulation::Parameters::Lora(gw::LoraModulationInfo {
                        spreading_factor: 10,
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            },
            rx_info_set: vec![
                gw::UplinkRxInfo {
                    gateway_id: "0101010101010101".into(),
                    snr: -2.0,
                    rssi: -110,
                    ..Default::default()
                },
                gw::UplinkRxInfo {
                    gateway_id: "0202020202020202".into(),
                    snr: 2.0,
                    rssi: -100,
                    ..Default::default()
                },
            ],
            gateway_private_up_map: HashMap::new(),
            gateway_private_down_map: HashMap::new(),
            gateway_tenant_id_map: HashMap::new(),
            region_common_name: lrwn::region::CommonName::EU868,
            region_config_id: "eu868".into(),
            roaming_meta_data: None,
        };

        let tenant = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let app = application::create(application::Application {
            tenant_id: tenant.id,
            name: "test-app".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: Some(tenant.id),
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();
        let dev = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp.id,
            dev_eui: EUI64::from_str("0102030405060708").unwrap(),
            name: "test-device".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let block = lrwn::MACCommandSet::new(vec![lrwn::MACCommand::LinkCheckReq]);

        let resp = handle(&ufs, &tenant, &app, &dp, &dev, &block)
            .await
            .unwrap();

        // required_snr for SF10 = -15 dB
        // gw1: snr -2.0 - (-15.0) = 13, gw2: snr 2.0 - (-15.0) = 17 → best = 17
        assert_eq!(
            Some(lrwn::MACCommandSet::new(vec![
                lrwn::MACCommand::LinkCheckAns(lrwn::LinkCheckAnsPayload {
                    margin: 17,
                    gw_cnt: 2,
                })
            ])),
            resp
        );

        // Integration events are handled async.
        sleep(Duration::from_millis(100)).await;

        let events = mock::get_link_check_events().await;
        assert_eq!(1, events.len());
        let ev = &events[0];
        assert_eq!(ufs.uplink_set_id.to_string(), ev.deduplication_id);
        assert_eq!(17, ev.margin);
        assert_eq!(2, ev.gw_cnt);
        assert_eq!(2, ev.gw_rx_info.len());

        let gw1 = ev.gw_rx_info.iter().find(|g| g.gateway_id == "0101010101010101").unwrap();
        assert_eq!(-110, gw1.rssi);
        assert_eq!(-2.0, gw1.snr);
        assert_eq!(13, gw1.margin);

        let gw2 = ev.gw_rx_info.iter().find(|g| g.gateway_id == "0202020202020202").unwrap();
        assert_eq!(-100, gw2.rssi);
        assert_eq!(2.0, gw2.snr);
        assert_eq!(17, gw2.margin);
    }
}
