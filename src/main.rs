extern crate chrono;

use chrono::{TimeZone, Utc};

fn tick(nano: u32) -> i64 {
    let utc = Utc::now();
    let nanos = utc.timestamp_subsec_nanos() + nano;
    let dt = Utc.timestamp(utc.timestamp(), nanos);

    dt.timestamp_nanos()
}

fn main() {
    let mut nano = 0;

    tick(nano);

    nano += 1;
}
