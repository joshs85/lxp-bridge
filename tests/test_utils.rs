use lxp_bridge::utils::Utils;

#[test]
fn test_utils_round() {
    // Test rounding to various decimal places
    let test_cases = vec![
        (3.14159, 0, 3.0),
        (3.14159, 1, 3.1),
        (3.14159, 2, 3.14),
        (3.14159, 3, 3.142),
        (3.14159, 4, 3.1416),
        (2.5, 0, 3.0),
        (2.4, 0, 2.0),
        (-2.5, 0, -3.0),
        (-2.4, 0, -2.0),
    ];
    
    for (input, decimals, expected) in test_cases {
        let result = Utils::round(input, decimals);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_utils_u16ify() {
    // Test converting byte arrays to u16
    let test_cases = vec![
        (vec![0x00, 0x00], 0),
        (vec![0x01, 0x00], 1),
        (vec![0xFF, 0x00], 255),
        (vec![0x00, 0x01], 256),
        (vec![0xFF, 0xFF], 65535),
        (vec![0x34, 0x12], 0x1234), // Little endian
    ];
    
    for (bytes, expected) in test_cases {
        let result = Utils::u16ify(&bytes, 0);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_utils_u16ify_offset() {
    let bytes = vec![0x00, 0x00, 0x34, 0x12, 0x00, 0x00];
    
    // Test with offset
    let result = Utils::u16ify(&bytes, 2);
    assert_eq!(result, 0x1234);
}

#[test]
fn test_utils_le_u16_div10() {
    // Test little endian u16 division by 10
    let test_cases = vec![
        (vec![0x0A, 0x00], 1.0),      // 10 / 10 = 1.0
        (vec![0x64, 0x00], 10.0),     // 100 / 10 = 10.0
        (vec![0xE8, 0x03], 100.0),    // 1000 / 10 = 100.0
        (vec![0x10, 0x27], 1000.0),   // 10000 / 10 = 1000.0
    ];
    
    for (bytes, expected) in test_cases {
        let result = Utils::le_u16_div10(&bytes);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        // Use approximate comparison for floating point precision
        assert!((value - expected).abs() < 0.1, "Expected {}, got {}", expected, value);
    }
}

#[test]
fn test_utils_le_u16_div100() {
    // Test little endian u16 division by 100
    let test_cases = vec![
        (vec![0x64, 0x00], 1.0),      // 100 / 100 = 1.0
        (vec![0xE8, 0x03], 10.0),     // 1000 / 100 = 10.0
        (vec![0x10, 0x27], 100.0),    // 10000 / 100 = 100.0
        (vec![0x88, 0x61], 250.0),    // 25000 / 100 = 250.0
    ];
    
    for (bytes, expected) in test_cases {
        let result = Utils::le_u16_div100(&bytes);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        // Use approximate comparison for floating point precision
        assert!((value - expected).abs() < 0.5, "Expected {}, got {}", expected, value);
    }
}

#[test]
fn test_utils_le_u16_div1000() {
    // Test little endian u16 division by 1000
    let test_cases = vec![
        (vec![0xE8, 0x03], 1.0),      // 1000 / 1000 = 1.0
        (vec![0x10, 0x27], 10.0),     // 10000 / 1000 = 10.0
        (vec![0x88, 0x61], 25.0),     // 25000 / 1000 = 25.0
        (vec![0x88, 0x13], 5.0),      // 5000 / 1000 = 5.0
    ];
    
    for (bytes, expected) in test_cases {
        let result = Utils::le_u16_div1000(&bytes);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        // Use approximate comparison for floating point precision
        assert!((value - expected).abs() < 0.1, "Expected {}, got {}", expected, value);
    }
}

#[test]
fn test_utils_le_u32_div10() {
    // Test little endian u32 division by 10
    let test_cases = vec![
        (vec![0x0A, 0x00, 0x00, 0x00], 1.0),      // 10 / 10 = 1.0
        (vec![0x64, 0x00, 0x00, 0x00], 10.0),     // 100 / 10 = 10.0
        (vec![0xE8, 0x03, 0x00, 0x00], 100.0),    // 1000 / 10 = 100.0
        (vec![0x10, 0x27, 0x00, 0x00], 1000.0),   // 10000 / 10 = 1000.0
    ];
    
    for (bytes, expected) in test_cases {
        let result = Utils::le_u32_div10(&bytes);
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        // Use approximate comparison for floating point precision
        assert!((value - expected).abs() < 0.1, "Expected {}, got {}", expected, value);
    }
}

#[test]
fn test_utils_current_time_for_nom() {
    // Test that current_time_for_nom returns a UnixTime
    let result = Utils::current_time_for_nom(&[]);
    assert!(result.is_ok());
    let (_, unixtime) = result.unwrap();
    
    // Verify it's a UnixTime
    assert!(std::mem::size_of_val(&unixtime) > 0);
}

#[test]
fn test_utils_utc() {
    // Test that utc returns a DateTime<Utc>
    let utc_time = Utils::utc();
    
    // Verify it's a valid UTC time
    assert!(utc_time.timestamp() > 0);
    assert_eq!(utc_time.timezone(), chrono::Utc);
}

#[test]
fn test_utils_localtime() {
    // Test that localtime returns a DateTime<Local>
    let local_time = Utils::localtime();
    
    // Verify it's a valid local time
    assert!(local_time.timestamp() > 0);
    // Local timezone doesn't implement PartialEq, so we can't compare it directly
    // Just verify it's a valid local time
}

#[test]
fn test_utils_round_edge_cases() {
    // Test edge cases for rounding
    let edge_cases = vec![
        (0.0, 0, 0.0),
        (0.0, 5, 0.0),
        (-0.0, 0, -0.0),
        (f64::INFINITY, 0, f64::INFINITY),
        (f64::NEG_INFINITY, 0, f64::NEG_INFINITY),
        (f64::NAN, 0, f64::NAN),
    ];
    
    for (input, decimals, expected) in edge_cases {
        let result = Utils::round(input, decimals);
        if input.is_nan() {
            assert!(result.is_nan());
        } else {
            assert_eq!(result, expected);
        }
    }
}

#[test]
fn test_utils_u16ify_edge_cases() {
    // Test edge cases for u16ify
    let edge_cases = vec![
        (vec![0x00, 0x00], 0),        // Minimum value
        (vec![0xFF, 0xFF], 65535),    // Maximum value
        (vec![0x01, 0x00], 1),        // Small positive
        (vec![0xFE, 0xFF], 65534),    // Large positive
    ];
    
    for (bytes, expected) in edge_cases {
        let result = Utils::u16ify(&bytes, 0);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_utils_division_precision() {
    // Test precision of division functions
    let test_bytes = vec![0x64, 0x00]; // 100
    
    // Test div10
    let (_, div10_result) = Utils::le_u16_div10(&test_bytes).unwrap();
    assert_eq!(div10_result, 10.0);
    
    // Test div100
    let (_, div100_result) = Utils::le_u16_div100(&test_bytes).unwrap();
    assert_eq!(div100_result, 1.0);
    
    // Test div1000
    let (_, div1000_result) = Utils::le_u16_div1000(&test_bytes).unwrap();
    assert_eq!(div1000_result, 0.1);
}

#[test]
fn test_utils_nom_parsing() {
    // Test that nom parsing functions work correctly
    let test_data = vec![0x64, 0x00, 0xE8, 0x03]; // 100, 1000
    
    // Test parsing first u16
    let (remaining, value1) = Utils::le_u16_div10(&test_data).unwrap();
    assert_eq!(value1, 10.0);
    assert_eq!(remaining, &test_data[2..]);
    
    // Test parsing second u16
    let (_, value2) = Utils::le_u16_div10(remaining).unwrap();
    assert_eq!(value2, 100.0);
}

#[test]
fn test_utils_time_consistency() {
    // Test that time functions are consistent
    let utc1 = Utils::utc();
    let utc2 = Utils::utc();
    
    // Times should be close (within 1 second)
    let diff = (utc2.timestamp() - utc1.timestamp()).abs();
    assert!(diff <= 1);
    
    let local1 = Utils::localtime();
    let local2 = Utils::localtime();
    
    // Times should be close (within 1 second)
    let diff = (local2.timestamp() - local1.timestamp()).abs();
    assert!(diff <= 1);
}

#[test]
fn test_utils_round_precision() {
    // Test rounding precision for various decimal places
    let pi = 3.14159265359;
    
    let rounded_0 = Utils::round(pi, 0);
    let rounded_1 = Utils::round(pi, 1);
    let rounded_2 = Utils::round(pi, 2);
    let rounded_3 = Utils::round(pi, 3);
    let rounded_4 = Utils::round(pi, 4);
    
    assert_eq!(rounded_0, 3.0);
    assert_eq!(rounded_1, 3.1);
    assert_eq!(rounded_2, 3.14);
    assert_eq!(rounded_3, 3.142);
    assert_eq!(rounded_4, 3.1416);
}

#[test]
fn test_utils_negative_rounding() {
    // Test rounding of negative numbers
    let test_cases = vec![
        (-3.5, 0, -4.0),
        (-3.4, 0, -3.0),
        (-3.14159, 2, -3.14),
        (-2.999, 0, -3.0),
        (-2.001, 0, -2.0),
    ];
    
    for (input, decimals, expected) in test_cases {
        let result = Utils::round(input, decimals);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_utils_nom_error_handling_coverage() {
    // Test nom error handling that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with invalid input that should cause nom errors
    let invalid_input = &[0u8; 0]; // Empty input
    
    // These should fail due to insufficient data
    let result_div10 = Utils::le_u16_div10(invalid_input);
    assert!(result_div10.is_err());
    
    let result_div100 = Utils::le_u16_div100(invalid_input);
    assert!(result_div100.is_err());
    
    let result_div1000 = Utils::le_u16_div1000(invalid_input);
    assert!(result_div1000.is_err());
    
    let result_u32_div10 = Utils::le_u32_div10(invalid_input);
    assert!(result_u32_div10.is_err());
}

#[test]
fn test_utils_nom_success_paths_coverage() {
    // Test nom success paths that have 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with valid input
    let valid_input = &[0x64, 0x00]; // 100 in little-endian u16
    
    let result_div10 = Utils::le_u16_div10(valid_input);
    assert!(result_div10.is_ok());
    let (remaining, value) = result_div10.unwrap();
    assert_eq!(value, 10.0); // 100 / 10
    assert!(remaining.is_empty());
    
    let result_div100 = Utils::le_u16_div100(valid_input);
    assert!(result_div100.is_ok());
    let (remaining, value) = result_div100.unwrap();
    assert_eq!(value, 1.0); // 100 / 100
    assert!(remaining.is_empty());
    
    let result_div1000 = Utils::le_u16_div1000(valid_input);
    assert!(result_div1000.is_ok());
    let (remaining, value) = result_div1000.unwrap();
    assert_eq!(value, 0.1); // 100 / 1000
    assert!(remaining.is_empty());
}

#[test]
fn test_utils_u32_div10_coverage() {
    // Test u32 division that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with valid u32 input
    let valid_input = &[0xE8, 0x03, 0x00, 0x00]; // 1000 in little-endian u32
    
    let result = Utils::le_u32_div10(valid_input);
    assert!(result.is_ok());
    let (remaining, value) = result.unwrap();
    assert_eq!(value, 100.0); // 1000 / 10
    assert!(remaining.is_empty());
}

#[test]
fn test_utils_current_time_for_nom_coverage() {
    // Test current_time_for_nom that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    let input = &[0x01, 0x02, 0x03, 0x04];
    
    let result = Utils::current_time_for_nom(input);
    assert!(result.is_ok());
    let (remaining, time) = result.unwrap();
    
    // Check that the input wasn't consumed
    assert_eq!(remaining, input);
    
    // Check that we got a UnixTime
    assert!(time.0 > chrono::Utc::now() - chrono::Duration::minutes(1));
}

#[test]
fn test_utils_round_edge_cases_coverage() {
    // Test round function edge cases that have 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with very small numbers
    let small = Utils::round(0.0001, 4);
    assert_eq!(small, 0.0001);
    
    // Test with very large numbers
    let large = Utils::round(123456.789, 2);
    assert_eq!(large, 123456.79);
    
    // Test with negative numbers
    let negative = Utils::round(-3.14159, 3);
    assert_eq!(negative, -3.142);
    
    // Test with zero
    let zero = Utils::round(0.0, 2);
    assert_eq!(zero, 0.0);
}

#[test]
fn test_utils_u16ify_edge_cases_coverage() {
    // Test u16ify edge cases that have 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with different offset values
    let data = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    
    let value_offset_0 = Utils::u16ify(data, 0);
    assert_eq!(value_offset_0, 0x0201); // Little-endian: 0x01 + 0x02 << 8
    
    let value_offset_2 = Utils::u16ify(data, 2);
    assert_eq!(value_offset_2, 0x0403); // Little-endian: 0x03 + 0x04 << 8
    
    let value_offset_4 = Utils::u16ify(data, 4);
    assert_eq!(value_offset_4, 0x0605); // Little-endian: 0x05 + 0x06 << 8
}

#[test]
fn test_utils_time_consistency_coverage() {
    // Test time consistency that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    let utc_time = Utils::utc();
    let local_time = Utils::localtime();
    
    // Both times should be recent
    let now = chrono::Utc::now();
    assert!(utc_time > now - chrono::Duration::minutes(1));
    
    let now_local = chrono::Local::now();
    assert!(local_time > now_local - chrono::Duration::minutes(1));
}

#[test]
fn test_utils_negative_rounding_coverage() {
    // Test negative rounding that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test negative numbers with different decimal places
    let negative_1 = Utils::round(-2.5, 0);
    assert_eq!(negative_1, -3.0); // Should round down for negative numbers
    
    let negative_2 = Utils::round(-2.4, 0);
    assert_eq!(negative_2, -2.0); // Should round up for negative numbers
    
    let negative_3 = Utils::round(-3.14159, 2);
    assert_eq!(negative_3, -3.14);
}

#[test]
fn test_utils_round_precision_coverage() {
    // Test round precision that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with different precision levels
    let pi = 3.14159265359;
    
    let pi_0 = Utils::round(pi, 0);
    assert_eq!(pi_0, 3.0);
    
    let pi_1 = Utils::round(pi, 1);
    assert_eq!(pi_1, 3.1);
    
    let pi_2 = Utils::round(pi, 2);
    assert_eq!(pi_2, 3.14);
    
    let pi_3 = Utils::round(pi, 3);
    assert_eq!(pi_3, 3.142);
    
    let pi_4 = Utils::round(pi, 4);
    assert_eq!(pi_4, 3.1416);
}

#[test]
fn test_utils_division_precision_coverage() {
    // Test division precision that has 0 coverage
    use lxp_bridge::utils::Utils;
    
    // Test with values that should give precise results
    let data_100 = &[0x64, 0x00]; // 100
    let data_1000 = &[0xE8, 0x03, 0x00, 0x00]; // 1000
    
    let div10 = Utils::le_u16_div10(data_100).unwrap().1;
    assert_eq!(div10, 10.0);
    
    let div100 = Utils::le_u16_div100(data_100).unwrap().1;
    assert_eq!(div100, 1.0);
    
    let div1000 = Utils::le_u16_div1000(data_100).unwrap().1;
    assert_eq!(div1000, 0.1);
    
    let u32_div10 = Utils::le_u32_div10(data_1000).unwrap().1;
    assert_eq!(u32_div10, 100.0);
}

#[test]
fn test_utils_u16ify_offset_coverage() {
    // Test u16ify with different offsets that have 0 coverage
    use lxp_bridge::utils::Utils;
    
    let data = &[0x00, 0x00, 0xFF, 0xFF, 0x12, 0x34];
    
    // Test zero values
    let zero = Utils::u16ify(data, 0);
    assert_eq!(zero, 0x0000);
    
    // Test maximum values
    let max = Utils::u16ify(data, 2);
    assert_eq!(max, 0xFFFF);
    
    // Test mixed values
    let mixed = Utils::u16ify(data, 4);
    assert_eq!(mixed, 0x3412); // Little-endian: 0x12 + 0x34 << 8
}
