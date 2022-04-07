use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn get_time() -> Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch;
}

pub fn parse_timestamp(timestamp: u64) -> chrono::DateTime<Utc> {
    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp as i64, 0), Utc);
    return dt;
}
