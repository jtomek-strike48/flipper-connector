# Flipper Zero .sub File Format

## Overview

The `.sub` file format is a human-readable text format used by Flipper Zero to store Sub-GHz (Sub-Gigahertz) radio signal captures. These files store raw or decoded radio frequency signals from remote controls, key fobs, garage door openers, and other wireless devices operating in the 300-928 MHz range. All `.sub` files are stored in `/ext/subghz/` on the SD card.

## File Structure

All .sub files follow this basic structure:

```
Filetype: Flipper SubGHz <type> file
Version: 1
# Comments start with hash
Frequency: <frequency_in_hz>
Preset: <preset_name>
Protocol: <protocol_name>
# Protocol-specific fields follow
```

## File Types

Sub-GHz files can be either:
1. **RAW** - Raw signal capture (timing data)
2. **Key** - Decoded protocol data

## Common Fields

| Field | Description | Format | Example |
|-------|-------------|--------|---------|
| `Filetype` | File type identifier | String | `Flipper SubGHz Key file` or `Flipper SubGHz RAW file` |
| `Version` | File format version | Integer | `1` |
| `Frequency` | Carrier frequency in Hz | Integer | `433920000` (433.92 MHz) |
| `Preset` | Modulation preset | String | `FuriHalSubGhzPresetOok650Async` |
| `Protocol` | Protocol name (if decoded) | String | `Princeton`, `KeeLoq`, etc. |

## Format 1: Decoded Key Files

When Flipper successfully decodes a known protocol, it creates a Key file.

### Example: Princeton Protocol (Simple Rolling Code)

```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: Princeton
Bit: 24
Key: 00 00 00 00 00 57 45 3A
TE: 402
```

**Princeton Fields:**

| Field | Description | Format |
|-------|-------------|--------|
| `Bit` | Number of bits in transmission | Integer (usually 24) |
| `Key` | Transmitted code/data | Hex bytes |
| `TE` | Time Element (pulse duration in µs) | Integer |

### Example: KeeLoq Protocol (Advanced Rolling Code)

```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: KeeLoq
Bit: 64
Key: 00 00 00 00 00 00 00 00 12 34 56 78 9A BC DE F0
Manufacture: Unknown
```

**KeeLoq Fields:**

| Field | Description | Format |
|-------|-------------|--------|
| `Bit` | Number of bits (64 for KeeLoq) | Integer |
| `Key` | Encrypted rolling code + fixed code | Hex bytes |
| `Manufacture` | Car/device manufacturer | String |

### Example: Gate TX Protocol

```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: GateTX
Bit: 24
Key: 00 00 00 00 00 AB CD EF
```

### Example: Star Line Protocol

```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok270Async
Protocol: Star_Line
Bit: 64
Key: 00 00 00 00 12 34 56 78 9A BC DE F0 11 22 33 44
Manufacture: StarLine
```

## Format 2: RAW Signal Files

When Flipper cannot decode a signal or the user chooses to save as RAW, it stores timing data.

### Example: RAW Capture

```
Filetype: Flipper SubGHz RAW file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: RAW
RAW_Data: 268 -1036 268 -1036 268 -364 904 -364 268 -1036 904 -364 904 -364 268 -1036 904 -364 268 -1036 268 -364 904 -1036 268 -364 268 -1036 268 -364 904 -1036 268 -364 268 -1036 904 -364 268 -1036 904 -364 268 -1036 268 -1036 268 -364 904 -1036 268 -1036 268 -1036 268 -1036 268 -1036 268 -1036 268 -1036 268 -1036 268 -364 904 -364 268 -1036 904 -12036
```

**RAW Fields:**

| Field | Description | Format |
|-------|-------------|--------|
| `Protocol` | Always "RAW" for raw captures | String |
| `RAW_Data` | Timing values in microseconds | Space-separated integers |

**RAW_Data Format:**
- Positive values: signal ON duration (µs)
- Negative values: signal OFF duration (µs)
- Example: `268` = 268µs ON, `-1036` = 1036µs OFF

