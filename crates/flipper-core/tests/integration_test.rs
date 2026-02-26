//! Integration tests for Flipper Zero Connector

use flipper_core::connector::FlipperConnector;
use flipper_core::tools::ToolRegistry;
use flipper_tools::{
    AppInfoTool, AppListTool, DirCreateTool, FileStatTool,
    NfcReadTool, NfcWriteTool, RfidReadTool, RfidWriteTool,
    SubGhzReadTool, SubGhzWriteTool,
    BatchReadTool, FileSearchTool, NfcCloneTool, RfidGenerateTool,
    BadUsbUploadTool, BadUsbListTool, BadUsbReadTool,
    BadUsbDeleteTool, BadUsbValidateTool,
};
use serde_json::json;
use strike48_connector::BaseConnector;

#[test]
fn test_connector_creation() {
    let registry = ToolRegistry::new();
    let connector = FlipperConnector::new(registry);

    assert_eq!(connector.connector_type(), "flipper-zero");
    assert_eq!(connector.version(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_connector_metadata() {
    let registry = ToolRegistry::new();
    let connector = FlipperConnector::new(registry);

    let metadata = connector.metadata();
    assert!(metadata.contains_key("tool_schemas"));
    assert!(metadata.contains_key("tool_names"));
    assert!(metadata.contains_key("tool_count"));
}

#[test]
fn test_connector_capabilities() {
    let registry = ToolRegistry::new();
    let connector = FlipperConnector::new(registry);

    let capabilities = connector.capabilities();
    // With empty registry, should have 0 capabilities
    assert_eq!(capabilities.len(), 0);
}

#[test]
fn test_tool_registry() {
    let registry = ToolRegistry::new();

    // Initially empty
    assert_eq!(registry.tools().len(), 0);
    assert_eq!(registry.names().len(), 0);
    assert_eq!(registry.schemas().len(), 0);
}

#[tokio::test]
async fn test_connector_execute_missing_tool() {
    let registry = ToolRegistry::new();
    let connector = FlipperConnector::new(registry);

    let request = json!({
        "tool": "nonexistent_tool",
        "parameters": {}
    });

    let result = connector.execute(request, None).await;

    // Should succeed but return error in response
    assert!(result.is_ok());
    let response = result.unwrap();

    // Check if response indicates failure
    if let Some(success) = response.get("success") {
        assert_eq!(success, &json!(false));
    }
}

#[test]
fn test_connector_timeout() {
    let registry = ToolRegistry::new();
    let connector = FlipperConnector::new(registry);

    // Should have a reasonable timeout (5 minutes = 300,000 ms)
    assert_eq!(connector.timeout_ms(), 300_000);
}

// ============================================================================
// Week 2 Tool Tests
// ============================================================================

#[test]
fn test_week2_tool_registration() {
    let mut registry = ToolRegistry::new();

    // Register Week 2 tools
    registry.register(DirCreateTool);
    registry.register(FileStatTool);
    registry.register(AppListTool);
    registry.register(AppInfoTool);

    // Verify tools are registered
    assert_eq!(registry.tools().len(), 4);

    let names = registry.names();
    assert!(names.contains(&"flipper_dir_create"));
    assert!(names.contains(&"flipper_file_stat"));
    assert!(names.contains(&"flipper_app_list"));
    assert!(names.contains(&"flipper_app_info"));
}

#[test]
fn test_week2_tool_schemas() {
    let mut registry = ToolRegistry::new();

    registry.register(DirCreateTool);
    registry.register(FileStatTool);
    registry.register(AppListTool);
    registry.register(AppInfoTool);

    let schemas = registry.schemas();
    assert_eq!(schemas.len(), 4);

    // Check DirCreateTool schema
    let dir_create = schemas.iter().find(|s| s.name == "flipper_dir_create");
    assert!(dir_create.is_some());
    let dir_create = dir_create.unwrap();
    assert_eq!(dir_create.description, "Create a directory on the Flipper Zero");
    assert_eq!(dir_create.params.len(), 1);
    assert_eq!(dir_create.params[0].name, "path");
    assert!(dir_create.params[0].required);

    // Check FileStatTool schema
    let file_stat = schemas.iter().find(|s| s.name == "flipper_file_stat");
    assert!(file_stat.is_some());
    let file_stat = file_stat.unwrap();
    assert_eq!(file_stat.description, "Get file or directory metadata from the Flipper Zero");
    assert_eq!(file_stat.params.len(), 1);
    assert_eq!(file_stat.params[0].name, "path");

    // Check AppListTool schema
    let app_list = schemas.iter().find(|s| s.name == "flipper_app_list");
    assert!(app_list.is_some());
    let app_list = app_list.unwrap();
    assert_eq!(app_list.description, "List installed applications on the Flipper Zero");
    assert_eq!(app_list.params.len(), 1);
    assert_eq!(app_list.params[0].name, "category");
    assert!(!app_list.params[0].required); // category is optional

    // Check AppInfoTool schema
    let app_info = schemas.iter().find(|s| s.name == "flipper_app_info");
    assert!(app_info.is_some());
    let app_info = app_info.unwrap();
    assert_eq!(app_info.description, "Get information about a specific Flipper Zero app");
    assert_eq!(app_info.params.len(), 1);
    assert_eq!(app_info.params[0].name, "path");
    assert!(app_info.params[0].required);
}

#[test]
fn test_full_registry_with_all_tools() {
    use flipper_tools::create_tool_registry;

    let registry = create_tool_registry();

    // Should have all 24 tools registered
    // Week 1: device_info, file_list, file_read, file_write, file_delete (5)
    // Week 2: dir_create, file_stat, app_list, app_info (4)
    // Week 3: nfc_read, nfc_write, rfid_read, rfid_write, subghz_read, subghz_write (6)
    // Week 4: batch_read, file_search, nfc_clone, rfid_generate (4)
    // BadUSB: badusb_upload, badusb_list, badusb_read, badusb_delete, badusb_validate (5)
    assert_eq!(registry.tools().len(), 24);

    let names = registry.names();

    // Week 1 tools
    assert!(names.contains(&"flipper_device_info"));
    assert!(names.contains(&"flipper_file_list"));
    assert!(names.contains(&"flipper_file_read"));
    assert!(names.contains(&"flipper_file_write"));
    assert!(names.contains(&"flipper_file_delete"));

    // Week 2 tools
    assert!(names.contains(&"flipper_dir_create"));
    assert!(names.contains(&"flipper_file_stat"));
    assert!(names.contains(&"flipper_app_list"));
    assert!(names.contains(&"flipper_app_info"));
}

#[test]
fn test_connector_with_week2_tools() {
    use flipper_tools::create_tool_registry;

    let registry = create_tool_registry();
    let connector = FlipperConnector::new(registry);

    // Verify connector has all capabilities (now 24 with BadUSB)
    let capabilities = connector.capabilities();
    assert_eq!(capabilities.len(), 24);

    // Verify metadata includes all tools
    let metadata = connector.metadata();
    let tool_names_str = metadata.get("tool_names").unwrap();
    let tool_names: Vec<&str> = tool_names_str.split(',').collect();
    assert_eq!(tool_names.len(), 24);

    let tool_count = metadata.get("tool_count").unwrap();
    assert_eq!(tool_count, "24");
}

// ============================================================================
// Week 3 Tool Tests
// ============================================================================

#[test]
fn test_week3_tool_registration() {
    let mut registry = ToolRegistry::new();

    // Register Week 3 tools
    registry.register(NfcReadTool);
    registry.register(NfcWriteTool);
    registry.register(RfidReadTool);
    registry.register(RfidWriteTool);
    registry.register(SubGhzReadTool);
    registry.register(SubGhzWriteTool);

    // Verify tools are registered
    assert_eq!(registry.tools().len(), 6);

    let names = registry.names();
    assert!(names.contains(&"flipper_nfc_read"));
    assert!(names.contains(&"flipper_nfc_write"));
    assert!(names.contains(&"flipper_rfid_read"));
    assert!(names.contains(&"flipper_rfid_write"));
    assert!(names.contains(&"flipper_subghz_read"));
    assert!(names.contains(&"flipper_subghz_write"));
}

#[test]
fn test_week3_nfc_tool_schemas() {
    let mut registry = ToolRegistry::new();
    registry.register(NfcReadTool);
    registry.register(NfcWriteTool);

    let schemas = registry.schemas();
    assert_eq!(schemas.len(), 2);

    // Check NfcReadTool
    let nfc_read = schemas.iter().find(|s| s.name == "flipper_nfc_read");
    assert!(nfc_read.is_some());
    let nfc_read = nfc_read.unwrap();
    assert_eq!(nfc_read.description, "Read and parse NFC file from the Flipper Zero");
    assert_eq!(nfc_read.params.len(), 1);
    assert_eq!(nfc_read.params[0].name, "path");
    assert!(nfc_read.params[0].required);

    // Check NfcWriteTool
    let nfc_write = schemas.iter().find(|s| s.name == "flipper_nfc_write");
    assert!(nfc_write.is_some());
    let nfc_write = nfc_write.unwrap();
    assert_eq!(nfc_write.description, "Create an NFC file on the Flipper Zero");
    assert!(nfc_write.params.len() >= 3); // path, device_type, uid at minimum
}

#[test]
fn test_week3_rfid_tool_schemas() {
    let mut registry = ToolRegistry::new();
    registry.register(RfidReadTool);
    registry.register(RfidWriteTool);

    let schemas = registry.schemas();
    assert_eq!(schemas.len(), 2);

    // Check RfidReadTool
    let rfid_read = schemas.iter().find(|s| s.name == "flipper_rfid_read");
    assert!(rfid_read.is_some());
    let rfid_read = rfid_read.unwrap();
    assert_eq!(rfid_read.description, "Read and parse RFID file from the Flipper Zero");

    // Check RfidWriteTool
    let rfid_write = schemas.iter().find(|s| s.name == "flipper_rfid_write");
    assert!(rfid_write.is_some());
    let rfid_write = rfid_write.unwrap();
    assert_eq!(rfid_write.description, "Create an RFID file on the Flipper Zero");
    assert!(rfid_write.params.len() >= 2); // path, key_type at minimum
}

#[test]
fn test_week3_subghz_tool_schemas() {
    let mut registry = ToolRegistry::new();
    registry.register(SubGhzReadTool);
    registry.register(SubGhzWriteTool);

    let schemas = registry.schemas();
    assert_eq!(schemas.len(), 2);

    // Check SubGhzReadTool
    let subghz_read = schemas.iter().find(|s| s.name == "flipper_subghz_read");
    assert!(subghz_read.is_some());
    let subghz_read = subghz_read.unwrap();
    assert_eq!(subghz_read.description, "Read and parse Sub-GHz file from the Flipper Zero");

    // Check SubGhzWriteTool
    let subghz_write = schemas.iter().find(|s| s.name == "flipper_subghz_write");
    assert!(subghz_write.is_some());
    let subghz_write = subghz_write.unwrap();
    assert_eq!(subghz_write.description, "Create a Sub-GHz file on the Flipper Zero");
    assert!(subghz_write.params.len() >= 4); // path, frequency, protocol, key at minimum
}

#[test]
fn test_full_registry_weeks_1_2_3() {
    use flipper_tools::create_tool_registry;

    let registry = create_tool_registry();

    // Should have all 24 tools registered
    // Week 1: device_info, file_list, file_read, file_write, file_delete (5)
    // Week 2: dir_create, file_stat, app_list, app_info (4)
    // Week 3: nfc_read, nfc_write, rfid_read, rfid_write, subghz_read, subghz_write (6)
    // Week 4: batch_read, file_search, nfc_clone, rfid_generate (4)
    // BadUSB: badusb_upload, badusb_list, badusb_read, badusb_delete, badusb_validate (5)
    assert_eq!(registry.tools().len(), 24);

    let names = registry.names();

    // Week 1 tools
    assert!(names.contains(&"flipper_device_info"));
    assert!(names.contains(&"flipper_file_list"));
    assert!(names.contains(&"flipper_file_read"));
    assert!(names.contains(&"flipper_file_write"));
    assert!(names.contains(&"flipper_file_delete"));

    // Week 2 tools
    assert!(names.contains(&"flipper_dir_create"));
    assert!(names.contains(&"flipper_file_stat"));
    assert!(names.contains(&"flipper_app_list"));
    assert!(names.contains(&"flipper_app_info"));

    // Week 3 tools
    assert!(names.contains(&"flipper_nfc_read"));
    assert!(names.contains(&"flipper_nfc_write"));
    assert!(names.contains(&"flipper_rfid_read"));
    assert!(names.contains(&"flipper_rfid_write"));
    assert!(names.contains(&"flipper_subghz_read"));
    assert!(names.contains(&"flipper_subghz_write"));
}

#[test]
fn test_connector_with_all_weeks() {
    use flipper_tools::create_tool_registry;

    let registry = create_tool_registry();
    let connector = FlipperConnector::new(registry);

    // Verify connector has all capabilities
    let capabilities = connector.capabilities();
    assert_eq!(capabilities.len(), 24);

    // Verify metadata
    let metadata = connector.metadata();
    let tool_count = metadata.get("tool_count").unwrap();
    assert_eq!(tool_count, "24");
}

// ============================================================================
// BadUSB Tool Tests
// ============================================================================

#[test]
fn test_badusb_tool_registration() {
    let mut registry = ToolRegistry::new();

    // Register BadUSB tools
    registry.register(BadUsbUploadTool);
    registry.register(BadUsbListTool);
    registry.register(BadUsbReadTool);
    registry.register(BadUsbDeleteTool);
    registry.register(BadUsbValidateTool);

    // Verify tools are registered
    assert_eq!(registry.tools().len(), 5);

    let names = registry.names();
    assert!(names.contains(&"flipper_badusb_upload"));
    assert!(names.contains(&"flipper_badusb_list"));
    assert!(names.contains(&"flipper_badusb_read"));
    assert!(names.contains(&"flipper_badusb_delete"));
    assert!(names.contains(&"flipper_badusb_validate"));
}

#[test]
fn test_badusb_tool_schemas() {
    let mut registry = ToolRegistry::new();

    registry.register(BadUsbUploadTool);
    registry.register(BadUsbListTool);
    registry.register(BadUsbReadTool);
    registry.register(BadUsbDeleteTool);
    registry.register(BadUsbValidateTool);

    let schemas = registry.schemas();
    assert_eq!(schemas.len(), 5);

    // Check BadUsbUploadTool schema
    let upload = schemas.iter().find(|s| s.name == "flipper_badusb_upload");
    assert!(upload.is_some());
    let upload = upload.unwrap();
    assert_eq!(upload.description, "Upload a BadUSB Ducky Script to the Flipper Zero");
    assert_eq!(upload.params.len(), 3);
    assert_eq!(upload.params[0].name, "filename");
    assert!(upload.params[0].required);
    assert_eq!(upload.params[1].name, "script");
    assert!(upload.params[1].required);

    // Check BadUsbListTool schema
    let list = schemas.iter().find(|s| s.name == "flipper_badusb_list");
    assert!(list.is_some());
    let list = list.unwrap();
    assert_eq!(list.description, "List all BadUSB scripts on the Flipper Zero");
    assert_eq!(list.params.len(), 0);

    // Check BadUsbReadTool schema
    let read = schemas.iter().find(|s| s.name == "flipper_badusb_read");
    assert!(read.is_some());
    let read = read.unwrap();
    assert_eq!(read.description, "Read and parse a BadUSB script from the Flipper Zero");
    assert_eq!(read.params.len(), 1);
    assert_eq!(read.params[0].name, "filename");

    // Check BadUsbDeleteTool schema
    let delete = schemas.iter().find(|s| s.name == "flipper_badusb_delete");
    assert!(delete.is_some());
    let delete = delete.unwrap();
    assert_eq!(delete.description, "Delete a BadUSB script from the Flipper Zero");

    // Check BadUsbValidateTool schema
    let validate = schemas.iter().find(|s| s.name == "flipper_badusb_validate");
    assert!(validate.is_some());
    let validate = validate.unwrap();
    assert_eq!(validate.description, "Validate Ducky Script syntax without uploading");
    assert_eq!(validate.params.len(), 1);
    assert_eq!(validate.params[0].name, "script");
}

#[test]
fn test_full_registry_with_badusb() {
    use flipper_tools::create_tool_registry;

    let registry = create_tool_registry();

    // Should have all 24 tools registered
    // Week 1: device_info, file_list, file_read, file_write, file_delete (5)
    // Week 2: dir_create, file_stat, app_list, app_info (4)
    // Week 3: nfc_read, nfc_write, rfid_read, rfid_write, subghz_read, subghz_write (6)
    // Week 4: batch_read, file_search, nfc_clone, rfid_generate (4)
    // BadUSB: badusb_upload, badusb_list, badusb_read, badusb_delete, badusb_validate (5)
    assert_eq!(registry.tools().len(), 24);

    let names = registry.names();

    // Week 1 tools
    assert!(names.contains(&"flipper_device_info"));
    assert!(names.contains(&"flipper_file_list"));
    assert!(names.contains(&"flipper_file_read"));
    assert!(names.contains(&"flipper_file_write"));
    assert!(names.contains(&"flipper_file_delete"));

    // Week 2 tools
    assert!(names.contains(&"flipper_dir_create"));
    assert!(names.contains(&"flipper_file_stat"));
    assert!(names.contains(&"flipper_app_list"));
    assert!(names.contains(&"flipper_app_info"));

    // Week 3 tools
    assert!(names.contains(&"flipper_nfc_read"));
    assert!(names.contains(&"flipper_nfc_write"));
    assert!(names.contains(&"flipper_rfid_read"));
    assert!(names.contains(&"flipper_rfid_write"));
    assert!(names.contains(&"flipper_subghz_read"));
    assert!(names.contains(&"flipper_subghz_write"));

    // Week 4 tools
    assert!(names.contains(&"flipper_batch_read"));
    assert!(names.contains(&"flipper_file_search"));
    assert!(names.contains(&"flipper_nfc_clone"));
    assert!(names.contains(&"flipper_rfid_generate"));

    // BadUSB tools
    assert!(names.contains(&"flipper_badusb_upload"));
    assert!(names.contains(&"flipper_badusb_list"));
    assert!(names.contains(&"flipper_badusb_read"));
    assert!(names.contains(&"flipper_badusb_delete"));
    assert!(names.contains(&"flipper_badusb_validate"));
}

#[test]
fn test_connector_with_badusb_tools() {
    use flipper_tools::create_tool_registry;

    let registry = create_tool_registry();
    let connector = FlipperConnector::new(registry);

    // Verify connector has all capabilities (now 24 with BadUSB)
    let capabilities = connector.capabilities();
    assert_eq!(capabilities.len(), 24);

    // Verify metadata includes all tools
    let metadata = connector.metadata();
    let tool_names_str = metadata.get("tool_names").unwrap();
    let tool_names: Vec<&str> = tool_names_str.split(',').collect();
    assert_eq!(tool_names.len(), 24);

    let tool_count = metadata.get("tool_count").unwrap();
    assert_eq!(tool_count, "24");
}
