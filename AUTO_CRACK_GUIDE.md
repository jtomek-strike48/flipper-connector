# Automatic NFC Card Cracking

The new `flipper_nfc_auto_crack` tool provides a fully automated workflow to scan, detect, crack, and save NFC cards.

## What It Does

When you call this tool from Prospector Studio, it will:

1. **Wait for card** - Instructs you to place an NFC card on the Flipper
2. **Detect card type** - Identifies MIFARE Classic, Ultralight, NTAG, etc.
3. **Run dictionary attack** - Tries 10 common MIFARE keys against all sectors
4. **Run mfkey attack** - Recovers additional keys using cryptanalysis
5. **Save results** - Stores the cracked card data to `/ext/nfc/cracked/`
6. **Report success** - Returns detailed workflow log with success rate

## Usage in Prospector Studio

Simply ask the AI:

```
"Scan and crack the NFC card on my Flipper"
```

Or:

```
"Automatically detect and crack this MIFARE card"
```

The AI will call `flipper_nfc_auto_crack` with appropriate parameters.

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `output_dir` | string | `/ext/nfc/cracked` | Where to save cracked cards |
| `timeout` | number | `10` | Card detection timeout (seconds) |
| `aggressive` | boolean | `true` | Use aggressive key recovery |

## Example Workflow

```json
{
  "tool": "flipper_nfc_auto_crack",
  "parameters": {
    "output_dir": "/ext/nfc/cracked",
    "timeout": 10,
    "aggressive": true
  }
}
```

**Response:**
```json
{
  "success": true,
  "card_uid": "04A1B2C3",
  "card_type": "MIFARE Classic 1K",
  "keys_recovered": 12,
  "total_sectors": 16,
  "success_rate": "75.0%",
  "scan_file": "/ext/nfc/cracked/scan_04A1B2C3.nfc",
  "cracked_file": "/ext/nfc/cracked/cracked_04A1B2C3.nfc",
  "workflow_log": [...],
  "duration_seconds": 8,
  "instructions": "Partial key recovery. Some sectors remain locked..."
}
```

## Workflow Steps

The tool logs each step:

1. **waiting_for_card** - "Place NFC card on Flipper Zero..."
2. **scanning** - "Scanning for NFC card (timeout: 10s)"
3. **detected** - "Detected MIFARE Classic 1K with UID 04A1B2C3"
4. **saved_scan** - "Initial card data saved"
5. **dictionary_attack** - "Running dictionary attack..."
6. **dict_complete** - "Found keys for 4/16 sectors"
7. **mfkey_attack** - "Running mfkey attack..."
8. **mfkey_complete** - "Recovered 8 additional keys"
9. **reading_card** - "Reading complete card data..."
10. **save_complete** - "Cracked card data saved successfully"

## Success Rates

- **100%** - All 16 sectors cracked (full access)
- **75-99%** - Most sectors accessible (good for cloning)
- **50-74%** - Partial access (some data recovered)
- **<50%** - Limited access (may need physical techniques)

## What Happens Next

After successful cracking, you can:

### Clone the Card
```
"Clone this cracked card with a new UID"
```

Uses `flipper_nfc_clone`:
```json
{
  "tool": "flipper_nfc_clone",
  "parameters": {
    "source_path": "/ext/nfc/cracked/cracked_04A1B2C3.nfc",
    "dest_path": "/ext/nfc/cloned/badge.nfc",
    "new_uid": "04 D1 E2 F3"
  }
}
```

### Emulate the Card
```
"Emulate this cracked card"
```

Uses `flipper_nfc_emulate`:
```json
{
  "tool": "flipper_nfc_emulate",
  "parameters": {
    "path": "/ext/nfc/cracked/cracked_04A1B2C3.nfc",
    "duration": 0
  }
}
```

### Analyze the Data
```
"Read the cracked card and show me the data"
```

Uses `flipper_nfc_read`:
```json
{
  "tool": "flipper_nfc_read",
  "parameters": {
    "path": "/ext/nfc/cracked/cracked_04A1B2C3.nfc"
  }
}
```

## Tips for Best Results

### Physical Setup
- **Card positioning** - Center card on Flipper's NFC antenna
- **Keep still** - Don't move card during scan
- **Battery** - Ensure Flipper has sufficient charge
- **Distance** - Card should be touching or very close

### Aggressive Mode
- **Enabled** (default) - Tries all techniques, takes longer
- **Disabled** - Only dictionary attack, faster but fewer keys

### Common Keys Tried
1. `FF FF FF FF FF FF` - Factory default
2. `A0 A1 A2 A3 A4 A5` - Common key
3. `D3 F7 D3 F7 D3 F7` - MAD key
4. `00 00 00 00 00 00` - Blank key
5. `B0 B1 B2 B3 B4 B5` - Transport key
6. And 5 more...

## Troubleshooting

### "Card not detected"
- Open NFC app on Flipper manually
- Select "Read" mode
- Present card and wait for beep

### "Low success rate"
- Try aggressive mode: `"aggressive": true`
- Card may use custom keys
- Some sectors may be read-protected

### "No keys recovered"
- Card might be blank/empty
- Try different card type (not MIFARE Classic)
- Use Flipper's NFC app to verify card works

## Security & Compliance

⚠️ **Authorized Testing Only**

This tool is for:
- ✅ Authorized penetration testing
- ✅ Your own access cards/systems
- ✅ Security research with permission
- ✅ Badge cloning for redundancy (authorized)

**Illegal to use for:**
- ❌ Cloning cards you don't own
- ❌ Unauthorized access to facilities
- ❌ Bypassing security systems
- ❌ Identity theft or fraud

All operations are logged for audit purposes.

## See Also

- [NFC File Format](docs/nfc-file-format.md) - Understanding .nfc files
- [Tool Usage Guide](docs/tool-usage-guide.md) - All 109 tools documented
- [Prospector Setup](PROSPECTOR_SETUP.md) - Connecting to Prospector Studio
