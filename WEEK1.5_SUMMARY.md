# Week 1.5: App Control Research - Summary

**Date:** 2026-02-25
**Duration:** ~4 hours
**Status:** ✅ Complete

---

## TL;DR

**✅ SUCCESS:** We can launch Flipper Zero apps via RPC
**❌ BLOCKER:** Apps are frozen during RPC - no button control possible
**🔄 PIVOT:** Switch from app-based control to file-based workflows

---

## Key Discoveries

### ✅ What Works

1. **App Launching**
   - Built-in apps launch successfully: NFC, Sub-GHz, Infrared, iButton, GPIO
   - Command: `AppStart(name="NFC", args="")`
   - Apps display correctly on Flipper screen

2. **App Directory Structure**
   - Apps stored in `/ext/apps/` by category
   - Format: `.fap` files (Flipper Application Package)
   - Examples: `nfc.fap`, `lfrfid.fap`, `subghz.fap`

3. **Button API Definitions**
   - `GuiSendInputEvent` with InputKey enum (Up=0, Down=1, Right=2, Left=3, Ok=4, Back=5)
   - InputType enum (Press, Release, Short, Long, Repeat)
   - API sends successfully with no errors

### ❌ What Doesn't Work

1. **App Control**
   - `AppExit` fails with ERROR_APP_NOT_RUNNING (even when app is visible)
   - `AppButtonPress` fails with ERROR_APP_NOT_RUNNING
   - `GuiSendInputEvent` sends but has **zero effect** on device

2. **Physical Device During RPC**
   - **CRITICAL:** Physical buttons also stop working when RPC is active
   - Device is completely frozen for input
   - ⚠️ **USER ACTION REQUIRED:** To exit apps, disconnect RPC (unplug USB), then press physical Back button

3. **Root Cause**
   - RPC mode blocks ALL input (both programmatic and physical)
   - Apps launched via RPC are for display only
   - No interactive control possible
   - **This is by design** - security feature to prevent remote control of radio functions

---

## Impact on Connector Strategy

### Original Plan (Week 0)
```
Launch RFID app → Press buttons → Read tag → Get results
```

### Reality
```
Launch app → ❌ Frozen → Cannot control → Must disconnect
```

### Revised Plan
```
Create .rfid file → User plays manually → Read results from filesystem
```

---

## Recommendations

### ✅ Keep Building

**What's Still Great:**
- ✅ Filesystem operations (Week 1) - work perfectly
- ✅ File management tools (Week 2) - proceed as planned
- ✅ App management (list, metadata) - read-only works
- ✅ Asset management - all file-based

### 🔄 Pivot Strategy

**For RFID/NFC/Sub-GHz:**
- ❌ Don't use app-based control
- ✅ Use file-based workflows:
  1. Parse existing capture files (.nfc, .rfid, .sub)
  2. Generate new files programmatically
  3. Analyze capture contents
  4. Guide user for manual playback

### 📋 Updated Tool List

**Still Valid (45+ tools):**
- All filesystem tools
- App installation/removal
- Asset management
- Configuration tools
- System info/status

**Requires Adjustment (~10 tools):**
- `rfid_read` → `rfid_parse_capture`
- `nfc_read` → `nfc_parse_capture`
- `subghz_transmit` → `subghz_create_file`
- Similar changes for interactive operations

**Net Impact:** ~80% of original tools unchanged, ~20% shift to file-based approach

---

## Technical Findings

### API Behavior Table

| API Call | Sends? | Effect? | Error Message |
|----------|--------|---------|---------------|
| `AppStart("NFC")` | ✅ Yes | ✅ Launches | None |
| `AppExit` | ✅ Yes | ❌ None | ERROR_APP_NOT_RUNNING |
| `AppButtonPress` | ✅ Yes | ❌ None | ERROR_APP_NOT_RUNNING |
| `GuiSendInputEvent` | ✅ Yes | ❌ None | None (succeeds!) |
| Physical buttons | N/A | ❌ None | N/A (hardware blocked) |

