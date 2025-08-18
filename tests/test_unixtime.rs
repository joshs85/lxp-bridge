use lxp_bridge::unixtime::UnixTime;
use chrono::{Utc, TimeZone, Datelike, Timelike};

#[test]
fn test_unixtime_now() {
    let unixtime = UnixTime::now();
    
    // Test that we can create a UnixTime
    assert!(std::mem::size_of_val(&unixtime) > 0);
}

#[test]
fn test_unixtime_timestamp() {
    let unixtime = UnixTime::now();
    
    // Test that we can access the timestamp
    let timestamp = unixtime.0.timestamp();
    assert!(timestamp > 0);
}

#[test]
fn test_unixtime_from_chrono() {
    let chrono_time = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    
    assert_eq!(unixtime.0.timestamp(), 1640995200);
}

#[test]
fn test_unixtime_to_chrono() {
    let unixtime = UnixTime::now();
    let chrono_time = unixtime.0;
    
    // Test that we can access the chrono DateTime
    assert!(chrono_time.timestamp() > 0);
    assert_eq!(chrono_time.timezone(), Utc);
}

#[test]
fn test_unixtime_conversion_roundtrip() {
    let original_timestamp = 1640995200;
    let chrono_time = Utc.timestamp_opt(original_timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let converted_chrono = unixtime.0;
    
    assert_eq!(converted_chrono.timestamp(), original_timestamp);
}

#[test]
fn test_unixtime_epoch() {
    let epoch = UnixTime(Utc.timestamp_opt(0, 0).unwrap());
    let chrono_epoch = epoch.0;
    
    assert_eq!(chrono_epoch.year(), 1970);
    assert_eq!(chrono_epoch.month(), 1);
    assert_eq!(chrono_epoch.day(), 1);
    assert_eq!(chrono_epoch.hour(), 0);
    assert_eq!(chrono_epoch.minute(), 0);
    assert_eq!(chrono_epoch.second(), 0);
}

#[test]
fn test_unixtime_future_date() {
    let future_timestamp = 2000000000; // 2033-05-18 03:33:20 UTC
    let chrono_time = Utc.timestamp_opt(future_timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let chrono_future = unixtime.0;
    
    assert_eq!(chrono_future.year(), 2033);
    assert_eq!(chrono_future.month(), 5);
    assert_eq!(chrono_future.day(), 18);
}

#[test]
fn test_unixtime_past_date() {
    let past_timestamp = 946684800; // 2000-01-01 00:00:00 UTC
    let chrono_time = Utc.timestamp_opt(past_timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
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
        1640995199,          // Just before 2022-01-01
        1640995201,          // Just after 2022-01-01
    ];
    
    for timestamp in edge_cases {
        let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
        let unixtime = UnixTime(chrono_time);
        
        assert_eq!(unixtime.0.timestamp(), timestamp);
        assert!(unixtime.0 >= Utc.timestamp_opt(0, 0).unwrap());
    }
}

#[test]
fn test_unixtime_chrono_operations() {
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let chrono_time = unixtime.0;
    
    // Test adding time
    let one_hour_later = chrono_time + chrono::Duration::hours(1);
    assert_eq!(one_hour_later.hour(), 1);
    
    // Test subtracting time
    let one_hour_earlier = chrono_time - chrono::Duration::hours(1);
    assert_eq!(one_hour_earlier.hour(), 23);
    assert_eq!(one_hour_earlier.day(), 31); // Previous day
    
    // Test adding days
    let one_day_later = chrono_time + chrono::Duration::days(1);
    assert_eq!(one_day_later.day(), 2);
}

#[test]
fn test_unixtime_formatting() {
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let chrono_time = unixtime.0;
    
    // Test various formatting options
    let formatted_rfc3339 = chrono_time.to_rfc3339();
    println!("RFC3339 format: {}", formatted_rfc3339);
    
    // The RFC3339 format might vary slightly, so let's check for the key components
    assert!(formatted_rfc3339.contains("2022-01-01"));
    assert!(formatted_rfc3339.contains("T"));
    assert!(formatted_rfc3339.contains("00:00:00"));
    
    let formatted_iso = chrono_time.format("%Y-%m-%d %H:%M:%S").to_string();
    assert_eq!(formatted_iso, "2022-01-01 00:00:00");
    
    let formatted_date = chrono_time.format("%B %d, %Y").to_string();
    assert_eq!(formatted_date, "January 01, 2022");
}

#[test]
fn test_unixtime_comparison() {
    let earlier_timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let later_timestamp = 1640995260;   // 2022-01-01 00:01:00 UTC
    
    let earlier_chrono = Utc.timestamp_opt(earlier_timestamp, 0).unwrap();
    let later_chrono = Utc.timestamp_opt(later_timestamp, 0).unwrap();
    
    let earlier = UnixTime(earlier_chrono);
    let later = UnixTime(later_chrono);
    
    assert!(earlier.0 < later.0);
    assert!(later.0 > earlier.0);
    assert!(earlier.0 <= later.0);
    assert!(later.0 >= earlier.0);
    assert_ne!(earlier.0, later.0);
}

#[test]
fn test_unixtime_equality() {
    let timestamp = 1640995200;
    let chrono_time1 = Utc.timestamp_opt(timestamp, 0).unwrap();
    let chrono_time2 = Utc.timestamp_opt(timestamp, 0).unwrap();
    let chrono_time3 = Utc.timestamp_opt(timestamp + 60, 0).unwrap();
    
    let time1 = UnixTime(chrono_time1);
    let time2 = UnixTime(chrono_time2);
    let time3 = UnixTime(chrono_time3);
    
    assert_eq!(time1.0, time2.0);
    assert_ne!(time1.0, time3.0);
}

#[test]
fn test_unixtime_hash() {
    let timestamp1 = 1640995200;
    let timestamp2 = 1640995260;
    
    let chrono_time1 = Utc.timestamp_opt(timestamp1, 0).unwrap();
    let chrono_time2 = Utc.timestamp_opt(timestamp2, 0).unwrap();
    
    let time1 = UnixTime(chrono_time1);
    let time2 = UnixTime(chrono_time2);
    
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert(time1.0.timestamp(), "time1_value");
    hash_map.insert(time2.0.timestamp(), "time2_value");
    
    assert_eq!(hash_map.len(), 2);
    assert_eq!(hash_map.get(&time1.0.timestamp()), Some(&"time1_value"));
    assert_eq!(hash_map.get(&time2.0.timestamp()), Some(&"time2_value"));
}

#[test]
fn test_unixtime_debug() {
    let timestamp = 1640995200;
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    
    let debug_str = format!("{:?}", unixtime);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("UnixTime"));
}

#[test]
fn test_unixtime_clone() {
    let timestamp = 1640995200;
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let cloned = unixtime.clone();
    
    assert_eq!(unixtime.0, cloned.0);
    assert_eq!(unixtime.0.timestamp(), cloned.0.timestamp());
}

#[test]
fn test_unixtime_copy() {
    let timestamp = 1640995200;
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    
    // Test that UnixTime implements Copy
    let copied = unixtime;
    let _ = unixtime; // Should still work if Copy is implemented
    
    assert_eq!(copied.0.timestamp(), timestamp);
}

#[test]
fn test_unixtime_serde() {
    let timestamp = 1640995200;
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    
    // Test serialization
    let serialized = serde_json::to_string(&unixtime).unwrap();
    assert_eq!(serialized, timestamp.to_string());
}

#[test]
fn test_unixtime_chrono_timezone() {
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let chrono_time = unixtime.0;
    
    // Test timezone conversion
    let local_time = chrono_time.with_timezone(&chrono::Local);
    assert_eq!(local_time.timestamp(), timestamp);
    
    // Test back to UTC
    let utc_time = local_time.with_timezone(&Utc);
    assert_eq!(utc_time, chrono_time);
}

#[test]
fn test_unixtime_chrono_duration() {
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let chrono_time = unixtime.0;
    
    // Test duration calculations
    let one_second = chrono::Duration::seconds(1);
    let one_minute = chrono::Duration::minutes(1);
    let one_hour = chrono::Duration::hours(1);
    let one_day = chrono::Duration::days(1);
    
    let time_plus_second = chrono_time + one_second;
    let time_plus_minute = chrono_time + one_minute;
    let time_plus_hour = chrono_time + one_hour;
    let time_plus_day = chrono_time + one_day;
    
    assert_eq!(time_plus_second.timestamp(), timestamp + 1);
    assert_eq!(time_plus_minute.timestamp(), timestamp + 60);
    assert_eq!(time_plus_hour.timestamp(), timestamp + 3600);
    assert_eq!(time_plus_day.timestamp(), timestamp + 86400);
}

#[test]
fn test_unixtime_chrono_naive() {
    let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    let chrono_time = unixtime.0;
    
    // Test conversion to naive datetime
    let naive_time = chrono_time.naive_utc();
    assert_eq!(naive_time.timestamp(), timestamp);
    
    // Test conversion back to UTC
    let utc_time = chrono::DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);
    assert_eq!(utc_time, chrono_time);
}

#[test]
fn test_unixtime_utils_integration() {
    // Test that UnixTime works with Utils
    let unixtime = UnixTime::now();
    
    // Test that we can access the inner chrono DateTime
    let chrono_time = unixtime.0;
    assert!(chrono_time.timestamp() > 0);
    
    // Test that we can format it
    let formatted = chrono_time.format("%Y-%m-%d").to_string();
    assert!(!formatted.is_empty());
    assert!(formatted.contains("-"));
}

#[test]
fn test_unixtime_serialization_format() {
    let timestamp = 1640995200;
    let chrono_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let unixtime = UnixTime(chrono_time);
    
    // Test that serialization produces the expected format
    let serialized = serde_json::to_string(&unixtime).unwrap();
    
    // Should be just the timestamp as a string
    assert_eq!(serialized, "1640995200");
    
    // Should not contain any chrono formatting
    assert!(!serialized.contains("T"));
    assert!(!serialized.contains("Z"));
    assert!(!serialized.contains("2022"));
}
