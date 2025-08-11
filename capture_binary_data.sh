#!/bin/bash

# Script to capture binary data from real LXP system
# This will help you see what the decode looks like

echo "=== LXP Bridge Binary Data Capture Script ==="
echo "This script will help you capture and analyze binary data from your real system"
echo ""

# Check if config file exists
if [ ! -f "config_real_system_debug.yaml" ]; then
    echo "Error: config_real_system_debug.yaml not found!"
    echo "Please make sure the configuration file exists and points to your real system."
    exit 1
fi

echo "Configuration file found: config_real_system_debug.yaml"
echo ""

# Check if MQTT broker is running
echo "Checking MQTT broker status..."
if ! pgrep -x "mosquitto" > /dev/null; then
    echo "Warning: Mosquitto MQTT broker is not running"
    echo "You may need to start it with: brew services start mosquitto"
    echo ""
else
    echo "✓ Mosquitto MQTT broker is running"
fi

echo ""
echo "=== Instructions ==="
echo "1. Make sure your real system is accessible at 192.168.3.45:8000"
echo "2. Run the LXP bridge with debug logging:"
echo "   cargo run -- -c config_real_system_debug.yaml"
echo ""
echo "3. The debug logs will show:"
echo "   - Raw binary data in hex format"
echo "   - Raw binary data in binary format"
echo "   - Decoded register values"
echo "   - MQTT messages being published"
echo ""
echo "4. Look for log lines like:"
echo "   'Raw binary data (hex): [XX XX XX ...]'"
echo "   'Raw binary data (binary): [XXXXXXXX ...]'"
echo "   'inverter BA31100197: RX packet: ...'"
echo "   'inverter BA31100197: TX hex: [XX XX XX ...]'"
echo ""
echo "5. To capture specific data, you can:"
echo "   - Read specific registers via MQTT commands"
echo "   - Monitor the logs for incoming data"
echo "   - Check MQTT topics for decoded values"
echo ""
echo "=== MQTT Commands to Test ==="
echo "Read holding register 12:"
echo "  mosquitto_pub -t 'cmd/BA31100197/read/hold/12' -m ''"
echo ""
echo "Read input registers:"
echo "  mosquitto_pub -t 'cmd/BA31100197/read/inputs/1' -m ''"
echo ""
echo "Read all holding registers:"
echo "  mosquitto_pub -t 'cmd/BA31100197/read/hold/0' -m '280'"
echo ""
echo "=== Monitoring MQTT Messages ==="
echo "Subscribe to all messages:"
echo "  mosquitto_sub -t 'BA31100197/#' -v"
echo ""
echo "Subscribe to specific topics:"
echo "  mosquitto_sub -t 'BA31100197/hold/12' -v"
echo "  mosquitto_sub -t 'BA31100197/inputs/all' -v"
echo ""
echo "=== Expected Output ==="
echo "When reading register 12, you should see:"
echo "  - Binary data in hex: [XX XX XX ...]"
echo "  - Binary data in binary: [XXXXXXXX ...]"
echo "  - Decoded value: 2073 (or current value)"
echo "  - MQTT message: BA31100197/hold/12 = 2073"
echo ""
echo "Ready to capture data! Run 'cargo run -- -c config_real_system_debug.yaml'"
