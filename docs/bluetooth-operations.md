# Bluetooth LE Operations - Flipper Zero

Complete guide to Bluetooth Low Energy (BLE) scanning, enumeration, and security testing.

## Overview

Flipper Zero's Bluetooth LE capabilities enable wireless device discovery, service enumeration, and security assessment of BLE-based access controls, IoT devices, and wearables.

---

## BLE Fundamentals

### Architecture

```
┌──────────────┐
│  Application │  (User apps, services)
├──────────────┤
│     GATT     │  (Generic Attribute Profile)
├──────────────┤
│     ATT      │  (Attribute Protocol)
├──────────────┤
│     L2CAP    │  (Logical Link Control)
├──────────────┤
│     Link     │  (Physical layer)
└──────────────┘
```

### Key Concepts

- **GATT** - Generic Attribute Profile (services/characteristics)
- **UUID** - Universally Unique Identifier (128-bit service IDs)
- **Characteristics** - Data values with properties (read/write/notify)
- **Descriptors** - Metadata about characteristics
- **Advertising** - Broadcast packets for discovery

---

## Connector Tools

### Tool: `flipper_ble_scan`

**Scan for BLE devices:**
```json
{
  "tool": "flipper_ble_scan",
  "params": {
    "duration": 10,
    "active": false
  }
}
```

**Parameters:**
- `duration` - Scan time in seconds (1-60)
- `active` - Active scanning (requests more data)

**Passive vs Active Scanning:**
| Type | Data | Detection | Use Case |
|------|------|-----------|----------|
| **Passive** | Advertising only | Stealthy | Reconnaissance |
| **Active** | + Scan responses | Detectable | Full enumeration |

**Response:**
```json
{
  "scan_type": "passive",
  "duration": 10,
  "message": "BLE scan prepared",
  "note": "Devices found will include name, MAC, RSSI, and services"
}
```

### Tool: `flipper_ble_device_info`

**Get device information:**
```json
{
  "tool": "flipper_ble_device_info",
  "params": {
    "mac_address": "AA:BB:CC:DD:EE:FF"
  }
}
```

**Available Information:**
- Device name (if advertised)
- MAC address
- RSSI (signal strength)
- Advertised services (UUIDs)
- Manufacturer data
- Connection status

### Tool: `flipper_ble_enumerate`

**Enumerate GATT services:**
```json
{
  "tool": "flipper_ble_enumerate",
  "params": {
    "mac_address": "AA:BB:CC:DD:EE:FF",
    "save_path": "/ext/bluetooth/device_services.txt"
  }
}
```

**Enumeration Includes:**
- All GATT services (primary & secondary)
- Service characteristics
- Characteristic properties (read, write, notify, indicate)
- Descriptors
- UUID meanings (from Bluetooth SIG database)

### Tool: `flipper_ble_security_test`

**Test device security:**
```json
{
  "tool": "flipper_ble_security_test",
  "params": {
    "mac_address": "AA:BB:CC:DD:EE:FF",
    "tests": ["pairing", "encryption", "authentication"]
  }
}
```

**Security Tests:**
- **Pairing** - Required bonding method
- **Encryption** - Link layer encryption status
- **Authentication** - MITM protection
- **Authorization** - Read/write permissions
- **Privacy** - Address randomization

---

## Standard BLE Services

### Common UUIDs

| UUID (16-bit) | Service | Common In |
|---------------|---------|-----------|
| `0x180A` | Device Information | Most devices |
| `0x180F` | Battery Service | Wearables, peripherals |
| `0x1800` | Generic Access | All devices |
| `0x1801` | Generic Attribute | All devices |
| `0x1805` | Current Time | Watches |
| `0x1810` | Blood Pressure | Medical devices |
| `0x181A` | Environmental Sensing | IoT sensors |
| `0x181C` | User Data | Fitness trackers |
| `0x181D` | Weight Scale | Health devices |

### Custom Services

Format: `XXXXXXXX-0000-1000-8000-00805F9B34FB`
- Vendor-specific services
- Proprietary protocols
- Custom applications

---

## Security Testing Use Cases

### 1. BLE Lock Assessment

**Scenario:** Testing smart door locks

