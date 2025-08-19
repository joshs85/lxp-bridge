use crate::prelude::*;

use enum_dispatch::*;
use nom_derive::{Nom, Parse};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;

pub enum ReadInput {
    ReadInputAll(Box<ReadInputAll>),
    ReadInput1(ReadInput1),
    ReadInput2(ReadInput2),
    ReadInput3(ReadInput3),
}

// {{{ ReadInputAll
#[derive(PartialEq, Clone, Debug, Serialize, Nom)]
#[nom(LittleEndian)]
pub struct ReadInputAll {
    pub status: u16,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_pv_1: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_pv_2: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_pv_3: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_bat: f64,

    pub soc: i8,
    pub soh: i8,

    pub internal_fault: u16,

    #[nom(Ignore)]
    pub p_pv: u16,
    pub p_pv_1: u16,
    pub p_pv_2: u16,
    pub p_pv_3: u16,
    #[nom(Ignore)]
    pub p_battery: i32,
    pub p_charge: u16,
    pub p_discharge: u16,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_ac_r: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_ac_s: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_ac_t: f64,
    #[nom(Parse = "Utils::le_u16_div100")]
    pub f_ac: f64,

    pub p_inv: u16,
    pub p_rec: u16,

    #[nom(SkipBefore(2))] // IinvRMS
    #[nom(Parse = "Utils::le_u16_div1000")]
    pub pf: f64,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_eps_r: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_eps_s: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_eps_t: f64,
    #[nom(Parse = "Utils::le_u16_div100")]
    pub f_eps: f64,
    pub p_eps: u16,
    pub s_eps: u16,
    #[nom(Ignore)]
    pub p_grid: i32,
    pub p_to_grid: u16,
    pub p_to_user: u16,

    #[nom(Ignore)]
    pub e_pv_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_pv_day_1: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_pv_day_2: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_pv_day_3: f64,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_inv_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_rec_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_chg_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_dischg_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_eps_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_to_grid_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_to_user_day: f64,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_bus_1: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_bus_2: f64,

    #[nom(Ignore)]
    pub e_pv_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_pv_all_1: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_pv_all_2: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_pv_all_3: f64,

    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_inv_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_rec_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_chg_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_dischg_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_eps_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_to_grid_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_to_user_all: f64,

    pub fault_code: u32,
    pub warning_code: u32,

    pub t_inner: u16,
    pub t_rad_1: u16,
    pub t_rad_2: u16,
    pub t_bat: u16,
    #[nom(SkipBefore(2))] // reserved - radiator 3?
    pub runtime: u32,
    // 18 bytes of auto_test stuff here I'm not doing yet
    #[nom(SkipBefore(18))] // auto_test stuff, TODO..
    #[nom(SkipBefore(2))] // bat_brand, bat_com_type
    #[nom(Parse = "Utils::le_u16_div10")]
    pub max_chg_curr: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub max_dischg_curr: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub charge_volt_ref: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub dischg_cut_volt: f64,

    pub bat_status_0: u16,
    pub bat_status_1: u16,
    pub bat_status_2: u16,
    pub bat_status_3: u16,
    pub bat_status_4: u16,
    pub bat_status_5: u16,
    pub bat_status_6: u16,
    pub bat_status_7: u16,
    pub bat_status_8: u16,
    pub bat_status_9: u16,
    pub bat_status_inv: u16,

    pub bat_count: u16,
    pub bat_capacity: u16,

    #[nom(Parse = "Utils::le_u16_div100")]
    pub bat_current: f64,

    pub bms_event_1: u16, // FaultCode_BMS
    pub bms_event_2: u16, // WarningCode_BMS

    // TODO: probably floats but need non-zero sample data to check. just guessing at the div100.
    #[nom(Parse = "Utils::le_u16_div1000")]
    pub max_cell_voltage: f64,
    #[nom(Parse = "Utils::le_u16_div1000")]
    pub min_cell_voltage: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub max_cell_temp: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub min_cell_temp: f64,

    pub bms_fw_update_state: u16,

    pub cycle_count: u16,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub vbat_inv: f64,

    // 14 bytes I'm not sure what they are; possibly generator stuff
    #[nom(SkipBefore(14))]
    #[nom(Parse = "Utils::current_time_for_nom")]
    pub time: UnixTime,
    #[nom(Ignore)]
    pub datalog: Serial,
} // }}}

// {{{ ReadInput1
#[derive(Clone, Debug, Serialize, Nom)]
#[nom(LittleEndian)]
pub struct ReadInput1 {
    pub status: u16,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_pv_1: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_pv_2: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_pv_3: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_bat: f64,

    pub soc: i8,
    pub soh: i8,

    pub internal_fault: u16,

    #[nom(Ignore)]
    pub p_pv: u16,
    pub p_pv_1: u16,
    pub p_pv_2: u16,
    pub p_pv_3: u16,
    #[nom(Ignore)]
    pub p_battery: i32,
    pub p_charge: u16,
    pub p_discharge: u16,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_ac_r: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_ac_s: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_ac_t: f64,
    #[nom(Parse = "Utils::le_u16_div100")]
    pub f_ac: f64,

    pub p_inv: u16,
    pub p_rec: u16,

    #[nom(SkipBefore(2))] // IinvRMS
    #[nom(Parse = "Utils::le_u16_div1000")]
    pub pf: f64,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_eps_r: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_eps_s: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_eps_t: f64,
    #[nom(Parse = "Utils::le_u16_div100")]
    pub f_eps: f64,
    pub p_eps: u16,
    pub s_eps: u16,
    #[nom(Ignore)]
    pub p_grid: i32,
    pub p_to_grid: u16,
    pub p_to_user: u16,

    #[nom(Ignore)]
    pub e_pv_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_pv_day_1: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_pv_day_2: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_pv_day_3: f64,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_inv_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_rec_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_chg_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_dischg_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_eps_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_to_grid_day: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub e_to_user_day: f64,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_bus_1: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub v_bus_2: f64,

    #[nom(Parse = "Utils::current_time_for_nom")]
    pub time: UnixTime,
    #[nom(Ignore)]
    pub datalog: Serial,
} // }}}

// {{{ ReadInput2
#[derive(Clone, Debug, Serialize, Nom)]
#[nom(Debug, LittleEndian)]
pub struct ReadInput2 {
    #[nom(Ignore)]
    pub e_pv_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_pv_all_1: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_pv_all_2: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_pv_all_3: f64,

    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_inv_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_rec_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_chg_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_dischg_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_eps_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_to_grid_all: f64,
    #[nom(Parse = "Utils::le_u32_div10")]
    pub e_to_user_all: f64,

    pub fault_code: u32,
    pub warning_code: u32,

    pub t_inner: u16,
    pub t_rad_1: u16,
    pub t_rad_2: u16,
    pub t_bat: u16,

    #[nom(SkipBefore(2))] // reserved
    pub runtime: u32,
    // 18 bytes of auto_test stuff here I'm not doing yet
    //
    #[nom(Parse = "Utils::current_time_for_nom")]
    pub time: UnixTime,
    #[nom(Ignore)]
    pub datalog: Serial,
} // }}}

// {{{ ReadInput3
#[derive(Clone, Debug, Serialize, Nom)]
#[nom(LittleEndian)]
pub struct ReadInput3 {
    #[nom(SkipBefore(2))] // bat_brand, bat_com_type
    #[nom(Parse = "Utils::le_u16_div100")]
    pub max_chg_curr: f64,
    #[nom(Parse = "Utils::le_u16_div100")]
    pub max_dischg_curr: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub charge_volt_ref: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub dischg_cut_volt: f64,

