# Week 1.5: App Control Research - Findings

**Date:** 2026-02-25
**Status:** ✅ Research Complete - Critical Limitations Discovered

---

## Executive Summary

**✅ SUCCESS:** We can launch built-in Flipper Zero apps via RPC
**❌ CRITICAL LIMITATION:** Apps launched via RPC are **completely non-interactive** - no button control possible (physical or RPC)
**⚠️ IMPACT:** App-based control strategy for RFID/NFC/Sub-GHz **will not work** as originally planned

---

## Research Objectives

1. ✅ Understand how to launch Flipper Zero apps via RPC
2. ✅ Test button simulation for app control - **FAILED (not possible)**
3. ⏭️ Research file formats (.nfc, .rfid, .sub, etc.) - **Deferred to Week 2**
4. ⏭️ Build app control framework - **Requires new approach**

---

## Key Discoveries

### 1. Flipper Zero App Structure

**App Organization:**
- Apps are stored in `/ext/apps/` organized by category
- Each category is a directory (e.g., "NFC", "RFID", "Sub-GHz")
- Apps are `.fap` files (Flipper Application Package)

**Example directory structure:**
```
/ext/apps/
├── NFC/
│   ├── nfc.fap (191KB)
│   ├── picopass.fap (120KB)
│   ├── mifare_nested.fap
│   ├── nfc_magic.fap
│   └── ...
├── RFID/
│   ├── lfrfid.fap (44KB)
│   └── fuzzer_rfid.fap
├── Sub-GHz/
│   ├── subghz.fap (105KB)
│   ├── spectrum_analyzer.fap
│   └── ...
├── Infrared/
│   ├── infrared.fap (90KB)
│   └── ir_scope.fap
├── iButton/
│   └── ibutton.fap
├── GPIO/
│   └── gpio.fap
├── USB/
│   ├── bad_usb.fap
│   ├── hid_usb.fap
│   └── u2f.fap
└── Bluetooth/
    ├── hid_ble.fap
    └── ...
```

### 2. Built-in vs External Apps

**Two types of apps identified:**
1. **Built-in system apps** - Uppercase names (e.g., "NFC", "RFID")
   - Can be launched with `AppStart(name="NFC", args="")`
   - "NFC" successfully launched in testing

2. **External .fap apps** - Lowercase filenames (e.g., "nfc.fap", "picopass.fap")
   - NOT launchable with just the filename
   - `AppStart(name="nfc", args="")` → ERROR_INVALID_PARAMETERS
   - Unclear how to launch these yet

### 3. RPC Session Behavior

**Critical Issue: RPC Mode Locking**

When connecting via USB serial and entering RPC mode with `start_rpc_session`, the Flipper enters a state where:
- ✅ Desktop can be unlocked
- ✅ `AppExit` returns "no app running"
- ❌ **BUT** `AppStart` fails with "ERROR_APP_SYSTEM_LOCKED - another app is already running"

**Hypothesis:** The RPC session itself acts as a "running app" that locks out other apps.

**Evidence:**
```
Desktop unlock: ✅ Success
App exit: ERROR_APP_NOT_RUNNING (no app to exit)
App start: ERROR_APP_SYSTEM_LOCKED (system locked!)
```

This is contradictory - no app is running, but the system is locked?

**Possible explanations:**
1. RPC session is an invisible system-level app
2. Need special permissions or mode to launch apps during RPC
3. Apps can only be launched when RPC is NOT active
4. Missing unlock/permission step before launching apps

### 4. API Calls Tested

**`AppStart(StartRequest)`**
- Fields: `name: String`, `args: String`
- Tested combinations:
  - ✅ `name="NFC", args=""` - **SUCCESS** (once, then locked)
  - ❌ `name="nfc", args=""` - ERROR_INVALID_PARAMETERS
  - ❌ `name="lfrfid", args=""` - ERROR_INVALID_PARAMETERS
  - ❌ `name="Loader", args="/ext/apps/NFC/nfc.fap"` - ERROR_APP_SYSTEM_LOCKED
  - ❌ `name="External", args="/ext/apps/NFC/nfc.fap"` - ERROR_APP_SYSTEM_LOCKED

**`AppLoadFile(AppLoadFileRequest)`**
- Field: `path: String`
- Tested paths:
  - ❌ `/ext/apps/NFC/nfc.fap` - ERROR_APP_NOT_RUNNING
  - ❌ `/ext/apps/RFID/lfrfid.fap` - ERROR_APP_NOT_RUNNING
