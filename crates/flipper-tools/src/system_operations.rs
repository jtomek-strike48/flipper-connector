//! System Utilities and Control Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === System Reboot Tool ===

pub struct SystemRebootTool;

#[async_trait]
impl PentestTool for SystemRebootTool {
    fn name(&self) -> &str {
        "flipper_system_reboot"
    }

    fn description(&self) -> &str {
        "Reboot Flipper Zero device"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "mode".to_string(),
                    param_type: ParamType::String,
                    description: "Reboot mode: normal, dfu, recovery".to_string(),
                    required: false,
                    default: Some(json!("normal")),
                },
                ToolParam {
                    name: "confirm".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Confirm reboot operation".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mode = params["mode"]
            .as_str()
            .unwrap_or("normal");

        let confirm = params["confirm"]
            .as_bool()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("confirm parameter required".to_string()))?;

        if !confirm {
            return Err(flipper_core::error::Error::InvalidParams(
                "Reboot operation requires explicit confirmation".to_string()
            ));
        }

        let valid_modes = ["normal", "dfu", "recovery"];
        if !valid_modes.contains(&mode) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid mode. Must be: {}", valid_modes.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "mode": mode,
                "message": format!("Reboot to {} mode requires manual operation", mode),
                "instructions": match mode {
                    "normal" => json!({
                        "method_1": "Long press BACK + LEFT simultaneously",
                        "method_2": "Navigate to Settings → System → Reboot",
                        "duration": "~10 seconds to restart"
                    }),
                    "dfu" => json!({
                        "step_1": "Power off Flipper Zero",
                        "step_2": "Hold LEFT button",
                        "step_3": "While holding LEFT, connect USB cable",
                        "step_4": "Screen will show DFU mode",
                        "note": "Used for firmware updates"
                    }),
                    "recovery" => json!({
                        "step_1": "Power off Flipper Zero",
                        "step_2": "Hold OK button (center of D-pad)",
                        "step_3": "While holding OK, power on",
                        "step_4": "Release OK when recovery menu appears",
                        "note": "Used for emergency recovery"
                    }),
                    _ => json!({"error": "Unknown mode"})
                },
                "warnings": [
                    "⚠️  Unsaved data may be lost",
                    "⚠️  RPC connection will be dropped",
                    "⚠️  Running apps will be terminated",
                    "⚠️  Wait for device to fully boot before reconnecting"
                ],
                "note": "RPC protocol does not support reboot command. Use device buttons."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === DateTime Sync Tool ===

pub struct DateTimeSyncTool;

#[async_trait]
impl PentestTool for DateTimeSyncTool {
    fn name(&self) -> &str {
        "flipper_datetime_sync"
    }

