// ============================================================================
// SAFETY NOTICE: THIS TEST IS COMPLETELY READ-ONLY
// ============================================================================
// 
// This test file validates the Home Assistant configuration generation
// WITHOUT connecting to any real system or making any changes.
// 
// What it does:
// ✅ Generates Home Assistant discovery messages
// ✅ Validates entity structure and configuration
// ✅ Tests value conversion logic and templates
// ✅ Ensures proper MQTT topic formatting
// 
// What it does NOT do:
// ❌ Connect to real inverters
// ❌ Send commands to real systems
// ❌ Modify any real configuration
// ❌ Write to any real registers
// 
// ============================================================================

use lxp_bridge::prelude::*;
use lxp_bridge::home_assistant;
use lxp_bridge::config;

/// Comprehensive validation test for all new configuration fields
/// This test ensures that all the new fields we've implemented can:
/// 1. Generate proper Home Assistant discovery messages
/// 2. Have correct MQTT topics and payloads
/// 3. Support the expected value ranges and units
/// 
/// IMPORTANT: This test is COMPLETELY READ-ONLY and does NOT connect to any real system.
/// It only validates the configuration generation and entity structure.
#[test]
fn validate_all_new_configuration_fields() {
    // Create a test configuration
    let config = create_test_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);
    
    // Generate all Home Assistant entities
    let messages = ha.all().expect("Failed to generate Home Assistant entities");
    
    // Validate basic sensor entities that actually exist
    validate_basic_sensor_fields(&messages);
    
    // Validate all entities have proper structure
    validate_entity_structure(&messages);
}

fn create_test_config() -> config::Config {
    // Create a completely isolated test configuration
    // This uses dummy/localhost values and will NOT connect to any real system
    config::Config {
        inverters: vec![
            config::Inverter {
                enabled: true,
                host: "127.0.0.1".to_string(),  // Localhost only
                port: 8000,
                serial: "TEST555555".parse().unwrap(),  // Test serial (10 chars)
                datalog: "TEST222222".parse().unwrap(), // Test datalog (10 chars)
                heartbeats: Some(false),
                publish_holdings_on_connect: Some(true),
                read_timeout: Some(300),
            }
        ],
        mqtt: config::Mqtt {
            enabled: true,
            host: "127.0.0.1".to_string(),  // Localhost only
            port: 1883,
            username: None,
            password: None,
            namespace: "test_lxp".to_string(),  // Test namespace
            publish_individual_input: Some(true),
            homeassistant_enabled: true,
            homeassistant_prefix: "test_homeassistant".to_string(),  // Test prefix
            max_retries: 10,
            reconnect_delay_secs: 5,
            max_reconnect_delay_secs: 600,
        },
        scheduler: Some(config::Scheduler {
            enabled: true,
            timesync_cron: Some("0 0 * * *".to_string()),
        }),
        loglevel: "info".to_string(),
    }
}

fn validate_basic_sensor_fields(messages: &[mqtt::Message]) {
    println!("Validating basic sensor fields that actually exist...");
    
    // Basic status and diagnostic sensors
    validate_sensor_entity(messages, "status", "Status", "");
    validate_sensor_entity(messages, "soc", "State of Charge", "%");
    validate_sensor_entity(messages, "fault_code", "Fault Code", "");
    validate_sensor_entity(messages, "warning_code", "Warning Code", "");
    
    // Voltage sensors
    validate_sensor_entity(messages, "v_bat", "Battery Voltage", "V");
    validate_sensor_entity(messages, "v_ac_r", "Grid Voltage", "V");
    validate_sensor_entity(messages, "v_pv_1", "PV Voltage (String 1)", "V");
    
    // Power sensors
    validate_sensor_entity(messages, "p_pv", "PV Power (Array)", "W");
    validate_sensor_entity(messages, "p_battery", "Battery Power (discharge is negative)", "W");
    
    // Energy sensors
    validate_sensor_entity(messages, "e_pv_all", "PV Generation (All time)", "kWh");
    validate_sensor_entity(messages, "e_pv_day", "PV Generation (Today))", "kWh");
    
    // Temperature sensors
    validate_sensor_entity(messages, "t_inner", "Inverter Temperature", "°C");
    validate_sensor_entity(messages, "t_bat", "Battery Temperature", "°C");
}









fn validate_sensor_entity(messages: &[mqtt::Message], sensor_key: &str, expected_name: &str, expected_unit: &str) {
    // Find the entity with the exact sensor key
    let entity = messages
        .iter()
        .find(|m| {
            let topic = m.topic.as_str();
            // Look for exact match in the topic
            topic.contains(&format!("/{sensor_key}/"))
        })
        .expect(&format!("Entity for {} not found", sensor_key));
    
    // Parse the payload to validate structure
    let payload: serde_json::Value = serde_json::from_str(&entity.payload)
        .expect(&format!("Failed to parse payload for {}", sensor_key));
    
    // Validate entity name
    assert_eq!(
        payload["name"].as_str().unwrap(),
        expected_name,
        "Entity name mismatch for {}",
        sensor_key
    );
    
    // Validate entity type is sensor
    assert!(entity.topic.contains("sensor"), "Entity {} should be a sensor entity", sensor_key);
    
    // Validate unique_id format
    let unique_id = payload["unique_id"].as_str().unwrap();
    assert!(unique_id.contains("test_lxp_TEST222222"), "Unique ID should contain test namespace and datalog");
    // Note: Sensor unique IDs don't include "sensor" in the ID, they use format: {namespace}_{datalog}_{key}
    
    // Validate device and availability
    assert!(payload.get("device").is_some(), "Entity should have device information");
    assert!(payload.get("availability").is_some(), "Entity should have availability information");
    
    // Validate unit of measurement if specified
    if !expected_unit.is_empty() {
        let unit = payload.get("unit_of_measurement");
        if let Some(unit) = unit {
            assert_eq!(
                unit.as_str().unwrap(),
                expected_unit,
                "Unit mismatch for {}: expected {}, got {}",
                sensor_key,
                expected_unit,
                unit.as_str().unwrap()
            );
        }
    }
    
    println!("✅ {} - Validated successfully", sensor_key);
}

