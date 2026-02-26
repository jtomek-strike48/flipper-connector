# Flipper Zero .rfid File Format

## Overview

The `.rfid` file format is a human-readable text format used by Flipper Zero to store Low Frequency RFID (LF-RFID) tag data. All `.rfid` files are stored in `/ext/lfrfid/` on the SD card.

## File Structure

All .rfid files follow this simple structure:

```
Filetype: Flipper RFID key
Version: 1
# Comments start with hash
Key type: <type>
# Data size varies by key type
Data: <hex bytes>
```

## File Format

| Field | Description | Format | Example |
|-------|-------------|--------|---------|
| `Filetype` | Always "Flipper RFID key" | String | `Flipper RFID key` |
| `Version` | File format version | Integer | `1` |
| `Key type` | Type of RFID protocol | String | `EM4100`, `H10301`, `I40134` |
| `Data` | Tag data in hex | Hex bytes (space-separated) | `1C 69 CE` |

## Supported Key Types

### 1. EM4100

**Description:** EM Microelectronic EM4100/EM4102 - Most common 125kHz proximity card.

**Data Size:** 5 bytes (40 bits)

**Format:**
- Customer ID (2 bytes)
- Tag ID (3 bytes)

**Example:**
```
Filetype: Flipper RFID key
Version: 1
Key type: EM4100
Data: 01 23 45 67 89
```

**Use Cases:**
- Generic proximity cards
- Simple access control
- Pet identification tags
- Low-security applications

### 2. H10301 (HID ProxCard II)

**Description:** HID ProxCard II with 26-bit Wiegand format (H10301).

**Data Size:** 3 bytes (26 bits)

**Format:**
- Facility Code (8 bits)
- Card Number (16 bits)
- Parity bits (2 bits)

**Example:**
```
Filetype: Flipper RFID key
Version: 1
Key type: H10301
Data: 1C 69 CE
```

**Decoding:**
- `1C 69 CE` = 0001 1100 0110 1001 1100 1110 (binary)
- Facility Code: 28 (0x1C)
- Card Number: 27086

**Use Cases:**
- Corporate access control
- Building security
- Office doors
- Common in commercial buildings

### 3. I40134 (Indala)

**Description:** Indala 26-bit format proximity card.

**Data Size:** 3 bytes (26 bits)

**Format:**
- Similar to H10301 (26-bit Wiegand)
- Facility Code (8 bits)
- Card Number (16 bits)

**Example:**
```
Filetype: Flipper RFID key
Version: 1
Key type: I40134
Data: 0A BC DE
```

**Use Cases:**
- Access control systems
- Time and attendance
- Motorola/Indala systems

## File Locations

- **Storage path**: `/ext/lfrfid/`
- **Also visible in**: `/any/lfrfid/` (appears to be a symlink or alternate path)
- **Typical files**: Access control badges, office keys, building entry cards

## Data Sizes by Type

| Key Type | Data Size | Bits | Description |
|----------|-----------|------|-------------|
| EM4100 | 5 bytes | 40 | Standard proximity card |
| H10301 | 3 bytes | 26 | HID 26-bit Wiegand |
| I40134 | 3 bytes | 26 | Indala 26-bit |

## Reading .rfid Files

Files can be read using the Flipper Protocol client:

```rust
let mut client = FlipperClient::new()?;
let content = client.read_file("/ext/lfrfid/badge.rfid").await?;
let text = String::from_utf8(content)?;
```

## Parsing Example

```rust
fn parse_rfid_file(content: &str) -> Result<(String, Vec<u8>)> {
    let mut key_type = String::new();
    let mut data = Vec::new();

    for line in content.lines() {
        if line.starts_with("Key type:") {
            key_type = line.split(':').nth(1).unwrap().trim().to_string();
        } else if line.starts_with("Data:") {
            let hex_str = line.split(':').nth(1).unwrap().trim();
            for byte_str in hex_str.split_whitespace() {
                data.push(u8::from_str_radix(byte_str, 16)?);
            }
        }
    }

    Ok((key_type, data))
}
```

## Security Notes

- **Low Frequency RFID is easily cloned**: These protocols have no encryption or authentication
- **UID-based access control**: Security relies solely on the tag ID
- **No mutual authentication**: Cards can be read and emulated passively
- **Predictable numbering**: Sequential card numbers can be guessed
- **Short range**: Typically 10-15cm read range (but can be extended with amplifiers)

## Wiegand Format (H10301 / I40134)

The 26-bit Wiegand format is structured as:

```
[1 bit even parity] [8 bits facility] [16 bits card number] [1 bit odd parity]
```

**Facility Code Range:** 0-255 (8 bits)
**Card Number Range:** 0-65535 (16 bits)

**Example Calculation for H10301:**
- Raw data: `1C 69 CE`
- Binary: `00011100 01101001 11001110`
- Facility: `00011100` = 28
- Card: `0110100111001110` = 27086

## Real-World Examples

### Office Access Badge (H10301)
```
Filetype: Flipper RFID key
Version: 1
Key type: H10301
Data: 1C 69 CE
```
- **Facility:** 28
- **Card Number:** 27086
- **Use:** Office building access

### Generic Proximity Card (EM4100)
```
Filetype: Flipper RFID key
Version: 1
Key type: EM4100
Data: 12 34 56 78 9A
```
- **Customer ID:** 0x1234
- **Tag ID:** 0x56789A
- **Use:** Generic access control

## Comparison with NFC

| Feature | LF-RFID (.rfid) | NFC (.nfc) |
|---------|-----------------|------------|
| Frequency | 125 kHz | 13.56 MHz |
| Range | 10-15 cm | 4-10 cm |
| Security | Very low | Medium to high |
| Data | ID only | ID + memory |
| Protocols | EM4100, H10301, I40134 | MIFARE, NTAG, Bank cards |
| Encryption | None | Optional (MIFARE Plus, DESFire) |

## Technical Details

### EM4100 Protocol
- Manchester encoding
- 64-bit data structure
- 4-bit header
- 8-bit customer ID
- 32-bit tag ID
- 4 row parity bits
- 4 column parity bits
- 1 stop bit

### HID ProxCard II (H10301)
- FSK (Frequency Shift Keying) encoding
- 26-bit Wiegand format
- Even and odd parity bits
- 84-bit total frame length
- Standard in North American access control

### Indala (I40134)
- PSK (Phase Shift Keying) encoding
- 26-bit standard format (multiple formats exist)
- Used in Motorola/Indala systems
- Less common than HID

## Version History

- **Version 1**: Current format

## References

- Flipper Zero RFID documentation
- HID ProxCard II specification
- EM4100/EM4102 datasheet
- Wiegand format standard
- Indala format specifications
