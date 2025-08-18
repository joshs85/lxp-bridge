mod common;
use common::*;

#[tokio::test]
async fn all_has_soc() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the SOC sensor exists with the correct topic
    let soc_message = messages.iter().find(|msg| msg.topic == "homeassistant/sensor/lxp_2222222222/soc/config");
    assert!(soc_message.is_some(), "SOC sensor not found");
    
    // Check that the payload contains the expected fields
    let payload = &soc_message.unwrap().payload;
    assert!(payload.contains("State of Charge"), "Missing name");
    assert!(payload.contains("measurement"), "Missing entity_category");
    assert!(payload.contains("battery"), "Missing device_class");
}

#[tokio::test]
async fn all_has_v_pv_1() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the PV voltage sensor exists with the correct topic
    let pv_message = messages.iter().find(|msg| msg.topic == "homeassistant/sensor/lxp_2222222222/v_pv_1/config");
    assert!(pv_message.is_some(), "PV voltage sensor not found");
    
    // Check that the payload contains the expected fields
    let payload = &pv_message.unwrap().payload;
    assert!(payload.contains("PV Voltage (String 1)"), "Missing name");
    assert!(payload.contains("measurement"), "Missing entity_category");
    assert!(payload.contains("voltage"), "Missing device_class");
}

#[tokio::test]
async fn all_has_p_pv() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the PV power sensor exists with the correct topic
    let pv_message = messages.iter().find(|msg| msg.topic == "homeassistant/sensor/lxp_2222222222/p_pv/config");
    assert!(pv_message.is_some(), "PV power sensor not found");
    
    // Check that the payload contains the expected fields
    let payload = &pv_message.unwrap().payload;
    assert!(payload.contains("PV Power (Array)"), "Missing name");
    assert!(payload.contains("measurement"), "Missing entity_category");
    assert!(payload.contains("power"), "Missing device_class");
}

#[tokio::test]
async fn all_has_e_pv_all() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the PV energy sensor exists with the correct topic
    let pv_message = messages.iter().find(|msg| msg.topic == "homeassistant/sensor/lxp_2222222222/e_pv_all/config");
    assert!(pv_message.is_some(), "PV energy sensor not found");
    
    // Check that the payload contains the expected fields
    let payload = &pv_message.unwrap().payload;
    assert!(payload.contains("PV Generation (All time)"), "Missing name");
    assert!(payload.contains("measurement"), "Missing entity_category");
    assert!(payload.contains("energy"), "Missing device_class");
}

#[tokio::test]
async fn all_has_fault_code() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the fault code sensor exists with the correct topic
    let fault_message = messages.iter().find(|msg| msg.topic == "homeassistant/sensor/lxp_2222222222/fault_code/config");
    assert!(fault_message.is_some(), "Fault code sensor not found");
    
    // Check that the payload contains the expected fields
    let payload = &fault_message.unwrap().payload;
    assert!(payload.contains("Fault Code"), "Missing name");
    assert!(payload.contains("diagnostic"), "Missing entity_category");
    assert!(payload.contains("mdi:alert"), "Missing icon");
}

#[tokio::test]
async fn all_has_switch_ac_charge() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the AC charge switch exists with the correct topic
    let switch_message = messages.iter().find(|msg| msg.topic == "homeassistant/switch/lxp_2222222222/ac_charge/config");
    assert!(switch_message.is_some(), "AC charge switch not found");
    
    // Check that the payload contains the expected fields
    let payload = &switch_message.unwrap().payload;
    assert!(payload.contains("AC Charge"), "Missing name");
    // Basic operational controls should be in primary controls (no entity_category)
    assert!(!payload.contains("entity_category"), "Basic operational controls should not have entity_category");
}

#[tokio::test]
async fn all_has_number_ac_charge_soc_limit_pct() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the AC charge SOC limit number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/AcChargeSocLimit/config");
    assert!(number_message.is_some(), "AC charge SOC limit number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("AC Charge Limit %"), "Missing name");
    // Basic operational controls should be in primary controls (no entity_category)
    assert!(!payload.contains("entity_category"), "Basic operational controls should not have entity_category");
    assert!(payload.contains("0.0"), "Missing min value");
    assert!(payload.contains("100.0"), "Missing max value");
}

