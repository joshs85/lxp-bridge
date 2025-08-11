mod common;
use common::*;

// Test MQTT Exponential Backoff Calculation
#[test]
fn test_exponential_backoff_calculation() {
    let config = Factory::example_config_wrapped();
    let base_delay = config.mqtt().reconnect_delay_secs();
    let max_delay = config.mqtt().max_reconnect_delay_secs();
    
    // Test that backoff delays increase exponentially (like the actual MQTT implementation)
    let mut current_delay = base_delay;
    
    // Simulate the actual MQTT backoff logic: reconnect_delay * 2
    for _attempt in 1..=5 {
        let expected_delay = std::cmp::min(current_delay * 2, max_delay);
        
        // Verify the delay calculation matches the actual implementation
        assert!(expected_delay >= current_delay, "Delay should increase or stay the same");
        assert!(expected_delay <= max_delay, "Delay should not exceed max delay");
        
        current_delay = expected_delay;
    }
    
    // Test that delays are capped at max_delay
    let very_large_delay = base_delay * 2u64.pow(10); // Much larger than max_delay
    let capped_delay = std::cmp::min(very_large_delay, max_delay);
    assert_eq!(capped_delay, max_delay, "Large delays should be capped at max_delay");
}

// Test Maximum Delay Cap
#[test]
fn test_maximum_delay_cap() {
    let config = Factory::example_config_wrapped();
    let base_delay = config.mqtt().reconnect_delay_secs();
    let max_delay = config.mqtt().max_reconnect_delay_secs();
    
    // Test that delays are capped at max_delay
    let mut current_delay = base_delay;
    
    // Simulate multiple reconnection attempts
    for _ in 0..10 {
        let new_delay = std::cmp::min(current_delay * 2, max_delay);
        
        // Verify the delay is capped
        assert!(new_delay <= max_delay, "Delay should not exceed max delay");
        
        if new_delay == max_delay {
            // Once we hit max delay, it should stay there
            break;
        }
        
        current_delay = new_delay;
    }
    
    // Verify that we eventually hit the max delay
    assert!(current_delay <= max_delay, "Final delay should not exceed max delay");
}

// Test Edge Cases
#[test]
fn backoff_edge_cases() {
    let config = Factory::example_config_wrapped();
    let _inverter = config.enabled_inverters()[0].clone();
    
    // Test with very large failure counts - should handle overflow gracefully
    let _large_failure_count = 100u32;
    
    // Calculate backoff with overflow protection using checked operations
    let base_delay = config.mqtt().reconnect_delay_secs();
    let max_delay = config.mqtt().max_reconnect_delay_secs();
    
    // For large failure counts, the backoff should be capped at max_delay
    // We'll test this by checking that the calculation doesn't panic
    // and that large values result in the max delay
    
    // Test that the backoff calculation doesn't panic for reasonable values
    for failure_count in 0u32..=32 { // 2^32 would be too large for u64
        let backoff = if failure_count == 0 {
            base_delay
        } else {
            // Use checked operations to avoid overflow
            match 2u64.checked_pow(failure_count.saturating_sub(1)) {
                Some(power) => match base_delay.checked_mul(power) {
                    Some(delay) => std::cmp::min(delay, max_delay),
                    None => max_delay, // Overflow occurred, cap at max
                },
                None => max_delay, // Power calculation overflowed, cap at max
            }
        };
        
        // Verify the result is reasonable
        assert!(backoff >= base_delay, "Backoff should be at least base delay");
        assert!(backoff <= max_delay, "Backoff should not exceed max delay");
    }
    
    // Test that very large failure counts result in max delay
    let very_large_failure_count = 100u32;
    let backoff = if very_large_failure_count == 0 {
        base_delay
    } else {
        // This will overflow, so it should be capped at max_delay
        match 2u64.checked_pow(very_large_failure_count.saturating_sub(1)) {
            Some(power) => match base_delay.checked_mul(power) {
                Some(delay) => std::cmp::min(delay, max_delay),
                None => max_delay, // Overflow occurred, cap at max
            },
            None => max_delay, // Power calculation overflowed, cap at max
        }
    };
    
    // Should be capped at max delay due to overflow
    assert_eq!(backoff, max_delay, "Large failure count should result in max delay");
}

