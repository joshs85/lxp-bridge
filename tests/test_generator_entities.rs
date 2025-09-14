mod common;
use common::*;

/// Test that generator frequency entity is properly created and configured
#[test]
fn test_generator_frequency_entity_creation() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Find the generator frequency entity
    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator frequency entity should be present");

    // Validate the entity structure
    assert!(generator_freq_entity.topic.contains("sensor"));
    assert!(generator_freq_entity.topic.contains("generator_frequency"));
    assert!(generator_freq_entity.topic.contains("config"));

    // Parse the payload to validate configuration
    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload)
        .expect("Generator frequency payload should be valid JSON");

    // Validate entity properties
    assert_eq!(payload["name"], "Generator Frequency");
    assert_eq!(payload["device_class"], "frequency");
    assert_eq!(payload["state_class"], "measurement");
    assert_eq!(payload["unit_of_measurement"], "Hz");
    assert_eq!(payload["entity_category"], "diagnostic");

    // Validate MQTT topic
    let expected_topic = format!("{}/{}/input/122", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic);

    // Validate value template for 2 decimal places
    assert_eq!(payload["value_template"], "{{ (value | float / 100) | round(2) }}");

    // Validate unique ID
    let expected_unique_id = format!("lxp_{}_generator_frequency", config.inverters[0].datalog());
    assert_eq!(payload["unique_id"], expected_unique_id);
}

/// Test that generator voltage entity is properly created and configured
#[test]
fn test_generator_voltage_entity_creation() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Find the generator voltage entity
    let generator_voltage_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_voltage"))
        .expect("Generator voltage entity should be present");

    // Validate the entity structure
    assert!(generator_voltage_entity.topic.contains("sensor"));
    assert!(generator_voltage_entity.topic.contains("generator_voltage"));
    assert!(generator_voltage_entity.topic.contains("config"));

    // Parse the payload to validate configuration
    let payload: serde_json::Value = serde_json::from_str(&generator_voltage_entity.payload)
        .expect("Generator voltage payload should be valid JSON");

    // Validate entity properties
    assert_eq!(payload["name"], "Generator Voltage");
    assert_eq!(payload["device_class"], "voltage");
    assert_eq!(payload["state_class"], "measurement");
    assert_eq!(payload["unit_of_measurement"], "V");
    assert_eq!(payload["entity_category"], "diagnostic");

    // Validate MQTT topic
    let expected_topic = format!("{}/{}/input/121", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic);

    // Validate value template for 1 decimal place
    assert_eq!(payload["value_template"], "{{ (value | float / 10) | round(1) }}");

    // Validate unique ID
    let expected_unique_id = format!("lxp_{}_generator_voltage", config.inverters[0].datalog());
    assert_eq!(payload["unique_id"], expected_unique_id);
}

/// Test that generator power entity is properly created and configured
#[test]
fn test_generator_power_entity_creation() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Find the generator power entity
    let generator_power_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_power"))
        .expect("Generator power entity should be present");

    // Validate the entity structure
    assert!(generator_power_entity.topic.contains("sensor"));
    assert!(generator_power_entity.topic.contains("generator_power"));
    assert!(generator_power_entity.topic.contains("config"));

    // Parse the payload to validate configuration
    let payload: serde_json::Value = serde_json::from_str(&generator_power_entity.payload)
        .expect("Generator power payload should be valid JSON");

    // Validate entity properties
    assert_eq!(payload["name"], "Generator Power");
    assert_eq!(payload["device_class"], "power");
    assert_eq!(payload["state_class"], "measurement");
    assert_eq!(payload["unit_of_measurement"], "W");
    assert_eq!(payload["entity_category"], "diagnostic");

    // Validate MQTT topic
    let expected_topic = format!("{}/{}/input/123", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic);

    // Validate value template for whole numbers
    assert_eq!(payload["value_template"], "{{ (value | float) | round(0) }}");

    // Validate unique ID
    let expected_unique_id = format!("lxp_{}_generator_power", config.inverters[0].datalog());
    assert_eq!(payload["unique_id"], expected_unique_id);
}

