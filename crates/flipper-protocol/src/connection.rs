//! Connection management for Flipper Zero devices

use crate::error::{FlipperError, Result};
use flipper_rpc::transport::serial::list_flipper_ports;
use tracing::{debug, info};

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub port: String,
    pub description: Option<String>,
}

/// List all connected Flipper Zero devices
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    debug!("Scanning for Flipper Zero devices...");

    let ports = list_flipper_ports()
        .map_err(|e| FlipperError::DeviceNotConnected(format!("Failed to list ports: {}", e)))?;

    let devices: Vec<DeviceInfo> = ports
        .into_iter()
        .map(|p| DeviceInfo {
            port: p.port_name,
            description: None,
        })
        .collect();

    info!("Found {} Flipper Zero device(s)", devices.len());

    Ok(devices)
}

/// Auto-detect the first available Flipper Zero device
pub fn auto_detect_device() -> Result<DeviceInfo> {
    let devices = list_devices()?;

    devices
        .into_iter()
        .next()
        .ok_or_else(|| FlipperError::DeviceNotConnected("No devices found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_devices() {
        // This test requires actual hardware
        // In production, we'd mock this
        let result = list_devices();
        println!("Devices: {:?}", result);
    }
}