    pub bat_status_0: u16,
    pub bat_status_1: u16,
    pub bat_status_2: u16,
    pub bat_status_3: u16,
    pub bat_status_4: u16,
    pub bat_status_5: u16,
    pub bat_status_6: u16,
    pub bat_status_7: u16,
    pub bat_status_8: u16,
    pub bat_status_9: u16,
    pub bat_status_inv: u16,

    pub bat_count: u16,
    pub bat_capacity: u16,

    #[nom(Parse = "Utils::le_u16_div100")]
    pub bat_current: f64,

    pub bms_event_1: u16,
    pub bms_event_2: u16,

    // TODO: probably floats but need non-zero sample data to check. just guessing at the div100.
    #[nom(Parse = "Utils::le_u16_div1000")]
    pub max_cell_voltage: f64,
    #[nom(Parse = "Utils::le_u16_div1000")]
    pub min_cell_voltage: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub max_cell_temp: f64,
    #[nom(Parse = "Utils::le_u16_div10")]
    pub min_cell_temp: f64,

    pub bms_fw_update_state: u16,

    pub cycle_count: u16,

    #[nom(Parse = "Utils::le_u16_div10")]
    pub vbat_inv: f64,

    #[nom(Parse = "Utils::current_time_for_nom")]
    pub time: UnixTime,
    #[nom(Ignore)]
    pub datalog: Serial,
} // }}}

// {{{ ReadInputs
#[derive(Default, Clone, Debug)]
pub struct ReadInputs {
    read_input_1: Option<ReadInput1>,
    read_input_2: Option<ReadInput2>,
    read_input_3: Option<ReadInput3>,
}

impl ReadInputs {
    pub fn set_read_input_1(&mut self, i: ReadInput1) {
        self.read_input_1 = Some(i);
    }
    pub fn set_read_input_2(&mut self, i: ReadInput2) {
        self.read_input_2 = Some(i);
    }
    pub fn set_read_input_3(&mut self, i: ReadInput3) {
        self.read_input_3 = Some(i);
    }

    pub fn to_input_all(&self) -> Option<ReadInputAll> {
        match (
            self.read_input_1.as_ref(),
            self.read_input_2.as_ref(),
            self.read_input_3.as_ref(),
        ) {
            (Some(ri1), Some(ri2), Some(ri3)) => Some(ReadInputAll {
                status: ri1.status,
                v_pv_1: ri1.v_pv_1,
                v_pv_2: ri1.v_pv_2,
                v_pv_3: ri1.v_pv_3,
                v_bat: ri1.v_bat,
                soc: ri1.soc,
                soh: ri1.soh,
                internal_fault: ri1.internal_fault,
                p_pv: ri1.p_pv,
                p_pv_1: ri1.p_pv_1,
                p_pv_2: ri1.p_pv_2,
                p_pv_3: ri1.p_pv_3,
                p_battery: ri1.p_battery,
                p_charge: ri1.p_charge,
                p_discharge: ri1.p_discharge,
                v_ac_r: ri1.v_ac_r,
                v_ac_s: ri1.v_ac_s,
                v_ac_t: ri1.v_ac_t,
                f_ac: ri1.f_ac,
                p_inv: ri1.p_inv,
                p_rec: ri1.p_rec,
                pf: ri1.pf,
                v_eps_r: ri1.v_eps_r,
                v_eps_s: ri1.v_eps_s,
                v_eps_t: ri1.v_eps_t,
                f_eps: ri1.f_eps,
                p_eps: ri1.p_eps,
                s_eps: ri1.s_eps,
                p_grid: ri1.p_grid,
                p_to_grid: ri1.p_to_grid,
                p_to_user: ri1.p_to_user,
                e_pv_day: ri1.e_pv_day,
                e_pv_day_1: ri1.e_pv_day_1,
                e_pv_day_2: ri1.e_pv_day_2,
                e_pv_day_3: ri1.e_pv_day_3,
                e_inv_day: ri1.e_inv_day,
                e_rec_day: ri1.e_rec_day,
                e_chg_day: ri1.e_chg_day,
                e_dischg_day: ri1.e_dischg_day,
                e_eps_day: ri1.e_eps_day,
                e_to_grid_day: ri1.e_to_grid_day,
                e_to_user_day: ri1.e_to_user_day,
                v_bus_1: ri1.v_bus_1,
                v_bus_2: ri1.v_bus_2,
                e_pv_all: ri2.e_pv_all,
                e_pv_all_1: ri2.e_pv_all_1,
                e_pv_all_2: ri2.e_pv_all_2,
                e_pv_all_3: ri2.e_pv_all_3,
                e_inv_all: ri2.e_inv_all,
                e_rec_all: ri2.e_rec_all,
                e_chg_all: ri2.e_chg_all,
                e_dischg_all: ri2.e_dischg_all,
                e_eps_all: ri2.e_eps_all,
                e_to_grid_all: ri2.e_to_grid_all,
                e_to_user_all: ri2.e_to_user_all,
                fault_code: ri2.fault_code,
                warning_code: ri2.warning_code,
                t_inner: ri2.t_inner,
                t_rad_1: ri2.t_rad_1,
                t_rad_2: ri2.t_rad_2,
                t_bat: ri2.t_bat,
                runtime: ri2.runtime,
                max_chg_curr: ri3.max_chg_curr,
                max_dischg_curr: ri3.max_dischg_curr,
                charge_volt_ref: ri3.charge_volt_ref,
                dischg_cut_volt: ri3.dischg_cut_volt,
                bat_status_0: ri3.bat_status_0,
                bat_status_1: ri3.bat_status_1,
                bat_status_2: ri3.bat_status_2,
                bat_status_3: ri3.bat_status_3,
                bat_status_4: ri3.bat_status_4,
                bat_status_5: ri3.bat_status_5,
                bat_status_6: ri3.bat_status_6,
                bat_status_7: ri3.bat_status_7,
                bat_status_8: ri3.bat_status_8,
                bat_status_9: ri3.bat_status_9,
                bat_status_inv: ri3.bat_status_inv,
                bat_count: ri3.bat_count,
                bat_capacity: ri3.bat_capacity,
                bat_current: ri3.bat_current,
                bms_event_1: ri3.bms_event_1,
                bms_event_2: ri3.bms_event_2,
                max_cell_voltage: ri3.max_cell_voltage,
                min_cell_voltage: ri3.min_cell_voltage,
                max_cell_temp: ri3.max_cell_temp,
                min_cell_temp: ri3.min_cell_temp,
                bms_fw_update_state: ri3.bms_fw_update_state,
                cycle_count: ri3.cycle_count,
                vbat_inv: ri3.vbat_inv,
                datalog: ri1.datalog,
                time: ri1.time.clone(),
            }),
            _ => None,
        }
    }
} // }}}

// {{{ TcpFunction
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TcpFunction {
    Heartbeat = 193,
    TranslatedData = 194,
    ReadParam = 195,
    WriteParam = 196,
} // }}}

// {{{ DeviceFunction
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DeviceFunction {
    ReadHold = 3,
    ReadInput = 4,
    WriteSingle = 6,
    WriteMulti = 16,
    // UpdatePrepare = 33
    // UpdateSendData = 34
    // UpdateReset = 35
    // ReadHoldError = 131
    // ReadInputError = 132
    // WriteSingleError = 134
    // WriteMultiError = 144
} // }}}

