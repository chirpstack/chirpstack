use std::collections::HashMap;

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use diesel::{dsl, prelude::*};
use diesel_async::RunQueryDsl;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use crate::config;
use crate::storage::error::Error;
use crate::storage::schema::{
    application, device, fuota_deployment, fuota_deployment_device, fuota_deployment_gateway,
    fuota_deployment_job, gateway, tenant,
};
use crate::storage::{self, db_transaction, device_profile, fields, get_async_db_conn};
use lrwn::{AES128Key, DevAddr, EUI64};

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq, Validate)]
#[diesel(table_name = fuota_deployment)]
pub struct FuotaDeployment {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub name: String,
    pub application_id: fields::Uuid,
    pub device_profile_id: fields::Uuid,
    pub multicast_addr: DevAddr,
    pub multicast_key: AES128Key,
    pub multicast_group_type: String,
    pub multicast_class_c_scheduling_type: fields::MulticastGroupSchedulingType,
    pub multicast_dr: i16,
    pub multicast_class_b_ping_slot_periodicity: i16,
    pub multicast_frequency: i64,
    pub multicast_timeout: i16,
    pub multicast_session_start: Option<DateTime<Utc>>,
    pub multicast_session_end: Option<DateTime<Utc>>,
    pub unicast_max_retry_count: i16,
    pub fragmentation_fragment_size: i16,
    pub fragmentation_redundancy_percentage: i16,
    pub fragmentation_session_index: i16,
    pub fragmentation_matrix: i16,
    pub fragmentation_block_ack_delay: i16,
    pub fragmentation_descriptor: Vec<u8>,
    pub request_fragmentation_session_status: fields::RequestFragmentationSessionStatus,
    pub payload: Vec<u8>,
    pub on_complete_set_device_tags: fields::KeyValue,
}

impl Default for FuotaDeployment {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4().into(),
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            name: "".into(),
            application_id: Uuid::nil().into(),
            device_profile_id: Uuid::nil().into(),
            multicast_addr: Default::default(),
            multicast_key: Default::default(),
            multicast_group_type: "".into(),
            multicast_class_c_scheduling_type: fields::MulticastGroupSchedulingType::DELAY,
            multicast_dr: 0,
            multicast_class_b_ping_slot_periodicity: 0,
            multicast_frequency: 0,
            multicast_timeout: 0,
            multicast_session_start: None,
            multicast_session_end: None,
            unicast_max_retry_count: 0,
            fragmentation_fragment_size: 0,
            fragmentation_redundancy_percentage: 0,
            fragmentation_session_index: 0,
            fragmentation_matrix: 0,
            fragmentation_block_ack_delay: 0,
            fragmentation_descriptor: Vec::new(),
            request_fragmentation_session_status:
                fields::RequestFragmentationSessionStatus::NoRequest,
            payload: Vec::new(),
            on_complete_set_device_tags: fields::KeyValue::new(HashMap::new()),
        }
    }
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct FuotaDeploymentListItem {
    pub id: fields::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub name: String,
}

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = fuota_deployment_device)]
pub struct FuotaDeploymentDevice {
    pub fuota_deployment_id: fields::Uuid,
    pub dev_eui: EUI64,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub mc_group_setup_completed_at: Option<DateTime<Utc>>,
    pub mc_session_completed_at: Option<DateTime<Utc>>,
    pub frag_session_setup_completed_at: Option<DateTime<Utc>>,
    pub frag_status_completed_at: Option<DateTime<Utc>>,
    pub error_msg: String,
}

impl Default for FuotaDeploymentDevice {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            fuota_deployment_id: Uuid::nil().into(),
            dev_eui: EUI64::default(),
            created_at: now,
            completed_at: None,
            mc_group_setup_completed_at: None,
            mc_session_completed_at: None,
            frag_session_setup_completed_at: None,
            frag_status_completed_at: None,
            error_msg: "".into(),
        }
    }
}

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = fuota_deployment_gateway)]
pub struct FuotaDeploymentGateway {
    pub fuota_deployment_id: fields::Uuid,
    pub gateway_id: EUI64,
    pub created_at: DateTime<Utc>,
}

