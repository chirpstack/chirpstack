use std::future::Future;
use std::io::Cursor;
use std::pin::Pin;
use std::time::Duration;

use prost::Message;
use redis::streams::StreamReadReply;
use tokio::sync::RwLock;
use tokio::time::sleep;

use crate::gateway::backend::mock as gateway_mock;
use crate::integration::mock;
use crate::storage::{
    device::{self, DeviceClass},
    device_queue, device_session, downlink_frame, get_redis_conn, redis_key,
};
use chirpstack_api::{api, gw, integration as integration_pb, internal, meta};
use lrwn::EUI64;

lazy_static! {
    static ref LAST_DOWNLINK_ID: RwLock<u32> = RwLock::new(0);
}

pub type Validator = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>;

pub fn f_cnt_up(dev_eui: EUI64, f_cnt: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(f_cnt, ds.f_cnt_up);
        })
    })
}

pub fn n_f_cnt_down(dev_eui: EUI64, f_cnt: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(f_cnt, ds.n_f_cnt_down);
        })
    })
}

pub fn a_f_cnt_down(dev_eui: EUI64, f_cnt: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(f_cnt, ds.a_f_cnt_down);
        })
    })
}

pub fn tx_power_index(dev_eui: EUI64, tx_power: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(tx_power, ds.tx_power_index);
        })
    })
}

pub fn nb_trans(dev_eui: EUI64, nb_trans: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(nb_trans, ds.nb_trans);
        })
    })
}

pub fn enabled_uplink_channel_indices(dev_eui: EUI64, channels: Vec<u32>) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        let channels = channels.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(channels, ds.enabled_uplink_channel_indices);
        })
    })
}

pub fn dr(dev_eui: EUI64, dr: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(dr, ds.dr);
        })
    })
}

pub fn mac_command_error_count(dev_eui: EUI64, cid: lrwn::CID, count: u32) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(
                count,
                ds.mac_command_error_count
                    .get(&(cid.to_u8() as u32))
                    .cloned()
                    .unwrap_or(0)
            )
        })
    })
}

pub fn uplink_adr_history(dev_eui: EUI64, uh: Vec<internal::UplinkAdrHistory>) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        let uh = uh.clone();
        Box::pin(async move {
            let ds = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(uh, ds.uplink_adr_history);
        })
    })
}

pub fn integration_log(logs: Vec<String>) -> Validator {
    Box::new(move || {
        let logs = logs.clone();
        Box::pin(async move {
            // Integration events are handled async.
            sleep(Duration::from_millis(100)).await;

            let mock_logs = mock::get_log_events().await;
            assert_eq!(logs.len(), mock_logs.len());

            let mock_logs: Vec<String> = mock_logs.iter().map(|l| l.description.clone()).collect();
            for (i, _) in mock_logs.iter().enumerate() {
                assert_eq!(logs[i], mock_logs[i]);
            }
        })
    })
}

pub fn no_uplink_event() -> Validator {
    Box::new(move || {
        Box::pin(async move {
            // Integration events are handled async.
            sleep(Duration::from_millis(100)).await;
            assert!(mock::get_uplink_event().await.is_none());
        })
    })
}

pub fn join_event(join: integration_pb::JoinEvent) -> Validator {
    Box::new(move || {
        let join = join.clone();
        Box::pin(async move {
            // Integration events are handled async.
            sleep(Duration::from_millis(100)).await;

            let mut event = mock::get_join_event().await.unwrap();

            assert_ne!("", event.deduplication_id);
            assert_ne!(None, event.time);

            event.deduplication_id = "".into();
            event.time = None;
            assert_eq!(join, event);
        })
    })
}

pub fn uplink_event(up: integration_pb::UplinkEvent) -> Validator {
    Box::new(move || {
        let up = up.clone();
        Box::pin(async move {
            // Integration events are handled async.
            sleep(Duration::from_millis(100)).await;

            let mut event = mock::get_uplink_event().await.unwrap();

            assert_ne!("", event.deduplication_id);
            assert_ne!(None, event.time);

            event.deduplication_id = "".into();
            event.time = None;
            assert_eq!(up, event);
        })
    })
}