## Frequency Bands

Common frequencies (in Hz):

| Frequency | MHz | Region/Use |
|-----------|-----|------------|
| 300000000 | 300 | Varies |
| 315000000 | 315 | North America (garage doors, car keys) |
| 318000000 | 318 | Japan |
| 390000000 | 390 | Europe |
| 433920000 | 433.92 | Europe, Asia (most common) |
| 868350000 | 868.35 | Europe |
| 915000000 | 915 | North America, ISM band |
| 928000000 | 928 | North America |

## Modulation Presets

| Preset | Description |
|--------|-------------|
| `FuriHalSubGhzPresetOok270Async` | OOK 270kHz async |
| `FuriHalSubGhzPresetOok650Async` | OOK 650kHz async (most common) |
| `FuriHalSubGhzPreset2FSKDev238Async` | 2-FSK deviation 2.38kHz |
| `FuriHalSubGhzPreset2FSKDev476Async` | 2-FSK deviation 4.76kHz |
| `FuriHalSubGhzPresetMSK99_97KbAsync` | MSK 99.97 kbit/s |

**OOK** = On-Off Keying (amplitude modulation)
**FSK** = Frequency Shift Keying (frequency modulation)
**MSK** = Minimum Shift Keying (continuous phase FSK)

## Supported Protocols

### Static Code Protocols (No Rolling Code)

| Protocol | Typical Use | Bits | Security |
|----------|------------|------|----------|
| Princeton | Generic remotes, doorbells | 24 | Very low |
| PT-2240 | Generic remotes | 24 | Very low |
| Nice FLO | Gate openers | 12-24 | Low |
| CAME | Gate openers | 12-24 | Low |
| Gate TX | Gate openers | 24 | Low |
| Linear | Garage doors | 10 | Very low |
| Chamberlain | Garage doors | 9-10 | Low |

### Dynamic Code Protocols (Rolling Code / Encrypted)

| Protocol | Typical Use | Bits | Security |
|----------|------------|------|----------|
| KeeLoq | Car keys, garage doors | 64 | Medium-High |
| Star Line | Car alarms | 64 | Medium |
| Megacode | Gate/garage systems | 24 | Medium |
| Security+ | Garage doors (LiftMaster) | Variable | Medium |

### Other Protocols

| Protocol | Typical Use | Description |
|----------|------------|-------------|
| Weather Station | Weather sensors | Various temperature/humidity sensors |
| TPMS | Tire pressure sensors | Tire pressure monitoring |
| BFT | Gate/garage systems | Italian gate manufacturer |

## File Locations

- **Storage path**: `/ext/subghz/`
- **Typical files**: Remote controls, garage door openers, car key fobs, gate remotes
- **File naming**: User-defined `.sub` extension

## Reading .sub Files

Files can be read using the Flipper Protocol client:

```rust
let mut client = FlipperClient::new()?;
let content = client.read_file("/ext/subghz/garage_remote.sub").await?;
let text = String::from_utf8(content)?;
```

## Parsing Example

```rust
fn parse_sub_file(content: &str) -> Result<SubGhzSignal> {
    let mut frequency = 0;
    let mut protocol = String::new();
    let mut is_raw = false;

    for line in content.lines() {
        if line.starts_with("Frequency:") {
            frequency = line.split(':').nth(1).unwrap().trim().parse()?;
        } else if line.starts_with("Protocol:") {
            protocol = line.split(':').nth(1).unwrap().trim().to_string();
            is_raw = protocol == "RAW";
        }
    }

    Ok(SubGhzSignal { frequency, protocol, is_raw })
}
```

## Security Notes

### Static Code Systems (Low Security)
- **Easily cloned**: Can be captured and replayed indefinitely
- **No encryption**: Signal is transmitted in cleartext
- **Predictable**: Codes may be sequential or easily guessed
- **Legacy systems**: Common in older devices (pre-1990s)

