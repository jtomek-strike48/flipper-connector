# Connecting Flipper Zero to Prospector Studio

This guide shows you how to hook your Flipper Zero to Prospector Studio so AI agents can control it.

## Quick Start

### 1. Set Environment Variables

```bash
# Prospector Studio address (default: localhost:50061)
export STRIKE48_HOST=localhost:50061

# Tenant ID (default: default)
export TENANT_ID=default

# Log level (optional, default: info)
export RUST_LOG=info
```

### 2. Run the Agent

```bash
./run-agent.sh
```

Or directly:
```bash
cargo run --package flipper-agent --release
```

### 3. Verify Connection

The agent will:
1. Connect to your Flipper Zero at `/dev/ttyACM0`
2. Register 108 tools with Prospector Studio
3. Wait for commands from AI agents

You should see:
```
🐬 Flipper Zero Connector Agent
   Version: 0.1.0

✅ Registered 108 tools across 20 categories
✅ Connector type: flipper-zero

📋 Available tool categories:
   • NFC (7 tools) - MIFARE cracking, cloning, emulation
   • RFID (3 tools) - Low frequency card operations
   • Sub-GHz (4 tools) - RF protocol capture & bruteforce
   • BadUSB/BadKB (7 tools) - USB & Bluetooth HID attacks
   • iButton, IR, GPIO, BLE, U2F, Zigbee, and more...

🚀 Connecting to Prospector Studio...
   STRIKE48_HOST: localhost:50061
   TENANT_ID: default

INFO flipper-agent: Starting flipper-zero connector
```

## Configuration Options

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `STRIKE48_HOST` | `localhost:50061` | Prospector Studio gRPC endpoint |
| `TENANT_ID` | `default` | Your tenant identifier |
| `RUST_LOG` | `info` | Log level: `trace`, `debug`, `info`, `warn`, `error` |
| `FLIPPER_AUDIT_ENABLED` | `true` | Enable audit logging |
| `FLIPPER_AUDIT_LOG` | `/var/log/flipper/audit.jsonl` | Audit log path |

### Connecting to Remote Prospector Studio

If Prospector Studio is running on a different machine:

```bash
export STRIKE48_HOST=prospector.example.com:50061
export TENANT_ID=your-tenant-id
./run-agent.sh
```

## Available Tools

Once connected, AI agents in Prospector Studio can use any of the 108 tools:

### NFC Operations (7 tools)
- `flipper_nfc_read` - Read NFC files
- `flipper_nfc_write` - Create NFC files
- `flipper_nfc_clone` - Clone cards with UID modification
- `flipper_nfc_detect` - Detect card types
- `flipper_nfc_mfkey` - MIFARE key recovery
- `flipper_nfc_dict_attack` - Dictionary attack
- `flipper_nfc_emulate` - Card emulation

### RFID Operations (3 tools)
- `flipper_rfid_read` - Read low-frequency cards
- `flipper_rfid_write` - Write RFID cards
- `flipper_rfid_generate` - Generate RFID sequences

### Sub-GHz Operations (4 tools)
- `flipper_subghz_read` - Capture RF signals
- `flipper_subghz_write` - Create RF files
- `flipper_subghz_bruteforce` - Static code bruteforce
- `flipper_subghz_remote` - 5-button remote creator

### BadUSB/BadKB Operations (7 tools)
- `flipper_badusb_upload` - Upload USB scripts
- `flipper_badusb_list` - List scripts
- `flipper_badusb_read` - Read scripts
- `flipper_badusb_delete` - Delete scripts
- `flipper_badusb_validate` - Validate Ducky Script
- `flipper_badkb_upload` - Upload Bluetooth scripts
- `flipper_badkb_execute` - Execute inline scripts

### And 87 More Tools

See [docs/tool-usage-guide.md](docs/tool-usage-guide.md) for complete documentation.

## Testing the Connection

### Option 1: Via Prospector Studio UI

In Prospector Studio, you should see "flipper-zero" connector available with 108 tools.

Ask the AI: "Read the NFC file at /ext/nfc/test.nfc on my Flipper"

### Option 2: Using nfc-test Binary

For direct testing without Prospector:

```bash
# Detect card type
cargo run --package nfc-test -- detect

# Write test file
cargo run --package nfc-test -- write /ext/nfc/test.nfc "Mifare Classic" "04 11 22 33"

# Read it back
cargo run --package nfc-test -- read /ext/nfc/test.nfc

# Clone with new UID
cargo run --package nfc-test -- clone /ext/nfc/test.nfc /ext/nfc/clone.nfc "04 AA BB CC"
```

## Troubleshooting

### Agent Can't Connect to Flipper

**Error:** `flipper-rpc error: connection failed`

**Solution:**
```bash
# Check if Flipper is connected
ls /dev/ttyACM0

# Check USB permissions
sudo usermod -a -G dialout $USER
# Then logout and login

# Or add udev rule
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="5740", MODE="0666", GROUP="plugdev"' | sudo tee /etc/udev/rules.d/99-flipper.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### Agent Can't Connect to Prospector Studio

**Error:** `Failed to connect to Strike48 host`

**Solution:**
```bash
# Check Prospector Studio is running
# Verify the host and port
echo $STRIKE48_HOST

# Check network connectivity
telnet localhost 50061

# Try with explicit host
STRIKE48_HOST=localhost:50061 ./run-agent.sh
```

### File Not Found Errors

**Error:** `ERROR_STORAGE_NOT_EXIST`

**Solution:**
Files must exist on the Flipper's SD card. Use the Flipper's NFC app to read a card first, or create test files:

```bash
cargo run --package nfc-test -- write /ext/nfc/test.nfc "Mifare Classic" "04 11 22 33"
```

## Architecture

```
┌─────────────────────┐
│ Prospector Studio   │
│ (AI Agent)          │
└──────────┬──────────┘
           │ gRPC
           │ STRIKE48_HOST:50061
           │
┌──────────▼──────────┐
│  flipper-agent      │
│  (Connector)        │
│  • 108 tools        │
│  • Strike48 SDK     │
└──────────┬──────────┘
           │ USB
           │ /dev/ttyACM0
           │
┌──────────▼──────────┐
│   Flipper Zero      │
│   (Hardware)        │
│   • Unleashed FW    │
└─────────────────────┘
```

## Security & Compliance

### Audit Logging

All tool executions are logged to `/var/log/flipper/audit.jsonl`:

```json
{
  "timestamp": "2024-03-18T10:30:45.123Z",
  "tool": "flipper_nfc_read",
  "parameters": {"path": "/ext/nfc/badge.nfc"},
  "success": true,
  "duration_ms": 150,
  "result": {"uid": "REDACTED"},
  "connector_version": "0.1.0"
}
```

Sensitive data (UIDs, keys, card data) is automatically redacted.

### Authorized Use Only

⚠️ **Legal Notice:** This tool is for authorized security testing only.

**Only use on:**
- ✅ Systems you own
- ✅ With explicit written authorization
- ✅ During authorized penetration tests

**Unauthorized use is illegal** and may result in criminal prosecution.

## Next Steps

- Read [docs/tool-usage-guide.md](docs/tool-usage-guide.md) for all tool documentation
- Review [docs/nfc-file-format.md](docs/nfc-file-format.md) for file format specs
- Check [docs/audit-logging.md](docs/audit-logging.md) for compliance setup
- See [README.md](README.md) for full project documentation

## Support

- **Issues:** https://github.com/jtomek-strike48/flipper-connector/issues
- **Documentation:** [docs/](docs/)
- **Strike48 Support:** support@strike48.com
