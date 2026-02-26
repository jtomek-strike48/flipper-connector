# iButton File Format - Flipper Zero

Complete specification for `.ibtn` files used with Flipper Zero iButton (Dallas Key) operations.

## Overview

iButton files store 1-Wire authentication keys used for physical access control systems. These keys are extremely common in older facilities, data centers, and secure areas.

**File Extension:** `.ibtn`
**Location:** `/ext/ibutton/` on Flipper Zero SD card

---

## File Format Specification

### Structure

```
Filetype: Flipper iButton key
Version: 1
Key type: <key_type>
Data: <hex_data>
```

### Fields

| Field | Type | Description | Required |
|-------|------|-------------|----------|
| `Filetype` | String | Always "Flipper iButton key" | Yes |
| `Version` | Integer | File format version (currently 1) | Yes |
| `Key type` | String | Dallas, Cyfral, or Metakom | Yes |
| `Data` | Hex | Space-separated hex bytes | Yes |

---

## Supported Key Types

### 1. Dallas (1-Wire)

**Most Common** - Used in ~80% of iButton deployments

**Data Format:** 8 bytes
**Structure:** `[Family Code] [6-byte Serial] [CRC8]`

**Example:**
```
Filetype: Flipper iButton key
Version: 1
Key type: Dallas
Data: 01 23 45 67 89 AB CD EF
```

**Family Codes:**
- `0x01` - DS1990A (Basic ID-only key) - **Most common**
- `0x02` - DS1991 (Multi-key with memory)
- `0x04` - DS1994 (4Kb memory with clock)
- `0x0B` - DS1985 (16Kb EPROM)
- `0x10` - DS18S20 (Temperature sensor, sometimes used as key)
- `0x12` - DS2406 (Dual switch)

**Decoding Dallas Keys:**
1. Byte 0 = Family code
2. Bytes 1-6 = 48-bit unique serial number
3. Byte 7 = CRC8 checksum

**CRC Validation:**
- Dallas keys use CRC-8-Dallas/Maxim algorithm
- Polynomial: 0x31
- CRC is calculated over first 7 bytes

### 2. Cyfral

**Regional Usage** - Common in Russia/Eastern Europe

**Data Format:** 2 bytes
**Structure:** 16-bit proprietary format

**Example:**
```
Filetype: Flipper iButton key
Version: 1
Key type: Cyfral
Data: 12 34
```

**Notes:**
- Simpler protocol than Dallas
- No CRC validation
- Less common outside specific regions

### 3. Metakom

**Regional Usage** - Common in Russia/Eastern Europe

**Data Format:** 4 bytes
**Structure:** 32-bit proprietary format

**Example:**
```
Filetype: Flipper iButton key
Version: 1
Key type: Metakom
Data: AB CD EF 12
```

**Notes:**
- More complex than Cyfral
- Still primarily regional
- Proprietary encoding

---

## Connector Tools

### Tool: `flipper_ibutton_read`

Read and parse iButton files from Flipper Zero.

**Parameters:**
```json
{
  "path": "/ext/ibutton/office_key.ibtn"
}
```

**Response:**
```json
{
  "key_type": "Dallas",
  "data": "01 23 45 67 89 AB CD EF",
  "family_code": 1,
  "serial_number": "23 45 67 89 AB CD",
  "crc": 239,
  "decoded": "Family: 0x01, Serial: 23 45 67 89 AB CD, CRC: 0xEF"
}
```

### Tool: `flipper_ibutton_write`

Create iButton files on Flipper Zero.

**Parameters:**
```json
{
  "path": "/ext/ibutton/test_key.ibtn",
  "key_type": "Dallas",
  "data": "01 23 45 67 89 AB CD EF"
}
```

**Response:**
```json
{
  "path": "/ext/ibutton/test_key.ibtn",
  "key_type": "Dallas",
  "data": "01 23 45 67 89 AB CD EF",
  "message": "iButton file created successfully"
}
```

### Tool: `flipper_ibutton_emulate`

Prepare iButton file for emulation.

**Parameters:**
```json
{
  "source_path": "/ext/ibutton/captured_key.ibtn",
  "emulate_path": "/ext/ibutton/emulate.ibtn"
}
```

**Response:**
```json
{
  "source_path": "/ext/ibutton/captured_key.ibtn",
  "emulate_path": "/ext/ibutton/emulate.ibtn",
  "key_type": "Dallas",
  "data": "01 23 45 67 89 AB CD EF",
  "message": "Ready for emulation",
  "instructions": "Open iButton app on Flipper → Saved → Select file → Emulate"
}
```

