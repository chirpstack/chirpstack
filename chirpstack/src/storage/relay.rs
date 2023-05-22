use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::dsl;
use diesel::prelude::*;
use tokio::task;
use tracing::info;
use uuid::Uuid;

use lrwn::{DevAddr, EUI64};

use super::schema::{device, device_profile, relay_device};
use super::{device::Device, error::Error, get_db_conn};

// This is set to 15, because the FilterList must contain a "catch-all" record to filter all
// uplinks that do not match the remaining records. This means that we can use 16 - 1 FilterList
// entries effectively.
const RELAY_MAX_DEVICES: i64 = 15;

#[derive(Default, Clone)]
pub struct RelayFilters {
    pub application_id: Option<Uuid>,
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct RelayListItem {
    pub dev_eui: EUI64,
    pub name: String,
}

#[derive(Default, Clone)]
pub struct DeviceFilters {
    pub relay_dev_eui: Option<EUI64>,
}

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct DeviceListItem {
    pub dev_eui: EUI64,
    pub join_eui: EUI64,
    pub dev_addr: Option<DevAddr>,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub relay_ed_uplink_limit_bucket_size: i16,
    pub relay_ed_uplink_limit_reload_rate: i16,
}

pub async fn get_relay_count(filters: &RelayFilters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = device::dsl::device
                .select(dsl::count_star())
                .inner_join(device_profile::table)
                .filter(device_profile::dsl::is_relay.eq(true))
                .into_boxed();

            if let Some(application_id) = &filters.application_id {
                q = q.filter(device::dsl::application_id.eq(application_id));
            }

            Ok(q.first(&mut c)?)
        }
    })
    .await?
}

pub async fn list_relays(
    limit: i64,
    offset: i64,
    filters: &RelayFilters,
) -> Result<Vec<RelayListItem>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<RelayListItem>, Error> {
            let mut c = get_db_conn()?;
            let mut q = device::dsl::device
                .inner_join(device_profile::table)
                .select((device::dev_eui, device::name))
                .filter(device_profile::dsl::is_relay.eq(true))
                .into_boxed();

            if let Some(application_id) = &filters.application_id {
                q = q.filter(device::dsl::application_id.eq(application_id));
            }

            q.order_by(device::dsl::name)
                .limit(limit)
                .offset(offset)
                .load(&mut c)
                .map_err(|e| Error::from_diesel(e, "".into()))
        }
    })
    .await?
}

pub async fn get_device_count(filters: &DeviceFilters) -> Result<i64, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<i64, Error> {
            let mut c = get_db_conn()?;
            let mut q = relay_device::dsl::relay_device
                .select(dsl::count_star())
                .into_boxed();

            if let Some(relay_dev_eui) = &filters.relay_dev_eui {
                q = q.filter(relay_device::dsl::relay_dev_eui.eq(relay_dev_eui));
            }

            q.first(&mut c)
                .map_err(|e| Error::from_diesel(e, "".into()))
        }
    })
    .await?
}

pub async fn list_devices(
    limit: i64,
    offset: i64,
    filters: &DeviceFilters,
) -> Result<Vec<DeviceListItem>, Error> {
    task::spawn_blocking({
        let filters = filters.clone();
        move || -> Result<Vec<DeviceListItem>, Error> {
            let mut c = get_db_conn()?;
            let mut q = relay_device::dsl::relay_device
                .inner_join(device::table.on(relay_device::dsl::dev_eui.eq(device::dsl::dev_eui)))
                .inner_join(
                    device_profile::table
                        .on(device::dsl::device_profile_id.eq(device_profile::dsl::id)),
                )
                .select((
                    relay_device::dev_eui,
                    device::join_eui,
                    device::dev_addr,
                    relay_device::created_at,
                    device::name,
                    device_profile::relay_ed_uplink_limit_bucket_size,
                    device_profile::relay_ed_uplink_limit_reload_rate,
                ))
                .into_boxed();

            if let Some(relay_dev_eui) = &filters.relay_dev_eui {
                q = q.filter(relay_device::dsl::relay_dev_eui.eq(relay_dev_eui));
            }

            q.order_by(device::dsl::name)
                .limit(limit)
                .offset(offset)
                .load(&mut c)
                .map_err(|e| Error::from_diesel(e, "".into()))
        }
    })
    .await?
}

