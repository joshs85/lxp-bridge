# LXP Inverter Input Registers (Read-Only)

## Overview
This document describes all input registers (read-only) available on LXP inverters. These registers provide real-time status, monitoring data, and system information.

**Note**: Input registers support function code 0x04 (Read Input Registers) and cannot be modified by external commands.

## Working Modes Definition
The inverter operates in various working modes that are reported via Register 0 (State). The following table defines all possible operational modes:

### Table 9: Working Modes Definition

| Status Code | Description | Remark |
|-------------|-------------|---------|
| **0x00** | Standby | Standby |
| **0x01** | Fault | Inverter is in Fault status |
| **0x02** | Programming | Firmware update is in progress |
| **0x04** | PV on-grid mode | PV power export to load and grid |
| **0x08** | PV Charge mode | PV power used to charge battery |
| **0x0C** | PV Charge+on-grid mode | PV used to charge battery, and excess part export to load or grid |
| **0x10** | Battery on-grid mode | Battery discharge power to load or grid |
| **0x14** | PV+Battery on-grid mode | PV and Battery discharge power to load or grid |
| **0x20** | AC Charge mode | Grid power used to charge battery |
| **0x28** | PV+AC charge mode | PV power and grid power used to charge battery |
| **0x40** | Battery off-grid mode | Battery power backup |
| **0x80** | PV off-grid mode | PV power power backup (PV power is unstable and this mode is not suggested) |
| **0xC0** | PV+battery off-grid mode | PV+ Battery power backup |
| **0x88** | PV charge +off-grid mode | PV power used to power backup and the excess part used to charge battery |

**Note**: Status codes 0x80 and 0xC0 are highlighted as not recommended due to PV power instability in off-grid modes.

## Fault Code and Warning Code Definitions
The inverter reports system faults and warnings via specific fault and warning codes. These codes are used in Register 6 (Internal Fault) and Registers 60-63 (Fault Code and Warning Code).

### Table 10: Fault Code and Warning Code Definition

| Bit 0-31 | Fault Description | Fault Code | Warning Description | Warning Code |
|----------|-------------------|-------------|---------------------|---------------|
| **Bit 0** | Internal communication fault 1 | E000 | Battery communication failure | W000 |
| **Bit 1** | Model fault | E001 | AFCI communication failure | W001 |
| **Bit 2** | rsvd | E002 | AFCI High | W002 |
| **Bit 3** | rsvd | E003 | Meter communication failure | W003 |
| **Bit 4** | rsvd | E004 | Both charge and discharge forbidden by battery | W004 |
| **Bit 5** | rsvd | E005 | Auto test failed | W005 |
| **Bit 6** | rsvd | E006 | rsvd | W006 |
| **Bit 7** | rsvd | E007 | LCD communication failure | W007 |
| **Bit 8** | Paralleling CAN communication lost | E008 | FW version mismatching | W008 |
| **Bit 9** | Master unit lost in paralleling system | E009 | Fan stuck | W009 |
| **Bit 10** | Multiple master units in paralleling system | E010 | rsvd | W010 |
| **Bit 11** | AC input inconsistent in paralleling system | E011 | Parallel number out of range | W011 |
| **Bit 12** | UPS short | E012 | rsvd | W012 |
| **Bit 13** | Reverse current on UPS output | E013 | rsvd | W013 |
| **Bit 14** | BUS short | E014 | rsvd | W014 |
| **Bit 15** | Grid phases inconsistent in 3phase paralleling system | E015 | Battery reverse connection | W015 |
| **Bit 16** | Relay Check Fault | E016 | Grid power outage | W016 |
| **Bit 17** | Internal communication fault 2 | E017 | Grid voltage out of range | W017 |
| **Bit 18** | Internal communication fault 3 | E018 | Grid frequency out of range | W018 |
| **Bit 19** | BUS Voltage high | E019 | rsvd | W019 |
| **Bit 20** | EPS connection fault | E020 | PV insulation low | W020 |
| **Bit 21** | PV Voltage high | E021 | Leakage current high | W021 |
| **Bit 22** | Over current protection | E022 | DCI high | W022 |
| **Bit 23** | Neutral fault | E023 | PV short | W023 |
| **Bit 24** | PV short | E024 | rsvd | W024 |
| **Bit 25** | Radiator temperature out of range | E025 | Battery voltage high | W025 |
| **Bit 26** | Internal Fault | E026 | Battery voltage low | W026 |
| **Bit 27** | Sample inconsistent between Main CPU and redundant CPU | E027 | Battery open circuit | W027 |
| **Bit 28** | rsvd | E028 | EPS overload | W028 |
| **Bit 29** | rsvd | E029 | EPS voltage high | W029 |
| **Bit 30** | rsvd | E030 | Meter reverse connection | W030 |
| **Bit 31** | Internal communication fault 4 | E031 | DCV high | W031 |

