use lxp_bridge::lxp::packet::StatusString;

#[test]
fn test_status_decoding() {
    // Test status 96 (0x60) - Battery Off-grid + AC Charge
    let status = 96u16;
    let status_text = StatusString::from_value_detailed(status);
    assert_eq!(status_text, "Battery Off-grid + AC Charge");
    
    // Test other common status values
    assert_eq!(StatusString::from_value_detailed(0x00), "Standby");
    assert_eq!(StatusString::from_value_detailed(0x01), "Fault");
    assert_eq!(StatusString::from_value_detailed(0x04), "PV On-grid");
    assert_eq!(StatusString::from_value_detailed(0x08), "PV Charge");
    assert_eq!(StatusString::from_value_detailed(0x10), "Battery On-grid");
    assert_eq!(StatusString::from_value_detailed(0x20), "AC Charge");
    assert_eq!(StatusString::from_value_detailed(0x40), "Battery Off-grid");
    assert_eq!(StatusString::from_value_detailed(0x80), "PV Off-grid");
    assert_eq!(StatusString::from_value_detailed(0xC0), "PV & Battery Off-grid");
}

#[test]
fn test_combined_status_decoding() {
    // Test combined status values
    let status_60 = 0x60u16; // Battery Off-grid + AC Charge
    let status_text = StatusString::from_value_detailed(status_60);
    assert_eq!(status_text, "Battery Off-grid + AC Charge");
    
    // Test a custom combined status
    let custom_status = 0x24u16; // PV On-grid + AC Charge
    let custom_text = StatusString::from_value_detailed(custom_status);
    assert_eq!(custom_text, "PV On-grid + AC Charge");
}