#[derive(Clone, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Register {
    // Firmware Version Registers (Registers 9-10)
    SlaveVerComVer = 9,          // Slave Ver and Com Ver (firmware versions)
    CntlVerFWVer = 10,           // Cntl Ver and FWVer (firmware versions)
    Register21 = 21,             // not sure of a better name for this one..
    GridConnectTime = 23,        // Grid connection waiting time (s)
    GridReconnectTime = 24,      // Grid reconnection waiting time (s)
    GridVoltConnLow = 25,        // Grid voltage connection low limit (0.1V)
    GridVoltConnHigh = 26,       // Grid voltage connection high limit (0.1V)
    GridFreqConnLow = 27,        // Grid frequency connection low limit (0.01Hz)
    GridFreqConnHigh = 28,       // Grid frequency connection high limit (0.01Hz)
    // Grid Voltage Protection Level 1 (Registers 29-32)
    GridVoltLimit1Low = 29,      // Grid voltage level 1 undervoltage protection point (0.1V)
    GridVoltLimit1High = 30,     // Grid voltage level 1 overvoltage protection point (0.1V)
    GridVoltLimit1LowTime = 31,  // Grid voltage level 1 undervoltage protection time (0.01s)
    GridVoltLimit1HighTime = 32, // Grid voltage level 1 overvoltage protection time (0.01s)
    // Grid Voltage Protection Level 2 (Registers 33-35)
    GridVoltLimit2Low = 33,      // Grid voltage level 2 undervoltage protection point (0.1V)
    GridVoltLimit2High = 34,     // Grid voltage level 2 overvoltage protection point (0.1V)
    GridVoltLimit2LowTime = 35,  // Grid voltage level 2 undervoltage protection time (0.01s)
    // Grid Voltage Protection Level 3 (Registers 37-40)
    GridVoltLimit3Low = 37,      // Grid voltage level 3 undervoltage protection point (0.1V)
    GridVoltLimit3High = 38,     // Grid voltage level 3 overvoltage protection point (0.1V)
    GridVoltLimit3LowTime = 39,  // Grid voltage level 3 undervoltage protection time (0.01s)
    GridVoltLimit3HighTime = 40, // Grid voltage level 3 overvoltage protection time (0.01s)
    // Grid Frequency Protection Level 1 (Registers 42-45)
    GridFreqLimit1Low = 42,      // Grid frequency class 1 underfrequency protection point (0.01Hz)
    GridFreqLimit1High = 43,     // Grid frequency class 1 overfrequency protection point (0.01Hz)
    GridFreqLimit1LowTime = 44,  // Grid frequency class 1 underfrequency protection time (0.01s)
    GridFreqLimit1HighTime = 45, // Grid frequency class 1 overfrequency protection time (0.01s)
    // Grid Frequency Protection Level 2 (Registers 46-49)
    GridFreqLimit2Low = 46,      // Grid frequency level 2 underfrequency protection point (0.01Hz)
    GridFreqLimit2High = 47,     // Grid frequency class 2 overfrequency protection point (0.01Hz)
    GridFreqLimit2LowTime = 48,  // Grid frequency level 2 underfrequency protection time (0.01s)
    GridFreqLimit2HighTime = 49, // Grid frequency class 2 overfrequency protection time (0.01s)
    // Grid Frequency Protection Level 3 (Registers 50-53)
    GridFreqLimit3Low = 50,      // Grid frequency level 3 underfrequency protection point (0.01Hz)
    GridFreqLimit3High = 51,     // Grid frequency class 3 overfrequency protection point (0.01Hz)
    GridFreqLimit3LowTime = 52,  // Grid frequency level 3 underfrequency protection time (0.01s)
    GridFreqLimit3HighTime = 53, // Grid frequency class 3 overfrequency protection time (0.01s)
    // Reactive Power Control (Registers 54-62)
    MaxQPercentForQV = 54,       // Maximum reactive power percentage for Q(V) curve
    V1L = 55,                     // Q(V) curve undervoltage 1 (0.1V)
    V2L = 56,                     // Q(V) curve undervoltage 2 (0.1V)
    V1H = 57,                     // Q(V) curve overvoltage 1 (0.1V)
    V2H = 58,                     // Q(V) curve overvoltage 2 (0.1V)
    ReactivePowerCMDType = 59,    // Reactive power command type
    ActivePowerPercentCMD = 60,   // Active power percentage command
    ReactivePowerPercentCMD = 61, // Reactive power percentage command
    PFCMD = 62,                   // Power factor command
    ChargePowerPercentCmd = 64,  // System Charge Rate (%)
    DischgPowerPercentCmd = 65,  // System Discharge Rate (%)
    AcChargePowerCmd = 66,       // Grid Charge Power Rate (%)
    AcChargeSocLimit = 67,       // AC Charge SOC Limit (%)
    ChargePriorityPowerCmd = 74, // Charge Priority Charge Rate (%)
    ChargePrioritySocLimit = 75, // Charge Priority SOC Limit (%)
    ForcedDischgSocLimit = 83,   // Forced Discarge SOC Limit (%)
    DischgCutOffSocEod = 105,    // Discharge cut-off SOC (%)
    EpsDischgCutoffSocEod = 125, // EPS Discharge cut-off SOC (%)
    AcChargeStartSocLimit = 160, // SOC at which AC charging will begin (%)
    AcChargeEndSocLimit = 161,   // SOC at which AC charging will end (%)
    DelayTimeForOverFDerate = 97, // Open Loop Response time for Frequency Active Power Mode (ms)
    OVFDerateStart = 115,         // Frequency Active Power Mode Over frequency Droop Start dbOF(Hz)
    OVFDerateEnd = 124,           // Frequency Active Power Mode Over frequency Droop End (Hz)
    OVFDeratePctPerHz = 136,      // Frequency Active Power Mode Over frequency Droop kUF (%/Hz) 
    UnderFrDroopStart = 134,      // Frequency Active Power Mode Under frequency Droop Start dbUF(Hz)
    UnderFrDroopEnd = 135,        // Frequency Active Power Mode Under frequency Droop End (Hz)
    UnderFrIncreasePctPerHz = 193,// Frequency Active Power Mode Under frequency Droop kUF (%/Hz)
    // Volt-Watt Open Loop Response Time (Register 183)
    VoltWattDelayTime = 183,      // Volt-Watt Open Loop Response Time (s)
    // Q(V) Curve Reference Parameters (Registers 185-186)
    VrefQv = 185,                // Q(V) curve reference voltage (0.1V)
    VrefFiltertime = 186,        // Q(V) curve voltage filter time (s)
    // Q(V) Curve Control Points (Registers 187-188)
    Q2Qv = 121,                  // Q2 reactive power percentage value for Q(V) curve
    Q3Qv = 187,                  // Q3 reactive power percentage value for Q(V) curve
    Q4Qv = 188,                  // Q4 reactive power percentage value for Q(V) curve
    // QP Priority Control Points (Registers 189-191)
    P1Qp = 189,                  // P1 power percentage value for QP priority curve
    P2Qp = 190,                  // P2 power percentage value for QP priority curve
    P3Qp = 191,                  // P3 power percentage value for QP priority curve
    // Generator Configuration (Register 237)
    GeneratorCoolDownTime = 237,    // Generator cool-down time (0.1 minute units)
    // Grid Always On Configuration (Register 137)
    Register137 = 137,              // Grid Always On control register (bit 0 = enable/disable)
    // AC Coupling Configuration (Registers 179, 220-223)
    Register179 = 179,              // Function Enable 2 register (bit 11 = AC coupling enable/disable)
    ACCoupleStartSOC = 220,         // AC coupling start SOC threshold (0-100%)
    ACCoupleEndSOC = 221,           // AC coupling end SOC threshold (0-101%)
    ACCoupleStartVolt = 222,        // AC coupling start voltage threshold (0.1V units)
    ACCoupleEndVolt = 223,          // AC coupling end voltage threshold (0.1V units)
    // Smart Load Configuration (Registers 213-217, 227-228)
    SmartLoadStartVolt = 213,       // Smart Load start voltage threshold (0.1V units)
    SmartLoadEndVolt = 214,         // Smart Load end voltage threshold (0.1V units)
    SmartLoadStartSOC = 215,        // Smart Load start SOC threshold (0-100%)
    SmartLoadEndSOC = 216,          // Smart Load end SOC threshold (0-100%)
    StartPVPower = 217,             // Start PV power threshold for Smart Load (0.1kW units)
    SmartLoadStartSOCAlt = 227,     // Alternative Smart Load start SOC threshold (0-100%)
    SmartLoadStartVoltAlt = 228,    // Alternative Smart Load start voltage threshold (0.1V units)
    // LCD Configuration (Register 225)
    LCDPassword = 225,            // LCD password for local display access
    ResetSetting = 11,              // Reset settings register (bit 7 = InvReboot)
}

