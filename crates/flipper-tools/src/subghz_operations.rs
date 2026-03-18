//! Sub-GHz File Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Sub-GHz File Read Tool ===

pub struct SubGhzReadTool;

#[async_trait]
impl PentestTool for SubGhzReadTool {
    fn name(&self) -> &str {
        "flipper_subghz_read"
    }

    fn description(&self) -> &str {
        "Read and parse Sub-GHz file from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path to .sub file (e.g., /ext/subghz/remote.sub)".to_string(),
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

        // Parse Sub-GHz file
        let parsed = parse_subghz_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        Ok(ToolResult {
            success: true,
            data: parsed,
            error: None,
            duration_ms: 0,
        })
    }
}

/// Parse Sub-GHz file content into structured data
fn parse_subghz_file(content: &str) -> Result<Value, String> {
    let mut filetype = String::new();
    let mut frequency = 0u32;
    let mut preset = String::new();
    let mut protocol = String::new();
    let mut key = String::new();
    let mut bit = 0u32;
    let mut te = 0u32;
    let mut raw_data = String::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse key-value pairs
        if let Some((key_str, value)) = line.split_once(':') {
            let key_str = key_str.trim();
            let value = value.trim();

            match key_str {
                "Filetype" => filetype = value.to_string(),
                "Frequency" => {
                    frequency = value.parse().unwrap_or(0);
                }
                "Preset" => preset = value.to_string(),
                "Protocol" => protocol = value.to_string(),
                "Key" => key = value.to_string(),
                "Bit" => {
                    bit = value.parse().unwrap_or(0);
                }
                "TE" => {
                    te = value.parse().unwrap_or(0);
                }
                "RAW_Data" => raw_data = value.to_string(),
                _ => {}
            }
        }
    }

    if protocol.is_empty() {
        return Err("Invalid Sub-GHz file format".to_string());
    }

    let is_raw = protocol == "RAW";
    let frequency_mhz = frequency as f64 / 1_000_000.0;

    let mut result = json!({
        "filetype": filetype,
        "frequency": frequency,
        "frequency_mhz": format!("{:.2} MHz", frequency_mhz),
        "preset": preset,
        "protocol": protocol,
        "is_raw": is_raw,
    });

    // Add protocol-specific fields
    if is_raw {
        result["raw_data"] = json!(raw_data);
        result["raw_data_length"] = json!(raw_data.split_whitespace().count());
    } else {
        result["key"] = json!(key);
        result["bit"] = json!(bit);
        if te > 0 {
            result["te"] = json!(te);
        }
    }

    Ok(result)
}

// === Sub-GHz File Write Tool ===

pub struct SubGhzWriteTool;

#[async_trait]
impl PentestTool for SubGhzWriteTool {
    fn name(&self) -> &str {
        "flipper_subghz_write"
    }

