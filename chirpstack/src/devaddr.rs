// This file has been modified to support ts-lora
use rand::seq::SliceRandom;
use rand::RngCore;
use lrwn::{DevAddr, EUI64};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use chrono::Utc;
use crate::storage::schema::{device, application, tenant, device_slot as schema_device_slot};

use crate::storage::device_slot;
use crate::config;
use crate::storage::get_async_db_conn;
use anyhow::{Result, Error};
use tracing::info;

const NUMBER_OF_SLOTS: u32 = 64;
static mut CURRENT_SLOT: u32 = 0;

pub async fn get_random_dev_addr_slot(dev_eui: EUI64) -> Result<DevAddr> {
    let mut conn = get_async_db_conn().await?;

    // Generate a new DevAddr always
    let new_dev_addr = generate_dev_addr().await?;

    // Check if the device already has a slot and address
    if let Ok(existing_slot) = device_slot::get(&dev_eui).await {
        // Check if the existing slot is more than a day old
        if Utc::now().signed_duration_since(existing_slot.created_at).num_days() < 1 {
            info!(
                dev_eui = %dev_eui,
                slot = existing_slot.slot,
                "Device already has an assigned slot within the last day"
            );
            return match existing_slot.dev_addr {
                Some(_dev_addr) => Ok(new_dev_addr),
                None => Err(Error::msg("Device address not found".to_string())),
            };
        }
    }

    // Fetch the max_slot_count from the tenant based on dev_eui
    let max_slot_count_result = device::table
        .inner_join(application::table.on(application::dsl::id.eq(device::dsl::application_id)))
        .inner_join(tenant::table.on(tenant::dsl::id.eq(application::dsl::tenant_id)))
        .select(tenant::dsl::max_slot_count)
        .filter(device::dsl::dev_eui.eq(dev_eui))
        .first::<i32>(&mut conn)
        .await;

    // Check the result and convert accordingly
    let max_slot_count = match max_slot_count_result {
        Ok(count) => if count == 0 { 64u32 } else { count as u32 }, // Convert directly here
        Err(e) => return Err(Error::msg(format!("Failed to fetch max slot count for device {}: {}", dev_eui.to_string(), e))),
    };
    
    let used_slots: Vec<Option<i32>> = schema_device_slot::table
        .select(schema_device_slot::dsl::slot)
        .load::<Option<i32>>(&mut conn)
        .await?;
    
    // Filter out None values and collect into a Vec<i32>
    let mut non_null_slots: Vec<i32> = used_slots.iter().filter_map(|&slot| slot).collect();

    non_null_slots.sort();
    let mut new_slot: u32 = 0;
    for i in 0..max_slot_count {
        if !non_null_slots.contains(&(i as i32)) {
            new_slot = i;
            break;
        }
    }

    // If all slots are used, start over from 0
    if new_slot == 0 && non_null_slots.len() == max_slot_count as usize {
        new_slot = ((*non_null_slots.iter().max().unwrap_or(&0) + 1) as u32) % max_slot_count;
    }

    // Create and save the new device slot record
    let new_device_slot = device_slot::DeviceSlot {
        dev_eui,
        dev_addr: Some(new_dev_addr.clone()),
        slot: Some(new_slot as i32),
        created_at: Utc::now(),
    };

    device_slot::create(new_device_slot).await?;

    Ok(new_dev_addr)
}

// pub async fn get_random_dev_addr_slot(dev_eui: EUI64) -> Result<DevAddr> {
//     let mut conn = get_async_db_conn().await?;

//     // Check if the device already has a slot and address
//     if let Ok(existing_slot) = device_slot::get(&dev_eui).await {
//         // If exists, return the existing DevAddr
//         info!(
//             dev_eui = %dev_eui,
//             slot = existing_slot.slot,
//             "Device already has an assigned slot and DevAddr"
//         );
//         return match existing_slot.dev_addr {
//             Some(dev_addr) => Ok(dev_addr),
//             None => Err(Error::msg("Device address not found".to_string())),
//         };        
//     }

//     // Fetch the max_slot_count from the tenant based on dev_eui
//     let max_slot_count_result = device::table
//         .inner_join(application::table.on(application::dsl::id.eq(device::dsl::application_id)))
//         .inner_join(tenant::table.on(tenant::dsl::id.eq(application::dsl::tenant_id)))
//         .select(tenant::dsl::max_slot_count)
//         .filter(device::dsl::dev_eui.eq(dev_eui))
//         .first::<i32>(&mut conn)
//         .await;

