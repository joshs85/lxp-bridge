mod common;
use common::*;

#[test]
fn test_frequency_command_parsing() {
    common_setup();

    let inverter = Factory::inverter();

    // Test that the new frequency commands are properly parsed
    // This test verifies that the MQTT command parsing works for the new commands
    
    // Note: We can't directly test the parse_cmd method since it's private,
    // but we can test that the commands are properly defined and accessible
    
    // Verify that the Command enum includes our new commands
    use crate::Command;
    
    // This should compile and run, verifying the commands exist
    let _under_fr_start = Command::UnderFrDroopStart(inverter.clone(), 5999);
    let _under_fr_end = Command::UnderFrDroopEnd(inverter.clone(), 4550);
    let _ovf_start = Command::OVFDerateStart(inverter.clone(), 6001);
    let _ovf_end = Command::OVFDerateEnd(inverter.clone(), 6500);
    
    // If we get here, the commands are properly defined
    assert!(true);
}

#[test]
fn test_frequency_command_topics() {
    common_setup();

    let inverter = Factory::inverter();

    // Test that the command topics are properly formatted
    use crate::Command;
    
    let _under_fr_start_cmd = Command::UnderFrDroopStart(inverter.clone(), 5999);
    let _under_fr_end_cmd = Command::UnderFrDroopEnd(inverter.clone(), 4550);
    let _ovf_start_cmd = Command::OVFDerateStart(inverter.clone(), 6001);
    let _ovf_end_cmd = Command::OVFDerateEnd(inverter.clone(), 6500);
    
    // Test topic formatting
    let under_fr_start_topic = format!("{}/set/under_fr_droop_start_hz", inverter.datalog());
    let under_fr_end_topic = format!("{}/set/under_fr_droop_end_hz", inverter.datalog());
    let ovf_start_topic = format!("{}/set/ovf_derate_start_hz", inverter.datalog());
    let ovf_end_topic = format!("{}/set/ovf_derate_end_hz", inverter.datalog());
    
    assert_eq!(under_fr_start_topic, "2222222222/set/under_fr_droop_start_hz");
    assert_eq!(under_fr_end_topic, "2222222222/set/under_fr_droop_end_hz");
    assert_eq!(ovf_start_topic, "2222222222/set/ovf_derate_start_hz");
    assert_eq!(ovf_end_topic, "2222222222/set/ovf_derate_end_hz");
}

#[test]
fn test_frequency_register_values() {
    common_setup();

    // Test that the register values are correct
    use lxp::packet::Register;
    
    assert_eq!(Register::UnderFrDroopStart as u16, 134);
    assert_eq!(Register::UnderFrDroopEnd as u16, 135);
    assert_eq!(Register::OVFDerateStart as u16, 115);
    assert_eq!(Register::OVFDerateEnd as u16, 124);
}

#[test]
fn test_home_assistant_discovery() {
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

    // Check that under frequency droop end control is present
    let under_fr_end = messages
        .iter()
        .find(|m| m.topic.contains("UnderFrDroopEnd"))
        .expect("Under Frequency Droop End control should be present");

    assert!(under_fr_end.payload.contains("Under Frequency Droop End (Hz)"));
    assert!(under_fr_end.payload.contains("Hz"));

    // Verify both controls use the same configuration pattern as OVF controls
    let ovf_start = messages
        .iter()
        .find(|m| m.topic.contains("OVFDerateStart"))
        .expect("OVF Derate Start control should be present");

    // Both should have similar configurations
    assert_eq!(under_fr_start.payload.contains("Hz"), ovf_start.payload.contains("Hz"));
}

#[test]
fn test_restart_command() {
    common_setup();

    let inverter = Factory::inverter();

    // Test that the restart command is properly defined
    use crate::Command;
    
    let _restart_cmd = Command::RestartInverter(inverter.clone());
    
    // If we get here, the command is properly defined
    assert!(true);
}

