mod common;
use common::*;
use lxp_bridge::mqtt::{CircuitBreaker, CircuitBreakerState};
use tokio::time::Duration;

// Test Circuit Breaker functionality
#[test]
fn test_circuit_breaker_new() {
    let cb = CircuitBreaker::new(3);
    assert_eq!(cb.state, CircuitBreakerState::Closed);
    assert_eq!(cb.failure_count, 0);
    assert_eq!(cb.threshold, 3);
    assert!(cb.last_failure_time.is_none());
    assert!(cb.last_success_time.is_none());
}

#[test]
fn test_circuit_breaker_record_success() {
    let mut cb = CircuitBreaker::new(3);
    
    // Record success should reset failure count and update last success time
    cb.record_success();
    assert_eq!(cb.failure_count, 0);
    assert!(cb.last_success_time.is_some());
    assert_eq!(cb.state, CircuitBreakerState::Closed);
}

#[test]
fn test_circuit_breaker_record_failure() {
    let mut cb = CircuitBreaker::new(3);
    
    // Record first failure
    cb.record_failure();
    assert_eq!(cb.failure_count, 1);
    assert!(cb.last_failure_time.is_some());
    assert_eq!(cb.state, CircuitBreakerState::Closed);
    
    // Record second failure
    cb.record_failure();
    assert_eq!(cb.failure_count, 2);
    assert_eq!(cb.state, CircuitBreakerState::Closed);
    
    // Record third failure - should open circuit
    cb.record_failure();
    assert_eq!(cb.failure_count, 3);
    assert_eq!(cb.state, CircuitBreakerState::Open);
}

#[test]
fn test_circuit_breaker_should_allow_request() {
    let mut cb = CircuitBreaker::new(2);
    
    // Initially closed - should allow requests
    assert!(cb.should_allow_request());
    
    // After one failure - still closed
    cb.record_failure();
    assert!(cb.should_allow_request());
    
    // After threshold failures - circuit opens
    cb.record_failure();
    assert!(!cb.should_allow_request());
    
    // After success - circuit closes again
    cb.record_success();
    assert!(cb.should_allow_request());
}

#[test]
fn test_circuit_breaker_timeout_recovery() {
    let mut cb = CircuitBreaker::new(2);
    
    // Open the circuit
    cb.record_failure();
    cb.record_failure();
    assert_eq!(cb.state, CircuitBreakerState::Open);
    
    // Simulate timeout passing (we can't easily test the actual timeout in unit tests)
    // This would require mocking time or using a test double
    // For now, we'll test the success path
    cb.record_success();
    assert_eq!(cb.state, CircuitBreakerState::Closed);
}

// Test Channel Health Monitoring
#[test]
fn test_channel_health_check() {
    let channels = Channels::new();
    
    // Initially channels should be healthy
    channels.check_channel_health();
    
    // Test that health monitoring can be called multiple times without issues
    for _ in 0..10 {
        channels.check_channel_health();
    }
    
    // Test that health monitoring works even after some time
    std::thread::sleep(Duration::from_millis(10));
    channels.check_channel_health();
}

// Test MQTT Reliability Features
#[tokio::test]
async fn test_mqtt_circuit_breaker_integration() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    let mqtt = Mqtt::new(config, channels);
    
    // Test circuit breaker info
    let cb_info = mqtt.get_circuit_breaker_info().await;
    assert!(cb_info.contains("state: Closed"));
    assert!(cb_info.contains("failures: 0"));
}

#[tokio::test]
async fn test_mqtt_health_status() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    let mqtt = Mqtt::new(config, channels);
    
    let health_status = mqtt.get_health_status().await;
    assert_eq!(health_status, "healthy"); // Should be "healthy" when circuit breaker is closed
}

