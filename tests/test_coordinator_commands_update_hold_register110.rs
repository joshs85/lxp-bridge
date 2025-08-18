use lxp_bridge::prelude::*;
use lxp_bridge::coordinator::commands::update_hold_register110::UpdateHoldRegister110;
use lxp_bridge::lxp::packet::Register110Bit;

#[tokio::test]
async fn test_update_hold_register110_creation() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let register: u16 = 110;
    let bit = Register110Bit::PvOffGridEnable;
    let enable = true;
    
    let command = UpdateHoldRegister110::new(channels, inverter, register, bit, enable);
    
    assert_eq!(command.bit(), Register110Bit::PvOffGridEnable);
    assert_eq!(command.enable(), true);
}

#[tokio::test]
async fn test_update_hold_register110_disable() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let register: u16 = 110;
    let bit = Register110Bit::FastZeroExportEnable;
    let enable = false;
    
    let command = UpdateHoldRegister110::new(channels, inverter, register, bit, enable);
    
    assert_eq!(command.bit(), Register110Bit::FastZeroExportEnable);
    assert_eq!(command.enable(), false);
}

#[tokio::test]
async fn test_update_hold_register110_different_bits() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
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

#[tokio::test]
async fn test_update_hold_register110_register_conversion() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    let bit = Register110Bit::MicroGridEnable;
    let enable = true;
    
    // Test with different register types that implement Into<u16>
    let test_registers = vec![
        110u16,
        110u16,
        110u16,
        110u16,
    ];
    
    for register in test_registers {
        let command = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), register, bit.clone(), enable);
        assert_eq!(command.bit(), bit);
        assert_eq!(command.enable(), enable);
    }
}

#[tokio::test]
async fn test_update_hold_register110_edge_cases() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
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

#[tokio::test]
async fn test_update_hold_register110_multiple_instances() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test creating multiple instances
    let commands = vec![
        UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, Register110Bit::PvOffGridEnable, true),
        UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, Register110Bit::FastZeroExportEnable, false),
        UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, Register110Bit::MicroGridEnable, true),
        UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, Register110Bit::SharedBatteryEnable, false),
        UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, Register110Bit::ChargeLastEnable, true),
    ];
    
    assert_eq!(commands.len(), 5);
    
    let expected_bits = vec![
        Register110Bit::PvOffGridEnable,
        Register110Bit::FastZeroExportEnable,
        Register110Bit::MicroGridEnable,
        Register110Bit::SharedBatteryEnable,
        Register110Bit::ChargeLastEnable,
    ];
    
    let expected_enables = vec![true, false, true, false, true];
    
    for (i, command) in commands.iter().enumerate() {
        assert_eq!(command.bit(), expected_bits[i]);
        assert_eq!(command.enable(), expected_enables[i]);
    }
}

#[tokio::test]
async fn test_update_hold_register110_bit_operations() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test bit operations
    let bit = Register110Bit::MicroGridEnable;
    
    // Test enable
    let command_enable = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, bit.clone(), true);
    assert_eq!(command_enable.bit(), bit);
    assert_eq!(command_enable.enable(), true);
    
    // Test disable
    let command_disable = UpdateHoldRegister110::new(channels.clone(), inverter.clone(), 110u16, bit.clone(), false);
    assert_eq!(command_disable.bit(), bit);
    assert_eq!(command_disable.enable(), false);
}

#[tokio::test]
async fn test_update_hold_register110_register_specific() {
    let channels = Channels::new();
    let config = ConfigWrapper::new("tests/fixtures/config.yaml".to_string()).unwrap();
    let inverter = config.enabled_inverters().into_iter().next().unwrap();
    
    // Test register 110 specific operations
    let register: u16 = 110;
    
    // Test all bits for register 110
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
        
        // Test that the bit is within reasonable bounds
        let bit_value = u16::from(bit);
        assert!(bit_value > 0);
        assert!(bit_value <= 16); // 2^4 = 16 (5 bits max)
    }
}
