//! Flipper Zero client implementation

use crate::error::{FlipperError, Result};
use flipper_rpc::fs::{FsCreateDir, FsMetadata, FsRead, FsReadDir, FsRemove, FsWrite};
use flipper_rpc::rpc::req::Request;
use flipper_rpc::transport::serial::{list_flipper_ports, rpc::SerialRpcTransport};
use flipper_rpc::transport::Transport;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Reconnecting,
}

/// High-level client for communicating with Flipper Zero
pub struct FlipperClient {
    transport: SerialRpcTransport,
    port: String,
    state: ConnectionState,
    last_health_check: Option<Instant>,
    health_check_interval: Duration,
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
}

impl FlipperClient {
    /// Default health check interval (30 seconds)
    pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);

    /// Default max reconnection attempts
    pub const DEFAULT_MAX_RECONNECT_ATTEMPTS: u32 = 3;

    /// Create a new client by auto-detecting the Flipper Zero device
    pub fn new() -> Result<Self> {
        info!("Auto-detecting Flipper Zero device...");

        let ports = list_flipper_ports()
            .map_err(|e| FlipperError::DeviceNotConnected(format!("Failed to list ports: {}", e)))?;

        if ports.is_empty() {
            return Err(FlipperError::DeviceNotConnected(
                "No Flipper Zero devices found".to_string(),
            ));
        }

        let port = ports[0].port_name.clone();
        info!("Found Flipper Zero at: {}", port);

        Self::connect(&port)
    }

    /// Create a new client by connecting to a specific port
    pub fn connect(port: &str) -> Result<Self> {
        debug!("Connecting to Flipper Zero at: {}", port);

        let transport = SerialRpcTransport::new(port)
            .map_err(|e| FlipperError::DeviceNotConnected(format!("Failed to connect: {}", e)))?;

        info!("Connected to Flipper Zero successfully");

        Ok(Self {
            transport,
            port: port.to_string(),
            state: ConnectionState::Connected,
            last_health_check: Some(Instant::now()),
            health_check_interval: Self::DEFAULT_HEALTH_CHECK_INTERVAL,
            reconnect_attempts: 0,
            max_reconnect_attempts: Self::DEFAULT_MAX_RECONNECT_ATTEMPTS,
        })
    }

    /// Get the port this client is connected to
    pub fn port(&self) -> &str {
        &self.port
    }

    /// Get the current connection state
    pub fn state(&self) -> ConnectionState {
        self.state
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.state == ConnectionState::Connected
    }

    /// Perform a health check by pinging the device
    pub async fn health_check(&mut self) -> Result<bool> {
        if !self.is_connected() {
            return Ok(false);
        }

        debug!("Performing health check...");

        // Send a ping
        match self.transport.send_and_receive(Request::Ping(vec![1, 2, 3, 4])) {
            Ok(_response) => {
                debug!("Health check passed");
                self.last_health_check = Some(Instant::now());
                Ok(true)
            }
            Err(e) => {
                warn!("Health check failed: {}", e);
                self.state = ConnectionState::Disconnected;
                Ok(false)
            }
        }
    }

    /// Ensure the connection is healthy (with auto-reconnect)
    pub async fn ensure_connected(&mut self) -> Result<()> {
        // Check if we need a health check
        if let Some(last_check) = self.last_health_check {
            if last_check.elapsed() < self.health_check_interval {
                // Recent health check, assume connected
                if self.is_connected() {
                    return Ok(());
                }
            }
        }

        // Perform health check
        match self.health_check().await {
            Ok(true) => Ok(()),
            Ok(false) => {
                // Health check failed, try to reconnect
                self.reconnect().await
            }
            Err(e) => Err(e),
        }
    }

    /// Attempt to reconnect to the device
    pub async fn reconnect(&mut self) -> Result<()> {
        if self.reconnect_attempts >= self.max_reconnect_attempts {
            return Err(FlipperError::DeviceDisconnected);
        }

        self.state = ConnectionState::Reconnecting;
        self.reconnect_attempts += 1;

        info!(
            "Attempting to reconnect ({}/{})",
            self.reconnect_attempts, self.max_reconnect_attempts
        );

        // Small delay before reconnection
        tokio::time::sleep(Duration::from_millis(500)).await;

        match SerialRpcTransport::new(&self.port) {
            Ok(transport) => {
                self.transport = transport;
                self.state = ConnectionState::Connected;
                self.last_health_check = Some(Instant::now());
                self.reconnect_attempts = 0;
                info!("Reconnected successfully");
                Ok(())
            }
            Err(e) => {
                warn!("Reconnection failed: {}", e);
                Err(FlipperError::DeviceDisconnected)
            }
        }
    }

    /// Get a reference to the underlying transport
    pub fn transport(&mut self) -> &mut SerialRpcTransport {
        &mut self.transport
    }

    // === Filesystem Operations ===

    /// Read a file from the Flipper Zero
    pub async fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        self.ensure_connected().await?;
        let data = self.transport
            .fs_read(path)
            .map_err(FlipperError::from)?;
        Ok(data.to_vec())
    }

    /// Write a file to the Flipper Zero
    pub async fn write_file(&mut self, path: &str, data: Vec<u8>) -> Result<()> {
        self.ensure_connected().await?;
        self.transport
            .fs_write(path, data)
            .map_err(FlipperError::from)
    }

    /// Delete a file or directory
    pub async fn delete_path(&mut self, path: &str, recursive: bool) -> Result<()> {
        self.ensure_connected().await?;
        self.transport
            .fs_remove(path, recursive)
            .map_err(FlipperError::from)
    }

    /// List directory contents
    /// Returns a list of items (files and directories) in the specified path
    pub async fn list_directory(
        &mut self,
        path: &str,
        include_md5: bool,
    ) -> Result<Vec<flipper_rpc::rpc::res::ReadDirItem>> {
        self.ensure_connected().await?;
        self.transport
            .fs_read_dir(path, include_md5)
            .map(|iter| iter.collect())
            .map_err(FlipperError::from)
    }

    /// Create a directory on the Flipper Zero
    /// Returns true if the directory was created, false if it already exists
    pub async fn create_directory(&mut self, path: &str) -> Result<bool> {
        self.ensure_connected().await?;
        self.transport
            .fs_create_dir(path)
            .map_err(FlipperError::from)
    }

    /// Get file/directory metadata (size)
    /// Returns the size in bytes for files, or 0 for directories
    pub async fn get_metadata(&mut self, path: &str) -> Result<u32> {
        self.ensure_connected().await?;
        self.transport
            .fs_metadata(path)
            .map_err(FlipperError::from)
    }

    // === App Operations ===

    /// Start an application on the Flipper Zero
    pub async fn start_app(&mut self, app_name: &str) -> Result<()> {
        self.ensure_connected().await?;

        use flipper_rpc::proto::app::StartRequest;

        self.transport
            .send_and_receive(Request::AppStart(StartRequest {
                name: app_name.to_string(),
                args: String::new(),
            }))
            .map_err(FlipperError::from)?;

        Ok(())
    }

    /// Exit the current application
    pub async fn exit_app(&mut self) -> Result<()> {
        self.ensure_connected().await?;

        use flipper_rpc::proto::app::AppExitRequest;

        self.transport
            .send_and_receive(Request::AppExit(AppExitRequest {}))
            .map_err(FlipperError::from)?;

        Ok(())
    }
}
