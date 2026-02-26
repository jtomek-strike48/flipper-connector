//! Power Management Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Battery Info Tool ===

pub struct BatteryInfoTool;

#[async_trait]
impl PentestTool for BatteryInfoTool {
    fn name(&self) -> &str {
        "flipper_battery_info"
    }

    fn description(&self) -> &str {
        "Get Flipper Zero battery status and health information"
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
        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Verify connection
        let is_connected = client.health_check().await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "connected": is_connected,
                "message": "Battery info requires device menu or extended RPC",
                "instructions": {
                    "device_menu": "Check Settings → Power on Flipper Zero",
                    "display_info": "Battery percentage shown on main screen",
                    "extended_api": "Custom firmware may expose battery telemetry"
                },
                "battery_indicators": {
                    "charging": "Lightning bolt icon when USB connected",
                    "percentage": "Shown in top-right corner",
                    "low_battery": "Warning at 10% and 5%",
                    "critical": "Auto-shutdown at ~2%"
                },
                "battery_specs": {
                    "type": "2000mAh Li-Po battery",
                    "voltage": "3.7V nominal",
                    "charging": "5V USB-C",
                    "runtime": "~1-7 days depending on usage"
                },
                "note": "Standard RPC does not expose battery telemetry"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Power Mode Tool ===

pub struct PowerModeTool;

#[async_trait]
impl PentestTool for PowerModeTool {
    fn name(&self) -> &str {
        "flipper_power_mode"
    }

    fn description(&self) -> &str {
        "Control Flipper Zero power modes and sleep settings"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "mode".to_string(),
                    param_type: ParamType::String,
                    description: "Power mode: normal, low_power, auto_sleep".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "sleep_timeout_sec".to_string(),
                    param_type: ParamType::Integer,
                    description: "Auto-sleep timeout in seconds (0 to disable)".to_string(),
                    required: false,
                    default: Some(json!(90)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mode = params["mode"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("mode parameter required".to_string()))?;

        let sleep_timeout = params["sleep_timeout_sec"]
            .as_u64()
            .unwrap_or(90);

        let valid_modes = ["normal", "low_power", "auto_sleep"];
        if !valid_modes.contains(&mode) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid mode. Must be: {}", valid_modes.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "mode": mode,
                "sleep_timeout_sec": sleep_timeout,
                "message": "Power mode configuration prepared",
                "instructions": {
                    "normal": "Settings → Display → Always On (disable auto-lock)",
                    "low_power": "Settings → Display → reduce brightness, enable auto-lock",
                    "auto_sleep": format!("Settings → Display → Auto Lock: {} seconds", sleep_timeout)
                },
                "power_saving_tips": [
                    "Reduce screen brightness",
                    "Enable auto-lock/sleep",
                    "Disable Bluetooth when not needed",
                    "Use USB passthrough for long sessions",
                    "Close apps when finished"
                ],
                "power_modes": {
                    "active": "Normal operation, full power",
                    "screen_off": "Screen off, systems active (~50% power)",
                    "sleep": "Deep sleep, wake on button (~5% power)",
                    "off": "Powered off, no consumption"
                },
                "note": "Power mode control requires device menu. RPC cannot modify system settings."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Charging Status Tool ===

pub struct ChargingStatusTool;

#[async_trait]
impl PentestTool for ChargingStatusTool {
    fn name(&self) -> &str {
        "flipper_charging_status"
    }

    fn description(&self) -> &str {
        "Check Flipper Zero charging status and USB connection"
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
        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        let port = client.port().to_string();
        let is_connected = client.health_check().await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // If connected via USB, it's definitely charging or fully charged
        let usb_connected = is_connected;

        Ok(ToolResult {
            success: true,
            data: json!({
                "usb_connected": usb_connected,
                "port": port,
                "message": if usb_connected {
                    "USB connected - device is charging or fully charged"
                } else {
                    "USB not connected - running on battery"
                },
                "charging_indicators": {
                    "screen": "Lightning bolt icon indicates charging",
                    "led": "No dedicated charging LED",
                    "full_charge": "Icon changes when battery at 100%"
                },
                "charging_specs": {
                    "input": "5V USB-C",
                    "charge_current": "~500mA typical",
                    "charge_time": "~4 hours from empty to full",
                    "usb_modes": "Supports USB 2.0 data + power"
                },
                "usb_modes": {
                    "vcp": "Virtual COM Port (serial)",
                    "charging": "Charge-only mode",
                    "vcp_charging": "Data + charging (default)"
                },
                "note": "USB connection detected via serial port. Full telemetry requires extended API."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Power Optimization Tool ===

pub struct PowerOptimizeTool;

#[async_trait]
impl PentestTool for PowerOptimizeTool {
    fn name(&self) -> &str {
        "flipper_power_optimize"
    }

    fn description(&self) -> &str {
        "Get power optimization recommendations for Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "use_case".to_string(),
                    param_type: ParamType::String,
                    description: "Use case: pentest, storage, development, standby".to_string(),
                    required: false,
                    default: Some(json!("pentest")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let use_case = params["use_case"]
            .as_str()
            .unwrap_or("pentest");

        let recommendations = match use_case {
            "pentest" => json!({
                "priority": "Balance performance and battery life",
                "settings": [
                    "Medium screen brightness (50-70%)",
                    "Auto-lock: 2-5 minutes",
                    "Bluetooth: Enable only when needed",
                    "Keep USB cable for extended operations"
                ],
                "expected_runtime": "4-8 hours of active use"
            }),
            "storage" => json!({
                "priority": "Maximize battery life, minimal usage",
                "settings": [
                    "Minimum screen brightness (10-20%)",
                    "Auto-lock: 30 seconds",
                    "Bluetooth: Disabled",
                    "Close all apps after use"
                ],
                "expected_runtime": "5-7 days on standby"
            }),
            "development" => json!({
                "priority": "Always-on, USB powered",
                "settings": [
                    "High screen brightness (80-100%)",
                    "Auto-lock: Disabled",
                    "USB cable connected (powered)",
                    "Bluetooth: As needed"
                ],
                "expected_runtime": "Unlimited (USB powered)"
            }),
            "standby" => json!({
                "priority": "Maximum battery preservation",
                "settings": [
                    "Power off when not in use",
                    "If on: minimum brightness, 30s auto-lock",
                    "All radios disabled",
                    "Remove SD card if not needed"
                ],
                "expected_runtime": "Weeks to months in deep sleep"
            }),
            _ => json!({
                "error": "Unknown use case",
                "valid_cases": ["pentest", "storage", "development", "standby"]
            })
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "use_case": use_case,
                "recommendations": recommendations,
                "general_tips": [
                    "Lower screen brightness = longer battery",
                    "Bluetooth uses significant power when active",
                    "Sub-GHz operations are power-intensive",
                    "GPIO operations drain battery quickly",
                    "Sleep mode extends battery dramatically"
                ],
                "power_consumption_estimates": {
                    "screen_on_idle": "~50-100mA",
                    "bluetooth_active": "+30-50mA",
                    "subghz_rx": "+50-80mA",
                    "subghz_tx": "+100-150mA",
                    "gpio_active": "+20-100mA (varies)",
                    "sleep_mode": "~1-5mA"
                },
                "message": format!("Power optimization recommendations for: {}", use_case)
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
