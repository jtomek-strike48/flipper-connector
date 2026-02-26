//! Flipper Zero Connector Core Library

pub mod connector;
pub mod error;
pub mod logging;
pub mod tools;

pub mod prelude {
    pub use crate::connector::FlipperConnector;
    pub use crate::error::{Error, Result};
    pub use crate::tools::{PentestTool, ToolRegistry, ToolResult, ToolSchema};
}