    fn description(&self) -> &str {
        "Create a Sub-GHz file on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path for new .sub file (e.g., /ext/subghz/remote.sub)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "frequency".to_string(),
                    param_type: ParamType::Number,
                    description: "Frequency in Hz (e.g., 433920000 for 433.92 MHz)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "protocol".to_string(),
                    param_type: ParamType::String,
                    description: "Protocol name (e.g., 'Princeton', 'GateTX', 'KeeLoq')".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "key".to_string(),
                    param_type: ParamType::String,
                    description: "Key data in hex format (e.g., '00 00 00 00 00 12 34 56')".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "bit".to_string(),
                    param_type: ParamType::Number,
                    description: "Number of bits (e.g., 24 for Princeton, 64 for KeeLoq)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "te".to_string(),
                    param_type: ParamType::Number,
                    description: "Time Element in microseconds (optional, for Princeton)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "preset".to_string(),
                    param_type: ParamType::String,
                    description: "Modulation preset (default: FuriHalSubGhzPresetOok650Async)".to_string(),
                    required: false,
                    default: Some(json!("FuriHalSubGhzPresetOok650Async")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing path parameter".to_string()))?;

        let frequency = params["frequency"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing frequency parameter".to_string()))? as u32;

        let protocol = params["protocol"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing protocol parameter".to_string()))?;

        let key = params["key"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing key parameter".to_string()))?;

        let bit = params["bit"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing bit parameter".to_string()))? as u32;

        let te = params["te"].as_u64().map(|v| v as u32);
        let preset = params["preset"].as_str().unwrap_or("FuriHalSubGhzPresetOok650Async");

        // Generate Sub-GHz file content
        let content = generate_subghz_file(frequency, preset, protocol, key, bit, te);
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
                "frequency": frequency,
                "frequency_mhz": format!("{:.2} MHz", frequency as f64 / 1_000_000.0),
                "protocol": protocol,
                "bit": bit,
                "size": content_size
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Generate Sub-GHz file content
fn generate_subghz_file(frequency: u32, preset: &str, protocol: &str, key: &str, bit: u32, te: Option<u32>) -> String {
    let mut content = String::new();

    content.push_str("Filetype: Flipper SubGHz Key file\n");
    content.push_str("Version: 1\n");
    content.push_str("# Generated by Flipper Zero Connector\n");
    content.push_str(&format!("Frequency: {}\n", frequency));
    content.push_str(&format!("Preset: {}\n", preset));
    content.push_str(&format!("Protocol: {}\n", protocol));
    content.push_str(&format!("Bit: {}\n", bit));
    content.push_str(&format!("Key: {}\n", key));

    if let Some(te_val) = te {
        content.push_str(&format!("TE: {}\n", te_val));
    }

    content
}

// === Sub-GHz Bruteforce Tool ===

pub struct SubGhzBruteforceTool;

#[async_trait]
impl PentestTool for SubGhzBruteforceTool {
    fn name(&self) -> &str {
        "flipper_subghz_bruteforce"
    }

    fn description(&self) -> &str {
        "Generate Sub-GHz bruteforce files for static code testing (Unleashed firmware)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "output_dir".to_string(),
                    param_type: ParamType::String,
                    description: "Output directory for bruteforce files (e.g., /ext/subghz/bruteforce)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "protocol".to_string(),
                    param_type: ParamType::String,
                    description: "Protocol name (e.g., 'Princeton', 'PT-2240', 'Nice FLO')".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "frequency".to_string(),
                    param_type: ParamType::Number,
                    description: "Frequency in Hz (e.g., 433920000 for 433.92 MHz, 315000000 for 315 MHz)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "start_key".to_string(),
                    param_type: ParamType::String,
                    description: "Starting key in hex (e.g., '00 00 00 00' for 32-bit)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "end_key".to_string(),
                    param_type: ParamType::String,
                    description: "Ending key in hex (e.g., '00 00 FF FF' for 32-bit)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "bit_length".to_string(),
                    param_type: ParamType::Number,
                    description: "Bit length (12, 24, 32, or 64)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "te".to_string(),
                    param_type: ParamType::Number,
                    description: "Time Element in microseconds (optional, protocol-specific)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "batch_size".to_string(),
                    param_type: ParamType::Number,
                    description: "Number of keys per file (default: 100, max: 1000)".to_string(),
                    required: false,
                    default: Some(json!(100)),
                },
                ToolParam {
                    name: "delay_ms".to_string(),
                    param_type: ParamType::Number,
                    description: "Delay between transmissions in milliseconds (default: 50)".to_string(),
                    required: false,
                    default: Some(json!(50)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let output_dir = params["output_dir"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing output_dir parameter".to_string()))?;

        let protocol = params["protocol"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing protocol parameter".to_string()))?;

        let frequency = params["frequency"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing frequency parameter".to_string()))? as u32;

        let start_key_str = params["start_key"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing start_key parameter".to_string()))?;

        let end_key_str = params["end_key"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing end_key parameter".to_string()))?;

        let bit_length = params["bit_length"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing bit_length parameter".to_string()))? as u32;

        let te = params["te"].as_u64().map(|v| v as u32);
        let batch_size = params["batch_size"].as_u64().unwrap_or(100) as usize;
        let delay_ms = params["delay_ms"].as_u64().unwrap_or(50) as u32;

        // Validate batch size
        if batch_size < 1 || batch_size > 1000 {
            return Err(flipper_core::error::Error::InvalidParams(
                "batch_size must be between 1 and 1000".to_string()
            ));
        }

        // Parse start and end keys
        let start_key = parse_hex_key(start_key_str)?;
        let end_key = parse_hex_key(end_key_str)?;

        if start_key > end_key {
            return Err(flipper_core::error::Error::InvalidParams(
                "start_key must be less than or equal to end_key".to_string()
            ));
        }

        let total_keys = (end_key - start_key + 1) as usize;

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Create output directory
        client.create_directory(output_dir).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut files_created = Vec::new();
        let mut current_key = start_key;
        let mut file_index = 0;

        while current_key <= end_key {
            let batch_end = std::cmp::min(current_key + batch_size as u64 - 1, end_key);
            let filename = format!("{}/bruteforce_{:04}.sub", output_dir, file_index);

            // Generate file content with all keys in this batch
            let content = generate_bruteforce_batch(
                frequency,
                protocol,
                current_key,
                batch_end,
                bit_length,
                te,
                delay_ms,
            );

            // Write file
            client.write_file(&filename, content.into_bytes()).await
                .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

            files_created.push(filename);
            current_key = batch_end + 1;
            file_index += 1;
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "output_dir": output_dir,
                "protocol": protocol,
                "frequency": frequency,
                "frequency_mhz": format!("{:.2} MHz", frequency as f64 / 1_000_000.0),
                "start_key": start_key_str,
                "end_key": end_key_str,
                "bit_length": bit_length,
                "total_keys": total_keys,
                "files_created": files_created.len(),
                "batch_size": batch_size,
                "delay_ms": delay_ms,
                "files": files_created,
                "estimated_time_seconds": (total_keys as u64 * delay_ms as u64) / 1000,
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Parse hex key string to u64
fn parse_hex_key(key_str: &str) -> Result<u64, flipper_core::error::Error> {
    let hex_string: String = key_str.split_whitespace().collect();
    u64::from_str_radix(&hex_string, 16)
        .map_err(|e| flipper_core::error::Error::InvalidParams(format!("Invalid hex key: {}", e)))
}

/// Format u64 key to hex string with proper spacing
fn format_hex_key(key: u64, bit_length: u32) -> String {
    let byte_count = (bit_length + 7) / 8;
    let mut result = Vec::new();

    for i in (0..byte_count).rev() {
        let byte = ((key >> (i * 8)) & 0xFF) as u8;
        result.push(format!("{:02X}", byte));
    }

    result.join(" ")
}

/// Generate bruteforce batch file content
fn generate_bruteforce_batch(
    frequency: u32,
    protocol: &str,
    start_key: u64,
    end_key: u64,
    bit_length: u32,
    te: Option<u32>,
    delay_ms: u32,
) -> String {
    let mut content = String::new();

    content.push_str("Filetype: Flipper SubGHz Key file\n");
    content.push_str("Version: 1\n");
    content.push_str("# Sub-GHz Bruteforce Batch - Unleashed Firmware\n");
    content.push_str(&format!("# Generated by Flipper Zero Connector\n"));
    content.push_str(&format!("# Range: {} to {}\n", start_key, end_key));
    content.push_str(&format!("# Keys in batch: {}\n", end_key - start_key + 1));
    content.push_str(&format!("# Delay: {}ms between transmissions\n", delay_ms));
    content.push_str("\n");

    for key in start_key..=end_key {
        content.push_str(&format!("Frequency: {}\n", frequency));
        content.push_str("Preset: FuriHalSubGhzPresetOok650Async\n");
        content.push_str(&format!("Protocol: {}\n", protocol));
        content.push_str(&format!("Bit: {}\n", bit_length));
        content.push_str(&format!("Key: {}\n", format_hex_key(key, bit_length)));

        if let Some(te_val) = te {
            content.push_str(&format!("TE: {}\n", te_val));
        }

        content.push_str(&format!("Repeat: 3\n"));
        content.push_str(&format!("Delay: {}\n", delay_ms));
        content.push_str("\n");
    }

    content
}

// === Sub-GHz Remote Creator Tool ===

pub struct SubGhzRemoteTool;

#[async_trait]
impl PentestTool for SubGhzRemoteTool {
    fn name(&self) -> &str {
        "flipper_subghz_remote_create"
    }

    fn description(&self) -> &str {
        "Create Sub-GHz Remote map file for 5-button quick-access remote (Unleashed firmware)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "output_path".to_string(),
                    param_type: ParamType::String,
                    description: "Output path for remote map file (e.g., /ext/subghz_remote/living_room.txt)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "up_file".to_string(),
                    param_type: ParamType::String,
                    description: "Path to .sub file for UP button".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "up_label".to_string(),
                    param_type: ParamType::String,
                    description: "Label for UP button (max 16 chars)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "down_file".to_string(),
                    param_type: ParamType::String,
                    description: "Path to .sub file for DOWN button".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "down_label".to_string(),
                    param_type: ParamType::String,
                    description: "Label for DOWN button (max 16 chars)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "left_file".to_string(),
                    param_type: ParamType::String,
                    description: "Path to .sub file for LEFT button".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "left_label".to_string(),
                    param_type: ParamType::String,
                    description: "Label for LEFT button (max 16 chars)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "right_file".to_string(),
                    param_type: ParamType::String,
                    description: "Path to .sub file for RIGHT button".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "right_label".to_string(),
                    param_type: ParamType::String,
                    description: "Label for RIGHT button (max 16 chars)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "ok_file".to_string(),
                    param_type: ParamType::String,
                    description: "Path to .sub file for OK (center) button".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "ok_label".to_string(),
                    param_type: ParamType::String,
                    description: "Label for OK button (max 16 chars)".to_string(),
                    required: false,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let output_path = params["output_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing output_path parameter".to_string()))?;

        // Collect button configurations
        let mut buttons: Vec<(&str, &str, &str, &str)> = Vec::new();
        let button_configs = [
            ("up", "UP", "ULABEL"),
            ("down", "DOWN", "DLABEL"),
            ("left", "LEFT", "LLABEL"),
            ("right", "RIGHT", "RLABEL"),
            ("ok", "OK", "OKLABEL"),
        ];

        for (param_prefix, file_key, label_key) in button_configs.iter() {
            let file_param = format!("{}_file", param_prefix);
            let label_param = format!("{}_label", param_prefix);

            if let Some(file_path) = params[&file_param].as_str() {
                let label = params[&label_param].as_str().unwrap_or("");

                // Validate label length
                if label.len() > 16 {
                    return Err(flipper_core::error::Error::InvalidParams(
                        format!("{} label exceeds 16 characters", param_prefix.to_uppercase())
                    ));
                }

                // Validate file path (no spaces or special chars except - and _)
                if file_path.contains(' ') || file_path.chars().any(|c| !c.is_ascii_alphanumeric() && c != '/' && c != '.' && c != '-' && c != '_') {
                    return Err(flipper_core::error::Error::InvalidParams(
                        format!("{} file path contains invalid characters (no spaces or special chars allowed)", param_prefix.to_uppercase())
                    ));
                }

                buttons.push((*file_key, *label_key, file_path, label));
            }
        }

        if buttons.is_empty() {
            return Err(flipper_core::error::Error::InvalidParams(
                "At least one button must be configured".to_string()
            ));
        }

        // Generate remote map file content
        let content = generate_remote_map(&buttons);
        let button_count = buttons.len();

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Ensure directory exists
        if let Some(dir_path) = std::path::Path::new(output_path).parent() {
            let dir_str = dir_path.to_string_lossy();
            client.create_directory(&dir_str).await
                .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;
        }

        // Write remote map file
        client.write_file(output_path, content.into_bytes()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let mut button_summary = json!({});
        for (_, _, file_path, label) in &buttons {
            button_summary[label] = json!(file_path);
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "output_path": output_path,
                "button_count": button_count,
                "buttons": button_summary,
                "usage": "Open Sub-GHz Remote app on Flipper Zero and select this map file"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Generate Sub-GHz Remote map file content
fn generate_remote_map(buttons: &[(&str, &str, &str, &str)]) -> String {
    let mut content = String::new();

    content.push_str("# Sub-GHz Remote Map File\n");
    content.push_str("# Generated by Flipper Zero Connector\n");
    content.push_str("# Format: BUTTON: /path/to/file.sub\n");
    content.push_str("#         BUTTONLABEL: Display Text\n");
    content.push_str("# Buttons: UP, DOWN, LEFT, RIGHT, OK\n");
    content.push_str("# Labels: Max 16 characters\n");
    content.push_str("# File paths: No spaces or special characters (- and _ allowed)\n");
    content.push_str("\n");

    for (file_key, label_key, file_path, label) in buttons {
        content.push_str(&format!("{}: {}\n", file_key, file_path));
        if !label.is_empty() {
            content.push_str(&format!("{}: {}\n", label_key, label));
        }
    }

    content
}
