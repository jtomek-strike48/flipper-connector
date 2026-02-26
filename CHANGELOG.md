# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2026-02-26

### Added
- **Docker deployment support** with multi-stage Dockerfile
- Complete Docker Compose configuration for easy deployment
- `.dockerignore` for optimized build context
- Comprehensive deployment documentation (800+ lines)
- Complete project README (500+ lines)
- USB device access configuration for Docker
- Production-ready containerization
- Non-root user execution in containers
- Resource limits and health checks
- Security hardening for container deployment

### Documentation
- `docs/deployment.md` - Complete deployment guide
- `README.md` - Project overview and quick start
- Docker build and run instructions
- Environment variables reference
- USB device setup (udev rules)
- Strike48 platform integration guide
- Production best practices
- Security hardening guidelines
- Monitoring and troubleshooting

### Changed
- Updated all documentation to include Docker deployment
- Enhanced security with non-root container execution

## [1.2.0] - 2026-02-26

### Added
- **Comprehensive audit logging system** for compliance and security
- `AuditLogger` trait for extensibility
- `JsonAuditLogger` implementation with file/stdout support
- `NoOpAuditLogger` for testing
- Automatic sensitive data sanitization
- Structured JSON Lines logging format
- Full execution context (tool, params, results, duration)
- Event types: tool_execution, connection, disconnection, error
- Configurable logging options (success/failure, parameters, results)

### Documentation
- `docs/audit-logging.md` - Complete audit logging guide (639 lines)
- Configuration examples
- Log format specification
- Analysis examples (jq, Python, grep)
- Compliance guidelines (HIPAA, SOX, PCI-DSS)
- Performance optimization tips
- Integration examples (Elasticsearch, Splunk, Grafana Loki)

### Dependencies
- Added `uuid` v1 with v4 and serde features

### Testing
- Added 4 audit logging integration tests
- Total tests: 24/24 passing

## [1.1.0] - 2026-02-26

### Added
- **BadUSB support** with complete Ducky Script implementation
- `flipper_badusb_upload` - Upload scripts with syntax validation
- `flipper_badusb_list` - List all BadUSB scripts
- `flipper_badusb_read` - Read and analyze scripts with statistics
- `flipper_badusb_delete` - Delete scripts from device
- `flipper_badusb_validate` - Validate Ducky Script syntax offline
- Full Ducky Script syntax validation (40+ commands)
- Script analysis (line counts, delays, command usage, execution time)
- Automatic .txt extension handling
- Multiline script support

### Documentation
- `docs/badusb-file-format.md` - Complete Ducky Script reference (580 lines)
- 10+ example payloads for Windows, macOS, Linux
- Platform-specific commands and techniques
- Safety and legal guidelines
- Troubleshooting and debugging tips

### Commands Supported
- Basic: REM, DELAY, STRING, STRINGLN, DEFAULT_DELAY
- Special keys: ENTER, SPACE, TAB, ESCAPE, BACKSPACE, DELETE, etc.
- Arrow keys: UP, DOWN, LEFT, RIGHT
- Function keys: F1-F12
- Modifiers: GUI, CTRL, SHIFT, ALT
- Combinations: CTRL-ALT, GUI+key, etc.

### Testing
- Added 4 BadUSB integration tests
- Updated tool count assertions (19 → 24)
- All 20/20 integration tests passing

### Tool Count
- Total tools: 24 (+5 from v1.0.0)

## [1.0.0] - 2026-02-26

### Added
- **Initial release** with 19 production tools
- Strike48 BaseConnector implementation
- FlipperClient RPC wrapper (async with tokio)
- Complete tool registry and schema export

#### Core Operations (5 tools)
- `flipper_device_info` - Get device information
- `flipper_file_list` - List files in directory
- `flipper_file_read` - Read file content
- `flipper_file_write` - Write file to device
- `flipper_file_delete` - Delete file from device