**Note**: 
- Fault codes (E000-E031) indicate serious system issues that require immediate attention
- Warning codes (W000-W031) indicate potential issues that should be monitored
- "rsvd" indicates reserved bits that are not currently used
- Multiple faults/warnings can be active simultaneously (bitwise OR operation)

## System State & PV Monitoring (Registers 0-9)
- **Register 0**: State - see operating mode definition table above
- **Register 1**: PV String 1 Voltage (0.1V)
- **Register 2**: PV String 2 Voltage (0.1V)
- **Register 3**: PV String 3 Voltage (0.1V)
- **Register 4**: Battery Voltage (0.1V)
- **Register 5**: Battery State of Charge (SOC) and State of Health (SOH) - Combined data
- **Register 6**: Internal Fault - see fault code definition table above
- **Register 7**: PV1 Power (W)
- **Register 8**: PV2 Power (W)
- **Register 9**: PV3 Power (W)

## Power & Grid Monitoring (Registers 10-19)
- **Register 10**: Charging Power (W) - Charging power (incoming battery power)
- **Register 11**: Discharge Power (W) - Discharge power (outflow battery power)
- **Register 12**: R-phase Mains Voltage (0.1V) - R-phase mains voltage
- **Register 13**: S-phase Mains Voltage (0.1V) - S-phase mains voltage
- **Register 14**: T-phase Mains Voltage (0.1V) - T-phase mains voltage
- **Register 15**: Mains Frequency (0.01Hz) - Mains frequency
- **Register 16**: Inverter Output Power (W) - Inverter output power (Grid port)
- **Register 17**: AC Charging Rectified Power (W) - AC charging rectified power
- **Register 18**: Inverter Current RMS (0.01A) - Inverter current RMS (signed)
- **Register 19**: Power Factor (0.001) - PF x€(0,1000]->x/1000 x€(1000,2000]->(1000-x)/1000 (signed)

## Off-Grid System (Registers 20-28)
- **Register 20**: R-phase Off-grid Output Voltage (0.1V) - R-phase off-grid output voltage
- **Register 21**: S-phase Off-grid Output Voltage (0.1V) - S-phase off-grid output voltage
- **Register 22**: T-phase Off-grid Output Voltage (0.1V) - T-phase off-grid output voltage
- **Register 23**: Off-grid Output Frequency (0.01Hz) - Off-grid output frequency
- **Register 24**: Off-grid Inverter Power (W) - Off-grid inverter power
- **Register 25**: Off-grid Apparent Power (VA) - Off-grid apparent power
- **Register 26**: Export Power to Grid (W) - Export power to grid
- **Register 27**: Import Power from Grid (W) - Import power from grid
- **Register 28**: PV1 Power Generation Today (0.1kWh) - PV1 power generation today

## Daily Energy Production (Registers 29-37)
- **Register 29**: PV2 Power Generation Today (0.1kWh) - PV2 power generation today
- **Register 30**: PV3 Power Generation Today (0.1kWh) - PV3 power generation today
- **Register 31**: Grid-Connected Inverter Output Energy Today (0.1kWh) - Today's grid-connected inverter output energy
- **Register 32**: AC Charging Rectified Energy Today (0.1kWh) - Today's AC charging rectified energy
- **Register 33**: Charged Energy Today (0.1kWh) - Charged energy today
- **Register 34**: Discharged Energy Today (0.1kWh) - Discharged energy today
- **Register 35**: Off-Grid Output Energy Today (0.1kWh) - Off-grid output energy today
- **Register 36**: Export Energy to Grid Today (0.1kWh) - Today's export energy to grid
- **Register 37**: Import Energy from Grid Today (0.1kWh) - Today's import energy from grid

## Bus Voltages (Registers 38-39)
- **Register 38**: Bus 1 Voltage (0.1V) - Bus 1 Voltage
- **Register 39**: Bus 2 Voltage (0.1V) - Bus 2 Voltage

