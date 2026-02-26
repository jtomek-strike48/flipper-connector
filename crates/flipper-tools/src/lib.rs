//! Flipper Zero Tool Implementations

pub mod device_info;
pub mod file_operations;
pub mod dir_operations;
pub mod app_management;
pub mod nfc_operations;
pub mod rfid_operations;
pub mod subghz_operations;
pub mod batch_operations;
pub mod search_operations;
pub mod clone_operations;
pub mod badusb_operations;

use flipper_core::tools::ToolRegistry;

pub use device_info::DeviceInfoTool;
pub use file_operations::{FileDeleteTool, FileListTool, FileReadTool, FileWriteTool};
pub use dir_operations::{DirCreateTool, FileStatTool};
pub use app_management::{AppListTool, AppInfoTool};
pub use nfc_operations::{NfcReadTool, NfcWriteTool};
pub use rfid_operations::{RfidReadTool, RfidWriteTool};
pub use subghz_operations::{SubGhzReadTool, SubGhzWriteTool};
pub use batch_operations::BatchReadTool;
pub use search_operations::FileSearchTool;
pub use clone_operations::{NfcCloneTool, RfidGenerateTool};
pub use badusb_operations::{
    BadUsbUploadTool, BadUsbListTool, BadUsbReadTool,
    BadUsbDeleteTool, BadUsbValidateTool
};

/// Create a tool registry with all available tools
pub fn create_tool_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    // Device tools
    registry.register(DeviceInfoTool);

    // File operations
    registry.register(FileListTool);
    registry.register(FileReadTool);
    registry.register(FileWriteTool);
    registry.register(FileDeleteTool);

    // Directory operations
    registry.register(DirCreateTool);
    registry.register(FileStatTool);

    // App management
    registry.register(AppListTool);
    registry.register(AppInfoTool);

    // NFC operations
    registry.register(NfcReadTool);
    registry.register(NfcWriteTool);

    // RFID operations
    registry.register(RfidReadTool);
    registry.register(RfidWriteTool);

    // Sub-GHz operations
    registry.register(SubGhzReadTool);
    registry.register(SubGhzWriteTool);

    // Batch operations
    registry.register(BatchReadTool);

    // Search operations
    registry.register(FileSearchTool);

    // Clone operations
    registry.register(NfcCloneTool);
    registry.register(RfidGenerateTool);

    // BadUSB operations
    registry.register(BadUsbUploadTool);
    registry.register(BadUsbListTool);
    registry.register(BadUsbReadTool);
    registry.register(BadUsbDeleteTool);
    registry.register(BadUsbValidateTool);

    registry
}

/// Get all tool names
pub fn tool_names() -> Vec<String> {
    create_tool_registry()
        .names()
        .into_iter()
        .map(String::from)
        .collect()
}
