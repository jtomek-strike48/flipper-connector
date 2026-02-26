# Hello World Connector

Minimal Strike48 connector built with the same architecture as
`dioxus-connector`, stripped to essentials for testing and learning.

## Architecture

Rust workspace with four crates:

- **hello-core** — Error types, `PentestTool` trait, `ToolRegistry`,
  `HelloWorldConnector` (implements `BaseConnector` from the SDK),
  logging init.
- **hello-platform** — `SystemInfo` and `CommandExec` traits with a
  desktop implementation backed by `sysinfo` and `tokio::process`.
- **hello-tools** — Two concrete tools: `hello_world` (greeting) and
  `device_info` (system info via platform).
- **hello-headless** — Binary (`hello-agent`) that creates the
  connector, runs both tools, and prints results.

## SDK Dependency

Uses a **path dependency** to the local SDK checkout:

```
strike48-connector = { path = "../../sdk-rs/crates/connector" }
```

## Build Commands

```bash
just check          # cargo check --workspace
just lint           # cargo clippy --workspace -- -D warnings
just fmt            # cargo fmt --all
just test           # cargo test --workspace
just build          # cargo build --workspace
just run            # cargo run --package hello-headless
```

## Key Patterns

- **tools.rs** in `hello-core` is nearly identical to
  `dioxus-connector/crates/core/src/tools.rs` — same `PentestTool`
  trait, `ToolSchema`, `ToolResult`, `ToolRegistry`, `execute_timed`.
- **connector.rs** implements `BaseConnector` directly (no ToolEvent
  broadcast, no file browser, no workspace path).
- Every tool uses `execute_timed` for consistent duration tracking.
- Integration tests in `crates/core/tests/` exercise the full
  connector → registry → tool pipeline.