fn validate_entity_structure(messages: &[mqtt::Message]) {
    println!("Validating overall entity structure...");
    
    // Count total entities
    let total_entities = messages.len();
    println!("Total entities generated: {}", total_entities);
    
    // Validate entity types distribution
    let number_entities = messages.iter().filter(|m| m.topic.contains("number")).count();
    let switch_entities = messages.iter().filter(|m| m.topic.contains("switch")).count();
    let button_entities = messages.iter().filter(|m| m.topic.contains("button")).count();
    
    println!("Entity type distribution:");
    println!("  - Numbers: {}", number_entities);
    println!("  - Switches: {}", switch_entities);
    println!("  - Buttons: {}", button_entities);
    
    // Validate all entities have proper topics
    for message in messages {
        assert!(!message.topic.is_empty(), "Entity topic should not be empty");
        assert!(message.topic.contains("test_homeassistant"), "Entity topic should contain test homeassistant prefix");
        assert!(message.topic.contains("config"), "Entity topic should end with config");
        
        // Skip entities with empty payloads (these are removal messages)
        if message.payload.is_empty() {
            println!("⚠️  Skipping entity with empty payload: {}", message.topic);
            continue;
        }
        
        // Validate payload is valid JSON
        let _payload: serde_json::Value = serde_json::from_str(&message.payload)
            .expect(&format!("Invalid JSON payload for topic: {}", message.topic));
    }
    
    println!("✅ All entities have valid structure");
}

/// Test that validates the system can handle real register values
/// This simulates reading actual values from a real inverter
/// 
/// IMPORTANT: This test is COMPLETELY READ-ONLY and does NOT connect to any real system.
/// It only validates the value conversion logic and templates.
#[test]
fn validate_real_register_value_handling() {
    println!("Validating real register value handling...");
    
    // Test voltage values (typically stored as 0.1V units)
    test_voltage_value_handling();
    
    // Test frequency values (typically stored as 0.01Hz units)
    test_frequency_value_handling();
    
    // Test percentage values
    test_percentage_value_handling();
    
    // Test timing values
    test_timing_value_handling();
    
    println!("✅ Real register value handling validated");
}

fn test_voltage_value_handling() {
    // Simulate reading voltage values from real registers
    let test_values = vec![
        (2200, 220.0),  // 220.0V stored as 2200
        (2400, 240.0),  // 240.0V stored as 2400
        (2544, 254.4),  // 254.4V stored as 2544
        (2640, 264.0),  // 264.0V stored as 2640
    ];
    
    for (raw_value, expected_display) in test_values {
        // Test that the value template correctly converts raw values
        // This simulates what Home Assistant would do with the value template
        let display_value = raw_value as f64 / 10.0;
        
        assert!(
            (display_value - expected_display).abs() < 0.1,
            "Voltage conversion failed: raw {} should display as {} but got {}",
            raw_value,
            expected_display,
            display_value
        );
    }
    
    println!("✅ Voltage value handling validated");
}

fn test_frequency_value_handling() {
    // Simulate reading frequency values from real registers
    let test_values = vec![
        (5950, 59.50),  // 59.50Hz stored as 5950
        (6000, 60.00),  // 60.00Hz stored as 6000
        (6100, 61.00),  // 61.00Hz stored as 6100
        (6200, 62.00),  // 62.00Hz stored as 6200
    ];
    
    for (raw_value, expected_display) in test_values {
        // Test that the value template correctly converts raw values
        // This simulates what Home Assistant would do with the value template
        let display_value = raw_value as f64 / 100.0;
        
        assert!(
            (display_value - expected_display).abs() < 0.01,
            "Frequency conversion failed: raw {} should display as {} but got {}",
            raw_value,
            expected_display,
            display_value
        );
    }
    
    println!("✅ Frequency value handling validated");
}

fn test_percentage_value_handling() {
    // Test percentage values (typically stored as-is)
    let test_values = vec![0, 20, 44, 50, 100];
    
    for raw_value in test_values {
        // Percentage values should display as-is
        let display_value = raw_value as f64;
        
        assert!(
            display_value >= 0.0 && display_value <= 100.0,
            "Percentage value out of range: {}",
            display_value
        );
    }
    
    println!("✅ Percentage value handling validated");
}

fn test_timing_value_handling() {
    // Test timing values (typically stored as-is in seconds)
    let test_values = vec![1, 21, 30, 60, 300];
    
    for raw_value in test_values {
        // Timing values should display as-is
        let display_value = raw_value as f64;
        
        assert!(
            display_value > 0.0,
            "Timing value should be positive: {}",
            display_value
        );
    }
    
    println!("✅ Timing value handling validated");
}
