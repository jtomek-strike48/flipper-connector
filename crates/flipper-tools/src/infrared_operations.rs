//! Infrared Remote Control Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === IR File Read Tool ===

pub struct InfraredReadTool;

#[async_trait]
impl PentestTool for InfraredReadTool {
    fn name(&self) -> &str {
        "flipper_ir_read"
    }

    fn description(&self) -> &str {
        "Read and parse Infrared remote file from the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path to .ir file (e.g., /ext/infrared/TV.ir)".to_string(),
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

        // Parse IR file
        let parsed = parse_ir_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        Ok(ToolResult {
            success: true,
            data: parsed,
            error: None,
            duration_ms: 0,
        })
    }
}

/// IR signal structure
#[derive(Debug)]
struct IrSignal {
    name: String,
    signal_type: String,
    protocol: Option<String>,
    address: Option<String>,
    command: Option<String>,
    frequency: Option<u32>,
    duty_cycle: Option<f32>,
    data: Option<String>,
}

/// Parse IR file content into structured data
fn parse_ir_file(content: &str) -> Result<Value, String> {
    let mut signals = Vec::new();
    let mut current_signal: Option<IrSignal> = None;

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
                "name" => {
                    // Save previous signal if exists
                    if let Some(signal) = current_signal.take() {
                        signals.push(signal);
                    }
                    // Start new signal
                    current_signal = Some(IrSignal {
                        name: value.to_string(),
                        signal_type: String::new(),
                        protocol: None,
                        address: None,
                        command: None,
                        frequency: None,
                        duty_cycle: None,
                        data: None,
                    });
                }
                "type" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.signal_type = value.to_string();
                    }
                }
                "protocol" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.protocol = Some(value.to_string());
                    }
                }
                "address" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.address = Some(value.to_string());
                    }
                }
                "command" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.command = Some(value.to_string());
                    }
                }
                "frequency" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.frequency = value.parse().ok();
                    }
                }
                "duty_cycle" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.duty_cycle = value.parse().ok();
                    }
                }
                "data" => {
                    if let Some(ref mut signal) = current_signal {
                        signal.data = Some(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    // Save last signal
    if let Some(signal) = current_signal {
        signals.push(signal);
    }

    if signals.is_empty() {
        return Err("No IR signals found in file".to_string());
    }

    // Convert to JSON
    let signals_json: Vec<Value> = signals
        .into_iter()
        .map(|s| {
            let mut obj = json!({
                "name": s.name,
                "type": s.signal_type,
            });

            if let Some(protocol) = s.protocol {
                obj["protocol"] = json!(protocol);
            }
            if let Some(address) = s.address {
                obj["address"] = json!(address);
            }
            if let Some(command) = s.command {
                obj["command"] = json!(command);
            }
            if let Some(frequency) = s.frequency {
                obj["frequency"] = json!(frequency);
            }
            if let Some(duty_cycle) = s.duty_cycle {
                obj["duty_cycle"] = json!(duty_cycle);
            }
            if let Some(data) = s.data {
                obj["data"] = json!(data);
            }

            obj
        })
        .collect();

    Ok(json!({
        "signals": signals_json,
        "count": signals_json.len(),
    }))
}

// === IR File Write Tool ===

pub struct InfraredWriteTool;

#[async_trait]
impl PentestTool for InfraredWriteTool {
    fn name(&self) -> &str {
        "flipper_ir_write"
    }

    fn description(&self) -> &str {
        "Create Infrared remote file on the Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "path".to_string(),
                    param_type: ParamType::String,
                    description: "Full path for new .ir file (e.g., /ext/infrared/remote.ir)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "buttons".to_string(),
                    param_type: ParamType::Object,
                    description: "Array of button definitions with name, protocol, address, command".to_string(),
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

        let buttons = params["buttons"]
            .as_array()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("buttons must be an array".to_string()))?;

        if buttons.is_empty() {
            return Err(flipper_core::error::Error::InvalidParams(
                "At least one button required".to_string()
            ));
        }

        // Generate IR file content
        let mut content = String::from("Filetype: IR signals file\nVersion: 1\n");

        for button in buttons {
            let name = button["name"]
                .as_str()
                .ok_or_else(|| flipper_core::error::Error::InvalidParams("Each button must have a name".to_string()))?;

            let protocol = button["protocol"]
                .as_str()
                .ok_or_else(|| flipper_core::error::Error::InvalidParams("Each button must have a protocol".to_string()))?;

            let address = button["address"]
                .as_str()
                .ok_or_else(|| flipper_core::error::Error::InvalidParams("Each button must have an address".to_string()))?;

            let command = button["command"]
                .as_str()
                .ok_or_else(|| flipper_core::error::Error::InvalidParams("Each button must have a command".to_string()))?;

            // Validate protocol
            let valid_protocols = [
                "NEC", "NECext", "Samsung32", "RC5", "RC6", "SIRC", "SIRC15", "SIRC20",
                "Kaseikyo", "RCA"
            ];
            if !valid_protocols.contains(&protocol) {
                return Err(flipper_core::error::Error::InvalidParams(
                    format!("Invalid protocol. Supported: {}", valid_protocols.join(", "))
                ));
            }

            content.push_str(&format!("\nname: {}\n", name));
            content.push_str("type: parsed\n");
            content.push_str(&format!("protocol: {}\n", protocol));
            content.push_str(&format!("address: {}\n", address));
            content.push_str(&format!("command: {}\n", command));
        }

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Write file
        client.write_file(path, content.as_bytes().to_vec()).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "path": path,
                "buttons": buttons.len(),
                "message": "IR remote file created successfully"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === IR Send Tool ===

pub struct InfraredSendTool;

#[async_trait]
impl PentestTool for InfraredSendTool {
    fn name(&self) -> &str {
        "flipper_ir_send"
    }

    fn description(&self) -> &str {
        "Send IR signal from file using Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "file_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to .ir file containing signals".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "button_name".to_string(),
                    param_type: ParamType::String,
                    description: "Name of button/signal to transmit".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let file_path = params["file_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing file_path parameter".to_string()))?;

        let button_name = params["button_name"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing button_name parameter".to_string()))?;

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Read and validate file
        let content = client.read_file(file_path).await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let text = String::from_utf8(content)
            .map_err(|e| flipper_core::error::Error::ToolExecution(format!("Invalid UTF-8: {}", e)))?;

        let parsed = parse_ir_file(&text)
            .map_err(|e| flipper_core::error::Error::ToolExecution(e))?;

        // Find button in parsed signals
        let signals = parsed["signals"]
            .as_array()
            .ok_or_else(|| flipper_core::error::Error::ToolExecution("Invalid parsed format".to_string()))?;

        let button = signals
            .iter()
            .find(|s| s["name"].as_str() == Some(button_name))
            .ok_or_else(|| flipper_core::error::Error::InvalidParams(
                format!("Button '{}' not found in IR file", button_name)
            ))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "file_path": file_path,
                "button_name": button_name,
                "signal": button,
                "message": "IR signal prepared for transmission",
                "instructions": "Use Flipper Zero IR app to transmit: Saved → Select file → Select button → Send"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
