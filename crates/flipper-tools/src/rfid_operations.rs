//! RFID File Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === RFID File Read Tool ===

pub struct RfidReadTool;

#[async_trait]
impl PentestTool for RfidReadTool {
    fn name(&self) -> &str {
        "flipper_rfid_read"
    }

    fn description(&self) -> &str {
        "Read and parse RFID file from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path to .rfid file (e.g., /ext/lfrfid/badge.rfid)".to_string(),
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

        // Parse RFID file
        let parsed = parse_rfid_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        Ok(ToolResult {
            success: true,
            data: parsed,
            error: None,
            duration_ms: 0,
        })
    }
}

/// Parse RFID file content into structured data
fn parse_rfid_file(content: &str) -> Result<Value, String> {
    let mut key_type = String::new();
    let mut data = String::new();

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
                "Key type" => key_type = value.to_string(),
                "Data" => data = value.to_string(),
                _ => {}
            }
        }
    }

    if key_type.is_empty() || data.is_empty() {
        return Err("Invalid RFID file format".to_string());
    }

    let mut result = json!({
        "key_type": key_type,
        "data": data,
    });

    // Decode H10301 Wiegand format
    if key_type == "H10301" {
        if let Ok(decoded) = decode_h10301(&data) {
            result["facility_code"] = json!(decoded.0);
            result["card_number"] = json!(decoded.1);
            result["decoded"] = json!(format!("Facility: {}, Card: {}", decoded.0, decoded.1));
        }
    }

    Ok(result)
}

/// Decode H10301 26-bit Wiegand format
/// Returns (facility_code, card_number)
fn decode_h10301(data_hex: &str) -> Result<(u8, u16), String> {
    // Parse hex bytes
    let bytes: Result<Vec<u8>, _> = data_hex
        .split_whitespace()
        .map(|b| u8::from_str_radix(b, 16))
        .collect();

    let bytes = bytes.map_err(|e| format!("Invalid hex: {}", e))?;

    if bytes.len() != 3 {
        return Err("H10301 requires 3 bytes".to_string());
    }

    // Convert to 24-bit value
    let value = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);

    // Extract 26-bit Wiegand (ignore top 6 bits, bottom 2 bits)
    // Format: [6 pad][1 even parity][8 facility][16 card][1 odd parity][2 pad]
    let facility = ((value >> 17) & 0xFF) as u8;
    let card = ((value >> 1) & 0xFFFF) as u16;

    Ok((facility, card))
}

// === RFID File Write Tool ===

pub struct RfidWriteTool;

#[async_trait]
impl PentestTool for RfidWriteTool {
    fn name(&self) -> &str {
        "flipper_rfid_write"
    }

    fn description(&self) -> &str {
        "Create an RFID file on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path for new .rfid file (e.g., /ext/lfrfid/badge.rfid)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "key_type".to_string(),
                    param_type: ParamType::String,
                    description: "Key type: 'EM4100', 'H10301', or 'I40134'".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "data".to_string(),
                    param_type: ParamType::String,
                    description: "Data in hex format (e.g., '1C 69 CE' for H10301) - OR use facility_code/card_number for H10301".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "facility_code".to_string(),
                    param_type: ParamType::Number,
                    description: "Facility code (0-255) for H10301 format - alternative to data parameter".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "card_number".to_string(),
                    param_type: ParamType::Number,
                    description: "Card number (0-65535) for H10301 format - alternative to data parameter".to_string(),
                    required: false,
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

        let key_type = params["key_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing key_type parameter".to_string()))?;

        // Get data - either direct hex or from facility/card for H10301
        let data = if let Some(data_str) = params["data"].as_str() {
            data_str.to_string()
        } else if key_type == "H10301" {
            // Try to encode from facility/card
            let facility = params["facility_code"]
                .as_u64()
                .ok_or_else(|| flipper_core::error::Error::InvalidParams(
                    "H10301 requires either 'data' or 'facility_code' and 'card_number'".to_string()
                ))? as u8;

            let card = params["card_number"]
                .as_u64()
                .ok_or_else(|| flipper_core::error::Error::InvalidParams(
                    "H10301 requires both 'facility_code' and 'card_number'".to_string()
                ))? as u16;

            encode_h10301(facility, card)?
        } else {
            return Err(flipper_core::error::Error::InvalidParams("Missing data parameter".to_string()));
        };

        // Generate RFID file content
        let content = generate_rfid_file(key_type, &data);
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
                "key_type": key_type,
                "data": data,
                "size": content_size
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Generate RFID file content
fn generate_rfid_file(key_type: &str, data: &str) -> String {
    let mut content = String::new();

    content.push_str("Filetype: Flipper RFID key\n");
    content.push_str("Version: 1\n");
    content.push_str("# Generated by Flipper Zero Connector\n");
    content.push_str(&format!("Key type: {}\n", key_type));

    let data_size = match key_type {
        "EM4100" => 5,
        "H10301" | "I40134" => 3,
        _ => 3,
    };

    content.push_str(&format!("# Data size for EM4100 is 5, for H10301 is 3, for I40134 is 3\n"));
    content.push_str(&format!("Data: {}\n", data));

    content
}

/// Encode H10301 26-bit Wiegand format from facility code and card number
fn encode_h10301(facility: u8, card: u16) -> Result<String, flipper_core::error::Error> {
    // 26-bit Wiegand format: [1 even parity][8 facility][16 card][1 odd parity]
    // Stored as 3 bytes with padding

    let mut value: u32 = 0;

    // Place facility code (bits 17-24)
    value |= (facility as u32) << 17;

    // Place card number (bits 1-16)
    value |= (card as u32) << 1;

    // TODO: Calculate parity bits properly for full correctness
    // For now, just encoding without parity

    // Extract 3 bytes
    let byte0 = ((value >> 16) & 0xFF) as u8;
    let byte1 = ((value >> 8) & 0xFF) as u8;
    let byte2 = (value & 0xFF) as u8;

    Ok(format!("{:02X} {:02X} {:02X}", byte0, byte1, byte2))
}
