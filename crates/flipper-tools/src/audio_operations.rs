//! Audio and Speaker Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Speaker Control Tool ===

pub struct SpeakerControlTool;

#[async_trait]
impl PentestTool for SpeakerControlTool {
    fn name(&self) -> &str {
        "flipper_speaker_control"
    }

    fn description(&self) -> &str {
        "Control Flipper Zero speaker and buzzer"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "enabled".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Enable or disable speaker".to_string(),
                    required: false,
                    default: Some(json!(true)),
                },
                ToolParam {
                    name: "volume".to_string(),
                    param_type: ParamType::Integer,
                    description: "Volume percentage (0-100)".to_string(),
                    required: false,
                    default: Some(json!(50)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let enabled = params["enabled"].as_bool().unwrap_or(true);
        let volume = params["volume"].as_u64().unwrap_or(50);

        if volume > 100 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Volume must be 0-100".to_string()
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "enabled": enabled,
                "volume": volume,
                "message": "Speaker control requires device settings or GPIO",
                "speaker_specs": {
                    "type": "Piezo buzzer",
                    "frequency_range": "~20Hz - 20kHz",
                    "typical_freq": "400Hz - 4kHz (most audible)",
                    "max_volume": "~80dB at 10cm",
                    "power": "GPIO PWM controlled"
                },
                "volume_control": {
                    "setting": "Settings → System → Sound",
                    "options": ["On", "Off"],
                    "note": "No fine-grained volume control in standard firmware"
                },
                "use_cases": [
                    "UI feedback (button clicks)",
                    "Alerts and notifications",
                    "BadUSB attack indicators",
                    "Music playback (simple tones)",
                    "Morse code output",
                    "DTMF tone generation"
                ],
                "gpio_control": {
                    "pin": "Speaker on dedicated GPIO with PWM",
                    "method": "PWM duty cycle controls volume",
                    "api": "Available via GPIO operations or custom app"
                },
                "note": "RPC does not support speaker control. Use device settings or GPIO."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Tone Generation Tool ===

pub struct ToneGeneratorTool;

#[async_trait]
impl PentestTool for ToneGeneratorTool {
    fn name(&self) -> &str {
        "flipper_tone_generate"
    }

    fn description(&self) -> &str {
        "Generate audio tones and beeps on Flipper Zero speaker"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "frequency_hz".to_string(),
                    param_type: ParamType::Integer,
                    description: "Tone frequency in Hz (20-20000)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "duration_ms".to_string(),
                    param_type: ParamType::Integer,
                    description: "Tone duration in milliseconds".to_string(),
                    required: false,
                    default: Some(json!(500)),
                },
                ToolParam {
                    name: "waveform".to_string(),
                    param_type: ParamType::String,
                    description: "Waveform type: square, sine (square only on piezo)".to_string(),
                    required: false,
                    default: Some(json!("square")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let frequency = params["frequency_hz"]
            .as_u64()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("frequency_hz required".to_string()))?;

        let duration = params["duration_ms"].as_u64().unwrap_or(500);
        let waveform = params["waveform"].as_str().unwrap_or("square");

        if frequency < 20 || frequency > 20000 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Frequency must be 20-20000 Hz".to_string()
            ));
        }

        let valid_waveforms = ["square", "sine"];
        if !valid_waveforms.contains(&waveform) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid waveform. Must be: {}", valid_waveforms.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "frequency_hz": frequency,
                "duration_ms": duration,
                "waveform": waveform,
                "message": "Tone generation requires custom application",
                "frequency_info": {
                    "sub_bass": "20-60 Hz (barely audible on piezo)",
                    "bass": "60-250 Hz (weak on piezo)",
                    "midrange": "250-4000 Hz (optimal for piezo)",
                    "treble": "4000-20000 Hz (piezo efficient)"
                },
                "common_tones": {
                    "beep": "1000 Hz",
                    "warning": "800 Hz",
                    "error": "400 Hz",
                    "success": "1200 Hz",
                    "dtmf_0": "941/1336 Hz",
                    "dtmf_1": "697/1209 Hz"
                },
                "musical_notes": {
                    "C4": "261.63 Hz (middle C)",
                    "A4": "440.00 Hz (concert pitch)",
                    "C5": "523.25 Hz",
                    "note": "Octave doubles/halves frequency"
                },
                "implementation": {
                    "method": "Custom FAP with speaker API",
                    "api": "speaker_start(), speaker_stop()",
                    "pwm": "GPIO PWM for frequency control",
                    "example": "Music player app in firmware"
                },
                "note": "RPC does not support audio output. Requires custom app with speaker API."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Music Player Tool ===

pub struct MusicPlayerTool;

#[async_trait]
impl PentestTool for MusicPlayerTool {
    fn name(&self) -> &str {
        "flipper_music_play"
    }

    fn description(&self) -> &str {
        "Play music and melodies on Flipper Zero speaker"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "melody".to_string(),
                    param_type: ParamType::String,
                    description: "Melody name: startup, success, error, custom".to_string(),
                    required: false,
                    default: Some(json!("startup")),
                },
                ToolParam {
                    name: "rtttl".to_string(),
                    param_type: ParamType::String,
                    description: "RTTTL format melody string (optional)".to_string(),
                    required: false,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let melody = params["melody"].as_str().unwrap_or("startup");
        let rtttl = params["rtttl"].as_str();

        Ok(ToolResult {
            success: true,
            data: json!({
                "melody": melody,
                "rtttl": rtttl,
                "message": "Music playback requires music player app",
                "music_formats": {
                    "rtttl": {
                        "name": "Ring Tone Text Transfer Language",
                        "description": "Simple text-based format for melodies",
                        "example": "Tetris:d=4,o=5,b=160:e6,8b,8c6,8d6,16e6,16d6,8c6,8b",
                        "support": "Supported by Flipper music player apps"
                    },
                    "fmf": {
                        "name": "Flipper Music Format",
                        "description": "Custom format for Flipper Zero",
                        "location": "/ext/music/*.fmf",
                        "support": "Native format"
                    }
                },
                "built_in_melodies": {
                    "startup": "Device boot sound",
                    "success": "Operation success chime",
                    "error": "Error sound",
                    "levelup": "Achievement sound",
                    "warning": "Warning beep"
                },
                "music_player_apps": [
                    "Music Player (built-in)",
                    "Music Beeper (custom firmware)",
                    "RTTTL Player (community app)"
                ],
                "file_locations": {
                    "music": "/ext/music/",
                    "rtttl": "/ext/music/*.rtttl",
                    "fmf": "/ext/music/*.fmf"
                },
                "implementation": {
                    "app": "Use Music Player app on device",
                    "custom": "Create FAP with RTTTL parser",
                    "api": "speaker_start() with frequency array"
                },
                "note": "RPC does not support music playback. Use Music Player app or custom FAP."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Audio Alert Tool ===

pub struct AudioAlertTool;

#[async_trait]
impl PentestTool for AudioAlertTool {
    fn name(&self) -> &str {
        "flipper_audio_alert"
    }

    fn description(&self) -> &str {
        "Trigger audio alerts and notification sounds"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "alert_type".to_string(),
                    param_type: ParamType::String,
                    description: "Alert type: info, success, warning, error, critical".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "repeat".to_string(),
                    param_type: ParamType::Integer,
                    description: "Number of times to repeat alert".to_string(),
                    required: false,
                    default: Some(json!(1)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let alert_type = params["alert_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("alert_type required".to_string()))?;

        let repeat = params["repeat"].as_u64().unwrap_or(1);

        let valid_types = ["info", "success", "warning", "error", "critical"];
        if !valid_types.contains(&alert_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid alert_type. Must be: {}", valid_types.join(", "))
            ));
        }

        let alert_pattern = match alert_type {
            "info" => json!({
                "frequency": "1000 Hz",
                "duration": "100ms",
                "pattern": "Single beep"
            }),
            "success" => json!({
                "frequency": "1200 Hz",
                "duration": "200ms",
                "pattern": "Two short beeps"
            }),
            "warning" => json!({
                "frequency": "800 Hz",
                "duration": "300ms",
                "pattern": "Three medium beeps"
            }),
            "error" => json!({
                "frequency": "400 Hz",
                "duration": "500ms",
                "pattern": "Long low beep"
            }),
            "critical" => json!({
                "frequency": "600 Hz alternating 400 Hz",
                "duration": "1000ms",
                "pattern": "Alternating siren pattern"
            }),
            _ => json!({"error": "Unknown type"})
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "alert_type": alert_type,
                "repeat": repeat,
                "alert_pattern": alert_pattern,
                "message": "Audio alerts require custom application",
                "alert_use_cases": {
                    "info": "General notifications, button feedback",
                    "success": "Operation completed successfully",
                    "warning": "Caution required, check status",
                    "error": "Operation failed, user action needed",
                    "critical": "Urgent attention required, security alert"
                },
                "accessibility": {
                    "visual_impaired": "Audio feedback crucial for navigation",
                    "deaf_users": "Pair with vibration motor",
                    "silent_mode": "Option to disable in settings"
                },
                "implementation": {
                    "method": "Custom FAP with speaker API",
                    "pattern": "Define frequency, duration, repeat arrays",
                    "api": "speaker_start(), delay(), speaker_stop() loop"
                },
                "note": "RPC does not support audio alerts. Requires custom app or system events."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Volume Control Tool ===

pub struct VolumeControlTool;

#[async_trait]
impl PentestTool for VolumeControlTool {
    fn name(&self) -> &str {
        "flipper_volume_control"
    }

    fn description(&self) -> &str {
        "Control audio volume and sound settings"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "volume".to_string(),
                    param_type: ParamType::Integer,
                    description: "Volume level (0-100)".to_string(),
                    required: false,
                    default: Some(json!(75)),
                },
                ToolParam {
                    name: "mute".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Mute all audio output".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let volume = params["volume"].as_u64().unwrap_or(75);
        let mute = params["mute"].as_bool().unwrap_or(false);

        if volume > 100 {
            return Err(flipper_core::error::Error::InvalidParams(
                "Volume must be 0-100".to_string()
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "volume": volume,
                "mute": mute,
                "message": "Volume control requires device settings or custom firmware",
                "standard_firmware": {
                    "volume_control": false,
                    "options": ["On", "Off"],
                    "setting": "Settings → System → Sound",
                    "note": "Binary on/off only"
                },
                "custom_firmware": {
                    "unleashed": "Some firmware versions have volume control",
                    "xtreme": "May include volume slider",
                    "check": "Varies by firmware version"
                },
                "hardware_limitation": {
                    "speaker": "Piezo buzzer",
                    "volume_method": "PWM duty cycle",
                    "range": "Limited dynamic range",
                    "note": "Not true analog volume control"
                },
                "software_volume": {
                    "implementation": "PWM duty cycle adjustment",
                    "granularity": "Typically 3-5 levels max",
                    "api": "Custom app can implement via GPIO PWM"
                },
                "alternatives": {
                    "mute": "Turn sound fully off in settings",
                    "app_control": "Some apps have internal volume",
                    "physical": "Cover speaker with hand/tape for quieter operation"
                },
                "note": "RPC does not support volume control. Standard firmware is binary on/off."
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