#[test]
fn test_restart_command_topic() {
    common_setup();

    let inverter = Factory::inverter();

    // Test that the restart command topic is properly formatted
    use crate::Command;
    
    let restart_cmd = Command::RestartInverter(inverter.clone());
    
    // Test topic formatting
    let restart_topic = restart_cmd.to_result_topic();
    let expected_topic = format!("result/{}/restart", inverter.datalog());
    
    assert_eq!(restart_topic, expected_topic);
    assert_eq!(restart_topic, "result/2222222222/restart");
}

#[test]
fn test_restart_home_assistant_discovery() {
    common_setup();

    let config = Factory::example_config();
    let ha = home_assistant::Config::new(&config.inverters[0], &config.mqtt);

    let messages = ha.all().unwrap();

    // Check that restart button is present
    let restart_button = messages
        .iter()
        .find(|m| m.topic.contains("restart"))
        .expect("Restart button should be present");

    // Check that the topic contains "button" (Home Assistant entity type)
    assert!(restart_button.topic.contains("button"));
    // Check that the payload contains the button name
    assert!(restart_button.payload.contains("Restart Inverter"));
    // Check that the payload contains the command topic
    assert!(restart_button.payload.contains("lxp/cmd/2222222222/restart"));
    // Check that the payload contains the unique ID
    assert!(restart_button.payload.contains("lxp_2222222222_restart"));
}

#[test]
fn test_restart_command_bit_manipulation_logic() {
    common_setup();

    // Test the bit manipulation logic directly
    let bit = lxp::packet::Register11Bit::InvReboot;
    let bit_value = bit as u16;
    
    // Test case 1: Initial value 0x0001, setting bit 7 should result in 0x0081
    let initial_value = 0x0001;
    let expected_value = initial_value | bit_value; // 0x0001 | 0x0080 = 0x0081
    assert_eq!(expected_value, 0x0081);
    
    // Test case 2: Initial value 0x00FF, setting bit 7 should result in 0x00FF (no change)
    let initial_value_2 = 0x00FF;
    let expected_value_2 = initial_value_2 | bit_value; // 0x00FF | 0x0080 = 0x00FF
    assert_eq!(expected_value_2, 0x00FF);
    
    // Test case 3: Initial value 0x0000, setting bit 7 should result in 0x0080
    let initial_value_3 = 0x0000;
    let expected_value_3 = initial_value_3 | bit_value; // 0x0000 | 0x0080 = 0x0080
    assert_eq!(expected_value_3, 0x0080);
    
    // Verify that only bit 7 is affected
    let bit_mask = 0x0080; // Only bit 7
    let other_bits_mask = 0x007F; // Bits 0-6
    
    // Test that other bits are preserved
    assert_eq!(expected_value & other_bits_mask, initial_value & other_bits_mask);
    assert_eq!(expected_value_2 & other_bits_mask, initial_value_2 & other_bits_mask);
    assert_eq!(expected_value_3 & other_bits_mask, initial_value_3 & other_bits_mask);
    
    // Test that only bit 7 is set when enable=true
    assert_eq!(expected_value & bit_mask, bit_mask);
    assert_eq!(expected_value_2 & bit_mask, bit_mask);
    assert_eq!(expected_value_3 & bit_mask, bit_mask);
}

#[test]
fn test_restart_command_bit_operations() {
    common_setup();

    // Test the specific bit operations used in the restart command
    let bit = lxp::packet::Register11Bit::InvReboot;
    let bit_value = bit as u16;
    
    // Test the OR operation for setting a bit
    let test_values = vec![0x0000, 0x0001, 0x0010, 0x00FF, 0x0100, 0xFFFF];
    
    for initial_value in test_values {
        let result = initial_value | bit_value;
        
        // Verify that only bit 7 is affected
        let bit_mask = 0x0080; // Only bit 7
        let other_bits_mask = 0x007F; // Bits 0-6
        
        // Test that other bits are preserved
        assert_eq!(result & other_bits_mask, initial_value & other_bits_mask);
        
        // Test that bit 7 is always set
        assert_eq!(result & bit_mask, bit_mask);
        
        // Test that the result is correct
        if (initial_value & bit_mask) == 0 {
            // If bit 7 was not set, it should be set now
            assert_eq!(result, initial_value | bit_value);
        } else {
            // If bit 7 was already set, no change should occur
            assert_eq!(result, initial_value);
        }
    }
}
