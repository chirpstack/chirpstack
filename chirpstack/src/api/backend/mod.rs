use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use redis::streams::StreamReadReply;
use tokio::sync::oneshot;
use tokio::task;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use warp::{http::StatusCode, Filter, Reply};

use crate::backend::{joinserver, keywrap, roaming};
use crate::downlink::data_fns;
use crate::storage::{
    device_session, error::Error as StorageError, get_redis_conn, passive_roaming, redis_key,
};
use crate::uplink::{data_sns, helpers, join_sns, RoamingMetaData, UplinkFrameSet};
use crate::{config, region};
use backend::{BasePayload, MessageType};
use lrwn::region::CommonName;
use lrwn::{AES128Key, NetID, EUI64};

pub async fn setup() -> Result<()> {
    let conf = config::get();
    if conf.backend_interfaces.bind.is_empty() {
        warn!("Backend interfaces API is disabled");
        return Ok(());
    }

    let addr: SocketAddr = conf.backend_interfaces.bind.parse()?;
    info!(bind = %conf.backend_interfaces.bind, "Setting up backend interfaces API");

    let routes = warp::post()
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::aggregate())
        .then(handle_request);

    if !conf.backend_interfaces.ca_cert.is_empty()
        || !conf.backend_interfaces.tls_cert.is_empty()
        || !conf.backend_interfaces.tls_key.is_empty()
    {
        let mut w = warp::serve(routes).tls();
        if !conf.backend_interfaces.ca_cert.is_empty() {
            w = w.client_auth_required_path(&conf.backend_interfaces.ca_cert);
        }
        if !conf.backend_interfaces.tls_cert.is_empty() {
            w = w.cert_path(&conf.backend_interfaces.tls_cert);
        }
        if !conf.backend_interfaces.tls_key.is_empty() {
            w = w.key_path(&conf.backend_interfaces.tls_key);
        }
        w.run(addr).await;
    } else {
        warp::serve(routes).run(addr).await;
    }

    Ok(())
}

