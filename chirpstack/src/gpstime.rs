use chrono::{DateTime, Duration, TimeZone, Utc};

lazy_static! {
    static ref GPS_EPOCH_TIME: DateTime<Utc> = Utc.ymd(1980, 1, 6).and_hms(0, 0, 0);
    static ref LEAP_SECONDS_TABLE: Vec<(DateTime<Utc>, Duration)> = vec![
        (
            Utc.ymd(1981, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1982, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1983, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1985, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1987, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1989, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1990, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1992, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1993, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1994, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1995, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1997, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(1998, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(2005, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(2008, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(2012, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(2015, 6, 30).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
        (
            Utc.ymd(2016, 12, 31).and_hms(23, 59, 59),
            Duration::seconds(1)
        ),
    ];
}

pub trait ToGpsTime {
    fn to_gps_time(&self) -> Duration;
}

pub trait ToDateTime {
    fn to_date_time(&self) -> DateTime<Utc>;
}

impl ToGpsTime for DateTime<Utc> {
    fn to_gps_time(&self) -> Duration {
        let mut offset = Duration::zero();
        for ls in LEAP_SECONDS_TABLE.iter() {
            if &ls.0 < self {
                offset = offset + ls.1;
            }
        }

        self.signed_duration_since(*GPS_EPOCH_TIME) + offset
    }
}

impl ToDateTime for Duration {
    fn to_date_time(&self) -> DateTime<Utc> {
        let mut t = *GPS_EPOCH_TIME + *self;
        for ls in LEAP_SECONDS_TABLE.iter() {
            if ls.0 < t {
                t -= ls.1;
            }
        }
        t
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    struct Test {
        time: DateTime<Utc>,
        time_since_gps_epoch: Duration,
    }

    #[test]
    fn test() {
        let tests = vec![
            Test {
                time: *GPS_EPOCH_TIME,
                time_since_gps_epoch: Duration::zero(),
            },
            Test {
                time: Utc.ymd(2010, 1, 28).and_hms(16, 36, 24),
                time_since_gps_epoch: Duration::seconds(948731799),
            },
            Test {
                time: Utc.ymd(2025, 7, 14).and_hms(0, 0, 0),
                time_since_gps_epoch: Duration::seconds(1436486418),
            },
            Test {
                time: Utc.ymd(2012, 6, 30).and_hms(23, 59, 59),
                time_since_gps_epoch: Duration::seconds(1025136014),
            },
            Test {
                time: Utc.ymd(2012, 7, 1).and_hms(0, 0, 0),
                time_since_gps_epoch: Duration::seconds(1025136016),
            },
        ];

        for tst in tests {
            assert_eq!(tst.time_since_gps_epoch, tst.time.to_gps_time());
            assert_eq!(tst.time, tst.time_since_gps_epoch.to_date_time());
        }
    }
}
