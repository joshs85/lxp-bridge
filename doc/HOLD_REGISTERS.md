# LXP Inverter Hold Registers (Read/Write)

## Overview
This document describes all hold registers (read/write) available on LXP inverters. These registers allow configuration, control, and parameter setting of the inverter system.

**Note**: Hold registers support function codes 0x03 (Read Hold Registers), 0x06 (Write Single Hold Register), and 0x10 (Write Multiple Hold Registers).

## Serial Number Registers (Registers 2-6)
These registers store the system serial number components as ASCII codes.

### Register Details
- **Register 2**: SN[0]-Year and SN[1]-Week
  - **Range**: '0'-'9', 'A'-'Z' (ASCII codes)
  - **Note**: SN[0] = 0x41(A), SN[1] = week identifier
  
- **Register 3**: SN[2]-Week and SN[3]-Factory
  - **Range**: '0'-'9', 'A'-'Z' (ASCII codes)
  - **Note**: SN[2] = week identifier, SN[3] = factory identifier
  
- **Register 4**: SN[4]-Product Code and SN[5]-Product Code
  - **Range**: '0'-'9', 'A'-'Z' (ASCII codes)
  - **Note**: SN[4] and SN[5] = product code identifiers
  
- **Register 5**: SN[6]-Product Code and SN[7]-Batch Number
  - **Range**: '0'-'9', 'A'-'Z' (ASCII codes)
  - **Note**: SN[6] = product code, SN[7] = batch number
  
- **Register 6**: SN[8]-Batch Number and SN[9]-Batch Number
  - **Range**: '0'-'9', 'A'-'Z' (ASCII codes)
  - **Note**: SN[8] = batch number, SN[9] = 0x38(8)

**Serial Number Format**: The complete serial number consists of 10 bytes with ASCII codes. Example: AB12345678 where SN[0]=0x41(A), SN[9]=0x38(8)

## Firmware Version Registers (Registers 9-10)
These registers store firmware version information for different CPU components.

### Register Details
- **Register 9**: Slave Ver and Com Ver
  - **Slave Ver**: Redundant CPU FW version (0-255)
  - **Com Ver**: Communication CPU FW version (0-255)
  
- **Register 10**: Cntl Ver and FWVer
  - **Cntl Ver**: Control CPU FW version (0-255)
  - **FWVer**: FW Version code (0-255)

## Reset Settings Register (Register 11)
This register controls various system reset and configuration functions through individual bits.

### Bit Definitions
- **Bit 0**: ResetSetting.EnergyRecordClr
  - **Unit**: Bit0
  - **Range**: 0/1
  - **Function**: Power and running time reset
  
- **Bit 1**: ResetSetting.AlltoDefault
  - **Unit**: Bit1
  - **Range**: 0/1
  - **Function**: Restoring system settings to default values
  
- **Bit 2**: ResetSetting.AdjRatioClr
  - **Unit**: Bit2
  - **Range**: 0/1
  - **Function**: Correction coefficient returns to default (1)
  
- **Bit 3**: ResetSetting.FaultRecordClr
  - **Unit**: Bit3
  - **Range**: 0/1
  - **Function**: Clear fault record
  
- **Bit 4**: ResetSetting.MonitorData
  - **Unit**: Bit4
  - **Range**: 0/1
  - **Function**: Clear offline monitoring data records
  
- **Bit 5**: ResetSetting.BMSChgSWOn
  - **Unit**: Bit5
  - **Range**: 0/1
  - **Function**: 0-null, 1-turn on charge switch
  
- **Bit 6**: ResetSetting.BMSDischgSWOn
  - **Unit**: Bit6
  - **Range**: 0/1
  - **Function**: 0-null, 1-turn on discharge switch
  
- **Bit 7**: ResetSetting.InvReboot
  - **Unit**: Bit7
  - **Range**: 0/1
  - **Function**: 0-null, 1-restart inverter
  
- **Bit 8-15**: ResetSetting.Reserved

## Time and Communication Registers (Registers 12-16)
These registers control system time settings and communication parameters.

### Register Details
- **Register 12**: Time_Year and Time_Month
  - **Time_Year**: Range 17-255, Description: year
  - **Time_Month**: Range 1-12, Description: month
  
- **Register 13**: Time_Date and Time_Hour
  - **Time_Date**: Range 1-31, Description: day
  - **Time_Hour**: Range 0-23, Description: Time
  
- **Register 14**: Time_Minute and Time_Second
  - **Time_Minute**: Range 0-59, Description: Minute
  - **Time_Second**: Range 0-59, Description: Second
  
- **Register 15**: Com Addr
  - **Range**: 0-150
  - **Description**: Modbus communication address
  
- **Register 16**: Language
  - **Range**: 0-1
  - **Description**: Language 0-English 1-German

## PV Input Model Register (Register 20)
This register configures the PV input configuration and working mode.

### Register Details
- **General Range**: 0-4
- **12KHybrid Range**: 0-7
- **Configuration Options**:
  - **0**: No PV plug in
  - **1**: PV1 plug in
  - **2**: PV2 plug in
  - **3**: Two parallel PV
  - **4**: Two separate PV
  
- **12KHybrid Specific Configurations**:
  - **0**: No PV
  - **1**: PV1 in
  - **2**: PV2 in
  - **3**: PV3 in
  - **4**: PV1&2 in
  - **5**: PV1&3 in
  - **6**: PV2&3 in
  - **7**: PV1&2&3 in

## Function Enable Register (Register 21) - Detailed Bit Definitions
This register controls various system functions through individual bits with detailed descriptions.

### Bit Definitions
- **Bit 0**: FuncEn.EPSEn
  - **Range**: 0/1
  - **Description**: Off-grid mode enabled
  
- **Bit 1**: FuncEn.OVFLoadDerateEn
  - **Range**: 0/1
  - **Description**: Over frequency load reduction enable
  
- **Bit 2**: FuncEn.DRMSEn
  - **Range**: 0/1
  - **Description**: DRMS enabled
  
- **Bit 3**: FuncEn.LVRTEn
  - **Range**: 0/1
  - **Description**: Low Voltage Ride Through Enable
  
- **Bit 4**: FuncEn.AntiIslandEn
  - **Range**: 0/1
  - **Description**: Anti-islanding enable
  
- **Bit 5**: FuncEn.NeutralDetectEn
  - **Range**: 0/1
  - **Description**: Zero ground detection enable
  
- **Bit 6**: FuncEn.GridOnPowerSSEn
  - **Range**: 0/1
  - **Description**: Grid-connected power soft start enable
  
- **Bit 7**: FuncEn.ACChargeEn
  - **Range**: 0/1
  - **Description**: AC Charge Enable
  - **Current Value**: 1 (Enabled)
  - **Note**: AC charging functionality is currently enabled and active
  
- **Bit 8**: FuncEn.SWSeamlesslyEn
  - **Range**: 0/1
  - **Description**: Off-grid mode seamless switching enabled
  
- **Bit 9**: FuncEn.SetToStandby
  - **Range**: 0/1
  - **Description**: 0: Standby 1: Power on
  
- **Bit 10**: FuncEn.ForcedDischgEn
  - **Range**: 0/1
  - **Description**: Forced discharge enable
  
- **Bit 11**: FuncEn.ForcedChgEn
  - **Range**: 0/1
  - **Description**: Force charge enable
  
- **Bit 12**: FuncEn.ISOEn
  - **Range**: 0/1
  - **Description**: ISO enabled
  
- **Bit 13**: FuncEn.GFCIEn
  - **Range**: 0/1
  - **Description**: GFCI enabled
  
- **Bit 14**: FuncEn.DCIEn
  - **Range**: 0/1
  - **Description**: DCI enable
  
- **Bit 15**: FuncEn.FeedInGridEn
  - **Range**: 0/1
  - **Description**: 0-disable 1-enable

## PV Working Starting Voltage Register (Register 22)
This register sets the minimum voltage required for PV operation.

### Register Details
- **Register 22**: StartPVVolt
  - **Unit**: 0.1V
  - **Range**: 900-5000
  - **Description**: PV working starting voltage

## Grid Connection Timing Registers (Registers 23-24)
These registers control the timing parameters for grid connection and reconnection.

### Register Details
- **Register 23**: ConnectTime
  - **Unit**: s (seconds)
  - **Range**: 30-600
  - **Description**: Grid connection waiting time
  
