//! Flipper Zero Protocol Layer
//!
//! This crate wraps `flipper-rpc` and provides high-level abstractions
//! for communicating with Flipper Zero devices.

pub mod client;
pub mod connection;
pub mod error;

pub use client::{ConnectionState, FlipperClient};
pub use connection::{auto_detect_device, list_devices, DeviceInfo};
pub use error::{FlipperError, Result};

/// Re-export flipper-rpc types for convenience
pub use flipper_rpc;