**Workflow:**
```json
// 1. Discover lock
{
  "tool": "flipper_ble_scan",
  "params": {"duration": 15, "active": true}
}

// 2. Enumerate services
{
  "tool": "flipper_ble_enumerate",
  "params": {"mac_address": "AA:BB:CC:DD:EE:FF"}
}

// 3. Test security
{
  "tool": "flipper_ble_security_test",
  "params": {
    "mac_address": "AA:BB:CC:DD:EE:FF",
    "tests": ["pairing", "encryption", "authentication"]
  }
}
```

**Common Findings:**
- No pairing required ("Just Works")
- Unencrypted characteristics
- Weak PINs (000000, 123456)
- Static MAC addresses
- Replay-able commands

### 2. Fitness Tracker Enumeration

**Scenario:** Wearable device assessment

**Services to Check:**
- Heart Rate (0x180D)
- User Data (0x181C)
- Device Information (0x180A)
- Battery (0x180F)

**Privacy Concerns:**
- Personal health data exposure
- User identification via MAC
- Unencrypted data transmission

### 3. IoT Sensor Analysis

**Scenario:** Smart home device testing

**Check for:**
- Unsecured sensor data
- Control characteristics (writable)
- Default credentials
- Firmware update mechanism
- Debug services

---

## BLE Security

### Pairing Methods

| Method | Security | User Experience | Vulnerability |
|--------|----------|-----------------|---------------|
| **Just Works** | None | Seamless | No MITM protection |
| **Passkey Entry** | Low | PIN entry | Weak PINs common |
| **Numeric Comparison** | Medium | Compare numbers | Social engineering |
| **Out-of-Band** | High | NFC/QR code | Requires second channel |

### Encryption Levels

1. **No Encryption** - Plaintext (common in cheap devices)
2. **Link Layer Encryption** - AES-128 CCM
3. **Application Layer** - Custom encryption

### Common Vulnerabilities

**1. No Pairing Required**
- Device accepts connections without authentication
- MITM attacks trivial
- Data readable/writable by anyone

**2. Weak Pairing**
- 4-digit or 6-digit PINs
- Default PINs (000000, 123456)
- Static PINs printed on device

**3. Improper Authorization**
- Services accessible without pairing
- Write access to critical characteristics
- No permission checks

**4. Static MAC Addresses**
- Device tracking
- Fingerprinting users
- Privacy concerns

**5. Information Disclosure**
- Sensitive data in advertising packets
- Unencrypted characteristics
- Debug services exposed

---

## GATT Service Discovery

### Service Hierarchy

```
Service (UUID)
├── Characteristic 1 (UUID)
│   ├── Properties: Read, Write, Notify
│   ├── Value: <data>
│   └── Descriptors
│       ├── Client Characteristic Configuration (0x2902)
│       └── Characteristic User Description (0x2901)
├── Characteristic 2 (UUID)
│   └── ...
└── Characteristic N (UUID)
```

### Characteristic Properties

| Property | Description | Security Impact |
|----------|-------------|-----------------|
| **Read** | Can read value | Data exposure |
| **Write** | Can write value | Command injection |
| **Write No Response** | Write without ACK | Faster attacks |
| **Notify** | Server-initiated updates | Monitoring |
| **Indicate** | Notify with ACK | Reliable monitoring |
| **Broadcast** | Advertising data | Privacy concern |

---

## Attack Scenarios

### 1. Unauthorized Access

**Target:** BLE door locks, garage openers

**Attack:**
```
1. Scan for device
2. Enumerate services
3. Find "unlock" characteristic
4. Write unlock command (if no pairing)
```

**Mitigation:**
- Require secure pairing
- Implement challenge-response
- Use encrypted characteristics

### 2. MITM (Man-in-the-Middle)

**Target:** Devices using "Just Works" pairing

**Attack:**
```
1. Intercept pairing request
2. Complete pairing with both devices
3. Relay communications
4. Sniff/modify data
```

**Mitigation:**
- Use Numeric Comparison or OOB
- Implement certificate pinning
- Detect relay attacks (latency)

### 3. Replay Attacks

**Target:** Devices with static commands

**Attack:**
```
1. Capture unlock command
2. Replay command later
3. Gain unauthorized access
```

**Mitigation:**
- Use rolling codes
- Implement nonces/timestamps
- Require fresh pairing

