extern crate chrono;

use chrono::{DateTime, TimeZone, NaiveDateTime, UTC, Duration};

pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    start_date + Duration::seconds(1000000000)
}
