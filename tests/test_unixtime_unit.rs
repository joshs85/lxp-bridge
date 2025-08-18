use lxp_bridge::unixtime::UnixTime;
use chrono::{Datelike, Timelike};

#[test]
fn test_unixtime_now() {
    let unixtime = UnixTime::now();
    assert!(std::mem::size_of_val(&unixtime) > 0);
}

#[test]
fn test_unixtime_new() {
    let unixtime = UnixTime::new(1640995200);
    assert_eq!(unixtime.0.timestamp(), 1640995200);
}

#[test]
fn test_unixtime_epoch() {
    let epoch = UnixTime::new(0);
    let chrono_epoch = epoch.0;
    
    assert_eq!(chrono_epoch.year(), 1970);
    assert_eq!(chrono_epoch.month(), 1);
    assert_eq!(chrono_epoch.day(), 1);
}

#[test]
fn test_unixtime_serialization() {
    let unixtime = UnixTime::new(1640995200);
    let serialized = serde_json::to_string(&unixtime).unwrap();
    assert_eq!(serialized, "1640995200");
}

#[test]
fn test_unixtime_future_date() {
    let future_timestamp = 2000000000; // 2033-05-18 03:33:20 UTC
    let unixtime = UnixTime::new(future_timestamp as u32);
    let chrono_future = unixtime.0;
    
    assert_eq!(chrono_future.year(), 2033);
    assert_eq!(chrono_future.month(), 5);
    assert_eq!(chrono_future.day(), 18);
}

#[test]
fn test_unixtime_past_date() {
    let past_timestamp = 946684800; // 2000-01-01 00:00:00 UTC
    let unixtime = UnixTime::new(past_timestamp as u32);
    let chrono_past = unixtime.0;
    
    assert_eq!(chrono_past.year(), 2000);
    assert_eq!(chrono_past.month(), 1);
    assert_eq!(chrono_past.day(), 1);
}

#[test]
fn test_unixtime_edge_cases() {
    // Test various edge cases
    let edge_cases = vec![
        0,                    // Unix epoch
        1,                    // One second after epoch
        86400,               // One day after epoch
        31536000,            // One year after epoch
        2147483647,          // Maximum 32-bit signed integer
    ];
    
    for timestamp in edge_cases {
        let unixtime = UnixTime::new(timestamp as u32);
        assert_eq!(unixtime.0.timestamp(), timestamp);
    }
}

#[test]
fn test_unixtime_clone() {
    let unixtime = UnixTime::new(1640995200);
    let cloned = unixtime.clone();
    assert_eq!(unixtime.0.timestamp(), cloned.0.timestamp());
}

#[test]
fn test_unixtime_debug() {
    let unixtime = UnixTime::new(1640995200);
    let debug_str = format!("{:?}", unixtime);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("UnixTime"));
}

#[test]
fn test_unixtime_equality() {
    let time1 = UnixTime::new(1640995200);
    let time2 = UnixTime::new(1640995200);
    let time3 = UnixTime::new(1640995260);
    
    assert_eq!(time1, time2);
    assert_ne!(time1, time3);
}

#[test]
fn test_unixtime_chrono_operations() {
    let unixtime = UnixTime::new(1640995200); // 2022-01-01 00:00:00 UTC
    let chrono_time = unixtime.0;
    
    // Test adding time
    let one_hour_later = chrono_time + chrono::Duration::hours(1);
    assert_eq!(one_hour_later.hour(), 1);
    
    // Test subtracting time
    let one_hour_earlier = chrono_time - chrono::Duration::hours(1);
    assert_eq!(one_hour_earlier.hour(), 23);
    assert_eq!(one_hour_earlier.day(), 31); // Previous day
}

#[test]
fn test_unixtime_formatting() {
    let unixtime = UnixTime::new(1640995200); // 2022-01-01 00:00:00 UTC
    let chrono_time = unixtime.0;
    
    // Test various formatting options
    let formatted_rfc3339 = chrono_time.to_rfc3339();
    println!("RFC3339 format: {}", formatted_rfc3339);
    
    // The RFC3339 format uses +00:00 for UTC timezone
    assert!(formatted_rfc3339.contains("2022-01-01"));
    assert!(formatted_rfc3339.contains("T"));
    assert!(formatted_rfc3339.contains("00:00:00"));
    assert!(formatted_rfc3339.contains("+00:00"));
    
    let formatted_iso = chrono_time.format("%Y-%m-%d %H:%M:%S").to_string();
    assert_eq!(formatted_iso, "2022-01-01 00:00:00");
}
