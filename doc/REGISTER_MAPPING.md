# EG4 18KPV Modbus Register Mapping

This document provides a comprehensive mapping of all Modbus registers used by the EG4 18KPV inverter, based on the LuxPower Modbus protocol specification.

This is a homeassistant plugin that uses rust to connect to an eg4 18kpv inverter and manage its settings remotely/monitor its status via homeassistant/mqtt.
Use the documents available below to validate the design and search for bugs.  
modbus protoocol: https://www.dth.net/solar/luxpower/modbus/EG4-18KPV-12LV-Modbus-Protocol.pdf
Analyze @https://github.com/jaredmauch/eg4-bridge/tree/main/src  for any fixes.  This code in my editor  is a fork from the same code base that this git repo above is from.

## Input Registers (Read-Only)

### System Status (Register 0)
- **Function**: System operating status
- **Data Type**: 16-bit unsigned integer
- **Bit Mapping**:
  - Bit 0: Standby
  - Bit 1: Grid Connected
  - Bit 2: Grid Connected + Battery Charging
  - Bit 3: Grid Connected + Battery Discharging
  - Bit 4: Grid Connected + PV Generation
  - Bit 5: Grid Connected + PV Generation + Battery Charging
  - Bit 6: Grid Connected + PV Generation + Battery Discharging
  - Bit 7: EPS Mode
  - Bit 8: EPS Mode + Battery Charging
  - Bit 9: EPS Mode + Battery Discharging
  - Bit 10: EPS Mode + PV Generation
  - Bit 11: EPS Mode + PV Generation + Battery Charging
  - Bit 12: EPS Mode + PV Generation + Battery Discharging
  - Bit 13: Fault Mode
  - Bit 14: Maintenance Mode
  - Bit 15: Test Mode

### PV System (Registers 1-19)
- **Register 1**: PV String 1 Voltage (0.1V)
- **Register 2**: PV String 2 Voltage (0.1V)
- **Register 3**: PV String 3 Voltage (0.1V)
- **Register 4**: Battery Voltage (0.1V)
- **Register 5**: Battery State of Charge (%)
- **Register 6**: Battery State of Health (%)
- **Register 7**: Internal Fault Code
- **Register 8**: PV Array Power (W)
- **Register 9**: PV String 1 Power (W)
- **Register 10**: PV String 2 Power (W)
- **Register 11**: PV String 3 Power (W)
- **Register 12**: Battery Power (W, negative = discharging)
- **Register 13**: Battery Charge Power (W)
- **Register 14**: Battery Discharge Power (W)

### Grid Connection (Registers 20-39)
- **Register 20**: Grid Voltage R (0.1V)
- **Register 21**: Grid Voltage S (0.1V)
- **Register 22**: Grid Voltage T (0.1V)
- **Register 23**: Grid Frequency (0.01Hz)
- **Register 24**: Inverter Power (W)
- **Register 25**: Rectifier Power (W)
- **Register 26**: Power Factor (0.001)
- **Register 27**: Grid Power (W, negative = export)
- **Register 28**: Power to Grid (W)
- **Register 29**: Power to User (W)

### EPS System (Registers 40-59)
- **Register 40**: EPS Voltage R (0.1V)
- **Register 41**: EPS Voltage S (0.1V)
- **Register 42**: EPS Voltage T (0.1V)
- **Register 43**: EPS Frequency (0.01Hz)
- **Register 44**: EPS Power (W)
- **Register 45**: EPS Apparent Power (VA)

### Energy Counters (Registers 60-99)
- **Register 60**: PV Energy Today (0.1kWh)
- **Register 61**: PV Energy Today String 1 (0.1kWh)
- **Register 62**: PV Energy Today String 2 (0.1kWh)
- **Register 63**: PV Energy Today String 3 (0.1kWh)
- **Register 64**: Inverter Energy Today (0.1kWh)
- **Register 65**: Rectifier Energy Today (0.1kWh)
- **Register 66**: Battery Charge Energy Today (0.1kWh)
- **Register 67**: Battery Discharge Energy Today (0.1kWh)
- **Register 68**: EPS Energy Today (0.1kWh)
- **Register 69**: Energy to Grid Today (0.1kWh)
- **Register 70**: Energy to User Today (0.1kWh)
- **Register 71**: Bus Voltage 1 (0.1V)
- **Register 72**: Bus Voltage 2 (0.1V)