#[derive(Clone, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum RegisterBit {
    // Register 21
    EpsEnable = 1 << 0,
    OVFLoadDerateEnable = 1 << 1,
    DRMSEnable = 1 << 2,
    LVRTEnable = 1 << 3,
    AntiIslandingEnable = 1 << 4,
    NeutralDetectEnable = 1 << 5,
    GridOnPowerSSEnable = 1 << 6,
    AcChargeEnable = 1 << 7,
    SwSeamlessEnable = 1 << 8,
    SetToStandbyEnable = 1 << 9,
    ForcedDischargeEnable = 1 << 10,
    ChargePriorityEnable = 1 << 11,
    ISOEnable = 1 << 12,
    GFCIEnable = 1 << 13,
    DCIEnable = 1 << 14,
    FeedInGridEnable = 1 << 15,
}

#[derive(Clone, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Register110Bit {
    // Register 110
    PvOffGridEnable = 1 << 0,
    FastZeroExportEnable = 1 << 1,
    MicroGridEnable = 1 << 2,
    SharedBatteryEnable = 1 << 3,
    ChargeLastEnable = 1 << 4,
}

#[derive(Clone, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Register179Bit {
    // Register 179 - Function Enable 2
    ACCTDirection = 1 << 0,        // 0-Normal 1-Reversed
    PVCTDirection = 1 << 1,        // 0-Normal 1-Reversed
    AFCIAlarmClr = 1 << 2,         // 0-null 1-clear
    BatWakeupEn = 1 << 3,          // 0-Disable 1-Enable
    VoltWattEn = 1 << 4,           // 0-Disable 1-Enable
    TriptimeUnit = 1 << 5,         // 0-Disable 1-Enable
    ActPowerCMDEN = 1 << 6,        // 0-Disable 1-Enable
    ACCouplingEnable = 1 << 11,    // 0-Disable 1-Enable AC coupling functionality
    SmartLoadEnable = 1 << 13,     // 0-Disable 1-Enable Smart Load functionality
}

#[derive(Clone, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Register137Bit {
    // Register 137 - Grid Always On Control
    GridAlwaysOnDisable = 1 << 0,    // 0-Enable 1-Disable Grid Always On functionality
}

#[derive(Clone, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Register11Bit {
    // Register 11 - Reset settings
    InvReboot = 1 << 7,            // Inverter reboot bit
}

// Register21Bits {{{
#[derive(Clone, Debug, Serialize)]
pub struct Register21Bits {
    pub eps_en: String,
    pub ovf_load_derate_en: String,
    pub drms_en: String,
    pub lvrt_en: String,
    pub anti_island_en: String,
    pub neutral_detect_en: String,
    pub grid_on_power_ss_en: String,
    pub ac_charge_en: String,
    pub sw_seamless_en: String,
    pub set_to_standby_en: String,
    pub forced_discharge_en: String,
    pub charge_priority_en: String,
    pub iso_en: String,
    pub gfci_en: String,
    pub dci_en: String,
    pub feed_in_grid_en: String,
}