pub fn ack_event(ack: integration_pb::AckEvent) -> Validator {
    Box::new(move || {
        let ack = ack.clone();
        Box::pin(async move {
            // Integration events are handled async.
            sleep(Duration::from_millis(100)).await;

            let mut mock_events = mock::get_ack_events().await;
            assert_eq!(1, mock_events.len());

            assert_ne!("", mock_events[0].deduplication_id);
            assert_ne!(None, mock_events[0].time);

            mock_events[0].deduplication_id = "".into();
            mock_events[0].time = None;
            assert_eq!(ack, mock_events[0]);
        })
    })
}

pub fn status_event(st: integration_pb::StatusEvent) -> Validator {
    Box::new(move || {
        let st = st.clone();
        Box::pin(async move {
            // Integration events are handled async.
            sleep(Duration::from_millis(100)).await;

            let mut mock_events = mock::get_status_events().await;
            assert_eq!(1, mock_events.len());

            assert_ne!("", mock_events[0].deduplication_id);
            assert_ne!(None, mock_events[0].time);

            mock_events[0].deduplication_id = "".into();
            mock_events[0].time = None;
            assert_eq!(st, mock_events[0]);
        })
    })
}

pub fn device_join_eui(dev_eui: EUI64, join_eui: EUI64) -> Validator {
    Box::new(move || {
        Box::pin(async move {
            let d = device::get(&dev_eui).await.unwrap();
            assert_eq!(join_eui, d.join_eui);
        })
    })
}

pub fn device_session(dev_eui: EUI64, ds: internal::DeviceSession) -> Validator {
    Box::new(move || {
        let ds = ds.clone();
        Box::pin(async move {
            let ds_get = device_session::get(&dev_eui).await.unwrap();
            assert_eq!(ds, ds_get);
        })
    })
}

pub fn no_device_session(dev_eui: EUI64) -> Validator {
    Box::new(move || {
        Box::pin(async move {
            let res = device_session::get(&dev_eui).await;
            assert_eq!(true, res.is_err());
        })
    })
}

pub fn no_downlink_frame() -> Validator {
    Box::new(|| {
        Box::pin(async move {
            let items = gateway_mock::get_downlink_frames().await;
            assert_eq!(0, items.len());
        })
    })
}

pub fn downlink_frame(df: gw::DownlinkFrame) -> Validator {
    Box::new(move || {
        let df = df.clone();
        Box::pin(async move {
            let mut items = gateway_mock::get_downlink_frames().await;

            assert_eq!(1, items.len());
            assert!(items[0].downlink_id != 0);

            let mut last_downlink_id = LAST_DOWNLINK_ID.write().await;
            *last_downlink_id = items[0].downlink_id;

            items[0].downlink_id = 0;
            assert_eq!(df, items[0]);
        })
    })
}

pub fn downlink_phy_payloads(phys: Vec<lrwn::PhyPayload>) -> Validator {
    Box::new(move || {
        let phys = phys.clone();
        Box::pin(async move {
            let items = gateway_mock::get_downlink_frames().await;

            assert_eq!(1, items.len());
            assert!(items[0].downlink_id != 0);

            let mut last_downlink_id = LAST_DOWNLINK_ID.write().await;
            *last_downlink_id = items[0].downlink_id;

            assert_eq!(phys.len(), items[0].items.len());

            for (i, phy) in phys.iter().enumerate() {
                let phy_received =
                    lrwn::PhyPayload::from_slice(&items[0].items[i].phy_payload).unwrap();
                assert_eq!(phy, &phy_received);
            }
        })
    })
}

pub fn downlink_phy_payloads_decoded_f_opts(phys: Vec<lrwn::PhyPayload>) -> Validator {
    Box::new(move || {
        let phys = phys.clone();
        Box::pin(async move {
            let items = gateway_mock::get_downlink_frames().await;

            assert_eq!(1, items.len());
            assert!(items[0].downlink_id != 0);

            let mut last_downlink_id = LAST_DOWNLINK_ID.write().await;
            *last_downlink_id = items[0].downlink_id;

            assert_eq!(phys.len(), items[0].items.len());

            for (i, phy) in phys.iter().enumerate() {
                let mut phy_received =
                    lrwn::PhyPayload::from_slice(&items[0].items[i].phy_payload).unwrap();
                phy_received.decode_f_opts_to_mac_commands().unwrap();
                assert_eq!(phy, &phy_received);
            }
        })
    })
}

