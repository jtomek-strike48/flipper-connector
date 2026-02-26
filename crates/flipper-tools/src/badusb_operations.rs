//! BadUSB Operations
//!
//! BadUSB allows Flipper Zero to emulate a USB keyboard/mouse for penetration testing.
//! Scripts use Ducky Script syntax and are stored as .txt files in /ext/badusb/

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

/// Default BadUSB directory
pub const BADUSB_DIR: &str = "/ext/badusb";

// ============================================================================
// BadUSB Script Upload Tool
// ============================================================================

pub struct BadUsbUploadTool;

#[async_trait]
impl PentestTool for BadUsbUploadTool {
    fn name(&self) -> &str {
        "flipper_badusb_upload"
    }

    fn description(&self) -> &str {
        "Upload a BadUSB Ducky Script to the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "filename".to_string(),
                    param_type: ParamType::String,
                    description: "Script filename (without .txt extension)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "script".to_string(),
                    param_type: ParamType::String,
                    description: "Ducky Script content (multiline string)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "validate".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Validate Ducky Script syntax before upload".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let filename = params["filename"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing filename parameter".to_string()))?;

        let script = params["script"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing script parameter".to_string()))?;

        let validate = params["validate"].as_bool().unwrap_or(true);

        // Validate script if requested
        if validate {
            if let Err(validation_error) = validate_ducky_script(script) {
                return Ok(ToolResult {
                    success: false,
                    data: json!(null),
                    error: Some(format!("Script validation failed: {}", validation_error)),
                    duration_ms: 0,
                });
            }
        }

        // Ensure filename ends with .txt
        let full_filename = if filename.ends_with(".txt") {
            filename.to_string()
        } else {
            format!("{}.txt", filename)
        };

        let path = format!("{}/{}", BADUSB_DIR, full_filename);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Upload script
        client.write_file(&path, script.as_bytes().to_vec()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let line_count = script.lines().count();
        let size = script.len();

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "filename": full_filename,
                "size": size,
                "lines": line_count,
                "validated": validate
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// ============================================================================
// BadUSB Script List Tool
// ============================================================================

pub struct BadUsbListTool;

#[async_trait]
impl PentestTool for BadUsbListTool {
    fn name(&self) -> &str {
        "flipper_badusb_list"
    }

    fn description(&self) -> &str {
        "List all BadUSB scripts on the Flipper Zero"
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
        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let items = client.list_directory(BADUSB_DIR, false).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut scripts = Vec::new();

        for item in items {
            let debug_str = format!("{:?}", item);
            if debug_str.starts_with("File(") {
                if let Some(name) = extract_filename(&debug_str) {
                    if name.ends_with(".txt") {
                        if let Some(size) = extract_filesize(&debug_str) {
                            scripts.push(json!({
                                "name": name,
                                "path": format!("{}/{}", BADUSB_DIR, name),
                                "size": size,
                                "size_human": format_size(size)
                            }));
                        }
                    }
                }
            }
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "scripts": scripts,
                "count": scripts.len(),
                "directory": BADUSB_DIR
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// ============================================================================
// BadUSB Script Read Tool
// ============================================================================

pub struct BadUsbReadTool;

#[async_trait]
impl PentestTool for BadUsbReadTool {
    fn name(&self) -> &str {
        "flipper_badusb_read"
    }

    fn description(&self) -> &str {
        "Read and parse a BadUSB script from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "filename".to_string(),
                    param_type: ParamType::String,
                    description: "Script filename (with or without .txt extension)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let filename = params["filename"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing filename parameter".to_string()))?;

        // Ensure filename ends with .txt
        let full_filename = if filename.ends_with(".txt") {
            filename.to_string()
        } else {
            format!("{}.txt", filename)
        };

        let path = format!("{}/{}", BADUSB_DIR, full_filename);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let content = client.read_file(&path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let script = String::from_utf8(content)
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Invalid UTF-8: {}", e)))?;

        let analysis = analyze_ducky_script(&script);

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "filename": full_filename,
                "script": script,
                "analysis": analysis
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// ============================================================================
// BadUSB Script Delete Tool
// ============================================================================

pub struct BadUsbDeleteTool;

#[async_trait]
impl PentestTool for BadUsbDeleteTool {
    fn name(&self) -> &str {
        "flipper_badusb_delete"
    }

    fn description(&self) -> &str {
        "Delete a BadUSB script from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "filename".to_string(),
                    param_type: ParamType::String,
                    description: "Script filename (with or without .txt extension)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let filename = params["filename"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing filename parameter".to_string()))?;

        // Ensure filename ends with .txt
        let full_filename = if filename.ends_with(".txt") {
            filename.to_string()
        } else {
            format!("{}.txt", filename)
        };

        let path = format!("{}/{}", BADUSB_DIR, full_filename);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        client.delete_path(&path, false).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "deleted": path,
                "filename": full_filename
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// ============================================================================
// BadUSB Script Validate Tool
// ============================================================================

pub struct BadUsbValidateTool;

#[async_trait]
impl PentestTool for BadUsbValidateTool {
    fn name(&self) -> &str {
        "flipper_badusb_validate"
    }

    fn description(&self) -> &str {
        "Validate Ducky Script syntax without uploading"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "script".to_string(),
                    param_type: ParamType::String,
                    description: "Ducky Script content to validate".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let script = params["script"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing script parameter".to_string()))?;

        match validate_ducky_script(script) {
            Ok(_) => {
                let analysis = analyze_ducky_script(script);
                Ok(ToolResult {
                    success: true,
                    data: json!({
                        "valid": true,
                        "analysis": analysis
                    }),
                    error: None,
                    duration_ms: 0,
                })
            }
            Err(error) => {
                Ok(ToolResult {
                    success: false,
                    data: json!({
                        "valid": false,
                        "error": error
                    }),
                    error: Some(error),
                    duration_ms: 0,
                })
            }
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Validate Ducky Script syntax
fn validate_ducky_script(script: &str) -> Result<(), String> {
    let valid_commands = [
        "REM", "DELAY", "DEFAULT_DELAY", "DEFAULTDELAY",
        "STRING", "STRINGLN",
        "ENTER", "SPACE", "TAB", "ESCAPE", "ESC", "BACKSPACE",
        "CAPSLOCK", "DELETE", "END", "HOME", "INSERT", "PAGEUP", "PAGEDOWN",
        "PRINTSCREEN", "SCROLLLOCK", "NUMLOCK",
        "UP", "DOWN", "LEFT", "RIGHT", "UPARROW", "DOWNARROW", "LEFTARROW", "RIGHTARROW",
        "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
        "GUI", "WINDOWS", "COMMAND", "CTRL", "CONTROL", "SHIFT", "ALT", "OPTION",
        "CTRL-ALT", "CTRL-SHIFT", "ALT-SHIFT", "ALT-TAB",
    ];

    for (line_num, line) in script.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("REM ") {
            continue;
        }

        // Check for valid command
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_uppercase();

        // Special handling for STRING and DELAY
        if command == "STRING" || command == "STRINGLN" {
            continue; // Any text after STRING is valid
        }

        if command == "DELAY" || command == "DEFAULT_DELAY" || command == "DEFAULTDELAY" {
            if parts.len() < 2 {
                return Err(format!("Line {}: {} requires a duration", line_num + 1, command));
            }
            if parts[1].parse::<u32>().is_err() {
                return Err(format!("Line {}: Invalid delay value '{}'", line_num + 1, parts[1]));
            }
            continue;
        }

        // Check if command is valid
        if !valid_commands.contains(&command.as_str()) {
            return Err(format!("Line {}: Unknown command '{}'", line_num + 1, command));
        }
    }

    Ok(())
}

/// Analyze Ducky Script and extract statistics
fn analyze_ducky_script(script: &str) -> Value {
    let mut total_lines = 0;
    let mut command_lines = 0;
    let mut comment_lines = 0;
    let mut empty_lines = 0;
    let mut total_delay_ms = 0;
    let mut commands_used = std::collections::HashSet::new();

    for line in script.lines() {
        total_lines += 1;
        let line = line.trim();

        if line.is_empty() {
            empty_lines += 1;
            continue;
        }

        if line.starts_with("REM ") {
            comment_lines += 1;
            continue;
        }

        command_lines += 1;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if !parts.is_empty() {
            let command = parts[0].to_uppercase();
            commands_used.insert(command.clone());

            // Track delays
            if command == "DELAY" && parts.len() > 1 {
                if let Ok(delay) = parts[1].parse::<u32>() {
                    total_delay_ms += delay;
                }
            }
        }
    }

    let commands_list: Vec<String> = commands_used.into_iter().collect();

    json!({
        "total_lines": total_lines,
        "command_lines": command_lines,
        "comment_lines": comment_lines,
        "empty_lines": empty_lines,
        "total_delay_ms": total_delay_ms,
        "estimated_duration_sec": total_delay_ms as f64 / 1000.0,
        "commands_used": commands_list,
        "command_count": commands_list.len()
    })
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
    let parts: Vec<&str> = debug_str.split(',').collect();
    if parts.len() >= 2 {
        let size_str = parts[1].trim();
        return size_str.parse().ok();
    }
    None
}

/// Format size in human-readable form
fn format_size(bytes: u32) -> String {
    const KB: u32 = 1024;

    if bytes == 0 {
        "0 B".to_string()
    } else if bytes < KB {
        format!("{} B", bytes)
    } else {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    }
}
