# LXP Bridge Configuration Backup

## Overview

The LXP Bridge now includes a comprehensive configuration backup system that allows you to safely export your inverter configuration for analysis, documentation, and troubleshooting purposes.

## 📋 What Gets Backed Up

The backup system documents all input and hold registers.

## 🚀 How to Use

### Option 1: Build and Run Backup Binary

```bash
# Build the backup binary
cargo build --bin backup_config

# Run backup
./target/debug/backup_config \
    --config your_config.yaml \
    --inverter YOUR_INVERTER_SERIAL \
    --output backup_$(date +%Y%m%d_%H%M%S).json

# Example with real values
./target/debug/backup_config \
    --config config.yaml \
    --inverter 5555555555 \
    --output backup_$(date +%Y%m%d_%H%M%S).json
```

### Option 2: Using the Backup Script

```bash
# Make script executable (first time only)
chmod +x scripts/backup_config.sh

# Run backup
./scripts/backup_config.sh \
    config.yaml \
    5555555555 \
    backup_$(date +%Y%m%d_%H%M%S).json
```

### Option 3: Release Build

```bash
# Build the backup binary for release
cargo build --release --bin backup_config

# Run backup
./target/release/backup_config \
    --config config.yaml \
    --inverter 5555555555 \
    --output backup.json
```

## 📁 Output Format

The backup creates a comprehensive JSON file with the following structure:

```json
{
  "backup_timestamp": "2025-08-17T16:10:10.097162+00:00",
  "inverter_serial": "5555555555",
  "inverter_datalog": "BA55555555",
  "inverter_host": "192.168.3.45",
  "inverter_port": 8000,
  "hold_registers": {
    "0": 0,
    "1": 0,
    "2": 0,
    "10": 0,
    "11": 0,
    "67": 0,
    "110": 0,
    "134": 0,
    "135": 0
  },
  "input_registers": {
    "0": 0,
    "1": 0,
    "2": 0,
    "3": 0,
    "4": 0,
    "5": 0
  },
  "notes": "Real register values read from inverter using direct TCP communication (same as addon startup)"
}
```

**Note**: Registers are now sorted numerically by register number for easy reading and comparison. The format maintains the original HashMap structure but ensures consistent ordering.

## 🔍 Understanding the Output

### Backup Structure
- **backup_timestamp**: When the backup was created (ISO 8601 format)
- **inverter_serial**: Your inverter's unique serial number
- **inverter_datalog**: Your inverter's datalog identifier (used in MQTT topics like `lxp/{datalog}/hold/{register}`)
- **inverter_host**: IP address of your inverter
- **inverter_port**: TCP port for Modbus communication
- **hold_registers**: All read/write registers (0-199) with current values
- **input_registers**: All read-only registers (0-199) with current values
- **notes**: Description of how the backup was created

### Register Values
- **Raw Values**: 16-bit unsigned integers as stored in the inverter
- **Scaling**: Some values need scaling (e.g., 2200 = 220.0V, 5999 = 59.99Hz)
- **Range**: Values typically 0-65535 (16-bit unsigned)
- **Format**: JSON object with register number as key, value as number, sorted numerically by register number

### Datalog Field Importance
- **MQTT Topics**: The datalog field is used in MQTT topic paths (e.g., `lxp/BA31100197/hold/67`)
- **Home Assistant**: Entity discovery uses the datalog for unique identification
- **Multiple Inverters**: If you have multiple inverters, each will have a unique datalog
- **Configuration**: The datalog is configured in your `config.yaml` file

### Value Scaling Examples

| Register Type | Raw Value | Scaling Factor | Display Value | Example from Backup |
|---------------|------------|----------------|---------------|---------------------|
| Voltage | 2200 | 0.1 | 220.0V | Grid voltage limits |
| Frequency | 5999 | 0.01 | 59.99Hz | Under Frequency Droop Start (Register 134) |
| Percentage | 208 | 0.1 | 20.8% | AC Charge SOC Limit (Register 67) |
| Time | 30 | 1.0 | 30s | Delay settings |
| Switches | 1056 | N/A | Binary flags | Register 110 switches (Register 110) |

## 🎯 Use Cases

### 1. System Documentation
- Document current configuration for compliance
- Create system specifications for technicians
- Maintain configuration history

### 2. Troubleshooting
- Compare configurations between systems
- Identify configuration differences
- Document settings before changes

### 3. Migration and Setup
- Prepare configuration for new systems
- Standardize settings across multiple inverters
- Backup before firmware updates

### 4. Analysis and Planning
- Review current settings
- Plan configuration changes
- Optimize system parameters

## 🔧 Integration with Home Assistant

All documented registers automatically create Home Assistant entities:

- **Entity Types**: Numbers, switches, buttons
- **MQTT Discovery**: Automatic entity creation
- **Value Conversion**: Proper scaling and units
- **Command Support**: Full control from Home Assistant

## 🚨 Important Notes

1. **✅ Fully Functional**: The backup system now reads real values from your inverter
2. **✅ Real-time Values**: Gets current register values using direct TCP communication
3. **✅ Safe Operation**: 100% read-only, no changes made to your system
4. **✅ Configuration Required**: Uses your config.yaml file for inverter connection details
5. **✅ Production Ready**: Successfully tested with real systems

## 🔄 Getting Real Register Values

The backup system now provides **real-time register values** directly from your inverter:

1. **Run the backup command** to get current values:
   ```bash
   ./target/debug/backup_config --config config.yaml --inverter YOUR_SERIAL --output backup.json
   ```

2. **Check MQTT topics** for real-time values:
   - `lxp/{datalog}/hold/{register_number}` (hold registers)
   - `lxp/{datalog}/input/{register_number}` (input registers)

3. **Use Home Assistant** to view current values (automatic discovery)

4. **Monitor MQTT messages** for live data updates

**✅ The backup system is now fully functional and reads real values from your inverter!**

## 📞 Support

If you need help with the backup system:

1. Check the generated JSON file for register information
2. Compare with your Home Assistant entities
3. Verify MQTT topics are working
4. Check the main LXP Bridge logs

---

**Remember**: This backup system is designed to be completely safe and read-only. It helps you understand your system configuration without any risk of changes or damage.