impl Register21Bits {
    fn is_bit_set(data: u16, bit: u16) -> String {
        if (data & bit) == bit {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    pub fn new(data: u16) -> Self {
        Self {
            eps_en: Self::is_bit_set(data, 1 << 0),
            ovf_load_derate_en: Self::is_bit_set(data, 1 << 1),
            drms_en: Self::is_bit_set(data, 1 << 2),
            lvrt_en: Self::is_bit_set(data, 1 << 3),
            anti_island_en: Self::is_bit_set(data, 1 << 4),
            neutral_detect_en: Self::is_bit_set(data, 1 << 5),
            grid_on_power_ss_en: Self::is_bit_set(data, 1 << 6),
            ac_charge_en: Self::is_bit_set(data, 1 << 7),
            sw_seamless_en: Self::is_bit_set(data, 1 << 8),
            set_to_standby_en: Self::is_bit_set(data, 1 << 9),
            forced_discharge_en: Self::is_bit_set(data, 1 << 10),
            charge_priority_en: Self::is_bit_set(data, 1 << 11),
            iso_en: Self::is_bit_set(data, 1 << 12),
            gfci_en: Self::is_bit_set(data, 1 << 13),
            dci_en: Self::is_bit_set(data, 1 << 14),
            feed_in_grid_en: Self::is_bit_set(data, 1 << 15),
        }
    }
} // }}}

// Register110Bits {{{
#[derive(Clone, Debug, Serialize)]
pub struct Register110Bits {
    pub pv_off_grid_en: String,
    pub fast_zero_export_en: String,
    pub micro_grid_en: String,
    pub shared_battery_en: String,
    pub charge_last_en: String,
}
impl Register110Bits {
    fn is_bit_set(data: u16, bit: u16) -> String {
        if (data & bit) == bit {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    pub fn new(data: u16) -> Self {
        Self {
            pv_off_grid_en: Self::is_bit_set(data, 1 << 0),
            fast_zero_export_en: Self::is_bit_set(data, 1 << 1),
            micro_grid_en: Self::is_bit_set(data, 1 << 2),
            shared_battery_en: Self::is_bit_set(data, 1 << 3),
            charge_last_en: Self::is_bit_set(data, 1 << 4),
        }
    }
} // }}}

// Register179Bits {{{
#[derive(Clone, Debug, Serialize)]
pub struct Register179Bits {
    pub acct_direction_en: String,
    pub pvct_direction_en: String,
    pub afci_alarm_clr_en: String,
    pub bat_wakeup_en: String,
    pub volt_watt_en: String,
    pub triptime_unit_en: String,
    pub act_power_cmd_en: String,
    pub ac_coupling_enable: String,
    pub smart_load_enable: String,
}

impl Register179Bits {
    fn is_bit_set(data: u16, bit: u16) -> String {
        if (data & bit) == bit {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    pub fn new(data: u16) -> Self {
        Self {
            acct_direction_en: Self::is_bit_set(data, 1 << 0),
            pvct_direction_en: Self::is_bit_set(data, 1 << 1),
            afci_alarm_clr_en: Self::is_bit_set(data, 1 << 2),
            bat_wakeup_en: Self::is_bit_set(data, 1 << 3),
            volt_watt_en: Self::is_bit_set(data, 1 << 4),
            triptime_unit_en: Self::is_bit_set(data, 1 << 5),
            act_power_cmd_en: Self::is_bit_set(data, 1 << 6),
            ac_coupling_enable: Self::is_bit_set(data, 1 << 11),
            smart_load_enable: Self::is_bit_set(data, 1 << 13),
        }
    }
} // }}}

// Register137Bits {{{
#[derive(Clone, Debug, Serialize)]
pub struct Register137Bits {
    pub grid_always_on_disable: String,
}

impl Register137Bits {
    fn is_bit_set(data: u16, bit: u16) -> String {
        if (data & bit) == bit {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    pub fn new(data: u16) -> Self {
        Self {
            grid_always_on_disable: Self::is_bit_set(data, 1 << 0),
        }
    }
} // }}}

// SystemInfo {{{
#[derive(Clone, Debug, Serialize)]
pub struct SystemInfo {
    pub model: String,
    pub serial_number: String,
    pub firmware_version: String,
}

impl SystemInfo {
    pub fn new(model_reg0: u16, model_reg1: u16, serial_regs: [u16; 5], firmware_reg: u16) -> Self {
        // Decode model information from registers 0-1
        let model = Self::decode_model(model_reg0, model_reg1);
        
        // Decode serial number from registers 2-6
        let serial_number = Self::decode_serial_number(serial_regs);
        
        // Decode firmware version from register 7
        let firmware_version = Self::decode_firmware(firmware_reg);
        
        Self {
            model,
            serial_number,
            firmware_version,
        }
    }
    
    fn decode_model(reg0: u16, reg1: u16) -> String {
        // Extract individual fields from model registers
        let lithium_type = (reg0 >> 12) & 0x0F;
        let power_rating = (reg0 >> 8) & 0x0F;
        let lead_acid_type = (reg0 >> 4) & 0x0F;
        let _rule_mask = reg0 & 0x0F;
        let battery_type = (reg1 >> 12) & 0x0F;
        let meter_brand = (reg1 >> 8) & 0x0F;
        let _measurement = (reg1 >> 4) & 0x0F;
        let _rule = reg1 & 0x0F;
        
        format!("EG4-{power_rating}kW-LV (Li:{lithium_type}, Pb:{lead_acid_type}, Bat:{battery_type}, Meter:{meter_brand})")
    }
    
    fn decode_serial_number(regs: [u16; 5]) -> String {
        // Combine 5 registers into a serial number string
        let mut serial = String::new();
        for reg in regs.iter() {
            // Each register contains 2 ASCII characters
            let high_byte = ((reg >> 8) & 0xFF) as u8;
            let low_byte = (reg & 0xFF) as u8;
            
            if (32..=126).contains(&high_byte) {
                serial.push(high_byte as char);
            }
            if (32..=126).contains(&low_byte) {
                serial.push(low_byte as char);
            }
        }
        serial
    }
    
    fn decode_firmware(reg: u16) -> String {
        // Decode firmware version from register 7
        let major = (reg >> 8) & 0xFF;
        let minor = reg & 0xFF;
        format!("v{major}.{minor}")
    }
} // }}}

#[enum_dispatch]
pub trait PacketCommon {
    fn datalog(&self) -> Serial;
    fn set_datalog(&mut self, datalog: Serial);
    fn inverter(&self) -> Option<Serial>;
    fn set_inverter(&mut self, serial: Serial);
    fn protocol(&self) -> u16;
    fn tcp_function(&self) -> TcpFunction;
    fn bytes(&self) -> Vec<u8>;

    fn register(&self) -> u16 {
        unimplemented!("register() not implemented");
    }
    fn value(&self) -> u16 {
        unimplemented!("value() not implemented");
    }
}

pub struct TcpFrameFactory;
impl TcpFrameFactory {
    pub fn build(data: &Packet) -> Vec<u8> {
        let data_bytes = data.bytes();
        let data_length = data_bytes.len() as u8;
        let frame_length = (18 + data_length) as u16;

        // debug!("data_length={}, frame_length={}", data_length, frame_length);

        let mut r = vec![0; frame_length as usize];

        r[0] = 161;
        r[1] = 26;
        r[2..4].copy_from_slice(&data.protocol().to_le_bytes());
        r[4..6].copy_from_slice(&(frame_length - 6).to_le_bytes());
        r[6] = 1; // unsure what this is, always seems to be 1
        r[7] = data.tcp_function() as u8;

        r[8..18].copy_from_slice(&data.datalog().data());
        // WIP - trying to work out how to learn the inverter sn
        //r[8..18].copy_from_slice(&[0; 10]);

        r[18..].copy_from_slice(&data_bytes);

        r
    }
}

#[enum_dispatch(PacketCommon)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Packet {
    Heartbeat(Heartbeat),
    TranslatedData(TranslatedData),
    ReadParam(ReadParam),
    WriteParam(WriteParam),
}

#[derive(PartialEq)]
enum PacketSource {
    Inverter,
    Client,
}

/////////////
//
// HEARTBEATS
//
/////////////

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Heartbeat {
    pub datalog: Serial,
}
impl Heartbeat {
    fn decode(input: &[u8]) -> Result<Self> {
        let len = input.len();
        if len < 19 {
            bail!("heartbeat packet too short");
        }

        // assert that the final byte is 0, meaning 0 data bytes follow it
        if input[18] != 0 {
            bail!("heartbeat with non-zero ({}) length byte?", input[18]);
        }

        let datalog = Serial::new(&input[8..18])?;

        Ok(Self { datalog })
    }
}

impl PacketCommon for Heartbeat {
    fn protocol(&self) -> u16 {
        2
    }

    fn datalog(&self) -> Serial {
        self.datalog
    }
    fn set_datalog(&mut self, datalog: Serial) {
        self.datalog = datalog;
    }
    fn inverter(&self) -> Option<Serial> {
        None
    }
    fn set_inverter(&mut self, _datalog: Serial) {}

    fn tcp_function(&self) -> TcpFunction {
        TcpFunction::Heartbeat
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0]
    }
}

/////////////
//
// TRANSLATED DATA
//
/////////////

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TranslatedData {
    pub datalog: Serial,
    pub device_function: DeviceFunction, // ReadHold or ReadInput etc..
    pub inverter: Serial,                // inverter serial
    pub register: u16,                   // first register of values
    pub values: Vec<u8>,                 // undecoded, since can be u16 or u32s?
}
impl TranslatedData {
    pub fn pairs(&self) -> Vec<(u16, u16)> {
        self.values
            .chunks(2)
            .enumerate()
            .map(|(pos, value)| (self.register + pos as u16, Utils::u16ify(value, 0)))
            .collect()
    }

    pub fn read_input(&self) -> Result<ReadInput> {
        // note len() is of Vec<u8>, so not register count
        match (self.register, self.values.len()) {
            (0, 254) => Ok(ReadInput::ReadInputAll(Box::new(self.read_input_all()?))),
            // (127, 254) has been seen but containing all zeroes, not sure what they are
            (0, 80) => Ok(ReadInput::ReadInput1(self.read_input1()?)),
            (40, 80) => Ok(ReadInput::ReadInput2(self.read_input2()?)),
            (80, 80) => Ok(ReadInput::ReadInput3(self.read_input3()?)),
            (r1, r2) => bail!("unhandled ReadInput register={} len={}", r1, r2),
        }
    }

    fn read_input_all(&self) -> Result<ReadInputAll> {
        match ReadInputAll::parse(&self.values) {
            Ok((_, mut r)) => {
                r.p_pv = r.p_pv_1 + r.p_pv_2 + r.p_pv_3;
                r.p_grid = r.p_to_user as i32 - r.p_to_grid as i32;
                r.p_battery = r.p_charge as i32 - r.p_discharge as i32;
                r.e_pv_day = Utils::round(r.e_pv_day_1 + r.e_pv_day_2 + r.e_pv_day_3, 1);
                r.e_pv_all = Utils::round(r.e_pv_all_1 + r.e_pv_all_2 + r.e_pv_all_3, 1);
                r.datalog = self.datalog;
                Ok(r)
            }
            Err(_) => Err(anyhow!("meh")),
        }
    }

    fn read_input1(&self) -> Result<ReadInput1> {
        match ReadInput1::parse(&self.values) {
            Ok((_, mut r)) => {
                r.p_pv = r.p_pv_1 + r.p_pv_2 + r.p_pv_3;
                r.p_grid = r.p_to_user as i32 - r.p_to_grid as i32;
                r.p_battery = r.p_charge as i32 - r.p_discharge as i32;
                r.e_pv_day = Utils::round(r.e_pv_day_1 + r.e_pv_day_2 + r.e_pv_day_3, 1);
                r.datalog = self.datalog;
                Ok(r)
            }
            Err(_) => Err(anyhow!("meh")),
        }
    }

    fn read_input2(&self) -> Result<ReadInput2> {
        match ReadInput2::parse(&self.values) {
            Ok((_, mut r)) => {
                r.e_pv_all = Utils::round(r.e_pv_all_1 + r.e_pv_all_2 + r.e_pv_all_3, 1);
                r.datalog = self.datalog;
                Ok(r)
            }
            Err(_) => Err(anyhow!("meh")),
        }
    }

    fn read_input3(&self) -> Result<ReadInput3> {
        match ReadInput3::parse(&self.values) {
            Ok((_, mut r)) => {
                r.datalog = self.datalog;
                Ok(r)
            }
            Err(_) => Err(anyhow!("meh")),
        }
    }

    fn decode(input: &[u8]) -> Result<Self> {
        let len = input.len();
        
        // Allow 38 bytes as minimum for all packet types
        // This accommodates WriteSingle reply packets which can be shorter
        if len < 38 {
            bail!("TranslatedData::decode packet too short (got {} bytes, need at least 38)", len);
        }

        let protocol = Utils::u16ify(input, 2);
        let datalog = Serial::new(&input[8..18])?;

        let data = &input[20..len - 2];

        let checksum = &input[len - 2..];
        if Self::checksum(data) != checksum {
            bail!(
                "TranslatedData::decode checksum mismatch - got {:?}, expected {:?}",
                checksum,
                Self::checksum(data)
            );
        }

        //let address = data[0]; // 0=client, 1=inverter?
        let device_function = DeviceFunction::try_from(data[1])?;
        let inverter = Serial::new(&data[2..12])?;
        let register = Utils::u16ify(data, 12);

        let mut value_len = 2;
        let mut value_offset = 14;

        if Self::has_value_length_byte(PacketSource::Inverter, protocol, device_function) {
            value_len = data[value_offset] as usize;
            value_offset += 1;
        }

        let values = data[value_offset..].to_vec();

        if values.len() != value_len {
            bail!(
                "TranslatedData::decode mismatch: values.len()={}, value_length_byte={}",
                values.len(),
                value_len
            );
        }

        Ok(Self {
            datalog,
            device_function,
            inverter,
            register,
            values,
        })
    }

    fn has_value_length_byte(
        source: PacketSource,
        protocol: u16,
        device_function: DeviceFunction,
    ) -> bool {
        use DeviceFunction::*;

        let p1 = protocol == 1;
        let psi = source == PacketSource::Inverter;
        match device_function {
            ReadHold | ReadInput => !p1 && psi,
            WriteSingle => false,
            WriteMulti => !p1 && !psi,
        }
    }

    fn checksum(data: &[u8]) -> [u8; 2] {
        crc16::State::<crc16::MODBUS>::calculate(data).to_le_bytes()
    }
}

impl PacketCommon for TranslatedData {
    fn protocol(&self) -> u16 {
        if self.device_function == DeviceFunction::WriteMulti {
            2
        } else {
            1
        }
    }

    fn datalog(&self) -> Serial {
        self.datalog
    }
    fn set_datalog(&mut self, datalog: Serial) {
        self.datalog = datalog;
    }

    fn inverter(&self) -> Option<Serial> {
        Some(self.inverter)
    }
    fn set_inverter(&mut self, serial: Serial) {
        self.inverter = serial;
    }

    fn tcp_function(&self) -> TcpFunction {
        TcpFunction::TranslatedData
    }

    fn bytes(&self) -> Vec<u8> {
        let mut data = vec![0; 16];

        // data[2] (address) is 0 when writing to inverter, 1 when reading from it
        data[3] = self.device_function as u8;

        // experimental: looks like maybe you don't need to fill this in..
        data[4..14].copy_from_slice(&self.inverter.data());
        //data[4..14].copy_from_slice(&[0; 10]);

        data[14..16].copy_from_slice(&self.register.to_le_bytes());

        if self.device_function == DeviceFunction::WriteMulti {
            let register_count = self.pairs().len() as u16;
            data.extend_from_slice(&register_count.to_le_bytes());
        }

        if Self::has_value_length_byte(PacketSource::Client, self.protocol(), self.device_function)
        {
            let len = self.values.len() as u8;
            data.extend_from_slice(&[len]);
        }

        let mut m = Vec::new();
        for i in &self.values {
            m.extend_from_slice(&i.to_le_bytes());
        }
        data.append(&mut m);

        // the first two bytes are the data length, excluding checksum which we'll add next
        let data_length = data.len() as u16;
        data[0..2].copy_from_slice(&data_length.to_le_bytes());

        // checksum does not include the first two bytes (data length)
        data.extend_from_slice(&Self::checksum(&data[2..]));

        data
    }

    fn register(&self) -> u16 {
        self.register
    }

    fn value(&self) -> u16 {
        Utils::u16ify(&self.values, 0)
    }
}

/////////////
//
// READ PARAM
//
/////////////

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ReadParam {
    pub datalog: Serial,
    pub register: u16,   // first register of values
    pub values: Vec<u8>, // undecoded, since can be u16 or i32s?
}
impl ReadParam {
    pub fn pairs(&self) -> Vec<(u16, u16)> {
        self.values
            .chunks(2)
            .enumerate()
            .map(|(pos, value)| (self.register + pos as u16, Utils::u16ify(value, 0)))
            .collect()
    }

    fn decode(input: &[u8]) -> Result<Self> {
        let len = input.len();
        if len < 24 {
            bail!("ReadParam::decode packet too short (got {} bytes, need at least 24)", len);
        }

        let protocol = Utils::u16ify(input, 2);
        let datalog = Serial::new(&input[8..18])?;

        let data = &input[18..];
        let register = Utils::u16ify(data, 0);

        let mut value_len = 2;
        let mut value_offset = 2;

        if Self::has_value_length_bytes(protocol) {
            value_len = Utils::u16ify(data, value_offset) as usize;
            value_offset += 2;
        }

        let values = data[value_offset..].to_vec();

        if values.len() != value_len {
            bail!(
                "ReadParam::decode mismatch: values.len()={}, value_length_byte={}",
                values.len(),
                value_len
            );
        }

        Ok(Self {
            datalog,
            register,
            values,
        })
    }

    fn has_value_length_bytes(protocol: u16) -> bool {
        protocol == 2
    }
}

impl PacketCommon for ReadParam {
    fn protocol(&self) -> u16 {
        2
    }

    fn datalog(&self) -> Serial {
        self.datalog
    }
    fn set_datalog(&mut self, datalog: Serial) {
        self.datalog = datalog;
    }
    fn inverter(&self) -> Option<Serial> {
        None
    }
    fn set_inverter(&mut self, _datalog: Serial) {}

    fn tcp_function(&self) -> TcpFunction {
        TcpFunction::ReadParam
    }

    fn bytes(&self) -> Vec<u8> {
        vec![self.register() as u8, 0]
    }

    fn register(&self) -> u16 {
        self.register
    }

    fn value(&self) -> u16 {
        Utils::u16ify(&self.values, 0)
    }
}

/////////////
//
// WRITE PARAM
//
/////////////

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct WriteParam {
    pub datalog: Serial,
    pub register: u16,   // first register of values
    pub values: Vec<u8>, // undecoded, since can be u16 or i32s?
}
impl WriteParam {
    pub fn pairs(&self) -> Vec<(u16, u16)> {
        self.values
            .chunks(2)
            .enumerate()
            .map(|(pos, value)| (self.register + pos as u16, Utils::u16ify(value, 0)))
            .collect()
    }

    fn decode(input: &[u8]) -> Result<Self> {
        let len = input.len();
        if len < 21 {
            bail!("WriteParam::decode packet too short (got {} bytes, need at least 21)", len);
        }

        let protocol = Utils::u16ify(input, 2);
        let datalog = Serial::new(&input[8..18])?;

        let data = &input[18..];
        let register = u16::from(data[0]);

        let mut value_len = 2;
        let mut value_offset = 1;

        if Self::has_value_length_bytes(protocol) {
            value_len = Utils::u16ify(data, value_offset) as usize;
            value_offset += 2;
        }

        let values = data[value_offset..].to_vec();

        if values.len() != value_len {
            bail!(
                "WriteParam::decode mismatch: values.len()={}, value_length_byte={}",
                values.len(),
                value_len
            );
        }

        Ok(Self {
            datalog,
            register,
            values,
        })
    }

    fn has_value_length_bytes(_protocol: u16) -> bool {
        false
    }
}

impl PacketCommon for WriteParam {
    fn protocol(&self) -> u16 {
        2
    }

    fn datalog(&self) -> Serial {
        self.datalog
    }
    fn set_datalog(&mut self, datalog: Serial) {
        self.datalog = datalog;
    }
    fn inverter(&self) -> Option<Serial> {
        None
    }
    fn set_inverter(&mut self, _datalog: Serial) {}

    fn tcp_function(&self) -> TcpFunction {
        TcpFunction::WriteParam
    }

    fn bytes(&self) -> Vec<u8> {
        let mut data = vec![0; 2];

        data[0..2].copy_from_slice(&self.register.to_le_bytes());

        let len = self.values.len() as u16;
        data.extend_from_slice(&len.to_le_bytes());

        let mut m = Vec::new();
        for i in &self.values {
            m.extend_from_slice(&i.to_le_bytes());
        }
        data.append(&mut m);

        data
    }

    fn register(&self) -> u16 {
        self.register
    }

    fn value(&self) -> u16 {
        Utils::u16ify(&self.values, 0)
    }
}

pub struct Parser;
impl Parser {
    pub fn parse(input: &[u8]) -> Result<Packet> {
        let input_len = input.len() as u8;
        if input_len < 18 {
            bail!("packet less than 18 bytes?");
        }

        if input[0..2] != [161, 26] {
            bail!("invalid packet prefix");
        }

        if input_len < input[4] - 6 {
            bail!(
                "Parser::parse mismatch: input.len()={},  frame_length={}",
                input_len,
                input[4] - 6
            );
        }

        let r = match TcpFunction::try_from(input[7])? {
            TcpFunction::Heartbeat => Packet::Heartbeat(Heartbeat::decode(input)?),
            TcpFunction::TranslatedData => Packet::TranslatedData(TranslatedData::decode(input)?),
            TcpFunction::ReadParam => Packet::ReadParam(ReadParam::decode(input)?),
            TcpFunction::WriteParam => Packet::WriteParam(WriteParam::decode(input)?),
            //_ => bail!("unhandled: tcp_function={} input={:?}", input[7], input),
        };

        Ok(r)
    }
}

pub struct StatusString;
impl StatusString {
    pub fn from_value(status: u16) -> &'static str {
        match status {
            0x00 => "Standby",
            0x01 => "Fault",
            0x02 => "FW Updating",
            0x04 => "PV On-grid",
            0x08 => "PV Charge",
            0x0C => "PV Charge On-grid",
            0x10 => "Battery On-grid",
            0x11 => "Bypass",
            0x14 => "PV & Battery On-grid",
            0x19 => "PV Charge + Bypass",
            0x20 => "AC Charge",
            0x28 => "PV & AC Charge",
            0x40 => "Battery Off-grid",
            0x80 => "PV Off-grid",
            0xC0 => "PV & Battery Off-grid",
            0x88 => "PV Charge Off-grid",

            _ => "Unknown",
        }
    }
}

pub struct WarningCodeString;
impl WarningCodeString {
    pub fn from_value(value: u32) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=31)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u32) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=31)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "W000: Communication failure with battery",
            1 => "W001: AFCI communication fault",
            2 => "W002: AFCI high",
            3 => "W003: Communication failure with meters",
            4 => "W004: Battery status check",
            5 => "W005: AutoTest failure",
            6 => "W006: RSD active",
            7 => "W007: LCD communication fault",
            8 => "W008: Software mismatch",
            9 => "W009: Fan Stuck",
            10 => "W010: AC over load",
            11 => "W011: Slave overflow",
            12 => "W012: Battery On Mos",
            13 => "W013: Over temperature",
            14 => "W014: Multi-Master set in parallel system",
            15 => "W015: Battery Reverse",
            16 => "W016: No AC Connection",
            17 => "W017: AC Voltage out of range",
            18 => "W018: AC Frequency out of range",
            19 => "W019: AC inconsistent in parallel system2",
            20 => "W020: PV Isolation low",
            21 => "W021: Leakage I high",
            22 => "W022: DC injection high",
            23 => "W023: PV short circuit",
            24 => "W024: W024",
            25 => "W025: Battery voltage high",
            26 => "W026: Battery voltage low",
            27 => "W027: Battery open",
            28 => "W028: EPS Over load",
            29 => "W029: EPS voltage high",
            30 => "W030: Check meter connection",
            31 => "W031: EPS DCV high",
            _ => "Unknown Warning",
        }
    }
}