- **Register 24**: ReconnectTime
  - **Unit**: s (seconds)
  - **Range**: 0-900
  - **Description**: Reconnection waiting time

## Grid Voltage Connection Limits (Registers 25-26)
These registers define the allowable grid voltage range for connection.

### Register Details
- **Register 25**: GridVoltConnLow
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: The lower limit of the allowable grid-connected mains voltage range
  
- **Register 26**: GridVoltConnHigh
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: The upper limit of the allowable grid-connected mains voltage range

## Grid Frequency Connection Limits (Registers 27-28)
These registers define the allowable grid frequency range for connection.

### Register Details
- **Register 27**: GridFreqConnLow
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: The lower limit of the allowable grid-connected mains frequency range
  
- **Register 28**: GridFreqConnHigh
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: The upper limit of the allowable grid-connected mains frequency range

## Grid Voltage Protection Level 1 (Registers 29-32)
These registers define the first level of grid voltage protection with timing.

### Register Details
- **Register 29**: GridVoltLimit1Low
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 1 undervoltage protection point
  
- **Register 30**: GridVoltLimit1High
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 1 overvoltage protection point
  
- **Register 31**: GridVoltLimit1LowTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 1 undervoltage protection time
  
- **Register 32**: GridVoltLimit1HighTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 1 overvoltage protection time

## Grid Voltage Protection Level 2 (Registers 33-35)
These registers define the second level of grid voltage protection with timing.

### Register Details
- **Register 33**: GridVoltLimit2Low
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 2 undervoltage protection point
  
- **Register 34**: GridVoltLimit2High
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 2 overvoltage protection point
  
- **Register 35**: GridVoltLimit2LowTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 2 undervoltage protection time

## Grid Voltage Protection Level 3 (Registers 37-40)
These registers define the third level of grid voltage protection with timing.

### Register Details
- **Register 37**: GridVoltLimit3Low
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 3 undervoltage protection point
  
- **Register 38**: GridVoltLimit3High
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 3 overvoltage protection point
  
- **Register 39**: GridVoltLimit3LowTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 3 undervoltage protection time
  
- **Register 40**: GridVoltLimit3HighTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage level 3 overvoltage protection time

## Grid Voltage Moving Average Protection (Register 36)
This register provides sliding average overvoltage protection.

### Register Details
- **Register 36**: GridVoltMovAvgHigh
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Grid voltage sliding average overvoltage protection point

## Grid Frequency Protection Level 1 (Registers 42-45)
These registers define the first level of grid frequency protection with timing.

### Register Details
- **Register 42**: GridFreqLimit1Low
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 1 underfrequency protection point
  
- **Register 43**: GridFreqLimit1High
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 1 overfrequency protection point
  
- **Register 44**: GridFreqLimit1LowTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 1 underfrequency protection time
  
- **Register 45**: GridFreqLimit1HighTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 1 overfrequency protection time

## Grid Frequency Protection Level 2 (Registers 46-49)
These registers define the second level of grid frequency protection with timing.

### Register Details
- **Register 46**: GridFreqLimit2Low
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency level 2 under-frequency protection point
  
- **Register 47**: GridFreqLimit2High
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 2 overfrequency protection point
  
- **Register 48**: GridFreqLimit2LowTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency level 2 under-frequency protection time
  
- **Register 49**: GridFreqLimit2HighTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 2 overfrequency protection time

## Grid Frequency Protection Level 3 (Registers 50-53)
These registers define the third level of grid frequency protection with timing.

### Register Details
- **Register 50**: GridFreqLimit3Low
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency level 3 under-frequency protection point
  
- **Register 51**: GridFreqLimit3High
  - **Unit**: 0.01Hz
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 3 overfrequency protection point
  
- **Register 52**: GridFreqLimit3LowTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency level 3 under-frequency protection time
  
- **Register 53**: GridFreqLimit3HighTime
  - **Unit**: Main period
  - **Range**: According to Grid regulation
  - **Description**: Grid frequency class 3 overfrequency protection time

## Reactive Power Control Parameters (Registers 54-58)
These registers control reactive power management and Q(V) curve settings.

### Register Details
- **Register 54**: MaxQPercentForQV
  - **Unit**: %
  - **Range**: According to Grid regulation
  - **Description**: Maximum reactive power percentage of Q(V) curve
  
- **Register 55**: V1L
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Q(V) curve undervoltage 1
  
- **Register 56**: V2L
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Q(V) curve undervoltage 2
  
- **Register 57**: V1H
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Q(V) curve overvoltage 1
  
- **Register 58**: V2H
  - **Unit**: 0.1V
  - **Range**: According to Grid regulation
  - **Description**: Q(V) curve overvoltage 2

## Reactive Power Command Type (Register 59)
This register defines the type of reactive power control to be used.

### Register Details
- **Register 59**: ReactivePowerCMDType
  - **Range**: 0-7
  - **Description**: Reactive command type with detailed options:
    - **0**: Unit power factor
    - **1**: Fixed PF
    - **2**: Default PF curve (American machine: Q(P))
    - **3**: Custom PF curve
    - **4**: Capacitive reactive power percentage
    - **5**: Inductive reactive power percentage
    - **6**: QV curve
    - **7**: QV_Dynamic

## Power Control Commands (Registers 60-62)
These registers control active power, reactive power, and power factor settings.

### Register Details
- **Register 60**: ActivePowerPercentCMD
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Active power percentage set value
  
- **Register 61**: ReactivePowerPercentCMD
  - **Unit**: %
  - **Range**: 0-60
  - **Description**: Reactive power percentage setting value
  
- **Register 62**: PFCMD
  - **Unit**: 0.001
  - **Range**: 750-1000, 1750-2000
  - **Description**: PF setting value, 750-1000(under), 1750-2000(over)

## AC Charging Configuration Overview
**Current Configuration Status**: AC charging is **ENABLED** and configured for **VOLTAGE-BASED** operation.

**Configuration Details**:
- **AC Charge Enable**: Register 21, Bit 7 = 1 (Enabled)
- **AC Charge Type**: Register 120, Bits 1-3 = 2 (According to voltage) ⚠️ **CONFIGURATION ISSUE**
- **Power Limit**: 12.0% of rated capacity (Register 66 = 120)
- **SOC Limit**: 90% maximum (Register 67 = 90)
- **Time Windows**: All three time windows are disabled (00:00-00:00)
- **Voltage Thresholds**: Start at 40.0V, Stop at 53.0V
- **SOC Thresholds**: Start at 85%, Stop at 0% (configuration error)

**Operation Mode**: Since time windows are disabled, AC charging operates purely based on voltage thresholds. Charging begins when battery voltage drops to 40.0V and stops when it reaches 53.0V.

**⚠️ Configuration Issues and Recommendations**:
1. **AC Charge Type Mismatch**: Currently set to voltage-based (2) but SOC thresholds are configured
   - **Current**: Register 120, Bits 1-3 = 2 (According to voltage)
   - **Recommended**: Register 120, Bits 1-3 = 3 (According to SOC)
   - **Reason**: SOC thresholds (Start: 85%, End: 0%) are configured but not being used

2. **SOC End Threshold Error**: Register 161 is set to 0% (should be 90% or higher)
   - **Current**: 0% (charging would stop at 0% SOC)
   - **Recommended**: 90% (to match Register 67 SOC limit)
   - **Reason**: Prevents overcharging and aligns with the configured SOC limit

3. **Time Windows Disabled**: All three time windows are set to 00:00-00:00
   - **Current**: Time-based charging is completely disabled
   - **Recommended**: Configure appropriate time windows or ensure voltage/SOC thresholds are correct
   - **Reason**: Provides backup charging control and flexibility

## Power Soft Start and Charging Control (Registers 63-67)
These registers control power ramp rates and basic charging parameters.

### Register Details
- **Register 63**: PowerSoftStartSlope
  - **Unit**: ‰/min (per mille per minute)
  - **Range**: 1-4000
  - **Description**: Loading rate, percent power increase per minute
  
- **Register 64**: ChargePowerPercentCMD
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Charging power percentage setting
  
- **Register 65**: DischgPowerPercentCMD
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Discharge power percentage setting
  
