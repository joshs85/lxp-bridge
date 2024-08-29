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
        };

        format!("result/{}", rest)
    }
}
