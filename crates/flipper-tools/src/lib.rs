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
pub mod ibutton_operations;
pub mod infrared_operations;
pub mod gpio_operations;
pub mod bluetooth_operations;
pub mod u2f_operations;
pub mod zigbee_operations;
pub mod ble_advanced_operations;
pub mod firmware_operations;
pub mod storage_operations;
pub mod power_operations;
pub mod system_operations;
pub mod display_operations;
pub mod audio_operations;
pub mod network_operations;
pub mod crypto_operations;
pub mod protocol_db_operations;
pub mod script_operations;
pub mod audit_operations;

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
pub use ibutton_operations::{IButtonReadTool, IButtonWriteTool, IButtonEmulateTool};
pub use infrared_operations::{InfraredReadTool, InfraredWriteTool, InfraredSendTool};
pub use gpio_operations::{GpioSetTool, GpioReadTool, UartTool, I2cScanTool, SpiExchangeTool};
pub use bluetooth_operations::{BleScanTool, BleDeviceInfoTool, BleEnumerateTool, BleSecurityTestTool};
pub use u2f_operations::{U2fRegisterTool, U2fAuthenticateTool, Fido2RegisterTool, Fido2AuthenticateTool};
pub use zigbee_operations::{ZigbeeScanTool, ZigbeeJoinTool, ZigbeeSniffTool, ZigbeeDeviceInfoTool};
pub use ble_advanced_operations::{BleMitmTool, BleCrackPinTool, BleReplayTool};
pub use firmware_operations::{FirmwareInfoTool, FirmwareBackupTool, FirmwareUpdateTool, FirmwareVerifyTool};
pub use storage_operations::{StorageInfoTool, StorageFormatTool, StorageBenchmarkTool, BackupCreateTool, ArchiveTool};
pub use power_operations::{BatteryInfoTool, PowerModeTool, ChargingStatusTool, PowerOptimizeTool};
pub use system_operations::{SystemRebootTool, DateTimeSyncTool, LedControlTool, VibrationControlTool, SystemDiagnosticsTool};
pub use display_operations::{ScreenshotTool, CanvasDrawTool, DisplayInfoTool, BacklightControlTool, ScreenTestTool};
pub use audio_operations::{SpeakerControlTool, ToneGeneratorTool, MusicPlayerTool, AudioAlertTool, VolumeControlTool};
pub use network_operations::{WiFiInfoTool, HttpRequestTool, NetworkScanTool, PingTool, DnsLookupTool};
pub use crypto_operations::{HashGenerateTool, KeyGenerateTool, EncryptTool, DecryptTool, RandomDataTool, ChecksumTool};
pub use protocol_db_operations::{DatabaseInfoTool, DatabaseUpdateTool, ProtocolImportTool, LibraryExportTool, ProtocolSearchTool};
pub use script_operations::{ScriptTemplateTool, ScriptValidatorTool, BatchExecuteTool, WorkflowAutomationTool, TaskSchedulerTool};
pub use audit_operations::SecurityAuditTool;

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

    // iButton operations
    registry.register(IButtonReadTool);
    registry.register(IButtonWriteTool);
    registry.register(IButtonEmulateTool);

    // Infrared operations
    registry.register(InfraredReadTool);
    registry.register(InfraredWriteTool);
    registry.register(InfraredSendTool);

    // GPIO operations
    registry.register(GpioSetTool);
    registry.register(GpioReadTool);
    registry.register(UartTool);
    registry.register(I2cScanTool);
    registry.register(SpiExchangeTool);

    // Bluetooth LE operations
    registry.register(BleScanTool);
    registry.register(BleDeviceInfoTool);
    registry.register(BleEnumerateTool);
    registry.register(BleSecurityTestTool);

    // U2F/FIDO2 operations
    registry.register(U2fRegisterTool);
    registry.register(U2fAuthenticateTool);
    registry.register(Fido2RegisterTool);
    registry.register(Fido2AuthenticateTool);

    // Zigbee operations
    registry.register(ZigbeeScanTool);
    registry.register(ZigbeeJoinTool);
    registry.register(ZigbeeSniffTool);
    registry.register(ZigbeeDeviceInfoTool);

    // Advanced BLE attacks
    registry.register(BleMitmTool);
    registry.register(BleCrackPinTool);
    registry.register(BleReplayTool);

    // Firmware management
    registry.register(FirmwareInfoTool);
    registry.register(FirmwareBackupTool);
    registry.register(FirmwareUpdateTool);
    registry.register(FirmwareVerifyTool);

    // Storage operations
    registry.register(StorageInfoTool);
    registry.register(StorageFormatTool);
    registry.register(StorageBenchmarkTool);
    registry.register(BackupCreateTool);
    registry.register(ArchiveTool);

    // Power management
    registry.register(BatteryInfoTool);
    registry.register(PowerModeTool);
    registry.register(ChargingStatusTool);
    registry.register(PowerOptimizeTool);

    // System utilities
    registry.register(SystemRebootTool);
    registry.register(DateTimeSyncTool);
    registry.register(LedControlTool);
    registry.register(VibrationControlTool);
    registry.register(SystemDiagnosticsTool);

    // Display operations
    registry.register(ScreenshotTool);
    registry.register(CanvasDrawTool);
    registry.register(DisplayInfoTool);
    registry.register(BacklightControlTool);
    registry.register(ScreenTestTool);

    // Audio operations
    registry.register(SpeakerControlTool);
    registry.register(ToneGeneratorTool);
    registry.register(MusicPlayerTool);
    registry.register(AudioAlertTool);
    registry.register(VolumeControlTool);

    // Network operations
    registry.register(WiFiInfoTool);
    registry.register(HttpRequestTool);
    registry.register(NetworkScanTool);
    registry.register(PingTool);
    registry.register(DnsLookupTool);

    // Cryptography operations
    registry.register(HashGenerateTool);
    registry.register(KeyGenerateTool);
    registry.register(EncryptTool);
    registry.register(DecryptTool);
    registry.register(RandomDataTool);
    registry.register(ChecksumTool);

    // Protocol database management
    registry.register(DatabaseInfoTool);
    registry.register(DatabaseUpdateTool);
    registry.register(ProtocolImportTool);
    registry.register(LibraryExportTool);
    registry.register(ProtocolSearchTool);

    // Script & automation
    registry.register(ScriptTemplateTool);
    registry.register(ScriptValidatorTool);
    registry.register(BatchExecuteTool);
    registry.register(WorkflowAutomationTool);
    registry.register(TaskSchedulerTool);

    // Security audit and reporting (Tool #100!)
    registry.register(SecurityAuditTool);

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
