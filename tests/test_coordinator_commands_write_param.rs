use lxp_bridge::prelude::*;
use lxp_bridge::coordinator::commands::write_param::WriteParam;
use lxp_bridge::channels::Channels;

#[tokio::test]
async fn test_write_param_creation() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let register: u16 = 100;
    let value: u16 = 500;
    
    let command = WriteParam::new(channels, inverter, register, value);
    
    assert_eq!(command.register(), 100);
    assert_eq!(command.value(), 500);
}

#[tokio::test]
async fn test_write_param_new() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let register: u16 = 200;
    let value: u16 = 1000;
    
    let command = WriteParam::new(channels, inverter, register, value);
    
    assert_eq!(command.register(), 200);
    assert_eq!(command.value(), 1000);
}

#[tokio::test]
async fn test_write_param_fields() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let register: u16 = 150;
    let value: u16 = 750;
    
    let command = WriteParam::new(channels, inverter, register, value);
    
    // Test that we can access command fields
    assert_eq!(command.register(), 150);
    assert_eq!(command.value(), 750);
}

#[tokio::test]
async fn test_write_param_register_conversion() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let value: u16 = 1000;
    
    // Test with different register types that implement Into<u16>
    let command1 = WriteParam::new(channels.clone(), inverter.clone(), 100u16, value);
    let command2 = WriteParam::new(channels.clone(), inverter.clone(), 100u8, value);
    let command3 = WriteParam::new(channels.clone(), inverter.clone(), 100u16, value);
    
    assert_eq!(command1.register(), 100);
    assert_eq!(command2.register(), 100);
    assert_eq!(command3.register(), 100);
}

#[tokio::test]
async fn test_write_param_value_range() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let register: u16 = 100;
    
    // Test various value ranges
    let test_values: Vec<u16> = vec![0, 100, 1000, 10000, 65535];
    
    for value in test_values {
        let command = WriteParam::new(channels.clone(), inverter.clone(), register, value);
        assert_eq!(command.value(), value);
    }
}

#[tokio::test]
async fn test_write_param_edge_cases() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test edge cases
    let edge_cases = vec![
        (0u16, 0u16),           // Minimum register and value
        (0u16, 65535u16),       // Minimum register, maximum value
        (65535u16, 0u16),       // Maximum register, minimum value
        (65535u16, 65535u16),   // Maximum register and value
        (32767u16, 32767u16),   // Middle values
    ];
    
    for (register, value) in edge_cases {
        let command = WriteParam::new(channels.clone(), inverter.clone(), register, value);
        assert_eq!(command.register(), register);
        assert_eq!(command.value(), value);
    }
}

#[tokio::test]
async fn test_write_param_multiple_instances() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test creating multiple instances
    let commands = vec![
        WriteParam::new(channels.clone(), inverter.clone(), 100u16, 500u16),
        WriteParam::new(channels.clone(), inverter.clone(), 200u16, 1000u16),
        WriteParam::new(channels.clone(), inverter.clone(), 300u16, 1500u16),
        WriteParam::new(channels.clone(), inverter.clone(), 400u16, 2000u16),
        WriteParam::new(channels.clone(), inverter.clone(), 500u16, 2500u16),
    ];
    
    assert_eq!(commands.len(), 5);
    
    for (i, command) in commands.iter().enumerate() {
        let expected_register: u16 = 100 + (i as u16 * 100);
        let expected_value: u16 = 500 + (i as u16 * 500);
        assert_eq!(command.register(), expected_register);
        assert_eq!(command.value(), expected_value);
    }
}

#[tokio::test]
async fn test_write_param_parameter_specific() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test various parameter types
    let parameter_tests = vec![
        (10u16, 30u16),     // Connection delay time (seconds)
        (11u16, 1u16),      // Reset setting
        (67u16, 208u16),    // AC Charge SOC Limit (20.8%)
        (134u16, 5999u16),  // Under Frequency Droop Start (59.99 Hz)
        (135u16, 5998u16),  // Under Frequency Droop End (59.98 Hz)
    ];
    
    for (register, value) in parameter_tests {
        let command = WriteParam::new(channels.clone(), inverter.clone(), register, value);
        assert_eq!(command.register(), register);
        assert_eq!(command.value(), value);
        
        // Test that the value is within reasonable bounds
        assert!(value >= 0);
        assert!(value <= 65535);
    }
}

#[tokio::test]
async fn test_write_param_parameter_validation() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test parameter validation scenarios
    let validation_tests = vec![
        (10u16, 0u16),      // Minimum connection delay
        (10u16, 300u16),    // Maximum connection delay
        (67u16, 0u16),      // Minimum SOC limit
        (67u16, 1000u16),   // Maximum SOC limit (100.0%)
        (134u16, 5000u16),  // Minimum frequency (50.00 Hz)
        (134u16, 7000u16),  // Maximum frequency (70.00 Hz)
    ];
    
    for (register, value) in validation_tests {
        let command = WriteParam::new(channels.clone(), inverter.clone(), register, value);
        assert_eq!(command.register(), register);
        assert_eq!(command.value(), value);
        
        // Verify the value makes sense for the parameter type
        match register {
            10 => assert!(value >= 0 && value <= 300),      // Connection delay: 0-300s
            67 => assert!(value >= 0 && value <= 1000),     // SOC limit: 0-100%
            134 => assert!(value >= 5000 && value <= 7000), // Frequency: 50-70 Hz
            _ => assert!(value >= 0 && value <= 65535),     // General range
        }
    }
}
