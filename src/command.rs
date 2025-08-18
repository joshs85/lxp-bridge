use crate::prelude::*;

#[derive(Debug)]
pub enum Command {
    ReadInputs(config::Inverter, u16),
    ReadInput(config::Inverter, u16, u16),
    ReadHold(config::Inverter, u16, u16),
    ReadParam(config::Inverter, u16),
    ReadAcChargeTime(config::Inverter, u16),
    ReadAcFirstTime(config::Inverter, u16),
    ReadChargePriorityTime(config::Inverter, u16),
    ReadForcedDischargeTime(config::Inverter, u16),
    SetHold(config::Inverter, u16, u16),
    WriteParam(config::Inverter, u16, u16),
    SetAcChargeTime(config::Inverter, u16, [u8; 4]),
    SetAcFirstTime(config::Inverter, u16, [u8; 4]),
    SetChargePriorityTime(config::Inverter, u16, [u8; 4]),
    SetForcedDischargeTime(config::Inverter, u16, [u8; 4]),
    ChargeRate(config::Inverter, u16),
    DischargeRate(config::Inverter, u16),
    EPS(config::Inverter, bool),
    OVFLoadDerate(config::Inverter, bool),
    DelayTimeForOverFDerate(config::Inverter, u16),
    OVFDerateStart(config::Inverter, u16),
    OVFDerateEnd(config::Inverter, u16),
    OVFDeratePctPerHz(config::Inverter, u16),
    UnderFrIncreasePctPerHz(config::Inverter, u16),
    UnderFrDroopStart(config::Inverter, u16),
    UnderFrDroopEnd(config::Inverter, u16),
    DRMS(config::Inverter, bool),
    LVRT(config::Inverter, bool),
    AntiIslanding(config::Inverter, bool),
    NeutralDetect(config::Inverter, bool),
    GridOnPowerSS(config::Inverter, bool),
    AcCharge(config::Inverter, bool),
    SwSeamless(config::Inverter, bool),
    SetToStandby(config::Inverter, bool),
    ChargePriority(config::Inverter, bool),
    ForcedDischarge(config::Inverter, bool),
    ISO(config::Inverter, bool),
    GFCI(config::Inverter, bool),
    DCI(config::Inverter, bool),
    FeedInGrid(config::Inverter, bool),
    AcChargeRate(config::Inverter, u16),
    AcChargeSocLimit(config::Inverter, u16),
    DischargeCutoffSocLimit(config::Inverter, u16),
    // Connection and reconnection configuration
    SetGridConnectTime(config::Inverter, u16),
    SetGridReconnectTime(config::Inverter, u16),
    SetGridVoltageLow(config::Inverter, u16),
    SetGridVoltageHigh(config::Inverter, u16),
    SetGridFrequencyLow(config::Inverter, u16),
    SetGridFrequencyHigh(config::Inverter, u16),
    // Interface Protection - Grid Voltage Limits
    SetGridVoltLimit1Low(config::Inverter, u16),
    SetGridVoltLimit1High(config::Inverter, u16),
    SetGridVoltLimit1LowTime(config::Inverter, u16),
    SetGridVoltLimit1HighTime(config::Inverter, u16),
    SetGridVoltLimit2Low(config::Inverter, u16),
    SetGridVoltLimit2High(config::Inverter, u16),
    SetGridVoltLimit2LowTime(config::Inverter, u16),
    SetGridVoltLimit3Low(config::Inverter, u16),
    SetGridVoltLimit3High(config::Inverter, u16),
    SetGridVoltLimit3LowTime(config::Inverter, u16),
    SetGridVoltLimit3HighTime(config::Inverter, u16),
    // Interface Protection - Grid Frequency Limits
    SetGridFreqLimit1Low(config::Inverter, u16),
    SetGridFreqLimit1High(config::Inverter, u16),
    SetGridFreqLimit1LowTime(config::Inverter, u16),
    SetGridFreqLimit1HighTime(config::Inverter, u16),
    SetGridFreqLimit2Low(config::Inverter, u16),
    SetGridFreqLimit2High(config::Inverter, u16),
    SetGridFreqLimit2LowTime(config::Inverter, u16),
    SetGridFreqLimit2HighTime(config::Inverter, u16),
    SetGridFreqLimit3Low(config::Inverter, u16),
    SetGridFreqLimit3High(config::Inverter, u16),
    SetGridFreqLimit3LowTime(config::Inverter, u16),
    SetGridFreqLimit3HighTime(config::Inverter, u16),
    // Register 110 switches
    PvOffGrid(config::Inverter, bool),
    FastZeroExport(config::Inverter, bool),
    MicroGrid(config::Inverter, bool),
    SharedBattery(config::Inverter, bool),
    ChargeLast(config::Inverter, bool),
    // System control
    RestartInverter(config::Inverter),
}

