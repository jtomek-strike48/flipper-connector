# Flipper Zero Connector - Tool Usage Guide

Complete reference for all 24 tools in the Flipper Zero connector.

## Table of Contents

1. [Core Operations](#core-operations) (5 tools)
2. [Filesystem & Apps](#filesystem--apps) (4 tools)
3. [NFC Operations](#nfc-operations) (2 tools)
4. [RFID Operations](#rfid-operations) (2 tools)
5. [Sub-GHz Operations](#sub-ghz-operations) (2 tools)
6. [Batch & Utility Operations](#batch--utility-operations) (4 tools)
7. [BadUSB Operations](#badusb-operations) (5 tools)
8. [Common Workflows](#common-workflows)

---

## Core Operations

### 1. flipper_device_info

Get device information from connected Flipper Zero.

**Parameters:** None

**Returns:**
- Device name, serial number
- Hardware/software version
- Uptime, battery level

**Example:**
```json
{
  "tool": "flipper_device_info",
  "params": {}
}
```

### 2. flipper_file_list

List files in a directory.

**Parameters:**
- `path` (required): Directory path

**Example:**
```json
{
  "tool": "flipper_file_list",
  "params": {
    "path": "/ext/nfc"
  }
}
```

### 3. flipper_file_read

Read file content from Flipper Zero.

**Parameters:**
- `path` (required): Full file path

**Example:**
```json
{
  "tool": "flipper_file_read",
  "params": {
    "path": "/ext/nfc/badge.nfc"
  }
}
```

### 4. flipper_file_write

Write file to Flipper Zero.

**Parameters:**
- `path` (required): Full file path
- `content` (required): File content (string)

**Example:**
```json
{
  "tool": "flipper_file_write",
  "params": {
    "path": "/ext/test.txt",
    "content": "Hello World"
  }
}
```

### 5. flipper_file_delete

Delete file from Flipper Zero.

**Parameters:**
- `path` (required): Full file path

**Example:**
```json
{
  "tool": "flipper_file_delete",
  "params": {
    "path": "/ext/test.txt"
  }
}
```

---

## Filesystem & Apps

### 6. flipper_dir_create

Create directory on Flipper Zero.

**Parameters:**
- `path` (required): Directory path to create

**Example:**
```json
{
  "tool": "flipper_dir_create",
  "params": {
    "path": "/ext/my_captures"
  }
}
```

### 7. flipper_file_stat

Get file or directory metadata.

**Parameters:**
- `path` (required): File/directory path

**Returns:**
- Size in bytes
- Human-readable size
- Type (file/directory)

**Example:**
```json
{
  "tool": "flipper_file_stat",
  "params": {
    "path": "/ext/nfc/badge.nfc"
  }
}
```

### 8. flipper_app_list

List installed applications.

**Parameters:**
- `category` (optional): Filter by category (NFC, RFID, Sub-GHz, etc.)

**Example:**
```json
{
  "tool": "flipper_app_list",
  "params": {
    "category": "NFC"
  }
}
```

### 9. flipper_app_info

Get information about specific app.

**Parameters:**
- `path` (required): Full path to .fap file

**Example:**
```json
{
  "tool": "flipper_app_info",
  "params": {
    "path": "/ext/apps/NFC/nfc.fap"
  }
}
```

---

## NFC Operations

### 10. flipper_nfc_read

Read and parse NFC file.

**Supported Formats:**
- Bank Card
- MIFARE Classic (1K/4K)
- NTAG (203/213/215/216)
- Mifare Ultralight

**Parameters:**
- `path` (required): Path to .nfc file

**Returns:**
- Parsed device type, UID, ATQA, SAK
- Format-specific data (blocks/pages)

**Example:**
```json
{
  "tool": "flipper_nfc_read",
  "params": {
    "path": "/ext/nfc/office_badge.nfc"
  }
}
```

### 11. flipper_nfc_write

Create NFC file.

**Parameters:**
- `path` (required): Destination path
- `device_type` (required): "UID", "NTAG203", "Bank card", etc.
- `uid` (required): UID in hex (e.g., "04 12 34 56")
- `atqa` (optional): Default "44 00"
- `sak` (optional): Default "00"

**Example:**
```json
{
  "tool": "flipper_nfc_write",
  "params": {
    "path": "/ext/nfc/clone.nfc",
    "device_type": "NTAG203",
    "uid": "04 AA BB CC DD EE FF"
  }
}
```

---

## RFID Operations

### 12. flipper_rfid_read

Read and parse RFID file with automatic Wiegand decoding.

**Supported Formats:**
- EM4100 (5 bytes)
- H10301 (3 bytes, 26-bit Wiegand)
- I40134 (3 bytes, 26-bit Wiegand)

**Parameters:**
- `path` (required): Path to .rfid file

**Returns:**
- Key type, data
- For H10301: facility_code, card_number, decoded string

**Example:**
```json
{
  "tool": "flipper_rfid_read",
  "params": {
    "path": "/ext/lfrfid/badge.rfid"
  }
}
```

**Sample Output (H10301):**
```json
{
  "key_type": "H10301",
  "data": "1C 69 CE",
  "facility_code": 14,
  "card_number": 13543,
  "decoded": "Facility: 14, Card: 13543"
}
```

### 13. flipper_rfid_write

Create RFID file with Wiegand encoding.

**Parameters:**
- `path` (required): Destination path
- `key_type` (required): "EM4100", "H10301", or "I40134"
- **Option A - Direct hex:**
  - `data` (required): Hex data (e.g., "1C 69 CE")
- **Option B - H10301 from facility/card:**
  - `facility_code` (required): 0-255
  - `card_number` (required): 0-65535

**Example (direct hex):**
```json
{
  "tool": "flipper_rfid_write",
  "params": {
    "path": "/ext/lfrfid/test.rfid",
    "key_type": "H10301",
    "data": "1C 69 CE"
  }
}
```

**Example (facility/card):**
```json
{
  "tool": "flipper_rfid_write",
  "params": {
    "path": "/ext/lfrfid/badge_001.rfid",
    "key_type": "H10301",
    "facility_code": 42,
    "card_number": 12345
  }
}
```

---

## Sub-GHz Operations

### 14. flipper_subghz_read

Read and parse Sub-GHz file.

**Supported Formats:**
- Key format (decoded protocols)
- RAW format (timing data)

**Parameters:**
- `path` (required): Path to .sub file

**Returns:**
- Frequency (Hz and MHz)
- Protocol, preset
- Key data or RAW data
- is_raw flag

**Example:**
```json
{
  "tool": "flipper_subghz_read",
  "params": {
    "path": "/ext/subghz/garage.sub"
  }
}
```

### 15. flipper_subghz_write

Create Sub-GHz Key file.

**Supported Protocols:**
- Princeton
- KeeLoq
- GateTX
- Star Line
- And more...

**Parameters:**
- `path` (required): Destination path
- `frequency` (required): Frequency in Hz (e.g., 433920000)
- `protocol` (required): Protocol name
- `key` (required): Key data in hex
- `bit` (required): Number of bits
- `te` (optional): Time Element in µs (for Princeton)
- `preset` (optional): Modulation preset

**Example (Princeton garage remote):**
```json
{
  "tool": "flipper_subghz_write",
  "params": {
    "path": "/ext/subghz/garage.sub",
    "frequency": 315000000,
    "protocol": "Princeton",
    "key": "00 00 00 00 00 12 34 56",
    "bit": 24,
    "te": 400
  }
}
```

**Common Frequencies:**
- 315 MHz: `315000000` (North America)
- 433.92 MHz: `433920000` (Europe/Asia)
- 868.35 MHz: `868350000` (Europe)
- 915 MHz: `915000000` (North America ISM)

---

## Batch & Utility Operations

### 16. flipper_batch_read

Read multiple files in one operation.

**Parameters:**
- `paths` (required): Array of file paths
- `parse` (optional): Parse files by extension (default: true)

**Example:**
```json
{
  "tool": "flipper_batch_read",
  "params": {
    "paths": [
      "/ext/nfc/badge1.nfc",
      "/ext/nfc/badge2.nfc",
      "/ext/lfrfid/card1.rfid"
    ],
    "parse": true
  }
}
```

**Returns:**
- Array of results (successful reads)
- Array of errors (failed reads)
- Total, successful, and failed counts

### 17. flipper_file_search

Search for files by pattern.

**Parameters:**
- `pattern` (required): Search pattern with wildcards (* for any characters)
- `directories` (optional): Array of directories to search
- `extension` (optional): Filter by extension

**Pattern Examples:**
- `"*"` - All files
- `"badge*"` - Files starting with "badge"
- `"*test*"` - Files containing "test"
- `"*badge"` - Files ending with "badge"

**Example:**
```json
{
  "tool": "flipper_file_search",
  "params": {
    "pattern": "*office*",
    "extension": ".rfid"
  }
}
```

### 18. flipper_nfc_clone

Clone NFC file with optional UID modification.

**Parameters:**
- `source_path` (required): Source .nfc file
- `dest_path` (required): Destination path
- `new_uid` (optional): New UID in hex format

**Example:**
```json
{
  "tool": "flipper_nfc_clone",
  "params": {
    "source_path": "/ext/nfc/original.nfc",
    "dest_path": "/ext/nfc/clone.nfc",
    "new_uid": "04 11 22 33 44 55 66"
  }
}
```

### 19. flipper_rfid_generate

Generate sequential RFID badges for testing.

**Parameters:**
- `base_path` (required): Base path for files
- `facility_code` (required): Facility code (0-255)
- `start_card` (required): Starting card number
- `count` (required): Number of badges (1-100)

**Example:**
```json
{
  "tool": "flipper_rfid_generate",
  "params": {
    "base_path": "/ext/lfrfid/test_badge",
    "facility_code": 42,
    "start_card": 1000,
    "count": 10
  }
}
```

**Generates:**
- `/ext/lfrfid/test_badge_01000.rfid`
- `/ext/lfrfid/test_badge_01001.rfid`
- ...
- `/ext/lfrfid/test_badge_01009.rfid`

---

## BadUSB Operations

### 20. flipper_badusb_upload

Upload a BadUSB Ducky Script to the Flipper Zero.

**Parameters:**
- `filename` (required): Script filename (without .txt extension)
- `script` (required): Ducky Script content (multiline string)
- `validate` (optional): Validate syntax before upload (default: true)

**Example:**
```json
{
  "tool": "flipper_badusb_upload",
  "params": {
    "filename": "windows_info",
    "script": "REM System info collector\nDELAY 1000\nGUI r\nDELAY 500\nSTRING cmd\nENTER\nDELAY 1000\nSTRING systeminfo\nENTER",
    "validate": true
  }
}
```

### 21. flipper_badusb_list

List all BadUSB scripts on the Flipper Zero.

**Parameters:** None

**Example:**
```json
{
  "tool": "flipper_badusb_list",
  "params": {}
}
```

**Sample Response:**
```json
{
  "scripts": [
    {
      "name": "windows_info.txt",
      "path": "/ext/badusb/windows_info.txt",
      "size": 156,
      "size_human": "156 B"
    }
  ],
  "count": 1,
  "directory": "/ext/badusb"
}
```

### 22. flipper_badusb_read

Read and parse a BadUSB script from the Flipper Zero.

**Parameters:**
- `filename` (required): Script filename (with or without .txt extension)

**Returns:**
- Script content
- Analysis (line count, commands used, estimated duration, etc.)

**Example:**
```json
{
  "tool": "flipper_badusb_read",
  "params": {
    "filename": "windows_info"
  }
}
```

**Sample Response:**
```json
{
  "path": "/ext/badusb/windows_info.txt",
  "filename": "windows_info.txt",
  "script": "REM System info\nDELAY 1000\n...",
  "analysis": {
    "total_lines": 10,
    "command_lines": 7,
    "comment_lines": 2,
    "empty_lines": 1,
    "total_delay_ms": 3000,
    "estimated_duration_sec": 3.0,
    "commands_used": ["REM", "DELAY", "GUI", "STRING", "ENTER"],
    "command_count": 5
  }
}
```

### 23. flipper_badusb_delete

Delete a BadUSB script from the Flipper Zero.

**Parameters:**
- `filename` (required): Script filename (with or without .txt extension)

**Example:**
```json
{
  "tool": "flipper_badusb_delete",
  "params": {
    "filename": "old_script"
  }
}
```

### 24. flipper_badusb_validate

Validate Ducky Script syntax without uploading.

**Parameters:**
- `script` (required): Ducky Script content to validate

**Returns:**
- Validation status (valid/invalid)
- Analysis if valid
- Error message if invalid

**Example (Valid Script):**
```json
{
  "tool": "flipper_badusb_validate",
  "params": {
    "script": "DELAY 1000\nSTRING test\nENTER"
  }
}
```

**Response:**
```json
{
  "valid": true,
  "analysis": {
    "total_lines": 3,
    "command_lines": 3,
    "total_delay_ms": 1000,
    "commands_used": ["DELAY", "STRING", "ENTER"]
  }
}
```

**Example (Invalid Script):**
```json
{
  "tool": "flipper_badusb_validate",
  "params": {
    "script": "INVALID_COMMAND test\nDELAY bad_value"
  }
}
```

**Response:**
```json
{
  "valid": false,
  "error": "Line 1: Unknown command 'INVALID_COMMAND'"
}
```

### Supported Ducky Script Commands

**Basic Commands:**
- `REM` - Comment
- `DELAY` - Wait (milliseconds)
- `STRING` - Type text
- `STRINGLN` - Type text + ENTER

**Special Keys:**
- `ENTER`, `SPACE`, `TAB`, `ESCAPE`, `BACKSPACE`, `DELETE`
- `HOME`, `END`, `INSERT`, `PAGEUP`, `PAGEDOWN`
- `CAPSLOCK`, `NUMLOCK`, `SCROLLLOCK`, `PRINTSCREEN`

**Arrow Keys:**
- `UP`, `DOWN`, `LEFT`, `RIGHT`

**Function Keys:**
- `F1` through `F12`

**Modifiers:**
- `GUI` (Windows/Command key)
- `CTRL` / `CONTROL`
- `SHIFT`
- `ALT` / `OPTION`

**Modifier Combinations:**
- `CTRL-ALT DELETE`
- `CTRL-SHIFT ESC`
- `ALT-TAB`
- `GUI r` (Windows+R)

See [badusb-file-format.md](badusb-file-format.md) for complete syntax reference and examples.

---

## Common Workflows

### Workflow 1: Clone an NFC Badge

```json
// Step 1: Read original badge
{
  "tool": "flipper_nfc_read",
  "params": {"path": "/ext/nfc/original.nfc"}
}

// Step 2: Clone with modified UID
{
  "tool": "flipper_nfc_clone",
  "params": {
    "source_path": "/ext/nfc/original.nfc",
    "dest_path": "/ext/nfc/clone.nfc",
    "new_uid": "04 AA BB CC"
  }
}
```

### Workflow 2: Test RFID Access Control

```json
// Step 1: Read existing badge
{
  "tool": "flipper_rfid_read",
  "params": {"path": "/ext/lfrfid/office.rfid"}
}
// Returns: facility_code: 14, card_number: 13543

// Step 2: Generate sequential test badges
{
  "tool": "flipper_rfid_generate",
  "params": {
    "base_path": "/ext/lfrfid/test",
    "facility_code": 14,
    "start_card": 13540,
    "count": 10
  }
}
// Generates cards 13540-13549
```

### Workflow 3: Batch Process Multiple Cards

```json
// Step 1: Search for all office badges
{
  "tool": "flipper_file_search",
  "params": {
    "pattern": "*office*",
    "extension": ".rfid"
  }
}

// Step 2: Read all found badges at once
{
  "tool": "flipper_batch_read",
  "params": {
    "paths": [
      "/ext/lfrfid/office_1.rfid",
      "/ext/lfrfid/office_2.rfid",
      "/ext/lfrfid/office_3.rfid"
    ]
  }
}
```

### Workflow 4: Create Custom Sub-GHz Remote

```json
// Create garage door remote
{
  "tool": "flipper_subghz_write",
  "params": {
    "path": "/ext/subghz/my_garage.sub",
    "frequency": 315000000,
    "protocol": "Princeton",
    "key": "00 00 00 00 00 AB CD EF",
    "bit": 24,
    "te": 400
  }
}

// Verify the file
{
  "tool": "flipper_subghz_read",
  "params": {"path": "/ext/subghz/my_garage.sub"}
}
```

### Workflow 5: Organize Captures

```json
// Step 1: Create organized directories
{
  "tool": "flipper_dir_create",
  "params": {"path": "/ext/captures_2026"}
}
{
  "tool": "flipper_dir_create",
  "params": {"path": "/ext/captures_2026/badges"}
}
{
  "tool": "flipper_dir_create",
  "params": {"path": "/ext/captures_2026/remotes"}
}

// Step 2: Search and organize files
{
  "tool": "flipper_file_search",
  "params": {
    "pattern": "*",
    "directories": ["/ext/nfc", "/ext/lfrfid"]
  }
}
```

---

## Tool Categories Summary

| Category | Count | Tools |
|----------|-------|-------|
| Core Operations | 5 | device_info, file_list, file_read, file_write, file_delete |
| Filesystem & Apps | 4 | dir_create, file_stat, app_list, app_info |
| NFC | 2 | nfc_read, nfc_write |
| RFID | 2 | rfid_read, rfid_write |
| Sub-GHz | 2 | subghz_read, subghz_write |
| Batch & Utility | 4 | batch_read, file_search, nfc_clone, rfid_generate |
| BadUSB | 5 | badusb_upload, badusb_list, badusb_read, badusb_delete, badusb_validate |
| **Total** | **24** | |

---

## Tips & Best Practices

1. **Use batch_read for efficiency** - Read multiple files at once instead of individual calls
2. **Search before manual specification** - Use file_search to discover files dynamically
3. **Clone for testing** - Use nfc_clone to create test variants without modifying originals
4. **Generate sequentially** - Use rfid_generate for systematic access control testing
5. **Organize with directories** - Create logical directory structures for different engagements
6. **Verify after write** - Always read back files after writing to confirm success

---

## Error Handling

All tools return a consistent format:
```json
{
  "success": true/false,
  "data": {},
  "error": "error message if failed",
  "duration_ms": 0
}
```

Common errors:
- **File not found**: Check path spelling and SD card status
- **Invalid format**: Verify file format matches expected structure
- **Connection failed**: Ensure Flipper Zero is connected via USB

---

## Support

For issues or questions:
- Documentation: `/docs/` directory
- File Format Guides: `nfc-file-format.md`, `rfid-file-format.md`, `subghz-file-format.md`
- Week Summaries: `week1-summary.md` through `week4-summary.md`