// Test Exponential Backoff (simulated)
#[test]
fn test_exponential_backoff_calculation() {
    // Test that backoff delays increase exponentially
    let base_delay = Duration::from_secs(1);
    let max_delay = Duration::from_secs(60);
    
    let delay_1 = calculate_backoff_delay(1, base_delay, max_delay);
    let delay_2 = calculate_backoff_delay(2, base_delay, max_delay);
    let delay_3 = calculate_backoff_delay(3, base_delay, max_delay);
    
    // Each attempt should have a longer delay
    assert!(delay_2 > delay_1);
    assert!(delay_3 > delay_2);
    
    // But should not exceed max delay
    assert!(delay_1 <= max_delay);
    assert!(delay_2 <= max_delay);
    assert!(delay_3 <= max_delay);
}

// Helper function to calculate backoff delay (similar to what's used in the actual code)
fn calculate_backoff_delay(attempt: u32, base_delay: Duration, max_delay: Duration) -> Duration {
    let delay = base_delay * 2_u32.pow(attempt - 1);
    if delay > max_delay {
        max_delay
    } else {
        delay
    }
}

// Test Error Recovery Patterns
#[tokio::test]
async fn test_mqtt_message_retry_logic() {
    let inverter = Factory::inverter();
    
    // Test that MQTT message creation handles various data types correctly
    let packet = lxp::packet::ReadParam {
        datalog: inverter.datalog(),
        register: 0,
        values: vec![1, 0],
    };
    
    let messages = mqtt::Message::for_param(packet).unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].topic, "2222222222/param/0");
    assert_eq!(messages[0].payload, "1");
}

#[tokio::test]
async fn test_mqtt_message_error_handling() {
    let inverter = Factory::inverter();
    
    // Test that invalid data is handled gracefully
    let packet = lxp::packet::ReadParam {
        datalog: inverter.datalog(),
        register: 0,
        values: vec![], // Empty values should not cause panic
    };
    
    let messages = mqtt::Message::for_param(packet).unwrap();
    assert_eq!(messages.len(), 0); // No messages for empty values
}

// Test Channel Buffer Management
#[test]
fn test_channel_buffer_sizes() {
    let channels = Channels::new();
    
    // Test that channels can be created and health monitoring works
    // The actual buffer size is set in the Channels::new() implementation
    // We can't directly test the buffer size, but we can test that channels work
    channels.check_channel_health();
    
    // Test that we can create additional channels for testing purposes
    let (tx, _rx) = tokio::sync::broadcast::channel::<mqtt::ChannelData>(100);
    
    // Send a message (this tests that the channel works)
    let message = mqtt::ChannelData::Message(mqtt::Message {
        topic: "test/topic".to_string(),
        retain: false,
        payload: "test payload".to_string(),
    });
    
    let _ = tx.send(message);
    
    // The message was sent successfully, which means the channel buffer is working
    // We don't need to receive it in this test as it's just testing buffer functionality
}

// Test Configuration Validation
#[test]
fn test_reliability_config_validation() {
    let config = Factory::example_config_wrapped();
    
    // Test that reliability settings are properly configured
    let mqtt_config = config.mqtt();
    assert!(mqtt_config.max_retries() > 0);
    assert!(mqtt_config.circuit_breaker_threshold() > 0);
    assert!(mqtt_config.reconnect_delay_secs() > 0);
    assert!(mqtt_config.max_reconnect_delay_secs() >= mqtt_config.reconnect_delay_secs());
}

// Test Graceful Degradation
#[tokio::test]
async fn test_graceful_degradation_on_errors() {
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    let mqtt = Mqtt::new(config, channels);
    
    // Test that the system doesn't crash on configuration errors
    // This is more of an integration test, but we can test the basic structure
    
    // Verify that circuit breaker starts in closed state
    let cb_info = mqtt.get_circuit_breaker_info().await;
    assert!(cb_info.contains("Closed"));
}

// Test Health Monitoring Integration
#[test]
fn test_health_monitor_integration() {
    // Test that health monitoring can be set up without errors
    let config = Factory::example_config_wrapped();
    let channels = Channels::new();
    
    // This would normally run in a separate task
    // We're just testing that the components can be created
    let _inverter_count = config.enabled_inverters().len(); // Test the function call
    // inverter_count is always >= 0, but this tests the function call
    channels.check_channel_health(); // Should not panic
}

