# Flipper Zero Connector for Strike48

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/jtomek-strike48/flipper-connector)
[![Version](https://img.shields.io/badge/version-1.2.0-blue)](https://github.com/jtomek-strike48/flipper-connector/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

A comprehensive Strike48 connector for Flipper Zero, enabling AI-driven physical security testing with 24 production tools.

## 🎯 Overview

The Flipper Zero Connector integrates Flipper Zero with the Strike48 pentesting platform, exposing NFC, RFID, Sub-GHz, and BadUSB capabilities as discrete tools for autonomous AI-driven security assessments.

### Key Features

- **24 Production Tools** - NFC, RFID, Sub-GHz, BadUSB, filesystem, batch operations
- **USB HID Keyboard Emulation** - BadUSB with complete Ducky Script support
- **Enterprise Audit Logging** - Structured JSON logs with compliance support
- **File-Based Workflow** - Reliable alternative to app-based control
- **H10301 Wiegand** - Automatic encode/decode for access control testing
- **Batch Operations** - Efficient multi-file operations with graceful error handling

## 🚀 Quick Start

### Using Docker (Recommended)

```bash
# Clone repository
git clone https://github.com/jtomek-strike48/flipper-connector.git
cd flipper-connector

# Build Docker image
docker build -t flipper-connector:latest .

# Run with Docker Compose
docker-compose up -d

# View logs
docker-compose logs -f
```

### Local Development

```bash
# Prerequisites: Rust 1.75+, Flipper Zero connected via USB

# Build
cargo build --release

# Run tests
cargo test --workspace

# Run agent
cargo run --package flipper-agent
```

## 📋 Tool Categories

### Core Operations (5 tools)
- `flipper_device_info` - Get device information
- `flipper_file_list` - List files in directory
- `flipper_file_read` - Read file content
- `flipper_file_write` - Write file to device
- `flipper_file_delete` - Delete file

### Filesystem & Apps (4 tools)
- `flipper_dir_create` - Create directories
- `flipper_file_stat` - Get file/directory metadata
- `flipper_app_list` - List installed applications
- `flipper_app_info` - Get app information

### NFC Operations (2 tools)
- `flipper_nfc_read` - Read and parse NFC files
- `flipper_nfc_write` - Create NFC files

Supported formats: Bank Card, MIFARE Classic 1K/4K, NTAG203/213/215/216, Mifare Ultralight

### RFID Operations (2 tools)
- `flipper_rfid_read` - Read and parse RFID files with H10301 Wiegand decoding
- `flipper_rfid_write` - Create RFID files from facility+card or hex

Supported formats: EM4100, H10301, I40134

### Sub-GHz Operations (2 tools)
- `flipper_subghz_read` - Read and parse Sub-GHz files
- `flipper_subghz_write` - Create Sub-GHz remote files

Supported protocols: Princeton, KeeLoq, GateTX, Star Line, and more

### BadUSB Operations (5 tools)
- `flipper_badusb_upload` - Upload Ducky Scripts with syntax validation
- `flipper_badusb_list` - List all BadUSB scripts
- `flipper_badusb_read` - Read and analyze scripts
- `flipper_badusb_delete` - Delete scripts
- `flipper_badusb_validate` - Validate Ducky Script syntax

Supports 40+ Ducky Script commands for USB keyboard emulation

### Batch & Utility Operations (4 tools)
- `flipper_batch_read` - Read multiple files simultaneously
- `flipper_file_search` - Pattern-based file search with wildcards
- `flipper_nfc_clone` - Clone NFC files with UID modification
- `flipper_rfid_generate` - Generate sequential RFID badges (1-100)

## 🐳 Docker Deployment

### Build and Run

```bash
# Build image
docker build -t flipper-connector:latest .

# Run with USB access
docker run -d \
  --name flipper-connector \
  --device /dev/bus/usb:/dev/bus/usb \
  -v $(pwd)/logs:/var/log/flipper \
  flipper-connector:latest
```

### Docker Compose

```yaml
version: '3.8'

services:
  flipper-connector:
    image: flipper-connector:latest
    container_name: flipper-connector
    restart: unless-stopped

    environment:
      - RUST_LOG=info
      - FLIPPER_AUDIT_ENABLED=true

    volumes:
      - ./logs:/var/log/flipper
      - /dev/bus/usb:/dev/bus/usb:rw

    devices:
      - /dev/bus/usb:/dev/bus/usb

    cap_add:
      - SYS_RAWIO
```

See [docs/deployment.md](docs/deployment.md) for complete deployment guide.

## 🔐 Audit Logging

Enterprise-grade audit logging for compliance and security monitoring:

```rust
use flipper_core::audit::AuditConfig;
use std::path::PathBuf;

let audit_config = AuditConfig {
    enabled: true,
    output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl")),
    sanitize_data: true,
    ..Default::default()
};

let connector = FlipperConnector::with_audit_config(registry, Some(audit_config));
```

**Features:**
- Structured JSON Lines format
- Automatic sensitive data redaction
- Full execution context (tool, params, duration, results)
- Compliance support (HIPAA, SOX, PCI-DSS)

See [docs/audit-logging.md](docs/audit-logging.md) for complete guide.

## 📚 Documentation

### User Guides
- [Tool Usage Guide](docs/tool-usage-guide.md) - Complete reference for all 24 tools
- [Deployment Guide](docs/deployment.md) - Docker deployment and Strike48 integration
- [Audit Logging](docs/audit-logging.md) - Compliance and security monitoring

### File Format Specifications
- [NFC File Format](docs/nfc-file-format.md) - NFC file specifications with examples
- [RFID File Format](docs/rfid-file-format.md) - RFID formats with Wiegand encoding
- [Sub-GHz File Format](docs/subghz-file-format.md) - Sub-GHz protocol reference
- [BadUSB File Format](docs/badusb-file-format.md) - Complete Ducky Script reference

### Development Summaries
- [Week 1-4 Summaries](docs/) - Development progress and decisions

## 🏗️ Architecture

```
flipper-connector/
├── crates/
│   ├── flipper-core/        # BaseConnector, PentestTool trait, audit logging
│   ├── flipper-protocol/    # FlipperClient RPC wrapper
│   └── flipper-tools/       # 24 tool implementations
├── apps/
│   └── flipper-agent/       # Headless agent for Strike48
├── docs/                    # Comprehensive documentation
└── spike/                   # Research and prototypes
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run integration tests
cargo test --package flipper-core --test integration_test

# Run with logging
RUST_LOG=debug cargo test --workspace
```

**Test Coverage:**
- 24/24 integration tests passing
- Unit tests for core functionality
- Schema validation for all tools
- Audit logging tests

## 🛠️ Development

### Prerequisites

- **Rust:** 1.75+ with cargo
- **Flipper Zero:** Connected via USB
- **Strike48 SDK:** Local checkout at `../sdk-rs`

### Building from Source

```bash
# Check code
cargo check --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all

# Build release
cargo build --release --workspace
```

### Project Structure

- **flipper-core** - Core types, BaseConnector implementation, audit logging
- **flipper-protocol** - FlipperClient wrapper around flipper-rpc crate
- **flipper-tools** - All 24 tool implementations
- **flipper-agent** - Headless binary for Strike48 deployment

## 🔧 Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |
| `FLIPPER_AUDIT_ENABLED` | `true` | Enable audit logging |
| `FLIPPER_AUDIT_LOG` | `/var/log/flipper/audit.jsonl` | Audit log file path |

### USB Permissions

**Linux udev rules** (`/etc/udev/rules.d/99-flipper.rules`):

```bash
SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="5740", MODE="0666", GROUP="plugdev"
```

Reload rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## 📊 Usage Examples

### NFC Badge Reading

```json
{
  "tool": "flipper_nfc_read",
  "params": {
    "path": "/ext/nfc/office_badge.nfc"
  }
}
```

### RFID with Wiegand Decode

```json
{
  "tool": "flipper_rfid_read",
  "params": {
    "path": "/ext/lfrfid/access_card.rfid"
  }
}
```

**Returns:**
```json
{
  "key_type": "H10301",
  "data": "1C 69 CE",
  "facility_code": 14,
  "card_number": 13543,
  "decoded": "Facility: 14, Card: 13543"
}
```

### BadUSB Script Upload

```json
{
  "tool": "flipper_badusb_upload",
  "params": {
    "filename": "test_payload",
    "script": "DELAY 1000\nGUI r\nDELAY 500\nSTRING notepad\nENTER",
    "validate": true
  }
}
```

### Batch File Reading

```json
{
  "tool": "flipper_batch_read",
  "params": {
    "paths": [
      "/ext/nfc/badge1.nfc",
      "/ext/nfc/badge2.nfc",
      "/ext/lfrfid/card.rfid"
    ],
    "parse": true
  }
}
```

## 🎓 Use Cases

### Physical Access Assessment
- Read and analyze RFID/NFC badges
- Clone access cards for testing
- Generate sequential badges for systematic testing
- Test access control systems with H10301 Wiegand

### Wireless Attack Surface Analysis
- Capture and replay Sub-GHz signals
- Analyze garage door openers and key fobs
- Test RF security controls
- Duplicate wireless remotes

### USB Attack Simulation
- Test USB device controls with BadUSB
- Simulate keyboard-based attacks
- Validate endpoint security
- Assess user awareness training

### Security Research
- Analyze captured RF data
- Reverse engineer access control protocols
- Test physical security implementations
- Document vulnerabilities

## 🔒 Security & Compliance

### Responsible Use

⚠️ **Legal Notice:** This tool is for authorized security testing only.

**Only use on:**
- ✅ Systems you own
- ✅ With explicit written authorization
- ✅ During authorized penetration tests
- ✅ For educational purposes in controlled environments

**Unauthorized use is illegal** and may result in criminal prosecution.

### Compliance Features

- **Audit Trail:** Complete logs of all operations
- **Data Sanitization:** Automatic redaction of sensitive fields
- **Regulatory Support:** HIPAA, SOX, PCI-DSS compliance
- **Access Control:** User and session tracking

## 🚦 Project Status

**Current Version:** v1.2.0

**Released Features:**
- ✅ 24 production tools (v1.0.0)
- ✅ BadUSB with Ducky Script (v1.1.0)
- ✅ Audit logging system (v1.2.0)
- ✅ Docker deployment (v1.2.0)

**In Development:**
- ⏳ E2E testing with Prospector Studio
- 🔜 CI/CD pipeline with automated releases

**Roadmap (Phase 2):**
- Infrared (IR) remote control tools
- GPIO pin control and automation
- iButton (Dallas key) operations
- U2F security key emulation
- Bluetooth LE connectivity
- Firmware management tools

## 🤝 Contributing

Contributions welcome! Please see [CLAUDE.md](CLAUDE.md) for development guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Run linter: `cargo clippy --workspace`
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Strike48** - For the connector SDK and platform
- **Flipper Devices** - For the Flipper Zero and RPC protocol
- **Rust Community** - For excellent tooling and crates

## 📞 Support

For issues, questions, or feature requests:
- **GitHub Issues:** https://github.com/jtomek-strike48/flipper-connector/issues
- **Documentation:** [docs/](docs/) directory
- **Strike48 Support:** support@strike48.com

---

**Built with ❤️ using Rust and the Strike48 Connector SDK**

**Ready for production deployment!** 🎉