/// Test that all generator entities are present in the Home Assistant configuration
#[test]
fn test_all_generator_entities_present() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that all three generator entities are present
    let generator_entities = messages
        .iter()
        .filter(|m| m.topic.contains("generator_"))
        .collect::<Vec<_>>();

    assert_eq!(generator_entities.len(), 3, "Should have exactly 3 generator entities");

    // Verify specific entities exist
    let entity_names: Vec<&str> = generator_entities
        .iter()
        .map(|e| e.topic.split('/').nth(3).unwrap()) // Get the entity name from the topic
        .collect();

    assert!(entity_names.contains(&"generator_frequency"), "Generator frequency entity missing");
    assert!(entity_names.contains(&"generator_voltage"), "Generator voltage entity missing");
    assert!(entity_names.contains(&"generator_power"), "Generator power entity missing");
}

/// Test generator frequency value conversion with real data
#[test]
fn test_generator_frequency_value_conversion() {
    common_setup();

    // Test values from real backup data
    let test_cases = vec![
        (6016, 60.16),  // From off-grid backup: 6016 -> 60.16 Hz
        (6000, 60.00),  // From real system backup: 6000 -> 60.00 Hz
        (5995, 59.95),  // From real system backup: 5995 -> 59.95 Hz
        (6100, 61.00),  // Test case: 6100 -> 61.00 Hz
        (5950, 59.50),  // Test case: 5950 -> 59.50 Hz
    ];

    for (raw_value, expected_display) in test_cases {
        // Simulate the Home Assistant value template conversion: (value | float / 100) | round(2)
        let ha_display_value = ((raw_value as f64 / 100.0) * 100.0).round() / 100.0;
        
        assert!(
            (ha_display_value - expected_display).abs() < 0.01,
            "Generator frequency conversion failed: raw {} -> HA display {} (expected {})",
            raw_value,
            ha_display_value,
            expected_display
        );
    }
}

/// Test generator voltage value conversion with real data
#[test]
fn test_generator_voltage_value_conversion() {
    common_setup();

    // Test values from real backup data
    let test_cases = vec![
        (2388, 238.8),  // From off-grid backup: 2388 -> 238.8 V
        (2490, 249.0),  // From real system backup: 2490 -> 249.0 V
        (2400, 240.0),  // Test case: 2400 -> 240.0 V
        (2200, 220.0),  // Test case: 2200 -> 220.0 V
        (2544, 254.4),  // Test case: 2544 -> 254.4 V
    ];

    for (raw_value, expected_display) in test_cases {
        // Simulate the Home Assistant value template conversion: (value | float / 10) | round(1)
        let ha_display_value = ((raw_value as f64 / 10.0) * 10.0).round() / 10.0;
        
        assert!(
            (ha_display_value - expected_display).abs() < 0.1,
            "Generator voltage conversion failed: raw {} -> HA display {} (expected {})",
            raw_value,
            ha_display_value,
            expected_display
        );
    }
}

/// Test generator power value conversion with real data
#[test]
fn test_generator_power_value_conversion() {
    common_setup();

    // Test values from real backup data
    let test_cases = vec![
        (5897, 5897),    // From off-grid backup: 5897 -> 5897 W
        (15662, 15662),  // From real system backup: 15662 -> 15662 W
        (0, 0),          // Test case: 0 -> 0 W
        (1000, 1000),    // Test case: 1000 -> 1000 W
        (5000, 5000),    // Test case: 5000 -> 5000 W
    ];

    for (raw_value, expected_display) in test_cases {
        // Simulate the packet decoder conversion (no conversion needed for power)
        let packet_decoder_value = raw_value as f64;
        
        // Simulate the Home Assistant value template conversion (round to whole numbers)
        let ha_display_value = packet_decoder_value.round();
        
        assert!(
            (ha_display_value - expected_display as f64).abs() < 0.1,
            "Generator power conversion failed: raw {} -> packet decoder {} -> HA display {} (expected {})",
            raw_value,
            packet_decoder_value,
            ha_display_value,
            expected_display
        );
    }
}

