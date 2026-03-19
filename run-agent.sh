#!/usr/bin/env bash
# Start the Flipper Zero Connector Agent for Prospector Studio
#
# This script connects your Flipper Zero to Prospector Studio, exposing
# all 108 tools for AI-driven physical security testing.

set -euo pipefail

# Configuration (based on ~/Code/pick/.env)
# Note: STRIKE48_HOST can be:
#   - wss://host (WebSocket with TLS)
#   - host:port (gRPC)
export STRIKE48_HOST="${STRIKE48_HOST:-wss://jt-demo-01.strike48.engineering}"
export TENANT_ID="${TENANT_ID:-non-prod}"
export RUST_LOG="${RUST_LOG:-info}"

echo "🐬 Starting Flipper Zero Connector Agent"
echo ""
echo "Configuration:"
echo "  STRIKE48_HOST: $STRIKE48_HOST"
echo "  TENANT_ID: $TENANT_ID"
echo "  RUST_LOG: $RUST_LOG"
echo ""
echo "The agent will connect to Prospector Studio and register 108 tools:"
echo "  • NFC (7) - MIFARE cracking, cloning, emulation"
echo "  • RFID (3) - Low frequency operations"
echo "  • Sub-GHz (4) - RF capture & bruteforce"
echo "  • BadUSB/BadKB (7) - USB & Bluetooth HID attacks"
echo "  • And 87 more tools across 16 additional categories"
echo ""
echo "Press Ctrl+C to stop the agent"
echo ""

# Run the agent
exec cargo run --package flipper-agent --release