### Total Energy Counters (Registers 100-139)
- **Register 100**: PV Energy Total (0.1kWh)
- **Register 101**: PV Energy Total String 1 (0.1kWh)
- **Register 102**: PV Energy Total String 2 (0.1kWh)
- **Register 103**: PV Energy Total String 3 (0.1kWh)
- **Register 104**: Inverter Energy Total (0.1kWh)
- **Register 105**: Rectifier Energy Total (0.1kWh)
- **Register 106**: Battery Charge Energy Total (0.1kWh)
- **Register 107**: Battery Discharge Energy Total (0.1kWh)
- **Register 108**: EPS Energy Total (0.1kWh)
- **Register 109**: Energy to Grid Total (0.1kWh)
- **Register 110**: Energy to User Total (0.1kWh)

### Fault and Warning Codes (Registers 140-159)
- **Register 140**: Fault Code Low Word
- **Register 141**: Fault Code High Word
- **Register 142**: Warning Code Low Word
- **Register 143**: Warning Code High Word

### Temperature and Runtime (Registers 160-179)
- **Register 160**: Internal Temperature (0.1°C)
- **Register 161**: Radiator Temperature 1 (0.1°C)
- **Register 162**: Radiator Temperature 2 (0.1°C)
- **Register 163**: Battery Temperature (0.1°C)
- **Register 164**: Runtime (seconds)

### Battery Management (Registers 180-199)
- **Register 180**: Max Charge Current (0.01A)
- **Register 181**: Max Discharge Current (0.01A)
- **Register 182**: Charge Voltage Reference (0.1V)
- **Register 183**: Discharge Cut-off Voltage (0.1V)
- **Register 184**: Battery Status 0
- **Register 185**: Battery Status 1
- **Register 186**: Battery Status 2
- **Register 187**: Battery Status 3
- **Register 188**: Battery Status 4
- **Register 189**: Battery Status 5
- **Register 190**: Battery Status 6
- **Register 191**: Battery Status 7
- **Register 192**: Battery Status 8
- **Register 193**: Battery Status 9
- **Register 194**: Battery Status Inverter
- **Register 195**: Battery Count
- **Register 196**: Battery Capacity (0.01Ah)
- **Register 197**: Battery Current (0.01A)

### BMS Events and Cell Monitoring (Registers 200-219)
- **Register 200**: BMS Event 1 (Fault Code BMS)
- **Register 201**: BMS Event 2 (Warning Code BMS)
- **Register 202**: Max Cell Voltage (0.001V)
- **Register 203**: Min Cell Voltage (0.001V)
- **Register 204**: Max Cell Temperature (0.1°C)
- **Register 205**: Min Cell Temperature (0.1°C)
- **Register 206**: BMS Firmware Update State
- **Register 207**: Cycle Count
- **Register 208**: Battery Voltage Inverter (0.1V)

## Holding Registers (Read/Write)

### System Configuration (Registers 0-19)
- **Register 0**: Model Information
- **Register 2**: Serial Number (spans registers 2-6)
- **Register 7**: Firmware Version Code
- **Register 12**: System Time Month/Year
- **Register 13**: System Time Hour/Day
- **Register 14**: System Time Second/Minute
- **Register 15**: Communication Address
- **Register 16**: System Language
- **Register 19**: Device Type

### PV Configuration (Registers 20-39)
- **Register 20**: PV Input Mode
- **Register 21**: Function Enable/Disable Flags
- **Register 22**: PV Voltage Threshold for Startup
- **Register 23**: Grid Connection Time
- **Register 24**: Grid Reconnection Time Delay

### Grid Protection (Registers 25-40)
- **Register 25**: Grid Voltage Connection Low Limit
- **Register 26**: Grid Voltage Connection High Limit
- **Register 27**: Grid Frequency Connection Low Limit
- **Register 28**: Grid Frequency Connection High Limit
- **Register 29**: Grid Voltage Protection Level 1 Low
- **Register 30**: Grid Voltage Protection Level 1 High
- **Register 31**: Grid Voltage Protection Level 1 Time
- **Register 32**: Grid Voltage Protection Level 2 Low
- **Register 33**: Grid Voltage Protection Level 2 High
- **Register 34**: Grid Voltage Protection Level 2 Time
- **Register 35**: Grid Frequency Protection Level 1 Low
- **Register 36**: Grid Frequency Protection Level 1 High
- **Register 37**: Grid Frequency Protection Level 1 Time
- **Register 38**: Grid Frequency Protection Level 2 Low
- **Register 39**: Grid Frequency Protection Level 2 High
- **Register 40**: Grid Frequency Protection Level 2 Time