#[tokio::test]
async fn all_has_time_range_ac_charge_1() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the AC charge time range text exists with the correct topic
    let text_message = messages.iter().find(|msg| msg.topic == "homeassistant/text/lxp_2222222222/ac_charge_1/config");
    assert!(text_message.is_some(), "AC charge time range text not found");
    
    // Check that the payload contains the expected fields
    let payload = &text_message.unwrap().payload;
    assert!(payload.contains("AC Charge Timeslot 1"), "Missing name");
    // Basic operational controls should be in primary controls (no entity_category)
    assert!(!payload.contains("entity_category"), "Basic operational controls should not have entity_category");
}

#[test]
fn all_has_under_frequency_controls() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that under frequency droop start control is present
    let under_fr_start = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopStart"))
        .expect("Under Frequency Droop Start control should be present");

    assert!(under_fr_start.payload.contains("Under Frequency Droop Start (Hz)"));
    assert!(under_fr_start.payload.contains("Hz"));
    assert!(under_fr_start.payload.contains("0.0"));
    assert!(under_fr_start.payload.contains("100.0"));
    assert!(under_fr_start.payload.contains("0.01"));

    // Check that under frequency droop end control is present
    let under_fr_end = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopEnd"))
        .expect("Under Frequency Droop End control should be present");

    assert!(under_fr_end.payload.contains("Under Frequency Droop End (Hz)"));
    assert!(under_fr_end.payload.contains("Hz"));
    assert!(under_fr_end.payload.contains("0.0"));
    assert!(under_fr_end.payload.contains("100.0"));
    assert!(under_fr_end.payload.contains("0.01"));

    // Verify both controls use the same configuration pattern as OVF controls
    let ovf_start = messages
        .iter()
        .find(|m| m.topic.contains("OVFDerateStart"))
        .expect("OVF Derate Start control should be present");

    // Both should have similar configurations
    assert_eq!(under_fr_start.payload.contains("Hz"), ovf_start.payload.contains("Hz"));
    assert_eq!(under_fr_start.payload.contains("0.0"), ovf_start.payload.contains("0.0"));
    assert_eq!(under_fr_start.payload.contains("100.0"), ovf_start.payload.contains("100.0"));
    assert_eq!(under_fr_start.payload.contains("0.01"), ovf_start.payload.contains("0.01"));
}

#[test]
fn under_frequency_controls_have_correct_topics() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check under frequency droop start topics
    let under_fr_start = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopStart"))
        .unwrap();

    let payload: serde_json::Value = serde_json::from_str(&under_fr_start.payload).unwrap();
    
    // State topic should read from hold register
    assert!(payload["state_topic"].as_str().unwrap().contains("hold/134"));
    
    // Command topic should send to the command endpoint
    assert!(payload["command_topic"].as_str().unwrap().contains("cmd"));
    assert!(payload["command_topic"].as_str().unwrap().contains("set/under_fr_droop_start_hz"));

    // Check under frequency droop end topics
    let under_fr_end = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopEnd"))
        .unwrap();

    let payload: serde_json::Value = serde_json::from_str(&under_fr_end.payload).unwrap();
    
    // State topic should read from hold register
    assert!(payload["state_topic"].as_str().unwrap().contains("hold/135"));
    
    // Command topic should send to the command endpoint
    assert!(payload["command_topic"].as_str().unwrap().contains("cmd"));
    assert!(payload["command_topic"].as_str().unwrap().contains("set/under_fr_droop_end_hz"));
}

#[test]
fn under_frequency_controls_have_correct_value_template() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check under frequency droop start value template
    let under_fr_start = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopStart"))
        .unwrap();

    let payload: serde_json::Value = serde_json::from_str(&under_fr_start.payload).unwrap();
    
    // Value template should divide by 100 to convert from centi-Hz to Hz
    assert_eq!(payload["value_template"], "{{ float(value) / 100 }}");

    // Check under frequency droop end value template
    let under_fr_end = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopEnd"))
        .unwrap();

    let payload: serde_json::Value = serde_json::from_str(&under_fr_end.payload).unwrap();
    
    // Value template should divide by 100 to convert from centi-Hz to Hz
    assert_eq!(payload["value_template"], "{{ float(value) / 100 }}");
}

// ===== CONNECTION & RECONNECTION TESTS =====

#[tokio::test]
async fn all_has_connection_delay_time() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the connection delay time number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/GridConnectTime/config");
    assert!(number_message.is_some(), "Connection delay time number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("Grid Connection Delay (s)"), "Missing name");
    assert!(payload.contains("config"), "Missing entity_category");
    assert!(payload.contains("30.0"), "Missing min value");
    assert!(payload.contains("600.0"), "Missing max value");
    assert!(payload.contains("s"), "Missing unit");
}