- **Register 66**: ACChgPowerCMD
  - **Unit**: 0.1%
  - **Range**: 0-120% (0-1200 in register units)
  - **Description**: AC charge power percentage setting
  - **Current Value**: 120 (12.0%)
  - **Note**: Located at `UL Compliance > Active Power-Reactive Power Mode > AC Charge Power (kW)` on EG4 monitoring installer website. GUI displays "kW" but stores percentage values in 0.1% units. The GUI has a maximum limit of 12.0kW (120%) and will fail with error code 3 if set higher. This register controls the maximum AC charging power as a percentage of the inverter's rated capacity.
  
- **Register 67**: ACChgSOCLimit
  - **Unit**: %
  - **Range**: 0-100%
  - **Description**: AC charging SOC limit setting
  - **Current Value**: 90%
  - **Note**: This register sets the maximum battery SOC at which AC charging will stop. When the battery reaches 90% SOC, AC charging will automatically terminate to prevent overcharging.

## AC Charging Time Schedule 1 (Registers 68-69)
These registers define the primary AC charging time window.

### Register Details
- **Register 68**: ACChgStartHour / ACChgStartMinute
  - **ACChgStartHour**: Range 0-23, Description: AC charging start time_hour setting
  - **ACChgStartMinute**: Range 0-59, Description: AC charging start time_minute setting
  - **Current Value**: 0 (00:00 - AC charging disabled for primary time window)
  
- **Register 69**: ACChgEndHour / ACChgEndMinute
  - **ACChgEndHour**: Range 0-23, Description: AC charging end time_hour setting
  - **ACChgEndMinute**: Range 0-59, Description: AC charging end time_min setting
  - **Current Value**: 0 (00:00 - AC charging disabled for primary time window)

## AC Charging Time Schedule 2 (Registers 70-71)
These registers define the secondary AC charging time window.

### Register Details
- **Register 70**: ACChgStartHour1 / ACChgStartMinute1
  - **ACChgStartHour1**: Range 0-23, Description: AC charging start time_hour setting
  - **ACChgStartMinute1**: Range 0-59, Description: AC charging start time_minute setting
  - **Current Value**: 0 (00:00 - AC charging disabled for secondary time window)
  
- **Register 71**: ACChgEndHour1 / ACChgEndMinute1
  - **ACChgEndHour1**: Range 0-23, Description: AC charging end time_hour setting
  - **ACChgEndMinute1**: Range 0-59, Description: AC charging end time_min setting
  - **Current Value**: 0 (00:00 - AC charging disabled for secondary time window)

## AC Charging Time Schedule 3 (Registers 72-73)
These registers define the tertiary AC charging time window.

### Register Details
- **Register 72**: ACChgStartHour2 / ACChgStartMinute2
  - **ACChgStartHour2**: Range 0-23, Description: AC charging start time_hour setting
  - **ACChgStartMinute2**: Range 0-59, Description: AC charging start time_minute setting
  - **Current Value**: 0 (00:00 - AC charging disabled for tertiary time window)
  
- **Register 73**: ACChgEndHour2 / ACChgEndMinute2
  - **ACChgEndHour2**: Range 0-23, Description: AC charging end time_hour setting
  - **ACChgEndMinute2**: Range 0-59, Description: AC charging end time_min setting
  - **Current Value**: 0 (00:00 - AC charging disabled for tertiary time window)

## Charging Priority Control (Registers 74-78)
These registers control priority charging parameters and time schedules.

### Register Details
- **Register 74**: ChgFirstPowerCMD
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Charging priority percentage setting
  
- **Register 75**: ChgFirstSOCLimit
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Charging priority SOC limit setting
  
- **Register 76**: ChgFirstStartHour / ChgFirstStartMinute
  - **ChgFirstStartHour**: Range 0-23, Description: Charging priority start time_hour setting
  - **ChgFirstStartMinute**: Range 0-59, Description: Charging priority start time_min setting
  
- **Register 77**: ChgFirstEndHour / ChgFirstEndMinute
  - **ChgFirstEndHour**: Range 0-23, Description: Charging priority end time_hour setting
  - **ChgFirstEndMinute**: Range 0-59, Description: Charging priority end time_min setting
  
- **Register 78**: ChgFirstStartHour1 / ChgFirstStartMinute1
  - **ChgFirstStartHour1**: Range 0-23, Description: Charging priority start time_hour setting
  - **ChgFirstStartMinute1**: Range 0-59, Description: Charging priority start time_min setting

## Charging Priority Time Settings Completion (Registers 79-81)
These registers complete the priority charging time schedule configuration.

### Register Details
- **Register 79**: ChgFirstEndHour1 / ChgFirstEndMinute1
  - **ChgFirstEndHour1**: Range 0-23, Description: Charging priority end time_hour setting
  - **ChgFirstEndMinute1**: Range 0-59, Description: Charging priority end time_min setting
  
- **Register 80**: ChgFirstStartHour2 / ChgFirstStartMinute2
  - **ChgFirstStartHour2**: Range 0-23, Description: Charging priority start time_hour setting
  - **ChgFirstStartMinute2**: Range 0-59, Description: Charging priority start time_min setting
  
- **Register 81**: ChgFirstEndHour2 / ChgFirstEndMinute2
  - **ChgFirstEndHour2**: Range 0-23, Description: Charging priority end time_hour setting
  - **ChgFirstEndMinute2**: Range 0-59, Description: Charging priority end time_min setting

## Forced Discharge Control (Registers 82-89)
These registers control forced discharge operations with multiple time schedules.

### Register Details
- **Register 82**: ForcedDischgPowerCMD
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Forced discharge percentage setting
  
- **Register 83**: ForcedDischgSOCLimit
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Forced discharge SOC limit setting

### Forced Discharge Time Schedule 1 (Registers 84-85)
- **Register 84**: ForcedDischgStartHour / ForcedDischgStartMinute
  - **ForcedDischgStartHour**: Range 0-23, Description: Forced discharge start time_hour setting
  - **ForcedDischgStartMinute**: Range 0-59, Description: Forced discharge start time_min setting
  
- **Register 85**: ForcedDischgEndHour / ForcedDischgEndMinute
  - **ForcedDischgEndHour**: Range 0-23, Description: Forced discharge end time_hour setting
  - **ForcedDischgEndMinute**: Range 0-59, Description: Forced discharge end time_min setting

### Forced Discharge Time Schedule 2 (Registers 86-87)
- **Register 86**: ForcedDischgStartHour1 / ForcedDischgStartMinute1
  - **ForcedDischgStartHour1**: Range 0-23, Description: Forced discharge start time_hour setting
  - **ForcedDischgStartMinute1**: Range 0-59, Description: Forced discharge start time_min setting
  
- **Register 87**: ForcedDischgEndHour1 / ForcedDischgEndMinute1
  - **ForcedDischgEndHour1**: Range 0-23, Description: Forced discharge end time_hour setting
  - **ForcedDischgEndMinute1**: Range 0-59, Description: Forced discharge end time_min setting

### Forced Discharge Time Schedule 3 (Registers 88-89)
- **Register 88**: ForcedDischgStartHour2 / ForcedDischgStartMinute2
  - **ForcedDischgStartHour2**: Range 0-23, Description: Forced discharge start time_hour setting
  - **ForcedDischgStartMinute2**: Range 0-59, Description: Forced discharge start time_min setting
  
- **Register 89**: ForcedDischgEndHour2 / ForcedDischgEndMinute2
  - **ForcedDischgEndHour2**: Range 0-23, Description: Forced discharge end time_hour setting
  - **ForcedDischgEndMinute2**: Range 0-59, Description: Forced discharge end time_min setting

## Off-Grid Output Settings (Registers 90-91)
These registers control the output parameters when operating in off-grid (EPS) mode.

### Register Details
- **Register 90**: EPSVoltageSet
  - **Unit**: 1V
  - **Range**: 230, 240, 277, 208
  - **Description**: Off-grid output voltage level setting
  
- **Register 91**: EPSFreqSet
  - **Unit**: 1Hz
  - **Range**: 50, 60
  - **Description**: Off-grid output frequency system settings

## Power Factor and Q(V) Curve Settings (Registers 92-94)
These registers control power factor curve lock-in/out voltages and Q(V) curve power thresholds.

### Register Details
- **Register 92**: LockInGridVForPFCurve
  - **Unit**: 0.1V
  - **Range**: 2300-3000
  - **Description**: cosphi(P) lock in voltage
  
- **Register 93**: LockOutGridVForPFCurve
  - **Unit**: 0.1V
  - **Range**: 1500-3000
  - **Description**: cosphi(P) lock out voltage
  
- **Register 94**: LockInPowerForQVCurve
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Q(V) lock in power