    fn description(&self) -> &str {
        "Synchronize Flipper Zero date and time with host system"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "timezone_offset".to_string(),
                    param_type: ParamType::Integer,
                    description: "Timezone offset in hours from UTC (e.g., -5 for EST)".to_string(),
                    required: false,
                    default: Some(json!(0)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let timezone_offset = params["timezone_offset"]
            .as_i64()
            .unwrap_or(0);

        // Get current system time
        let now = chrono::Utc::now();
        let local_time = now + chrono::Duration::hours(timezone_offset);

        Ok(ToolResult {
            success: true,
            data: json!({
                "current_utc": now.to_rfc3339(),
                "target_time": local_time.to_rfc3339(),
                "timezone_offset": timezone_offset,
                "message": "DateTime sync requires manual configuration",
                "instructions": {
                    "step_1": "Navigate to Settings → System → Date & Time",
                    "step_2": format!("Set date: {}", local_time.format("%Y-%m-%d")),
                    "step_3": format!("Set time: {}", local_time.format("%H:%M:%S")),
                    "step_4": format!("Set timezone: UTC{:+}", timezone_offset)
                },
                "formatted_datetime": {
                    "date": local_time.format("%Y-%m-%d").to_string(),
                    "time": local_time.format("%H:%M:%S").to_string(),
                    "full": local_time.format("%Y-%m-%d %H:%M:%S").to_string()
                },
                "importance": [
                    "Accurate timestamps for captured signals",
                    "Proper file sorting by date",
                    "Correct log timestamps",
                    "Required for some security protocols"
                ],
                "note": "RPC protocol does not support datetime sync. Use device menu or qFlipper."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === LED Control Tool ===

pub struct LedControlTool;

#[async_trait]
impl PentestTool for LedControlTool {
    fn name(&self) -> &str {
        "flipper_led_control"
    }

    fn description(&self) -> &str {
        "Control Flipper Zero LED and backlight"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "brightness".to_string(),
                    param_type: ParamType::Integer,
                    description: "Screen brightness percentage (0-100)".to_string(),
                    required: false,
                    default: Some(json!(75)),
                },
                ToolParam {
                    name: "led_color".to_string(),
                    param_type: ParamType::String,
                    description: "LED color: red, green, blue, off (not supported via RPC)".to_string(),
                    required: false,
                    default: Some(json!("off")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let brightness = params["brightness"]
            .as_u64()
            .unwrap_or(75);

        let led_color = params["led_color"]
            .as_str()
            .unwrap_or("off");

        if brightness > 100 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Brightness must be 0-100".to_string()
            ));
        }

        let valid_colors = ["red", "green", "blue", "off"];
        if !valid_colors.contains(&led_color) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid LED color. Must be: {}", valid_colors.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "brightness": brightness,
                "led_color": led_color,
                "message": "LED/backlight control requires device settings or custom firmware",
                "brightness_settings": {
                    "low": "10-30% - Maximum battery life",
                    "medium": "40-60% - Balanced",
                    "high": "70-100% - Maximum visibility"
                },
                "instructions": {
                    "brightness": "Settings → Display → Brightness",
                    "led_control": "Not available in standard firmware",
                    "custom_firmware": "Unleashed/Xtreme firmware has LED control apps"
                },
                "led_usage": {
                    "system": "System uses LED for status (charging, alerts)",
                    "apps": "Some apps can control LED (custom firmware)",
                    "api": "GPIO operations can control LED programmatically"
                },
                "note": "Standard RPC does not support LED/backlight control. Use device menu or GPIO."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Vibration Control Tool ===

pub struct VibrationControlTool;

#[async_trait]
impl PentestTool for VibrationControlTool {
    fn name(&self) -> &str {
        "flipper_vibration_control"
    }

    fn description(&self) -> &str {
        "Control Flipper Zero vibration motor and haptic feedback"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "pattern".to_string(),
                    param_type: ParamType::String,
                    description: "Vibration pattern: short, long, double, triple, custom".to_string(),
                    required: false,
                    default: Some(json!("short")),
                },
                ToolParam {
                    name: "enabled".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Enable or disable vibration feedback".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let pattern = params["pattern"]
            .as_str()
            .unwrap_or("short");

        let enabled = params["enabled"]
            .as_bool()
            .unwrap_or(true);

        let valid_patterns = ["short", "long", "double", "triple", "custom"];
        if !valid_patterns.contains(&pattern) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid pattern. Must be: {}", valid_patterns.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "pattern": pattern,
                "enabled": enabled,
                "message": "Vibration control requires device settings or GPIO access",
                "patterns": {
                    "short": "50ms pulse - Button press feedback",
                    "long": "200ms pulse - Confirmation",
                    "double": "Two 50ms pulses - Warning",
                    "triple": "Three 50ms pulses - Alert",
                    "custom": "Programmable via GPIO or custom firmware"
                },
                "instructions": {
                    "enable_disable": "Settings → System → Vibration Motor",
                    "intensity": "Not adjustable in standard firmware",
                    "custom_patterns": "Requires GPIO control or custom app"
                },
                "use_cases": [
                    "Haptic feedback for UI navigation",
                    "Alert notifications",
                    "Silent operation mode indicators",
                    "Custom app interactions"
                ],
                "gpio_control": {
                    "pin": "Vibration motor on dedicated GPIO",
                    "method": "PWM control for intensity variation",
                    "api": "Available via GPIO operations tools"
                },
                "note": "Standard RPC does not support vibration control. Use device settings or GPIO."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === System Diagnostics Tool ===

pub struct SystemDiagnosticsTool;

#[async_trait]
impl PentestTool for SystemDiagnosticsTool {
    fn name(&self) -> &str {
        "flipper_system_diagnostics"
    }

    fn description(&self) -> &str {
        "Run comprehensive Flipper Zero system diagnostics"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "quick_check".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Run quick diagnostic check only".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let quick_check = params["quick_check"]
            .as_bool()
            .unwrap_or(false);

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let port = client.port().to_string();

        // Run health check
        let health = client.health_check().await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Check storage access
        let sd_accessible = client.list_directory("/ext", false).await.is_ok();
        let int_accessible = client.list_directory("/int", false).await.is_ok();

        // Test file operations
        let test_path = "/ext/.diag_test";
        let file_write = client.write_file(test_path, vec![0x42; 100]).await.is_ok();
        let file_read = if file_write {
            client.read_file(test_path).await.is_ok()
        } else {
            false
        };
        let _ = client.delete_path(test_path, false).await;

        let mut diagnostic_results = json!({
            "connection": {
                "status": if health { "OK" } else { "FAILED" },
                "port": port,
            },
            "storage": {
                "internal": if int_accessible { "OK" } else { "FAILED" },
                "external_sd": if sd_accessible { "OK" } else { "NOT DETECTED" },
            },
            "file_operations": {
                "write": if file_write { "OK" } else { "FAILED" },
                "read": if file_read { "OK" } else { "FAILED" },
            }
        });

        if !quick_check {
            diagnostic_results["extended_checks"] = json!({
                "message": "Extended diagnostics require device menu",
                "manual_tests": {
                    "display": "Check screen for dead pixels",
                    "buttons": "Test all 5 buttons (UP/DOWN/LEFT/RIGHT/OK/BACK)",
                    "sd_card": "Verify SD card in Settings → Storage",
                    "battery": "Check charge level in Settings → Power",
                    "bluetooth": "Test pairing in Settings → Bluetooth",
                    "ir": "Test IR transmitter with TV remote function",
                    "nfc": "Test NFC reader with card",
                    "rfid": "Test RFID reader with card"
                }
            });
        }

        let overall_status = if health && sd_accessible && file_write && file_read {
            "HEALTHY"
        } else if health {
            "DEGRADED"
        } else {
            "CRITICAL"
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "overall_status": overall_status,
                "quick_check": quick_check,
                "diagnostics": diagnostic_results,
                "message": format!("System diagnostics completed - Status: {}", overall_status),
                "recommendations": if overall_status == "HEALTHY" {
                    vec!["All systems operational"]
                } else {
                    vec![
                        "Check SD card connection if not detected",
                        "Verify firmware is up to date",
                        "Try power cycle if issues persist",
                        "Check qFlipper for error logs"
                    ]
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
