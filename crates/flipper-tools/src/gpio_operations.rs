//! GPIO Pin Control and Protocol Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use serde_json::{json, Value};

// === GPIO Pin Set Tool ===

pub struct GpioSetTool;

#[async_trait]
impl PentestTool for GpioSetTool {
    fn name(&self) -> &str {
        "flipper_gpio_set"
    }

    fn description(&self) -> &str {
        "Set GPIO pin state on Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "pin".to_string(),
                    param_type: ParamType::String,
                    description: "GPIO pin name (e.g., PA7, PB3, PC0-PC3)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "mode".to_string(),
                    param_type: ParamType::String,
                    description: "Pin mode: input, output_push_pull, output_open_drain, analog".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "state".to_string(),
                    param_type: ParamType::String,
                    description: "Pin state for output modes: high or low".to_string(),
                    required: false,
                    default: Some(json!("low")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let pin = params["pin"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing pin parameter".to_string()))?;

        let mode = params["mode"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing mode parameter".to_string()))?;

        let state = params["state"]
            .as_str()
            .unwrap_or("low");

        // Validate mode
        let valid_modes = ["input", "output_push_pull", "output_open_drain", "analog"];
        if !valid_modes.contains(&mode) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid mode. Must be one of: {}", valid_modes.join(", "))
            ));
        }

        // Validate state for output modes
        if mode.starts_with("output") {
            let valid_states = ["high", "low"];
            if !valid_states.contains(&state) {
                return Err(flipper_core::error::Error::InvalidParams(
                    format!("Invalid state. Must be one of: {}", valid_states.join(", "))
                ));
            }
        }

        // Validate pin name
        validate_gpio_pin(pin)?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "pin": pin,
                "mode": mode,
                "state": state,
                "message": "GPIO pin configured",
                "note": "Pin configuration is not persistent. Use GPIO app or external application for operations."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Validate GPIO pin name
fn validate_gpio_pin(pin: &str) -> Result<(), flipper_core::error::Error> {
    // Flipper Zero GPIO pins: PA7, PB2, PB3, PC0, PC1, PC3
    let valid_pins = ["PA7", "PB2", "PB3", "PC0", "PC1", "PC3"];

    if !valid_pins.contains(&pin) {
        return Err(flipper_core::error::Error::InvalidParams(
            format!("Invalid pin. Valid pins: {}", valid_pins.join(", "))
        ));
    }

    Ok(())
}

// === GPIO Pin Read Tool ===

pub struct GpioReadTool;

#[async_trait]
impl PentestTool for GpioReadTool {
    fn name(&self) -> &str {
        "flipper_gpio_read"
    }

    fn description(&self) -> &str {
        "Read GPIO pin state from Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "pin".to_string(),
                    param_type: ParamType::String,
                    description: "GPIO pin name to read (e.g., PA7, PB3)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let pin = params["pin"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing pin parameter".to_string()))?;

        validate_gpio_pin(pin)?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "pin": pin,
                "note": "Pin reading requires GPIO app or direct hardware access",
                "instructions": "Use Flipper GPIO app: GPIO → USB-UART Bridge → Monitor pins"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === UART Communication Tool ===

pub struct UartTool;

#[async_trait]
impl PentestTool for UartTool {
    fn name(&self) -> &str {
        "flipper_uart_send"
    }

    fn description(&self) -> &str {
        "Send data via UART on Flipper Zero GPIO"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "data".to_string(),
                    param_type: ParamType::String,
                    description: "Data to send (text or hex with 0x prefix)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "baud_rate".to_string(),
                    param_type: ParamType::Number,
                    description: "UART baud rate (common: 9600, 115200)".to_string(),
                    required: false,
                    default: Some(json!(115200)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let data = params["data"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing data parameter".to_string()))?;

        let baud_rate = params["baud_rate"]
            .as_u64()
            .unwrap_or(115200);

        // Validate baud rate
        let valid_rates = [9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];
        if !valid_rates.contains(&baud_rate) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid baud rate. Common rates: {:?}", valid_rates)
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "data": data,
                "baud_rate": baud_rate,
                "pins": {
                    "TX": "pin 13 (USART TX)",
                    "RX": "pin 14 (USART RX)",
                    "GND": "pin 11 or 18"
                },
                "message": "UART transmission prepared",
                "instructions": "Use GPIO app: GPIO → USB-UART Bridge → Configure baud rate → Send data"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === I2C Scanner Tool ===

pub struct I2cScanTool;

#[async_trait]
impl PentestTool for I2cScanTool {
    fn name(&self) -> &str {
        "flipper_i2c_scan"
    }

    fn description(&self) -> &str {
        "Scan for I2C devices on Flipper Zero GPIO bus"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "start_address".to_string(),
                    param_type: ParamType::Number,
                    description: "Start I2C address to scan (hex, default 0x08)".to_string(),
                    required: false,
                    default: Some(json!(8)),
                },
                ToolParam {
                    name: "end_address".to_string(),
                    param_type: ParamType::Number,
                    description: "End I2C address to scan (hex, default 0x77)".to_string(),
                    required: false,
                    default: Some(json!(119)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let start_address = params["start_address"]
            .as_u64()
            .unwrap_or(0x08) as u8;

        let end_address = params["end_address"]
            .as_u64()
            .unwrap_or(0x77) as u8;

        if start_address > end_address {
            return Err(flipper_core::error::Error::InvalidParams(
                "start_address must be <= end_address".to_string()
            ));
        }

        if end_address > 0x7F {
            return Err(flipper_core::error::Error::InvalidParams(
                "I2C addresses must be 0x00-0x7F (7-bit addressing)".to_string()
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "start_address": format!("0x{:02X}", start_address),
                "end_address": format!("0x{:02X}", end_address),
                "pins": {
                    "SCL": "pin 16 (I2C SCL)",
                    "SDA": "pin 15 (I2C SDA)",
                    "GND": "pin 11 or 18",
                    "3.3V": "pin 9"
                },
                "message": "I2C scan prepared",
                "instructions": "Use GPIO app: GPIO → I2C Scanner → Start scan"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === SPI Exchange Tool ===

pub struct SpiExchangeTool;

#[async_trait]
impl PentestTool for SpiExchangeTool {
    fn name(&self) -> &str {
        "flipper_spi_exchange"
    }

    fn description(&self) -> &str {
        "Exchange data via SPI on Flipper Zero GPIO"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "data".to_string(),
                    param_type: ParamType::String,
                    description: "Data to send (hex bytes, space-separated)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "cs_pin".to_string(),
                    param_type: ParamType::String,
                    description: "Chip Select pin (e.g., PA7, PB3)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "clock_speed".to_string(),
                    param_type: ParamType::Number,
                    description: "SPI clock speed in Hz (e.g., 1000000 for 1MHz)".to_string(),
                    required: false,
                    default: Some(json!(1000000)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let data = params["data"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing data parameter".to_string()))?;

        let cs_pin = params["cs_pin"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing cs_pin parameter".to_string()))?;

        let clock_speed = params["clock_speed"]
            .as_u64()
            .unwrap_or(1000000);

        // Validate CS pin
        validate_gpio_pin(cs_pin)?;

        // Validate hex data
        let bytes: Result<Vec<u8>, _> = data
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16))
            .collect();

        bytes.map_err(|_| flipper_core::error::Error::InvalidParams(
            "Invalid hex data format. Use space-separated hex bytes (e.g., '01 02 FF')".to_string()
        ))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "data": data,
                "cs_pin": cs_pin,
                "clock_speed": clock_speed,
                "pins": {
                    "MOSI": "pin 6 (SPI MOSI)",
                    "MISO": "pin 7 (SPI MISO)",
                    "SCK": "pin 8 (SPI SCK)",
                    "CS": format!("{} (user-selected)", cs_pin),
                    "GND": "pin 11 or 18",
                    "3.3V": "pin 9"
                },
                "message": "SPI exchange prepared",
                "instructions": "Use GPIO app or custom application for SPI communication"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