## Cumulative Energy Production - Low/High Word Pairs (Registers 40-55)
- **Register 40**: PV1 Cumulative Power Generation Low Word (0.1kWh) - PV1 cumulative power generation low word
- **Register 41**: PV1 Cumulative Power Generation High Word (0.1kWh) - PV1 cumulative power generation high word
- **Register 42**: PV2 Cumulative Power Generation Low Word (0.1kWh) - PV2 cumulative power generation low word
- **Register 43**: PV2 Cumulative Power Generation High Word (0.1kWh) - PV2 cumulative power generation high word
- **Register 44**: PV3 Cumulative Power Generation Low Word (0.1kWh) - Low word of PV3 cumulative power generation
- **Register 45**: PV3 Cumulative Power Generation High Word (0.1kWh) - PV3 cumulative power generation high word
- **Register 46**: Inverter Cumulative Output Energy Low Word (0.1kWh) - Inverter accumulative output energy low word
- **Register 47**: Inverter Cumulative Output Energy High Word (0.1kWh) - Inverter accumulative output energy High word
- **Register 48**: AC Charging Cumulative Rectified Energy Low Word (0.1kWh) - AC charging accumulative rectified energy low word
- **Register 49**: AC Charging Cumulative Rectified Energy High Word (0.1kWh) - AC charging accumulative rectified energy High word
- **Register 50**: Cumulative Charge Energy Low Word (0.1kWh) - Cumulative charge energy level low word
- **Register 51**: Cumulative Charge Energy High Word (0.1kWh) - Cumulative charge energy High word
- **Register 52**: Cumulative Discharge Energy Low Word (0.1kWh) - Cumulative discharge energy low word
- **Register 53**: Cumulative Discharge Energy High Word (0.1kWh) - Cumulative discharge energy High word
- **Register 54**: Cumulative Off-Grid Inverter Power Low Word (0.1kWh) - Cumulative off-grid inverter power Low word
- **Register 55**: Cumulative Off-Grid Inverter Power High Word (0.1kWh) - Cumulative off-grid inverter power High word

## Final Cumulative Energy Tracking (Registers 56-59)
- **Register 56**: Cumulative Export Energy to Grid Low Word (0.1kWh) - Cumulative export energy to grid low word
- **Register 57**: Cumulative Export Energy to Grid High Word (0.1kWh) - Cumulative export energy to grid High word
- **Register 58**: Cumulative Import Energy from Grid Low Word (0.1kWh) - Cumulative import energy from grid low word
- **Register 59**: Cumulative Import Energy from Grid High Word (0.1kWh) - Cumulative import energy from grid High word

## System Status & Diagnostics (Registers 60-63)
- **Register 60**: Fault Code Low Word - Check fault code definition table above
- **Register 61**: Fault Code High Word - Check fault code definition table above
- **Register 62**: Warning Code Low Word - Check warning code definition table above
- **Register 63**: Warning Code High Word - Check warning code definition table above

## Temperature Monitoring (Registers 64-67) Signed Number
- **Register 64**: Internal Ring Temperature (°C) - Internal ring temperature
- **Register 65**: Radiator Temperature 1 (°C) - Radiator temperature 1
- **Register 66**: Radiator Temperature 2 (°C) - Radiator temperature 2
- **Register 67**: Battery Temperature (°C) - Battery temperature

## System Runtime & Auto-Test (Registers 68-75)
- **Register 68**: Reserved/Unused
- **Register 69**: Running Time Low Word (Seconds) - Runtime low word
- **Register 70**: Running Time High Word (Seconds) - Runtime high word
- **Register 71**: Auto-Test Configuration Register - Auto-test configuration and status
- **Register 72**: Auto-Test Limit (0.1V/0.01Hz) - If ubAutoTestStep=1,2,5,6, Voltage limit; If ubAutoTestStep=3,4,7,8, Frequency limit
- **Register 73**: Auto-Test Default Time (ms) - Auto-test default timing
- **Register 74**: Auto-Test Trip Value (0.1V/0.01Hz) - If ubAutoTestStep=1,2,5,6, Voltage limit; If ubAutoTestStep=3,4,7,8, Frequency limit
- **Register 75**: Auto-Test Trip Time (ms) - Auto-test trip timing

## Auto-Test Status & Configuration (Register 71 Bit-fields)
- **Register 71 Bit0-3**: Auto-Test Start - 0-Not activated, 1-Activated
- **Register 71 Bit4-7**: Auto-Test Status - 0-waiting, 1-testing, 2-test fail, 3-V test OK, 4-F test OK, 5-test pass
- **Register 71 Bit8-11**: Auto-Test Step - 1-V1L test, 2-V1H, 3-F1L test, 4-F1H test, 5-V2L test, 6-V2H test, 7-F2L test, 8-F2H test