impl Default for FuotaDeploymentGateway {
    fn default() -> Self {
        Self {
            fuota_deployment_id: Uuid::nil().into(),
            gateway_id: EUI64::default(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Clone, Queryable, QueryableByName, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = fuota_deployment_job)]
pub struct FuotaDeploymentJob {
    pub fuota_deployment_id: fields::Uuid,
    pub job: fields::FuotaJob,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub max_retry_count: i16,
    pub attempt_count: i16,
    pub scheduler_run_after: DateTime<Utc>,
    pub warning_msg: String,
    pub error_msg: String,
}

impl Default for FuotaDeploymentJob {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            fuota_deployment_id: Uuid::nil().into(),
            job: fields::FuotaJob::McGroupSetup,
            created_at: now,
            completed_at: None,
            max_retry_count: 0,
            attempt_count: 0,
            scheduler_run_after: now,
            warning_msg: "".into(),
            error_msg: "".into(),
        }
    }
}

pub async fn create_deployment(d: FuotaDeployment) -> Result<FuotaDeployment, Error> {
    d.validate()?;

    let app = storage::application::get(&d.application_id).await?;
    let dp = device_profile::get(&d.device_profile_id).await?;
    if app.tenant_id != dp.tenant_id {
        return Err(Error::Validation(
            "The application and device-profile must be under the samen tenant".into(),
        ));
    }

    let d: FuotaDeployment = diesel::insert_into(fuota_deployment::table)
        .values(&d)
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, d.id.to_string()))?;

    info!(id = %d.id, "FUOTA deployment created");
    Ok(d)
}

pub async fn get_deployment(id: Uuid) -> Result<FuotaDeployment, Error> {
    fuota_deployment::dsl::fuota_deployment
        .find(&fields::Uuid::from(id))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, id.to_string()))
}

pub async fn update_deployment(d: FuotaDeployment) -> Result<FuotaDeployment, Error> {
    d.validate()?;

    let d: FuotaDeployment = diesel::update(fuota_deployment::dsl::fuota_deployment.find(&d.id))
        .set((
            fuota_deployment::updated_at.eq(&Utc::now()),
            fuota_deployment::started_at.eq(&d.started_at),
            fuota_deployment::completed_at.eq(&d.completed_at),
            fuota_deployment::name.eq(&d.name),
            fuota_deployment::multicast_group_type.eq(&d.multicast_group_type),
            fuota_deployment::multicast_class_c_scheduling_type
                .eq(&d.multicast_class_c_scheduling_type),
            fuota_deployment::multicast_dr.eq(&d.multicast_dr),
            fuota_deployment::multicast_class_b_ping_slot_periodicity
                .eq(&d.multicast_class_b_ping_slot_periodicity),
            fuota_deployment::multicast_frequency.eq(&d.multicast_frequency),
            fuota_deployment::multicast_timeout.eq(&d.multicast_timeout),
            fuota_deployment::multicast_session_start.eq(&d.multicast_session_start),
            fuota_deployment::multicast_session_end.eq(&d.multicast_session_end),
            fuota_deployment::unicast_max_retry_count.eq(&d.unicast_max_retry_count),
            fuota_deployment::fragmentation_fragment_size.eq(&d.fragmentation_fragment_size),
            fuota_deployment::fragmentation_redundancy_percentage
                .eq(&d.fragmentation_redundancy_percentage),
            fuota_deployment::fragmentation_session_index.eq(&d.fragmentation_session_index),
            fuota_deployment::fragmentation_matrix.eq(&d.fragmentation_matrix),
            fuota_deployment::fragmentation_block_ack_delay.eq(&d.fragmentation_block_ack_delay),
            fuota_deployment::fragmentation_descriptor.eq(&d.fragmentation_descriptor),
            fuota_deployment::request_fragmentation_session_status
                .eq(&d.request_fragmentation_session_status),
            fuota_deployment::payload.eq(&d.payload),
            fuota_deployment::on_complete_set_device_tags.eq(&d.on_complete_set_device_tags),
        ))
        .get_result(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, d.id.to_string()))?;

    info!(id = %d.id, "FUOTA deployment updated");
    Ok(d)
}

pub async fn delete_deployment(id: Uuid) -> Result<(), Error> {
    let ra = diesel::delete(fuota_deployment::dsl::fuota_deployment.find(&fields::Uuid::from(id)))
        .execute(&mut get_async_db_conn().await?)
        .await?;
    if ra == 0 {
        return Err(Error::NotFound(id.to_string()));
    }
    info!(id = %id, "FUOTA deployment deleted");
    Ok(())
}