#[tokio::test]
async fn all_has_reconnection_delay_time() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the reconnection delay time number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/GridReconnectTime/config");
    assert!(number_message.is_some(), "Reconnection delay time number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("Grid Reconnection Delay (s)"), "Missing name");
    assert!(payload.contains("config"), "Missing entity_category");
    assert!(payload.contains("0.0"), "Missing min value");
    assert!(payload.contains("900.0"), "Missing max value");
    assert!(payload.contains("s"), "Missing unit");
}

#[tokio::test]
async fn all_has_grid_voltage_low_limit() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the grid voltage low limit number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/GridVoltConnLow/config");
    assert!(number_message.is_some(), "Grid voltage low limit number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("Grid Voltage Low Limit (V)"), "Missing name");
    assert!(payload.contains("config"), "Missing entity_category");
    assert!(payload.contains("180.0"), "Missing min value");
    assert!(payload.contains("280.0"), "Missing max value");
    assert!(payload.contains("V"), "Missing unit");
}

#[tokio::test]
async fn all_has_grid_voltage_high_limit() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the grid voltage high limit number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/GridVoltConnHigh/config");
    assert!(number_message.is_some(), "Grid voltage high limit number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("Grid Voltage High Limit (V)"), "Missing name");
    assert!(payload.contains("config"), "Missing entity_category");
    assert!(payload.contains("180.0"), "Missing min value");
    assert!(payload.contains("280.0"), "Missing max value");
    assert!(payload.contains("V"), "Missing unit");
}

#[tokio::test]
async fn all_has_grid_frequency_low_limit() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the grid frequency low limit number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/GridFreqConnLow/config");
    assert!(number_message.is_some(), "Grid frequency low limit number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("Grid Frequency Low Limit (Hz)"), "Missing name");
    assert!(payload.contains("config"), "Missing entity_category");
    assert!(payload.contains("47.5"), "Missing min value");
    assert!(payload.contains("63.0"), "Missing max value");
    assert!(payload.contains("Hz"), "Missing unit");
}

#[tokio::test]
async fn all_has_grid_frequency_high_limit() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    let messages = r.unwrap();
    
    // Check that the grid frequency high limit number exists with the correct topic
    let number_message = messages.iter().find(|msg| msg.topic == "homeassistant/number/lxp_2222222222/GridFreqConnHigh/config");
    assert!(number_message.is_some(), "Grid frequency high limit number not found");
    
    // Check that the payload contains the expected fields
    let payload = &number_message.unwrap().payload;
    assert!(payload.contains("Grid Frequency High Limit (Hz)"), "Missing name");
    assert!(payload.contains("config"), "Missing entity_category");
    assert!(payload.contains("47.5"), "Missing min value");
    assert!(payload.contains("63.0"), "Missing max value");
    assert!(payload.contains("Hz"), "Missing unit");
}

#[test]
fn connection_controls_have_correct_value_templates() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that voltage controls have correct value template (convert from 0.1V units)
    let voltage_low = messages
        .iter()
        .find(|m| m.topic.contains("GridVoltConnLow"))
        .expect("Grid Voltage Low Limit control should be present");

    assert!(voltage_low.payload.contains("{{ float(value) / 10 }}"), "Value template should convert from 0.1V units");

    let voltage_high = messages
        .iter()
        .find(|m| m.topic.contains("GridVoltConnHigh"))
        .expect("Grid Voltage High Limit control should be present");

    assert!(voltage_high.payload.contains("{{ float(value) / 10 }}"), "Value template should convert from 0.1V units");

    // Check that frequency controls have correct value template (convert from 0.01Hz units)
    let frequency_low = messages
        .iter()
        .find(|m| m.topic.contains("GridFreqConnLow"))
        .expect("Grid Frequency Low Limit control should be present");

    assert!(frequency_low.payload.contains("{{ float(value) / 100 }}"), "Value template should convert from 0.01Hz units");

    let frequency_high = messages
        .iter()
        .find(|m| m.topic.contains("GridFreqConnHigh"))
        .expect("Grid Frequency High Limit control should be present");

    assert!(frequency_high.payload.contains("{{ float(value) / 100 }}"), "Value template should convert from 0.01Hz units");

    // Check that time controls have correct value template (no conversion needed)
    let connect_time = messages
        .iter()
        .find(|m| m.topic.contains("GridConnectTime"))
        .expect("Connection Delay Time control should be present");

    assert!(connect_time.payload.contains("{{ value }}"), "Value template should not convert time values");

    let reconnect_time = messages
        .iter()
        .find(|m| m.topic.contains("GridReconnectTime"))
        .expect("Reconnection Delay Time control should be present");

    assert!(reconnect_time.payload.contains("{{ value }}"), "Value template should not convert time values");
}