pub struct FaultCodeString;
impl FaultCodeString {
    pub fn from_value(value: u32) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=31)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u32) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=31)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "E000: Internal communication fault 1",
            1 => "E001: Model fault",
            2 => "E002: Battery anti-reverse Mos Fail",
            3 => "E003: Internal CT offset out of range",
            4 => "E004: Reserved",
            5 => "E005: Reserved",
            6 => "E006: Reserved",
            7 => "E007: Reserved",
            8 => "E008: CAN communication Fault in Parallel System",
            9 => "E009: Primary Inverter Lost in Parallel System",
            10 => "E010: Multi Master",
            11 => "E011: AC Connection Diff",
            12 => "E012: UPS output short circuit",
            13 => "E013: UPS output current reversed",
            14 => "E014: BUS short circuit",
            15 => "E015: Phase Error in 3 Phase System",
            16 => "E016: Relay fault",
            17 => "E017: Internal communication fault 2",
            18 => "E018: Internal communication fault 3",
            19 => "E019: Bus voltage high",
            20 => "E020: EPS connection fault",
            21 => "E021: PV voltage high",
            22 => "E022: Over current protected by TZ",
            23 => "E023: Neutral fault",
            24 => "E024: PV short",
            25 => "E025: Temperature over range",
            26 => "E026: Internal Fault",
            27 => "E027: Sample inconsistent between main and slave CPU",
            28 => "E028: Sync signal lost in parallel system",
            29 => "E029: Sync trigger signal lost in parallel system",
            30 => "E030: E030",
            31 => "E031: Internal communication fault 4",
            _ => "Unknown Fault",
        }
    }
}