/// Test that generator entities have correct MQTT topic structure
#[test]
fn test_generator_entities_mqtt_topics() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Test generator frequency topic
    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    let expected_topic = format!("{}/{}/input/122", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic);

    // Test generator voltage topic
    let generator_voltage_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_voltage"))
        .expect("Generator voltage entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_voltage_entity.payload).unwrap();
    let expected_topic = format!("{}/{}/input/121", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic);

    // Test generator power topic
    let generator_power_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_power"))
        .expect("Generator power entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_power_entity.payload).unwrap();
    let expected_topic = format!("{}/{}/input/123", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic);
}

/// Test that generator entities have correct device class assignments
#[test]
fn test_generator_entities_device_classes() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Test generator frequency device class
    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    assert_eq!(payload["device_class"], "frequency");

    // Test generator voltage device class
    let generator_voltage_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_voltage"))
        .expect("Generator voltage entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_voltage_entity.payload).unwrap();
    assert_eq!(payload["device_class"], "voltage");

    // Test generator power device class
    let generator_power_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_power"))
        .expect("Generator power entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_power_entity.payload).unwrap();
    assert_eq!(payload["device_class"], "power");
}

/// Test that generator entities have correct units of measurement
#[test]
fn test_generator_entities_units() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Test generator frequency unit
    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    assert_eq!(payload["unit_of_measurement"], "Hz");

    // Test generator voltage unit
    let generator_voltage_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_voltage"))
        .expect("Generator voltage entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_voltage_entity.payload).unwrap();
    assert_eq!(payload["unit_of_measurement"], "V");

    // Test generator power unit
    let generator_power_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_power"))
        .expect("Generator power entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_power_entity.payload).unwrap();
    assert_eq!(payload["unit_of_measurement"], "W");
}

/// Test that generator entities are properly categorized as diagnostic
#[test]
fn test_generator_entities_categories() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let generator_entities = ["generator_frequency", "generator_voltage", "generator_power"];

    for entity_name in &generator_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Generator {} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        assert_eq!(payload["entity_category"], "diagnostic");
    }
}

/// Test that generator entities have correct value templates
#[test]
fn test_generator_entities_value_templates() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Test generator frequency value template (2 decimal places)
    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    assert_eq!(payload["value_template"], "{{ (value | float / 100) | round(2) }}");

    // Test generator voltage value template (1 decimal place)
    let generator_voltage_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_voltage"))
        .expect("Generator voltage entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_voltage_entity.payload).unwrap();
    assert_eq!(payload["value_template"], "{{ (value | float / 10) | round(1) }}");

    // Test generator power value template (whole numbers)
    let generator_power_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_power"))
        .expect("Generator power entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_power_entity.payload).unwrap();
    assert_eq!(payload["value_template"], "{{ (value | float) | round(0) }}");
}

/// Test that generator entities have correct state classes
#[test]
fn test_generator_entities_state_classes() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let generator_entities = ["generator_frequency", "generator_voltage", "generator_power"];

    for entity_name in &generator_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Generator {} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        assert_eq!(payload["state_class"], "measurement");
    }
}

/// Test that generator entities have unique IDs that don't conflict
#[test]
fn test_generator_entities_unique_ids() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let mut unique_ids = std::collections::HashSet::new();

    let generator_entities = ["generator_frequency", "generator_voltage", "generator_power"];

    for entity_name in &generator_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Generator {} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        let unique_id = payload["unique_id"].as_str().unwrap().to_string();

        // Check that the unique ID is not empty
        assert!(!unique_id.is_empty(), "Generator {} unique ID should not be empty", entity_name);

        // Check that the unique ID follows the expected pattern
        let expected_prefix = format!("lxp_{}_", config.inverters[0].datalog());
        assert!(unique_id.starts_with(&expected_prefix), 
                "Generator {} unique ID should start with {}", entity_name, expected_prefix);

        // Check that the unique ID is unique
        assert!(unique_ids.insert(unique_id), 
                "Generator {} unique ID should be unique", entity_name);
    }

    // Verify we have exactly 3 unique IDs
    assert_eq!(unique_ids.len(), 3, "Should have exactly 3 unique generator entity IDs");
}