### Power Control (Registers 64-83)
- **Register 64**: System Charge Rate (%)
- **Register 65**: System Discharge Rate (%)
- **Register 66**: Grid Charge Power Rate (%)
- **Register 67**: AC Charge SOC Limit (%)
- **Register 74**: Charge Priority Charge Rate (%)
- **Register 75**: Charge Priority SOC Limit (%)
- **Register 83**: Forced Discharge SOC Limit (%)
- **Register 105**: Discharge Cut-off SOC (%)
- **Register 125**: EPS Discharge Cut-off SOC (%)
- **Register 160**: AC Charge Start SOC Limit (%)
- **Register 161**: AC Charge End SOC Limit (%)

### Frequency Response (Registers 97-136)
- **Register 97**: Delay Time for Over Frequency Derate (ms)
- **Register 115**: Over Frequency Droop Start (0.01Hz)
- **Register 124**: Over Frequency Droop End (0.01Hz)
- **Register 136**: Over Frequency Droop Rate (%/Hz)
- **Register 134**: Under Frequency Droop Start (0.01Hz)
- **Register 135**: Under Frequency Droop End (0.01Hz)
- **Register 193**: Under Frequency Increase Rate (%/Hz)

### Auto Test (Registers 171-175)
- **Register 171**: Auto Test Status
- **Register 172**: Auto Test Limit
- **Register 173**: Auto Test Default Time (ms)
- **Register 174**: Auto Test Trip Value
- **Register 175**: Auto Test Trip Time (ms)

## Register 21 Bit Definitions

### Function Enable/Disable Flags
- **Bit 0**: EPS Enable
- **Bit 1**: Over Frequency Load Derate Enable
- **Bit 2**: DRMS Enable
- **Bit 3**: Low Voltage Ride Through Enable
- **Bit 4**: Anti-Islanding Enable
- **Bit 5**: Neutral Detection Enable
- **Bit 6**: Grid On Power Soft Start Enable
- **Bit 7**: AC Charge Enable
- **Bit 8**: Switch Seamless Enable
- **Bit 9**: Set to Standby Enable
- **Bit 10**: Forced Discharge Enable
- **Bit 11**: Charge Priority Enable
- **Bit 12**: ISO Enable
- **Bit 13**: GFCI Enable
- **Bit 14**: DCI Enable
- **Bit 15**: Feed In Grid Enable

## Register 110 Bit Definitions

### Advanced Function Enable/Disable
- **Bit 0**: PV Off Grid Enable
- **Bit 1**: Fast Zero Export Enable
- **Bit 2**: Micro Grid Enable
- **Bit 3**: Shared Battery Enable
- **Bit 4**: Charge Last Enable

## Data Validation

### Voltage Ranges
- **PV Voltage**: 0-1000V (0.1V resolution)
- **Battery Voltage**: 0-100V (0.1V resolution)
- **Grid Voltage**: 0-300V (0.1V resolution)
- **EPS Voltage**: 0-300V (0.1V resolution)

### Frequency Ranges
- **Grid Frequency**: 45-65Hz (0.01Hz resolution)
- **EPS Frequency**: 45-65Hz (0.01Hz resolution)

### Temperature Ranges
- **Internal Temperature**: -40°C to +85°C (0.1°C resolution)
- **Radiator Temperature**: -40°C to +85°C (0.1°C resolution)
- **Battery Temperature**: -40°C to +85°C (0.1°C resolution)
- **Cell Temperature**: -40°C to +85°C (0.1°C resolution)

### Power Ranges
- **Inverter Power**: 0-18000W (1W resolution)
- **PV Power**: 0-18000W (1W resolution)
- **Battery Power**: -18000W to +18000W (1W resolution)

### Energy Ranges
- **Daily Energy**: 0-999.9kWh (0.1kWh resolution)
- **Total Energy**: 0-999999.9kWh (0.1kWh resolution)

## Error Handling

### Fault Code Categories
1. **Communication Faults** (Bits 0-7)
2. **System Faults** (Bits 8-15)
3. **Protection Faults** (Bits 16-23)
4. **Hardware Faults** (Bits 24-31)

### Warning Code Categories
1. **Communication Warnings** (Bits 0-7)
2. **System Warnings** (Bits 8-15)
3. **Protection Warnings** (Bits 16-23)
4. **Performance Warnings** (Bits 24-31)

### BMS Event Categories
1. **Battery Protection Events** (Bits 0-7)
2. **Battery Health Events** (Bits 8-15)

## Implementation Notes

- All multi-word registers use little-endian byte order
- Temperature values may be negative (two's complement)
- Power values may be negative (two's complement for battery discharge)
- Energy values are always positive and cumulative
- Status bits are active high (1 = active, 0 = inactive)
- Fault and warning codes use bitwise OR for multi-word registers