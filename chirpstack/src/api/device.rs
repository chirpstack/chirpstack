use std::str::FromStr;
use std::time::SystemTime;

use bigdecimal::ToPrimitive;
use chrono::{DateTime, Local};
use rand::RngCore;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use chirpstack_api::api::device_service_server::DeviceService;
use chirpstack_api::{api, common, internal};
use lrwn::{AES128Key, DevAddr, EUI64};

use super::auth::validator;
use super::error::ToStatus;
use super::helpers::{self, ToProto};
use crate::storage::error::Error;
use crate::storage::{
    device, device_keys, device_profile, device_queue, device_session, fields, metrics,
};
use crate::{codec, config};

pub struct Device {
    validator: validator::RequestValidator,
}

impl Device {
    pub fn new(validator: validator::RequestValidator) -> Self {
        Device { validator }
    }
}

#[tonic::async_trait]
impl DeviceService for Device {
    async fn create(
        &self,
        request: Request<api::CreateDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req_d = match &request.get_ref().device {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device is missing"));
            }
        };

        let dev_eui = EUI64::from_str(&req_d.dev_eui).map_err(|e| e.status())?;
        let app_id = Uuid::from_str(&req_d.application_id).map_err(|e| e.status())?;
        let dp_id = Uuid::from_str(&req_d.device_profile_id).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDevicesAccess::new(validator::Flag::Create, app_id),
            )
            .await?;

        let d = device::Device {
            dev_eui,
            application_id: app_id,
            device_profile_id: dp_id,
            name: req_d.name.clone(),
            description: req_d.description.clone(),
            skip_fcnt_check: req_d.skip_fcnt_check,
            tags: fields::KeyValue::new(req_d.tags.clone()),
            variables: fields::KeyValue::new(req_d.variables.clone()),
            ..Default::default()
        };

        let _ = device::create(d).await.map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn get(
        &self,
        request: Request<api::GetDeviceRequest>,
    ) -> Result<Response<api::GetDeviceResponse>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, dev_eui),
            )
            .await?;

        let d = device::get(&dev_eui).await.map_err(|e| e.status())?;

        Ok(Response::new(api::GetDeviceResponse {
            device: Some(api::Device {
                dev_eui: d.dev_eui.to_string(),
                name: d.name.clone(),
                description: d.description.clone(),
                application_id: d.application_id.to_string(),
                device_profile_id: d.device_profile_id.to_string(),
                skip_fcnt_check: d.skip_fcnt_check,
                is_disabled: d.is_disabled,
                variables: d.variables.into_hashmap(),
                tags: d.tags.into_hashmap(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&d.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&d.updated_at)),
            last_seen_at: d
                .last_seen_at
                .as_ref()
                .map(helpers::datetime_to_prost_timestamp),
            device_status: match d.margin.is_some() {
                true => Some(api::DeviceStatus {
                    margin: d.margin.unwrap(), // we already know it is Some(v)
                    external_power_source: d.external_power_source,
                    battery_level: match d.battery_level {
                        Some(v) => v.to_f32().unwrap(),
                        None => -1.0,
                    },
                }),
                false => None,
            },
        }))
    }

    async fn update(
        &self,
        request: Request<api::UpdateDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req_d = match &request.get_ref().device {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device is missing"));
            }
        };
        let dev_eui = EUI64::from_str(&req_d.dev_eui).map_err(|e| e.status())?;
        let app_id = Uuid::from_str(&req_d.application_id).map_err(|e| e.status())?;
        let dp_id = Uuid::from_str(&req_d.device_profile_id).map_err(|e| e.status())?;

        // Does the user have access to the device?
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        // Does the user have access to the application (the update could be related to moving the
        // device to a different application).
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateApplicationAccess::new(validator::Flag::Read, app_id),
            )
            .await?;

        // Does the user have access to the device-profile (the update could be related to changing
        // the device-profile of the device).
        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceProfileAccess::new(validator::Flag::Read, dp_id),
            )
            .await?;

        // update
        let _ = device::update(device::Device {
            dev_eui,
            application_id: app_id,
            device_profile_id: dp_id,
            name: req_d.name.clone(),
            description: req_d.description.clone(),
            skip_fcnt_check: req_d.skip_fcnt_check,
            is_disabled: req_d.is_disabled,
            tags: fields::KeyValue::new(req_d.tags.clone()),
            variables: fields::KeyValue::new(req_d.variables.clone()),
            ..Default::default()
        })
        .await
        .map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn delete(
        &self,
        request: Request<api::DeleteDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Delete, dev_eui),
            )
            .await?;

        device::delete(&dev_eui).await.map_err(|e| e.status())?;
        Ok(Response::new(()))
    }

    async fn list(
        &self,
        request: Request<api::ListDevicesRequest>,
    ) -> Result<Response<api::ListDevicesResponse>, Status> {
        let req = request.get_ref();
        let app_id = Uuid::from_str(&req.application_id).map_err(|e| e.status())?;
        let mg_id: Option<Uuid> = if req.multicast_group_id.is_empty() {
            None
        } else {
            Some(Uuid::from_str(&req.multicast_group_id).map_err(|e| e.status())?)
        };

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDevicesAccess::new(validator::Flag::List, app_id),
            )
            .await?;

        if let Some(mg_id) = mg_id {
            self.validator
                .validate(
                    request.extensions(),
                    validator::ValidateMulticastGroupAccess::new(validator::Flag::Read, mg_id),
                )
                .await?;
        }

        let filters = device::Filters {
            application_id: Some(app_id),
            multicast_group_id: mg_id,
            ..Default::default()
        };

        let count = device::get_count(&filters).await.map_err(|e| e.status())?;
        let items = device::list(req.limit as i64, req.offset as i64, &filters)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::ListDevicesResponse {
            total_count: count as u32,
            result: items
                .iter()
                .map(|d| api::DeviceListItem {
                    dev_eui: d.dev_eui.to_string(),
                    created_at: Some(helpers::datetime_to_prost_timestamp(&d.created_at)),
                    updated_at: Some(helpers::datetime_to_prost_timestamp(&d.updated_at)),
                    last_seen_at: d
                        .last_seen_at
                        .as_ref()
                        .map(helpers::datetime_to_prost_timestamp),
                    name: d.name.clone(),
                    description: d.description.clone(),
                    device_profile_id: d.device_profile_id.to_string(),
                    device_profile_name: d.device_profile_name.clone(),
                    device_status: match d.margin.is_some() {
                        true => Some(api::DeviceStatus {
                            margin: d.margin.unwrap(),
                            external_power_source: d.external_power_source,
                            battery_level: match &d.battery_level {
                                Some(v) => v.to_f32().unwrap(),
                                None => -1.0,
                            },
                        }),
                        false => None,
                    },
                })
                .collect(),
        }))
    }

    async fn create_keys(
        &self,
        request: Request<api::CreateDeviceKeysRequest>,
    ) -> Result<Response<()>, Status> {
        let req_dk = match &request.get_ref().device_keys {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device_keys is missing"));
            }
        };
        let dev_eui = EUI64::from_str(&req_dk.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        let dk = device_keys::DeviceKeys {
            dev_eui,
            nwk_key: AES128Key::from_str(&req_dk.nwk_key).map_err(|e| e.status())?,
            app_key: if !req_dk.app_key.is_empty() {
                AES128Key::from_str(&req_dk.app_key).map_err(|e| e.status())?
            } else {
                AES128Key::null()
            },
            ..Default::default()
        };

        let _ = device_keys::create(dk).await.map_err(|e| e.status())?;
        Ok(Response::new(()))
    }

    async fn get_keys(
        &self,
        request: Request<api::GetDeviceKeysRequest>,
    ) -> Result<Response<api::GetDeviceKeysResponse>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, dev_eui),
            )
            .await?;

        let dk = device_keys::get(&dev_eui).await.map_err(|e| e.status())?;

        Ok(Response::new(api::GetDeviceKeysResponse {
            device_keys: Some(api::DeviceKeys {
                dev_eui: dk.dev_eui.to_string(),
                nwk_key: dk.nwk_key.to_string(),
                app_key: dk.app_key.to_string(),
            }),
            created_at: Some(helpers::datetime_to_prost_timestamp(&dk.created_at)),
            updated_at: Some(helpers::datetime_to_prost_timestamp(&dk.updated_at)),
        }))
    }

    async fn update_keys(
        &self,
        request: Request<api::UpdateDeviceKeysRequest>,
    ) -> Result<Response<()>, Status> {
        let req_dk = match &request.get_ref().device_keys {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device_keys is missing"));
            }
        };
        let dev_eui = EUI64::from_str(&req_dk.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        let dk = device_keys::get(&dev_eui).await.map_err(|e| e.status())?;
        let dk = device_keys::DeviceKeys {
            dev_eui: dk.dev_eui,
            created_at: dk.created_at,
            nwk_key: AES128Key::from_str(&req_dk.nwk_key).map_err(|e| e.status())?,
            app_key: if !req_dk.app_key.is_empty() {
                AES128Key::from_str(&req_dk.app_key).map_err(|e| e.status())?
            } else {
                AES128Key::null()
            },
            ..Default::default()
        };
        let _ = device_keys::update(dk).await.map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn delete_keys(
        &self,
        request: Request<api::DeleteDeviceKeysRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        device_keys::delete(&dev_eui)
            .await
            .map_err(|e| e.status())?;
        Ok(Response::new(()))
    }

    async fn flush_dev_nonces(
        &self,
        request: Request<api::FlushDevNoncesRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        device_keys::set_dev_nonces(&dev_eui, &Vec::new())
            .await
            .map_err(|e| e.status())?;
        Ok(Response::new(()))
    }

    async fn activate(
        &self,
        request: Request<api::ActivateDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req_da = match &request.get_ref().device_activation {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("device_activation is missing"));
            }
        };

        let dev_eui = EUI64::from_str(&req_da.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        let d = device::get(&dev_eui).await.map_err(|e| e.status())?;
        let dp = device_profile::get(&d.device_profile_id)
            .await
            .map_err(|e| e.status())?;

        let dev_addr = DevAddr::from_str(&req_da.dev_addr).map_err(|e| e.status())?;
        let s_nwk_s_int_key =
            AES128Key::from_str(&req_da.s_nwk_s_int_key).map_err(|e| e.status())?;
        let f_nwk_s_int_key =
            AES128Key::from_str(&req_da.f_nwk_s_int_key).map_err(|e| e.status())?;
        let nwk_s_enc_key = AES128Key::from_str(&req_da.nwk_s_enc_key).map_err(|e| e.status())?;
        let app_s_key = AES128Key::from_str(&req_da.app_s_key).map_err(|e| e.status())?;

        let ds = internal::DeviceSession {
            region_name: "".to_string(),
            dev_eui: dev_eui.to_vec(),
            dev_addr: dev_addr.to_vec(),
            mac_version: dp.mac_version.to_proto().into(),
            s_nwk_s_int_key: s_nwk_s_int_key.to_vec(),
            f_nwk_s_int_key: f_nwk_s_int_key.to_vec(),
            nwk_s_enc_key: nwk_s_enc_key.to_vec(),
            app_s_key: Some(common::KeyEnvelope {
                kek_label: "".into(),
                aes_key: app_s_key.to_vec(),
            }),
            f_cnt_up: req_da.f_cnt_up,
            n_f_cnt_down: req_da.n_f_cnt_down,
            a_f_cnt_down: req_da.a_f_cnt_down,
            skip_f_cnt_check: d.skip_fcnt_check,
            rx1_delay: dp.abp_rx1_delay as u32,
            rx1_dr_offset: dp.abp_rx1_dr_offset as u32,
            rx2_dr: dp.abp_rx2_dr as u32,
            rx2_frequency: dp.abp_rx2_freq as u32,
            ..Default::default()
        };

        device_session::save(&ds).await.map_err(|e| e.status())?;
        if dp.flush_queue_on_activate {
            device_queue::flush_for_dev_eui(&dev_eui)
                .await
                .map_err(|e| e.status())?;
        }

        Ok(Response::new(()))
    }

    async fn deactivate(
        &self,
        request: Request<api::DeactivateDeviceRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Update, dev_eui),
            )
            .await?;

        device_queue::flush_for_dev_eui(&dev_eui)
            .await
            .map_err(|e| e.status())?;
        device_session::delete(&dev_eui)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn get_activation(
        &self,
        request: Request<api::GetDeviceActivationRequest>,
    ) -> Result<Response<api::GetDeviceActivationResponse>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, dev_eui),
            )
            .await?;

        let ds = match device_session::get(&dev_eui).await {
            Ok(v) => v,
            Err(e) => match e {
                Error::NotFound(_) => {
                    return Ok(Response::new(api::GetDeviceActivationResponse {
                        device_activation: None,
                    }));
                }
                _ => {
                    return Err(e.status());
                }
            },
        };

        Ok(Response::new(api::GetDeviceActivationResponse {
            device_activation: Some(api::DeviceActivation {
                dev_eui: hex::encode(&ds.dev_eui),
                dev_addr: hex::encode(&ds.dev_addr),
                app_s_key: match &ds.app_s_key {
                    Some(v) => hex::encode(&v.aes_key),
                    None => "".to_string(),
                },
                nwk_s_enc_key: hex::encode(&ds.nwk_s_enc_key),
                s_nwk_s_int_key: hex::encode(&ds.s_nwk_s_int_key),
                f_nwk_s_int_key: hex::encode(&ds.f_nwk_s_int_key),
                f_cnt_up: ds.f_cnt_up,
                n_f_cnt_down: ds.n_f_cnt_down,
                a_f_cnt_down: ds.a_f_cnt_down,
            }),
        }))
    }

    async fn get_random_dev_addr(
        &self,
        _request: Request<api::GetRandomDevAddrRequest>,
    ) -> Result<Response<api::GetRandomDevAddrResponse>, Status> {
        let conf = config::get();
        let mut dev_addr: [u8; 4] = [0; 4];

        rand::thread_rng().fill_bytes(&mut dev_addr);
        #[cfg(test)]
        {
            dev_addr = [1, 2, 3, 4];
        }

        let mut dev_addr = DevAddr::from_be_bytes(dev_addr);
        dev_addr.set_addr_prefix(&conf.network.net_id);

        Ok(Response::new(api::GetRandomDevAddrResponse {
            dev_addr: dev_addr.to_string(),
        }))
    }

    async fn get_stats(
        &self,
        request: Request<api::GetDeviceStatsRequest>,
    ) -> Result<Response<api::GetDeviceStatsResponse>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceAccess::new(validator::Flag::Read, dev_eui),
            )
            .await?;

        let start = SystemTime::try_from(
            req.start
                .as_ref()
                .ok_or(anyhow!("start is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let end = SystemTime::try_from(
            req.end
                .as_ref()
                .ok_or(anyhow!("end is None"))
                .map_err(|e| e.status())?
                .clone(),
        )
        .map_err(|e| e.status())?;

        let start: DateTime<Local> = start.into();
        let end: DateTime<Local> = end.into();

        let device_metrics = metrics::get(
            &format!("device:{}", dev_eui),
            metrics::Aggregation::DAY,
            start,
            end,
        )
        .await
        .map_err(|e| e.status())?;

        let mut out: api::GetDeviceStatsResponse = Default::default();

        for m in device_metrics {
            let ts: SystemTime = m.time.into();
            let ts: prost_types::Timestamp = ts.into();

            let mut item = api::DeviceStats {
                time: Some(ts),
                ..Default::default()
            };

            item.rx_packets = m.metrics.get("rx_count").cloned().unwrap_or(0.0) as u32;

            if item.rx_packets > 0 {
                let gw_rssi_sum = m.metrics.get("gw_rssi_sum").cloned().unwrap_or(0.0) as f32;
                let gw_snr_sum = m.metrics.get("gw_snr_sum").cloned().unwrap_or(0.0) as f32;

                item.gw_rssi = gw_rssi_sum / (item.rx_packets as f32);
                item.gw_snr = gw_snr_sum / (item.rx_packets as f32);
            }

            for (k, v) in m.metrics {
                if k.starts_with("rx_freq_") {
                    let freq: u32 = k
                        .trim_start_matches("rx_freq_")
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.status())?;
                    item.rx_packets_per_frequency.insert(freq, v as u32);
                }

                if k.starts_with("rx_dr_") {
                    let dr: u32 = k
                        .trim_start_matches("rx_dr_")
                        .parse()
                        .map_err(|e: std::num::ParseIntError| e.status())?;
                    item.rx_packets_per_dr.insert(dr, v as u32);
                }

                if k.starts_with("error_") {
                    let code = k.trim_start_matches("error_").to_string();
                    item.errors.insert(code, v as u32);
                }
            }

            out.result.push(item);
        }

        Ok(Response::new(out))
    }

    async fn enqueue(
        &self,
        request: Request<api::EnqueueDeviceQueueItemRequest>,
    ) -> Result<Response<api::EnqueueDeviceQueueItemResponse>, Status> {
        let req_qi = match &request.get_ref().item {
            Some(v) => v,
            None => {
                return Err(Status::invalid_argument("item is missing"));
            }
        };
        let dev_eui = EUI64::from_str(&req_qi.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceQueueAccess::new(validator::Flag::Create, dev_eui),
            )
            .await?;

        let mut data = req_qi.data.clone();

        if let Some(obj) = &req_qi.object {
            let dev = device::get(&dev_eui).await.map_err(|e| e.status())?;
            let dp = device_profile::get(&dev.device_profile_id)
                .await
                .map_err(|e| e.status())?;

            data = codec::struct_to_binary(
                dp.payload_codec_runtime,
                req_qi.f_port as u8,
                &dev.variables,
                &dp.payload_codec_script,
                obj,
            )
            .await
            .map_err(|e| e.status())?;
        }

        let qi = device_queue::DeviceQueueItem {
            id: Uuid::new_v4(),
            dev_eui,
            f_port: req_qi.f_port as i16,
            confirmed: req_qi.confirmed,
            data,
            ..Default::default()
        };

        let qi = device_queue::enqueue_item(qi)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::EnqueueDeviceQueueItemResponse {
            id: qi.id.to_string(),
        }))
    }

    async fn flush_queue(
        &self,
        request: Request<api::FlushDeviceQueueRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceQueueAccess::new(validator::Flag::Delete, dev_eui),
            )
            .await?;

        device_queue::flush_for_dev_eui(&dev_eui)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(()))
    }

    async fn get_queue(
        &self,
        request: Request<api::GetDeviceQueueItemsRequest>,
    ) -> Result<Response<api::GetDeviceQueueItemsResponse>, Status> {
        let req = request.get_ref();
        let dev_eui = EUI64::from_str(&req.dev_eui).map_err(|e| e.status())?;

        self.validator
            .validate(
                request.extensions(),
                validator::ValidateDeviceQueueAccess::new(validator::Flag::List, dev_eui),
            )
            .await?;

        let items = device_queue::get_for_dev_eui(&dev_eui)
            .await
            .map_err(|e| e.status())?;

        Ok(Response::new(api::GetDeviceQueueItemsResponse {
            total_count: items.len() as u32,
            result: items
                .iter()
                .map(|qi| api::DeviceQueueItem {
                    id: qi.id.to_string(),
                    dev_eui: qi.dev_eui.to_string(),
                    confirmed: qi.confirmed,
                    f_port: qi.f_port as u32,
                    data: qi.data.clone(),
                    object: None,
                    is_pending: qi.is_pending,
                    f_cnt_down: match qi.f_cnt_down {
                        None => 0,
                        Some(v) => v as u32,
                    },
                })
                .collect(),
        }))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::api::auth::validator::RequestValidator;
    use crate::api::auth::AuthID;
    use crate::storage::{application, tenant, user};
    use crate::test;
    use lrwn::NetID;

    #[tokio::test]
    async fn test_device() {
        let _guard = test::prepare().await;

        // setup admin user
        let u = user::User {
            is_admin: true,
            is_active: true,
            email: "admin@admin".into(),
            email_verified: true,
            ..Default::default()
        };
        let u = user::create(u).await.unwrap();

        // create tenant
        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            max_gateway_count: 10,
            ..Default::default()
        })
        .await
        .unwrap();

        // create application
        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create device-profile
        let dp = device_profile::create(device_profile::DeviceProfile {
            name: "test-dp".into(),
            tenant_id: t.id.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

        // setup the api
        let service = Device::new(RequestValidator::new());

        // create
        let create_req = get_request(
            &u.id,
            api::CreateDeviceRequest {
                device: Some(api::Device {
                    application_id: app.id.to_string(),
                    device_profile_id: dp.id.to_string(),
                    name: "test-device".into(),
                    dev_eui: "0102030405060708".into(),
                    ..Default::default()
                }),
            },
        );
        let _ = service.create(create_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetDeviceRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Device {
                application_id: app.id.to_string(),
                device_profile_id: dp.id.to_string(),
                name: "test-device".into(),
                dev_eui: "0102030405060708".into(),
                ..Default::default()
            }),
            get_resp.get_ref().device
        );

        // update
        let update_req = get_request(
            &u.id,
            api::UpdateDeviceRequest {
                device: Some(api::Device {
                    application_id: app.id.to_string(),
                    device_profile_id: dp.id.to_string(),
                    name: "test-device-updated".into(),
                    dev_eui: "0102030405060708".into(),
                    ..Default::default()
                }),
            },
        );
        let _ = service.update(update_req).await.unwrap();

        // get
        let get_req = get_request(
            &u.id,
            api::GetDeviceRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_resp = service.get(get_req).await.unwrap();
        assert_eq!(
            Some(api::Device {
                application_id: app.id.to_string(),
                device_profile_id: dp.id.to_string(),
                name: "test-device-updated".into(),
                dev_eui: "0102030405060708".into(),
                ..Default::default()
            }),
            get_resp.get_ref().device
        );

        // list
        let list_req = get_request(
            &u.id,
            api::ListDevicesRequest {
                search: "updated".into(),
                application_id: app.id.to_string(),
                multicast_group_id: "".into(),
                limit: 10,
                offset: 0,
            },
        );
        let list_resp = service.list(list_req).await.unwrap();
        assert_eq!(1, list_resp.get_ref().total_count);
        assert_eq!(1, list_resp.get_ref().result.len());

        // create keys
        let create_keys_req = get_request(
            &u.id,
            api::CreateDeviceKeysRequest {
                device_keys: Some(api::DeviceKeys {
                    dev_eui: "0102030405060708".into(),
                    nwk_key: "01020304050607080102030405060708".into(),
                    app_key: "02020304050607080202030405060708".into(),
                }),
            },
        );
        let _ = service.create_keys(create_keys_req).await.unwrap();

        // get keys
        let get_keys_req = get_request(
            &u.id,
            api::GetDeviceKeysRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_keys_resp = service.get_keys(get_keys_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceKeys {
                dev_eui: "0102030405060708".into(),
                nwk_key: "01020304050607080102030405060708".into(),
                app_key: "02020304050607080202030405060708".into(),
            }),
            get_keys_resp.get_ref().device_keys
        );

        // update keys
        let update_keys_req = get_request(
            &u.id,
            api::UpdateDeviceKeysRequest {
                device_keys: Some(api::DeviceKeys {
                    dev_eui: "0102030405060708".into(),
                    nwk_key: "01020304050607080102030405060708".into(),
                    app_key: "03020304050607080302030405060708".into(),
                }),
            },
        );
        let _ = service.update_keys(update_keys_req).await.unwrap();

        // get keys
        let get_keys_req = get_request(
            &u.id,
            api::GetDeviceKeysRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_keys_resp = service.get_keys(get_keys_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceKeys {
                dev_eui: "0102030405060708".into(),
                nwk_key: "01020304050607080102030405060708".into(),
                app_key: "03020304050607080302030405060708".into(),
            }),
            get_keys_resp.get_ref().device_keys
        );

        // flush dev nonces
        let _ = device_keys::set_dev_nonces(
            &EUI64::from_str("0102030405060708").unwrap(),
            &vec![1, 2, 3],
        )
        .await
        .unwrap();
        let flush_dev_nonces_req = get_request(
            &u.id,
            api::FlushDevNoncesRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let _ = service
            .flush_dev_nonces(flush_dev_nonces_req)
            .await
            .unwrap();
        let dk = device_keys::get(&EUI64::from_str("0102030405060708").unwrap())
            .await
            .unwrap();
        assert_eq!(0, dk.dev_nonces.len());

        // delete keys
        let del_keys_req = get_request(
            &u.id,
            api::DeleteDeviceKeysRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let _ = service.delete_keys(del_keys_req).await.unwrap();
        let del_keys_req = get_request(
            &u.id,
            api::DeleteDeviceKeysRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let del_keys_resp = service.delete_keys(del_keys_req).await;
        assert!(del_keys_resp.is_err());

        // activate
        let activate_req = get_request(
            &u.id,
            api::ActivateDeviceRequest {
                device_activation: Some(api::DeviceActivation {
                    dev_eui: "0102030405060708".into(),
                    dev_addr: "04030201".into(),
                    app_s_key: "01020304050607080102030405060708".into(),
                    nwk_s_enc_key: "02020304050607080102030405060708".into(),
                    s_nwk_s_int_key: "03020304050607080102030405060708".into(),
                    f_nwk_s_int_key: "04020304050607080102030405060708".into(),
                    f_cnt_up: 1,
                    n_f_cnt_down: 1,
                    a_f_cnt_down: 1,
                }),
            },
        );
        let _ = service.activate(activate_req).await.unwrap();

        // get activation
        let get_activation_req = get_request(
            &u.id,
            api::GetDeviceActivationRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_activation_resp = service.get_activation(get_activation_req).await.unwrap();
        assert_eq!(
            Some(api::DeviceActivation {
                dev_eui: "0102030405060708".into(),
                dev_addr: "04030201".into(),
                app_s_key: "01020304050607080102030405060708".into(),
                nwk_s_enc_key: "02020304050607080102030405060708".into(),
                s_nwk_s_int_key: "03020304050607080102030405060708".into(),
                f_nwk_s_int_key: "04020304050607080102030405060708".into(),
                f_cnt_up: 1,
                n_f_cnt_down: 1,
                a_f_cnt_down: 1,
            }),
            get_activation_resp.get_ref().device_activation
        );

        // deactivate
        let deactivate_req = get_request(
            &u.id,
            api::DeactivateDeviceRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let _ = service.deactivate(deactivate_req).await.unwrap();
        let get_activation_req = get_request(
            &u.id,
            api::GetDeviceActivationRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_activation_resp = service.get_activation(get_activation_req).await.unwrap();
        assert!(get_activation_resp.get_ref().device_activation.is_none());

        // get random dev addr
        let get_random_dev_addr_req = get_request(
            &u.id,
            api::GetRandomDevAddrRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let get_random_dev_addr_resp = service
            .get_random_dev_addr(get_random_dev_addr_req)
            .await
            .unwrap();
        let dev_addr = DevAddr::from_str(&get_random_dev_addr_resp.get_ref().dev_addr).unwrap();
        let mut dev_addr_copy = dev_addr.clone();
        dev_addr_copy.set_addr_prefix(&NetID::from_str("000000").unwrap());
        assert_eq!(dev_addr, dev_addr_copy);

        // enqueue
        let enqueue_req = get_request(
            &u.id,
            api::EnqueueDeviceQueueItemRequest {
                item: Some(api::DeviceQueueItem {
                    dev_eui: "0102030405060708".into(),
                    confirmed: true,
                    f_port: 2,
                    data: vec![3, 2, 1],
                    ..Default::default()
                }),
            },
        );
        let _ = service.enqueue(enqueue_req).await.unwrap();

        // get queue
        let get_queue_req = get_request(
            &u.id,
            api::GetDeviceQueueItemsRequest {
                dev_eui: "0102030405060708".into(),
                count_only: false,
            },
        );
        let get_queue_resp = service.get_queue(get_queue_req).await.unwrap();
        let get_queue_resp = get_queue_resp.get_ref();
        assert_eq!(1, get_queue_resp.total_count);
        assert_eq!(1, get_queue_resp.result.len());
        assert_eq!(vec![3, 2, 1], get_queue_resp.result[0].data);

        // flush queue
        let flush_queue_req = get_request(
            &u.id,
            api::FlushDeviceQueueRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let _ = service.flush_queue(flush_queue_req).await.unwrap();
        let get_queue_req = get_request(
            &u.id,
            api::GetDeviceQueueItemsRequest {
                dev_eui: "0102030405060708".into(),
                count_only: false,
            },
        );
        let get_queue_resp = service.get_queue(get_queue_req).await.unwrap();
        let get_queue_resp = get_queue_resp.get_ref();
        assert_eq!(0, get_queue_resp.total_count);
        assert_eq!(0, get_queue_resp.result.len());

        // delete
        let del_req = get_request(
            &u.id,
            api::DeleteDeviceRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let _ = service.delete(del_req).await.unwrap();

        let del_req = get_request(
            &u.id,
            api::DeleteDeviceRequest {
                dev_eui: "0102030405060708".into(),
            },
        );
        let del_resp = service.delete(del_req).await;
        assert!(del_resp.is_err());
    }

    fn get_request<T>(user_id: &Uuid, req: T) -> Request<T> {
        let mut req = Request::new(req);
        req.extensions_mut().insert(AuthID::User(user_id.clone()));
        req
    }
}
