//! Batch File Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Batch Read Tool ===

pub struct BatchReadTool;

#[async_trait]
impl PentestTool for BatchReadTool {
    fn name(&self) -> &str {
        "flipper_batch_read"
    }

    fn description(&self) -> &str {
        "Read multiple files from the Flipper Zero in a single operation"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "paths".to_string(),
                    param_type: ParamType::Array,
                    description: "Array of file paths to read (e.g., [\"/ext/nfc/card1.nfc\", \"/ext/rfid/badge1.rfid\"])".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "parse".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Parse files based on extension (true) or return raw content (false). Default: true".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let paths = params["paths"]
            .as_array()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing paths array".to_string()))?;

        let parse = params["parse"].as_bool().unwrap_or(true);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut results = Vec::new();
        let mut errors = Vec::new();

        for path_value in paths {
            let path = path_value.as_str().unwrap_or("");
            if path.is_empty() {
                continue;
            }

            match client.read_file(path).await {
                Ok(content) => {
                    match String::from_utf8(content) {
                        Ok(text) => {
                            let parsed = if parse {
                                parse_file_by_extension(path, &text)
                            } else {
                                json!({
                                    "path": path,
                                    "content": text,
                                    "size": text.len()
                                })
                            };

                            results.push(json!({
                                "path": path,
                                "success": true,
                                "data": parsed
                            }));
                        }
                        Err(e) => {
                            errors.push(json!({
                                "path": path,
                                "error": format!("Invalid UTF-8: {}", e)
                            }));
                        }
                    }
                }
                Err(e) => {
                    errors.push(json!({
                        "path": path,
                        "error": e.to_string()
                    }));
                }
            }
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "results": results,
                "errors": errors,
                "total": paths.len(),
                "successful": results.len(),
                "failed": errors.len()
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Parse file content based on file extension
fn parse_file_by_extension(path: &str, content: &str) -> Value {
    if path.ends_with(".nfc") {
        parse_nfc_basic(content)
    } else if path.ends_with(".rfid") {
        parse_rfid_basic(content)
    } else if path.ends_with(".sub") {
        parse_subghz_basic(content)
    } else {
        json!({
            "path": path,
            "content": content,
            "size": content.len(),
            "type": "unknown"
        })
    }
}

/// Basic NFC file parsing (simplified version)
fn parse_nfc_basic(content: &str) -> Value {
    let mut device_type = String::new();
    let mut uid = String::new();

    for line in content.lines() {
        if line.starts_with("Device type:") {
            device_type = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("UID:") {
            uid = line.split(':').nth(1).unwrap_or("").trim().to_string();
        }
    }

    json!({
        "type": "nfc",
        "device_type": device_type,
        "uid": uid
    })
}

/// Basic RFID file parsing (simplified version)
fn parse_rfid_basic(content: &str) -> Value {
    let mut key_type = String::new();
    let mut data = String::new();

    for line in content.lines() {
        if line.starts_with("Key type:") {
            key_type = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("Data:") {
            data = line.split(':').nth(1).unwrap_or("").trim().to_string();
        }
    }

    json!({
        "type": "rfid",
        "key_type": key_type,
        "data": data
    })
}

/// Basic Sub-GHz file parsing (simplified version)
fn parse_subghz_basic(content: &str) -> Value {
    let mut protocol = String::new();
    let mut frequency = 0u32;

    for line in content.lines() {
        if line.starts_with("Protocol:") {
            protocol = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("Frequency:") {
            if let Ok(freq) = line.split(':').nth(1).unwrap_or("").trim().parse() {
                frequency = freq;
            }
        }
    }

    json!({
        "type": "subghz",
        "protocol": protocol,
        "frequency": frequency,
        "frequency_mhz": format!("{:.2} MHz", frequency as f64 / 1_000_000.0)
    })
}
