//! Shared logging initialization.

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialise a console-only tracing subscriber.
///
/// `default_level` is the tracing level applied to the `flipper` target
/// (e.g. `"info"`, `"debug"`). The `RUST_LOG` env var can still override.
pub fn init_logging(default_level: &str) {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::from_default_env()
                .add_directive(format!("flipper={default_level}").parse().unwrap())
                .add_directive(format!("flipper_agent={default_level}").parse().unwrap()),
        )
        .init();
}
