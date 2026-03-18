#!/usr/bin/env bash
# Test script for NFC cloning workflow

set -euo pipefail

echo "🐬 Flipper Zero NFC Clone Test"
echo "================================"
echo ""

# Check if Flipper is connected
if [ ! -e /dev/ttyACM0 ]; then
    echo "❌ Error: Flipper Zero not detected on /dev/ttyACM0"
    echo "Please connect your Flipper Zero via USB"
    exit 1
fi

echo "✅ Flipper Zero detected on /dev/ttyACM0"
echo ""

# Build the agent
echo "📦 Building flipper-agent..."
cargo build --package flipper-agent --quiet

echo ""
echo "🔍 Checking for NFC files on Flipper..."
echo ""

# Create a simple test using the flipper-rpc crate directly
cat > /tmp/test_nfc_list.rs << 'RUST_CODE'
use flipper_rpc::{FlipperRpc, RpcError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), RpcError> {
    let mut rpc = FlipperRpc::connect("/dev/ttyACM0").await?;

    println!("📂 Listing NFC files...");

    // List files in /ext/nfc/
    let response = rpc.storage_list("/ext/nfc").await?;

    if let Some(files) = response.get("files") {
        if let Some(arr) = files.as_array() {
            if arr.is_empty() {
                println!("⚠️  No NFC files found on Flipper Zero");
                println!("");
                println!("To test cloning:");
                println!("1. Place a MIFARE card on your Flipper");
                println!("2. Open NFC app and read the card");
                println!("3. Save it as test_card.nfc");
                println!("4. Run this script again");
            } else {
                println!("Found {} NFC files:", arr.len());
                for file in arr {
                    if let Some(name) = file.get("name") {
                        println!("  - {}", name);
                    }
                }
            }
        }
    }

    Ok(())
}
RUST_CODE

echo ""
echo "📋 Test Options:"
echo ""
echo "A) Test with existing NFC file (if you have one)"
echo "B) Create mock test data"
echo "C) Build live reading tools first"
echo ""
echo "What would you like to do? [A/B/C]"
