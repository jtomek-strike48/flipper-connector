//! Device Information Tool

use flipper_core::tools::{PentestTool, Platform, ToolContext, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use async_trait::async_trait;
use serde_json::{json, Value};

/// Tool for getting Flipper Zero device information
pub struct DeviceInfoTool;

#[async_trait]
impl PentestTool for DeviceInfoTool {
    fn name(&self) -> &str {
        "flipper_device_info"
    }

    fn description(&self) -> &str {
        "Get Flipper Zero device information (firmware version, battery, storage)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, _params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        // For now, return basic connection info
        // TODO: Implement actual device info query when we understand the protocol better
        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let port = client.port().to_string();
        let is_connected = client.is_connected();

        // Perform health check
        let health = client.health_check().await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "port": port,
                "connected": is_connected,
                "health_check": health,
                "message": "Device info retrieved successfully"
            }),
            error: None,
            duration_ms: 0, // Will be set by execute_timed if used
        })
    }
}
