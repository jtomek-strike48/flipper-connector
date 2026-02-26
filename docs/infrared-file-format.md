# Infrared Remote Control File Format - Flipper Zero

Complete specification for `.ir` files used with Flipper Zero infrared remote operations.

## Overview

Infrared files store IR remote control signals for TVs, air conditioners, projectors, and other consumer electronics. Essential for testing IR-based security systems and access controls.

**File Extension:** `.ir`
**Location:** `/ext/infrared/` on Flipper Zero SD card

---

## File Format Specification

### Structure

```
Filetype: IR signals file
Version: 1

name: <button_name>
type: parsed
protocol: <protocol>
address: <hex_address>
command: <hex_command>

name: <next_button>
type: parsed
...
```

### Supported Protocols

| Protocol | Common Use | Address Size | Command Size |
|----------|-----------|--------------|--------------|
| **NEC** | TVs, DVD players | 8-bit | 8-bit |
| **NECext** | Extended NEC | 16-bit | 16-bit |
| **Samsung32** | Samsung devices | 8-bit | 8-bit |
| **RC5** | Philips devices | 5-bit | 6-bit |
| **RC6** | Microsoft MCE | 8-bit | 8-bit |
| **SIRC** | Sony devices | 5-bit | 7-bit |
| **SIRC15** | Sony extended | 8-bit | 7-bit |
| **SIRC20** | Sony advanced | 13-bit | 7-bit |
| **Kaseikyo** | Panasonic | 16-bit | 8-bit |
| **RCA** | RCA devices | 4-bit | 8-bit |

---

## Example Files

### TV Remote (NEC Protocol)

```
Filetype: IR signals file
Version: 1

name: Power
type: parsed
protocol: NEC
address: 04 00 00 00
command: 08 00 00 00

name: Vol_up
type: parsed
protocol: NEC
address: 04 00 00 00
command: 02 00 00 00

name: Vol_dn
type: parsed
protocol: NEC
address: 04 00 00 00
command: 03 00 00 00

name: Ch_next
type: parsed
protocol: NEC
address: 04 00 00 00
command: 20 00 00 00
```

### AC Remote (Samsung32)

```
Filetype: IR signals file
Version: 1

name: Power
type: parsed
protocol: Samsung32
address: 0C 00 00 00
command: 01 00 00 00

name: Temp_up
type: parsed
protocol: Samsung32
address: 0C 00 00 00
command: 10 00 00 00

name: Temp_dn
type: parsed
protocol: Samsung32
address: 0C 00 00 00
command: 11 00 00 00
```

---

## Connector Tools

### Tool: `flipper_ir_read`

**Read IR remote files:**
```json
{
  "tool": "flipper_ir_read",
  "params": {
    "path": "/ext/infrared/TV_remote.ir"
  }
}
```

**Response:**
```json
{
  "signals": [
    {
      "name": "Power",
      "type": "parsed",
      "protocol": "NEC",
      "address": "04 00 00 00",
      "command": "08 00 00 00"
    },
    {
      "name": "Vol_up",
      "type": "parsed",
      "protocol": "NEC",
      "address": "04 00 00 00",
      "command": "02 00 00 00"
    }
  ],
  "count": 2
}
```

### Tool: `flipper_ir_write`

**Create IR remote files:**
```json
{
  "tool": "flipper_ir_write",
  "params": {
    "path": "/ext/infrared/custom_remote.ir",
    "buttons": [
      {
        "name": "Power",
        "protocol": "NEC",
        "address": "04 00 00 00",
        "command": "08 00 00 00"
      },
      {
        "name": "Mute",
        "protocol": "NEC",
        "address": "04 00 00 00",
        "command": "09 00 00 00"
      }
    ]
  }
}
```

### Tool: `flipper_ir_send`

**Transmit IR signals:**
```json
{
  "tool": "flipper_ir_send",
  "params": {
    "file_path": "/ext/infrared/TV_remote.ir",
    "button_name": "Power"
  }
}
```

**Response:**
```json
{
  "file_path": "/ext/infrared/TV_remote.ir",
  "button_name": "Power",
  "signal": {
    "name": "Power",
    "protocol": "NEC",
    "address": "04 00 00 00",
    "command": "08 00 00 00"
  },
  "instructions": "Use Flipper Zero IR app to transmit: Saved → Select file → Select button → Send"
}
```

---

## Security Testing Use Cases

### 1. IR-Based Access Control

**Scenario:** Testing IR sensors for room access

- Capture legitimate IR codes
- Replay codes from different locations
- Test range limitations
- Check for replay attack protection

### 2. Conference Room Takeover

**Scenario:** Unauthorized control of AV equipment

- Scan for common device signatures
- Use universal remote database
- Test projection system control
- Document access control gaps

### 3. IR Sensor Testing

**Scenario:** Security sensor assessment

- Test IR motion sensors
- Check alarm system IR codes
- Verify anti-jamming protection
- Document vulnerability to IR flooding

---

## Protocol Details

### NEC Protocol

- **Frequency:** 38 kHz carrier
- **Format:** Start (9ms) + Address (8-bit) + ~Address + Command (8-bit) + ~Command
- **Repeat:** 110ms intervals for held buttons
- **Most Common:** Consumer electronics

### Samsung32 Protocol

- **Frequency:** 38 kHz carrier
- **Format:** Custom Samsung format with 32-bit data
- **Use:** Samsung TVs, ACs, appliances
- **Notes:** Similar to NEC but proprietary

### RC5 Protocol

- **Frequency:** 36 kHz carrier
- **Format:** Bi-phase modulation
- **Use:** Philips devices
- **Notes:** Toggle bit changes on each press

---

## Universal Remote Database

Flipper Zero includes a universal remote database with common codes for:
- TVs (500+ models)
- Air Conditioners (300+ models)
- Projectors (100+ models)
- Audio Systems
- Set-top Boxes

**Access:** Universal Remotes → Select category → Select brand

---

## Security Considerations

### IR Vulnerabilities

1. **No Encryption** - All signals unencrypted
2. **Easy Replay** - Capture and replay trivial
3. **Limited Range** - But sufficient for many attacks
4. **No Authentication** - Receivers accept all valid codes
5. **Predictable Codes** - Sequential patterns common

### Recommendations

**For Penetration Testers:**
- Document all IR-controlled systems
- Test from various angles/distances
- Check for security-critical IR devices
- Verify no unintended IR receivers

**For Defenders:**
- Don't use IR for security functions
- Shield critical IR receivers
- Monitor for unauthorized IR transmissions
- Use wired controls for security systems

---

## Hardware Specifications

- **Flipper IR Transceiver:** Omnidirectional
- **Range:** Up to 5-10 meters (device dependent)
- **Frequency Support:** 30-56 kHz carrier
- **Learning:** Can learn new remotes
- **Storage:** Unlimited remotes on SD card

---

**Security Notice:** Only test IR systems you own or have authorization to test. Unauthorized control of devices may be illegal.