impl Command {
    pub fn to_result_topic(&self) -> String {
        use Command::*;

        let rest = match self {
            ReadInputs(inverter, c) => format!("{}/read/inputs/{}", inverter.datalog(), c),
            ReadInput(inverter, register, _) => {
                format!("{}/read/input/{}", inverter.datalog(), register)
            }
            ReadHold(inverter, register, _) => {
                format!("{}/read/hold/{}", inverter.datalog(), register)
            }
            ReadParam(inverter, register) => {
                format!("{}/read/param/{}", inverter.datalog(), register)
            }
            ReadAcChargeTime(inverter, num) => {
                format!("{}/read/ac_charge/{}", inverter.datalog(), num)
            }
            ReadAcFirstTime(inverter, num) => {
                format!("{}/read/ac_first/{}", inverter.datalog(), num)
            }
            ReadChargePriorityTime(inverter, num) => {
                format!("{}/read/charge_priority/{}", inverter.datalog(), num)
            }
            ReadForcedDischargeTime(inverter, num) => {
                format!("{}/read/forced_discharge/{}", inverter.datalog(), num)
            }
            SetHold(inverter, register, _) => {
                format!("{}/set/hold/{}", inverter.datalog(), register)
            }
            WriteParam(inverter, register, _) => {
                format!("{}/set/param/{}", inverter.datalog(), register)
            }
            SetAcChargeTime(inverter, num, _) => {
                format!("{}/set/ac_charge/{}", inverter.datalog(), num)
            }
            SetAcFirstTime(inverter, num, _) => {
                format!("{}/set/ac_first/{}", inverter.datalog(), num)
            }
            SetChargePriorityTime(inverter, num, _) => {
                format!("{}/set/charge_priority/{}", inverter.datalog(), num)
            }
            SetForcedDischargeTime(inverter, num, _) => {
                format!("{}/set/forced_discharge/{}", inverter.datalog(), num)
            }
            EPS(inverter, _) => format!("{}/set/eps", inverter.datalog()),
            OVFLoadDerate(inverter, _) => format!("{}/set/ovf_load_derate", inverter.datalog()),
            DelayTimeForOverFDerate(inverter, _) => format!("{}/set/frequency_active_open_loop_response_time", inverter.datalog()),
            OVFDerateStart(inverter, _) => format!("{}/set/ovf_derate_start_hz", inverter.datalog()),
            OVFDerateEnd(inverter, _) => format!("{}/set/ovf_derate_end_hz", inverter.datalog()),
            OVFDeratePctPerHz(inverter, _) => format!("{}/set/ovf_derate_pct_per_hz", inverter.datalog()),
            UnderFrDroopStart(inverter, _) => format!("{}/set/under_fr_droop_start_hz", inverter.datalog()),
            UnderFrDroopEnd(inverter, _) => format!("{}/set/under_fr_droop_end_hz", inverter.datalog()),
            UnderFrIncreasePctPerHz(inverter, _) => format!("{}/set/under_fr_increase_pct_per_hz", inverter.datalog()),
            DRMS(inverter, _) => format!("{}/set/drms", inverter.datalog()),
            DCI(inverter, _) => format!("{}/set/dci", inverter.datalog()),
            LVRT(inverter, _) => format!("{}/set/lvrt", inverter.datalog()),
            AntiIslanding(inverter, _) => format!("{}/set/anti_island", inverter.datalog()),
            NeutralDetect(inverter, _) => format!("{}/set/neutral_detect", inverter.datalog()),
            GridOnPowerSS(inverter, _) => format!("{}/set/grid_on_power_ss", inverter.datalog()),
            AcCharge(inverter, _) => format!("{}/set/ac_charge", inverter.datalog()),
            SwSeamless(inverter, _) => format!("{}/set/sw_seamless", inverter.datalog()),
            SetToStandby(inverter, _) => format!("{}/set/set_to_standby", inverter.datalog()),
            ChargePriority(inverter, _) => format!("{}/set/charge_priority", inverter.datalog()),
            ForcedDischarge(inverter, _) => format!("{}/set/forced_discharge", inverter.datalog()),
            ISO(inverter, _) => format!("{}/set/iso", inverter.datalog()),
            GFCI(inverter, _) => format!("{}/set/gfci", inverter.datalog()),
            FeedInGrid(inverter, _) => format!("{}/set/feed_in_grid", inverter.datalog()),
            ChargeRate(inverter, _) => format!("{}/set/charge_rate_pct", inverter.datalog()),
            DischargeRate(inverter, _) => format!("{}/set/discharge_rate_pct", inverter.datalog()),
            AcChargeRate(inverter, _) => format!("{}/set/ac_charge_rate_pct", inverter.datalog()),
            AcChargeSocLimit(inverter, _) => {
                format!("{}/set/ac_charge_soc_limit_pct", inverter.datalog())
            }
            DischargeCutoffSocLimit(inverter, _) => {
                format!("{}/set/discharge_cutoff_soc_limit_pct", inverter.datalog())
            }
            // Connection and reconnection configuration
            SetGridConnectTime(inverter, _) => format!("{}/set/grid_connect_time", inverter.datalog()),
            SetGridReconnectTime(inverter, _) => format!("{}/set/grid_reconnect_time", inverter.datalog()),
            SetGridVoltageLow(inverter, _) => format!("{}/set/grid_voltage_low", inverter.datalog()),
            SetGridVoltageHigh(inverter, _) => format!("{}/set/grid_voltage_high", inverter.datalog()),
            SetGridFrequencyLow(inverter, _) => format!("{}/set/grid_frequency_low", inverter.datalog()),
            SetGridFrequencyHigh(inverter, _) => format!("{}/set/grid_frequency_high", inverter.datalog()),
            // Interface Protection - Grid Voltage Limits
            SetGridVoltLimit1Low(inverter, _) => format!("{}/set/grid_volt_limit1_low", inverter.datalog()),
            SetGridVoltLimit1High(inverter, _) => format!("{}/set/grid_volt_limit1_high", inverter.datalog()),
            SetGridVoltLimit1LowTime(inverter, _) => format!("{}/set/grid_volt_limit1_low_time", inverter.datalog()),
            SetGridVoltLimit1HighTime(inverter, _) => format!("{}/set/grid_volt_limit1_high_time", inverter.datalog()),
            SetGridVoltLimit2Low(inverter, _) => format!("{}/set/grid_volt_limit2_low", inverter.datalog()),
            SetGridVoltLimit2High(inverter, _) => format!("{}/set/grid_volt_limit2_high", inverter.datalog()),
            SetGridVoltLimit2LowTime(inverter, _) => format!("{}/set/grid_volt_limit2_low_time", inverter.datalog()),
            SetGridVoltLimit3Low(inverter, _) => format!("{}/set/grid_volt_limit3_low", inverter.datalog()),
            SetGridVoltLimit3High(inverter, _) => format!("{}/set/grid_volt_limit3_high", inverter.datalog()),
            SetGridVoltLimit3LowTime(inverter, _) => format!("{}/set/grid_volt_limit3_low_time", inverter.datalog()),
            SetGridVoltLimit3HighTime(inverter, _) => format!("{}/set/grid_volt_limit3_high_time", inverter.datalog()),
            // Interface Protection - Grid Frequency Limits
            SetGridFreqLimit1Low(inverter, _) => format!("{}/set/grid_freq_limit1_low", inverter.datalog()),
            SetGridFreqLimit1High(inverter, _) => format!("{}/set/grid_freq_limit1_high", inverter.datalog()),
            SetGridFreqLimit1LowTime(inverter, _) => format!("{}/set/grid_freq_limit1_low_time", inverter.datalog()),
            SetGridFreqLimit1HighTime(inverter, _) => format!("{}/set/grid_freq_limit1_high_time", inverter.datalog()),
            SetGridFreqLimit2Low(inverter, _) => format!("{}/set/grid_freq_limit2_low", inverter.datalog()),
            SetGridFreqLimit2High(inverter, _) => format!("{}/set/grid_freq_limit2_high", inverter.datalog()),
            SetGridFreqLimit2LowTime(inverter, _) => format!("{}/set/grid_freq_limit2_low_time", inverter.datalog()),
            SetGridFreqLimit2HighTime(inverter, _) => format!("{}/set/grid_freq_limit2_high_time", inverter.datalog()),
            SetGridFreqLimit3Low(inverter, _) => format!("{}/set/grid_freq_limit3_low", inverter.datalog()),
            SetGridFreqLimit3High(inverter, _) => format!("{}/set/grid_freq_limit3_high", inverter.datalog()),
            SetGridFreqLimit3LowTime(inverter, _) => format!("{}/set/grid_freq_limit3_low_time", inverter.datalog()),
            SetGridFreqLimit3HighTime(inverter, _) => format!("{}/set/grid_freq_limit3_high_time", inverter.datalog()),
            // Register 110 switches
            PvOffGrid(inverter, _) => format!("{}/set/pv_off_grid", inverter.datalog()),
            FastZeroExport(inverter, _) => format!("{}/set/fast_zero_export", inverter.datalog()),
            MicroGrid(inverter, _) => format!("{}/set/micro_grid", inverter.datalog()),
            SharedBattery(inverter, _) => format!("{}/set/shared_battery", inverter.datalog()),
            ChargeLast(inverter, _) => format!("{}/set/charge_last", inverter.datalog()),
            // System control
            RestartInverter(inverter) => format!("{}/restart", inverter.datalog()),
        };

        format!("result/{rest}")
    }
}
