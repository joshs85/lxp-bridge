use lxp_bridge::command::Command;
use lxp_bridge::config::Inverter;
use lxp_bridge::lxp::inverter::Serial;
use std::str::FromStr;

// Helper function to create a test inverter
fn create_test_inverter() -> Inverter {
    Inverter {
        enabled: true,
        host: "test.example.com".to_string(),
        port: 12345,
        serial: Serial::from_str("1111111111").unwrap(),
        datalog: Serial::from_str("2222222222").unwrap(),
        heartbeats: Some(false),
        publish_holdings_on_connect: Some(false),
        read_timeout: Some(900),
    }
}

#[test]
fn test_read_inputs_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadInputs(inverter.clone(), 1);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/inputs/1");
}

#[test]
fn test_read_input_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadInput(inverter.clone(), 10, 1);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/input/10");
}

#[test]
fn test_read_hold_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadHold(inverter.clone(), 20, 1);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/hold/20");
}

#[test]
fn test_read_param_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadParam(inverter.clone(), 30);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/param/30");
}

#[test]
fn test_read_ac_charge_time_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadAcChargeTime(inverter.clone(), 1);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/ac_charge/1");
}

#[test]
fn test_read_ac_first_time_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadAcFirstTime(inverter.clone(), 2);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/ac_first/2");
}

#[test]
fn test_read_charge_priority_time_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadChargePriorityTime(inverter.clone(), 3);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/charge_priority/3");
}

#[test]
fn test_read_forced_discharge_time_command() {
    let inverter = create_test_inverter();
    let command = Command::ReadForcedDischargeTime(inverter.clone(), 4);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/read/forced_discharge/4");
}

#[test]
fn test_set_hold_command() {
    let inverter = create_test_inverter();
    let command = Command::SetHold(inverter.clone(), 40, 100);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/hold/40");
}

#[test]
fn test_write_param_command() {
    let inverter = create_test_inverter();
    let command = Command::WriteParam(inverter.clone(), 50, 200);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/param/50");
}

#[test]
fn test_set_ac_charge_time_command() {
    let inverter = create_test_inverter();
    let time_data = [8, 0, 20, 0]; // 8:00 to 20:00
    let command = Command::SetAcChargeTime(inverter.clone(), 1, time_data);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ac_charge/1");
}

#[test]
fn test_set_ac_first_time_command() {
    let inverter = create_test_inverter();
    let time_data = [9, 0, 21, 0]; // 9:00 to 21:00
    let command = Command::SetAcFirstTime(inverter.clone(), 2, time_data);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ac_first/2");
}

#[test]
fn test_set_charge_priority_time_command() {
    let inverter = create_test_inverter();
    let time_data = [10, 0, 22, 0]; // 10:00 to 22:00
    let command = Command::SetChargePriorityTime(inverter.clone(), 3, time_data);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/charge_priority/3");
}

#[test]
fn test_set_forced_discharge_time_command() {
    let inverter = create_test_inverter();
    let time_data = [11, 0, 23, 0]; // 11:00 to 23:00
    let command = Command::SetForcedDischargeTime(inverter.clone(), 4, time_data);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/forced_discharge/4");
}