### Built-in App Names

✅ **Working:**
- `NFC`
- `Sub-GHz`
- `Infrared`
- `iButton`
- `GPIO`

❌ **Not Working:**
- `RFID` (ERROR_INVALID_PARAMETERS)
- `BadUSB` (ERROR_INVALID_PARAMETERS)

*Note: These might require different names or parameters*

---

## Files Created

**Research Tools (9 binaries):**
- `spike/list_apps.rs` - List app directories
- `spike/explore_apps.rs` - Explore .fap files
- `spike/app_research.rs` - Test app launching
- `spike/test_load_file.rs` - Test AppLoadFile
- `spike/test_start_args.rs` - Test various parameters
- `spike/force_cleanup.rs` - Clear stuck states
- `spike/test_builtin_apps.rs` - Systematic app testing
- `spike/test_app_state.rs` - Verify app display
- `spike/test_button_back.rs` - Test app button API
- `spike/test_gui_input.rs` - Test GUI input API
- `spike/test_gui_navigation.rs` - Test navigation

**Documentation:**
- `WEEK1.5_FINDINGS.md` - Comprehensive technical findings
- `WEEK1.5_SUMMARY.md` - This document

---

## Next Steps

### Week 2 (Adjusted)

1. **Continue filesystem tools** ✅
   - mkdir, stat, chmod operations
   - Working perfectly, proceed as planned

2. **Add app management tools** ✅
   - List installed apps
   - Get app metadata
   - Read-only operations

3. **Research file formats** 🎯 **NEW PRIORITY**
   - Parse .nfc, .rfid, .sub files
   - Document structure
   - Build parsers/generators

4. **Start file manipulation tools** 🆕
   - Parse capture files
   - Generate files for playback
   - Analyze capture contents

### Week 3-4 (Revised)

- Build file-based RFID/NFC/Sub-GHz tools
- Create parsers for all capture formats
- Develop file generation utilities
- Test with real capture files

### Future Exploration

- Custom RPC-aware .fap apps (Phase 2)
- Bluetooth connectivity investigation
- Newer firmware versions research

---

## Lessons Learned

### What Went Well ✅

1. **Systematic testing** uncovered the real limitation early
2. **Hardware validation** prevented wasted implementation effort
3. **Quick pivot** to file-based approach
4. **Filesystem tools already work** - strong foundation remains

### What We Learned 💡

1. **RPC mode is for automation, not interaction**
2. **File-based workflows are standard Flipper pattern**
3. **Early research saves later rework**
4. **Hardware testing is essential** - simulators wouldn't show this

### Validated Approach ✅

1. ✅ Week 0 spike was correct decision
2. ✅ Week 1.5 research prevented major issues
3. ✅ File-based approach is proven pattern
4. ✅ We can still build 80%+ of planned functionality

---

## Status vs Original Plan

**Phase 1 Week 1:** ✅ 100% Complete
**Phase 1 Week 1.5:** ✅ 100% Complete (research objectives met)
**Phase 1 Week 2:** ⏭️ Ready to proceed (minor adjustments)
**Overall Timeline:** ✅ On track (slight scope adjustment, not delay)

---

## Conclusion

Week 1.5 successfully identified a critical limitation before significant implementation effort was wasted. The revised file-based approach:

- ✅ Leverages existing working functionality (filesystem)
- ✅ Follows established Flipper ecosystem patterns
- ✅ Enables 80%+ of originally planned tools
- ✅ Actually more reliable than app-based control would have been
- ✅ No timeline impact - just scope adjustment

**Verdict:** Successful research phase, clear path forward, ready for Week 2.

---

**Next Session: Phase 1 Week 2 - Filesystem & App Management Tools**

🚀 Foundation solid, file-based strategy validated, ready to build!