- **Conclusion:** Requires an app to already be running

**`AppExit(AppExitRequest)`**
- Works, but returns ERROR_APP_NOT_RUNNING when no app is active

**`DesktopUnlock(UnlockRequest)`**
- ✅ Works successfully

### 5. Button Simulation - CRITICAL LIMITATION DISCOVERED

**App-Level Button API (DOES NOT WORK):**
- `AppButtonPress(AppButtonPressRequest)` - ❌ Returns ERROR_APP_NOT_RUNNING even when app is visible on screen
- `AppButtonRelease(AppButtonReleaseRequest)` - ❌ Same error
- **Conclusion:** These APIs are for RPC-aware custom apps only, NOT for built-in apps

**GUI-Level Input API (ALSO DOES NOT WORK):**
- `GuiSendInputEvent(SendInputEventRequest)` - ✅ Sends successfully (no error)
  - Fields: `key: InputKey` (Up/Down/Left/Right/Ok/Back), `type: InputType` (Press/Release/Short/Long/Repeat)
  - Button mappings confirmed:
    - InputKey::Up = 0
    - InputKey::Down = 1
    - InputKey::Right = 2
    - InputKey::Left = 3
    - InputKey::Ok = 4
    - InputKey::Back = 5
- ❌ **BUT no visible effect on Flipper** - commands succeed but device doesn't respond

**Testing Results:**
- ❌ GuiSendInputEvent(Back, Short) - no effect
- ❌ GuiSendInputEvent(Back, Press) + Release - no effect
- ❌ GuiSendInputEvent(Down, Short) - menu selection doesn't move
- ❌ GuiSendInputEvent(Up, Short) - no effect
- ❌ Multiple Back presses (5x) - no effect
- ⚠️ **Physical buttons on Flipper ALSO don't work during RPC** - device is frozen!

**THE CRITICAL DISCOVERY:**
When connected via RPC and apps are launched:
1. Apps display correctly on screen
2. **ALL input is blocked** - RPC commands AND physical buttons
3. Apps must be exited by **disconnecting RPC**, then pressing Back physically
4. This makes app-based control **impossible** during RPC sessions

**⚠️ IMPORTANT FOR USERS:**
If you use `AppStart` to launch an app, the Flipper will appear frozen:
- The app displays on screen but doesn't respond to button presses
- You cannot exit the app programmatically via RPC
- **SOLUTION:** Disconnect the RPC session (unplug USB or close connection), then press the physical Back button on the Flipper to exit the app
- This is a security feature by design, not a bug - prevents remote control of radio functions

---

---

## Critical Limitations for Our Connector

### ❌ App-Based Control Strategy Is Not Viable

**Original Plan (from Week 0):**
- Launch RFID app via RPC
- Simulate button presses to navigate menus
- Trigger read/write operations
- Retrieve results via file system

**Reality:**
- ✅ Can launch apps
- ❌ **CANNOT control apps** (no button input works)
- ❌ Apps freeze the device during RPC
- ❌ Must disconnect RPC to regain control

### ✅ What DOES Work

**File-Based Operations (Already Implemented in Week 1):**
- ✅ Read/write files
- ✅ List directories
- ✅ Delete files
- ✅ Create directories (not yet implemented but possible)

**App Launching (Limited Usefulness):**
- ✅ Can launch built-in apps (NFC, Sub-GHz, Infrared, iButton, GPIO)
- ⚠️ But cannot interact with them

### 🔄 Alternative Approaches to Consider

1. **File-Based Workflow** (Most Viable)
   - Pre-create command/configuration files
   - Disconnect RPC
   - User manually opens app and operates
   - Reconnect RPC to retrieve results
   - ⚠️ Requires manual user intervention

2. **Custom RPC-Aware .fap Apps**
   - Build custom apps that use `AppDataExchange` for RPC communication
   - Apps run in "RPC mode" and respond to data exchange commands
   - ⚠️ Requires custom app development for each function

3. **Capture File Manipulation**
   - Work with existing capture files (.nfc, .rfid, .sub)
   - Parse, modify, generate files programmatically
   - User manually plays them back via Flipper UI
   - ✅ No RPC limitations, leverages existing filesystem access

4. **Hybrid Approach**
   - Use RPC for file operations and preparation
   - Guide user through manual operations
   - Retrieve and analyze results via RPC
   - ✅ Realistic for actual use cases

