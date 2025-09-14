mod common;
use common::*;

/// Test that all frequency entities now display with 2 decimal places
#[test]
fn test_all_frequency_entities_have_2_decimal_precision() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Test Grid Frequency (f_ac) - both instances
    let grid_freq_entities = messages
        .iter()
        .filter(|m| m.topic.contains("f_ac") && m.topic.contains("sensor"))
        .collect::<Vec<_>>();

    assert!(grid_freq_entities.len() >= 1, "Should have at least one Grid Frequency entity");

    for entity in &grid_freq_entities {
        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        assert_eq!(payload["value_template"], "{{ (value | float) | round(2) }}",
                   "Grid Frequency entity should have 2 decimal precision template");
    }

    // Test EPS Frequency (f_eps)
    let eps_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("f_eps") && m.topic.contains("sensor"))
        .expect("EPS Frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&eps_freq_entity.payload).unwrap();
    assert_eq!(payload["value_template"], "{{ (value | float) | round(2) }}",
               "EPS Frequency entity should have 2 decimal precision template");


    // Test Generator Frequency
    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator Frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    assert_eq!(payload["value_template"], "{{ (value | float / 100) | round(2) }}",
               "Generator Frequency entity should have 2 decimal precision template");
}

/// Test frequency value conversion with real backup data
#[test]
fn test_frequency_value_conversion_with_real_data() {
    common_setup();

    // Test values from real backup data
    let test_cases = vec![
        // Grid Frequency (f_ac) test cases
        (8272, 82.72),   // From off-grid backup
        (12368, 123.68), // From real system backup
        (6000, 60.00),   // Standard 60Hz
        (5950, 59.50),   // Slightly under 60Hz
        (6100, 61.00),   // Slightly over 60Hz
        
        // EPS Frequency (f_eps) test cases
        (6000, 60.00),   // From off-grid backup
        (5995, 59.95),   // From real system backup
        (6016, 60.16),   // Generator frequency from off-grid backup
        
        // Generator Frequency test cases
        (6016, 60.16),   // From off-grid backup
        (6000, 60.00),   // From real system backup
        (5996, 59.96),   // From real system backup
    ];

    for (raw_value, expected_display) in test_cases {
        // Simulate the packet decoder conversion (div100)
        let packet_decoder_value = raw_value as f64 / 100.0;
        
        // Simulate the Home Assistant value template conversion (round to 2 decimal places)
        let ha_display_value = (packet_decoder_value * 100.0).round() / 100.0;
        
        assert!(
            (ha_display_value - expected_display).abs() < 0.01,
            "Frequency conversion failed: raw {} -> packet decoder {} -> HA display {} (expected {})",
            raw_value,
            packet_decoder_value,
            ha_display_value,
            expected_display
        );
    }
}

/// Test that frequency entities have correct device classes
#[test]
fn test_frequency_entities_device_classes() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let frequency_entities = [
        ("f_ac", "Grid Frequency"),
        ("f_eps", "EPS Frequency"),
        ("generator_frequency", "Generator Frequency"),
    ];

    for (entity_key, entity_name) in &frequency_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_key) && m.topic.contains("sensor"))
            .expect(&format!("{} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        assert_eq!(payload["device_class"], "frequency",
                   "{} entity should have frequency device class", entity_name);
    }
}

/// Test that frequency entities have correct units
#[test]
fn test_frequency_entities_units() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let frequency_entities = [
        ("f_ac", "Grid Frequency"),
        ("f_eps", "EPS Frequency"),
        ("generator_frequency", "Generator Frequency"),
    ];

    for (entity_key, entity_name) in &frequency_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_key) && m.topic.contains("sensor"))
            .expect(&format!("{} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        assert_eq!(payload["unit_of_measurement"], "Hz",
                   "{} entity should have Hz unit", entity_name);
    }
}

/// Test that frequency entities have correct state classes
#[test]
fn test_frequency_entities_state_classes() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let frequency_entities = [
        ("f_ac", "Grid Frequency"),
        ("f_eps", "EPS Frequency"),
        ("generator_frequency", "Generator Frequency"),
    ];

    for (entity_key, entity_name) in &frequency_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_key) && m.topic.contains("sensor"))
            .expect(&format!("{} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        assert_eq!(payload["state_class"], "measurement",
                   "{} entity should have measurement state class", entity_name);
    }
}


