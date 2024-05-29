// This file has been modified to support ts-lora
use rand::seq::SliceRandom;
use rand::RngCore;

use crate::config;
use lrwn::DevAddr;

const NUMBER_OF_SLOTS: u32 = 64;
static mut CURRENT_SLOT: u32 = 0;

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