pub struct BmsFaultCodeString;
impl BmsFaultCodeString {
    pub fn from_value(value: u16) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=15)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u16) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=15)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "LSP_BAT_FAULT_000: BDC Over Curr Fault",
            1 => "LSP_BAT_FAULT_001: BDC No Balance Fault",
            2 => "LSP_BAT_FAULT_002: BDC Over Temp Fault",
            3 => "LSP_BAT_FAULT_003: Lithium Battery Open",
            4 => "LSP_BAT_FAULT_004: BMS Not Ready",
            5 => "LSP_BAT_FAULT_005: Soft Start Fail",
            6 => "LSP_BAT_FAULT_006: Battery Voltage High",
            7 => "LSP_BAT_FAULT_007: Battery Voltage Low",
            8 => "LSP_BAT_FAULT_008: BMS Error",
            9 => "LSP_BAT_FAULT_009: BMS COM Fault",
            10 => "LSP_BAT_FAULT_010: Battery Sleep",
            11 => "LSP_BAT_FAULT_011: Lead-acid Battery NTC Open",
            12 => "LSP_BAT_FAULT_012: Battery Overload",
            13 => "LSP_BAT_FAULT_013: Battery Temp Over Range",
            14 => "LSP_BAT_FAULT_014: Battery Relay Fault",
            15 => "LSP_BAT_FAULT_015: Battery Reversed",
            _ => "Unknown BMS Fault",
        }
    }
}

