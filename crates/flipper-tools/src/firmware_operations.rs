//! Firmware Management Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Firmware Info Tool ===

pub struct FirmwareInfoTool;

#[async_trait]
impl PentestTool for FirmwareInfoTool {
    fn name(&self) -> &str {
        "flipper_firmware_info"
    }

    fn description(&self) -> &str {
        "Get Flipper Zero firmware information"
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
        // Firmware information requires qFlipper or manual inspection
        // The RPC protocol doesn't expose firmware version directly
        Ok(ToolResult {
            success: true,
            data: json!({
                "message": "Firmware information requires external tools",
                "instructions": {
                    "qFlipper": "Use qFlipper GUI to view firmware version and details",
                    "manual_check": "Check Settings → System Info on Flipper Zero device",
                    "cli_check": "Use 'flipper-cli info' command if available"
                },
                "firmware_types": {
                    "Official": {
                        "description": "Official Flipper Devices firmware",
                        "source": "https://update.flipperzero.one/",
                        "features": "Stable, officially supported"
                    },
                    "Unleashed": {
                        "description": "Unleashed firmware (more features)",
                        "source": "https://github.com/DarkFlippers/unleashed-firmware",
                        "features": "Extended Sub-GHz, NFC, RFID capabilities"
                    },
                    "Xtreme": {
                        "description": "Xtreme firmware (customized UI/UX)",
                        "source": "https://github.com/Flipper-XFW/Xtreme-Firmware",
                        "features": "Custom animations, themes, extended features"
                    },
                    "RogueMaster": {
                        "description": "RogueMaster firmware (extended plugins)",
                        "source": "https://github.com/RogueMaster/flipperzero-firmware-wPlugins",
                        "features": "Maximum features and plugins"
                    }
                },
                "note": "RPC protocol does not expose firmware version. Use qFlipper or device menu."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Firmware Backup Tool ===

pub struct FirmwareBackupTool;

#[async_trait]
impl PentestTool for FirmwareBackupTool {
    fn name(&self) -> &str {
        "flipper_firmware_backup"
    }

    fn description(&self) -> &str {
        "Backup current Flipper Zero firmware"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "backup_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to save firmware backup".to_string(),
                    required: false,
                    default: Some(json!("/ext/backups/firmware_backup.bin")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let backup_path = params["backup_path"]
            .as_str()
            .unwrap_or("/ext/backups/firmware_backup.bin");

        Ok(ToolResult {
            success: true,
            data: json!({
                "backup_path": backup_path,
                "message": "Firmware backup prepared",
                "instructions": "Firmware backup requires DFU mode and external tools",
                "backup_process": {
                    "step_1": "Enter DFU mode (hold LEFT + BACK while powering on)",
                    "step_2": "Use qFlipper or dfu-util to read firmware",
                    "step_3": "Save to specified path for restoration",
                    "tools": "qFlipper (GUI) or dfu-util (CLI)"
                },
                "backup_includes": [
                    "Firmware binary",
                    "Bootloader",
                    "Option bytes",
                    "System settings"
                ],
                "note": "Backup does NOT include SD card data or user files"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Firmware Update Tool ===

pub struct FirmwareUpdateTool;

#[async_trait]
impl PentestTool for FirmwareUpdateTool {
    fn name(&self) -> &str {
        "flipper_firmware_update"
    }

    fn description(&self) -> &str {
        "Update Flipper Zero firmware"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "firmware_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to firmware file (.dfu or .bin)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "firmware_type".to_string(),
                    param_type: ParamType::String,
                    description: "Firmware type: official, unleashed, xtreme, custom".to_string(),
                    required: false,
                    default: Some(json!("official")),
                },
                ToolParam {
                    name: "verify".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Verify firmware signature before flashing".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let firmware_path = params["firmware_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing firmware_path parameter".to_string()))?;

        let firmware_type = params["firmware_type"]
            .as_str()
            .unwrap_or("official");

        let verify = params["verify"]
            .as_bool()
            .unwrap_or(true);

        // Validate firmware type
        let valid_types = ["official", "unleashed", "xtreme", "custom"];
        if !valid_types.contains(&firmware_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid firmware_type. Must be: {}", valid_types.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "firmware_path": firmware_path,
                "firmware_type": firmware_type,
                "verify": verify,
                "message": "Firmware update prepared",
                "instructions": "Firmware update requires DFU mode and external tools",
                "update_process": {
                    "step_1": "Backup current firmware (recommended)",
                    "step_2": "Download firmware from official source",
                    "step_3": "Verify firmware checksum",
                    "step_4": "Enter DFU mode (hold LEFT + BACK while powering on)",
                    "step_5": "Use qFlipper or dfu-util to flash firmware",
                    "step_6": "Reboot and verify update"
                },
                "firmware_sources": {
                    "Official": "https://update.flipperzero.one/",
                    "Unleashed": "https://github.com/DarkFlippers/unleashed-firmware",
                    "Xtreme": "https://github.com/Flipper-XFW/Xtreme-Firmware",
                    "RogueMaster": "https://github.com/RogueMaster/flipperzero-firmware-wPlugins"
                },
                "warnings": [
                    "⚠️  Always backup firmware before updating",
                    "⚠️  Only flash firmware from trusted sources",
                    "⚠️  Verify checksums before flashing",
                    "⚠️  Do not disconnect during flash process",
                    "⚠️  Bricking is possible with incorrect firmware"
                ]
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Firmware Verify Tool ===

pub struct FirmwareVerifyTool;

#[async_trait]
impl PentestTool for FirmwareVerifyTool {
    fn name(&self) -> &str {
        "flipper_firmware_verify"
    }

    fn description(&self) -> &str {
        "Verify Flipper Zero firmware integrity"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "check_signature".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Verify firmware signature".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
                ToolParam {
                    name: "check_version".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Check for firmware updates".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let check_signature = params["check_signature"]
            .as_bool()
            .unwrap_or(true);

        let check_version = params["check_version"]
            .as_bool()
            .unwrap_or(false);

        // Verify connection is available
        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let is_connected = client.health_check().await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "check_signature": check_signature,
                "check_version": check_version,
                "connected": is_connected,
                "message": "Firmware verification requires external tools",
                "verification_checks": {
                    "signature": if check_signature { "Verify cryptographic signature" } else { "Skipped" },
                    "version": if check_version { "Check for newer versions" } else { "Skipped" },
                    "integrity": "Verify firmware not corrupted",
                    "official": "Check if official or custom firmware"
                },
                "security_indicators": [
                    "Official firmware is signed by Flipper Devices",
                    "Custom firmware (Unleashed, Xtreme) have different signatures",
                    "Modified firmware may have no valid signature",
                    "Check GitHub releases for official checksums"
                ],
                "instructions": {
                    "manual_verify": "Compare firmware hash with official release",
                    "qFlipper": "Use qFlipper to verify and update firmware",
                    "command_line": "Use dfu-util --device 0483:df11 --alt 0 --dfuse-address 0x08000000 --upload firmware.bin"
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