pub async fn handle_request(mut body: impl warp::Buf) -> http::Response<hyper::Body> {
    let mut b: Vec<u8> = vec![];

    while body.has_remaining() {
        b.extend_from_slice(body.chunk());
        let cnt = body.chunk().len();
        body.advance(cnt);
    }

    debug!("JSON: {}", String::from_utf8(b.clone()).unwrap_or_default());

    let bp: BasePayload = match serde_json::from_slice(&b) {
        Ok(v) => v,
        Err(e) => {
            return warp::reply::with_status(e.to_string(), StatusCode::BAD_REQUEST)
                .into_response();
        }
    };

    info!(sender_id = %hex::encode(&bp.sender_id), transaction_id = %bp.transaction_id, message_type = ?bp.message_type, "Request received");

    let sender_client = {
        if bp.sender_id.len() == 8 {
            // JoinEUI.
            let sender_id = match EUI64::from_slice(&bp.sender_id) {
                Ok(v) => v,
                Err(e) => {
                    error!(error = %e, "Error decoding SenderID as EUI64");
                    let msg = format!("Error decoding SenderID: {}", e);
                    let pl = bp.to_base_payload_result(backend::ResultCode::MalformedRequest, &msg);
                    return warp::reply::json(&pl).into_response();
                }
            };

            match joinserver::get(&sender_id) {
                Ok(v) => v,
                Err(_) => {
                    error!(sender_id = %sender_id, "Unknown SenderID");
                    let msg = format!("Unknown SenderID: {}", sender_id);
                    let pl = bp.to_base_payload_result(backend::ResultCode::UnknownSender, &msg);
                    return warp::reply::json(&pl).into_response();
                }
            }
        } else if bp.sender_id.len() == 3 {
            // NetID
            let sender_id = match NetID::from_slice(&bp.sender_id) {
                Ok(v) => v,
                Err(e) => {
                    error!(error = %e, "Error decoding SenderID as NetID");
                    let msg = format!("Error decoding SenderID: {}", e);
                    let pl = bp.to_base_payload_result(backend::ResultCode::MalformedRequest, &msg);
                    return warp::reply::json(&pl).into_response();
                }
            };

            match roaming::get(&sender_id) {
                Ok(v) => v,
                Err(_) => {
                    error!(sender_id = %sender_id, "Unknown SenderID");
                    let msg = format!("Unknown SenderID: {}", sender_id);
                    let pl = bp.to_base_payload_result(backend::ResultCode::UnknownSender, &msg);
                    return warp::reply::json(&pl).into_response();
                }
            }
        } else {
            // Unknown size
            error!(sender_id = ?bp.sender_id, "Invalid SenderID length");
            let pl = bp.to_base_payload_result(
                backend::ResultCode::MalformedRequest,
                "Invalid SenderID length",
            );
            return warp::reply::json(&pl).into_response();
        }
    };

    // Request is an async answer.
    if bp.is_answer() {
        tokio::spawn(async move {
            if let Err(e) = handle_async_ans(&bp, &b).await {
                error!(error = %e, "Handle async answer error");
            }
        });
        return warp::reply::with_status("", StatusCode::OK).into_response();
    }

    match bp.message_type {
        MessageType::PRStartReq => handle_pr_start_req(sender_client, bp, &b).await,
        MessageType::PRStopReq => handle_pr_stop_req(sender_client, bp, &b).await,
        MessageType::XmitDataReq => handle_xmit_data_req(sender_client, bp, &b).await,
        // Unknown message
        _ => warp::reply::with_status(
            "Handler for {:?} is not implemented",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

fn err_to_response(e: anyhow::Error, bp: &backend::BasePayload) -> http::Response<hyper::Body> {
    let msg = format!("{}", e);
    let pl = bp.to_base_payload_result(err_to_result_code(e), &msg);
    warp::reply::json(&pl).into_response()
}

fn err_to_result_code(e: anyhow::Error) -> backend::ResultCode {
    if let Some(e) = e.downcast_ref::<StorageError>() {
        return match e {
            StorageError::NotFound(_) => backend::ResultCode::UnknownDevAddr,
            StorageError::InvalidMIC | StorageError::InvalidDevNonce => {
                backend::ResultCode::MICFailed
            }
            _ => backend::ResultCode::Other,
        };
    }
    backend::ResultCode::Other
}

async fn handle_pr_start_req(
    sender_client: Arc<backend::Client>,
    bp: backend::BasePayload,
    b: &[u8],
) -> http::Response<hyper::Body> {
    if sender_client.is_async() {
        let b = b.to_vec();
        task::spawn(async move {
            let ans = match _handle_pr_start_req(&b).await {
                Ok(v) => v,
                Err(e) => {
                    let msg = e.to_string();
                    backend::PRStartAnsPayload {
                        base: bp.to_base_payload_result(err_to_result_code(e), &msg),
                        ..Default::default()
                    }
                }
            };

            if let Err(e) = sender_client.pr_start_ans(backend::Role::FNS, &ans).await {
                error!(error = %e, "Send async PRStartAns error");
            }
        });

        warp::reply::with_status("", StatusCode::OK).into_response()
    } else {
        match _handle_pr_start_req(b).await {
            Ok(v) => warp::reply::json(&v).into_response(),
            Err(e) => err_to_response(e, &bp),
        }
    }
}

async fn _handle_pr_start_req(b: &[u8]) -> Result<backend::PRStartAnsPayload> {
    let pl: backend::PRStartReqPayload = serde_json::from_slice(b)?;
    let phy = lrwn::PhyPayload::from_slice(&pl.phy_payload)?;

    if phy.mhdr.m_type == lrwn::MType::JoinRequest {
        _handle_pr_start_req_join(pl, phy).await
    } else {
        _handle_pr_start_req_data(pl, phy).await
    }
}

async fn _handle_pr_start_req_join(
    pl: backend::PRStartReqPayload,
    phy: lrwn::PhyPayload,
) -> Result<backend::PRStartAnsPayload> {
    let rx_info = roaming::ul_meta_data_to_rx_info(&pl.ul_meta_data)?;
    let tx_info = roaming::ul_meta_data_to_tx_info(&pl.ul_meta_data)?;
    let region_common_name = CommonName::from_str(&pl.ul_meta_data.rf_region)?;
    let region_config_id = region::get_region_config_id(region_common_name)?;
    let dr = pl.ul_meta_data.data_rate.unwrap_or_default();

    let ufs = UplinkFrameSet {
        uplink_set_id: Uuid::new_v4(),
        dr,
        ch: helpers::get_uplink_ch(&region_config_id, tx_info.frequency, dr)?,
        phy_payload: phy,
        tx_info,
        rx_info_set: rx_info,
        gateway_private_up_map: HashMap::new(),
        gateway_private_down_map: HashMap::new(),
        gateway_tenant_id_map: HashMap::new(),
        region_common_name,
        region_config_id,
        roaming_meta_data: Some(RoamingMetaData {
            base_payload: pl.base.clone(),
            ul_meta_data: pl.ul_meta_data.clone(),
        }),
    };

    join_sns::JoinRequest::start_pr(ufs, pl).await
}

async fn _handle_pr_start_req_data(
    pl: backend::PRStartReqPayload,
    phy: lrwn::PhyPayload,
) -> Result<backend::PRStartAnsPayload> {
    let sender_id = NetID::from_slice(&pl.base.sender_id)?;
    let rx_info = roaming::ul_meta_data_to_rx_info(&pl.ul_meta_data)?;
    let tx_info = roaming::ul_meta_data_to_tx_info(&pl.ul_meta_data)?;
    let region_common_name = CommonName::from_str(&pl.ul_meta_data.rf_region)?;
    let region_config_id = region::get_region_config_id(region_common_name)?;
    let dr = pl.ul_meta_data.data_rate.unwrap_or_default();

    let mut ufs = UplinkFrameSet {
        uplink_set_id: Uuid::new_v4(),
        dr,
        ch: helpers::get_uplink_ch(&region_config_id, tx_info.frequency, dr)?,
        phy_payload: phy,
        tx_info,
        rx_info_set: rx_info,
        gateway_private_up_map: HashMap::new(),
        gateway_private_down_map: HashMap::new(),
        gateway_tenant_id_map: HashMap::new(),
        region_common_name,
        region_config_id,
        roaming_meta_data: Some(RoamingMetaData {
            base_payload: pl.base.clone(),
            ul_meta_data: pl.ul_meta_data.clone(),
        }),
    };

    // get device-session
    let ds = device_session::get_for_phypayload(&mut ufs.phy_payload, ufs.dr, ufs.ch as u8).await?;
    let pr_lifetime = roaming::get_passive_roaming_lifetime(sender_id)?;
    let kek_label = roaming::get_passive_roaming_kek_label(sender_id)?;

    let nwk_s_key = if ds.mac_version().to_string().starts_with("1.0") {
        Some(keywrap::wrap(
            &kek_label,
            AES128Key::from_slice(&ds.nwk_s_enc_key)?,
        )?)
    } else {
        None
    };

    let f_nwk_s_int_key = if ds.mac_version().to_string().starts_with("1.0") {
        None
    } else {
        Some(keywrap::wrap(
            &kek_label,
            AES128Key::from_slice(&ds.f_nwk_s_int_key)?,
        )?)
    };

    // In case of stateless, the payload is directly handled
    if pr_lifetime.is_zero() {
        data_sns::Data::handle(ufs).await;
    }

    Ok(backend::PRStartAnsPayload {
        base: pl
            .base
            .to_base_payload_result(backend::ResultCode::Success, ""),
        dev_eui: ds.dev_eui.clone(),
        lifetime: if pr_lifetime.is_zero() {
            None
        } else {
            Some(pr_lifetime.as_secs() as usize)
        },
        f_nwk_s_int_key,
        nwk_s_key,
        f_cnt_up: Some(ds.f_cnt_up),
        ..Default::default()
    })
}

async fn handle_pr_stop_req(
    sender_client: Arc<backend::Client>,
    bp: backend::BasePayload,
    b: &[u8],
) -> http::Response<hyper::Body> {
    if sender_client.is_async() {
        let b = b.to_vec();
        task::spawn(async move {
            let ans = match _handle_pr_stop_req(&b).await {
                Ok(v) => v,
                Err(e) => {
                    let msg = e.to_string();
                    backend::PRStopAnsPayload {
                        base: bp.to_base_payload_result(err_to_result_code(e), &msg),
                    }
                }
            };

            if let Err(e) = sender_client.pr_stop_ans(backend::Role::SNS, &ans).await {
                error!(error = %e, "Send async PRStopAns error");
            }
        });

        warp::reply::with_status("", StatusCode::OK).into_response()
    } else {
        match _handle_pr_stop_req(b).await {
            Ok(v) => warp::reply::json(&v).into_response(),
            Err(e) => err_to_response(e, &bp),
        }
    }
}

async fn _handle_pr_stop_req(b: &[u8]) -> Result<backend::PRStopAnsPayload> {
    let pl: backend::PRStopReqPayload = serde_json::from_slice(b)?;
    let dev_eui = EUI64::from_slice(&pl.dev_eui)?;

    let sess_ids = passive_roaming::get_session_ids_for_dev_eui(dev_eui).await?;
    if sess_ids.is_empty() {
        return Ok(backend::PRStopAnsPayload {
            base: pl
                .base
                .to_base_payload_result(backend::ResultCode::UnknownDevEUI, ""),
        });
    }

    for sess_id in sess_ids {
        if let Err(e) = passive_roaming::delete(sess_id).await {
            error!(error = %e, "Delete passive-roaming device-session error");
        }
    }

    Ok(backend::PRStopAnsPayload {
        base: pl
            .base
            .to_base_payload_result(backend::ResultCode::Success, ""),
    })
}

async fn handle_xmit_data_req(
    sender_client: Arc<backend::Client>,
    bp: backend::BasePayload,
    b: &[u8],
) -> http::Response<hyper::Body> {
    let pl: backend::XmitDataReqPayload = match serde_json::from_slice(b) {
        Ok(v) => v,
        Err(e) => {
            return err_to_response(anyhow::Error::new(e), &bp);
        }
    };

    if sender_client.is_async() {
        task::spawn(async move {
            let sender_role = if pl.ul_meta_data.is_some() {
                backend::Role::FNS
            } else {
                backend::Role::SNS
            };

            let ans = match _handle_xmit_data_req(pl).await {
                Ok(v) => v,
                Err(e) => {
                    let msg = e.to_string();
                    backend::XmitDataAnsPayload {
                        base: bp.to_base_payload_result(err_to_result_code(e), &msg),
                    }
                }
            };

            if let Err(e) = sender_client.xmit_data_ans(sender_role, &ans).await {
                error!(error = %e, "Send async XmitDataAns error");
            }
        });

        warp::reply::with_status("", StatusCode::OK).into_response()
    } else {
        match _handle_xmit_data_req(pl).await {
            Ok(v) => warp::reply::json(&v).into_response(),
            Err(e) => err_to_response(e, &bp),
        }
    }
}

async fn _handle_xmit_data_req(
    pl: backend::XmitDataReqPayload,
) -> Result<backend::XmitDataAnsPayload> {
    if let Some(ul_meta_data) = &pl.ul_meta_data {
        let rx_info = roaming::ul_meta_data_to_rx_info(ul_meta_data)?;
        let tx_info = roaming::ul_meta_data_to_tx_info(ul_meta_data)?;
        let region_common_name = CommonName::from_str(&ul_meta_data.rf_region)?;
        let region_config_id = region::get_region_config_id(region_common_name)?;
        let dr = ul_meta_data.data_rate.unwrap_or_default();
        let phy = lrwn::PhyPayload::from_slice(&pl.phy_payload)?;

        let ufs = UplinkFrameSet {
            uplink_set_id: Uuid::new_v4(),
            dr,
            ch: helpers::get_uplink_ch(&region_config_id, tx_info.frequency, dr)?,
            phy_payload: phy,
            tx_info,
            rx_info_set: rx_info,
            gateway_private_up_map: HashMap::new(),
            gateway_private_down_map: HashMap::new(),
            gateway_tenant_id_map: HashMap::new(),
            region_common_name,
            region_config_id,
            roaming_meta_data: Some(RoamingMetaData {
                base_payload: pl.base.clone(),
                ul_meta_data: ul_meta_data.clone(),
            }),
        };

        data_sns::Data::handle(ufs).await;
    }

    if let Some(dl_meta_data) = &pl.dl_meta_data {
        data_fns::Data::handle(pl.clone(), dl_meta_data.clone()).await?;
    }

    Ok(backend::XmitDataAnsPayload {
        base: pl
            .base
            .to_base_payload_result(backend::ResultCode::Success, ""),
    })
}

async fn handle_async_ans(bp: &BasePayload, b: &[u8]) -> Result<http::Response<hyper::Body>> {
    task::spawn_blocking({
        let b = b.to_vec();
        let transaction_id = bp.transaction_id;
        move || -> Result<()> {
            let mut c = get_redis_conn()?;
            let key = redis_key(format!("backend:async:{}", transaction_id));

            c.new_pipeline()
                .atomic()
                .cmd("XADD")
                .arg(&key)
                .arg("MAXLEN")
                .arg(1_i64)
                .arg("*")
                .arg("pl")
                .arg(&b)
                .ignore()
                .cmd("EXPIRE")
                .arg(&key)
                .arg(30_i64)
                .ignore()
                .query(&mut c)?;

            Ok(())
        }
    })
    .await??;

    Ok(warp::reply().into_response())
}

pub async fn get_async_receiver(
    transaction_id: u32,
    timeout: Duration,
) -> Result<oneshot::Receiver<Vec<u8>>> {
    let (tx, rx) = oneshot::channel();

    task::spawn_blocking(move || -> Result<()> {
        let mut c = get_redis_conn()?;
        let key = redis_key(format!("backend:async:{}", transaction_id));

        let srr: StreamReadReply = redis::cmd("XREAD")
            .arg("BLOCK")
            .arg(timeout.as_millis() as u64)
            .arg("COUNT")
            .arg(1_u64)
            .arg("STREAMS")
            .arg(&key)
            .arg("0")
            .query(&mut *c)?;

        for stream_key in &srr.keys {
            for stream_id in &stream_key.ids {
                for (k, v) in &stream_id.map {
                    match k.as_ref() {
                        "pl" => {
                            if let redis::Value::Data(b) = v {
                                let _ = tx.send(b.to_vec());
                                return Ok(());
                            }
                        }
                        _ => {
                            error!(key = %k, "Unexpected key in async stream");
                        }
                    }
                }
            }
        }

        Ok(())
    });

    Ok(rx)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test;

    #[tokio::test]
    async fn test_async_response() {
        let _guard = test::prepare().await;

        let bp = BasePayload {
            transaction_id: 1234,
            ..Default::default()
        };

        let b = vec![1, 2, 3, 4];
        handle_async_ans(&bp, &b).await.unwrap();

        let rx = get_async_receiver(1234, Duration::from_millis(100))
            .await
            .unwrap();

        let rx_b = rx.await.unwrap();
        assert_eq!(b, rx_b);
    }
}
