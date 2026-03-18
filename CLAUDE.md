# Flipper Zero Connector

Production-ready Strike48 connector for Flipper Zero with 102 tools across 20 categories, including Unleashed firmware support.

## Architecture

Rust workspace with four crates:

- **flipper-core** — Core types, `PentestTool` trait, `ToolRegistry`,
  `FlipperConnector` (implements `BaseConnector` from the SDK),
  audit logging, metrics, error handling.
- **flipper-protocol** — `FlipperClient` RPC wrapper around the `flipper-rpc` crate,
  providing connection management and protocol abstraction.
- **flipper-tools** — 102 tool implementations across NFC, RFID, Sub-GHz (including bruteforce
  and remote creator), BadUSB, iButton, Infrared, GPIO, Bluetooth LE, U2F/FIDO2, Zigbee,
  firmware, storage, system utilities, display/audio, network, cryptography, protocol DB,
  scripting, batch operations, and security audit.
- **flipper-agent** — Headless binary for Strike48 platform deployment.

## SDK Dependency

Uses a **path dependency** to the local SDK checkout:

```
strike48-connector = { path = "../sdk-rs/crates/connector" }
```

## Build Commands

```bash
just check          # cargo check --workspace
just lint           # cargo clippy --workspace -- -D warnings
just fmt            # cargo fmt --all
just test           # cargo test --workspace
just build          # cargo build --workspace
just run            # cargo run --package flipper-agent
```

## Key Patterns

- **tools.rs** defines the `PentestTool` trait, `ToolSchema`, `ToolResult`,
  `ToolRegistry`, and `execute_timed` for consistent duration tracking.
- **connector.rs** implements `BaseConnector` with full audit logging,
  metrics collection, and error handling.
- **audit.rs** provides enterprise-grade audit logging with automatic
  sensitive data redaction and compliance support.
- **metrics.rs** implements Prometheus metrics for observability.
- Every tool uses `execute_timed` for consistent duration tracking.
- Integration tests in `crates/core/tests/` exercise the full
  connector → registry → tool pipeline for all 102 tools.

## Documentation

Complete documentation in the `docs/` directory:
- Tool usage guide with examples
- File format specifications (NFC, RFID, Sub-GHz, BadUSB, iButton, IR)
- Deployment guide for Docker and Strike48 integration
- Audit logging configuration
- Hardware operations reference (GPIO, Bluetooth, etc.)