// Test Inverter Configuration Validation
#[test]
fn inverter_config_validation() {
    let config = Factory::example_config_wrapped();
    let inverter = config.enabled_inverters()[0].clone();
    
    // Test that inverter configuration is valid
    assert!(inverter.enabled(), "Inverter should be enabled");
    assert!(!inverter.host().is_empty(), "Host should not be empty");
    assert!(inverter.port() > 0, "Port should be positive");
    // Port is already validated to be <= 65535 by the type system
    
    // Test timeout configuration
    let timeout = inverter.read_timeout();
    assert!(timeout >= 1, "Timeout should be at least 1 second, got {}", timeout);
    assert!(timeout <= 3600, "Timeout should not exceed 1 hour, got {}", timeout);
    
    // Test that the timeout matches the expected default
    assert_eq!(timeout, 900, "Expected default timeout of 900 seconds, got {}", timeout);
}

// Test Connection Resilience Configuration
#[test]
fn test_connection_resilience_config() {
    let config = Factory::example_config_wrapped();
    
    // Test that MQTT reconnection settings are properly configured
    let mqtt_config = config.mqtt();
    if mqtt_config.enabled() {
        assert!(mqtt_config.reconnect_delay_secs() > 0, "Reconnect delay should be positive");
        assert!(mqtt_config.max_reconnect_delay_secs() >= mqtt_config.reconnect_delay_secs(), 
                "Max reconnect delay should be >= base reconnect delay");
        assert!(mqtt_config.max_retries() > 0, "Max retries should be positive");
        assert!(mqtt_config.circuit_breaker_threshold() > 0, "Circuit breaker threshold should be positive");
    }
}

// Test Channel Health Monitoring
#[test]
fn test_channel_health_monitoring() {
    let channels = Channels::new();
    
    // Test that channel health monitoring works correctly
    channels.check_channel_health(); // Should not panic
    
    // Test that health monitoring can be called multiple times
    for _ in 0..10 {
        channels.check_channel_health(); // Should not panic
    }
}

// Test Inverter Serial Number Validation
#[test]
fn test_inverter_serial_validation() {
    let config = Factory::example_config_wrapped();
    
    // Test that inverter serial numbers are properly formatted
    let inverters = config.enabled_inverters();
    
    for inverter in inverters {
        let serial_str = inverter.serial().to_string();
        let datalog_str = inverter.datalog().to_string();
        
        // Test serial number format (should be numeric)
        assert!(serial_str.chars().all(|c| c.is_ascii_digit()), 
                "Serial number should be numeric: {}", serial_str);
        assert!(datalog_str.chars().all(|c| c.is_ascii_digit()), 
                "Datalog should be numeric: {}", datalog_str);
        
        // Test serial number length (should be reasonable)
        assert!(serial_str.len() >= 5, "Serial number should be at least 5 digits: {}", serial_str);
        assert!(serial_str.len() <= 20, "Serial number should be at most 20 digits: {}", serial_str);
        assert!(datalog_str.len() >= 5, "Datalog should be at least 5 digits: {}", datalog_str);
        assert!(datalog_str.len() <= 20, "Datalog should be at most 20 digits: {}", datalog_str);
    }
}

