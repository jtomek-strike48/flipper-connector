# GPIO Operations - Flipper Zero

Complete guide to GPIO pin control, UART, I2C, and SPI operations for hardware debugging and testing.

## Overview

Flipper Zero's GPIO capabilities enable direct hardware interaction, protocol analysis, and custom hardware debugging. Essential for IoT security testing and hardware reverse engineering.

---

## GPIO Pin Mapping

### Available Pins

| Pin # | Name | Functions | Voltage | Notes |
|-------|------|-----------|---------|-------|
| 1 | +5V | Power | 5V | From USB, max 1A |
| 2 | GND | Ground | 0V | Common ground |
| 6 | PC3/MOSI | SPI MOSI, GPIO | 3.3V | 5V tolerant |
| 7 | PC2/MISO | SPI MISO, GPIO | 3.3V | 5V tolerant |
| 8 | PB3/SCK | SPI SCK, GPIO | 3.3V | 5V tolerant |
| 9 | +3.3V | Power | 3.3V | Max 500mA |
| 11 | GND | Ground | 0V | - |
| 13 | PC1/TX | UART TX, GPIO | 3.3V | 5V tolerant |
| 14 | PC0/RX | UART RX, GPIO | 3.3V | 5V tolerant |
| 15 | PB2/SDA | I2C SDA, GPIO | 3.3V | 5V tolerant with pullup |
| 16 | PA7/SCL | I2C SCL, GPIO | 3.3V | 5V tolerant with pullup |
| 17 | PA6 | GPIO | 3.3V | General purpose |
| 18 | GND | Ground | 0V | - |

---

## Connector Tools

### Tool: `flipper_gpio_set`

**Set GPIO pin state:**
```json
{
  "tool": "flipper_gpio_set",
  "params": {
    "pin": "PA7",
    "mode": "output_push_pull",
    "state": "high"
  }
}
```

**Modes:**
- `input` - High impedance input
- `output_push_pull` - Standard digital output
- `output_open_drain` - Open-drain output
- `analog` - Analog input (ADC)

### Tool: `flipper_gpio_read`

**Read GPIO pin state:**
```json
{
  "tool": "flipper_gpio_read",
  "params": {
    "pin": "PA7"
  }
}
```

---

## UART Operations

### Tool: `flipper_uart_send`

**Send data via UART:**
```json
{
  "tool": "flipper_uart_send",
  "params": {
    "data": "Hello World",
    "baud_rate": 115200
  }
}
```

**Common Baud Rates:**
- 9600 - Standard low-speed
- 19200 - Low-speed
- 38400 - Medium-speed
- 57600 - Medium-speed
- 115200 - High-speed (most common)
- 230400 - Very high-speed
- 921600 - Maximum

**Pin Connections:**
- **TX (Pin 13)** → Target device RX
- **RX (Pin 14)** → Target device TX
- **GND (Pin 11)** → Target device GND

### UART Testing Scenarios

**1. Serial Console Access**
```json
{
  "tool": "flipper_uart_send",
  "params": {
    "data": "root\n",
    "baud_rate": 115200
  }
}
```

**2. Arduino/ESP32 Debugging**
```json
{
  "tool": "flipper_uart_send",
  "params": {
    "data": "AT+GMR\r\n",
    "baud_rate": 115200
  }
}
```

---

## I2C Operations

### Tool: `flipper_i2c_scan`

**Scan I2C bus for devices:**
```json
{
  "tool": "flipper_i2c_scan",
  "params": {
    "start_address": 8,
    "end_address": 119
  }
}
```

**Pin Connections:**
- **SCL (Pin 16)** → I2C clock
- **SDA (Pin 15)** → I2C data
- **GND (Pin 11)** → Common ground
- **3.3V (Pin 9)** → Power (if needed)

**Common I2C Addresses:**
- `0x3C/0x3D` - OLED displays
- `0x50-0x57` - EEPROMs
- `0x68` - Real-time clocks (RTC)
- `0x76/0x77` - BME280/BMP280 sensors
- `0xA0-0xAF` - Various EEPROMs

### I2C Security Testing

**1. EEPROM Dumping**
```json
{
  "tool": "flipper_i2c_scan",
  "params": {
    "start_address": 80,
    "end_address": 87
  }
}
```

**2. Sensor Manipulation**
- Read environmental sensors
- Modify sensor calibration
- Test tamper detection

---

## SPI Operations

### Tool: `flipper_spi_exchange`