pub async fn get_deployment_count(application_id: Uuid) -> Result<i64, Error> {
    fuota_deployment::dsl::fuota_deployment
        .select(dsl::count_star())
        .filter(fuota_deployment::dsl::application_id.eq(fields::Uuid::from(application_id)))
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn list_deployments(
    application_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<FuotaDeploymentListItem>, Error> {
    fuota_deployment::dsl::fuota_deployment
        .select((
            fuota_deployment::id,
            fuota_deployment::created_at,
            fuota_deployment::updated_at,
            fuota_deployment::started_at,
            fuota_deployment::completed_at,
            fuota_deployment::name,
        ))
        .filter(fuota_deployment::dsl::application_id.eq(fields::Uuid::from(application_id)))
        .order_by(fuota_deployment::dsl::name)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn add_devices(fuota_deployment_id: Uuid, dev_euis: Vec<EUI64>) -> Result<(), Error> {
    let mut errors = Vec::new();

    let dev_euis_filtered: Vec<EUI64> = device::dsl::device
        .select(device::dsl::dev_eui)
        .inner_join(
            fuota_deployment::table
                .on(fuota_deployment::dsl::device_profile_id.eq(device::dsl::device_profile_id)),
        )
        .inner_join(application::table.on(application::dsl::id.eq(device::dsl::application_id)))
        .filter(application::dsl::id.eq(fuota_deployment::dsl::application_id))
        .filter(fuota_deployment::dsl::id.eq(fields::Uuid::from(fuota_deployment_id)))
        .filter(device::dsl::dev_eui.eq_any(&dev_euis))
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))?;

    if dev_euis_filtered.len() != dev_euis.len() {
        return Err(Error::Validation(
            "All devices must have the same device-profile as the FUOTA deployment".into(),
        ));
    }

    for dev_eui in dev_euis {
        let res = diesel::insert_into(fuota_deployment_device::table)
            .values(&FuotaDeploymentDevice {
                fuota_deployment_id: fuota_deployment_id.into(),
                dev_eui,
                ..Default::default()
            })
            .execute(&mut get_async_db_conn().await?)
            .await
            .map_err(|e| Error::from_diesel(e, dev_eui.to_string()));

        if let Err(e) = res {
            errors.push(e);
        }
    }

    info!(fuota_deployment_id = %fuota_deployment_id, "Added DeEUIs to FUOTA Deployment");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::Multi(errors))
    }
}