## Q(V) Curve and Frequency Derate Settings (Registers 95-97)
These registers control reactive power compensation based on voltage and frequency load reduction.

### Register Details
- **Register 95**: LockOutPowerForQVCurve
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Q(V) lock out power
  
- **Register 96**: DelayTimeForQVCurve
  - **Unit**: Main period
  - **Range**: 0-2000
  - **Description**: Q(V) Time delay
  
- **Register 97**: DelayTimeForOverFDerate
  - **Unit**: Main period
  - **Range**: 0-1000
  - **Description**: Over frequency load reduction delay

## Auto Test Start (Register 98)
This register controls the initiation of an auto-test function.

### Register Details
- **Register 98**: AutoTestStart
  - **Range**: 0/1
  - **Description**: 0-Stop 1-Start

## Battery Charging and Discharging Parameters (Registers 99-102)
These registers define voltage and current settings for lead-acid battery charging and discharging.

### Register Details
- **Register 99**: ChargeVoltRef
  - **Unit**: 0.1V
  - **Range**: 500-590
  - **Description**: Lead-acid battery charging given voltage
  
- **Register 100**: CutVoltForDischg
  - **Unit**: 0.1V
  - **Range**: 400-500
  - **Description**: Lead-acid battery discharge cut-off voltage
  
- **Register 101**: ChargeCurr
  - **Unit**: %A
  - **Range**: 0-140
  - **Description**: Recharging current
  
- **Register 102**: DischgCurr
  - **Unit**: %A
  - **Range**: 0-140
  - **Description**: Discharge current

## Grid Interaction / Max Backflow (Register 103)
This register sets the maximum power allowed to feed back into the grid.

### Register Details
- **Register 103**: MaxBackFlow
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Feed-in grid power set

## Discharge/Charge Switch Command (Register 104)
This register controls the switching between discharge and charge modes.

### Register Details
- **Register 104**: DischgChgSWCMD
  - **Range**: 0-2
  - **Description**:
    - 0: No action
    - 1: Discharge to charge
    - 2: Charge to discharge

## End of Discharge (EOD) SOC (Register 105)
This register sets the State of Charge (SOC) at which discharge is cut off.

### Register Details
- **Register 105**: EOD
  - **Unit**: %
  - **Range**: 10%-90%
  - **Description**: Cut SOC for discharge

## Temperature Limits for Battery Operation (Registers 106-109)
These registers define the operational temperature limits for lead-acid battery discharge and charge.

### Register Details
- **Register 106**: TemprLowerLimitDischg
  - **Unit**: 0.1°C
  - **Range**: 0-65536
  - **Description**: Lead-acid Temperature low limit for discharge
  
- **Register 107**: TemprUpperLimitDischg
  - **Unit**: 0.1°C
  - **Range**: 0-65536
  - **Description**: Lead-acid Temperature high limit for discharge
  
- **Register 108**: TemprLowerLimitChg
  - **Unit**: 0.1°C
  - **Range**: 0-65536
  - **Description**: Lead-acid Temperature low limit for charge
  
- **Register 109**: TemprUpperLimitChg
  - **Unit**: 0.1°C
  - **Range**: 0-65536
  - **Description**: Lead-acid Temperature high limit for charge

## Function Enable 1 (Register 110)
This register controls various system functions through individual bits.

### Bit Definitions
- **Bit 0**: FunctionEn1.ubPVGridOffEn
  - **Range**: 0,1
  - **Description**: 0-disable 1-enable
  
- **Bit 1**: FunctionEn1.ubFastZeroExport
  - **Range**: 0,1
  - **Description**: 0-disable 1-enable
  
- **Bit 2**: FunctionEn1.ubMicroGridEn
  - **Range**: 0,1
  - **Description**: 0 - disable, 1-enable
  
- **Bit 3**: FunctionEn1.ubBatShared
  - **Range**: 0,1
  - **Description**: 0 - disable, 1-enable
  
- **Bit 4**: FunctionEn1.ubChgLastEn
  - **Range**: 0,1
  - **Description**: 0 - disable, 1-enable
  
- **Bit 5-6**: FunctionEn1.CTSampleRatio
  - **Description**: 0:1/1000 1-1/3000
  
- **Bit 12-13**: FunctionEn1.PVCTSampleRatio
  - **Description**: 0:1/1000 1-1/3000

## System Type Setting (Register 112)
This register defines the operational mode of the system, especially for parallel configurations.

### Register Details
- **Register 112**: SetSystemType
  - **Range**: 0,1,2
  - **Description**: System type
    - 0: no parallel (non-parallel system)
    - 1: single phase parallel (master)
    - 2: slave

## Parallel System Configuration (Registers 113-114)
These registers control parallel system setup and fault management.

### Register Details
- **Register 113**: SetComposedPhase
  - **Range**: 1-3
  - **Description**: Configures composed phases in a parallel system
    - 1: R phase
    - 2: S phase
    - 3: T phase (three phase parallel master)
  
- **Register 114**: ClearFunction
  - **Range**: 1
  - **Description**: Used to clear the fault code of a parallel connection (1-clear)

## Over-Frequency Derating Control (Registers 115, 124)
These registers control over-frequency power derating parameters.

### Register Details
- **Register 115**: OVFDerateStartPoint
  - **Unit**: 0.01Hz
  - **Range**: 5000-5200
  - **Description**: Defines the starting frequency point for over-frequency derating power
  
- **Register 124**: OVFDerateEndPoint
  - **Unit**: 0.01Hz
  - **Range**: 5000-5200
  - **Description**: Defines the end frequency point for over-frequency load reduction

## Battery Power and Derating Control (Registers 116, 118)
These registers control battery power limits and derating behavior.

### Register Details
- **Register 116**: PtoUserStartdischg
  - **Unit**: 1W
  - **Range**: 50W-
  - **Description**: Sets the "Ptouser" limit for using battery power
  - **Note**: This register may also serve as Smart Load Start SOC in some configurations. Current backup shows value 100, which could represent 100% SOC for Smart Load operation.
  
- **Register 118**: VbatStartDerating
  - **Unit**: 0.1V
  - **Range**: >CutVolt ForDischg +2V
  - **Description**: For lead-acid batteries, this value dictates the voltage threshold below which discharge power decreases according to a given curve

## CT Power Compensation (Register 119)
This register provides power offset compensation for Current Transformer measurements.

### Register Details
- **Register 119**: wCT_PowerOffset
  - **Unit**: 1W
  - **Range**: ±1000W
  - **Description**: A signed short integer used for CT (Current Transformer) Power compensation, where the "PtoUser" direction is positive

## System Enable Functions (Register 120)
This register controls various system enable functions through individual bits.

### Bit Definitions
- **Bit 0**: HalfHourACChrStartEn
  - **Range**: 0,1
  - **Description**: 0-Disable, 1-Enable; Default:0
  
- **Bit 1-3**: ACChargeType
  - **Range**: 0-3
  - **Description**: 
    - 0: disable
    - 1: according to time
    - 2: according to voltage
    - 3: according to SOC
  - **Current Value**: 2 (According to voltage)
  - **Note**: AC charging is currently configured to operate based on battery voltage thresholds rather than time schedules or SOC levels. **RECOMMENDATION**: This should be set to 3 (according to SOC) since SOC thresholds are configured (Start: 85%, End: 0%). The current voltage-based operation may not align with the intended SOC-based charging strategy.
  
- **Bit 4-5**: DischgCtrlType
  - **Range**: 0-2
  - **Description**:
    - 0: according to voltage
    - 1: according to SOC
    - 2: according to both
  
- **Bit 6**: OnGridEODType
  - **Range**: 0-1
  - **Description**: 0-according to voltage, 1-according to SOC
  
- **Bit 7**: GenChargeType
  - **Range**: 0-1
  - **Description**: 0-According to Battery voltage, 1-According to Battery SOC

## Bus Voltage and QV Settings (Register 121)
This register controls bus voltage parameters for QV curve operation.

### Register Details
- **Register 121**: Q2_QV
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: Q2 reactive power percentage value for Q(V) curve
  - **Note**: Located at `UL Compliance > Reactive Power Capability > Voltage-Reactive Power Mode > Q2(%)` on EG4 monitoring installer website. This register stores the Q2 reactive power percentage value for the Q(V) curve. Backup data shows values like 0% and 10%, confirming it stores percentage values. The original documentation showing "0.1V-%" and range "4500-5500" appears to be incorrect.  The Manufacturer documentation also had BusVoltageHighEE? Crossed out for the name of this one.

