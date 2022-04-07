use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use chrono_tz::US::Central;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn get_time() -> Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch;
}

pub fn parse_timestamp(timestamp: u64) -> chrono::DateTime<Tz> {
    let native_dt = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let dt = Central.from_utc_datetime(&native_dt);

    return dt;
}
