//! Error types for Flipper protocol operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, FlipperError>;

#[derive(Debug, Error)]
pub enum FlipperError {
    #[error("Device not connected: {0}")]
    DeviceNotConnected(String),

    #[error("Device disconnected during operation")]
    DeviceDisconnected,

    #[error("Connection timeout after {timeout_sec}s")]
    ConnectionTimeout { timeout_sec: u64 },

    #[error("Operation timeout after {timeout_sec}s: {context}")]
    OperationTimeout { timeout_sec: u64, context: String },

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Operation failed: {reason}")]
    OperationFailed { reason: String },

    #[error("Device busy: {current_operation} in progress")]
    DeviceBusy { current_operation: String },

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("flipper-rpc error: {0}")]
    RpcError(#[from] flipper_rpc::error::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}
