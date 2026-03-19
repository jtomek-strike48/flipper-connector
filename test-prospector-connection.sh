#!/usr/bin/env bash
# Test connection to Prospector Studio
#
# This script verifies that the Flipper Zero connector can connect
# to your Prospector Studio instance.

set -euo pipefail

# Load configuration from ~/Code/pick/.env if available
if [ -f ~/Code/pick/.env ]; then
    echo "📋 Loading configuration from ~/Code/pick/.env"
    # shellcheck disable=SC1090
    source <(grep -E '^(STRIKE48_HOST|STRIKE48_TENANT|RUST_LOG)=' ~/Code/pick/.env | sed 's/^/export /')

    # Map STRIKE48_TENANT to TENANT_ID
    if [ -n "${STRIKE48_TENANT:-}" ]; then
        export TENANT_ID="$STRIKE48_TENANT"
    fi
else
    echo "⚠️  ~/Code/pick/.env not found, using defaults"
    export STRIKE48_HOST="${STRIKE48_HOST:-wss://jt-demo-01.strike48.engineering}"
    export TENANT_ID="${TENANT_ID:-non-prod}"
    export RUST_LOG="${RUST_LOG:-info}"
fi

echo ""
echo "🔧 Configuration:"
echo "   STRIKE48_HOST: $STRIKE48_HOST"
echo "   TENANT_ID: $TENANT_ID"
echo "   RUST_LOG: $RUST_LOG"
echo ""

# Check if Flipper is connected
if [ -e /dev/ttyACM0 ]; then
    echo "✅ Flipper Zero detected at /dev/ttyACM0"
else
    echo "⚠️  Flipper Zero not detected at /dev/ttyACM0"
    echo "   Make sure your Flipper is connected via USB"
fi
echo ""

echo "🚀 Starting Flipper Zero Connector..."
echo "   Press Ctrl+C to stop"
echo ""

# Run the agent
exec cargo run --package flipper-agent --release
