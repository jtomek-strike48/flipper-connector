//! App Management Tools

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === App List Tool ===

pub struct AppListTool;

#[async_trait]
impl PentestTool for AppListTool {
    fn name(&self) -> &str {
        "flipper_app_list"
    }

    fn description(&self) -> &str {
        "List installed applications on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "category".to_string(),
                    param_type: ParamType::String,
                    description: "App category to list (e.g., 'NFC', 'RFID', 'Sub-GHz'). Leave empty for all categories.".to_string(),
                    required: false,
                    default: Some(json!("")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let category = params["category"].as_str().unwrap_or("");

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let app_categories = if category.is_empty() {
            // List all categories
            vec!["NFC", "RFID", "Sub-GHz", "Infrared", "iButton", "GPIO", "USB", "Bluetooth", "Games", "Tools"]
        } else {
            vec![category]
        };

        let mut all_apps = Vec::new();

        for cat in app_categories {
            let path = format!("/ext/apps/{}", cat);

            match client.list_directory(&path, false).await {
                Ok(items) => {
                    for item in items {
                        // Use Debug format to check item type since we can't import ReadDirItem
                        let debug_str = format!("{:?}", item);
                        if debug_str.starts_with("File(") {
                            // Extract name and size from debug string
                            // Format: File("name.fap", size, md5)
                            if let Some(name_end) = debug_str.find("\",") {
                                let name_start = "File(\"".len();
                                let name = &debug_str[name_start..name_end];

                                if name.ends_with(".fap") {
                                    // Extract size
                                    let size_start = name_end + 3;
                                    if let Some(size_end) = debug_str[size_start..].find(",") {
                                        if let Ok(size) = debug_str[size_start..size_start + size_end].trim().parse::<u32>() {
                                            all_apps.push(json!({
                                                "category": cat,
                                                "name": name,
                                                "size": size,
                                                "size_human": format_size(size),
                                                "path": format!("{}/{}", path, name)
                                            }));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // Category doesn't exist or is empty, skip it
                    continue;
                }
            }
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "apps": all_apps,
                "count": all_apps.len(),
                "category_filter": if category.is_empty() { "all" } else { category }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === App Info Tool ===

pub struct AppInfoTool;

#[async_trait]
impl PentestTool for AppInfoTool {
    fn name(&self) -> &str {
        "flipper_app_info"
    }

    fn description(&self) -> &str {
        "Get information about a specific Flipper Zero app"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path to the .fap file (e.g., /ext/apps/NFC/nfc.fap)".to_string(),
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

        // Get file metadata (size)
        let size = client.get_metadata(path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Extract app name and category from path
        let parts: Vec<&str> = path.split('/').collect();
        let app_name = parts.last().unwrap_or(&"unknown");
        let category = if parts.len() >= 4 {
            parts[parts.len() - 2]
        } else {
            "unknown"
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "name": app_name,
                "category": category,
                "size": size,
                "size_human": format_size(size),
                "type": "Flipper Application Package (.fap)"
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
