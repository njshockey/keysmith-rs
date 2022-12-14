//! Gets a timestamp in seconds since the first second of 2022

use chrono::{DateTime, Utc};

// Generates a timestamp from the seconds since 00:00:00 Jan 1, 2022
fn gen_timestamp() -> i64 {
    let epoch = DateTime::parse_from_rfc2822("Sat, 1 Jan 2022 00:00:00 +0000")
        .expect("ERROR: Could not get timestamp epoch.");
    
    let utc = Utc::now();
    let stamp = utc.timestamp() - epoch.timestamp();

    stamp
}

/// Gets a timestamp from the seconds since 00:00:00 Jan 1, 2022 as a String
pub fn get_timestamp() -> String {
    let stamp = gen_timestamp().to_string();

    stamp
}

/// Gets a timestamp from the seconds since 00:00:00 Jan 1, 2022 as an i64
pub fn get_timestamp_i64() -> i64 {
    gen_timestamp()
}

// TODO: More tests to make sure that the format is correct. Need to allow custom epochs first.