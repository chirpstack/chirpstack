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
    fuota_deployment_job, gateway, tenant,
};
use crate::storage::{self, db_transaction, device_profile, fields, get_async_db_conn};
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

#[derive(Clone, Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = fuota_deployment_job)]
pub struct FuotaDeploymentJob {
    pub fuota_deployment_id: fields::Uuid,
    pub job: fields::FuotaJob,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub max_attempt_count: i16,
    pub attempt_count: i16,
    pub scheduler_run_after: DateTime<Utc>,
}

impl Default for FuotaDeploymentJob {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            fuota_deployment_id: Uuid::nil().into(),
            job: fields::FuotaJob::McGroupSetup,
            created_at: now,
            completed_at: None,
            max_attempt_count: 0,
            attempt_count: 0,
            scheduler_run_after: now,
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

pub async fn get_gateways(
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
        .order_by(fuota_deployment_job::dsl::scheduler_run_after)
        .load(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, fuota_deployment_id.to_string()))
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
            max_attempt_count: 3,
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
            max_attempt_count: 3,
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
    }
}
