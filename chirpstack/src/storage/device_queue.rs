use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::{dsl, prelude::*};
use tokio::task;
use tracing::info;
use uuid::Uuid;

use super::error::Error;
use super::get_db_conn;
use super::schema::device_queue_item;
use lrwn::EUI64;

#[derive(Queryable, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = device_queue_item)]
pub struct DeviceQueueItem {
    pub id: Uuid,
    pub dev_eui: EUI64,
    pub created_at: DateTime<Utc>,
    pub f_port: i16,
    pub confirmed: bool,
    pub data: Vec<u8>,
    pub is_pending: bool,
    pub f_cnt_down: Option<i64>,
    pub timeout_after: Option<DateTime<Utc>>,
    pub is_encrypted: bool,
}

impl DeviceQueueItem {
    fn validate(&self) -> Result<(), Error> {
        if self.f_port == 0 || self.f_port > 255 {
            return Err(Error::Validation(
                "FPort must be between 1 - 255".to_string(),
            ));
        }

        if self.is_encrypted && self.f_cnt_down.is_none() {
            return Err(Error::Validation(
                "FCntDown must be set for encrypted queue-items".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for DeviceQueueItem {
    fn default() -> Self {
        let now = Utc::now();

        DeviceQueueItem {
            id: Uuid::new_v4(),
            dev_eui: EUI64::from_be_bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            created_at: now,
            f_port: 0,
            confirmed: false,
            data: Vec::new(),
            is_pending: false,
            f_cnt_down: None,
            timeout_after: None,
            is_encrypted: false,
        }
    }
}

pub async fn enqueue_item(qi: DeviceQueueItem) -> Result<DeviceQueueItem, Error> {
    qi.validate()?;

    let qi = task::spawn_blocking({
        move || -> Result<DeviceQueueItem, Error> {
            let mut c = get_db_conn()?;
            diesel::insert_into(device_queue_item::table)
                .values(&qi)
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, qi.id.to_string()))
        }
    })
    .await??;
    info!(id = %qi.id, dev_eui = %qi.dev_eui, "Device queue-item enqueued");
    Ok(qi)
}

pub async fn get_item(id: &Uuid) -> Result<DeviceQueueItem, Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<DeviceQueueItem, Error> {
            let mut c = get_db_conn()?;
            let qi = device_queue_item::dsl::device_queue_item
                .find(&id)
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, id.to_string()))?;
            Ok(qi)
        }
    })
    .await?
}

pub async fn update_item(qi: DeviceQueueItem) -> Result<DeviceQueueItem, Error> {
    let qi = task::spawn_blocking({
        move || -> Result<DeviceQueueItem, Error> {
            let mut c = get_db_conn()?;
            diesel::update(device_queue_item::dsl::device_queue_item.find(&qi.id))
                .set((
                    device_queue_item::is_pending.eq(&qi.is_pending),
                    device_queue_item::f_cnt_down.eq(&qi.f_cnt_down),
                    device_queue_item::timeout_after.eq(&qi.timeout_after),
                ))
                .get_result(&mut c)
                .map_err(|e| Error::from_diesel(e, qi.id.to_string()))
        }
    })
    .await??;
    info!(id = %qi.id, dev_eui = %qi.dev_eui, "Device queue-item updated");
    Ok(qi)
}

pub async fn delete_item(id: &Uuid) -> Result<(), Error> {
    task::spawn_blocking({
        let id = *id;
        move || -> Result<(), Error> {
            let mut c = get_db_conn()?;
            let ra = diesel::delete(device_queue_item::dsl::device_queue_item.find(&id))
                .execute(&mut c)?;
            if ra == 0 {
                return Err(Error::NotFound(id.to_string()));
            }
            Ok(())
        }
    })
    .await??;
    info!(id = %id, "Device queue-item deleted");
    Ok(())
}

/// It returns the device queue-item and a bool indicating if there are more items in the queue.
pub async fn get_next_for_dev_eui(dev_eui: &EUI64) -> Result<(DeviceQueueItem, bool), Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<(DeviceQueueItem, bool), Error> {
            let mut c = get_db_conn()?;
            let items: Vec<DeviceQueueItem> = device_queue_item::dsl::device_queue_item
                .filter(device_queue_item::dev_eui.eq(&dev_eui))
                .order_by(device_queue_item::created_at)
                .limit(2)
                .load(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;

            // Return NotFound on empty Vec.
            if items.is_empty() {
                return Err(Error::NotFound(dev_eui.to_string()));
            }

            // In case the transmission is pending and hasn't timed-out yet, do not
            // return it.
            if items[0].is_pending {
                if let Some(timeout_after) = &items[0].timeout_after {
                    if timeout_after > &Utc::now() {
                        return Err(Error::NotFound(dev_eui.to_string()));
                    }
                }
            }

            // Return first item and bool indicating if there are more items in the queue.
            Ok((items[0].clone(), items.len() > 1))
        }
    })
    .await?
}

