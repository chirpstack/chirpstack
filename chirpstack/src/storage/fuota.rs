use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{dsl, prelude::*};
use diesel_async::RunQueryDsl;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use crate::storage::error::Error;
use crate::storage::schema::{
    application, device, fuota_deployment, fuota_deployment_device, fuota_deployment_gateway,
    gateway, tenant,
};
use crate::storage::{self, device_profile, fields, get_async_db_conn};
use lrwn::EUI64;

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
    pub multicast_group_type: String,
    pub multicast_class_c_scheduling_type: fields::MulticastGroupSchedulingType,
    pub multicast_dr: i16,
    pub multicast_class_b_ping_slot_nb_k: i16,
    pub multicast_frequency: i64,
    pub multicast_timeout: i16,
    pub unicast_attempt_count: i16,
    pub fragmentation_fragment_size: i16,
    pub fragmentation_redundancy: i16,
    pub fragmentation_session_index: i16,
    pub fragmentation_matrix: i16,
    pub fragmentation_block_ack_delay: i16,
    pub fragmentation_descriptor: Vec<u8>,
    pub request_fragmentation_session_status: fields::RequestFragmentationSessionStatus,
    pub payload: Vec<u8>,
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
            multicast_group_type: "".into(),
            multicast_class_c_scheduling_type: fields::MulticastGroupSchedulingType::DELAY,
            multicast_dr: 0,
            multicast_class_b_ping_slot_nb_k: 0,
            multicast_frequency: 0,
            multicast_timeout: 0,
            unicast_attempt_count: 0,
            fragmentation_fragment_size: 0,
            fragmentation_redundancy: 0,
            fragmentation_session_index: 0,
            fragmentation_matrix: 0,
            fragmentation_block_ack_delay: 0,
            fragmentation_descriptor: Vec::new(),
            request_fragmentation_session_status:
                fields::RequestFragmentationSessionStatus::NoRequest,
            payload: Vec::new(),
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
    pub updated_at: DateTime<Utc>,
    pub mc_group_setup_completed_at: Option<DateTime<Utc>>,
    pub mc_session_completed_at: Option<DateTime<Utc>>,
    pub frag_session_setup_completed_at: Option<DateTime<Utc>>,
    pub frag_status_completed_at: Option<DateTime<Utc>>,
}

impl Default for FuotaDeploymentDevice {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            fuota_deployment_id: Uuid::nil().into(),
            dev_eui: EUI64::default(),
            created_at: now,
            updated_at: now,
            mc_group_setup_completed_at: None,
            mc_session_completed_at: None,
            frag_session_setup_completed_at: None,
            frag_status_completed_at: None,
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
            fuota_deployment::multicast_class_b_ping_slot_nb_k
                .eq(&d.multicast_class_b_ping_slot_nb_k),
            fuota_deployment::multicast_frequency.eq(&d.multicast_frequency),
            fuota_deployment::multicast_timeout.eq(&d.multicast_timeout),
            fuota_deployment::unicast_attempt_count.eq(&d.unicast_attempt_count),
            fuota_deployment::fragmentation_fragment_size.eq(&d.fragmentation_fragment_size),
            fuota_deployment::fragmentation_redundancy.eq(&d.fragmentation_redundancy),
            fuota_deployment::fragmentation_session_index.eq(&d.fragmentation_session_index),
            fuota_deployment::fragmentation_matrix.eq(&d.fragmentation_matrix),
            fuota_deployment::fragmentation_block_ack_delay.eq(&d.fragmentation_block_ack_delay),
            fuota_deployment::fragmentation_descriptor.eq(&d.fragmentation_descriptor),
            fuota_deployment::request_fragmentation_session_status
                .eq(&d.request_fragmentation_session_status),
            fuota_deployment::payload.eq(&d.payload),
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
                dev_eui: dev_eui,
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
        Err(Error::MultiError(errors))
    }
}

pub async fn get_devices(
    fuota_deployment_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<FuotaDeploymentDevice>, Error> {
    fuota_deployment_device::dsl::fuota_deployment_device
        .filter(
            fuota_deployment_device::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .order_by(fuota_deployment_device::dsl::dev_eui)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
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
                gateway_id: gateway_id,
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
        Err(Error::MultiError(errors))
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

pub async fn get_gateway(
    fuota_deployment_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<FuotaDeploymentGateway>, Error> {
    fuota_deployment_gateway::dsl::fuota_deployment_gateway
        .filter(
            fuota_deployment_gateway::dsl::fuota_deployment_id
                .eq(fields::Uuid::from(fuota_deployment_id)),
        )
        .order_by(fuota_deployment_gateway::dsl::gateway_id)
        .limit(limit)
        .offset(offset)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, "".into()))
}