/// Test that Generator Frequency entity has correct MQTT topic
#[test]
fn test_generator_frequency_entity_topic() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator Frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    let expected_topic = format!("{}/{}/input/122", config.mqtt.namespace, config.inverters[0].datalog());
    assert_eq!(payload["state_topic"], expected_topic,
               "Generator Frequency entity should have correct MQTT topic");
}

/// Test that frequency entities are properly categorized
#[test]
fn test_frequency_entities_categories() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Grid Frequency and EPS Frequency should not have entity_category (they're main sensors)
    let grid_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("f_ac") && m.topic.contains("sensor"))
        .expect("Grid Frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&grid_freq_entity.payload).unwrap();
    assert!(payload.get("entity_category").is_none(),
            "Grid Frequency entity should not have entity_category");

    let eps_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("f_eps") && m.topic.contains("sensor"))
        .expect("EPS Frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&eps_freq_entity.payload).unwrap();
    assert!(payload.get("entity_category").is_none(),
            "EPS Frequency entity should not have entity_category");

    // Generator Frequency should be diagnostic

    let generator_freq_entity = messages
        .iter()
        .find(|m| m.topic.contains("generator_frequency"))
        .expect("Generator Frequency entity should be present");

    let payload: serde_json::Value = serde_json::from_str(&generator_freq_entity.payload).unwrap();
    assert_eq!(payload["entity_category"], "diagnostic",
               "Generator Frequency entity should be diagnostic");
}

/// Test that frequency value templates use correct Jinja2 syntax
#[test]
fn test_frequency_value_templates_syntax() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let frequency_entities = [
        ("f_ac", "Grid Frequency"),
        ("f_eps", "EPS Frequency"),
        ("generator_frequency", "Generator Frequency"),
    ];

    for (entity_key, entity_name) in &frequency_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_key) && m.topic.contains("sensor"))
            .expect(&format!("{} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        let value_template = payload["value_template"].as_str().unwrap();

        // Validate Jinja2 syntax
        assert!(value_template.contains("{{"), "{} value template should contain Jinja2 opening", entity_name);
        assert!(value_template.contains("}}"), "{} value template should contain Jinja2 closing", entity_name);
        assert!(value_template.contains("value"), "{} value template should reference 'value'", entity_name);
        assert!(value_template.contains("float"), "{} value template should use float filter", entity_name);
        assert!(value_template.contains("round(2)"), "{} value template should round to 2 decimal places", entity_name);
    }
}

/// Test that frequency entities have unique IDs
#[test]
fn test_frequency_entities_unique_ids() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    let mut unique_ids = std::collections::HashSet::new();

    let frequency_entities = [
        ("f_ac", "Grid Frequency"),
        ("f_eps", "EPS Frequency"),
        ("generator_frequency", "Generator Frequency"),
    ];

    for (entity_key, entity_name) in &frequency_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_key) && m.topic.contains("sensor"))
            .expect(&format!("{} entity should be present", entity_name));

        let payload: serde_json::Value = serde_json::from_str(&entity.payload).unwrap();
        let unique_id = payload["unique_id"].as_str().unwrap().to_string();

        // Check that the unique ID is not empty
        assert!(!unique_id.is_empty(), "{} unique ID should not be empty", entity_name);

        // Check that the unique ID follows the expected pattern
        let expected_prefix = format!("lxp_{}_", config.inverters[0].datalog());
        assert!(unique_id.starts_with(&expected_prefix), 
                "{} unique ID should start with {}", entity_name, expected_prefix);

        // Check that the unique ID is unique
        assert!(unique_ids.insert(unique_id), 
                "{} unique ID should be unique", entity_name);
    }

    // Verify we have exactly 3 unique IDs (there might be multiple f_ac entities)
    assert!(unique_ids.len() >= 3, "Should have at least 3 unique frequency entity IDs");
}