#[test]
fn test_eps_command() {
    let inverter = create_test_inverter();
    let command = Command::EPS(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/eps");
}

#[test]
fn test_ovf_load_derate_command() {
    let inverter = create_test_inverter();
    let command = Command::OVFLoadDerate(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ovf_load_derate");
}

#[test]
fn test_delay_time_for_over_f_derate_command() {
    let inverter = create_test_inverter();
    let command = Command::DelayTimeForOverFDerate(inverter.clone(), 100);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/frequency_active_open_loop_response_time");
}

#[test]
fn test_ovf_derate_start_command() {
    let inverter = create_test_inverter();
    let command = Command::OVFDerateStart(inverter.clone(), 5000);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ovf_derate_start_hz");
}

#[test]
fn test_ovf_derate_end_command() {
    let inverter = create_test_inverter();
    let command = Command::OVFDerateEnd(inverter.clone(), 6000);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ovf_derate_end_hz");
}

#[test]
fn test_ovf_derate_pct_per_hz_command() {
    let inverter = create_test_inverter();
    let command = Command::OVFDeratePctPerHz(inverter.clone(), 5);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ovf_derate_pct_per_hz");
}

#[test]
fn test_under_fr_droop_start_command() {
    let inverter = create_test_inverter();
    let command = Command::UnderFrDroopStart(inverter.clone(), 4500);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/under_fr_droop_start_hz");
}

#[test]
fn test_under_fr_droop_end_command() {
    let inverter = create_test_inverter();
    let command = Command::UnderFrDroopEnd(inverter.clone(), 5500);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/under_fr_droop_end_hz");
}

#[test]
fn test_under_fr_increase_pct_per_hz_command() {
    let inverter = create_test_inverter();
    let command = Command::UnderFrIncreasePctPerHz(inverter.clone(), 3);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/under_fr_increase_pct_per_hz");
}

#[test]
fn test_drms_command() {
    let inverter = create_test_inverter();
    let command = Command::DRMS(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/drms");
}

#[test]
fn test_dci_command() {
    let inverter = create_test_inverter();
    let command = Command::DCI(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/dci");
}

#[test]
fn test_lvrt_command() {
    let inverter = create_test_inverter();
    let command = Command::LVRT(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/lvrt");
}

#[test]
fn test_anti_islanding_command() {
    let inverter = create_test_inverter();
    let command = Command::AntiIslanding(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/anti_island");
}

#[test]
fn test_neutral_detect_command() {
    let inverter = create_test_inverter();
    let command = Command::NeutralDetect(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/neutral_detect");
}

#[test]
fn test_grid_on_power_ss_command() {
    let inverter = create_test_inverter();
    let command = Command::GridOnPowerSS(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/grid_on_power_ss");
}

#[test]
fn test_ac_charge_command() {
    let inverter = create_test_inverter();
    let command = Command::AcCharge(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ac_charge");
}

#[test]
fn test_sw_seamless_command() {
    let inverter = create_test_inverter();
    let command = Command::SwSeamless(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/sw_seamless");
}

#[test]
fn test_set_to_standby_command() {
    let inverter = create_test_inverter();
    let command = Command::SetToStandby(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/set_to_standby");
}

#[test]
fn test_charge_priority_command() {
    let inverter = create_test_inverter();
    let command = Command::ChargePriority(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/charge_priority");
}

#[test]
fn test_forced_discharge_command() {
    let inverter = create_test_inverter();
    let command = Command::ForcedDischarge(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/forced_discharge");
}

#[test]
fn test_iso_command() {
    let inverter = create_test_inverter();
    let command = Command::ISO(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/iso");
}

#[test]
fn test_gfci_command() {
    let inverter = create_test_inverter();
    let command = Command::GFCI(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/gfci");
}

#[test]
fn test_feed_in_grid_command() {
    let inverter = create_test_inverter();
    let command = Command::FeedInGrid(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/feed_in_grid");
}

#[test]
fn test_charge_rate_command() {
    let inverter = create_test_inverter();
    let command = Command::ChargeRate(inverter.clone(), 80);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/charge_rate_pct");
}

#[test]
fn test_discharge_rate_command() {
    let inverter = create_test_inverter();
    let command = Command::DischargeRate(inverter.clone(), 70);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/discharge_rate_pct");
}

#[test]
fn test_ac_charge_rate_command() {
    let inverter = create_test_inverter();
    let command = Command::AcChargeRate(inverter.clone(), 60);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ac_charge_rate_pct");
}

#[test]
fn test_ac_charge_soc_limit_command() {
    let inverter = create_test_inverter();
    let command = Command::AcChargeSocLimit(inverter.clone(), 90);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/ac_charge_soc_limit_pct");
}

#[test]
fn test_discharge_cutoff_soc_limit_command() {
    let inverter = create_test_inverter();
    let command = Command::DischargeCutoffSocLimit(inverter.clone(), 20);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/discharge_cutoff_soc_limit_pct");
}

#[test]
fn test_pv_off_grid_command() {
    let inverter = create_test_inverter();
    let command = Command::PvOffGrid(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/pv_off_grid");
}

#[test]
fn test_fast_zero_export_command() {
    let inverter = create_test_inverter();
    let command = Command::FastZeroExport(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/fast_zero_export");
}

#[test]
fn test_micro_grid_command() {
    let inverter = create_test_inverter();
    let command = Command::MicroGrid(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/micro_grid");
}

#[test]
fn test_shared_battery_command() {
    let inverter = create_test_inverter();
    let command = Command::SharedBattery(inverter.clone(), false);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/shared_battery");
}

#[test]
fn test_charge_last_command() {
    let inverter = create_test_inverter();
    let command = Command::ChargeLast(inverter.clone(), true);
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/set/charge_last");
}

#[test]
fn test_restart_inverter_command() {
    let inverter = create_test_inverter();
    let command = Command::RestartInverter(inverter.clone());
    
    let topic = command.to_result_topic();
    assert_eq!(topic, "result/2222222222/restart");
}

#[test]
fn test_command_debug() {
    let inverter = create_test_inverter();
    let command = Command::ReadInputs(inverter, 1);
    
    let debug_str = format!("{:?}", command);
    assert!(debug_str.contains("ReadInputs"));
}

#[test]
fn test_multiple_inverters() {
    let inverter1 = Inverter {
        enabled: true,
        host: "inverter1.example.com".to_string(),
        port: 12345,
        serial: Serial::from_str("1111111111").unwrap(),
        datalog: Serial::from_str("INV1000000").unwrap(),
        heartbeats: Some(false),
        publish_holdings_on_connect: Some(false),
        read_timeout: Some(900),
    };
    
    let inverter2 = Inverter {
        enabled: true,
        host: "inverter2.example.com".to_string(),
        port: 12346,
        serial: Serial::from_str("2222222222").unwrap(),
        datalog: Serial::from_str("INV2000000").unwrap(),
        heartbeats: Some(false),
        publish_holdings_on_connect: Some(false),
        read_timeout: Some(900),
    };
    
    let command1 = Command::ReadInputs(inverter1, 1);
    let command2 = Command::ReadInputs(inverter2, 1);
    
    let topic1 = command1.to_result_topic();
    let topic2 = command2.to_result_topic();
    
    assert_eq!(topic1, "result/INV1000000/read/inputs/1");
    assert_eq!(topic2, "result/INV2000000/read/inputs/1");
    assert_ne!(topic1, topic2);
}
