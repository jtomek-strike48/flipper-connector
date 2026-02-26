# Flipper Zero .nfc File Format

## Overview

The `.nfc` file format is a human-readable text format used by Flipper Zero to store NFC tag data. All `.nfc` files are stored in `/ext/nfc/` on the SD card.

## File Structure

All .nfc files follow this basic structure:

```
Filetype: Flipper NFC device
Version: 2
# Comments start with hash
Device type: <type>
# Common fields for all types
UID: <hex bytes>
ATQA: <hex bytes>
SAK: <hex byte>
# Type-specific data follows
```

## Common Fields

These fields appear in all .nfc file types:

| Field | Description | Format | Example |
|-------|-------------|--------|---------|
| `Filetype` | Always "Flipper NFC device" | String | `Flipper NFC device` |
| `Version` | File format version | Integer | `2` |
| `Device type` | Type of NFC tag | String | `Bank card`, `Mifare Classic`, etc. |
| `UID` | Unique Identifier | Hex bytes (space-separated) | `51 AC AC C1` |
| `ATQA` | Answer To reQuest type A | 2 hex bytes | `04 00` |
| `SAK` | Select AcKnowledge | 1 hex byte | `20` |

## Device Types

### 1. Bank Card

Simple payment card format with basic identification.

**Example:**
```
Filetype: Flipper NFC device
Version: 2
# Nfc device type can be UID, Mifare Ultralight, Bank card
Device type: Bank card
# UID, ATQA and SAK are common for all formats
UID: 51 AC AC C1
ATQA: 04 00
SAK: 20
# Bank card specific data
AID: A0 00 00 00 03 10 10
Name: VISA CREDIT
Number: 41 00 39 01 92 16 11 52
```

**Bank Card Specific Fields:**

| Field | Description | Format |
|-------|-------------|--------|
| `AID` | Application Identifier | Hex bytes |
| `Name` | Card type/name | String |
| `Number` | Partial card number | Hex bytes |

### 2. MIFARE Classic

Block-based memory structure, commonly used for access control (hotel keys, building access).

**Subtypes:**
- `Mifare Classic type: 1K` - 64 blocks (1024 bytes)
- `Mifare Classic type: 4K` - 256 blocks (4096 bytes)

**Example (1K):**
```
Filetype: Flipper NFC device
Version: 2
Device type: Mifare Classic
UID: F1 C2 6C 1C
ATQA: 04 00
SAK: 08
# Mifare Classic specific data
Mifare Classic type: 1K
# Mifare Classic blocks
Block 0: F1 C2 6C 1C 43 08 04 00 03 54 5D BB BB 7E C4 90
Block 1: E7 C0 99 56 B1 CA 20 3F BA C6 51 0F 94 AD E7 B4
Block 2: 52 00 04 00 01 00 00 00 00 00 00 00 00 00 00 00
Block 3: 00 00 00 00 00 00 FF 07 80 69 FF FF FF FF FF FF
...
Block 63: 00 00 00 00 00 00 FF 07 80 69 FF FF FF FF FF FF
```

**MIFARE Classic Specific Fields:**

| Field | Description | Format |
|-------|-------------|--------|
| `Mifare Classic type` | Memory size (1K or 4K) | String |
| `Block N` | Block data (16 bytes per block) | 16 hex bytes |

**Block Structure:**
- Each block is 16 bytes
- Block 0 contains manufacturer data and UID
- Every 4th block (3, 7, 11, etc.) contains sector trailer with access keys
- 1K cards: 64 blocks (0-63)
- 4K cards: 256 blocks (0-255)

**Common Access Control Pattern:**
- Sector trailers often end with: `FF 07 80 69 FF FF FF FF FF FF`
- Format: `[Key A (6 bytes)] [Access Bits (4 bytes)] [Key B (6 bytes)]`

### 3. NTAG / Ultralight

Page-based memory structure, commonly used for simple data storage and authentication.

**Subtypes:**
- `NTAG203` - 42 pages
- `NTAG213` - 45 pages
- `NTAG215` - 135 pages
- `NTAG216` - 231 pages
- `Mifare Ultralight` - Various page counts

**Example (NTAG203):**
```
Filetype: Flipper NFC device
Version: 2
Device type: NTAG203
UID: 04 4A 98 B2 E8 70 80
ATQA: 44 00
SAK: 00
# Mifare Ultralight specific data
Data format version: 1
Signature: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
Mifare version: 00 00 00 00 00 00 00 00
Counter 0: 0
Tearing 0: 00
Counter 1: 0
Tearing 1: 00
Counter 2: 0
Tearing 2: 00
Pages total: 42
Pages read: 42
Page 0: 04 4A 98 5E
Page 1: B2 E8 70 80
Page 2: AA 48 00 00
Page 3: D8 06 CA 19
...
Page 41: 00 00 00 00
Failed authentication attempts: 0
```

**NTAG/Ultralight Specific Fields:**

| Field | Description | Format |
|-------|-------------|--------|
| `Data format version` | Data structure version | Integer |
| `Signature` | Cryptographic signature (32 bytes) | 32 hex bytes |
| `Mifare version` | Version info (8 bytes) | 8 hex bytes |
| `Counter N` | One-way counter value | Integer |
| `Tearing N` | Tearing event flag | Hex byte |
| `Pages total` | Total page count | Integer |
| `Pages read` | Successfully read pages | Integer |
| `Page N` | Page data (4 bytes per page) | 4 hex bytes |
| `Failed authentication attempts` | Auth failure count | Integer |

**Page Structure:**
- Each page is 4 bytes
- Pages 0-1: UID
- Page 2: Internal/lock bytes
- Page 3: Capability container
- Pages 4+: User data
- Last pages: Configuration and lock pages

## File Locations

- **Storage path**: `/ext/nfc/`
- **Typical files**: Hotel keys, access cards, payment cards, transit passes
- **File naming**: User-defined `.nfc` extension

## Reading .nfc Files

Files can be read using the Flipper Protocol client:

```rust
let mut client = FlipperClient::new()?;
let content = client.read_file("/ext/nfc/example.nfc").await?;
let text = String::from_utf8(content)?;
```

## Parsing Tips

1. **Line-based format**: Parse line by line
2. **Comments**: Lines starting with `#` are comments
3. **Key-value pairs**: Format is `Key: Value`
4. **Hex values**: Space-separated hex bytes (e.g., `AA BB CC DD`)
5. **Device type determines structure**: Check `Device type` field first to determine which fields to expect

## Security Notes

- UID can be cloned/emulated
- MIFARE Classic keys may be default or weak
- Bank card data is typically partial (not full PAN)
- Sector trailers contain access keys and should be protected

## Real-World Examples

### Hotel Key Card (MIFARE Classic 1K)
- **Use case**: Hotel room access
- **UID**: Variable per card
- **Blocks**: Custom access control data in blocks 4-6, 16-18
- **Security**: Often uses default or weak keys

### Payment Card (Bank Card)
- **Use case**: Contactless payment
- **UID**: Card-specific
- **AID**: Standard payment application IDs
- **Data**: Partial card number, name, type

### Access Badge (NTAG)
- **Use case**: Building/room access
- **UID**: Badge identifier
- **Pages**: Custom access control data
- **Security**: May use password protection or signature

## Version History

- **Version 1**: Original format
- **Version 2**: Current format with enhanced metadata

## References

- Flipper Zero NFC documentation
- ISO/IEC 14443 standard (contactless cards)
- MIFARE Classic specification
- NXP NTAG specification