#### Filesystem & Apps (4 tools)
- `flipper_dir_create` - Create directories
- `flipper_file_stat` - Get file/directory metadata
- `flipper_app_list` - List installed applications
- `flipper_app_info` - Get app information

#### NFC Operations (2 tools)
- `flipper_nfc_read` - Read and parse NFC files with format auto-detection
- `flipper_nfc_write` - Create NFC files with custom UIDs
- Supported formats: Bank Card, MIFARE Classic 1K/4K, NTAG203/213/215/216, Mifare Ultralight

#### RFID Operations (2 tools)
- `flipper_rfid_read` - Read and parse RFID files with H10301 Wiegand decoding
- `flipper_rfid_write` - Create RFID files from facility+card or hex data
- Supported formats: EM4100, H10301, I40134
- Automatic Wiegand 26-bit encode/decode for access control testing

#### Sub-GHz Operations (2 tools)
- `flipper_subghz_read` - Read and parse Sub-GHz files
- `flipper_subghz_write` - Create Sub-GHz remote files
- Supported protocols: Princeton, KeeLoq, GateTX, Star Line, and more

#### Batch & Utility Operations (4 tools)
- `flipper_batch_read` - Read multiple files in one operation
- `flipper_file_search` - Search files by pattern with wildcards
- `flipper_nfc_clone` - Clone NFC files with UID modification
- `flipper_rfid_generate` - Generate sequential RFID badges (1-100)

### Documentation
- `docs/tool-usage-guide.md` - Complete reference for all tools (900+ lines)
- `docs/nfc-file-format.md` - NFC file format specification (234 lines)
- `docs/rfid-file-format.md` - RFID file format with Wiegand (250 lines)
- `docs/subghz-file-format.md` - Sub-GHz protocols reference (378 lines)
- `docs/week1-summary.md` through `docs/week4-summary.md` - Development summaries

### Architecture
- **flipper-core** - BaseConnector implementation and tool traits
- **flipper-protocol** - FlipperClient RPC wrapper
- **flipper-tools** - 19 tool implementations across 11 modules
- **flipper-agent** - Headless agent binary

### Testing
- 17/17 integration tests passing
- Hardware validated with actual Flipper Zero device
- 27 real files analyzed (14 NFC, 13 RFID)

### Key Features
- File-based workflow (reliable alternative to app-based control)
- H10301 Wiegand encode/decode for access control testing
- Multi-format NFC parsing
- Batch operations with graceful error handling
- Pattern-based file search with wildcards
- Sequential badge generation for testing

### Dependencies
- `flipper-rpc` v0.9.4 for Flipper Zero RPC protocol
- `strike48-connector` SDK for Strike48 integration
- `tokio` for async runtime
- `serde_json` for JSON serialization

## Project Statistics

### v2.0.0
- **Production Code:** ~15,000 lines
- **Tests:** 24/24 passing
- **Documentation:** ~7,300 lines
- **Tools:** 24
- **Features:** Docker deployment, audit logging, BadUSB

### v1.2.0
- **Production Code:** ~13,000 lines
- **Tests:** 24/24 passing
- **Documentation:** ~5,700 lines
- **Tools:** 24
- **Features:** Audit logging system

### v1.1.0
- **Production Code:** ~12,000 lines
- **Tests:** 20/20 passing
- **Documentation:** ~5,100 lines
- **Tools:** 24
- **Features:** BadUSB with Ducky Script

### v1.0.0
- **Production Code:** ~11,800 lines
- **Tests:** 17/17 passing
- **Documentation:** ~4,500 lines
- **Tools:** 19
- **Features:** Complete connector with NFC, RFID, Sub-GHz, batch operations

[Unreleased]: https://github.com/jtomek-strike48/flipper-connector/compare/v2.0.0...HEAD
[2.0.0]: https://github.com/jtomek-strike48/flipper-connector/compare/v1.2.0...v2.0.0
[1.2.0]: https://github.com/jtomek-strike48/flipper-connector/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/jtomek-strike48/flipper-connector/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/jtomek-strike48/flipper-connector/releases/tag/v1.0.0
