use lxp_bridge::coordinator::commands::write_param::WriteParam;
use lxp_bridge::config::Inverter;
use lxp_bridge::prelude::*;

fn create_test_inverter() -> Inverter {
    Inverter {
        host: "192.168.1.100".to_string(),
        port: 8000,
        serial: Serial::new(b"1234567890").unwrap(),
        datalog: Serial::new(b"1234567890").unwrap(),
        enabled: true,
        heartbeats: Some(false),
        publish_holdings_on_connect: Some(false),
        read_timeout: Some(900),
    }
}

#[test]
fn test_write_param_creation() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 100;
    let value: u16 = 500;
    
    let command = WriteParam::new(channels, inverter, register, value);
    
    assert_eq!(command.register(), 100);
    assert_eq!(command.value(), 500);
}

    #[test]
    fn test_write_param_register_conversion() {
        let channels = Channels::new();
        let inverter = create_test_inverter();
        let value: u16 = 1000;
        
        // Test with different register types that implement Into<u16>
        let command1 = WriteParam::new(channels.clone(), inverter.clone(), 100u16, value);
        let command2 = WriteParam::new(channels.clone(), inverter.clone(), 100u8, value);
        let command3 = WriteParam::new(channels.clone(), inverter.clone(), 100u16, value);
        
        assert_eq!(command1.register(), 100);
        assert_eq!(command2.register(), 100);
        assert_eq!(command3.register(), 100);
    }

#[test]
fn test_write_param_value_range() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 100;
    
    // Test various value ranges
    let test_values: Vec<u16> = vec![0, 100, 1000, 10000, 65535];
    
    for value in test_values {
        let command = WriteParam::new(channels.clone(), inverter.clone(), register, value);
        assert_eq!(command.value(), value);
    }
}

#[test]
fn test_write_param_edge_cases() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    
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

#[test]
fn test_write_param_getters() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 150;
    let value: u16 = 750;
    
    let command = WriteParam::new(channels, inverter, register, value);
    
    // Test that we can access command fields through getters
    assert_eq!(command.register(), 150);
    assert_eq!(command.value(), 750);
}