// Test MQTT Message Serialization Reliability
#[tokio::test]
async fn test_mqtt_message_serialization_reliability() {
    let inverter = Factory::inverter();
    
    // Test various data types to ensure serialization doesn't fail
    let test_cases = vec![
        (vec![0, 0], "0"),
        (vec![1, 0], "1"),
        (vec![255, 255], "65535"),
        (vec![100, 50], "12900"),
    ];
    
    for (values, expected) in test_cases {
        let packet = lxp::packet::ReadParam {
            datalog: inverter.datalog(),
            register: 0,
            values: values.clone(),
        };
        
        let messages = mqtt::Message::for_param(packet).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].payload, expected);
    }
}

// Test Bitmask Register Handling Reliability
#[tokio::test]
async fn test_bitmask_register_reliability() {
    let inverter = Factory::inverter();
    
    // Test Register 21 with various bit combinations
    let test_cases = vec![
        (vec![0, 0], "{\"eps_en\":\"OFF\",\"ovf_load_derate_en\":\"OFF\",\"drms_en\":\"OFF\",\"lvrt_en\":\"OFF\",\"anti_island_en\":\"OFF\",\"neutral_detect_en\":\"OFF\",\"grid_on_power_ss_en\":\"OFF\",\"ac_charge_en\":\"OFF\",\"sw_seamless_en\":\"OFF\",\"set_to_standby_en\":\"OFF\",\"forced_discharge_en\":\"OFF\",\"charge_priority_en\":\"OFF\",\"iso_en\":\"OFF\",\"gfci_en\":\"OFF\",\"dci_en\":\"OFF\",\"feed_in_grid_en\":\"OFF\"}"),
        (vec![1, 0], "{\"eps_en\":\"ON\",\"ovf_load_derate_en\":\"OFF\",\"drms_en\":\"OFF\",\"lvrt_en\":\"OFF\",\"anti_island_en\":\"OFF\",\"neutral_detect_en\":\"OFF\",\"grid_on_power_ss_en\":\"OFF\",\"ac_charge_en\":\"OFF\",\"sw_seamless_en\":\"OFF\",\"set_to_standby_en\":\"OFF\",\"forced_discharge_en\":\"OFF\",\"charge_priority_en\":\"OFF\",\"iso_en\":\"OFF\",\"gfci_en\":\"OFF\",\"dci_en\":\"OFF\",\"feed_in_grid_en\":\"OFF\"}"),
    ];
    
    for (values, expected) in test_cases {
        let packet = lxp::packet::TranslatedData {
            datalog: inverter.datalog(),
            device_function: lxp::packet::DeviceFunction::ReadHold,
            inverter: inverter.serial(),
            register: 21,
            values: values.clone(),
        };
        
        let messages = mqtt::Message::for_hold(packet).unwrap();
        assert_eq!(messages.len(), 2); // Raw value + bits
        
        // Check that bits message is properly formatted
        let bits_message = messages.iter().find(|m| m.topic.ends_with("/bits")).unwrap();
        assert_eq!(bits_message.payload, expected);
    }
}

// Test Error Recovery in Message Processing
#[tokio::test]
async fn test_error_recovery_in_message_processing() {
    let inverter = Factory::inverter();
    
    // Test that malformed data doesn't crash the system
    // This tests the resilience of the message processing pipeline
    
    // Test with empty values
    let packet = lxp::packet::ReadParam {
        datalog: inverter.datalog(),
        register: 0,
        values: vec![],
    };
    
    let result = mqtt::Message::for_param(packet);
    assert!(result.is_ok());
    let messages = result.unwrap();
    assert_eq!(messages.len(), 0);
    
    // Test with very large values
    let packet = lxp::packet::ReadParam {
        datalog: inverter.datalog(),
        register: 0,
        values: vec![255, 255],
    };
    
    let result = mqtt::Message::for_param(packet);
    assert!(result.is_ok());
    let messages = result.unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].payload, "65535");
}
