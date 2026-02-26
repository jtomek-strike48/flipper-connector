//! iButton (Dallas Key) Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === iButton File Read Tool ===

pub struct IButtonReadTool;

#[async_trait]
impl PentestTool for IButtonReadTool {
    fn name(&self) -> &str {
        "flipper_ibutton_read"
    }

    fn description(&self) -> &str {
        "Read and parse iButton file from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path to .ibtn file (e.g., /ext/ibutton/key.ibtn)".to_string(),
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

        // Parse iButton file
        let parsed = parse_ibutton_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        Ok(ToolResult {
            success: true,
            data: parsed,
            error: None,
            duration_ms: 0,
        })
    }
}

/// Parse iButton file content into structured data
fn parse_ibutton_file(content: &str) -> Result<Value, String> {
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
        return Err("Invalid iButton file format".to_string());
    }

    let mut result = json!({
        "key_type": key_type,
        "data": data,
    });

    // Decode based on key type
    match key_type.as_str() {
        "Dallas" => {
            if let Ok(decoded) = decode_dallas_key(&data) {
                result["family_code"] = json!(decoded.family_code);
                result["serial_number"] = json!(decoded.serial_number);
                result["crc"] = json!(decoded.crc);
                result["decoded"] = json!(format!(
                    "Family: 0x{:02X}, Serial: {}, CRC: 0x{:02X}",
                    decoded.family_code, decoded.serial_number, decoded.crc
                ));
            }
        }
        "Cyfral" => {
            result["decoded"] = json!(format!("Cyfral key: {}", data));
        }
        "Metakom" => {
            result["decoded"] = json!(format!("Metakom key: {}", data));
        }
        _ => {}
    }

    Ok(result)
}

/// Dallas key decoded structure
struct DallasKey {
    family_code: u8,
    serial_number: String,
    crc: u8,
}

/// Decode Dallas (1-Wire) key format
/// Dallas keys are 8 bytes: [1 family code][6 serial][1 CRC]
fn decode_dallas_key(data_hex: &str) -> Result<DallasKey, String> {
    // Parse hex bytes
    let bytes: Result<Vec<u8>, _> = data_hex
        .split_whitespace()
        .map(|b| u8::from_str_radix(b, 16))
        .collect();

    let bytes = bytes.map_err(|e| format!("Invalid hex: {}", e))?;

    if bytes.len() != 8 {
        return Err("Dallas key requires 8 bytes".to_string());
    }

    let family_code = bytes[0];
    let serial_bytes = &bytes[1..7];
    let crc = bytes[7];

    // Format serial number as hex string
    let serial_number = serial_bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");

    Ok(DallasKey {
        family_code,
        serial_number,
        crc,
    })
}

// === iButton File Write Tool ===

pub struct IButtonWriteTool;

#[async_trait]
impl PentestTool for IButtonWriteTool {
    fn name(&self) -> &str {
        "flipper_ibutton_write"
    }

    fn description(&self) -> &str {
        "Create iButton file on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path for new .ibtn file (e.g., /ext/ibutton/mykey.ibtn)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "key_type".to_string(),
                    param_type: ParamType::String,
                    description: "Key type: Dallas, Cyfral, or Metakom".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "data".to_string(),
                    param_type: ParamType::String,
                    description: "Key data in hex format (space-separated bytes)".to_string(),
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

        let key_type = params["key_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing key_type parameter".to_string()))?;

        let data = params["data"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing data parameter".to_string()))?;

        // Validate key type
        let valid_types = ["Dallas", "Cyfral", "Metakom"];
        if !valid_types.contains(&key_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid key_type. Must be one of: {}", valid_types.join(", "))
            ));
        }

        // Validate data format
        validate_ibutton_data(key_type, data)
            .map_err(|e| flipper_core::error::Error::InvalidParams(e))?;

        // Generate iButton file content
        let content = format!(
            "Filetype: Flipper iButton key\nVersion: 1\nKey type: {}\nData: {}\n",
            key_type, data
        );

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Write file
        client.write_file(path, content.as_bytes().to_vec()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "key_type": key_type,
                "data": data,
                "message": "iButton file created successfully"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Validate iButton data format
fn validate_ibutton_data(key_type: &str, data: &str) -> Result<(), String> {
    let bytes: Result<Vec<u8>, _> = data
        .split_whitespace()
        .map(|b| u8::from_str_radix(b, 16))
        .collect();

    let bytes = bytes.map_err(|_| "Invalid hex format".to_string())?;

    match key_type {
        "Dallas" => {
            if bytes.len() != 8 {
                return Err("Dallas keys require exactly 8 bytes".to_string());
            }
        }
        "Cyfral" => {
            if bytes.len() != 2 {
                return Err("Cyfral keys require exactly 2 bytes".to_string());
            }
        }
        "Metakom" => {
            if bytes.len() != 4 {
                return Err("Metakom keys require exactly 4 bytes".to_string());
            }
        }
        _ => return Err("Unknown key type".to_string()),
    }

    Ok(())
}

// === iButton Emulation Tool ===

pub struct IButtonEmulateTool;

#[async_trait]
impl PentestTool for IButtonEmulateTool {
    fn name(&self) -> &str {
        "flipper_ibutton_emulate"
    }

    fn description(&self) -> &str {
        "Prepare iButton file for emulation on Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "source_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to source .ibtn file to emulate".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "emulate_path".to_string(),
                    param_type: ParamType::String,
                    description: "Optional path for emulation file (defaults to source path)".to_string(),
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

        let emulate_path = params["emulate_path"]
            .as_str()
            .unwrap_or(source_path);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Read source file to validate
        let content = client.read_file(source_path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let text = String::from_utf8(content)
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Invalid UTF-8: {}", e)))?;

        // Validate file format
        let parsed = parse_ibutton_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        // If different path, copy file
        if source_path != emulate_path {
            client.write_file(emulate_path, text.as_bytes().to_vec()).await
                .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "source_path": source_path,
                "emulate_path": emulate_path,
                "key_type": parsed["key_type"],
                "data": parsed["data"],
                "message": "Ready for emulation. Use Flipper Zero app to start emulation.",
                "instructions": "Open iButton app on Flipper → Saved → Select file → Emulate"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