### Rolling Code Systems (Medium-High Security)
- **KeeLoq encryption**: Uses 64-bit encryption, but has known vulnerabilities
- **One-time use**: Each transmission uses a different code
- **Replay attacks mitigated**: Old codes are rejected
- **Still vulnerable**: Some attacks exist (brute force, cryptanalysis)

### Best Practices
- Flipper can capture and replay static codes (legal for your own devices)
- Rolling code systems are harder to attack (and illegal to defeat)
- Only use on your own devices or with explicit permission
- Check local laws regarding RF transmission and replay

## Signal Analysis

### Reading RAW Data

RAW timing data can be analyzed to understand signal characteristics:

```
268 -1036 268 -1036 268 -364 904 -364
```

- **Short pulse (268µs)** followed by **long gap (1036µs)** = Bit 0
- **Short pulse (268µs)** followed by **short gap (364µs)** = Bit 1
- **Long pulse (904µs)** = May indicate different bit encoding

### Common Encoding Schemes

1. **Pulse Width Modulation (PWM)**
   - Different pulse widths = different bits
   - Example: Short pulse = 0, Long pulse = 1

2. **Pulse Position Modulation (PPM)**
   - Pulse position within time slot = bit value
   - Example: Early pulse = 0, Late pulse = 1

3. **Manchester Encoding**
   - Transition in middle of bit period
   - Low-to-high = 0, High-to-low = 1

## Real-World Examples

### Garage Door Opener (Princeton)
```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 315000000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: Princeton
Bit: 24
Key: 00 00 00 00 00 12 34 56
TE: 350
```
**Use:** Simple garage door remote (North America)
**Security:** Very low, easily cloned

### Car Key Fob (KeeLoq)
```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: KeeLoq
Bit: 64
Key: A1 B2 C3 D4 E5 F6 07 08 12 34 56 78 9A BC DE F0
Manufacture: Unknown
```
**Use:** Car door lock/unlock
**Security:** Medium, rolling code

### Gate Remote (CAME)
```
Filetype: Flipper SubGHz Key file
Version: 1
Frequency: 433920000
Preset: FuriHalSubGhzPresetOok650Async
Protocol: CAME
Bit: 12
Key: 00 00 00 00 00 AB C0
```
**Use:** Property gate opener
**Security:** Low, static code

## Technical Details

### OOK (On-Off Keying)
- Simplest form of amplitude modulation
- Carrier is either ON (1) or OFF (0)
- Very common in simple remote controls
- Easy to implement, low power consumption

### FSK (Frequency Shift Keying)
- Two different frequencies represent 0 and 1
- More resistant to interference than OOK
- Used in more sophisticated systems
- Better range and reliability

### Time Element (TE)
- Base timing unit for the protocol
- All other timings are multiples of TE
- Example: TE=400µs, short pulse=1×TE, long pulse=3×TE

## Capture Guidelines

1. **Frequency Selection**: Know your device's frequency
   - Check device label or manual
   - Common: 315 MHz (US), 433.92 MHz (EU)

2. **Signal Strength**: Get close to source
   - 5-10 cm for best capture
   - Reduce interference from other sources

3. **Multiple Captures**: For rolling codes
   - Capture 2-3 times for comparison
   - Note that rolling codes change each time

4. **RAW vs Decoded**: When to use each
   - Decoded: Cleaner, smaller files, specific protocol
   - RAW: Universal, works for unknown protocols, larger files

## Version History

- **Version 1**: Current format

## Legal and Ethical Considerations

- **Own devices only**: Only capture/replay your own remotes
- **Local laws**: Check RF transmission regulations in your region
- **Rolling codes**: Defeating security systems may be illegal
- **Responsible use**: Don't interfere with others' devices
- **Testing only**: Use in controlled environments

## References

- Flipper Zero Sub-GHz documentation
- CC1101 transceiver datasheet
- KeeLoq encryption specification
- ISM band regulations (FCC Part 15, ETSI EN 300 220)
- Sub-GHz protocol specifications
