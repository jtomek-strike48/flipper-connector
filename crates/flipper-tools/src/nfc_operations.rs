//! NFC File Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};
use std::collections::HashMap;

// === NFC File Read Tool ===

pub struct NfcReadTool;

#[async_trait]
impl PentestTool for NfcReadTool {
    fn name(&self) -> &str {
        "flipper_nfc_read"
    }

    fn description(&self) -> &str {
        "Read and parse NFC file from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path to .nfc file (e.g., /ext/nfc/card.nfc)".to_string(),
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

        // Read file content
        let content = client.read_file(path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let text = String::from_utf8(content)
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Invalid UTF-8: {}", e)))?;

        // Parse NFC file
        let parsed = parse_nfc_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        Ok(ToolResult {
            success: true,
            data: parsed,
            error: None,
            duration_ms: 0,
        })
    }
}

/// Parse NFC file content into structured data
fn parse_nfc_file(content: &str) -> Result<Value, String> {
    let mut result = HashMap::new();
    let mut device_type = String::new();
    let mut blocks = Vec::new();
    let mut pages = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse key-value pairs
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "Filetype" => result.insert("filetype".to_string(), json!(value)),
                "Version" => result.insert("version".to_string(), json!(value.parse::<u32>().unwrap_or(0))),
                "Device type" => {
                    device_type = value.to_string();
                    result.insert("device_type".to_string(), json!(value))
                }
                "UID" => result.insert("uid".to_string(), json!(value)),
                "ATQA" => result.insert("atqa".to_string(), json!(value)),
                "SAK" => result.insert("sak".to_string(), json!(value)),

                // Bank Card fields
                "AID" => result.insert("aid".to_string(), json!(value)),
                "Name" => result.insert("name".to_string(), json!(value)),
                "Number" => result.insert("number".to_string(), json!(value)),

                // MIFARE Classic fields
                "Mifare Classic type" => result.insert("mifare_type".to_string(), json!(value)),

                // NTAG/Ultralight fields
                "Data format version" => result.insert("data_format_version".to_string(), json!(value.parse::<u32>().unwrap_or(0))),
                "Signature" => result.insert("signature".to_string(), json!(value)),
                "Mifare version" => result.insert("mifare_version".to_string(), json!(value)),
                "Pages total" => result.insert("pages_total".to_string(), json!(value.parse::<u32>().unwrap_or(0))),
                "Pages read" => result.insert("pages_read".to_string(), json!(value.parse::<u32>().unwrap_or(0))),
                "Failed authentication attempts" => result.insert("failed_auth_attempts".to_string(), json!(value.parse::<u32>().unwrap_or(0))),

                // Handle blocks and pages
                _ if key.starts_with("Block ") => {
                    if let Some(num_str) = key.strip_prefix("Block ") {
                        if let Ok(num) = num_str.parse::<u32>() {
                            blocks.push(json!({
                                "number": num,
                                "data": value
                            }));
                        }
                    }
                    None
                }
                _ if key.starts_with("Page ") => {
                    if let Some(num_str) = key.strip_prefix("Page ") {
                        if let Ok(num) = num_str.parse::<u32>() {
                            pages.push(json!({
                                "number": num,
                                "data": value
                            }));
                        }
                    }
                    None
                }
                _ if key.starts_with("Counter ") || key.starts_with("Tearing ") => {
                    // Skip counters and tearing flags for now
                    None
                }
                _ => None
            };
        }
    }

    // Add blocks/pages if present
    if !blocks.is_empty() {
        result.insert("blocks".to_string(), json!(blocks));
        result.insert("block_count".to_string(), json!(blocks.len()));
    }
    if !pages.is_empty() {
        result.insert("pages".to_string(), json!(pages));
        result.insert("page_count".to_string(), json!(pages.len()));
    }

    // Add summary
    result.insert("format".to_string(), json!(classify_nfc_format(&device_type)));

    Ok(json!(result))
}

/// Classify NFC format for easier handling
fn classify_nfc_format(device_type: &str) -> &'static str {
    match device_type {
        "Bank card" => "bank_card",
        "Mifare Classic" => "mifare_classic",
        "NTAG203" | "NTAG213" | "NTAG215" | "NTAG216" => "ntag",
        "Mifare Ultralight" => "mifare_ultralight",
        "UID" => "uid_only",
        _ => "unknown"
    }
}

// === NFC File Write Tool ===

pub struct NfcWriteTool;

#[async_trait]
impl PentestTool for NfcWriteTool {
    fn name(&self) -> &str {
        "flipper_nfc_write"
    }

    fn description(&self) -> &str {
        "Create an NFC file on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path for new .nfc file (e.g., /ext/nfc/new_card.nfc)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "device_type".to_string(),
                    param_type: ParamType::String,
                    description: "Device type: 'UID', 'Bank card', 'Mifare Classic', 'NTAG203', etc.".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "uid".to_string(),
                    param_type: ParamType::String,
                    description: "UID in hex format (e.g., '04 4A 98 B2')".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "atqa".to_string(),
                    param_type: ParamType::String,
                    description: "ATQA in hex format (default: '44 00')".to_string(),
                    required: false,
                    default: Some(json!("44 00")),
                },
                ToolParam {
                    name: "sak".to_string(),
                    param_type: ParamType::String,
                    description: "SAK in hex format (default: '00')".to_string(),
                    required: false,
                    default: Some(json!("00")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let device_type = params["device_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing device_type parameter".to_string()))?;

        let uid = params["uid"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing uid parameter".to_string()))?;

        let atqa = params["atqa"].as_str().unwrap_or("44 00");
        let sak = params["sak"].as_str().unwrap_or("00");

        // Generate NFC file content
        let content = generate_nfc_file(device_type, uid, atqa, sak)?;
        let content_size = content.len();

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Write file
        client.write_file(path, content.into_bytes()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "device_type": device_type,
                "uid": uid,
                "size": content_size
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Generate NFC file content
fn generate_nfc_file(device_type: &str, uid: &str, atqa: &str, sak: &str) -> Result<String, flipper_core::error::Error> {
    let mut content = String::new();

    content.push_str("Filetype: Flipper NFC device\n");
    content.push_str("Version: 2\n");
    content.push_str("# Generated by Flipper Zero Connector\n");
    content.push_str(&format!("Device type: {}\n", device_type));
    content.push_str("# UID, ATQA and SAK are common for all formats\n");
    content.push_str(&format!("UID: {}\n", uid));
    content.push_str(&format!("ATQA: {}\n", atqa));
    content.push_str(&format!("SAK: {}\n", sak));

    Ok(content)
}