pub async fn add_device(relay_dev_eui: EUI64, device_dev_eui: EUI64) -> Result<(), Error> {
    task::spawn_blocking({
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            c.transaction::<(), Error, _>(|c| {
                // We lock the relay device to avoid race-conditions in the validation.
                let rd: Device = device::dsl::device
                    .find(&relay_dev_eui)
                    .for_update()
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, relay_dev_eui.to_string()))?;

                // Is the given relay_dev_eui a Relay?
                let rdp: super::device_profile::DeviceProfile = device_profile::dsl::device_profile
                    .find(&rd.device_profile_id)
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, rd.device_profile_id.to_string()))?;
                if !rdp.is_relay {
                    return Err(Error::Validation("Device is not a relay".to_string()));
                }

                // Validate that relay and device are under the same application.
                let d: Device = device::dsl::device
                    .find(&device_dev_eui)
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, device_dev_eui.to_string()))?;

                if rd.application_id != d.application_id {
                    return Err(Error::Validation(
                        "Relay and device must be under the same application".into(),
                    ));
                }

                // Validate that the relay and device are under the same region.
                let dp: super::device_profile::DeviceProfile = device_profile::dsl::device_profile
                    .find(&d.device_profile_id)
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, d.device_profile_id.to_string()))?;
                if rdp.region != dp.region {
                    return Err(Error::Validation(
                        "Relay and device must be under the same region".into(),
                    ));
                }

                // Validate that the device is not a relay.
                if dp.is_relay {
                    return Err(Error::Validation("Can not add relay to a relay".into()));
                }

                // Validate max. number of devices.
                let count: i64 = relay_device::dsl::relay_device
                    .select(dsl::count_star())
                    .filter(relay_device::dsl::relay_dev_eui.eq(&relay_dev_eui))
                    .first(c)
                    .map_err(|e| Error::from_diesel(e, "".into()))?;

                if count > RELAY_MAX_DEVICES {
                    return Err(Error::Validation(format!(
                        "Max number of devices that can be added to a relay is {}",
                        RELAY_MAX_DEVICES
                    )));
                }

                let _ = diesel::insert_into(relay_device::table)
                    .values((
                        relay_device::relay_dev_eui.eq(&relay_dev_eui),
                        relay_device::dev_eui.eq(&device_dev_eui),
                        relay_device::created_at.eq(Utc::now()),
                    ))
                    .execute(c)
                    .map_err(|e| Error::from_diesel(e, "".into()))?;

                Ok(())
            })
        }
    })
    .await??;

    info!(relay_dev_eui = %relay_dev_eui, device_dev_eui = %device_dev_eui, "Device added to relay");

    Ok(())
}

pub async fn remove_device(relay_dev_eui: EUI64, device_dev_eui: EUI64) -> Result<(), Error> {
    task::spawn_blocking({
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(
                relay_device::dsl::relay_device
                    .filter(relay_device::relay_dev_eui.eq(&relay_dev_eui))
                    .filter(relay_device::dev_eui.eq(&device_dev_eui)),
            )
            .execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(format!(
                    "relay_dev_eui: {}, device_dev_eui: {}",
                    relay_dev_eui, device_dev_eui
                )));
            }
            Ok(())
        }
    })
    .await??;

    info!(relay_dev_eui = %relay_dev_eui, device_dev_eui = %device_dev_eui, "Device removed from relay");

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage;
    use crate::test;

    #[tokio::test]
    async fn test_relay() {
        let _guard = test::prepare().await;

        let dp = storage::device_profile::test::create_device_profile(None).await;
        let dp_relay = storage::device_profile::create(storage::device_profile::DeviceProfile {
            tenant_id: dp.tenant_id,
            name: "relay".into(),
            is_relay: true,
            ..Default::default()
        })
        .await
        .unwrap();

        let d = storage::device::test::create_device(
            EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            None,
        )
        .await;

        let d_relay = storage::device::test::create_device(
            EUI64::from_be_bytes([2, 2, 3, 4, 5, 6, 7, 8]),
            dp_relay.id,
            Some(d.application_id),
        )
        .await;

        let d_other_app = storage::device::test::create_device(
            EUI64::from_be_bytes([3, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            None,
        )
        .await;

        let d_other_same_app = storage::device::test::create_device(
            EUI64::from_be_bytes([4, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            Some(d.application_id),
        )
        .await;

        // relay count
        let relay_count = get_relay_count(&RelayFilters {
            application_id: Some(d_relay.application_id),
        })
        .await
        .unwrap();
        assert_eq!(1, relay_count);

        // relay list
        let relay_list = list_relays(
            10,
            0,
            &RelayFilters {
                application_id: Some(d_relay.application_id),
            },
        )
        .await
        .unwrap();
        assert_eq!(1, relay_list.len());
        assert_eq!(d_relay.dev_eui, relay_list[0].dev_eui);

        // get device count (no devices)
        let device_count = get_device_count(&DeviceFilters {
            relay_dev_eui: Some(d_relay.dev_eui),
        })
        .await
        .unwrap();
        assert_eq!(0, device_count);

        // device list (no devices)
        let device_list = list_devices(
            10,
            0,
            &DeviceFilters {
                relay_dev_eui: Some(d_relay.dev_eui),
            },
        )
        .await
        .unwrap();
        assert_eq!(0, device_list.len());

        // add device from other app errors
        assert!(add_device(d_relay.dev_eui, d_other_app.dev_eui)
            .await
            .is_err());

        // add relay to relay errors
        assert!(add_device(d_relay.dev_eui, d_relay.dev_eui).await.is_err());

        // add to device that isn't relay errors
        assert!(add_device(d_other_same_app.dev_eui, d.dev_eui)
            .await
            .is_err());

        // add device to relay
        add_device(d_relay.dev_eui, d.dev_eui).await.unwrap();

        // get device count
        let device_count = get_device_count(&DeviceFilters {
            relay_dev_eui: Some(d_relay.dev_eui),
        })
        .await
        .unwrap();
        assert_eq!(1, device_count);

        // device list
        let device_list = list_devices(
            10,
            0,
            &DeviceFilters {
                relay_dev_eui: Some(d_relay.dev_eui),
            },
        )
        .await
        .unwrap();
        assert_eq!(1, device_list.len());
        assert_eq!(d.dev_eui, device_list[0].dev_eui);

        // remove device
        remove_device(d_relay.dev_eui, d.dev_eui).await.unwrap();
        assert!(remove_device(d_relay.dev_eui, d.dev_eui).await.is_err());
    }
}
