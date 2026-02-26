//! Directory Operation Tools

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Directory Create Tool ===

pub struct DirCreateTool;

#[async_trait]
impl PentestTool for DirCreateTool {
    fn name(&self) -> &str {
        "flipper_dir_create"
    }

    fn description(&self) -> &str {
        "Create a directory on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Directory path to create (e.g., /ext/my_folder)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let created = client.create_directory(path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "created": created,
                "message": if created { "Directory created successfully" } else { "Directory already exists" }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === File/Directory Stat Tool ===

pub struct FileStatTool;

#[async_trait]
impl PentestTool for FileStatTool {
    fn name(&self) -> &str {
        "flipper_file_stat"
    }

    fn description(&self) -> &str {
        "Get file or directory metadata from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "File or directory path (e.g., /ext/test.txt or /ext/nfc)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let size = client.get_metadata(path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Determine if it's a file or directory based on size
        // Directories typically return 0
        let item_type = if size == 0 {
            // Could be an empty file or a directory, we can't tell for sure
            // without additional info from the API
            "unknown (likely directory or empty file)"
        } else {
            "file"
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "size": size,
                "type": item_type,
                "size_human": format_size(size)
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Format size in human-readable form
fn format_size(bytes: u32) -> String {
    const KB: u32 = 1024;
    const MB: u32 = 1024 * 1024;

    if bytes == 0 {
        "0 B".to_string()
    } else if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    }
}