**Exchange data via SPI:**
```json
{
  "tool": "flipper_spi_exchange",
  "params": {
    "data": "01 02 03 FF",
    "cs_pin": "PA7",
    "clock_speed": 1000000
  }
}
```

**Pin Connections:**
- **MOSI (Pin 6)** → SPI MOSI (Master Out)
- **MISO (Pin 7)** → SPI MISO (Master In)
- **SCK (Pin 8)** → SPI clock
- **CS** → Chip select (user-selected GPIO)
- **GND (Pin 11)** → Common ground

**Common Clock Speeds:**
- 100 kHz - Very low-speed
- 1 MHz - Low-speed
- 4 MHz - Medium-speed
- 8 MHz - High-speed
- 16 MHz - Very high-speed

### SPI Security Testing

**1. Flash Memory Dumping**
```json
{
  "tool": "flipper_spi_exchange",
  "params": {
    "data": "03 00 00 00",
    "cs_pin": "PB3",
    "clock_speed": 1000000
  }
}
```

**2. Smart Card Analysis**
- Read SPI-based smart cards
- Analyze communication protocols
- Test encryption implementations

---

## Hardware Security Testing Use Cases

### 1. JTAG/SWD Access

**Scenario:** Debug port enumeration

- Connect to test points
- Scan for debug protocols
- Identify JTAG/SWD interfaces
- Document exposed debug ports

### 2. Serial Console Discovery

**Scenario:** Finding hidden UARTs

**Workflow:**
1. Identify TX/RX candidates
2. Test common baud rates
3. Monitor boot sequences
4. Document console access

### 3. I2C EEPROM Extraction

**Scenario:** Firmware/config extraction

**Workflow:**
1. Scan I2C bus: `flipper_i2c_scan`
2. Identify EEPROM addresses
3. Read memory contents
4. Analyze extracted data

### 4. SPI Flash Dumping

**Scenario:** Firmware extraction

**Workflow:**
1. Identify SPI flash pins
2. Connect Flipper Zero
3. Read flash contents
4. Analyze firmware

---

## Security Considerations

### GPIO Vulnerabilities

1. **Exposed Debug Ports** - UART/JTAG often unprotected
2. **No Authentication** - Hardware access = full control
3. **Bus Snooping** - Can monitor I2C/SPI traffic
4. **Firmware Extraction** - Via SPI/I2C memory
5. **Protocol Injection** - Manipulate hardware communications

### Recommendations

**For Penetration Testers:**
- Always document hardware interfaces
- Test for debug port access
- Extract firmware when possible
- Map all GPIO functionality
- Verify security protections

**For Defenders:**
- Disable debug ports in production
- Use secure boot
- Encrypt sensitive data on external memory
- Implement bus encryption (I2C/SPI)
- Use tamper detection
- Apply conformal coating to PCBs

---

## Common Hardware Targets

### IoT Devices
- Smart home hubs
- Security cameras
- Door locks
- Environmental sensors

### Embedded Systems
- Industrial controllers
- Medical devices
- Automotive ECUs
- Building automation

### Consumer Electronics
- Routers
- Smart speakers
- Fitness trackers
- Gaming consoles

---

## Hardware Tools

### Required Equipment

- **Flipper Zero** with GPIO expansion
- **Jumper wires** (male-to-male, male-to-female)
- **Logic analyzer** (optional, for protocol analysis)
- **Multimeter** for voltage/continuity testing
- **Magnifying glass/microscope** for trace identification

### Safety Precautions

⚠️ **WARNING:**
- Always verify voltage levels before connecting
- Use level shifters for 5V ↔ 3.3V interfaces
- Check pin functions before applying voltage
- Disconnect power before making connections
- Use ESD protection

---

## Troubleshooting

### No Response from UART

**Checklist:**
- TX/RX crossed correctly (TX→RX, RX→TX)
- Correct baud rate
- Correct voltage level (3.3V vs 5V)
- Common ground connected
- Device powered on

### I2C Not Detecting Devices

**Solutions:**
- Check pull-up resistors (4.7kΩ typical)
- Verify power supply voltage
- Reduce clock speed
- Check SDA/SCL connections
- Verify device is powered

### SPI Communication Fails

**Solutions:**
- Verify clock polarity/phase settings
- Check chip select (CS) pin
- Reduce clock speed
- Verify all connections (MOSI, MISO, SCK, CS, GND)
- Check device datasheet for timing requirements

---

**Security Notice:** Only access hardware you own or have authorization to test. Unauthorized hardware access may void warranties or violate laws.
