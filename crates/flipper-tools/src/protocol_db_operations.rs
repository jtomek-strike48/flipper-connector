//! Protocol Database Management Operations

use async_trait::async_trait;
use flipper_core::tools::{ParamType, PentestTool, Platform, ToolContext, ToolParam, ToolResult, ToolSchema};
use flipper_protocol::FlipperClient;
use serde_json::{json, Value};

// === Database Info Tool ===

pub struct DatabaseInfoTool;

#[async_trait]
impl PentestTool for DatabaseInfoTool {
    fn name(&self) -> &str {
        "flipper_database_info"
    }

    fn description(&self) -> &str {
        "Get information about protocol databases (NFC, RFID, Sub-GHz, IR)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "database_type".to_string(),
                    param_type: ParamType::String,
                    description: "Database type: nfc, rfid, subghz, infrared, all".to_string(),
                    required: false,
                    default: Some(json!("all")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let database_type = params["database_type"].as_str().unwrap_or("all");

        let valid_types = ["nfc", "rfid", "subghz", "infrared", "all"];
        if !valid_types.contains(&database_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid database_type. Must be: {}", valid_types.join(", "))
            ));
        }

        Ok(ToolResult {
            success: true,
            data: json!({
                "database_type": database_type,
                "message": "Protocol database information",
                "databases": {
                    "nfc": {
                        "location": "/ext/nfc/assets/",
                        "file_format": "*.nfc",
                        "protocols": ["MiFare Classic", "MiFare Ultralight", "NTAG", "EMV", "NFC-A/B/F/V"],
                        "keys_db": "/int/assets/nfc/mf_classic_dict.txt (MiFare keys)",
                        "updateable": true,
                        "community_dbs": "MiFare key databases available online"
                    },
                    "rfid": {
                        "location": "/ext/lfrfid/",
                        "file_format": "*.rfid",
                        "protocols": ["EM4100", "HID Prox", "Indala", "T5577"],
                        "keys_db": "No key database (protocol-specific)",
                        "updateable": false,
                        "note": "Most RFID protocols don't use keys"
                    },
                    "subghz": {
                        "location": "/ext/subghz/assets/",
                        "file_format": "*.sub",
                        "protocols": ["KeeLoq", "Princeton", "Came", "Nice", "BFT"],
                        "keys_db": "/int/assets/subghz/keeloq_mfcodes (manufacturer codes)",
                        "updateable": true,
                        "community_dbs": "KeeLoq databases, frequency lists"
                    },
                    "infrared": {
                        "location": "/ext/infrared/assets/",
                        "file_format": "*.ir",
                        "protocols": ["NEC", "Samsung", "RC5", "RC6", "Sony SIRC"],
                        "keys_db": "Built-in universal remote database",
                        "updateable": true,
                        "community_dbs": "IRDB - huge community IR database"
                    }
                },
                "database_sources": {
                    "official": "Firmware updates include database updates",
                    "unleashed": "Extended databases in Unleashed firmware",
                    "community": "GitHub repositories with protocol databases",
                    "irdb": "https://github.com/Flipper-IRDB/IRDB",
                    "mifare_keys": "Various MiFare key dictionaries online"
                },
                "update_methods": [
                    "Firmware updates (automatic)",
                    "qFlipper file transfer",
                    "SD card file copy",
                    "Custom firmware with extended databases"
                ]
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Database Update Tool ===

pub struct DatabaseUpdateTool;

#[async_trait]
impl PentestTool for DatabaseUpdateTool {
    fn name(&self) -> &str {
        "flipper_database_update"
    }

    fn description(&self) -> &str {
        "Update protocol databases with new entries"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "database_type".to_string(),
                    param_type: ParamType::String,
                    description: "Database to update: nfc, rfid, subghz, infrared".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "source_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to updated database file".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let database_type = params["database_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("database_type required".to_string()))?;

        let source_path = params["source_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("source_path required".to_string()))?;

        let valid_types = ["nfc", "rfid", "subghz", "infrared"];
        if !valid_types.contains(&database_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid database_type. Must be: {}", valid_types.join(", "))
            ));
        }

        let target_path = match database_type {
            "nfc" => "/int/assets/nfc/mf_classic_dict.txt",
            "subghz" => "/int/assets/subghz/keeloq_mfcodes",
            "infrared" => "/ext/infrared/assets/",
            _ => "/ext/"
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "database_type": database_type,
                "source_path": source_path,
                "target_path": target_path,
                "message": "Database update prepared",
                "update_methods": {
                    "rpc_transfer": "Use flipper_file_write tool to upload database",
                    "qflipper": "Drag and drop via qFlipper interface",
                    "sd_card": "Copy file to SD card directly",
                    "firmware_update": "Some databases update with firmware"
                },
                "database_formats": {
                    "nfc_keys": {
                        "format": "One key per line, hex format",
                        "example": "FFFFFFFFFFFF\nA0A1A2A3A4A5",
                        "note": "MiFare Classic keys"
                    },
                    "subghz": {
                        "format": "Manufacturer code database",
                        "example": "Secure coding format",
                        "note": "KeeLoq manufacturer codes"
                    },
                    "infrared": {
                        "format": "Flipper IR format (.ir files)",
                        "structure": "Protocol, address, command data",
                        "note": "See IRDB for examples"
                    }
                },
                "safety_warnings": [
                    "⚠️  Backup existing database before updating",
                    "⚠️  Verify database format matches Flipper requirements",
                    "⚠️  Test with known working entries first",
                    "⚠️  Malformed databases may cause app crashes",
                    "⚠️  Some databases are in internal flash (careful!)"
                ],
                "recommended_sources": {
                    "nfc": "Proxmark3 MiFare dictionaries",
                    "infrared": "Flipper-IRDB GitHub repository",
                    "subghz": "Community KeeLoq databases",
                    "note": "Always verify source trustworthiness"
                }
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Protocol Import Tool ===

pub struct ProtocolImportTool;

#[async_trait]
impl PentestTool for ProtocolImportTool {
    fn name(&self) -> &str {
        "flipper_protocol_import"
    }

    fn description(&self) -> &str {
        "Import protocol definitions from external sources"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "protocol_type".to_string(),
                    param_type: ParamType::String,
                    description: "Protocol type: nfc, rfid, subghz, infrared".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "file_path".to_string(),
                    param_type: ParamType::String,
                    description: "Path to protocol file to import".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let protocol_type = params["protocol_type"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("protocol_type required".to_string()))?;

        let file_path = params["file_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("file_path required".to_string()))?;

        let valid_types = ["nfc", "rfid", "subghz", "infrared"];
        if !valid_types.contains(&protocol_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid protocol_type. Must be: {}", valid_types.join(", "))
            ));
        }

        let destination = match protocol_type {
            "nfc" => "/ext/nfc/",
            "rfid" => "/ext/lfrfid/",
            "subghz" => "/ext/subghz/",
            "infrared" => "/ext/infrared/",
            _ => "/ext/"
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "protocol_type": protocol_type,
                "file_path": file_path,
                "destination": destination,
                "message": "Protocol import prepared",
                "import_process": {
                    "step_1": "Read local protocol file",
                    "step_2": "Validate format matches Flipper specification",
                    "step_3": "Convert if necessary (e.g., Proxmark to Flipper format)",
                    "step_4": "Upload to appropriate directory",
                    "step_5": "Verify file readable on device"
                },
                "format_conversions": {
                    "proxmark_nfc": "Convert Proxmark dumps to .nfc format",
                    "pronto_ir": "Convert Pronto hex to Flipper IR format",
                    "rtl_433": "Convert RTL-433 captures to .sub format",
                    "note": "Many formats require conversion"
                },
                "validation": {
                    "format_check": "Verify file structure matches Flipper spec",
                    "protocol_check": "Ensure protocol is supported",
                    "data_integrity": "Validate checksums and data",
                    "test_load": "Try loading in relevant app after import"
                },
                "community_tools": [
                    "FlipperScripts - batch conversion tools",
                    "Proxmark3 to Flipper converters",
                    "IRDB format validators",
                    "Sub-GHz signal analyzers"
                ],
                "note": "Use flipper_file_write to transfer imported protocols to device"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Library Export Tool ===

pub struct LibraryExportTool;

#[async_trait]
impl PentestTool for LibraryExportTool {
    fn name(&self) -> &str {
        "flipper_library_export"
    }

    fn description(&self) -> &str {
        "Export protocol libraries for backup or sharing"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "library_type".to_string(),
                    param_type: ParamType::String,
                    description: "Library to export: nfc, rfid, subghz, infrared, all".to_string(),
                    required: false,
                    default: Some(json!("all")),
                },
                ToolParam {
                    name: "output_path".to_string(),
                    param_type: ParamType::String,
                    description: "Local path to save exported library".to_string(),
                    required: true,
                    default: None,
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let library_type = params["library_type"].as_str().unwrap_or("all");
        let output_path = params["output_path"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("output_path required".to_string()))?;

        let valid_types = ["nfc", "rfid", "subghz", "infrared", "all"];
        if !valid_types.contains(&library_type) {
            return Err(flipper_core::error::Error::InvalidParams(
                format!("Invalid library_type. Must be: {}", valid_types.join(", "))
            ));
        }

        let export_paths = match library_type {
            "nfc" => vec!["/ext/nfc/", "/int/assets/nfc/"],
            "rfid" => vec!["/ext/lfrfid/"],
            "subghz" => vec!["/ext/subghz/", "/int/assets/subghz/"],
            "infrared" => vec!["/ext/infrared/"],
            "all" => vec!["/ext/nfc/", "/ext/lfrfid/", "/ext/subghz/", "/ext/infrared/"],
            _ => vec!["/ext/"]
        };

        Ok(ToolResult {
            success: true,
            data: json!({
                "library_type": library_type,
                "output_path": output_path,
                "export_paths": export_paths,
                "message": "Library export prepared",
                "export_process": {
                    "step_1": "List all files in library directories",
                    "step_2": "Download each file via RPC",
                    "step_3": "Organize in local directory structure",
                    "step_4": "Create manifest/index file",
                    "step_5": "Optionally compress into archive"
                },
                "export_benefits": [
                    "Backup your captured signals",
                    "Share libraries with team members",
                    "Version control for protocol data",
                    "Transfer between multiple Flippers",
                    "Archive for compliance/documentation"
                ],
                "organization_tips": {
                    "categorize": "Organize by project or client",
                    "naming": "Use descriptive filenames",
                    "metadata": "Include capture date, location, notes",
                    "privacy": "Redact sensitive information",
                    "format": "Keep original Flipper formats"
                },
                "export_formats": {
                    "raw": "Original Flipper format (recommended)",
                    "archive": "ZIP/tar.gz for easy transport",
                    "documented": "Include README with context",
                    "converted": "Convert to other tool formats if needed"
                },
                "note": "Use batch file operations to download entire directories efficiently"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}

// === Protocol Search Tool ===

pub struct ProtocolSearchTool;

#[async_trait]
impl PentestTool for ProtocolSearchTool {
    fn name(&self) -> &str {
        "flipper_protocol_search"
    }

    fn description(&self) -> &str {
        "Search protocol databases for specific entries"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            params: vec![
                ToolParam {
                    name: "query".to_string(),
                    param_type: ParamType::String,
                    description: "Search query (device name, protocol, UID, etc.)".to_string(),
                    required: true,
                    default: None,
                },
                ToolParam {
                    name: "database".to_string(),
                    param_type: ParamType::String,
                    description: "Database to search: nfc, rfid, subghz, infrared, all".to_string(),
                    required: false,
                    default: Some(json!("all")),
                },
            ],
            supported_platforms: vec![Platform::Desktop],
        }
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> flipper_core::error::Result<ToolResult> {
        let query = params["query"]
            .as_str()
            .ok_or_else(|| flipper_core::error::Error::InvalidParams("query required".to_string()))?;

        let database = params["database"].as_str().unwrap_or("all");

        let mut client = FlipperClient::new()
            .map_err(|e| flipper_core::error::Error::ToolExecution(e.to_string()))?;

        // Search implementation would go here - for now provide instructions
        let _ = client.health_check().await;

        Ok(ToolResult {
            success: true,
            data: json!({
                "query": query,
                "database": database,
                "message": "Protocol search requires database indexing",
                "search_capabilities": {
                    "filename": "Search by file name patterns",
                    "content": "Search within protocol data (requires file reading)",
                    "uid": "Search for specific UIDs/IDs",
                    "protocol": "Filter by protocol type",
                    "date": "Search by capture date (from metadata)"
                },
                "search_paths": {
                    "nfc": "/ext/nfc/*.nfc",
                    "rfid": "/ext/lfrfid/*.rfid",
                    "subghz": "/ext/subghz/*.sub",
                    "infrared": "/ext/infrared/*.ir"
                },
                "implementation": {
                    "method_1": "List directory + filter by filename",
                    "method_2": "Download and grep file contents",
                    "method_3": "Build local index for fast searching",
                    "optimal": "Maintain local mirror with search index"
                },
                "search_examples": {
                    "by_name": "Search 'hotel' finds all hotel key captures",
                    "by_protocol": "Search 'mifare' finds MiFare cards",
                    "by_uid": "Search 'DEADBEEF' finds cards with that UID",
                    "by_frequency": "Search '433' finds 433MHz signals"
                },
                "note": "Full-text search requires downloading and indexing protocol files"
            }),
            error: None,
            duration_ms: 0,
        })
    }
}