## AC Input Configuration (Register 77)
- **Register 76**: Reserved/Unused
- **Register 77**: AC Input Type (0 or 1) - 0-Grid, 1-Generator for 12KHybrid

## Reserved/Unused (Registers 78-80)
- **Register 78**: Reserved/Unused
- **Register 79**: Reserved/Unused
- **Register 80**: Reserved/Unused

## Battery Management System (BMS) Parameters (Registers 81-100)
- **Register 81**: BMS Max Charging Current (0.01A) - BMS limited maximum charging current
- **Register 82**: BMS Max Discharge Current (0.01A) - BMS limited maximum discharge current
- **Register 83**: BMS Charge Voltage Reference (0.1V) - BMS recommended charging voltage
- **Register 84**: BMS Discharge Cut-off Voltage (0.1V) - BMS recommends discharge cut-off voltage
- **Register 85**: BMS Battery Status 0 - BMS status information
- **Register 86**: BMS Battery Status 1 - BMS status information
- **Register 87**: BMS Battery Status 2 - BMS status information
- **Register 88**: BMS Battery Status 3 - BMS status information
- **Register 89**: BMS Battery Status 4 - BMS status information
- **Register 90**: BMS Battery Status 5 - BMS status information
- **Register 91**: BMS Battery Status 6 - BMS status information
- **Register 92**: BMS Battery Status 7 - BMS status information
- **Register 93**: BMS Battery Status 8 - BMS status information
- **Register 94**: BMS Battery Status 9 - BMS status information
- **Register 95**: Inverter Battery Status Summary - Inverter summarizes lithium battery status information
- **Register 96**: Battery Parallel Number - Number of batteries in parallel
- **Register 97**: Battery Capacity (Ah) - Battery capacity
- **Register 98**: BMS Battery Current (0.01A) - Battery current, signed
- **Register 99**: BMS Fault Code - BMS fault code information
- **Register 100**: BMS Warning Code - BMS warning code information

## BMS Cell Monitoring & Firmware (Registers 101-106)
- **Register 101**: BMS Max Cell Voltage (0.001V) - Maximum cell voltage
- **Register 102**: BMS Min Cell Voltage (0.001V) - Minimum cell voltage
- **Register 103**: BMS Max Cell Temperature (0.1°C) - Maximum monomer temperature, signed number
- **Register 104**: BMS Min Cell Temperature (0.1°C) - Minimum monomer temperature, signed number
- **Register 105**: BMS Firmware Update State (1-3) - 1-Upgrading, 2-Upgrading successful, 3-Upgrading failed
- **Register 106**: BMS Cycle Count - Number of charge and discharge cycles

## Inverter Battery & Temperature Sensors (Registers 107-112)
- **Register 107**: Inverter Battery Voltage Sample (0.1V) - Inverter battery voltage sampling
- **Register 108**: Temperature Sensor 1 (0.1°C) - 12K BT temperature
- **Register 109**: Temperature Sensor 2 (0.1°C) - Reserved
- **Register 110**: Temperature Sensor 3 (0.1°C) - Reserved
- **Register 111**: Temperature Sensor 4 (0.1°C) - Reserved
- **Register 112**: Temperature Sensor 5 (0.1°C) - Reserved

## System Configuration Register (Register 113)
- **Register 113 Bit0~1**: Master/Slave Configuration (1,2) - 1:master, 2:slave
- **Register 113 Bit2~3**: Single/Three Phase (1-3) - Phase 1:R, 2:S, 3:T
- **Register 113 Bit4~7**: Reserved
- **Register 113 Bit8~16**: Parallel Machine Number (1~255) - Number of parallel machines

## Reserved/Unused (Registers 114-119)
- **Register 114**: Reserved/Unused
- **Register 115**: Reserved/Unused
- **Register 116**: Reserved/Unused
- **Register 117**: Reserved/Unused
- **Register 118**: Reserved/Unused
- **Register 119**: Reserved/Unused