#[test]
fn connection_controls_have_correct_mqtt_topics() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that all connection controls have correct command topics
    let connect_time = messages
        .iter()
        .find(|m| m.topic.contains("GridConnectTime"))
        .expect("Connection Delay Time control should be present");

    assert!(connect_time.payload.contains("lxp/cmd/2222222222/set/grid_connect_time"), "Command topic should be correct");

    let reconnect_time = messages
        .iter()
        .find(|m| m.topic.contains("GridReconnectTime"))
        .expect("Reconnection Delay Time control should be present");

    assert!(reconnect_time.payload.contains("lxp/cmd/2222222222/set/grid_reconnect_time"), "Command topic should be correct");

    let voltage_low = messages
        .iter()
        .find(|m| m.topic.contains("GridVoltConnLow"))
        .expect("Grid Voltage Low Limit control should be present");

    assert!(voltage_low.payload.contains("lxp/cmd/2222222222/set/grid_voltage_low"), "Command topic should be correct");

    let voltage_high = messages
        .iter()
        .find(|m| m.topic.contains("GridVoltConnHigh"))
        .expect("Grid Voltage High Limit control should be present");

    assert!(voltage_high.payload.contains("lxp/cmd/2222222222/set/grid_voltage_high"), "Command topic should be correct");

    let frequency_low = messages
        .iter()
        .find(|m| m.topic.contains("GridFreqConnLow"))
        .expect("Grid Frequency Low Limit control should be present");

    assert!(frequency_low.payload.contains("lxp/cmd/2222222222/set/grid_frequency_low"), "Command topic should be correct");

    let frequency_high = messages
        .iter()
        .find(|msg| msg.topic.contains("GridFreqConnHigh"))
        .expect("Grid Frequency High Limit control should be present");

    assert!(frequency_high.payload.contains("lxp/cmd/2222222222/set/grid_frequency_high"), "Command topic should be correct");
}

#[test]
fn interface_protection_entities_are_disabled_by_default() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that interface protection entities are disabled by default
    let interface_protection_entities = [
        "GridVoltLimit1Low", "GridVoltLimit1High", "GridVoltLimit1LowTime", "GridVoltLimit1HighTime",
        "GridVoltLimit2Low", "GridVoltLimit2High", "GridVoltLimit2LowTime",
        "GridVoltLimit3Low", "GridVoltLimit3High", "GridVoltLimit3LowTime", "GridVoltLimit3HighTime",
        "GridFreqLimit1Low", "GridFreqLimit1High", "GridFreqLimit1LowTime", "GridFreqLimit1HighTime",
        "GridFreqLimit2Low", "GridFreqLimit2High", "GridFreqLimit2LowTime", "GridFreqLimit2HighTime",
        "GridFreqLimit3Low", "GridFreqLimit3High", "GridFreqLimit3LowTime", "GridFreqLimit3HighTime",
    ];

    for entity_name in &interface_protection_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Interface protection entity {} should be present", entity_name));

        // Check that the entity is disabled by default
        assert!(entity.payload.contains("\"enabled_by_default\":false"), 
                "Entity {} should be disabled by default", entity_name);
    }
}

#[test]
fn connection_and_reconnection_entities_are_disabled_by_default() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that connection and reconnection entities are disabled by default
    let connection_entities = [
        "GridConnectTime", "GridReconnectTime", "GridVoltConnLow", "GridVoltConnHigh", 
        "GridFreqConnLow", "GridFreqConnHigh"
    ];

    for entity_name in &connection_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Connection entity {} should be present", entity_name));

        // Check that the entity is disabled by default
        assert!(entity.payload.contains("\"enabled_by_default\":false"), 
                "Entity {} should be disabled by default", entity_name);
    }
}

#[test]
fn regular_entities_are_enabled_by_default() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that regular entities are enabled by default (no enabled_by_default field or it's true)
    let regular_entities = [
        "ForcedDischgSocLimit", "DischgCutOffSocEod", "EpsDischgCutoffSocEod",
        "UnderFrDroopStart", "UnderFrDroopEnd", "UnderFrIncreasePctPerHz", "DelayTimeForOverFDerate"
    ];

    for entity_name in &regular_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Regular entity {} should be present", entity_name));

        // Check that the entity either has no enabled_by_default field (defaults to enabled)
        // or explicitly has enabled_by_default: true
        let payload = &entity.payload;
        assert!(
            !payload.contains("\"enabled_by_default\":false") && 
            (payload.contains("\"enabled_by_default\":true") || !payload.contains("enabled_by_default")),
            "Entity {} should be enabled by default", entity_name
        );
    }
}