## EPS Discharge SOC Limit (Register 125)
This register sets the low SOC limit for Emergency Power Supply operation.

### Register Details
- **Register 125**: SOCLowLimitForEPSDischg
  - **Unit**: %
  - **Range**: 0-EOD
  - **Description**: Sets the low SOC limit for EPS (Emergency Power Supply) discharge

## Optimal Charge/Discharge Time Periods (Registers 126-127)
These registers control optimal charge and discharge time periods with bit-field encoding.

### Register Details
- **Register 126**: OptimalChg_DisChg
  - **Bit 0-1**: Time0
    - **Range**: 0-2
    - **Description**: 0:00~0:30 time period charge and discharge mark
  
  - **Bit 2-3**: Time1
    - **Range**: 0-2
    - **Description**: Default: 0
  
  - **Bit 4-5**: Time2
    - **Range**: 0-2
    - **Description**: 0-no action, 1-charging, 2-discharging
  
  - **Bit 14-15**: Time7
    - **Range**: 0-2
    - **Description**: 0:30~1:00 and 1:00~1:30 time period charge and discharge marks

- **Register 127**: OptimalChg_DisChg.Time
  - **Bit 0-1**: Time8
    - **Range**: 0-2
    - **Description**: charge and discharge mark
  
  - **Bit 2-3**: Time9
    - **Range**: 0-2
    - **Description**: 3:30~4:00 time period charge and discharge mark
  
  - **Bit 4-5**: Time10
    - **Range**: 0-2
    - **Description**: 4:00~4:30 time period charge and discharge mark
  
  - **Bit 6-7**: Time11
    - **Range**: 0-2
    - **Description**: 4:30~5:00 time period charge and discharge mark
  
  - **Bit 8-9**: Time12
    - **Range**: 0-2
    - **Description**: 5:00~5:30 time period charge and discharge mark
  
  - **Bit 10-11**: Time13
    - **Range**: 0-2
    - **Description**: 5:30~6:00 time period charge and discharge mark
  
  - **Bit 12-13**: Time14
    - **Range**: 0-2
    - **Description**: 6:00~6:30 time period charge and discharge mark
  
  - **Bit 14-15**: Time15
    - **Range**: 0-2
    - **Description**: 7:30~8:00 time period charge and discharge mark

**Note**: Each 2-bit field controls a 30-minute time period with values:
- 0: No action
- 1: Charging
- 2: Discharging

## Optimal Charge/Discharge Time Periods Continuation (Registers 128-132)
These registers continue the time period control for the remaining hours of the day.

### Register Details
- **Register 128**: OptimalChg_DisChg.Time16-19
  - **Bit 0-1**: Time16
    - **Range**: 0-2
    - **Description**: 8:00~8:30 time period charge and discharge mark
  
  - **Bit 2-3**: Time17
    - **Range**: 0-2
    - **Description**: Default: 0
  
  - **Bit 4-5**: Time18
    - **Range**: 0-2
    - **Description**: 0-no action, 1-charging, 2-discharging
  
  - **Bit 6-7**: Time19
    - **Range**: 0-2
    - **Description**: 8:30~9:00 time period charge and discharge mark
  
  - **Bit 8-9**: Time20
    - **Range**: 0-2
    - **Description**: 9:00~9:30 time period charge and discharge mark
  
  - **Bit 10-11**: Time21
    - **Range**: 0-2
    - **Description**: 9:30~10:00 time period charge and discharge mark
  
  - **Bit 12-13**: Time22
    - **Range**: 0-2
    - **Description**: 10:00~10:30 time period charge and discharge mark
  
  - **Bit 14-15**: Time23
    - **Range**: 0-2
    - **Description**: 11:30~12:00 time period charge and discharge mark

- **Register 129**: OptimalChg_DisChg.Time24-27
  - **Bit 0-1**: Time24
    - **Range**: 0-2
    - **Description**: 12:00~12:30 time period charge and discharge mark
  
  - **Bit 2-3**: Time25
    - **Range**: 0-2
    - **Description**: Default: 0
  
  - **Bit 4-5**: Time26
    - **Range**: 0-2
    - **Description**: 0-no action, 1-charging, 2-discharging
  
  - **Bit 6-7**: Time27
    - **Range**: 0-2
    - **Description**: 12:30~13:00 time period charge and discharge mark
  
  - **Bit 8-9**: Time28
    - **Range**: 0-2
    - **Description**: 13:00~13:30 time period charge and discharge mark
  
  - **Bit 10-11**: Time29
    - **Range**: 0-2
    - **Description**: 13:30~14:00 time period charge and discharge mark
  
  - **Bit 12-13**: Time30
    - **Range**: 0-2
    - **Description**: 14:00~14:30 time period charge and discharge mark
  
  - **Bit 14-15**: Time31
    - **Range**: 0-2
    - **Description**: 15:30~16:00 time period charge and discharge mark

- **Register 130**: OptimalChg_DisChg.Time32-35
  - **Bit 0-1**: Time32
    - **Range**: 0-2
    - **Description**: 16:00~16:30 time period charge and discharge mark
  
  - **Bit 2-3**: Time33
    - **Range**: 0-2
    - **Description**: Default: 0
  
  - **Bit 4-5**: Time34
    - **Range**: 0-2
    - **Description**: 0-no action, 1-charging, 2-discharging
  
  - **Bit 6-7**: Time35
    - **Range**: 0-2
    - **Description**: 16:30~17:00 time period charge and discharge mark
  
  - **Bit 8-9**: Time36
    - **Range**: 0-2
    - **Description**: 17:00~17:30 time period charge and discharge mark
  
  - **Bit 10-11**: Time37
    - **Range**: 0-2
    - **Description**: 17:30~18:00 time period charge and discharge mark
  
  - **Bit 12-13**: Time38
    - **Range**: 0-2
    - **Description**: 18:00~18:30 time period charge and discharge mark
  
  - **Bit 14-15**: Time39
    - **Range**: 0-2
    - **Description**: 19:30~20:00 time period charge and discharge mark

- **Register 131**: OptimalChg_DisChg.Time40-43
  - **Bit 0-1**: Time40
    - **Range**: 0-2
    - **Description**: 20:00~20:30 time period charge and discharge mark
  
  - **Bit 2-3**: Time41
    - **Range**: 0-2
    - **Description**: Default: 0
  
  - **Bit 4-5**: Time42
    - **Range**: 0-2
    - **Description**: 0-no action, 1-charging, 2-discharging
  
  - **Bit 6-7**: Time43
    - **Range**: 0-2
    - **Description**: 20:30~21:00 time period charge and discharge mark
  
  - **Bit 8-9**: Time44
    - **Range**: 0-2
    - **Description**: 21:00~21:30 time period charge and discharge mark
  
  - **Bit 10-11**: Time45
    - **Range**: 0-2
    - **Description**: 21:30~22:00 time period charge and discharge mark
  
  - **Bit 12-13**: Time46
    - **Range**: 0-2
    - **Description**: 22:00~22:30 time period charge and discharge mark
  
  - **Bit 14-15**: Time47
    - **Range**: 0-2
    - **Description**: 23:30~0:00 time period charge and discharge mark

## Battery Cell Management (Register 132)
This register also contains battery cell voltage limits and configuration.

### Register Details
- **Register 132**: Battery Cell Parameters
  - **BatCellVoltLow**: Unit 0.1V, Range 0-200, Description: Battery cell voltage lower limit
  - **BatCellVoltHigh**: Unit 0.1V, Range 0-200, Description: Battery cell voltage upper limit

## Battery Cell Configuration (Register 133)
This register defines the battery cell arrangement.

### Register Details
- **Register 133**: Battery Cell Configuration
  - **BatCellSerialNum**: Unit 1, Range 0-200, Description: The number of battery cells in series
  - **BatCellParaNum**: Unit 1, Range 0-200, Description: The number of battery cells in parallel

## Under-Frequency Derating Control (Register 134)
This register controls under-frequency load reduction parameters.

### Register Details
- **Register 134**: UVFDerateStartPoint
  - **Unit**: 0.01Hz
  - **Range**: 4500-5000
  - **Description**: Underfrequency load reduction starting point

## Complete Frequency Derating Control (Registers 134-136)
These registers provide comprehensive frequency-based load control.