## Generator & EPS Parameters (Registers 120-131)
- **Register 120**: Half BUS Voltage (0.1V) - Half BUS voltage
- **Register 121**: Generator Voltage (0.1V) - Generator voltage
- **Register 122**: Generator Frequency (0.01Hz) - Generator frequency
- **Register 123**: Generator Power (W) - Generator power
- **Register 124**: Generator Daily Energy (0.1kWh) - Daily energy of generator
- **Register 125**: Generator Total Energy Low Word (0.1kWh) - Low word of total generator energy
- **Register 126**: Generator Total Energy High Word (0.1kWh) - High word of total generator energy
- **Register 127**: EPS Voltage L1N (0.1V) - Voltage of EPS L1N
- **Register 128**: EPS Voltage L2N (0.1V) - Voltage of EPS L2N
- **Register 129**: EPS Active Power L1N (W) - Active power of EPS L1N
- **Register 130**: EPS Active Power L2N (W) - Active power of EPS L2N
- **Register 131**: EPS Apparent Power L1N (VA) - Apparent power of EPS L1N

## EPS Energy Tracking (Registers 132-138)
- **Register 132**: EPS Apparent Power L2N (VA) - Apparent power of EPS L2N
- **Register 133**: EPS L1N Daily Energy (0.1kWh) - Daily energy of EPSL1N
- **Register 134**: EPS L2N Daily Energy (0.1kWh) - Daily energy of EPSL2N
- **Register 135**: EPS L1N Total Energy Low Word (0.1kWh) - Low word of total EPSL1N energy
- **Register 136**: EPS L1N Total Energy High Word (0.1kWh) - High word of total EPSL1N energy
- **Register 137**: EPS L2N Total Energy Low Word (0.1kWh) - Low word of total EPSL2N energy
- **Register 138**: EPS L2N Total Energy High Word (0.1kWh) - High word of total EPSL2N energy

## Reserved/Unused (Register 139)
- **Register 139**: Reserved/Unused

## AFCI (Arc Fault Circuit Interrupter) Monitoring (Registers 140-152)
- **Register 140**: AFCI Current Channel 1 (mA) - AFCI current
- **Register 141**: AFCI Current Channel 2 (mA) - AFCI current
- **Register 142**: AFCI Current Channel 3 (mA) - AFCI current
- **Register 143**: AFCI Current Channel 4 (mA) - AFCI current

### AFCI Status & Alarms (Register 144 Bit-fields)
- **Register 144 Bit0**: AFCI Arc Alarm Channel 1 - Arc status of CH1 0-Normal 1-Alarm
- **Register 144 Bit1**: AFCI Arc Alarm Channel 2 - Arc status of CH2 0-Normal 1-Alarm
- **Register 144 Bit2**: AFCI Arc Alarm Channel 3 - Arc status of CH3 0-Normal 1-Alarm
- **Register 144 Bit3**: AFCI Arc Alarm Channel 4 - Arc status of CH4 0-Normal 1-Alarm
- **Register 144 Bit4**: AFCI Self-Test Result Channel 1 - Test result of CH1 0-Normal 1-fail
- **Register 144 Bit5**: AFCI Self-Test Result Channel 2 - Test result of CH2 0-Normal 1-fail
- **Register 144 Bit6**: AFCI Self-Test Result Channel 3 - Test result of CH3 0-Normal 1-fail
- **Register 144 Bit7**: AFCI Self-Test Result Channel 4 - Test result of CH4 0-Normal 1-fail
- **Register 144 Bit8-15**: AFCI ArcAlarm Reserved

### AFCI Real-Time & Maximum Arc Values (Registers 145-152)
- **Register 145**: AFCI Real-Time Arc Channel 1 - Real time arc of CH1
- **Register 146**: AFCI Real-Time Arc Channel 2 - Real time arc of CH2
- **Register 147**: AFCI Real-Time Arc Channel 3 - Real time arc of CH3
- **Register 148**: AFCI Real-Time Arc Channel 4 - Real time arc of CH4
- **Register 149**: AFCI Maximum Arc Channel 1 - Max arc of CH1
- **Register 150**: AFCI Maximum Arc Channel 2 - Max arc of CH2
- **Register 151**: AFCI Maximum Arc Channel 3 - Max arc of CH3
- **Register 152**: AFCI Maximum Arc Channel 4 - Max arc of CH4

## Implementation Notes
- All multi-word registers use little-endian byte order
- Temperature values may be negative (two's complement)
- Status bits are active high (1 = active, 0 = inactive)
- Fault and warning codes use bitwise OR for multi-word registers
- AFCI system provides critical safety monitoring for arc fault detection
- BMS integration provides comprehensive battery health monitoring
