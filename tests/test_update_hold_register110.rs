use lxp_bridge::coordinator::commands::update_hold_register110::UpdateHoldRegister110;
use lxp_bridge::config::Inverter;
use lxp_bridge::prelude::*;
use lxp_bridge::lxp::packet::Register110Bit;

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
fn test_update_hold_register110_creation() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 110;
    let bit = Register110Bit::PvOffGridEnable;
    let enable = true;
    
    let command = UpdateHoldRegister110::new(channels, inverter, register, bit, enable);
    
    assert_eq!(command.bit(), Register110Bit::PvOffGridEnable);
    assert_eq!(command.enable(), true);
}

#[test]
fn test_update_hold_register110_disable() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 110;
    let bit = Register110Bit::FastZeroExportEnable;
    let enable = false;
    
    let command = UpdateHoldRegister110::new(channels, inverter, register, bit, enable);
    
    assert_eq!(command.bit(), Register110Bit::FastZeroExportEnable);
    assert_eq!(command.enable(), false);
}

#[test]
fn test_update_hold_register110_different_bits() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 110;
    
    // Test all available Register110Bit variants
    let test_bits = vec![
        Register110Bit::PvOffGridEnable,
        Register110Bit::FastZeroExportEnable,
        Register110Bit::MicroGridEnable,
        Register110Bit::SharedBatteryEnable,
        Register110Bit::ChargeLastEnable,
    ];
    
    for bit in test_bits {
        let command = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), register, bit.clone(), true);
        assert_eq!(command.bit(), bit);
        assert_eq!(command.enable(), true);
    }
}

#[test]
fn test_update_hold_register110_register_conversion() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let bit = Register110Bit::MicroGridEnable;
    let enable = true;
    
                    // Test with different register types that implement Into<u16>
        let command1 = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, bit.clone(), enable);
        let command2 = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u8, bit.clone(), enable);
        let command3 = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, bit.clone(), enable);
        
        assert_eq!(command1.bit(), bit);
        assert_eq!(command1.enable(), enable);
        assert_eq!(command2.bit(), bit);
        assert_eq!(command2.enable(), enable);
        assert_eq!(command3.bit(), bit);
        assert_eq!(command3.enable(), enable);
}

#[test]
fn test_update_hold_register110_edge_cases() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    
    // Test edge cases
    let edge_cases = vec![
        (0u16, Register110Bit::PvOffGridEnable, true),           // Minimum register, first bit, enabled
        (0u16, Register110Bit::ChargeLastEnable, false),         // Minimum register, last bit, disabled
        (65535u16, Register110Bit::PvOffGridEnable, true),       // Maximum register, first bit, enabled
        (65535u16, Register110Bit::ChargeLastEnable, false),     // Maximum register, last bit, disabled
        (32767u16, Register110Bit::MicroGridEnable, true),       // Middle register, middle bit, enabled
    ];
    
    for (register, bit, enable) in edge_cases {
        let command = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), register, bit.clone(), enable);
        assert_eq!(command.bit(), bit);
        assert_eq!(command.enable(), enable);
    }
}

#[test]
fn test_update_hold_register110_getters() {
    let channels = Channels::new();
    let inverter = create_test_inverter();
    let register: u16 = 110;
    let bit = Register110Bit::SharedBatteryEnable;
    let enable = true;
    
    let command = UpdateHoldRegister110::new(channels, inverter, register, bit.clone(), enable);
    
    // Test that we can access command fields through getters
    assert_eq!(command.bit(), bit);
    assert_eq!(command.enable(), enable);
}