pub async fn get_devices(
    fuota_deployment_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<FuotaDeploymentDevice>, Error> {
    let mut q = fuota_deployment_device::dsl::fuota_deployment_device
        .filter(
            fuota_deployment_device::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .into_boxed();

    if limit != -1 {
        q = q
            .order_by(fuota_deployment_device::dsl::dev_eui)
            .limit(limit)
            .offset(offset);
    }

    q.load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn get_latest_device_by_dev_eui(dev_eui: EUI64) -> Result<FuotaDeploymentDevice, Error> {
    fuota_deployment_device::dsl::fuota_deployment_device
        .filter(fuota_deployment_device::dsl::dev_eui.eq(&dev_eui))
        .order_by(fuota_deployment_device::created_at.desc())
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
}

pub async fn update_device(d: FuotaDeploymentDevice) -> Result<FuotaDeploymentDevice, Error> {
    let d: FuotaDeploymentDevice = diesel::update(
        fuota_deployment_device::dsl::fuota_deployment_device
            .find((&d.fuota_deployment_id, &d.dev_eui)),
    )
    .set((
        fuota_deployment_device::completed_at.eq(&d.completed_at),
        fuota_deployment_device::mc_group_setup_completed_at.eq(&d.mc_group_setup_completed_at),
        fuota_deployment_device::mc_session_completed_at.eq(&d.mc_session_completed_at),
        fuota_deployment_device::frag_session_setup_completed_at
            .eq(&d.frag_session_setup_completed_at),
        fuota_deployment_device::frag_status_completed_at.eq(&d.frag_status_completed_at),
        fuota_deployment_device::error_msg.eq(&d.error_msg),
    ))
    .get_result(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, d.dev_eui.to_string()))?;

    info!(fuota_deployment_id = %d.fuota_deployment_id, dev_eui = %d.dev_eui, "FUOTA deployment device updated");
    Ok(d)
}

pub async fn remove_devices(fuota_deployment_id: Uuid, dev_euis: Vec<EUI64>) -> Result<(), Error> {
    diesel::delete(
        fuota_deployment_device::table
            .filter(
                fuota_deployment_device::dsl::fuota_deployment_id
                    .eq(fields::Uuid::from(fuota_deployment_id)),
            )
            .filter(fuota_deployment_device::dsl::dev_eui.eq_any(&dev_euis)),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;

    info!(fuota_deployment_id = %fuota_deployment_id, "DevEUIs removed from FUOTA Deployment");
    Ok(())
}

pub async fn get_device_count(fuota_deployment_id: Uuid) -> Result<i64, Error> {
    fuota_deployment_device::dsl::fuota_deployment_device
        .select(dsl::count_star())
        .filter(
            fuota_deployment_device::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn set_device_timeout_error(
    fuota_deployment_id: Uuid,
    mc_group_setup_timeout: bool,
    mc_session_timeout: bool,
    frag_session_setup_timeout: bool,
    frag_status_timeout: bool,
) -> Result<()> {
    let fuota_deployment_id = fields::Uuid::from(fuota_deployment_id);

    let mut error_msg = String::new();
    if mc_group_setup_timeout {
        error_msg = "McGroupSetupReq timeout.".into();
    }
    if mc_session_timeout {
        error_msg = "McSessionReq timeout".into();
    }
    if frag_session_setup_timeout {
        error_msg = "FragSessionSetupReq timeout.".into();
    }
    if frag_status_timeout {
        error_msg = "FragStatusReq timeout.".into();
    }

    let mut q = diesel::update(fuota_deployment_device::table)
        .set(fuota_deployment_device::dsl::error_msg.eq(&error_msg))
        .filter(fuota_deployment_device::dsl::fuota_deployment_id.eq(&fuota_deployment_id))
        .filter(fuota_deployment_device::dsl::error_msg.eq(""))
        .into_boxed();

    if mc_group_setup_timeout {
        q = q.filter(fuota_deployment_device::dsl::mc_group_setup_completed_at.is_null());
    }

    if mc_session_timeout {
        q = q.filter(fuota_deployment_device::dsl::mc_session_completed_at.is_null());
    }

    if frag_session_setup_timeout {
        q = q.filter(fuota_deployment_device::dsl::frag_session_setup_completed_at.is_null());
    }

    if frag_status_timeout {
        q = q.filter(fuota_deployment_device::dsl::frag_status_completed_at.is_null());
    }

    q.execute(&mut get_async_db_conn().await?).await?;

    Ok(())
}

pub async fn set_device_completed(
    fuota_deployment_id: Uuid,
    mc_group_setup_completed: bool,
    mc_session_completed: bool,
    frag_session_setup_completed: bool,
    frag_status_completed: bool,
) -> Result<()> {
    let fuota_deployment_id = fields::Uuid::from(fuota_deployment_id);

    let mut q = diesel::update(fuota_deployment_device::table)
        .set(fuota_deployment_device::dsl::completed_at.eq(Some(Utc::now())))
        .filter(fuota_deployment_device::dsl::fuota_deployment_id.eq(&fuota_deployment_id))
        .into_boxed();

    if mc_group_setup_completed {
        q = q.filter(fuota_deployment_device::dsl::mc_group_setup_completed_at.is_not_null());
    }

    if mc_session_completed {
        q = q.filter(fuota_deployment_device::dsl::mc_session_completed_at.is_not_null());
    }

    if frag_session_setup_completed {
        q = q.filter(fuota_deployment_device::dsl::frag_session_setup_completed_at.is_not_null());
    }

    if frag_status_completed {
        q = q.filter(fuota_deployment_device::dsl::frag_status_completed_at.is_not_null());
    }

    q.execute(&mut get_async_db_conn().await?).await?;

    Ok(())
}

pub async fn add_gateways(fuota_deployment_id: Uuid, gateway_ids: Vec<EUI64>) -> Result<(), Error> {
    let mut errors = Vec::new();

    let gateway_ids_filtered: Vec<EUI64> = gateway::dsl::gateway
        .select(gateway::dsl::gateway_id)
        .inner_join(tenant::table.on(tenant::dsl::id.eq(gateway::dsl::tenant_id)))
        .inner_join(application::table.on(application::dsl::tenant_id.eq(tenant::dsl::id)))
        .inner_join(
            fuota_deployment::table
                .on(fuota_deployment::dsl::application_id.eq(application::dsl::id)),
        )
        .filter(fuota_deployment::dsl::id.eq(fields::Uuid::from(fuota_deployment_id)))
        .filter(gateway::dsl::gateway_id.eq_any(&gateway_ids))
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))?;

    if gateway_ids_filtered.len() != gateway_ids.len() {
        return Err(Error::Validation(
            "All gateways must be under the same tenant as the FUOTA deployment".into(),
        ));
    }

    for gateway_id in gateway_ids {
        let res = diesel::insert_into(fuota_deployment_gateway::table)
            .values(&FuotaDeploymentGateway {
                fuota_deployment_id: fuota_deployment_id.into(),
                gateway_id,
                ..Default::default()
            })
            .execute(&mut get_async_db_conn().await?)
            .await
            .map_err(|e| Error::from_diesel(e, gateway_id.to_string()));

        if let Err(e) = res {
            errors.push(e);
        }
    }

    info!(fuota_deployment_id = %fuota_deployment_id, "Added Gateway IDs to FUOTA Deployment");

    if errors.is_empty() {
        Ok(())
    } else {
        Err(Error::Multi(errors))
    }
}

pub async fn remove_gateways(
    fuota_deployment_id: Uuid,
    gateway_ids: Vec<EUI64>,
) -> Result<(), Error> {
    diesel::delete(
        fuota_deployment_gateway::table
            .filter(
                fuota_deployment_gateway::dsl::fuota_deployment_id
                    .eq(fields::Uuid::from(fuota_deployment_id)),
            )
            .filter(fuota_deployment_gateway::dsl::gateway_id.eq_any(gateway_ids)),
    )
    .execute(&mut get_async_db_conn().await?)
    .await?;

    info!(fuota_deployment_id = %fuota_deployment_id, "Gateway IDs removed from FUOTA Deployment");
    Ok(())
}

pub async fn get_gateway_count(fuota_deployment_id: Uuid) -> Result<i64, Error> {
    fuota_deployment_gateway::dsl::fuota_deployment_gateway
        .select(dsl::count_star())
        .filter(
            fuota_deployment_gateway::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .first(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

pub async fn get_gateways(
    fuota_deployment_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<FuotaDeploymentGateway>, Error> {
    let mut q = fuota_deployment_gateway::dsl::fuota_deployment_gateway
        .filter(
            fuota_deployment_gateway::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .into_boxed();

    if limit != -1 {
        q = q
            .order_by(fuota_deployment_gateway::dsl::gateway_id)
            .limit(limit)
            .offset(offset);
    }

    q.load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}

// Creating a new job, will set any pending job(s) to completed within the same transaction.
pub async fn create_job(j: FuotaDeploymentJob) -> Result<FuotaDeploymentJob, Error> {
    let mut c = get_async_db_conn().await?;
    let j: FuotaDeploymentJob = db_transaction::<FuotaDeploymentJob, Error, _>(&mut c, |c| {
        Box::pin(async move {
            // set pending job(s) to completed
            diesel::update(
                fuota_deployment_job::dsl::fuota_deployment_job
                    .filter(
                        fuota_deployment_job::dsl::fuota_deployment_id.eq(&j.fuota_deployment_id),
                    )
                    .filter(fuota_deployment_job::dsl::completed_at.is_null()),
            )
            .set(fuota_deployment_job::dsl::completed_at.eq(Utc::now()))
            .execute(c)
            .await?;

            // create new job
            diesel::insert_into(fuota_deployment_job::table)
                .values(&j)
                .get_result(c)
                .await
                .map_err(|e| Error::from_diesel(e, j.fuota_deployment_id.to_string()))
        })
    })
    .await?;

    info!(fuota_deployment_id = %j.fuota_deployment_id, job = %j.job, "FUOTA deployment job created");
    Ok(j)
}

pub async fn update_job(j: FuotaDeploymentJob) -> Result<FuotaDeploymentJob, Error> {
    let j: FuotaDeploymentJob = diesel::update(
        fuota_deployment_job::dsl::fuota_deployment_job.find((&j.fuota_deployment_id, &j.job)),
    )
    .set((
        fuota_deployment_job::completed_at.eq(&j.completed_at),
        fuota_deployment_job::attempt_count.eq(&j.attempt_count),
        fuota_deployment_job::scheduler_run_after.eq(&j.scheduler_run_after),
        fuota_deployment_job::warning_msg.eq(&j.warning_msg),
        fuota_deployment_job::error_msg.eq(&j.error_msg),
    ))
    .get_result(&mut get_async_db_conn().await?)
    .await
    .map_err(|e| Error::from_diesel(e, j.fuota_deployment_id.to_string()))?;

    info!(fuota_deployment_id = %j.fuota_deployment_id, job = %j.job, "FUOTA deployment job updated");
    Ok(j)
}

pub async fn list_jobs(fuota_deployment_id: Uuid) -> Result<Vec<FuotaDeploymentJob>, Error> {
    fuota_deployment_job::dsl::fuota_deployment_job
        .filter(
            fuota_deployment_job::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .order_by(fuota_deployment_job::dsl::created_at)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, fuota_deployment_id.to_string()))
}

// Selected jobs will automatically have their scheduler_run_after column updated to now + 2 x scheduler interval value.
// This is such that concurrent queries will not result in the same job being executed twice.
pub async fn get_schedulable_jobs(limit: usize) -> Result<Vec<FuotaDeploymentJob>> {
    let mut c = get_async_db_conn().await?;
    db_transaction::<Vec<FuotaDeploymentJob>, Error, _>(&mut c, |c| {
        Box::pin(async move {
            let conf = config::get();
            diesel::sql_query(if cfg!(feature = "sqlite") {
                r#"
                    update
                        fuota_deployment_job
                    set
                        scheduler_run_after = ?3
                    where
                        (fuota_deployment_id, job) in (
                            select
                                fuota_deployment_id,
                                job
                            from
                                fuota_deployment_job
                            where
                                completed_at is null
                                and scheduler_run_after <= ?2
                            order by
                                created_at
                            limit ?1
                        )
                    returning *
                "#
            } else {
                r#"
                    update
                        fuota_deployment_job
                    set
                        scheduler_run_after = $3
                    where
                        (fuota_deployment_id, job) in (
                            select
                                fuota_deployment_id,
                                job
                            from
                                fuota_deployment_job
                            where
                                completed_at is null
                                and scheduler_run_after <= $2
                            order by
                                created_at
                            limit $1
                        )
                    returning *
                "#
            })
            .bind::<diesel::sql_types::Integer, _>(limit as i32)
            .bind::<fields::sql_types::Timestamptz, _>(Utc::now())
            .bind::<fields::sql_types::Timestamptz, _>(
                Utc::now() + Duration::from_std(2 * conf.network.scheduler.interval).unwrap(),
            )
            .load(c)
            .await
            .map_err(|e| Error::from_diesel(e, "".into()))
        })
    })
    .await
    .context("Get FUOTA jobs")
}

pub async fn get_max_fragment_size(d: &FuotaDeployment) -> Result<usize> {
    let dp = device_profile::get(&d.device_profile_id).await?;
    let region_conf = lrwn::region::get(dp.region, false, false);
    let max_pl_size = region_conf
        .get_max_payload_size(dp.mac_version, dp.reg_params_revision, d.multicast_dr as u8)?
        .n
        - 3;

    Ok(if max_pl_size > d.payload.len() {
        d.payload.len()
    } else {
        max_pl_size
    })
}

pub fn get_multicast_timeout(d: &FuotaDeployment) -> Result<usize> {
    let conf = config::get();

    let fragments = (d.payload.len() as f32 / d.fragmentation_fragment_size as f32).ceil() as usize;
    let redundancy =
        (fragments as f32 * d.fragmentation_redundancy_percentage as f32 / 100.0).ceil() as usize;
    let total_fragments = fragments + redundancy;

    match d.multicast_group_type.as_ref() {
        "B" => {
            // Calculate number of ping-slots per beacon period.
            let nb_ping_slots = 1 << (7 - d.multicast_class_b_ping_slot_periodicity as usize);

            // Calculate number of beacon-periods needed.
            // One beacon period is added as the first ping-slot might be in the next beacon-period.
            let beacon_periods =
                (total_fragments as f32 / nb_ping_slots as f32).ceil() as usize + 1;

            // Calculate the timeout value. In case of Class-B, timeout represents the number
            // of beacon periods (beacon periods = 2^timeout).
            for i in 0..16 {
                // i is 0-15
                if (1 << i) >= beacon_periods {
                    return Ok(i);
                }
            }

            Err(anyhow!("Max. number of beacon period exceeded"))
        }
        "C" => {
            // Get the margin between each multicast Class-C downlink.
            let mc_class_c_margin_secs =
                conf.network.scheduler.multicast_class_c_margin.as_secs() as usize;

            // Multiply by the number of fragments (+1 for additional margin).
            let mc_class_c_duration_secs = mc_class_c_margin_secs * (total_fragments + 1);

            // Calculate the timeout value. In case of Class-C, timeout is defined as seconds,
            // where the number of seconds is 2^timeout.
            for i in 0..16 {
                // i = 0-15
                if (1 << i) >= mc_class_c_duration_secs {
                    return Ok(i);
                }
            }

            Err(anyhow!("Max timeout exceeded"))
        }
        _ => Ok(0),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::{application, device, device_profile, gateway, tenant};
    use crate::test;

    #[tokio::test]
    async fn test_fuota() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create
        let mut d = create_deployment(FuotaDeployment {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-fuota-deployment".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let d_get = get_deployment(d.id.into()).await.unwrap();
        assert_eq!(d, d_get);

        // update
        d.name = "updated-test-fuota-deployment".into();
        let d = update_deployment(d).await.unwrap();

        // count
        assert_eq!(1, get_deployment_count(app.id.into()).await.unwrap());

        // list
        assert_eq!(
            vec![FuotaDeploymentListItem {
                id: d.id,
                created_at: d.created_at,
                updated_at: d.updated_at,
                started_at: None,
                completed_at: None,
                name: d.name.clone(),
            }],
            list_deployments(app.id.into(), 10, 0).await.unwrap()
        );

        // delete
        delete_deployment(d.id.into()).await.unwrap();
        assert!(delete_deployment(d.id.into()).await.is_err());
    }

    #[tokio::test]
    async fn test_fuota_devices() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let app2 = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let dp2 = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let dev = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-device".into(),
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            ..Default::default()
        })
        .await
        .unwrap();

        let dev2 = device::create(device::Device {
            application_id: app.id,
            device_profile_id: dp2.id,
            name: "test-device".into(),
            dev_eui: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
            ..Default::default()
        })
        .await
        .unwrap();

        let dev3 = device::create(device::Device {
            application_id: app2.id,
            device_profile_id: dp.id,
            name: "test-device".into(),
            dev_eui: EUI64::from_be_bytes([3, 2, 3, 4, 5, 6, 7, 8]),
            ..Default::default()
        })
        .await
        .unwrap();

        // create
        let d = create_deployment(FuotaDeployment {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-fuota-deployment".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // can't add devices from multiple device-profiles
        assert!(add_devices(d.id.into(), vec![dev2.dev_eui]).await.is_err());

        // can't add devices from other applications
        assert!(add_devices(d.id.into(), vec![dev3.dev_eui]).await.is_err());

        // add devices
        add_devices(d.id.into(), vec![dev.dev_eui]).await.unwrap();

        // get device count
        assert_eq!(1, get_device_count(d.id.into()).await.unwrap());

        // get devices
        let devices = get_devices(d.id.into(), 10, 0).await.unwrap();
        assert_eq!(1, devices.len());
        assert_eq!(dev.dev_eui, devices[0].dev_eui);
        assert_eq!(d.id, devices[0].fuota_deployment_id);

        // get device
        let mut devices = get_devices(d.id.into(), 1, 0).await.unwrap();
        devices[0].error_msg = "Error: kaboom".into();
        let fuota_d = update_device(devices[0].clone()).await.unwrap();
        assert_eq!("Error: kaboom", fuota_d.error_msg);

        // remove devices
        remove_devices(d.id.into(), vec![dev.dev_eui])
            .await
            .unwrap();
        assert_eq!(0, get_device_count(d.id.into()).await.unwrap());
    }

    #[tokio::test]
    async fn test_fuota_gateways() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            can_have_gateways: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let t2 = tenant::create(tenant::Tenant {
            name: "test-tenant-2".into(),
            can_have_gateways: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let d = create_deployment(FuotaDeployment {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-fuota-deployment".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let gw = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            name: "gw-1".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let gw2 = gateway::create(gateway::Gateway {
            gateway_id: EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
            name: "gw-2".into(),
            tenant_id: t2.id,
            ..Default::default()
        })
        .await
        .unwrap();

        // adding gateteway from other tenant fails
        assert!(add_gateways(d.id.into(), vec![gw2.gateway_id])
            .await
            .is_err());

        // add gateway
        add_gateways(d.id.into(), vec![gw.gateway_id])
            .await
            .unwrap();

        // get count
        assert_eq!(1, get_gateway_count(d.id.into()).await.unwrap());

        // get gateways
        let gateways = get_gateways(d.id.into(), 10, 0).await.unwrap();
        assert_eq!(1, gateways.len());

        // remove gateways
        remove_gateways(d.id.into(), vec![gw.gateway_id])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_jobs() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create
        let d = create_deployment(FuotaDeployment {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-fuota-deployment".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create job
        let mut job = create_job(FuotaDeploymentJob {
            fuota_deployment_id: d.id,
            job: fields::FuotaJob::McGroupSetup,
            max_retry_count: 3,
            attempt_count: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        // list jobs
        let jobs = list_jobs(d.id.into()).await.unwrap();
        assert_eq!(vec![job.clone()], jobs);

        // update job
        job.attempt_count = 2;
        let job = update_job(job).await.unwrap();
        let jobs = list_jobs(d.id.into()).await.unwrap();
        assert_eq!(vec![job.clone()], jobs);

        // create new job
        // we expect that this sets the previous one as completed
        let job2 = create_job(FuotaDeploymentJob {
            fuota_deployment_id: d.id,
            job: fields::FuotaJob::FragStatus,
            max_retry_count: 3,
            attempt_count: 1,
            ..Default::default()
        })
        .await
        .unwrap();

        let jobs = list_jobs(d.id.into()).await.unwrap();
        assert_eq!(2, jobs.len());
        assert_eq!(job.job, jobs[0].job);
        assert!(jobs[0].completed_at.is_some());
        assert_eq!(job2.job, jobs[1].job);
        assert!(jobs[1].completed_at.is_none());

        // get schedulable jobs
        let jobs = get_schedulable_jobs(10).await.unwrap();
        assert_eq!(1, jobs.len());
        assert_eq!(job2.job, jobs[0].job);

        let jobs = get_schedulable_jobs(10).await.unwrap();
        assert_eq!(0, jobs.len());
    }

    #[tokio::test]
    async fn test_get_max_fragment_size() {
        let _guard = test::prepare().await;

        let t = tenant::create(tenant::Tenant {
            name: "test-tenant".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        let app = application::create(application::Application {
            name: "test-app".into(),
            tenant_id: t.id,
            ..Default::default()
        })
        .await
        .unwrap();

        let dp = device_profile::create(device_profile::DeviceProfile {
            tenant_id: t.id,
            name: "test-dp".into(),
            ..Default::default()
        })
        .await
        .unwrap();

        // create
        let d = create_deployment(FuotaDeployment {
            application_id: app.id,
            device_profile_id: dp.id,
            name: "test-fuota-deployment".into(),
            multicast_dr: 5,
            payload: vec![0; 1000],
            ..Default::default()
        })
        .await
        .unwrap();

        assert_eq!(239, get_max_fragment_size(&d).await.unwrap());
    }

    #[tokio::test]
    async fn test_get_multicast_timeout() {
        let _guard = test::prepare().await;

        struct Test {
            name: String,
            deployment: FuotaDeployment,
            expected_timeout: usize,
            expected_error: Option<String>,
        }

        let tests = [
            Test {
                name: "Class-B - 1 / beacon period - 15 fragments".into(),
                deployment: FuotaDeployment {
                    multicast_group_type: "B".into(),
                    multicast_class_b_ping_slot_periodicity: 7,
                    fragmentation_fragment_size: 10,
                    fragmentation_redundancy_percentage: 50,
                    payload: vec![0; 100],
                    ..Default::default()
                },
                expected_timeout: 4,
                expected_error: None,
            },
            Test {
                name: "Class-B - 1 / beacon period - 16 fragments".into(),
                deployment: FuotaDeployment {
                    multicast_group_type: "B".into(),
                    multicast_class_b_ping_slot_periodicity: 7,
                    fragmentation_fragment_size: 10,
                    fragmentation_redundancy_percentage: 60,
                    payload: vec![0; 100],
                    ..Default::default()
                },
                expected_timeout: 5,
                expected_error: None,
            },
            Test {
                name: "Class-B - 16 / beacon period - 16 fragments".into(),
                deployment: FuotaDeployment {
                    multicast_group_type: "B".into(),
                    multicast_class_b_ping_slot_periodicity: 3,
                    fragmentation_fragment_size: 10,
                    fragmentation_redundancy_percentage: 60,
                    payload: vec![0; 100],
                    ..Default::default()
                },
                expected_timeout: 1,
                expected_error: None,
            },
            Test {
                name: "Class-B - 16 / beacon period - 17 fragments".into(),
                deployment: FuotaDeployment {
                    multicast_group_type: "B".into(),
                    multicast_class_b_ping_slot_periodicity: 3,
                    fragmentation_fragment_size: 10,
                    fragmentation_redundancy_percentage: 70,
                    payload: vec![0; 100],
                    ..Default::default()
                },
                expected_timeout: 2,
                expected_error: None,
            },
            Test {
                name: "Class-C - 1 fragment".into(),
                deployment: FuotaDeployment {
                    multicast_group_type: "C".into(),
                    fragmentation_fragment_size: 10,
                    payload: vec![0; 10],
                    ..Default::default()
                },
                expected_timeout: 4,
                expected_error: None,
            },
        ];

        for t in &tests {
            println!("> {}", t.name);
            let res = get_multicast_timeout(&t.deployment);
            if let Some(err_str) = &t.expected_error {
                assert!(res.is_err());
                assert_eq!(err_str, &res.err().unwrap().to_string());
            } else {
                assert!(res.is_ok());
                assert_eq!(t.expected_timeout, res.unwrap());
            }
        }
    }
}
