#!/bin/bash

# ============================================================================
# LXP Bridge Configuration Backup Script
# ============================================================================
# 
# This script safely backs up your inverter configuration by reading
# all hold registers and exporting them to a JSON file.
# 
# SAFETY FEATURES:
# ✅ READ-ONLY operation - no changes made to your system
# ✅ Comprehensive backup of all configuration registers
# ✅ Detailed documentation of each register
# ✅ Safe delays between reads to avoid overwhelming the system
# 
# ============================================================================

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the lxp-bridge project root directory"
    exit 1
fi

# Check if required arguments are provided
if [ $# -lt 2 ]; then
    echo "Usage: $0 <config_file> <inverter_serial> [output_file]"
    echo ""
    echo "Arguments:"
    echo "  config_file      Path to your lxp-bridge configuration file"
    echo "  inverter_serial  Serial number of the inverter to backup"
    echo "  output_file      (Optional) Path where the backup JSON file will be saved"
    echo "                   If not specified, will save to backups/backup_<timestamp>.json"
    echo ""
    echo "Examples:"
    echo "  $0 config.yaml 5555555555"
    echo "  $0 config.yaml 5555555555 backup_$(date +%Y%m%d_%H%M%S).json"
    echo "  $0 config.yaml 5555555555 backups/my_custom_backup.json"
    echo ""
    exit 1
fi

CONFIG_FILE="$1"
INVERTER_SERIAL="$2"

# Set default output file if not provided
if [ $# -eq 2 ]; then
    # Default to backups folder with timestamp
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    OUTPUT_FILE="backups/backup_${TIMESTAMP}.json"
    print_status "No output file specified, using default: $OUTPUT_FILE"
else
    OUTPUT_FILE="$3"
fi

# Validate inputs
if [ ! -f "$CONFIG_FILE" ]; then
    print_error "Configuration file not found: $CONFIG_FILE"
    exit 1
fi

if [ -z "$INVERTER_SERIAL" ]; then
    print_error "Inverter serial number cannot be empty"
    exit 1
fi

# Create output directory if it doesn't exist
OUTPUT_DIR=$(dirname "$OUTPUT_FILE")
if [ ! -d "$OUTPUT_DIR" ] && [ "$OUTPUT_DIR" != "." ]; then
    print_status "Creating output directory: $OUTPUT_DIR"
    mkdir -p "$OUTPUT_DIR"
fi

print_status "🔒 Starting SAFE configuration backup..."
print_status "   This operation is READ-ONLY - no changes will be made to your system"
echo ""

print_status "Configuration file: $CONFIG_FILE"
print_status "Inverter serial: $INVERTER_SERIAL"
print_status "Output file: $OUTPUT_FILE"
echo ""

# Check if the binary exists, if not build it
if [ ! -f "target/debug/lxp-bridge" ]; then
    print_status "Building lxp-bridge binary..."
    cargo build --bin lxp-bridge
    if [ $? -ne 0 ]; then
        print_error "Failed to build lxp-bridge binary"
        exit 1
    fi
    print_success "Binary built successfully"
fi

print_status "Starting configuration backup..."
echo ""

# Run the backup command
./target/debug/lxp-bridge backup-config \
    --config "$CONFIG_FILE" \
    --inverter "$INVERTER_SERIAL" \
    --output "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo ""
    print_success "Configuration backup completed successfully!"
    print_success "Backup saved to: $OUTPUT_FILE"
    echo ""
    print_status "📊 Backup contains:"
    print_status "   - Raw hold register values from your real system"
    print_status "   - Hexadecimal values for protocol validation"
    echo ""
    print_status "🔒 No changes were made to your system"
    print_status "   You can now share this file for analysis"
else
    print_error "Configuration backup failed"
    exit 1
fi
