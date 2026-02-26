//! Desktop platform implementation

use crate::traits::*;
use async_trait::async_trait;
use hello_core::error::Result;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Desktop platform provider
pub struct DesktopPlatform;

impl DesktopPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DesktopPlatform {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SystemInfo for DesktopPlatform {
    async fn get_device_info(&self) -> Result<DeviceInfo> {
        use sysinfo::System;

        let mut sys = System::new_all();
        sys.refresh_all();

        Ok(DeviceInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_count: sys.cpus().len(),
            total_memory_mb: sys.total_memory() / 1024 / 1024,
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            cpu_brand: sys
                .cpus()
                .first()
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            used_memory_mb: sys.used_memory() / 1024 / 1024,
        })
    }
}

#[async_trait]
impl CommandExec for DesktopPlatform {
    async fn execute_command(
        &self,
        cmd: &str,
        args: &[&str],
        timeout_duration: Duration,
    ) -> Result<CommandResult> {
        let start = std::time::Instant::now();

        let child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(hello_core::error::Error::Io)?;

        match timeout(timeout_duration, child.wait_with_output()).await {
            Ok(Ok(output)) => Ok(CommandResult::success(
                String::from_utf8_lossy(&output.stdout).to_string(),
                String::from_utf8_lossy(&output.stderr).to_string(),
                output.status.code().unwrap_or(-1),
                start.elapsed().as_millis() as u64,
            )),
            Ok(Err(e)) => Err(hello_core::error::Error::Io(e)),
            Err(_) => Ok(CommandResult::timeout(
                String::new(),
                "Command timed out".to_string(),
                start.elapsed().as_millis() as u64,
            )),
        }
    }
}

impl PlatformProvider for DesktopPlatform {}