---

## Physical Security Testing Use Cases

### 1. Access Control Assessment

**Scenario:** Testing iButton-based door access systems

**Workflow:**
1. Read captured keys: `flipper_ibutton_read`
2. Analyze key patterns (sequential serial numbers)
3. Clone keys for testing: `flipper_ibutton_write`
4. Test emulation: `flipper_ibutton_emulate`

**Common Findings:**
- Sequential serial numbers (predictable)
- No encryption beyond family code
- Easily cloneable with Flipper Zero
- No mutual authentication

### 2. Key Cloning

**Scenario:** Clone captured iButton keys

```json
// Read original key
{
  "tool": "flipper_ibutton_read",
  "params": {"path": "/ext/ibutton/original.ibtn"}
}

// Create clone with modified serial
{
  "tool": "flipper_ibutton_write",
  "params": {
    "path": "/ext/ibutton/clone.ibtn",
    "key_type": "Dallas",
    "data": "01 23 45 67 89 AB CD F0"  // Modified CRC
  }
}
```

### 3. Sequential Key Testing

**Scenario:** Generate sequential keys to find valid ranges

```python
# Generate keys with sequential serials
base_serial = 0x234567
for i in range(100):
    serial = base_serial + i
    # Calculate CRC and create key
```

---

## Security Considerations

### Dallas Key Vulnerabilities

1. **No Encryption** - Keys transmitted in plaintext
2. **Static Keys** - Same key always grants access
3. **Easy Cloning** - Flipper Zero can clone in seconds
4. **Predictable Serials** - Many deployments use sequential numbers
5. **No Mutual Auth** - Reader doesn't authenticate to key

### Recommendations

**For Penetration Testers:**
- Document all captured keys
- Test sequential serial numbers
- Check for pattern-based serials
- Verify no additional authentication layers
- Test physical reader access

**For Defenders:**
- Upgrade to MIFARE or RFID with encryption
- Implement multi-factor authentication
- Monitor access logs for anomalies
- Use biometrics + iButton
- Regular key rotation

---

## Hardware

### iButton Physical Characteristics

- **Form Factor:** 16mm diameter, 6mm height (looks like a coin battery)
- **Connector:** Single-wire plus ground (2 contacts)
- **Power:** Powered by reader (no battery)
- **Read Distance:** Contact-based (must touch reader)
- **Durability:** Very high (enclosed stainless steel)

### Flipper Zero iButton Support

- **Reading:** Via GPIO pins with 1-Wire protocol
- **Writing:** Software-based key generation
- **Emulation:** Can emulate Dallas, Cyfral, Metakom
- **Storage:** Unlimited keys on SD card

---

## Common Deployment Patterns

### Facility Types Using iButton

1. **Data Centers** - Server room access (very common)
2. **Industrial Facilities** - Equipment access control
3. **Older Buildings** - Installed 1990s-2010s
4. **Secure Storage** - Vault and safe rooms
5. **Research Labs** - Equipment and chemical storage

### Typical Serial Number Patterns

```
Sequential:
01 00 00 00 00 00 00 XX
01 00 00 00 00 00 01 XX
01 00 00 00 00 00 02 XX

Batch-based:
01 12 34 56 00 00 XX YY
01 12 34 56 00 01 XX YY

Random (rare):
01 A3 F7 2B 8C D1 9E XX
```

---

## Troubleshooting

### Invalid CRC

**Problem:** Dallas key CRC doesn't match

**Solutions:**
- Recalculate CRC using Dallas CRC-8 algorithm
- Use flipper-rfidtool or online calculator
- Flip

per may auto-correct on write

### Reader Not Recognizing Key

**Problem:** Emulated key not working

**Checklist:**
- Verify family code matches
- Check CRC is correct
- Ensure contact with reader
- Test original key first
- Check battery in reader

### Cyfral/Metakom Keys

**Problem:** Less documentation available

**Solutions:**
- Use Flipper Zero built-in read function
- Capture and replay rather than decoding
- Community resources (Flipper Zero Discord)

---

## References

- [Dallas/Maxim 1-Wire Protocol](https://www.maximintegrated.com/en/design/technical-documents/app-notes/1/126.html)
- [DS1990A Datasheet](https://datasheets.maximintegrated.com/en/ds/DS1990A.pdf)
- [CRC-8 Calculator](http://www.sunshine2k.de/coding/javascript/crc/crc_js.html)
- [Flipper Zero Docs](https://docs.flipper.net)

---

**Security Notice:** This information is for authorized security testing only. Unauthorized access to physical spaces using cloned keys is illegal.