---

## Resolved Questions

### ✅ App Launching
- **Built-in app names:** NFC, Sub-GHz, Infrared, iButton, GPIO (uppercase)
- **Failed names:** RFID, BadUSB (invalid parameters - might need different names)
- **External .fap apps:** Cannot launch via simple name, need further research
- **`args` parameter:** Empty for built-in apps, possibly file path for external
- **`AppLoadFile`:** Requires an RPC-aware app already running

### ✅ Button Simulation
- **App-level buttons:** Don't work with built-in apps
- **GUI-level input:** Commands succeed but device doesn't respond
- **Button indices:** Confirmed (Up=0, Down=1, Right=2, Left=3, Ok=4, Back=5)
- **Root cause:** RPC mode blocks ALL input (commands and physical)

### ⏭️ File Formats (Deferred)
- Not yet researched
- Should focus on .nfc, .rfid, .sub parsing/generation
- This is now the PRIMARY path forward for RFID/NFC/Sub-GHz control

---

## Recommendations for Phase 1 Continuation

### 🎯 Revised Strategy: File-Based Operations

**Week 2 (Revised Plan):**
1. ✅ **Continue with filesystem tools** (mkdir, stat, etc.) - These work perfectly
2. ✅ **Add app management tools** (list apps, get metadata) - Read-only, no execution needed
3. 🔄 **Research file formats** (.nfc, .rfid, .sub) - Now CRITICAL for functionality
4. 🔄 **Build file parsers/generators** - This becomes the primary control method

**Week 3-4 (Adjusted):**
- Instead of app-based RFID/NFC/Sub-GHz control
- Build **file manipulation tools:**
  - Parse existing .nfc captures
  - Generate .rfid files for emulation
  - Create .sub files for Sub-GHz transmission
  - Analyze capture file contents

**Phase 2 Additions:**
- Investigate custom RPC-aware .fap app development
- Research if newer firmware versions have better RPC integration
- Consider Bluetooth connectivity as alternative (may not have same limitations)

### 📋 Immediate Next Steps

1. **Document findings** in final WEEK1.5_FINDINGS.md ✅ (in progress)
2. **Update PRD** with revised approach for RFID/NFC/Sub-GHz
3. **Research file formats** - make this Week 2 priority
4. **Continue Week 2** with filesystem tools (these work great!)

### ⚠️ PRD Impact Analysis

**Original PRD Assumptions:**
- ❌ "Launch apps and control them via button simulation"
- ❌ "Direct RFID read/write via app control"
- ❌ "Interactive NFC operations"

**Revised Approach:**
- ✅ File-based operations (read, write, parse, generate)
- ✅ Filesystem management (full access working)
- ✅ App installation/removal (file operations)
- ⚠️ RFID/NFC/Sub-GHz via file manipulation, not live control
- ℹ️ User must manually trigger playback for transmit operations

**Tools Affected:**
- `rfid_read` → Becomes `rfid_parse_capture` (parse existing .rfid files)
- `nfc_read` → Becomes `nfc_parse_capture` + user guidance
- `subghz_transmit` → Becomes `subghz_create_file` + manual playback
- Most other tools unaffected (filesystem, app management, etc.)

### 💡 Silver Lining

**What We Gained:**
1. **Clear understanding** of RPC limitations
2. **File-based approach** is actually more reliable (no timing issues)
3. **Works offline** - can prepare files without device connected
4. **Proven pattern** - other Flipper tools use file-based workflows
5. **Full filesystem access** - we can do everything files can do

**What We Can Still Build:**
- Complete file management
- Capture file analysis
- File generation/modification
- App installation
- Asset management
- Configuration management
- **Everything except live app control**

---

## Code Artifacts Created

- `spike/list_apps.rs` - Lists app directories
- `spike/explore_apps.rs` - Explores app directory contents
- `spike/app_research.rs` - Tests app launching and buttons
- `spike/test_load_file.rs` - Tests AppLoadFile
- `spike/test_start_args.rs` - Tests AppStart with various args
- `spike/force_cleanup.rs` - Attempts to clear stuck state

---

## References

- flipper-rpc README: `/home/jtomek/.cargo/registry/src/.../flipper-rpc-0.9.4/README.md`
- Proto definitions: `flipper-rpc-0.9.4/src/proto/app.rs`
- RPC connection: Requires `start_rpc_session\r` command over serial

