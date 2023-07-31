use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use tokio::task;
use tracing::info;

use lrwn::{AES128Key, EUI64};

use super::error::Error;
use super::get_db_conn;
use super::schema::device_keys;

#[derive(Queryable, Insertable, AsChangeset, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = device_keys)]
pub struct DeviceKeys {
    pub dev_eui: EUI64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nwk_key: AES128Key,
    pub app_key: AES128Key,
    pub dev_nonces: Vec<Option<i32>>,
    pub join_nonce: i32,
}

impl Default for DeviceKeys {
    fn default() -> Self {
        let now = Utc::now();

        DeviceKeys {
            dev_eui: EUI64::from_be_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            created_at: now,
            updated_at: now,
            nwk_key: AES128Key::from_bytes([
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]),
            app_key: AES128Key::from_bytes([
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ]),
            dev_nonces: Vec::new(),
            join_nonce: 0,
        }
    }
}

pub async fn create(dk: DeviceKeys) -> Result<DeviceKeys, Error> {
    let dk = task::spawn_blocking({
        move || -> Result<DeviceKeys, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(device_keys::table)
                .values(&dk)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dk.dev_eui.to_string()))
        }
    })
    .await??;
    info!(
        dev_eui = %dk.dev_eui,
        "Device-keys created"
    );
    Ok(dk)
}

pub async fn get(dev_eui: &EUI64) -> Result<DeviceKeys, Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<DeviceKeys, Error> {
            let mut c = get_db_conn()?;
            let dk = device_keys::dsl::device_keys
                .find(&dev_eui)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
            Ok(dk)
        }
    })
    .await?
}

pub async fn update(dk: DeviceKeys) -> Result<DeviceKeys, Error> {
    let dk = task::spawn_blocking({
        move || -> Result<DeviceKeys, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device_keys::dsl::device_keys.find(&dk.dev_eui))
                .set(&dk)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dk.dev_eui.to_string()))
        }
    })
    .await??;
    info!(
        dev_eui = %dk.dev_eui,
        "Device-keys updated"
    );
    Ok(dk)
}

pub async fn delete(dev_eui: &EUI64) -> Result<(), Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra =
                diesel::delete(device_keys::dsl::device_keys.find(&dev_eui)).execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(dev_eui.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(
        dev_eui = %dev_eui,
        "Device-keys deleted"
    );
    Ok(())
}

pub async fn set_dev_nonces(dev_eui: &EUI64, nonces: &[i32]) -> Result<DeviceKeys, Error> {
    let dk = task::spawn_blocking({
        let dev_eui = *dev_eui;
        let nonces = nonces.to_vec();
        move || -> Result<DeviceKeys, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device_keys::dsl::device_keys.find(&dev_eui))
                .set(device_keys::dev_nonces.eq(&nonces))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(
        dev_eui = %dev_eui,
        "Dev-nonces updated"
    );
    Ok(dk)
}

pub async fn validate_incr_join_and_store_dev_nonce(
    dev_eui: &EUI64,
    dev_nonce: i32,
) -> Result<DeviceKeys, Error> {
    let dk = task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<DeviceKeys, Error> {
            let mut c = get_db_conn()?;
            c.transaction::<DeviceKeys, Error, _>(|c| {
                let mut dk: DeviceKeys = device_keys::dsl::device_keys
                    .find(&dev_eui)
                    .for_update()
                    .first(c)
                    .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;

                if dk.dev_nonces.contains(&(Some(dev_nonce))) {
                    return Err(Error::InvalidDevNonce);
                }

                dk.dev_nonces.push(Some(dev_nonce));
                dk.join_nonce += 1;

                diesel::update(device_keys::dsl::device_keys.find(&dev_eui))
                    .set((
                        device_keys::updated_at.eq(Utc::now()),
                        device_keys::dev_nonces.eq(&dk.dev_nonces),
                        device_keys::join_nonce.eq(&dk.join_nonce),
                    ))
                    .get_result(c)
                    .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
            })
        }
    })
    .await??;

    info!(dev_eui = %dev_eui, dev_nonce = dev_nonce, "Device-nonce validated, join-nonce incremented and stored");
    Ok(dk)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage;
    use crate::test;

    pub async fn reset_nonces(dev_eui: &EUI64) -> Result<DeviceKeys, Error> {
        let dk = task::spawn_blocking({
            let dev_eui = *dev_eui;
            move || -> Result<DeviceKeys, Error> {
                let mut c = get_db_conn()?;
                diesel::update(device_keys::dsl::device_keys.find(&dev_eui))
                    .set((
                        device_keys::dev_nonces.eq::<Vec<i32>>(Vec::new()),
                        device_keys::join_nonce.eq(0),
                    ))
                    .get_result(&mut c)
                    .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
            }
        })
        .await??;
        info!(
            dev_eui = %dev_eui,
            "Nonces reset"
        );
        Ok(dk)
    }

    pub async fn create_device_keys(dev_eui: Option<EUI64>) -> DeviceKeys {
        let dev_eui = match dev_eui {
            Some(v) => v,
            None => {
                let dp = storage::device_profile::test::create_device_profile(None).await;
                let a = storage::application::test::create_application(None).await;
                let d = storage::device::Device {
                    name: "test-dev".into(),
                    dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
                    application_id: a.id,
                    device_profile_id: dp.id,
                    ..Default::default()
                };

                let d = storage::device::create(d).await.unwrap();
                d.dev_eui
            }
        };

        let dk = DeviceKeys {
            dev_eui: dev_eui,
            ..Default::default()
        };

        create(dk).await.unwrap()
    }

    #[tokio::test]
    async fn test_device_keys() {
        let _guard = test::prepare().await;
        let mut dk = create_device_keys(None).await;

        // get
        let dk_get = get(&dk.dev_eui).await.unwrap();
        assert_eq!(dk, dk_get);

        // update
        dk.join_nonce = 10;
        dk = update(dk).await.unwrap();
        let dk_get = get(&dk.dev_eui).await.unwrap();
        assert_eq!(dk, dk_get);

        // delete
        delete(&dk.dev_eui).await.unwrap();
        assert_eq!(true, delete(&dk.dev_eui).await.is_err());
    }
}