// You must use downlink_frame first, in order to set the LAST_DOWNLINK_ID.
pub fn downlink_frame_saved(df: internal::DownlinkFrame) -> Validator {
    Box::new(move || {
        let df = df.clone();
        Box::pin(async move {
            let mut df_get = downlink_frame::get(*LAST_DOWNLINK_ID.read().await)
                .await
                .unwrap();

            df_get.downlink_id = 0;
            if let Some(df) = &mut df_get.downlink_frame {
                df.downlink_id = 0;
            }

            assert_eq!(df, df_get);
        })
    })
}

pub fn device_queue_items(dev_eui: EUI64, items: Vec<device_queue::DeviceQueueItem>) -> Validator {
    Box::new(move || {
        let items = items.clone();
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let items_get = device_queue::get_for_dev_eui(&dev_eui).await.unwrap();

            let items: Vec<device_queue::DeviceQueueItem> = items
                .iter()
                .map(|item| device_queue::DeviceQueueItem {
                    f_port: item.f_port,
                    confirmed: item.confirmed,
                    data: item.data.clone(),
                    is_pending: item.is_pending,
                    ..Default::default()
                })
                .collect();

            let items_get: Vec<device_queue::DeviceQueueItem> = items_get
                .iter()
                .map(|item| device_queue::DeviceQueueItem {
                    f_port: item.f_port,
                    confirmed: item.confirmed,
                    data: item.data.clone(),
                    is_pending: item.is_pending,
                    ..Default::default()
                })
                .collect();

            assert_eq!(items, items_get);
        })
    })
}

pub fn enabled_class(dev_eui: EUI64, c: DeviceClass) -> Validator {
    Box::new(move || {
        let c = c.clone();
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let dev = device::get(&dev_eui).await.unwrap();
            assert_eq!(c, dev.enabled_class);
        })
    })
}

pub fn uplink_meta_log(um: meta::UplinkMeta) -> Validator {
    Box::new(move || {
        let um = um.clone();
        Box::pin(async move {
            let mut c = get_redis_conn().unwrap();
            let key = redis_key("stream:meta".to_string());
            let srr: StreamReadReply = redis::cmd("XREAD")
                .arg("COUNT")
                .arg(1 as usize)
                .arg("STREAMS")
                .arg(&key)
                .arg("0")
                .query(&mut *c)
                .unwrap();

            for stream_key in &srr.keys {
                for stream_id in &stream_key.ids {
                    for (k, v) in &stream_id.map {
                        assert_eq!("up", k);
                        if let redis::Value::Data(b) = v {
                            let pl = meta::UplinkMeta::decode(&mut Cursor::new(b)).unwrap();
                            assert_eq!(um, pl);
                        } else {
                            panic!("Invalid payload");
                        }

                        return;
                    }
                }
            }

            panic!("No UplinkMeta");
        })
    })
}

pub fn device_uplink_frame_log(uf: api::UplinkFrameLog) -> Validator {
    Box::new(move || {
        let uf = uf.clone();
        Box::pin(async move {
            let mut c = get_redis_conn().unwrap();
            let key = redis_key(format!("device:{{{}}}:stream:frame", uf.dev_eui));
            let srr: StreamReadReply = redis::cmd("XREAD")
                .arg("COUNT")
                .arg(1 as usize)
                .arg("STREAMS")
                .arg(&key)
                .arg("0")
                .query(&mut *c)
                .unwrap();

            for stream_key in &srr.keys {
                for stream_id in &stream_key.ids {
                    for (k, v) in &stream_id.map {
                        assert_eq!("up", k);
                        if let redis::Value::Data(b) = v {
                            let mut pl = api::UplinkFrameLog::decode(&mut Cursor::new(b)).unwrap();
                            pl.time = None; // we don't have control over this value
                            assert_eq!(uf, pl);
                        } else {
                            panic!("Invalid payload");
                        }

                        return;
                    }
                }
            }
        })
    })
}

pub fn downlink_device_lock(dev_eui: EUI64) -> Validator {
    Box::new(move || {
        let dev_eui = dev_eui.clone();
        Box::pin(async move {
            let mut c = get_redis_conn().unwrap();
            let key = redis_key(format!("device:{{{}}}:lock", dev_eui));
            let _: String = redis::cmd("GET").arg(key).query(&mut *c).unwrap();
        })
    })
}
