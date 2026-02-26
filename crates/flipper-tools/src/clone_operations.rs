//! Clone and Generate Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === NFC Clone Tool ===

pub struct NfcCloneTool;

#[async_trait]
impl PentestTool for NfcCloneTool {
    fn name(&self) -> &str {
        "flipper_nfc_clone"
    }

    fn description(&self) -> &str {
        "Clone an NFC file with optional UID modification"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "source_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to source .nfc file to clone".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "dest_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path for cloned .nfc file".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "new_uid".to_string(),
                    param_type: ParamType::String,
                    description: "New UID in hex format (optional, keeps original if not specified)".to_string(),
                    required: false,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let source_path = params["source_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing source_path parameter".to_string()))?;

        let dest_path = params["dest_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing dest_path parameter".to_string()))?;

        let new_uid = params["new_uid"].as_str();

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Read source file
        let content = client.read_file(source_path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut text = String::from_utf8(content)
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Invalid UTF-8: {}", e)))?;

        // Modify UID if requested
        let mut original_uid = String::new();
        if let Some(uid) = new_uid {
            if let Some(uid_line_start) = text.find("UID:") {
                if let Some(newline) = text[uid_line_start..].find('\n') {
                    let uid_line_end = uid_line_start + newline;
                    let old_line = &text[uid_line_start..uid_line_end];
                    original_uid = old_line.split(':').nth(1).unwrap_or("").trim().to_string();
                    let new_line = format!("UID: {}", uid);
                    text.replace_range(uid_line_start..uid_line_end, &new_line);
                }
            }
        }

        // Write cloned file
        client.write_file(dest_path, text.clone().into_bytes()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "source_path": source_path,
                "dest_path": dest_path,
                "original_uid": original_uid,
                "new_uid": new_uid.unwrap_or("unchanged"),
                "size": text.len()
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === RFID Generator Tool ===

pub struct RfidGenerateTool;

#[async_trait]
impl PentestTool for RfidGenerateTool {
    fn name(&self) -> &str {
        "flipper_rfid_generate"
    }

    fn description(&self) -> &str {
        "Generate sequential RFID badges for testing"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "base_path".to_string(),
                    param_type: ParamType::String,
                    description: "Base path for generated files (e.g., \"/ext/lfrfid/test_badge\")".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "facility_code".to_string(),
                    param_type: ParamType::Number,
                    description: "Facility code (0-255) for H10301 format".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "start_card".to_string(),
                    param_type: ParamType::Number,
                    description: "Starting card number (0-65535)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "count".to_string(),
                    param_type: ParamType::Number,
                    description: "Number of badges to generate (1-100)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let base_path = params["base_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing base_path parameter".to_string()))?;

        let facility = params["facility_code"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing facility_code parameter".to_string()))? as u8;

        let start_card = params["start_card"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing start_card parameter".to_string()))? as u16;

        let count = params["count"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing count parameter".to_string()))? as u16;

        if count == 0 || count > 100 {
            return Err(flipper_core::error::Error::InvalidParams("Count must be between 1 and 100".to_string()));
        }

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut generated = Vec::new();

        for i in 0..count {
            let card_number = start_card.wrapping_add(i);
            let filename = format!("{}_{:05}.rfid", base_path, card_number);

            // Encode H10301
            let data = encode_h10301(facility, card_number)?;

            // Generate file content
            let content = format!(
                "Filetype: Flipper RFID key\nVersion: 1\n# Generated by Flipper Zero Connector\nKey type: H10301\n# Data size for EM4100 is 5, for H10301 is 3, for I40134 is 3\nData: {}\n",
                data
            );

            // Write file
            client.write_file(&filename, content.into_bytes()).await
                .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

            generated.push(json!({
                "path": filename,
                "facility_code": facility,
                "card_number": card_number
            }));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "generated": generated,
                "count": count,
                "facility_code": facility,
                "card_range": format!("{} - {}", start_card, start_card + count - 1)
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Encode H10301 26-bit Wiegand format
fn encode_h10301(facility: u8, card: u16) -> Result<String, flipper_core::error::Error> {
    let mut value: u32 = 0;
    value |= (facility as u32) << 17;
    value |= (card as u32) << 1;

    let byte0 = ((value >> 16) & 0xFF) as u8;
    let byte1 = ((value >> 8) & 0xFF) as u8;
    let byte2 = (value & 0xFF) as u8;

    Ok(format!("{:02X} {:02X} {:02X}", byte0, byte1, byte2))
}
