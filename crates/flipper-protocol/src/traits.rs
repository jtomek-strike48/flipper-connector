//! Platform trait definitions

use async_trait::async_trait;
use hello_core::error::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Combined platform provider trait
#[async_trait]
pub trait PlatformProvider: SystemInfo + CommandExec + Send + Sync {}

/// System information trait
#[async_trait]
pub trait SystemInfo: Send + Sync {
    /// Get device/system information
    async fn get_device_info(&self) -> Result<DeviceInfo>;
}

/// Command execution trait
#[async_trait]
pub trait CommandExec: Send + Sync {
    /// Execute a command
    async fn execute_command(
        &self,
        cmd: &str,
        args: &[&str],
        timeout: Duration,
    ) -> Result<CommandResult>;
}

/// Device/system information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub architecture: String,
    pub cpu_count: usize,
    pub total_memory_mb: u64,
    pub kernel_version: String,
    pub cpu_brand: String,
    pub used_memory_mb: u64,
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub timed_out: bool,
    pub duration_ms: u64,
}

impl CommandResult {
    /// Create a successful result
    pub fn success(stdout: String, stderr: String, exit_code: i32, duration_ms: u64) -> Self {
        Self {
            stdout,
            stderr,
            exit_code,
            timed_out: false,
            duration_ms,
        }
    }

    /// Create a timeout result
    pub fn timeout(stdout: String, stderr: String, duration_ms: u64) -> Self {
        Self {
            stdout,
            stderr,
            exit_code: -1,
            timed_out: true,
            duration_ms,
        }
    }
}
