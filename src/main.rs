extern crate chrono;

use chrono::Utc;

fn main() {
    let utc = Utc::now();
    println!("{}", utc.timestamp_nanos());
}