### Register Details
- **Register 134**: UVFDerateStartPoint
  - **Unit**: 0.01Hz
  - **Range**: 4500-5000
  - **Description**: Underfrequency load reduction starting point
  
- **Register 135**: UVFDerateEndPoint
  - **Unit**: 0.01Hz
  - **Range**: 4500-5000
  - **Description**: Underfrequency derating end point
  
- **Register 136**: OVFDerateRatio
  - **Unit**: %Pm/Hz
  - **Range**: 1-100
  - **Description**: Underfrequency load shedding slope

## Load Compensation (Register 137)
This register controls specific load compensation parameters.

### Register Details
- **Register 137**: SpecLoadCompensate
  - **Unit**: W
  - **Range**: 0-65535
  - **Description**: Maximum compensation amount for a specific load

## Power Command Control (Registers 138-143)
These registers provide precise power percentage control for various operations.

### Register Details
- **Register 138**: ChargePowerPercentCMD
  - **Unit**: 0.1%
  - **Range**: 0-1000
  - **Description**: Charging power percentage setting
  
- **Register 139**: DischgPowerPercentCMD
  - **Unit**: 0.1%
  - **Range**: 0-1000
  - **Description**: Discharge power percentage setting
  
- **Register 140**: ACChgPowerCMD
  - **Unit**: 0.1%
  - **Range**: 0-1000
  - **Description**: AC charge percentage setting
  
- **Register 141**: ChgFirstPowerCMD
  - **Unit**: 0.1%
  - **Range**: 0-1000
  - **Description**: Charging priority percentage setting
  
- **Register 142**: ForcedDischgPowerCMD
  - **Unit**: 0.1%
  - **Range**: 0-1000
  - **Description**: Forced discharge percentage setting
  
- **Register 143**: ActivePowerPercentCMD
  - **Unit**: 0.1%
  - **Range**: 0-1000
  - **Description**: Inverter active power percentage setting

## Float Charge and Output Configuration (Registers 144-146)
These registers control float charging and output priority settings.

### Register Details
- **Register 144**: FloatChargeVolt
  - **Unit**: 0.1V
  - **Range**: 500-560
  - **Description**: Float given voltage
  - **Note**: This register may also serve as Smart Load Start Volt in some configurations. Current backup shows value 540, which represents 54.0V and matches the Smart Load Start Volt setting in the GUI.
  
- **Register 145**: OutputPrioConfig
  - **Range**: 0-3
  - **Description**: Output priority configuration
    - 0: bat first
    - 1: PV first
    - 2: AC first
  
- **Register 146**: LineMode
  - **Range**: 0-2
  - **Description**: Line mode settings
    - 0: APL (90-280V 20ms)
    - 1: UPS (170-280V 10ms)
    - 2: GEN (90-280V 20ms)

## Battery Parameters for Unmatched Batteries (Registers 147-148)
These registers define battery characteristics for unmatched battery configurations.

### Register Details
- **Register 147**: Battery capacity
  - **Unit**: Ah
  - **Range**: 0-10000
  - **Description**: Battery capacity, for unmatched batteries
  
- **Register 148**: Battery nominal Voltage
  - **Unit**: 0.1V
  - **Range**: 400-590
  - **Description**: Battery rated voltage for unmatched batteries

## Cell Balancing and Equalization (Registers 149-151)
These registers control battery cell balancing and maintenance.

### Register Details
- **Register 149**: EqualizationVolt
  - **Range**: 500-590
  - **Description**: Cell Balancing Voltage
  
- **Register 150**: EqualizationInterval
  - **Unit**: Day
  - **Range**: 0-365
  - **Description**: Equalization interval
  
- **Register 151**: EqualizationTime
  - **Unit**: hour
  - **Range**: 0-24
  - **Description**: Equilibrium duration

## AC Load Time Settings (Registers 152-154)
These registers control AC load operation time windows.

### Register Details
- **Register 152**: ACFirstStartHour / ACFirstStartMinute
  - **ACFirstStartHour**: Range 0-23, Description: AC load start time_hour setting
  - **ACFirstStartMinute**: Range 0-59, Description: AC load start time_minute setting
  
- **Register 153**: ACFirstEndHour / ACFirstEndMinute
  - **ACFirstEndHour**: Range 0-23, Description: AC load end time_hour setting
  - **ACFirstEndMinute**: Range 0-59, Description: AC load end time_minute setting
  
- **Register 154**: ACFirstStartHour1 / ACFirstStartMinute1
  - **ACFirstStartHour1**: Range 0-23, Description: AC load start time_hour setting
  - **ACFirstStartMinute1**: Range 0-59, Description: AC load start time_minute setting

## AC Load Time Settings Continuation (Registers 155-157)
These registers complete the AC load time window configuration.

### Register Details
- **Register 155**: ACFirstEndHour1 / ACFirstEndMinute1
  - **ACFirstEndHour1**: Range 0-23, Description: AC load end time_hour setting
  - **ACFirstEndMinute1**: Range 0-59, Description: AC load end time_minute setting
  
- **Register 156**: ACFirstStartHour2 / ACFirstStartMinute2
  - **ACFirstStartHour2**: Range 0-23, Description: AC load start time_hour setting
  - **ACFirstStartMinute2**: Range 0-59, Description: AC load start time_minute setting
  
- **Register 157**: ACFirstEndHour2 / ACFirstEndMinute2
  - **ACFirstEndHour2**: Range 0-23, Description: AC load end time_hour setting
  - **ACFirstEndMinute2**: Range 0-59, Description: AC load end time_minute setting

## AC Charging Parameters (Registers 158-161)
These registers control the battery voltage and SOC thresholds for AC charging.

### Register Details
- **Register 158**: ACChgStartVolt
  - **Unit**: 0.1V
  - **Range**: 384-520
  - **Description**: AC charging starting battery voltage, valid after selecting ACChg according to voltage
  - **Current Value**: 400 (40.0V)
  - **Note**: AC charging will begin when the battery voltage drops to 40.0V or below
  
- **Register 159**: ACChgEndVolt
  - **Unit**: 0.1V
  - **Range**: 480-590
  - **Description**: AC charging cut off the battery voltage, valid after selecting ACChg according to voltage
  - **Current Value**: 530 (53.0V)
  - **Note**: AC charging will stop when the battery voltage reaches 53.0V or above
  
- **Register 160**: ACChgStartSOC
  - **Unit**: %
  - **Range**: 0-90
  - **Description**: AC charging starting SOC, valid after selecting ACChg according to SOC
  - **Current Value**: 85%
  - **Note**: AC charging will begin when the battery SOC drops to 85% or below
  
- **Register 161**: ACChgEndSOC
  - **Unit**: %
  - **Range**: 20-100
  - **Description**: AC charging stops SOC, it is valid after selecting ACChg according to SOC
  - **Current Value**: 0%
  - **Note**: AC charging will stop when the battery SOC reaches 0% (this appears to be a configuration error - should typically be set to 90% or higher)

## Battery Low Voltage/SOC Alarm and Utility Conversion (Registers 162-167)
These registers define alarm and recovery points for battery undervoltage and SOC, and parameters for converting to mains utility.

### Register Details
- **Register 162**: BatLowVoltage
  - **Unit**: 0.1V
  - **Range**: 400-500
  - **Description**: Battery undervoltage alarm point, DisChgCtrl selects according to voltage or both to be valid
  
- **Register 163**: BatLowBackVoltage
  - **Unit**: 0.1V
  - **Range**: 420-520
  - **Description**: Battery undervoltage alarm recovery point, DisChgCtrl selects according to voltage or both is valid
  
- **Register 164**: BatLowSOC
  - **Unit**: %
  - **Range**: 0-90
  - **Description**: Battery undervoltage alarm point, DisChgCtrl selects according to SOC or both is valid
  
- **Register 165**: BatLowBackSOC
  - **Unit**: %
  - **Range**: 20-100
  - **Description**: Battery undervoltage alarm recovery point, DisChgCtrl selects according to SOC or both is valid
  
- **Register 166**: BatLowtoUtilityVoltage
  - **Unit**: 0.1V
  - **Range**: 444-514
  - **Description**: When the battery is under voltage to the mains voltage point, DisChgCtrl selects according to voltage or both to be valid
  
- **Register 167**: BatLowtoUtilitySOC
  - **Unit**: %
  - **Range**: 0-100
  - **Description**: The battery undervoltage is converted to the mains SOC. DisChgCtrl selects according to SOC or both to be valid

