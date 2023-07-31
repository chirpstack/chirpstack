use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::{Aes128, Block};
use anyhow::Result;
use chrono::Duration;
use tracing::info;

use lrwn::DevAddr;

lazy_static! {
    static ref BEACON_PERIOD: Duration = Duration::seconds(128);
    static ref BEACON_RESERVED: Duration = Duration::milliseconds(2120);
    static ref BEACON_GUARD: Duration = Duration::seconds(3);
    static ref BEACON_WINDOW: Duration = Duration::milliseconds(122880);
    static ref PING_PERIOD_BASE: usize = 1 << 12;
    static ref SLOT_LEN: Duration = Duration::milliseconds(30);
}

pub fn get_beacon_start(ts: Duration) -> Duration {
    Duration::seconds(ts.num_seconds() - (ts.num_seconds() % BEACON_PERIOD.num_seconds()))
}

pub fn get_ping_offset(beacon_ts: Duration, dev_addr: &DevAddr, ping_nb: usize) -> Result<usize> {
    if ping_nb == 0 {
        return Err(anyhow!("ping_nb must be > 0"));
    }

    let ping_period = *PING_PERIOD_BASE / ping_nb;
    let beacon_time = (beacon_ts.num_seconds() % (1 << 32)) as u32;

    let key_bytes: [u8; 16] = [0x00; 16];
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b: [u8; 16] = [0x00; 16];
    b[0..4].clone_from_slice(&beacon_time.to_le_bytes());
    b[4..8].clone_from_slice(&dev_addr.to_le_bytes());

    let mut block = Block::clone_from_slice(&b);
    cipher.encrypt_block(&mut block);
    let rand = block.as_slice();

    Ok(((rand[0] as usize) + ((rand[1] as usize) * 256)) % ping_period)
}

pub fn get_next_ping_slot_after(
    after_gps_epoch_ts: Duration,
    dev_addr: &DevAddr,
    ping_nb: usize,
) -> Result<Duration> {
    if ping_nb == 0 {
        return Err(anyhow!("ping_nb must be > 0"));
    }

    let mut beacon_start_ts = get_beacon_start(after_gps_epoch_ts);
    let ping_period = *PING_PERIOD_BASE / ping_nb;

    loop {
        let ping_offset = get_ping_offset(beacon_start_ts, dev_addr, ping_nb)?;
        for n in 0..ping_nb {
            let ping_slot_ts = beacon_start_ts
                + *BEACON_RESERVED
                + (*SLOT_LEN * ((ping_offset + n * ping_period) as i32));

            if ping_slot_ts > after_gps_epoch_ts {
                info!(
                    dev_addr = %dev_addr,
                    beacon_start_time_s = beacon_start_ts.num_seconds(),
                    after_beacon_start_time_ms = (ping_slot_ts - beacon_start_ts).num_milliseconds(),
                    ping_offset_ms = ping_offset,
                    ping_slot_n = n,
                    ping_nb = ping_nb,
                    "Get next ping-slot timestamp"
                );
                return Ok(ping_slot_ts);
            }
        }

        beacon_start_ts = beacon_start_ts + *BEACON_PERIOD;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::gpstime::{ToDateTime, ToGpsTime};
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn test_get_beacon_start() {
        let gps_epoch_time: DateTime<Utc> = Utc.with_ymd_and_hms(1980, 1, 6, 0, 0, 0).unwrap();

        // For GPS epoch time
        let start_ts = get_beacon_start(gps_epoch_time.to_gps_time());
        assert_eq!(start_ts, Duration::zero());

        // For now
        let start_ts = get_beacon_start(Utc::now().to_gps_time());

        // > 0
        assert!(start_ts > Duration::zero());

        // Multiple of 128 seconds.
        assert_eq!(
            0,
            start_ts.num_nanoseconds().unwrap() % Duration::seconds(128).num_nanoseconds().unwrap()
        );

        // Les than 128 seconds ago.
        let ts = start_ts.to_date_time();
        assert!(ts < Utc::now());
        assert!((Utc::now() - ts) < *BEACON_PERIOD);
    }

    #[test]
    fn test_get_ping_offset() {
        for k in 0..8 {
            let mut beacon_ts = Duration::zero();
            let ping_nb: usize = 1 << k;
            let ping_period = *PING_PERIOD_BASE / ping_nb;
            let dev_addr = DevAddr::from_be_bytes([0, 0, 0, 0]);

            for _ in 0..100000 {
                let offset = get_ping_offset(beacon_ts, &dev_addr, ping_nb).unwrap();
                assert!(offset <= ping_period - 1);
                beacon_ts = beacon_ts + *BEACON_PERIOD;
            }
        }
    }

    #[test]
    fn test_get_next_ping_slot_after() {
        struct Test {
            after: Duration,
            dev_addr: DevAddr,
            ping_nb: usize,
            expected_ping_slot_ts: Duration,
        }

        let tests = vec![
            Test {
                after: Duration::zero(),
                dev_addr: DevAddr::from_be_bytes([0, 0, 0, 0]),
                ping_nb: 1,
                expected_ping_slot_ts: Duration::minutes(1)
                    + Duration::seconds(14)
                    + Duration::milliseconds(300),
            },
            Test {
                after: Duration::minutes(2),
                dev_addr: DevAddr::from_be_bytes([0, 0, 0, 0]),
                ping_nb: 1,
                expected_ping_slot_ts: Duration::minutes(3)
                    + Duration::seconds(5)
                    + Duration::milliseconds(620),
            },
            Test {
                after: Duration::zero(),
                dev_addr: DevAddr::from_be_bytes([0, 0, 0, 0]),
                ping_nb: 2,
                expected_ping_slot_ts: Duration::seconds(12) + Duration::milliseconds(860),
            },
            Test {
                after: Duration::seconds(13),
                dev_addr: DevAddr::from_be_bytes([0, 0, 0, 0]),
                ping_nb: 2,
                expected_ping_slot_ts: Duration::minutes(1)
                    + Duration::seconds(14)
                    + Duration::milliseconds(300),
            },
            Test {
                after: Duration::seconds(124),
                dev_addr: DevAddr::from_be_bytes([0, 0, 0, 0]),
                ping_nb: 128,
                expected_ping_slot_ts: Duration::minutes(2)
                    + Duration::seconds(4)
                    + Duration::milliseconds(220),
            },
        ];

        for tst in &tests {
            let ping_slot_ts =
                get_next_ping_slot_after(tst.after, &tst.dev_addr, tst.ping_nb).unwrap();
            assert_eq!(tst.expected_ping_slot_ts, ping_slot_ts);
        }
    }
}
