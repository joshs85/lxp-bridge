# LXP Inverter Modbus Register Mapping

## Overview
This document provides an overview of the LXP inverter Modbus register structure and references to detailed documentation.

## Documentation Structure
The LXP inverter documentation has been organized into three focused documents for better navigation and maintenance:

### 1. [MODBUS_PROTOCOL.md](MODBUS_PROTOCOL.md)
- **Modbus RTU Communication Protocol**
- Function codes (0x03, 0x04, 0x06, 0x10)
- Message formats and examples
- Error codes and handling
- CRC calculation
- Communication timing and retry logic
- Data validation guidelines

### 2. [INPUT_REGISTERS.md](INPUT_REGISTERS.md)
- **All Input Registers (Read-Only)**
- System state and PV monitoring (0-9)
- Power and grid monitoring (10-19)
- Off-grid system (20-28)
- Daily energy production (29-37)
- Cumulative energy tracking (40-59)
- System status and diagnostics (60-63)
- Temperature monitoring (64-67)
- System runtime and auto-test (68-75)
- BMS parameters (81-106)
- Generator and EPS monitoring (120-138)
- AFCI safety monitoring (140-152)

### 3. [HOLD_REGISTERS.md](HOLD_REGISTERS.md)
- **All Hold Registers (Read/Write)**
- Function control registers (21, 110)
- Frequency response control
- Battery management settings
- Grid connection parameters
- Inverter operation parameters
- Advanced features (Volt-Watt, Volt-Var)
- Time synchronization
- Implementation guidelines

## Register Type Summary

### Input Registers (Read-Only)
- **Function Code**: 0x04 (Read Input Registers)
- **Purpose**: Real-time monitoring, status, and historical data
- **Count**: 153 registers (0-152).  There may be more not documented.
- **Key Features**: PV monitoring, energy tracking, BMS data, temperature sensors, AFCI safety

### Hold Registers (Read/Write)
- **Function Codes**: 0x03 (Read), 0x06 (Write Single), 0x10 (Write Multiple)
- **Purpose**: Configuration, control, and parameter setting
- **Key Features**: Function control, frequency response, battery management, grid settings

## Quick Reference

### Critical Safety Registers
- **Anti-Islanding**: Register 21, Bit 4
- **AFCI Monitoring**: Registers 140-152
- **BMS Integration**: Registers 81-106

### Key Monitoring Registers
- **System State**: Register 0
- **PV Power**: Registers 7-9
- **Battery Status**: Registers 4-5, 81-100
- **Grid Parameters**: Registers 12-15

### Important Control Registers
- **Function Control**: Register 21 (basic functions)
- **Function Control 2**: Register 110 (advanced features)
- **Frequency Response**: Registers 115, 124, 134-136, 193

## Implementation Notes
- All multi-word registers use little-endian byte order
- Temperature values may be negative (two's complement)
- Status bits are active high (1 = active, 0 = inactive)
- Fault and warning codes use bitwise OR for multi-word registers

## Source Documentation
This documentation is based on the official LXP inverter Modbus protocol specification:
- **Document**: EG4-18KPV-12LV-Modbus-Protocol.pdf
- **Version**: Latest available
- **Coverage**: Complete protocol and register mapping

## Getting Started
1. **For Communication**: Start with [MODBUS_PROTOCOL.md](MODBUS_PROTOCOL.md)
2. **For Monitoring**: Use [INPUT_REGISTERS.md](INPUT_REGISTERS.md)
3. **For Control**: Reference [HOLD_REGISTERS.md](HOLD_REGISTERS.md)

Each document contains comprehensive details, examples, and implementation guidance for its specific area of focus.