## AC Charging and Grid Integration (Registers 168-169)
These registers control AC charging current and on-grid end-of-discharge voltage.

### Register Details
- **Register 168**: ACCharge Bat Current
  - **Unit**: A (Amperes)
  - **Range**: 0-140
  - **Description**: ChargeCurrent from AC Active When TakeLoadTogether enabled
  - **Current Value**: 0A
  - **Note**: This register controls the AC charging current when the inverter is powering loads simultaneously. A value of 0A disables this feature.
  
- **Register 169**: OngridEOD_Voltage
  - **Unit**: 0.1V
  - **Range**: 400-560
  - **Description**: On-grid end-of-discharge voltage threshold

## SOC Curve Configuration (Registers 171-175)
These registers define the SOC-voltage relationship curve and battery characteristics.

### Register Details
- **Register 171**: SOCCurve_BatVolt1
  - **Unit**: 0.1V
  - **Range**: 400-600
  - **Description**: SOC(V) curve voltage point 1
  
- **Register 172**: SOCCurve_BatVolt2
  - **Unit**: 0.1V
  - **Range**: 400-600
  - **Description**: SOC(V) curve voltage point 2
  
- **Register 173**: SOCCurve_SOC1
  - **Unit**: 1%
  - **Range**: 0-100
  - **Description**: SOC(V) curve SOC point 1
  
- **Register 174**: SOCCurve_SOC2
  - **Unit**: 1%
  - **Range**: 0-100
  - **Description**: SOC(V) curve SOC point 2
  
- **Register 175**: SOCCurve_InnerResistance
  - **Unit**: mΩ (milliohms)
  - **Range**: 0-100
  - **Description**: Battery internal resistance for SOC calculation

## Grid Power Limits (Registers 176-177)
These registers control maximum grid input power and generator rate power.

### Register Details
- **Register 176**: MaxGridInputPower
  - **Unit**: W (Watts)
  - **Range**: (Not specified)
  - **Description**: Maximum grid input power limit
  
- **Register 177**: GenRatePower
  - **Unit**: W (Watts)
  - **Range**: (Not specified)
  - **Description**: Generator rate power setting

## Function Enable 2 (Register 179)
This register controls various system functions through individual bits.

### Bit Definitions
- **Bit 0**: uFunctionEn2.ACCTDirection
  - **Range**: 0,1
  - **Description**: 0-Normal 1-Reversed
  
- **Bit 1**: uFunctionEn2.PVCTDirection
  - **Range**: 0,1
  - **Description**: 0-Normal 1-Reversed
  
- **Bit 2**: uFunctionEn2.AFCIAlarmClr
  - **Range**: 0,1
  - **Description**: 0-null 1-clear
  
- **Bit 3**: uFunctionEn2.BatWakeupEn
  - **Range**: 0,1
  - **Description**: 0-Disable 1-Enable
  
- **Bit 4**: uFunctionEn2.VoltWattEn
  - **Range**: 0,1
  - **Description**: 0-Disable 1-Enable
  
- **Bit 5**: uFunctionEn2.TriptimeUnit
  - **Range**: 0,1
  - **Description**: 0-Disable 1-Enable
  
- **Bit 6**: uFunctionEn2.ActPowerCMDEN
  - **Range**: 0,1
  - **Description**: 0-Disable 1-Enable
  
- **Bit 11**: uFunctionEn2.ACCouplingEnable
  - **Range**: 0,1
  - **Description**: 0-Disable 1-Enable AC coupling functionality
  - **Current Value**: 1 (Enabled)
  - **Note**: This bit controls whether AC coupling functionality is enabled. When enabled, the inverter can manage AC-coupled loads and generators.
  
- **Bit 7-15**: uFunctionEn2.all

## AC Coupling Configuration (Registers 220-223)
These registers control AC coupling functionality for managing AC-coupled loads and generators.

### Register Details
- **Register 220**: ACCoupleStartSOC
  - **Unit**: %
  - **Range**: 0-100%
  - **Description**: AC coupling start SOC threshold
  - **Current Value**: 99%
  - **Note**: Located at `Smart Load Settings > AC Coupling Settings > Smart Load Start SOC(%)` on EG4 monitoring installer website. This register sets the battery SOC at which AC coupling operation begins. When the battery SOC drops to 99% or below, AC coupling will activate to manage loads.
  
- **Register 221**: ACCoupleEndSOC
  - **Unit**: %
  - **Range**: 0-100%
  - **Description**: AC coupling end SOC threshold
  - **Current Value**: 101%
  - **Note**: Located at `Smart Load Settings > AC Coupling Settings > Smart Load End SOC(%)` on EG4 monitoring installer website. This register sets the battery SOC at which AC coupling operation ends. The value 101% is intentionally set above 100% to ensure AC coupling continues until the battery is fully charged. In off-grid mode, this allows the AC Couple bypass to take over the load after the battery reaches full charge, providing seamless power management.
  
- **Register 222**: ACCoupleStartVolt
  - **Unit**: 0.1V
  - **Range**: 40.0V-60.0V (400-600 in register units)
  - **Description**: AC coupling start voltage threshold
  - **Current Value**: 595 (59.5V)
  - **Note**: Located at `Smart Load Settings > AC Coupling Settings > Smart Load Start Volt(V)` on EG4 monitoring installer website. This register sets the battery voltage at which AC coupling operation begins. When the battery voltage drops to 59.5V or below, AC coupling will activate.
  
- **Register 223**: ACCoupleEndVolt
  - **Unit**: 0.1V
  - **Range**: 40.0V-60.0V (400-600 in register units)
  - **Description**: AC coupling end voltage threshold
  - **Current Value**: 800 (80.0V)
  - **Note**: Located at `Smart Load Settings > AC Coupling Settings > Smart Load End Volt(V)` on EG4 monitoring installer website. This register sets the battery voltage at which AC coupling operation ends. The value 80.0V may use a different voltage scale or represent a different voltage reference point than the typical 40.0V-60.0V range.

### AC Coupling Operation Mode
**Current Configuration Status**: AC coupling is **ENABLED** (Register 179, Bit 11 = 1).

**Configuration Details**:
- **Enable/Disable**: Register 179, Bit 11 = 1 (Enabled)
- **SOC Thresholds**: Start at 99%, End at 101% (intentionally set above 100% for full charge)
- **Voltage Thresholds**: Start at 59.5V, End at 80.0V (may use different voltage scale)

**Operation Behavior**: 
- AC coupling activates when battery SOC drops to 99% or voltage drops to 59.5V
- AC coupling deactivates when battery SOC reaches 101% (ensures full charge) or voltage reaches 80.0V
- The SOC threshold of 101% is intentionally set above 100% to ensure AC coupling continues until the battery is fully charged

**Note**: The SOC threshold of 101% is a deliberate configuration choice. In off-grid mode, this ensures AC coupling continues until the battery reaches full charge, at which point the AC Couple bypass takes over the load. The voltage threshold of 80.0V may use a different scale or represent a different voltage reference point than the typical 40.0V-60.0V range.

## AFCI Arc Fault Circuit Interrupter (Register 180)
This register controls arc fault detection parameters.

### Register Details
- **Register 180**: AFCIArcThreshold
  - **Range**: (Not specified)
  - **Description**: Arc fault detection threshold

## Volt-Watt Control Parameters (Registers 181-184)
These registers control the Volt-Watt curve for grid voltage response.

### Register Details
- **Register 181**: VoltWatt_V1
  - **Unit**: 0.1V
  - **Range**: 1.05Vn-1.09Vn, default 1.06Vn
  - **Description**: Volt-Watt curve voltage point 1
  
- **Register 182**: VoltWatt_V2
  - **Unit**: 0.1V
  - **Range**: (V1+0.01Vn)-1.10Vn, default 1.1Vn
  - **Description**: Volt-Watt curve voltage point 2
  
- **Register 183**: VoltWatt_DelayTime
  - **Unit**: Seconds
  - **Range**: 500-60000ms
  - **Description**: Default 10000ms
  - **Note**: Located at `UL Compliance > Voltage-Active Power Mode > Open Loop Response Time(s)` on EG4 Monitoring and installer website. GUI displays time in seconds (e.g., 10s, 12s) but documentation shows range in milliseconds (500-60000ms). Backup data shows values like 10 and 12, suggesting it stores seconds directly.
  
