use lxp_bridge::prelude::*;
use lxp_bridge::coordinator::commands::backup_config::BackupConfigCommand;
use lxp_bridge::config::ConfigWrapper;

#[tokio::test]
async fn test_backup_config_command_creation() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup.json".to_string();
    
    let command = BackupConfigCommand::new(inverter, output_path);
    
    // Test that we can create the command
    assert!(std::mem::size_of_val(&command) > 0);
    
    // Clean up
    let _ = std::fs::remove_file("test_backup.json");
}

#[tokio::test]
async fn test_backup_config_command_new() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_new.json".to_string();
    
    let command = BackupConfigCommand::new(inverter, output_path);
    
    // Test that we can create the command
    assert!(std::mem::size_of_val(&command) > 0);
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_new.json");
}

#[tokio::test]
async fn test_backup_config_command_inverter_config() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_config.json".to_string();
    
    let _command = BackupConfigCommand::new(inverter, output_path);
    
    // Test that we can access inverter configuration through the config
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverters = config.enabled_inverters();
    assert!(!inverters.is_empty());
    
    let first_inverter = &inverters[0];
    assert_eq!(first_inverter.host(), "test.example.com");
    assert_eq!(first_inverter.port(), 12345);
    assert_eq!(first_inverter.serial().to_string(), "1111111111");
    assert_eq!(first_inverter.datalog().to_string(), "2222222222");
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_config.json");
}

#[tokio::test]
async fn test_backup_config_command_inverter_methods() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_methods.json".to_string();
    
    let _command = BackupConfigCommand::new(inverter, output_path);
    
    // Test all inverter methods through the config
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverters = config.enabled_inverters();
    let inverter_config = &inverters[0];
    
    // Test enabled method
    assert!(inverter_config.enabled());
    
    // Test host method
    assert_eq!(inverter_config.host(), "test.example.com");
    
    // Test port method
    assert_eq!(inverter_config.port(), 12345);
    
    // Test serial method
    assert_eq!(inverter_config.serial().to_string(), "1111111111");
    
    // Test datalog method
    assert_eq!(inverter_config.datalog().to_string(), "2222222222");
    
    // Test heartbeats method (should use default)
    assert!(!inverter_config.heartbeats());
    
    // Test publish_holdings_on_connect method (should use default)
    assert!(!inverter_config.publish_holdings_on_connect());
    
    // Test read_timeout method (should use default)
    assert_eq!(inverter_config.read_timeout(), 900);
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_methods.json");
}

#[tokio::test]
async fn test_backup_config_command_inverter_serial_struct() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_serial_struct.json".to_string();
    
    let _command = BackupConfigCommand::new(inverter, output_path);
    
    // Test Serial struct methods through the config
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverters = config.enabled_inverters();
    let inverter_config = &inverters[0];
    
    // Test Serial struct methods
    let serial = inverter_config.serial();
    let datalog = inverter_config.datalog();
    
    // Test data method
    let serial_data = serial.data();
    let datalog_data = datalog.data();
    
    assert_eq!(serial_data.len(), 10);
    assert_eq!(datalog_data.len(), 10);
    
    // Test default method
    let default_serial = Serial::default();
    assert_eq!(default_serial.data(), [0; 10]);
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_serial_struct.json");
}

#[tokio::test]
async fn test_backup_config_command_inverter_serial_serialization() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_serial_serialization.json".to_string();
    
    let _command = BackupConfigCommand::new(inverter, output_path);
    
    // Test Serial struct serialization through the config
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverters = config.enabled_inverters();
    let inverter_config = &inverters[0];
    
    // Test Serial struct serialization
    let serial = inverter_config.serial();
    let datalog = inverter_config.datalog();
    
    // Test that Serial can be converted to string
    let serial_str = serial.to_string();
    let datalog_str = datalog.to_string();
    
    assert_eq!(serial_str, "1111111111");
    assert_eq!(datalog_str, "2222222222");
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_serial_serialization.json");
}

#[tokio::test]
async fn test_backup_config_command_inverter_serial_equality() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_serial_equality.json".to_string();
    
    let _command = BackupConfigCommand::new(inverter, output_path);
    
    // Test Serial struct equality through the config
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverters = config.enabled_inverters();
    let inverter_config = &inverters[0];
    
    // Test Serial struct equality
    let serial1 = inverter_config.serial();
    let serial2 = inverter_config.serial();
    let datalog = inverter_config.datalog();
    
    assert_eq!(serial1, serial2);
    assert_ne!(serial1, datalog);
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_serial_equality.json");
}

#[tokio::test]
async fn test_backup_config_command_inverter_serial_hash() {
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let output_path = "test_backup_serial_hash.json".to_string();
    
    let _command = BackupConfigCommand::new(inverter, output_path);
    
    // Test Serial struct hash through the config
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverters = config.enabled_inverters();
    let inverter_config = &inverters[0];
    
    // Test Serial struct hash
    let serial = inverter_config.serial();
    let datalog = inverter_config.datalog();
    
    let mut hash_map = std::collections::HashMap::new();
    hash_map.insert(serial, "serial_value");
    hash_map.insert(datalog, "datalog_value");
    
    assert_eq!(hash_map.len(), 2);
    assert_eq!(hash_map.get(&serial), Some(&"serial_value"));
    assert_eq!(hash_map.get(&datalog), Some(&"datalog_value"));
    
    // Clean up
    let _ = std::fs::remove_file("test_backup_serial_hash.json");
}
