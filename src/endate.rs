// Copyright 2025 Anders F Björklund

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_date() -> String {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let timestamp = duration.as_secs() as i64;
    // Use the newer DateTime API instead of deprecated NaiveDateTime::from_timestamp_opt
    let dt = chrono::DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp");

    dt.format("%a %b %d %T UTC %Y").to_string()
}