- **Register 184**: VoltWatt_P2
  - **Unit**: %
  - **Range**: 0-200
  - **Description**: Volt-Watt power reduction at V2

## QV Curve Reference Parameters (Registers 185-186)
These registers control the Q(V) curve reference voltage and filter time.

### Register Details
- **Register 185**: Vref_QV
  - **Unit**: 0.1V
  - **Range**: (Not specified)
  - **Description**: Q(V) curve reference voltage
  
- **Register 186**: Vref_filtertime
  - **Unit**: s (seconds)
  - **Range**: 300-5000
  - **Description**: Q(V) curve voltage filter time

## QV Curve Control Points (Registers 187-188)
These registers define the reactive power control points for the Q(V) curve.

### Register Details
- **Register 187**: Q3_QV
  - **Unit**: %
  - **Range**: (Not specified)
  - **Description**: Q(V) curve reactive power point 3
  
- **Register 188**: Q4_QV
  - **Unit**: %
  - **Range**: (Not specified)
  - **Description**: Q(V) curve reactive power point 4

## QP Priority Control Points (Registers 189-192)
These registers control the QP priority curve for power management.

### Register Details
- **Register 189**: P1_QP
  - **Unit**: %
  - **Range**: (Not specified)
  - **Description**: QP priority curve power point 1
  
- **Register 190**: P2_QP
  - **Unit**: %
  - **Range**: (Not specified)
  - **Description**: QP priority curve power point 2
  
- **Register 191**: P3_QP
  - **Unit**: %
  - **Range**: (Not specified)
  - **Description**: QP priority curve power point 3
  
- **Register 192**: P4_QP
  - **Unit**: %
  - **Range**: (Not specified)
  - **Description**: QP priority curve power point 4

## Under-Frequency Increase Ratio (Register 193)
This register controls the under-frequency power increase response.

### Register Details
- **Register 193**: UVFIncreaseRatio
  - **Unit**: %Pm/Hz
  - **Range**: 1-100
  - **Description**: Underfrequency loading slope

## Generator Charging Parameters (Registers 194-198)
These registers control comprehensive generator charging parameters including voltage, SOC, and current limits.

### Register Details
- **Register 194**: GenChgStartVolt
  - **Unit**: 0.1V
  - **Range**: 384-520
  - **Description**: Generator charging starting battery voltage, valid after selecting GenChg according to voltage
  
- **Register 195**: GenChgEndVolt
  - **Unit**: 0.1V
  - **Range**: 480-590
  - **Description**: The battery voltage is cut off when the alternator is charged. It is valid after selecting GenChg according to voltage
  
- **Register 196**: GenChgStartSOC
  - **Unit**: %
  - **Range**: 0-90
  - **Description**: Generator charging starting SOC, valid after selecting GenChg according to SOC
  
- **Register 197**: GenChgEndSOC
  - **Unit**: %
  - **Range**: 20-100
  - **Description**: The SOC of the alternator is stopped when charging, and it is valid after selecting GenChg according to SOC
  
- **Register 198**: MaxGenChgBatCurr
  - **Unit**: A
  - **Range**: 0-60
  - **Description**: Charge current from generator

## LCD Password Configuration (Register 225)
This register stores the LCD password for accessing the inverter's local display interface.

### Register Details
- **Register 225**: LCDPassword
  - **Unit**: Integer
  - **Range**: 0-65535
  - **Description**: LCD password value for local display access
  - **Default**: 00000
  - **Note**: Located at `Maintenance > Ramote Set > LCD Password` on EG4 monitoring installer website. This register stores the 5-digit password used to access the inverter's local LCD display interface. The password is stored as a plain integer value.

## Smart Load Configuration (Registers 213-217, 227-228)
These registers control Smart Load functionality for intelligent load management based on battery state and PV power.

### Register Details
- **Register 213**: SmartLoadStartVolt
  - **Unit**: 0.1V
  - **Range**: 40.0V-60.0V
  - **Description**: Smart Load start voltage threshold (currently 54.0V)
  - **Note**: Located at `Smart Load Settings > Smart Load Start Volt(V)` on EG4 monitoring installer website. This register controls the battery voltage at which Smart Load operation begins.

- **Register 214**: SmartLoadEndVolt
  - **Unit**: 0.1V
  - **Range**: 40.0V-60.0V
  - **Description**: Smart Load end voltage threshold (currently 48.0V)
  - **Note**: Located at `Smart Load Settings > Smart Load End Volt(V)` on EG4 monitoring installer website. This register controls the battery voltage at which Smart Load operation ends.

- **Register 215**: SmartLoadStartSOC
  - **Unit**: %
  - **Range**: 0-100%
  - **Description**: Smart Load start SOC threshold (currently 100%)
  - **Note**: Located at `Smart Load Settings > Smart Load Start SOC(%)` on EG4 monitoring installer website. This register controls the battery SOC at which Smart Load operation begins.

- **Register 216**: SmartLoadEndSOC
  - **Unit**: %
  - **Range**: 0-100%
  - **Description**: Smart Load end SOC threshold (currently 0%)
  - **Note**: Located at `Smart Load Settings > Smart Load End SOC(%)` on EG4 monitoring installer website. This register controls the battery SOC at which Smart Load operation ends.

- **Register 217**: StartPVPower
  - **Unit**: 0.1kW
  - **Range**: 0.0kW-10.0kW
  - **Description**: Start PV power threshold for Smart Load operation (currently 0.5kW)
  - **Note**: Located at `Smart Load Settings > Start PV Power(kW)` on EG4 monitoring installer website. This register controls the minimum PV power required to start Smart Load operation.

- **Register 227**: SmartLoadStartSOCAlt
  - **Unit**: %
  - **Range**: 0-100%
  - **Description**: Alternative Smart Load start SOC threshold (currently 100%)
  - **Note**: This appears to be a duplicate or alternative register for Smart Load start SOC control.

- **Register 228**: SmartLoadStartVoltAlt
  - **Unit**: 0.1V
  - **Range**: 40.0V-60.0V
  - **Description**: Alternative Smart Load start voltage threshold (currently 54.0V)
  - **Note**: This appears to be a duplicate or alternative register for Smart Load start voltage control.

### Smart Load Control Bits
Smart Load enable/disable is controlled through function enable registers:

- **Register 21, Bit X**: Smart Load Enable/Disable (specific bit to be determined)
- **Register 110, Bit X**: Grid Always On Enable/Disable (specific bit to be determined)
- **Register 179, Bit X**: Additional Smart Load control (specific bit to be determined)

**Note**: The exact bit positions for Smart Load and Grid Always On controls need to be determined through interactive testing (changing values in the GUI and taking before/after backups).

### AC Coupling Integration
**Important**: AC coupling functionality is closely integrated with Smart Load settings. The AC coupling registers (220-223) control thresholds that determine when AC-coupled loads are managed:

- **Registers 220-221**: SOC thresholds for AC coupling operation
- **Registers 222-223**: Voltage thresholds for AC coupling operation
- **Register 179, Bit 11**: AC coupling enable/disable control

When AC coupling is enabled, the inverter can manage AC-coupled loads and generators based on the configured SOC and voltage thresholds. This functionality works in conjunction with Smart Load settings to provide intelligent load management.



## Implementation Notes

### Writing to Hold Registers
- Use function code 0x06 for single register writes
- Use function code 0x10 for multiple register writes
- Validate data ranges before writing
- Some registers may require system restart to take effect

### Bit-Field Registers
- Read the current value before modifying
- Use bitwise OR (|) to set bits: `new_value = current_value | bit_mask`
- Use bitwise AND with NOT (& !) to clear bits: `new_value = current_value & !bit_mask`
- Preserve other bits when modifying individual functions

### Data Validation
- Check minimum and maximum ranges for each register
- Ensure correct data types (uint16, int16)
- Apply appropriate scaling factors
- Validate bit combinations for function control registers

### Safety Considerations
- Anti-islanding must be enabled for grid-connected operation
- Frequency response settings affect grid stability
- Battery limits should not exceed BMS recommendations
- Voltage and current limits protect equipment and safety

## Register Access Patterns

### Common Read Operations
- Read all function control registers on startup
- Monitor frequency response parameters during operation
- Check battery management settings regularly
- Verify time synchronization periodically

### Common Write Operations
- Configure function control bits during setup
- Adjust frequency response for grid requirements
- Update battery parameters for different battery types
- Modify power limits for system changes
- Synchronize system time with external sources
