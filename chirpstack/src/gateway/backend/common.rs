use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::io::Cursor;
use std::str;
use std::sync::RwLock;

use anyhow::Result;
use prometheus_client::encoding::text::Encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prost::Message;
use tokio::task;
use tracing::{error, info, trace};

use lrwn::region::CommonName;

use crate::{downlink, uplink};
use crate::monitoring::prometheus;
use crate::storage::{get_redis_conn, redis_key};

lazy_static! {
    pub static ref EVENT_COUNTER: Family<EventLabels, Counter> = {
        let counter = Family::<EventLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_events",
            "Number of events received",
            Box::new(counter.clone()),
        );
        counter
    };
  pub static ref COMMAND_COUNTER: Family<CommandLabels, Counter> = {
        let counter = Family::<CommandLabels, Counter>::default();
        prometheus::register(
            "gateway_backend_mqtt_commands",
            "Number of commands sent",
            Box::new(counter.clone()),
        );
        counter
    };
    static ref GATEWAY_JSON: RwLock<HashMap<String, bool>> = RwLock::new(HashMap::new());
}
#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct EventLabels {
    event: String,
}

#[derive(Clone, Hash, PartialEq, Eq, Encode)]
pub struct CommandLabels {
    pub command: String,
}

pub async fn message_callback(region_common_name: CommonName, payload: &[u8], region_name: &str, topic: &str) {
    let mut hasher = DefaultHasher::new();
    hasher.write(payload);
    let key = redis_key(format!("gw:mqtt:lock:{:x}", hasher.finish()));
    let locked = is_locked(key).await;

    match manage_message(&region_common_name, &payload, region_name, topic, locked) {
        Ok(()) => (),
        Err(err) =>   error!(
            topic = topic,
            "Processing gateway event error: {}",
            err
        ),
    };
}

fn manage_message(region_common_name: &CommonName, payload: &&[u8], region_name: &str, topic: &str, locked: Result<bool>) -> Result<(),anyhow::Error> {
    if locked? {
        trace!(
                region_name = region_name,
                topic = topic,
                "Message is already handled by different instance"
            );
        return Ok(());
    }

    let json = payload_is_json(payload);

    info!(
            region_name = region_name,
            topic = topic,
            json = json,
            "Message received from gateway"
        );

    if topic.eq("up") {
        EVENT_COUNTER
            .get_or_create(&EventLabels {
                event: "up".to_string(),
            })
            .inc();
        let mut event = match json {
            true => serde_json::from_slice(payload)?,
            false => chirpstack_api::gw::UplinkFrame::decode(&mut Cursor::new(payload))?,
        };
        event.v4_migrate();

        if let Some(rx_info) = &mut event.rx_info {
            set_gateway_json(&rx_info.gateway_id, json);
            rx_info.set_metadata_string("region_name", region_name);
            rx_info.set_metadata_string("region_common_name", &region_common_name.to_string());
        }

        tokio::spawn(uplink::deduplicate_uplink(event));
    } else if topic.eq("stats") {
        EVENT_COUNTER
            .get_or_create(&EventLabels {
                event: "stats".to_string(),
            })
            .inc();
        let mut event = match json {
            true => serde_json::from_slice(payload)?,
            false => chirpstack_api::gw::GatewayStats::decode(&mut Cursor::new(payload))?,
        };
        event.v4_migrate();
        event
            .meta_data
            .insert("region_name".to_string(), region_name.to_string());
        event.meta_data.insert(
            "region_common_name".to_string(),
            region_common_name.to_string(),
        );
        set_gateway_json(&event.gateway_id, json);
        tokio::spawn(uplink::stats::Stats::handle(event));
    } else if topic.eq("ack") {
        EVENT_COUNTER
            .get_or_create(&EventLabels {
                event: "ack".to_string(),
            })
            .inc();
        let mut event = match json {
            true => serde_json::from_slice(payload)?,
            false => chirpstack_api::gw::DownlinkTxAck::decode(&mut Cursor::new(payload))?,
        };
        event.v4_migrate();
        set_gateway_json(&event.gateway_id, json);
        tokio::spawn(downlink::tx_ack::TxAck::handle(event));
    } else {
        return Err(anyhow!("Unknown event type"));
    }

    Ok(())
}


//TODO common
async fn is_locked(key: String) -> Result<bool> {
    task::spawn_blocking({
        move || -> Result<bool> {
            let mut c = get_redis_conn()?;

            let set: bool = redis::cmd("SET")
                .arg(key)
                .arg("lock")
                .arg("PX")
                .arg(5000)
                .arg("NX")
                .query(&mut *c)?;

            Ok(!set)
        }
    })
        .await?
}

//TODO common
fn payload_is_json(b: &[u8]) -> bool {
    String::from_utf8_lossy(b).contains("gatewayId")
}

//TODO common
fn set_gateway_json(gateway_id: &str, is_json: bool) {
    let mut gw_json_w = GATEWAY_JSON.write().unwrap();
    gw_json_w.insert(gateway_id.to_string(), is_json);
}

pub fn gateway_is_json(gateway_id: &str) -> bool {
    let gw_json_r = GATEWAY_JSON.read().unwrap();
    gw_json_r.get(gateway_id).cloned().unwrap_or(false)
}


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_is_json() {
        let r = payload_is_json(b"{\"phyPayload\":\"ABkCMP7/49dgGQIw/v/j12ChxkpEVts=\",\"txInfo\":{\"frequency\":867500000,\"modulation\":{\"lora\":{\"bandwidth\":125000,\"spreadingFactor\":12,\"codeRate\":\"CR_4_5\"}}},\"rxInfo\":{\"gatewayId\":\"1000000000000009\",\"uplinkId\":64154,\"time\":\"2022-09-15T08:21:25.369388Z\",\"rssi\":-60,\"snr\":5.5,\"channel\":5,\"context\":\"AAAAAA==\"}}");
        assert_eq!(r, true);
    }
}