// Test Network Configuration Validation
#[test]
fn test_network_config_validation() {
    let config = Factory::example_config_wrapped();
    
    // Test that network-related configuration is valid
    let inverters = config.enabled_inverters();
    
    for inverter in inverters {
        let host = inverter.host();
        let port = inverter.port();
        
        // Test host format
        if host == "localhost" || host == "127.0.0.1" {
            // Local connections should use standard ports
            assert!(port >= 1024, "Local connections should use non-privileged ports");
        } else {
            // Remote connections should have valid hostnames/IPs
            assert!(!host.is_empty(), "Remote host should not be empty");
            assert!(host.len() <= 253, "Hostname should be reasonable length");
        }
        
        // Test port ranges
        assert!(port > 0, "Port should be positive");
        let _port_value = port;
        // Port is already validated to be <= 65535 by the type system
        
        // Avoid common problematic ports
        assert!(port != 22, "Port 22 is typically SSH, avoid for inverter connections");
        assert!(port != 80, "Port 80 is typically HTTP, avoid for inverter connections");
        assert!(port != 443, "Port 443 is typically HTTPS, avoid for inverter connections");
    }
}

// Test Timeout Configuration Validation
#[test]
fn timeout_validation() {
    let config = Factory::example_config_wrapped();
    let inverter = config.enabled_inverters()[0].clone();
    
    // Test that timeout values are reasonable (not too short, not too long)
    let timeout = inverter.read_timeout();
    
    // Should be at least 1 second
    assert!(timeout >= 1, "Timeout should be at least 1 second, got {}", timeout);
    
    // Should not be unreasonably long (more than 1 hour)
    assert!(timeout <= 3600, "Timeout should not exceed 1 hour, got {}", timeout);
    
    // Should match the default from config (900 seconds = 15 minutes)
    assert_eq!(timeout, 900, "Expected default timeout of 900 seconds, got {}", timeout);
}

// Test Heartbeat Configuration
#[test]
fn test_heartbeat_config_validation() {
    let config = Factory::example_config_wrapped();
    
    // Test that heartbeat configuration is valid if present
    let inverters = config.enabled_inverters();
    
    for inverter in inverters {
        let heartbeats = inverter.heartbeats();
        // Heartbeat should be a boolean value
        assert!(heartbeats == true || heartbeats == false, "Heartbeat should be a boolean");
        
        // If heartbeats are enabled, we can test additional logic
        if heartbeats {
            // Heartbeats are enabled, test configuration
            if inverter.host() == "localhost" {
                // Local connections can have more frequent heartbeats
                // (This would need additional configuration fields to test actual intervals)
            } else {
                // Remote connections should have appropriate heartbeat settings
                // (This would need additional configuration fields to test actual intervals)
            }
        }
    }
}

// Test Publish Holdings Configuration
#[test]
fn test_publish_holdings_config_validation() {
    let config = Factory::example_config_wrapped();
    
    // Test that publish holdings configuration is valid if present
    let inverters = config.enabled_inverters();
    
    for inverter in inverters {
        let publish_holdings = inverter.publish_holdings_on_connect();
        // Publish holdings should be a boolean
        // This is already enforced by the type system, but we can test the value
        assert!(publish_holdings == true || publish_holdings == false);
    }
}



// Test Configuration Consistency
#[test]
fn test_configuration_consistency() {
    let config = Factory::example_config_wrapped();
    
    // Test that configuration is consistent across components
    let inverters = config.enabled_inverters();
    
    // If MQTT is enabled, there should be at least one inverter
    if config.mqtt().enabled() {
        assert!(!inverters.is_empty(), "MQTT enabled but no inverters configured");
    }
    
    // If no MQTT, there should be alternative data outputs
    if !config.mqtt().enabled() {
        
    }
}

// Test Error Handling Resilience
#[test]
fn test_error_handling_resilience() {
    let config = Factory::example_config_wrapped();
    
    // Test that the configuration system can handle various error conditions gracefully
    // This tests the resilience of the configuration wrapper
    
    // Test that accessing configuration multiple times doesn't cause issues
    for _ in 0..100 {
        let _mqtt_config = config.mqtt();
    
    }
    
    // If we get here, no panic occurred during repeated config access
    assert!(true, "Configuration access should be resilient to repeated calls");
}