### 4. Brute Force Pairing

**Target:** Devices with 6-digit PINs

**Attack:**
```
Possible PINs: 1,000,000 (000000-999999)
Common patterns: 000000, 123456, 111111
Time: Seconds to minutes
```

**Mitigation:**
- Rate limiting
- Account lockout
- Strong random PINs
- Use Numeric Comparison instead

---

## Testing Methodology

### 1. Reconnaissance

```json
// Passive scan
{
  "tool": "flipper_ble_scan",
  "params": {"duration": 30, "active": false}
}
```

**Document:**
- All nearby devices
- Signal strengths (RSSI)
- Advertised services
- Device names

### 2. Enumeration

```json
// Active enumeration
{
  "tool": "flipper_ble_enumerate",
  "params": {
    "mac_address": "TARGET_MAC",
    "save_path": "/ext/bluetooth/target_services.txt"
  }
}
```

**Analyze:**
- Service UUIDs
- Characteristic properties
- Security requirements
- Custom services

### 3. Security Assessment

```json
// Test security
{
  "tool": "flipper_ble_security_test",
  "params": {
    "mac_address": "TARGET_MAC",
    "tests": ["pairing", "encryption", "authentication", "authorization"]
  }
}
```

**Evaluate:**
- Pairing requirements
- Encryption status
- Access controls
- Privacy protections

### 4. Exploitation

**Only if authorized:**
- Attempt unauthorized access
- Test command injection
- Verify security controls
- Document vulnerabilities

---

## Recommendations

### For Penetration Testers

**Best Practices:**
- Get explicit written authorization
- Document all findings
- Test in isolated environment when possible
- Verify fixes after reporting
- Follow responsible disclosure

**Reporting:**
- List all vulnerable devices
- Provide exploitation proof-of-concept
- Recommend specific mitigations
- Include CVSS scores
- Suggest remediation timeline

### For Defenders

**Device Selection:**
- Require Bluetooth 5.0+
- Verify secure pairing support
- Check for security certifications
- Review vendor security practices

**Configuration:**
- Enable strongest pairing method
- Use random MAC addresses
- Disable advertising when not needed
- Update firmware regularly
- Monitor for unauthorized connections

**Network Security:**
- Segment BLE devices
- Monitor BLE traffic
- Implement intrusion detection
- Log all connections
- Alert on pairing attempts

---

## Hardware Limitations

### Flipper Zero BLE

**Capabilities:**
- BLE 5.0 support
- Passive/active scanning
- GATT enumeration
- Connection support
- Advertising packet analysis

**Limitations:**
- Cannot perform actual pairing attacks (use dedicated tools)
- Limited to Bluetooth LE (no Classic)
- Range: ~10-30 meters
- No packet injection (use Ubertooth/HackRF)

### Recommended Additional Tools

**For Advanced Testing:**
- **Ubertooth One** - Bluetooth sniffing
- **Nordic nRF52840 DK** - Development/testing
- **HackRF One** - Wide-spectrum SDR
- **Wireshark** - Packet analysis
- **Bettercap** - Automated attacks

---

## Legal and Ethical Considerations

### Authorized Testing Only

**Requirements:**
- Written authorization
- Defined scope
- Clear objectives
- Incident response plan
- Insurance coverage

### Prohibited Activities

⚠️ **Never:**
- Test devices you don't own without permission
- Disrupt medical devices
- Attack critical infrastructure
- Access personal data without consent
- Deploy malware

### Privacy Concerns

**BLE Can Expose:**
- User location (via MAC tracking)
- Personal health data
- Daily routines
- Device ownership
- Relationships (proximity)

---

## References

- [Bluetooth Core Specification](https://www.bluetooth.com/specifications/specs/)
- [GATT Services](https://www.bluetooth.com/specifications/assigned-numbers/)
- [BLE Security Guide](https://www.bluetooth.com/blog/bluetooth-pairing-part-1-pairing-feature-exchange/)
- [NIST SP 800-121](https://csrc.nist.gov/publications/detail/sp/800-121/rev-2/final)

---

**Security Notice:** This information is for authorized security testing only. Unauthorized access to Bluetooth devices is illegal and may result in criminal prosecution.