#[test]
fn interface_protection_entities_have_correct_value_templates() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that voltage limit entities have correct value template (convert from 0.1V units)
    let voltage_entities = [
        "GridVoltLimit1Low", "GridVoltLimit1High", "GridVoltLimit2Low", "GridVoltLimit2High",
        "GridVoltLimit3Low", "GridVoltLimit3High"
    ];

    for entity_name in &voltage_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Voltage entity {} should be present", entity_name));

        assert!(entity.payload.contains("{{ float(value) / 10 }}"), 
                "Voltage entity {} should convert from 0.1V units", entity_name);
    }

    // Check that frequency limit entities have correct value template (convert from 0.01Hz units)
    let frequency_entities = [
        "GridFreqLimit1Low", "GridFreqLimit1High", "GridFreqLimit2Low", "GridFreqLimit2High",
        "GridFreqLimit3Low", "GridFreqLimit3High"
    ];

    for entity_name in &frequency_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Frequency entity {} should be present", entity_name));

        assert!(entity.payload.contains("{{ float(value) / 100 }}"), 
                "Frequency entity {} should convert from 0.01Hz units", entity_name);
    }

    // Check that time limit entities have correct value template (convert from 0.01s units)
    let time_entities = [
        "GridVoltLimit1LowTime", "GridVoltLimit1HighTime", "GridVoltLimit2LowTime",
        "GridVoltLimit3LowTime", "GridVoltLimit3HighTime",
        "GridFreqLimit1LowTime", "GridFreqLimit1HighTime", "GridFreqLimit2LowTime", "GridFreqLimit2HighTime",
        "GridFreqLimit3LowTime", "GridFreqLimit3HighTime"
    ];

    for entity_name in &time_entities {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Time entity {} should be present", entity_name));

        assert!(entity.payload.contains("{{ float(value) / 100 }}"), 
                "Time entity {} should convert from 0.01s units", entity_name);
    }
}

#[test]
fn interface_protection_entities_have_correct_mqtt_topics() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that interface protection entities have correct command topics
    let entity_topic_pairs = [
        ("GridVoltLimit1Low", "grid_volt_limit1_low"),
        ("GridVoltLimit1High", "grid_volt_limit1_high"),
        ("GridVoltLimit1LowTime", "grid_volt_limit1_low_time"),
        ("GridVoltLimit1HighTime", "grid_volt_limit1_high_time"),
        ("GridVoltLimit2Low", "grid_volt_limit2_low"),
        ("GridVoltLimit2High", "grid_volt_limit2_high"),
        ("GridVoltLimit2LowTime", "grid_volt_limit2_low_time"),
        ("GridVoltLimit3Low", "grid_volt_limit3_low"),
        ("GridVoltLimit3High", "grid_volt_limit3_high"),
        ("GridVoltLimit3LowTime", "grid_volt_limit3_low_time"),
        ("GridVoltLimit3HighTime", "grid_volt_limit3_high_time"),
        ("GridFreqLimit1Low", "grid_freq_limit1_low"),
        ("GridFreqLimit1High", "grid_freq_limit1_high"),
        ("GridFreqLimit1LowTime", "grid_freq_limit1_low_time"),
        ("GridFreqLimit1HighTime", "grid_freq_limit1_high_time"),
        ("GridFreqLimit2Low", "grid_freq_limit2_low"),
        ("GridFreqLimit2High", "grid_freq_limit2_high"),
        ("GridFreqLimit2LowTime", "grid_freq_limit2_low_time"),
        ("GridFreqLimit2HighTime", "grid_freq_limit2_high_time"),
        ("GridFreqLimit3Low", "grid_freq_limit3_low"),
        ("GridFreqLimit3High", "grid_freq_limit3_high"),
        ("GridFreqLimit3LowTime", "grid_freq_limit3_low_time"),
        ("GridFreqLimit3HighTime", "grid_freq_limit3_high_time"),
    ];

    for (entity_name, expected_topic) in &entity_topic_pairs {
        let entity = messages
            .iter()
            .find(|m| m.topic.contains(entity_name))
            .expect(&format!("Interface protection entity {} should be present", entity_name));

        let expected_command_topic = format!("lxp/cmd/2222222222/set/{}", expected_topic);
        assert!(entity.payload.contains(&expected_command_topic), 
                "Entity {} should have correct command topic: {}", entity_name, expected_command_topic);
    }
}