pub struct BmsWarningCodeString;
impl BmsWarningCodeString {
    pub fn from_value(value: u16) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=15)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u16) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=15)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "LSP_MDSP_WARNING_000: Reserved",
            1 => "LSP_MDSP_WARNING_001: Reserved",
            2 => "LSP_MDSP_WARNING_002: Reserved",
            3 => "LSP_MDSP_WARNING_003: Reserved",
            4 => "LSP_MDSP_WARNING_004: Reserved",
            5 => "LSP_MDSP_WARNING_005: Reserved",
            6 => "LSP_MDSP_WARNING_006: Reserved",
            7 => "LSP_MDSP_WARNING_007: Reserved",
            8 => "LSP_MDSP_WARNING_008: PVout OVP",
            9 => "LSP_MDSP_WARNING_009: PVout OCP",
            10 => "LSP_MDSP_WARNING_010: PVout Short",
            11 => "LSP_MDSP_WARNING_011: PVout OTP",
            12 => "LSP_MDSP_WARNING_012: Reserved",
            13 => "LSP_MDSP_WARNING_013: Reserved",
            14 => "LSP_MDSP_WARNING_014: Reserved",
            15 => "LSP_MDSP_WARNING_015: Reserved",
            _ => "Unknown BMS Warning",
        }
    }
}

pub struct MidboxFaultCodeString;
impl MidboxFaultCodeString {
    pub fn from_value(value: u16) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=15)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u16) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=15)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "MIDBOX_FAULT_000: Reserved",
            1 => "MIDBOX_FAULT_001: Reserved",
            2 => "MIDBOX_FAULT_002: Reserved",
            3 => "MIDBOX_FAULT_003: Reserved",
            4 => "MIDBOX_FAULT_004: 12K is the old firmware",
            5 => "MIDBOX_FAULT_005: NEC protection",
            6 => "MIDBOX_FAULT_006: Grid port over current",
            7 => "MIDBOX_FAULT_007: Load port Over current",
            8 => "MIDBOX_FAULT_008: GEN port over current",
            9 => "MIDBOX_FAULT_009: UPS port over current",
            10 => "MIDBOX_FAULT_010: Smart port 1 over current",
            11 => "MIDBOX_FAULT_011: Smart Port 2 over current",
            12 => "MIDBOX_FAULT_012: Smart Port 3 over current",
            13 => "MIDBOX_FAULT_013: Smart Port 4 over current",
            14 => "MIDBOX_FAULT_014: Reserved",
            15 => "MIDBOX_FAULT_015: Reserved",
            _ => "Unknown Midbox Fault",
        }
    }
}

pub struct MidboxWarningCodeString;
impl MidboxWarningCodeString {
    pub fn from_value(value: u16) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=15)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u16) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=15)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "MIDBOX_WARNING_000: Parallel communication abnormality",
            1 => "MIDBOX_WARNING_001: Reserved",
            2 => "MIDBOX_WARNING_002: Reserved",
            3 => "MIDBOX_WARNING_003: Load shedding overload",
            4 => "MIDBOX_WARNING_004: Reserved",
            5 => "MIDBOX_WARNING_005: Reserved",
            6 => "MIDBOX_WARNING_006: Parallel communication RSD alarm",
            7 => "MIDBOX_WARNING_007: Reserved",
            8 => "MIDBOX_WARNING_008: Reserved",
            9 => "MIDBOX_WARNING_009: Reserved",
            10 => "MIDBOX_WARNING_010: Reserved",
            11 => "MIDBOX_WARNING_011: Reserved",
            12 => "MIDBOX_WARNING_012: Reserved",
            13 => "MIDBOX_WARNING_013: Reserved",
            14 => "MIDBOX_WARNING_014: Reserved",
            15 => "MIDBOX_WARNING_015: GEN voltage and frequency abnormality",
            16 => "MIDBOX_WARNING_016: Grid voltage over/under voltage abnormality",
            17 => "MIDBOX_WARNING_017: Grid voltage abnormality",
            18 => "MIDBOX_WARNING_018: Grid frequency abnormality",
            _ => "Unknown Midbox Warning",
        }
    }
}

pub struct BmsEvent1String;
impl BmsEvent1String {
    pub fn from_value(value: u16) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=15)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u16) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=15)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "BMS_FAULT_0: Battery management system fault 0",
            1 => "BMS_FAULT_1: Battery management system fault 1",
            2 => "BMS_FAULT_2: Battery management system fault 2",
            3 => "BMS_FAULT_3: Battery management system fault 3",
            4 => "BMS_FAULT_4: Battery management system fault 4",
            5 => "BMS_FAULT_5: Battery management system fault 5",
            6 => "BMS_FAULT_6: Battery management system fault 6",
            7 => "BMS_FAULT_7: Battery management system fault 7",
            8 => "BMS_FAULT_8: Battery management system fault 8",
            9 => "BMS_FAULT_9: Battery management system fault 9",
            10 => "BMS_FAULT_10: Battery management system fault 10",
            11 => "BMS_FAULT_11: Battery management system fault 11",
            12 => "BMS_FAULT_12: Battery management system fault 12",
            13 => "BMS_FAULT_13: Battery management system fault 13",
            14 => "BMS_FAULT_14: Battery management system fault 14",
            15 => "BMS_FAULT_15: Battery management system fault 15",
            _ => "Unknown BMS fault",
        }
    }
}

pub struct BmsEvent2String;
impl BmsEvent2String {
    pub fn from_value(value: u16) -> &'static str {
        if value == 0 {
            return "OK";
        }

        (0..=15)
            .find(|i| value & (1 << i) > 0)
            .map(Self::from_bit)
            .unwrap()
    }

    // New method for multi-bit reporting
    pub fn from_value_all_bits(value: u16) -> Vec<&'static str> {
        if value == 0 {
            return vec!["OK"];
        }

        (0..=15)
            .filter_map(|i| {
                if value & (1 << i) > 0 {
                    Some(Self::from_bit(i))
                } else {
                    None
                }
            })
            .collect()
    }

    fn from_bit(bit: usize) -> &'static str {
        match bit {
            0 => "BMS_WARNING_0: Battery management system warning 0",
            1 => "BMS_WARNING_1: Battery management system warning 1",
            2 => "BMS_WARNING_2: Battery management system warning 2",
            3 => "BMS_WARNING_3: Battery management system warning 3",
            4 => "BMS_WARNING_4: Battery management system warning 4",
            5 => "BMS_WARNING_5: Battery management system warning 5",
            6 => "BMS_WARNING_6: Battery management system warning 6",
            7 => "BMS_WARNING_7: Battery management system warning 7",
            8 => "BMS_WARNING_8: Battery management system warning 8",
            9 => "BMS_WARNING_9: Battery management system warning 9",
            10 => "BMS_WARNING_10: Battery management system warning 10",
            11 => "BMS_WARNING_11: Battery management system warning 11",
            12 => "BMS_WARNING_12: Battery management system warning 12",
            13 => "BMS_WARNING_13: Battery management system warning 13",
            14 => "BMS_WARNING_14: Battery management system warning 14",
            15 => "BMS_WARNING_15: Battery management system warning 15",
            _ => "Unknown BMS warning",
        }
    }
}
