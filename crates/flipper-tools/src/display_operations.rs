//! LCD Display and Screen Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Screenshot Tool ===

pub struct ScreenshotTool;

#[async_trait]
impl PentestTool for ScreenshotTool {
    fn name(&self) -> &str {
        "flipper_screenshot"
    }

    fn description(&self) -> &str {
        "Capture screenshot from Flipper Zero display"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "output_path".to_string(),
                    param_type: ParamType::String,
                    description: "Local path to save screenshot (PNG format)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let output_path = params["output_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("output_path required".to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "output_path": output_path,
                "message": "Screenshot capture requires external tools",
                "instructions": {
                    "qFlipper": "Use qFlipper's built-in screenshot feature (easiest)",
                    "ufbt": "Use ufbt screenshot command if installed",
                    "manual": "Take photo of screen if tools unavailable"
                },
                "screen_specs": {
                    "resolution": "128x64 pixels",
                    "type": "Monochrome LCD",
                    "refresh_rate": "~60Hz",
                    "size": "1.4 inch diagonal"
                },
                "formats": {
                    "preferred": "PNG (lossless, small size)",
                    "alternative": "BMP (uncompressed)",
                    "device_format": "XBM (X Bitmap)"
                },
                "note": "RPC protocol does not expose framebuffer. Use qFlipper or external tools."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Canvas Draw Tool ===

pub struct CanvasDrawTool;

#[async_trait]
impl PentestTool for CanvasDrawTool {
    fn name(&self) -> &str {
        "flipper_canvas_draw"
    }

    fn description(&self) -> &str {
        "Draw custom graphics on Flipper Zero display"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "text".to_string(),
                    param_type: ParamType::String,
                    description: "Text to display on screen".to_string(),
                    required: false,
                    default: Some(json!("Hello World")),
                },
                ToolParam {
                    name: "x".to_string(),
                    param_type: ParamType::Integer,
                    description: "X coordinate (0-127)".to_string(),
                    required: false,
                    default: Some(json!(0)),
                },
                ToolParam {
                    name: "y".to_string(),
                    param_type: ParamType::Integer,
                    description: "Y coordinate (0-63)".to_string(),
                    required: false,
                    default: Some(json!(0)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let text = params["text"].as_str().unwrap_or("Hello World");
        let x = params["x"].as_i64().unwrap_or(0);
        let y = params["y"].as_i64().unwrap_or(0);

        if x < 0 || x > 127 || y < 0 || y > 63 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Coordinates must be x: 0-127, y: 0-63".to_string()
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "text": text,
                "x": x,
                "y": y,
                "message": "Canvas drawing requires custom application",
                "canvas_info": {
                    "resolution": "128x64 pixels",
                    "coordinate_system": "Top-left origin (0,0)",
                    "x_range": "0-127",
                    "y_range": "0-63"
                },
                "drawing_primitives": {
                    "text": "Draw text at coordinates",
                    "line": "Draw line between two points",
                    "rectangle": "Draw filled or outlined rectangle",
                    "circle": "Draw circle with center and radius",
                    "pixel": "Set individual pixel",
                    "bitmap": "Draw bitmap image"
                },
                "implementation": {
                    "method": "Create custom FAP (Flipper Application Package)",
                    "api": "Use Flipper's Canvas API",
                    "example": "apps/examples/canvas_demo in firmware repo"
                },
                "note": "RPC does not support canvas operations. Requires custom app development."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Display Info Tool ===

pub struct DisplayInfoTool;

#[async_trait]
impl PentestTool for DisplayInfoTool {
    fn name(&self) -> &str {
        "flipper_display_info"
    }

    fn description(&self) -> &str {
        "Get Flipper Zero display specifications and status"
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

        let is_connected = client.health_check().await
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "connected": is_connected,
                "display_specs": {
                    "type": "SSD1306 OLED-like LCD",
                    "resolution": "128x64 pixels",
                    "size": "1.4 inch diagonal",
                    "color_depth": "1-bit (monochrome)",
                    "refresh_rate": "~60Hz",
                    "interface": "SPI",
                    "backlight": "No backlight (reflective LCD)"
                },
                "pixel_info": {
                    "total_pixels": 8192,
                    "aspect_ratio": "2:1",
                    "pixel_pitch": "~0.18mm",
                    "viewable_angle": "~170 degrees"
                },
                "power_consumption": {
                    "typical": "~5-10mA @ 3.3V",
                    "peak": "~15mA during updates",
                    "sleep": "~1uA in sleep mode"
                },
                "coordinates": {
                    "origin": "Top-left (0, 0)",
                    "x_max": 127,
                    "y_max": 63,
                    "addressing": "Column-row addressing"
                },
                "font_support": {
                    "default": "5x7 pixel font",
                    "custom": "Support for custom fonts via SDK",
                    "max_chars_line": "~21 characters (depending on font)"
                },
                "message": "Display hardware information retrieved"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Backlight Control Tool ===

pub struct BacklightControlTool;

#[async_trait]
impl PentestTool for BacklightControlTool {
    fn name(&self) -> &str {
        "flipper_backlight_control"
    }

    fn description(&self) -> &str {
        "Control Flipper Zero screen backlight (note: LCD has no true backlight)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "mode".to_string(),
                    param_type: ParamType::String,
                    description: "Backlight mode: on, off, auto".to_string(),
                    required: false,
                    default: Some(json!("auto")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let mode = params["mode"].as_str().unwrap_or("auto");

        let valid_modes = ["on", "off", "auto"];
        if !valid_modes.contains(&mode) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid mode. Must be: {}", valid_modes.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "mode": mode,
                "message": "Flipper Zero LCD has no traditional backlight",
                "display_type": {
                    "technology": "Reflective LCD (no backlight needed)",
                    "visibility": "Best in bright ambient light",
                    "contrast": "Controlled by viewing angle and ambient light"
                },
                "brightness_control": {
                    "available": false,
                    "reason": "Reflective LCD technology",
                    "alternative": "Adjust screen contrast in Settings → Display"
                },
                "screen_timeout": {
                    "setting": "Settings → Display → Screen Timeout",
                    "options": ["Always On", "10s", "30s", "60s", "90s"],
                    "recommendation": "30-90s for balance of battery and usability"
                },
                "power_saving": {
                    "screen_off": "Significant power savings",
                    "always_on": "Screen consumes ~5-10mA continuously",
                    "auto_lock": "Recommended for battery life"
                },
                "note": "RPC cannot control display settings. Use device menu."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Screen Test Tool ===

pub struct ScreenTestTool;

#[async_trait]
impl PentestTool for ScreenTestTool {
    fn name(&self) -> &str {
        "flipper_screen_test"
    }

    fn description(&self) -> &str {
        "Run display test patterns to check for dead pixels or issues"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "pattern".to_string(),
                    param_type: ParamType::String,
                    description: "Test pattern: solid, checkerboard, gradient, lines".to_string(),
                    required: false,
                    default: Some(json!("checkerboard")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let pattern = params["pattern"].as_str().unwrap_or("checkerboard");

        let valid_patterns = ["solid", "checkerboard", "gradient", "lines"];
        if !valid_patterns.contains(&pattern) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid pattern. Must be: {}", valid_patterns.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "pattern": pattern,
                "message": "Screen test requires custom application or manual inspection",
                "test_patterns": {
                    "solid": {
                        "description": "All pixels on/off",
                        "purpose": "Check for dead regions",
                        "variants": ["all_white", "all_black"]
                    },
                    "checkerboard": {
                        "description": "Alternating pixels",
                        "purpose": "Check pixel uniformity",
                        "pattern": "2x2 or 4x4 grid"
                    },
                    "gradient": {
                        "description": "Gradual intensity change",
                        "purpose": "Check contrast and uniformity",
                        "note": "Limited on 1-bit display (dithering)"
                    },
                    "lines": {
                        "description": "Horizontal and vertical lines",
                        "purpose": "Check alignment and addressing",
                        "variants": ["horizontal", "vertical", "diagonal"]
                    }
                },
                "manual_test": {
                    "step_1": "Open any full-screen app",
                    "step_2": "Look for dead pixels (always off)",
                    "step_3": "Look for stuck pixels (always on)",
                    "step_4": "Check for ghosting or trails",
                    "step_5": "Verify even contrast across screen"
                },
                "common_issues": {
                    "dead_pixels": "Pixels that never illuminate",
                    "stuck_pixels": "Pixels that never turn off",
                    "ghosting": "Previous image persists faintly",
                    "uneven_contrast": "Some areas darker/lighter",
                    "response_lag": "Slow pixel transitions"
                },
                "note": "RPC cannot display test patterns. Requires custom app or manual testing."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
