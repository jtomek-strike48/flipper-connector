//! Flipper Zero Connector — Headless Agent
//!
//! This binary creates the Flipper Zero connector and demonstrates
//! its capabilities by running the available tools.

use flipper_core::connector::FlipperConnector;
use flipper_tools::create_tool_registry;
use serde_json::json;
use strike48_connector::BaseConnector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    flipper_core::logging::init_logging("info");

    tracing::info!("🐬 flipper-agent starting");
    tracing::info!("Flipper Zero Connector v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("");

    // Create tool registry with all available tools
    let tools = create_tool_registry();
    tracing::info!("✅ Registered {} tools", tools.tools().len());

    // Create the connector
    let connector = FlipperConnector::new(tools);
    tracing::info!("✅ Connector created: {}", connector.connector_type());
    tracing::info!("");

    // List all capabilities
    let capabilities = connector.capabilities();
    tracing::info!("📋 Available tools:");
    for cap in &capabilities {
        tracing::info!("  • {} - {}", cap.task_type_id, cap.description);
    }
    tracing::info!("");

    // Run a simple test: device_info
    tracing::info!("🧪 Testing flipper_device_info tool...");
    match run_test_tool(&connector, "flipper_device_info", json!({})).await {
        Ok(result) => {
            tracing::info!("✅ Test successful!");
            tracing::info!("Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            tracing::error!("❌ Test failed: {}", e);
        }
    }

    tracing::info!("");
    tracing::info!("✨ flipper-agent demo complete!");
    tracing::info!("The connector is ready to be integrated with Prospector Studio.");

    Ok(())
}

/// Helper function to test a tool
async fn run_test_tool(
    connector: &FlipperConnector,
    tool_name: &str,
    params: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let request = json!({
        "tool": tool_name,
        "parameters": params
    });

    connector
        .execute(request, None)
        .await
        .map_err(|e| anyhow::anyhow!("Tool execution failed: {}", e))
}
