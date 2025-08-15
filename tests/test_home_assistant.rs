mod common;
use common::*;

#[tokio::test]
async fn all_has_soc() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/sensor/lxp_2222222222/soc/config".to_string(),
        retain: true,
        payload: r#"{"unique_id":"lxp_2222222222_soc","name":"State of Charge","state_topic":"lxp/2222222222/inputs/all","state_class":"measurement","device_class":"battery","value_template":"{{ value_json.soc }}","unit_of_measurement":"%","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"}}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_v_pv_1() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/sensor/lxp_2222222222/v_pv_1/config".to_string(),
        retain: true,
        payload: r#"{"unique_id":"lxp_2222222222_v_pv_1","name":"PV Voltage (String 1)","state_topic":"lxp/2222222222/inputs/all","state_class":"measurement","device_class":"voltage","value_template":"{{ value_json.v_pv_1 }}","unit_of_measurement":"V","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"}}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_p_pv() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/sensor/lxp_2222222222/p_pv/config".to_string(),
        retain: true,
        payload: r#"{"unique_id":"lxp_2222222222_p_pv","name":"PV Power (Array)","state_topic":"lxp/2222222222/inputs/all","state_class":"measurement","device_class":"power","value_template":"{{ value_json.p_pv }}","unit_of_measurement":"W","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"}}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_e_pv_all() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/sensor/lxp_2222222222/e_pv_all/config".to_string(),
        retain: true,
        payload: r#"{"unique_id":"lxp_2222222222_e_pv_all","name":"PV Generation (All time)","state_topic":"lxp/2222222222/inputs/all","state_class":"total_increasing","device_class":"energy","value_template":"{{ value_json.e_pv_all }}","unit_of_measurement":"kWh","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"}}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_fault_code() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/sensor/lxp_2222222222/fault_code/config".to_string(),
        retain: true,
        payload: r#"{"unique_id":"lxp_2222222222_fault_code","name":"Fault Code","state_topic":"lxp/2222222222/input/fault_code/parsed","entity_category":"diagnostic","icon":"mdi:alert","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"}}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_switch_ac_charge() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/switch/lxp_2222222222/ac_charge/config".to_string(),
        retain: true,
        payload: r#"{"name":"AC Charge","state_topic":"lxp/2222222222/hold/21/bits","command_topic":"lxp/cmd/2222222222/set/ac_charge","value_template":"{{ value_json.ac_charge_en }}","unique_id":"lxp_2222222222_ac_charge","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"}}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_number_ac_charge_soc_limit_pct() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/number/lxp_2222222222/AcChargeSocLimit/config".to_string(),
        retain: true,
        payload: r#"{"name":"AC Charge Limit %","state_topic":"lxp/2222222222/hold/67","command_topic":"lxp/cmd/2222222222/set/hold/67","value_template":"{{ float(value) }}","unique_id":"lxp_2222222222_number_AcChargeSocLimit","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"},"min":0.0,"max":100.0,"step":1.0,"unit_of_measurement":"%"}"#.to_string()
    }));
}

#[tokio::test]
async fn all_has_time_range_ac_charge_1() {
    common_setup();

    let config = Factory::example_config();
    let r = home_assistant::Config::new(&config.inverters[0], &config.mqtt).all();

    assert!(r.is_ok());
    assert!(r.unwrap().contains(&mqtt::Message {
        topic: "homeassistant/text/lxp_2222222222/ac_charge_1/config".to_string(),
        retain: true,
        payload: r#"{"name":"AC Charge Timeslot 1","state_topic":"lxp/2222222222/ac_charge/1","command_topic":"lxp/cmd/2222222222/set/ac_charge/1","command_template":"{% set parts = value.split(\"-\") %}{\"start\":\"{{ parts[0] }}\", \"end\":\"{{ parts[1] }}\"}","value_template":"{{ value_json[\"start\"] }}-{{ value_json[\"end\"] }}","unique_id":"lxp_2222222222_text_ac_charge/1","device":{"manufacturer":"LuxPower","name":"lxp_2222222222","identifiers":["lxp_2222222222"]},"availability":{"topic":"lxp/LWT"},"pattern":"([01]?[0-9]|2[0-3]):[0-5][0-9]-([01]?[0-9]|2[0-3]):[0-5][0-9]"}"#.to_string()
    }));
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
    
    // Command topic should send to set/hold register
    assert!(payload["command_topic"].as_str().unwrap().contains("cmd"));
    assert!(payload["command_topic"].as_str().unwrap().contains("set/hold/134"));

    // Check under frequency droop end topics
    let under_fr_end = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopEnd"))
        .unwrap();

    let payload: serde_json::Value = serde_json::from_str(&under_fr_end.payload).unwrap();
    
    // State topic should read from hold register
    assert!(payload["state_topic"].as_str().unwrap().contains("hold/135"));
    
    // Command topic should send to set/hold register
    assert!(payload["command_topic"].as_str().unwrap().contains("cmd"));
    assert!(payload["command_topic"].as_str().unwrap().contains("set/hold/135"));
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
