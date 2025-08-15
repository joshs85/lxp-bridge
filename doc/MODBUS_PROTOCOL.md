# LXP Inverter Modbus RTU Communication Protocol

## Overview
This document describes the Modbus RTU communication protocol used by LXP inverters for local communication and control.

## Protocol Specifications
- **Protocol**: Modbus RTU
- **Byte Order**: Little-endian
- **Parity**: None (8N1)
- **Baud Rate**: Configurable (typically 9600, 19200, 38400, 57600, 115200)
- **Slave Address**: Configurable (typically 1-247)

## Communication Configuration

### Physical Interface
- **Interface**: RS-485
- **Communication Method**: Universal Asynchronous Transceiver (UART)
- **Default Baud Rate**: 19200bps
- **Data Format**: One start bit, 8 data bits, no parity bit, one stop bit (10 bytes total)

### Data Handling
- **Minimum Polling Period**: 1 second
- **Register Width**: 2 bytes (16-bit)
- **16-bit Integer Decoding**: High and low byte order is reversed
  - Example: `0x01 0x02` should be parsed as `0x0201 = 513`
- **32-bit Integer Decoding**: High and low word order is reversed, and byte order within each word is reversed
  - Example: `0x01 0x02 0x03 0x04` should be parsed as `0x04030201 = 67305985`

### Query Limitations
- **Maximum Registers per Query**: 40 registers
- **Register Grouping**: Inverter software groups registers into 40-register blocks:
  - Group 1: Registers 0-39
  - Group 2: Registers 40-79
  - Group 3: Registers 80-119
  - And so on...
- **Critical Rule**: When querying 40 registers, the starting address must align with group boundaries
  - Valid: Starting at 0, 40, 80, 120, etc.
  - Invalid: Starting at addresses that cross group boundaries
- **Example**: To query registers 38-40, you must make two separate queries:
  - Query 1: Registers 38-39 (Group 1)
  - Query 2: Register 40 (Group 2)

## Function Codes

### Read Operations
- **0x03 Read Hold Registers**: Read multiple hold registers
- **0x04 Read Input Registers**: Read multiple input registers

### Write Operations  
- **0x06 Write Single Hold Register**: Write single hold register
- **0x10 Write Multiple Hold Registers**: Write multiple hold registers

## Message Format

### Message Structure
A Modbus RTU message consists of the following components in sequence:

| Component | Size | Description |
|-----------|------|-------------|
| Address | 1 Byte | Slave device address (1-247) |
| Function Code | 1 Byte | Modbus function code (0x03, 0x04, 0x06, 0x10) |
| Data | 1-252 Bytes | Variable length data payload |
| CRC Low Byte | 1 Byte | Cyclic Redundancy Check, low byte |
| CRC High Byte | 1 Byte | Cyclic Redundancy Check, high byte |

### Function Code List
- **0x03 Read Hold**: Read the contents of hold registers
- **0x04 Read Input**: Read the contents of input (read-only) registers  
- **0x06 Write Single Hold**: Write a single value to a hold register
- **0x10 Write Multi Hold Registers**: Write multiple values to a block of hold registers

## Bit Definition

### Data Transmission Format
Each byte transmitted follows this bit sequence:

| Bit Position | Description |
|--------------|-------------|
| Start | 1 bit (always '1') |
| Bit0 | Data bit 0 (least significant) |
| Bit1 | Data bit 1 |
| Bit2 | Data bit 2 |
| Bit3 | Data bit 3 |
| Bit4 | Data bit 4 |
| Bit5 | Data bit 5 |
| Bit6 | Data bit 6 |
| Bit7 | Data bit 7 (most significant) |
| Stop | 1 bit (always '1') |

**Note**: This defines an 8N1 data format (8 data bits, no parity, 1 stop bit) consistent with standard Modbus RTU over UART.

## Request and Response

### Response Types
The inverter responds to commands with either:
- **Normal Response**: Successful execution of the requested function
- **Error Response**: Function failed, includes specific error code

### Error Codes
When an error occurs, the inverter returns an error response with one of the following codes:

| Error Code | Error Description | Remarks |
|------------|------------------|---------|
| **0x01** | Illegal Function Code | The slave cannot recognize the function code (function code sent in request is not supported or invalid) |
| **0x02** | Illegal Data Address | Data address does not match length (requested register address is out of range or number of registers requested exceeds valid length) |
| **0x03** | Illegal Data Value | Data value out of bounds or wrong (value attempted to be written to a register is outside its permissible range or is otherwise invalid) |
| **0x04** | Slave Read and Write Failure | Read and write errors (internal device error during register access operations) |
| **0x06** | Slave is Busy | Slave is busy (device is processing another request, retry later) |

## CRC Calculation
The CRC (Cyclic Redundancy Check) is calculated using the Modbus polynomial:
- **Polynomial**: 0xA001 (reversed 0x8005)
- **Initial Value**: 0xFFFF
- **XOR Value**: 0x0000

## Communication Timing
- **Response Timeout**: Typically 1-3 seconds
- **Inter-Frame Delay**: Minimum 3.5 character times
- **Retry Count**: Recommended 3 attempts
- **Retry Delay**: Exponential backoff (1s, 2s, 4s)

## Data Validation
- **Register Ranges**: Validate addresses before sending
- **Data Values**: Check min/max ranges for each register
- **Type Checking**: Ensure correct data types (uint16, int16, float)
- **Scaling**: Apply appropriate scaling factors (0.1V, 0.01Hz, etc.)

## Error Handling
- **CRC Errors**: Retry with exponential backoff
- **Timeout Errors**: Check device connection and address
- **Function Errors**: Verify function code support
- **Data Errors**: Validate register addresses and values
- **Device Errors**: Check device status and error codes

## Implementation Notes
- All multi-word registers use little-endian byte order
- Temperature values may be negative (two's complement)
- Status bits are active high (1 = active, 0 = inactive)
- Fault and warning codes use bitwise OR for multi-word registers
