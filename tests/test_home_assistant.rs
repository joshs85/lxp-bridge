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
    assert!(payload.contains("config"), "Missing entity_category");
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
    assert!(payload.contains("config"), "Missing entity_category");
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
    assert!(payload.contains("config"), "Missing entity_category");
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
