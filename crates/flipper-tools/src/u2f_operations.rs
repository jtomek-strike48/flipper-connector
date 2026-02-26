//! U2F/FIDO2 Security Key Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use serde_json::{json, Value};

// === U2F Register Tool ===

pub struct U2fRegisterTool;

#[async_trait]
impl PentestTool for U2fRegisterTool {
    fn name(&self) -> &str {
        "flipper_u2f_register"
    }

    fn description(&self) -> &str {
        "Register U2F security key with a service"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "app_id".to_string(),
                    param_type: ParamType::String,
                    description: "Application ID (usually domain URL)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "challenge".to_string(),
                    param_type: ParamType::String,
                    description: "Registration challenge (base64 or hex)".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let app_id = params["app_id"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing app_id parameter".to_string()))?;

        let challenge = params["challenge"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing challenge parameter".to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "app_id": app_id,
                "challenge": challenge,
                "message": "U2F registration prepared",
                "instructions": "Use Flipper Zero U2F app: Apps → Tools → U2F → Register",
                "note": "Registration generates key pair and returns public key + key handle",
                "security": {
                    "attestation": "Device attestation certificate included",
                    "counter": "Signature counter initialized to 0",
                    "key_handle": "Unique per-service key handle generated"
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === U2F Authenticate Tool ===

pub struct U2fAuthenticateTool;

#[async_trait]
impl PentestTool for U2fAuthenticateTool {
    fn name(&self) -> &str {
        "flipper_u2f_authenticate"
    }

    fn description(&self) -> &str {
        "Authenticate with U2F security key"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "app_id".to_string(),
                    param_type: ParamType::String,
                    description: "Application ID (must match registration)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "challenge".to_string(),
                    param_type: ParamType::String,
                    description: "Authentication challenge (base64 or hex)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "key_handle".to_string(),
                    param_type: ParamType::String,
                    description: "Key handle from registration".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let app_id = params["app_id"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing app_id parameter".to_string()))?;

        let challenge = params["challenge"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing challenge parameter".to_string()))?;

        let key_handle = params["key_handle"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing key_handle parameter".to_string()))?;

        Ok(ToolResult {
            success: true,
            data: json!({
                "app_id": app_id,
                "challenge": challenge,
                "key_handle": key_handle,
                "message": "U2F authentication prepared",
                "instructions": "Use Flipper Zero U2F app: Apps → Tools → U2F → Authenticate",
                "note": "Authentication signs challenge with private key and increments counter",
                "security": {
                    "signature": "ECDSA signature over challenge + counter",
                    "counter": "Incremented on each authentication",
                    "user_presence": "Physical button press required"
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === FIDO2 Register Tool ===

pub struct Fido2RegisterTool;

#[async_trait]
impl PentestTool for Fido2RegisterTool {
    fn name(&self) -> &str {
        "flipper_fido2_register"
    }

    fn description(&self) -> &str {
        "Register FIDO2/WebAuthn authenticator"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "rp_id".to_string(),
                    param_type: ParamType::String,
                    description: "Relying Party ID (domain)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "rp_name".to_string(),
                    param_type: ParamType::String,
                    description: "Relying Party name (display name)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "user_id".to_string(),
                    param_type: ParamType::String,
                    description: "User ID (base64 or hex)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "user_name".to_string(),
                    param_type: ParamType::String,
                    description: "User name (email or username)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "challenge".to_string(),
                    param_type: ParamType::String,
                    description: "Registration challenge (base64 or hex)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "resident_key".to_string(),
                    param_type: ParamType::Boolean,
                    description: "Create resident key (stored on device)".to_string(),
                    required: false,
                    default: Some(json!(false)),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let rp_id = params["rp_id"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing rp_id parameter".to_string()))?;

        let rp_name = params["rp_name"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing rp_name parameter".to_string()))?;

        let user_id = params["user_id"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing user_id parameter".to_string()))?;

        let user_name = params["user_name"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing user_name parameter".to_string()))?;

        let challenge = params["challenge"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing challenge parameter".to_string()))?;

        let resident_key = params["resident_key"]
            .as_bool()
            .unwrap_or(false);

        Ok(ToolResult {
            success: true,
            data: json!({
                "rp_id": rp_id,
                "rp_name": rp_name,
                "user_id": user_id,
                "user_name": user_name,
                "challenge": challenge,
                "resident_key": resident_key,
                "message": "FIDO2 registration prepared",
                "instructions": "Use Flipper Zero FIDO2 app: Apps → Tools → FIDO2 → Register",
                "note": "FIDO2 supports passwordless authentication and biometric options",
                "features": {
                    "algorithms": ["ES256 (ECDSA P-256)", "EdDSA (Ed25519)"],
                    "extensions": ["credProtect", "hmac-secret"],
                    "resident_keys": if resident_key { "Enabled (stored on device)" } else { "Disabled (server-side storage)" },
                    "user_verification": "PIN or biometric supported"
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === FIDO2 Authenticate Tool ===

pub struct Fido2AuthenticateTool;

#[async_trait]
impl PentestTool for Fido2AuthenticateTool {
    fn name(&self) -> &str {
        "flipper_fido2_authenticate"
    }

    fn description(&self) -> &str {
        "Authenticate with FIDO2/WebAuthn"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "rp_id".to_string(),
                    param_type: ParamType::String,
                    description: "Relying Party ID (must match registration)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "challenge".to_string(),
                    param_type: ParamType::String,
                    description: "Authentication challenge (base64 or hex)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "credential_id".to_string(),
                    param_type: ParamType::String,
                    description: "Credential ID from registration (optional if resident key)".to_string(),
                    required: false,
                    default: None,
                },
                ToolParam {
                    name: "user_verification".to_string(),
                    param_type: ParamType::String,
                    description: "User verification level: discouraged, preferred, required".to_string(),
                    required: false,
                    default: Some(json!("preferred")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let rp_id = params["rp_id"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing rp_id parameter".to_string()))?;

        let challenge = params["challenge"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("Missing challenge parameter".to_string()))?;

        let credential_id = params["credential_id"].as_str();
        let user_verification = params["user_verification"]
            .as_str()
            .unwrap_or("preferred");

        // Validate user verification level
        let valid_levels = ["discouraged", "preferred", "required"];
        if !valid_levels.contains(&user_verification) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid user_verification. Must be: {}", valid_levels.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "rp_id": rp_id,
                "challenge": challenge,
                "credential_id": credential_id,
                "user_verification": user_verification,
                "message": "FIDO2 authentication prepared",
                "instructions": "Use Flipper Zero FIDO2 app: Apps → Tools → FIDO2 → Authenticate",
                "note": "Passwordless authentication with biometric or PIN verification",
                "security": {
                    "attestation": "Optional attestation statement",
                    "counter": "Signature counter prevents cloning",
                    "user_presence": "Physical interaction required",
                    "user_verification": format!("User verification: {}", user_verification)
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
