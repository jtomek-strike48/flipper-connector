//! File Search Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

pub struct FileSearchTool;

#[async_trait]
impl PentestTool for FileSearchTool {
    fn name(&self) -> &str {
        "flipper_file_search"
    }

    fn description(&self) -> &str {
        "Search for files on the Flipper Zero by pattern"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "pattern".to_string(),
                    param_type: ParamType::String,
                    description: "Search pattern (supports wildcards: * for any characters)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "directories".to_string(),
                    param_type: ParamType::Array,
                    description: "Directories to search (default: [\"/ext/nfc\", \"/ext/lfrfid\", \"/ext/subghz\"])".to_string(),
                    required: false,
                    default: Some(json!(["/ext/nfc", "/ext/lfrfid", "/ext/subghz"])),
                },
                ToolParam {
                    name: "extension".to_string(),
                    param_type: ParamType::String,
                    description: "File extension filter (e.g., \".nfc\", \".rfid\", \".sub\")".to_string(),
                    required: false,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let pattern = params["pattern"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing pattern parameter".to_string()))?;

        let directories = if let Some(dirs) = params["directories"].as_array() {
            dirs.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        } else {
            vec![
                "/ext/nfc".to_string(),
                "/ext/lfrfid".to_string(),
                "/ext/subghz".to_string(),
            ]
        };

        let extension = params["extension"].as_str();

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut results = Vec::new();
        let dir_count = directories.len();

        for dir in &directories {
            match client.list_directory(&dir, false).await {
                Ok(items) => {
                    for item in items {
                        let debug_str = format!("{:?}", item);
                        if debug_str.starts_with("File(") {
                            if let Some(name) = extract_filename(&debug_str) {
                                // Apply extension filter
                                if let Some(ext) = extension {
                                    if !name.ends_with(ext) {
                                        continue;
                                    }
                                }

                                // Apply pattern matching
                                if matches_pattern(&name, pattern) {
                                    if let Some(size) = extract_filesize(&debug_str) {
                                        let full_path = format!("{}/{}", dir, name);
                                        results.push(json!({
                                            "path": full_path,
                                            "name": name,
                                            "directory": dir,
                                            "size": size,
                                            "size_human": format_size(size)
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // Directory doesn't exist or is empty, skip
                    continue;
                }
            }
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "results": results,
                "count": results.len(),
                "pattern": pattern,
                "searched_directories": dir_count
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Extract filename from debug string
fn extract_filename(debug_str: &str) -> Option<String> {
    if let Some(start) = debug_str.find("\"") {
        if let Some(end) = debug_str[start + 1..].find("\"") {
            return Some(debug_str[start + 1..start + 1 + end].to_string());
        }
    }
    None
}

/// Extract file size from debug string
fn extract_filesize(debug_str: &str) -> Option<u32> {
    // Format: File("name", size, hash)
    let parts: Vec<&str> = debug_str.split(',').collect();
    if parts.len() >= 2 {
        let size_str = parts[1].trim();
        return size_str.parse().ok();
    }
    None
}

/// Check if filename matches pattern (simple wildcard support)
fn matches_pattern(filename: &str, pattern: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    let pattern_lower = pattern.to_lowercase();

    if pattern_lower == "*" {
        return true;
    }

    if pattern_lower.starts_with('*') && pattern_lower.ends_with('*') {
        // *text* - contains
        let search = &pattern_lower[1..pattern_lower.len() - 1];
        return filename_lower.contains(search);
    } else if pattern_lower.starts_with('*') {
        // *text - ends with
        let search = &pattern_lower[1..];
        return filename_lower.ends_with(search);
    } else if pattern_lower.ends_with('*') {
        // text* - starts with
        let search = &pattern_lower[..pattern_lower.len() - 1];
        return filename_lower.starts_with(search);
    } else {
        // Exact match (case insensitive)
        return filename_lower == pattern_lower;
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