pub async fn get_for_dev_eui(dev_eui: &EUI64) -> Result<Vec<DeviceQueueItem>, Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<Vec<DeviceQueueItem>, Error> {
            let mut c = get_db_conn()?;
            let items = device_queue_item::dsl::device_queue_item
                .filter(device_queue_item::dev_eui.eq(&dev_eui))
                .order_by(device_queue_item::created_at)
                .load(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
            Ok(items)
        }
    })
    .await?
}

pub async fn flush_for_dev_eui(dev_eui: &EUI64) -> Result<(), Error> {
    let count = task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<usize, Error> {
            let mut c = get_db_conn()?;
            diesel::delete(
                device_queue_item::dsl::device_queue_item
                    .filter(device_queue_item::dev_eui.eq(&dev_eui)),
            )
            .execute(&mut c)
            .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))
        }
    })
    .await??;
    info!(dev_eui = %dev_eui, count = count, "Device queue flushed");
    Ok(())
}

pub async fn get_pending_for_dev_eui(dev_eui: &EUI64) -> Result<DeviceQueueItem, Error> {
    task::spawn_blocking({
        let dev_eui = *dev_eui;
        move || -> Result<DeviceQueueItem, Error> {
            let mut c = get_db_conn()?;
            let qi = device_queue_item::dsl::device_queue_item
                .filter(
                    device_queue_item::dev_eui
                        .eq(&dev_eui)
                        .and(device_queue_item::is_pending.eq(true)),
                )
                .first(&mut c)
                .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
            Ok(qi)
        }
    })
    .await?
}

pub async fn get_max_f_cnt_down(dev_eui: EUI64) -> Result<Option<i64>, Error> {
    task::spawn_blocking({
        move || -> Result<Option<i64>, Error> {
            let mut c = get_db_conn()?;
            Ok(device_queue_item::dsl::device_queue_item
                .select(dsl::max(device_queue_item::f_cnt_down))
                .filter(device_queue_item::dsl::dev_eui.eq(dev_eui))
                .first(&mut c)?)
        }
    })
    .await?
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::storage;
    use crate::test;

    #[tokio::test]
    async fn test_queue_item() {
        let _guard = test::prepare().await;
        let dp = storage::device_profile::test::create_device_profile(None).await;
        let d = storage::device::test::create_device(
            EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            None,
        )
        .await;

        // invalid fport
        let qi = DeviceQueueItem {
            dev_eui: d.dev_eui,
            f_port: 0,
            data: vec![0x01, 0x02, 0x03],
            ..Default::default()
        };
        assert!(enqueue_item(qi).await.is_err());

        let qi = DeviceQueueItem {
            dev_eui: d.dev_eui,
            f_port: 256,
            data: vec![0x01, 0x02, 0x03],
            ..Default::default()
        };
        assert!(enqueue_item(qi).await.is_err());

        // create
        let mut qi = DeviceQueueItem {
            dev_eui: d.dev_eui,
            f_port: 10,
            data: vec![0x01, 0x02, 0x03],
            ..Default::default()
        };
        qi = enqueue_item(qi).await.unwrap();

        // get
        let qi_get = get_item(&qi.id).await.unwrap();
        assert_eq!(qi, qi_get);

        // get for dev eui
        let queue = get_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(&qi, queue.first().unwrap());

        // next next queue item for dev eui
        let resp = get_next_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(qi, resp.0);
        assert_eq!(false, resp.1);

        // update
        qi.is_pending = true;
        qi = update_item(qi).await.unwrap();
        let qi_get = get_item(&qi.id).await.unwrap();
        assert_eq!(qi, qi_get);

        // delete
        delete_item(&qi.id).await.unwrap();
        assert_eq!(true, delete_item(&qi.id).await.is_err());
    }

    #[tokio::test]
    async fn test_flush_queue() {
        let _guard = test::prepare().await;
        let dp = storage::device_profile::test::create_device_profile(None).await;
        let d = storage::device::test::create_device(
            EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            None,
        )
        .await;

        // create
        let mut qi = DeviceQueueItem {
            dev_eui: d.dev_eui,
            f_port: 10,
            data: vec![0x01, 0x02, 0x03],
            ..Default::default()
        };
        qi = enqueue_item(qi).await.unwrap();

        // flush
        flush_for_dev_eui(&d.dev_eui).await.unwrap();
        assert_eq!(true, delete_item(&qi.id).await.is_err());
    }

    #[tokio::test]
    async fn test_get_max_f_cnt_down() {
        let _guard = test::prepare().await;
        let dp = storage::device_profile::test::create_device_profile(None).await;
        let d = storage::device::test::create_device(
            EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            dp.id,
            None,
        )
        .await;

        // create
        let mut qi = DeviceQueueItem {
            dev_eui: d.dev_eui,
            f_port: 10,
            data: vec![0x01, 0x02, 0x03],
            ..Default::default()
        };
        qi = enqueue_item(qi).await.unwrap();

        // No max_f_cnt.
        let max_f_cnt = get_max_f_cnt_down(d.dev_eui).await.unwrap();
        assert_eq!(None, max_f_cnt);

        qi.f_cnt_down = Some(10);
        update_item(qi).await.unwrap();
        let max_f_cnt = get_max_f_cnt_down(d.dev_eui).await.unwrap();
        assert_eq!(Some(10), max_f_cnt);
    }
}
