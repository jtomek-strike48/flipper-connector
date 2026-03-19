//! Flipper Zero Connector — Headless Agent
//!
//! This binary connects the Flipper Zero to Prospector Studio via the Strike48 SDK.
//! It runs as a long-lived service, handling tool execution requests from the platform.
//!
//! Configuration via environment variables:
//!   - STRIKE48_HOST: Prospector Studio address (default: localhost:50061)
//!   - TENANT_ID: Tenant identifier (default: default)
//!   - RUST_LOG: Log level (default: info)

use flipper_core::connector::FlipperConnector;
use flipper_tools::create_tool_registry;
use strike48_connector::{BaseConnector, ConnectorConfig, ConnectorRunner};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    flipper_core::logging::init_logging("info");

    tracing::info!("🐬 Flipper Zero Connector Agent");
    tracing::info!("   Version: {}", env!("CARGO_PKG_VERSION"));
    tracing::info!("");

    // Create tool registry with all available tools
    let tools = create_tool_registry();
    tracing::info!("✅ Registered {} tools across 20 categories", tools.tools().len());

    // Create the connector
    let connector = FlipperConnector::new(tools);
    tracing::info!("✅ Connector type: {}", connector.connector_type());
    tracing::info!("");

    // List all capabilities
    let capabilities = connector.capabilities();
    tracing::info!("📋 Available tool categories:");
    tracing::info!("   • NFC (7 tools) - MIFARE cracking, cloning, emulation");
    tracing::info!("   • RFID (3 tools) - Low frequency card operations");
    tracing::info!("   • Sub-GHz (4 tools) - RF protocol capture & bruteforce");
    tracing::info!("   • BadUSB/BadKB (7 tools) - USB & Bluetooth HID attacks");
    tracing::info!("   • iButton, IR, GPIO, BLE, U2F, Zigbee, and more...");
    tracing::info!("");

    tracing::info!("🚀 Connecting to Prospector Studio...");
    tracing::info!("   STRIKE48_HOST: {}", std::env::var("STRIKE48_HOST").unwrap_or_else(|_| "localhost:50061".to_string()));
    tracing::info!("   TENANT_ID: {}", std::env::var("TENANT_ID").unwrap_or_else(|_| "default".to_string()));
    tracing::info!("");

    // Load configuration from environment
    let config = ConnectorConfig::from_env();

    // Create runner and run the connector - this will block until shutdown signal
    let runner = ConnectorRunner::new(config, Arc::new(connector));
    runner.run().await?;

    tracing::info!("👋 Flipper Zero Connector shutting down");
    Ok(())
}