//     // Check the result and convert accordingly
//     let max_slot_count = match max_slot_count_result {
//         Ok(count) => if count == 0 { 64 } else { count as u32 },
//         Err(e) => return Err(Error::msg(format!("Failed to fetch max slot count for device {}: {}", dev_eui.to_string(), e))),
//     };

//     loop {
//         // If no existing slot and address, generate a new DevAddr
//         let dev_addr = generate_dev_addr().await?;

//         // Calculate the new slot number
//         let sum: u32 = dev_addr.clone().into_iter().map(|x| x as u32).sum();
//         let generated_slot: u32 = sum % max_slot_count;

//         match CURRENT_SLOT == generated_slot {
//             true => {
//                 // define the next time slot to generate
//                 CURRENT_SLOT = CURRENT_SLOT + 1;
//                 // print the generated time slot and devaddr value
//                 println!("{:?} connected, time slot is: {}", dev_addr.clone(), generated_slot);
//                 break;
//             }
//             false => {
//                 None;
//             }
//         };
//     }


//     // Create and save the new device slot record
//     let new_device_slot = device_slot::DeviceSlot {
//         dev_eui,
//         dev_addr: Some(dev_addr.clone()),
//         slot: Some(generated_slot as i32),
//         created_at: Utc::now(),
//     };

//     device_slot::create(new_device_slot).await?;

//     Ok(dev_addr)
// }

async fn generate_dev_addr() -> Result<DevAddr> {
    let conf = config::get();
    let mut rng = rand::thread_rng();

    // Get configured DevAddr prefixes.
    let prefixes = if conf.network.dev_addr_prefixes.is_empty() {
        vec![conf.network.net_id.dev_addr_prefix()]
    }
    else {
        conf.network.dev_addr_prefixes.clone()
    };

    // Pick a random one (in case multiple prefixes are configured).
    let prefix = *prefixes.choose(&mut rng).unwrap();

    // Generate random DevAddr.
    let mut dev_addr: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut dev_addr);
    #[cfg(test)]
    {
        dev_addr = [1, 2, 3, 4];
    }
    let mut dev_addr = DevAddr::from_be_bytes(dev_addr);

    // Set DevAddr prefix.
    dev_addr.set_dev_addr_prefix(prefix);

    Ok(dev_addr)
}





// project/chirpstack/src/devaddr.rs
pub fn get_random_dev_addr() -> DevAddr {
    // check whether we still have any time slots left
    unsafe {
        // old implementation, panic if I run out of time slots
        /*
        if CURRENT_SLOT == NUMBER_OF_SLOTS {
            panic!("no free time slots left!");
        }*/

        // new implementation, counts slots from zero
        if CURRENT_SLOT == NUMBER_OF_SLOTS {
            CURRENT_SLOT = 0;
        }
    }

    let conf = config::get();
    let mut rng = rand::thread_rng();

    // Get configured DevAddr prefixes.
    let prefixes = if conf.network.dev_addr_prefixes.is_empty() {
        vec![conf.network.net_id.dev_addr_prefix()]
    }
    else {
        conf.network.dev_addr_prefixes.clone()
    };

    // Pick a random one (in case multiple prefixes are configured).
    let prefix = *prefixes.choose(&mut rng).unwrap();

    // Generate random DevAddr.
    let mut dev_addr: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut dev_addr);
    #[cfg(test)]
    {
        dev_addr = [1, 2, 3, 4];
    }
    let mut dev_addr = DevAddr::from_be_bytes(dev_addr);

    // Set DevAddr prefix.
    dev_addr.set_dev_addr_prefix(prefix);

    // print DevAddr as 4 integers
    //dbg!(dev_addr.clone());
    // print the address prefix (AddrPrefix)
    //dbg!(prefix);
    
    // find the slot of the DevAddr

    // hash function
    let sum: u32 = dev_addr.clone().into_iter().map(|x| x as u32).sum();
    let generated_slot: u32 = sum % NUMBER_OF_SLOTS;

    // regenerate the dev_addr until the correct one is generated.
    loop {
        unsafe {
            match CURRENT_SLOT == generated_slot {
                true => {
                    // define the next time slot to generate
                    CURRENT_SLOT = CURRENT_SLOT + 1;
                    // print the generated time slot and devaddr value
                    println!("{:?} connected, time slot is: {}", dev_addr.clone(), generated_slot);
                    break;
                }
                false => {
                    return get_random_dev_addr();
                }
            };
        }
        
    }
    dev_addr
}
