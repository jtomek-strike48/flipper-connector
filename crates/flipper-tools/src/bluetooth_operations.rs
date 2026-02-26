//! Bluetooth Low Energy (BLE) Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === BLE Scan Tool ===

pub struct BleScanTool;

#[async_trait]
impl PentestTool for BleScanTool {
    fn name(&self) -> &str {
        "flipper_ble_scan"
    }

    fn description(&self) -> &str {
        "Scan for Bluetooth LE devices using Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "duration".to_string(),
                    param_type: ParamType::Number,
                    description: "Scan duration in seconds (1-60)".to_string(),
                    required: false,
                    default: Some(json!(10)),
                },
                ToolParam {
                    name: "active".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Use active scanning (requests more data from devices)".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let duration = params["duration"]
            .as_u64()
            .unwrap_or(10);

        let active = params["active"]
            .as_bool()
            .unwrap_or(false);

        if duration == 0 || duration > 60 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Duration must be between 1 and 60 seconds".to_string()
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "scan_type": if active { "active" } else { "passive" },
                "duration": duration,
                "message": "BLE scan prepared",
                "instructions": "Use Bluetooth app on Flipper Zero to start scan",
                "note": "BLE scanning shows device names, MAC addresses, RSSI, and advertised services"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === BLE Device Info Tool ===

pub struct BleDeviceInfoTool;

#[async_trait]
impl PentestTool for BleDeviceInfoTool {
    fn name(&self) -> &str {
        "flipper_ble_device_info"
    }

    fn description(&self) -> &str {
        "Get information about a BLE device"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "mac_address".to_string(),
                    param_type: ParamType::String,
                    description: "BLE device MAC address (e.g., AA:BB:CC:DD:EE:FF)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mac_address = params["mac_address"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing mac_address parameter".to_string()))?;

        // Validate MAC address format
        validate_mac_address(mac_address)?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "mac_address": mac_address,
                "message": "Device info request prepared",
                "instructions": "Use Bluetooth app: Scan → Select device → View details",
                "available_info": [
                    "Device name",
                    "MAC address",
                    "RSSI (signal strength)",
                    "Advertised services",
                    "Manufacturer data",
                    "Connection status"
                ]
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

/// Validate MAC address format
fn validate_mac_address(mac: &str) -> Result<(), flipper_core::error::Error> {
    let parts: Vec<&str> = mac.split(':').collect();

    if parts.len() != 6 {
        return Err(flipper_core::error::Error::InvalidParams(
            "MAC address must have 6 octets separated by colons (e.g., AA:BB:CC:DD:EE:FF)".to_string()
        ));
    }

    for part in parts {
        if part.len() != 2 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Each MAC address octet must be 2 hex digits".to_string()
            ));
        }

        u8::from_str_radix(part, 16).map_err(|_| {
            flipper_core::error::Error::InvalidParams(
                "MAC address must contain valid hex digits".to_string()
            )
        })?;
    }

    Ok(())
}

// === BLE Services Enumeration Tool ===

pub struct BleEnumerateTool;

#[async_trait]
impl PentestTool for BleEnumerateTool {
    fn name(&self) -> &str {
        "flipper_ble_enumerate"
    }

    fn description(&self) -> &str {
        "Enumerate GATT services and characteristics of a BLE device"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "mac_address".to_string(),
                    param_type: ParamType::String,
                    description: "BLE device MAC address to enumerate".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "save_path".to_string(),
                    param_type: ParamType::String,
                    description: "Optional path to save enumeration results".to_string(),
                    required: false,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mac_address = params["mac_address"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing mac_address parameter".to_string()))?;

        let save_path = params["save_path"].as_str();

        validate_mac_address(mac_address)?;

        let mut result = json!({
            "mac_address": mac_address,
            "message": "Service enumeration prepared",
            "instructions": "Use Bluetooth app: Scan → Select device → Connect → View services",
            "enumeration_includes": [
                "All GATT services (UUIDs)",
                "Service characteristics",
                "Characteristic properties (read/write/notify)",
                "Descriptors",
                "UUID meanings (from Bluetooth SIG database)"
            ]
        });

        if let Some(path) = save_path {
            // Generate example enumeration file content
            let content = format!(
                "# BLE Device Enumeration\n\
                Device: {}\n\
                Date: {}\n\n\
                ## Services\n\
                Use Flipper Zero Bluetooth app to view and export service details.\n",
                mac_address,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
            );

            let mut client = FlipperClient::new()
                .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

            client.write_file(path, content.as_bytes().to_vec()).await
                .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

            result["save_path"] = json!(path);
            result["message"] = json!("Enumeration template saved");
        }

        Ok(ToolResult {
            success: true,
            data: result,
            error: None,
            duration_ms: 0,
        })
    }
}

// === BLE Security Test Tool ===

pub struct BleSecurityTestTool;

#[async_trait]
impl PentestTool for BleSecurityTestTool {
    fn name(&self) -> &str {
        "flipper_ble_security_test"
    }

    fn description(&self) -> &str {
        "Test BLE device security (pairing, encryption, authentication)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "mac_address".to_string(),
                    param_type: ParamType::String,
                    description: "BLE device MAC address to test".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "tests".to_string(),
                    param_type: ParamType::Object,
                    description: "Array of security tests to perform".to_string(),
                    required: false,
                    default: Some(json!(["pairing", "encryption", "authentication"])),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mac_address = params["mac_address"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing mac_address parameter".to_string()))?;

        validate_mac_address(mac_address)?;

        let tests = params["tests"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| vec!["pairing".to_string(), "encryption".to_string(), "authentication".to_string()]);

        Ok(ToolResult {
            success: true,
            data: json!({
                "mac_address": mac_address,
                "tests": tests,
                "security_checks": {
                    "pairing": "Test if device requires pairing (bonding)",
                    "encryption": "Check if connection is encrypted",
                    "authentication": "Test authentication requirements",
                    "authorization": "Test read/write authorization",
                    "mitm_protection": "Check Man-in-the-Middle protection"
                },
                "message": "BLE security testing prepared",
                "instructions": "Use Bluetooth app: Scan → Select device → Attempt connection → Observe security prompts",
                "common_findings": [
                    "No pairing required (Just Works)",
                    "Weak pairing (6-digit PIN)",
                    "Unauthenticated characteristics (readable without pairing)",
                    "Missing encryption",
                    "Weak authentication"
                ]
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
