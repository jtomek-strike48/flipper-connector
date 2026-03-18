# ⚠️ IMPORTANT: App Launching Behavior

## If You Use `AppStart` - READ THIS

### What Happens

When you launch an app via RPC using `AppStart`:

```rust
client.start_app("NFC").await?;
```

1. ✅ **App launches successfully** - you'll see it on the Flipper screen
2. ❌ **Flipper becomes unresponsive** - physical buttons stop working
3. ❌ **Cannot exit programmatically** - `AppExit` returns ERROR_APP_NOT_RUNNING
4. ❌ **Cannot send button commands** - All button APIs fail

### How to Exit Apps

**You MUST manually intervene:**

1. **Disconnect RPC session**
   - Unplug USB cable, OR
   - Close the RPC connection in your code

2. **Press physical Back button**
   - Left side button on the Flipper
   - Press until you return to desktop

3. **Reconnect if needed**
   - Plug USB back in
   - Reestablish RPC connection

### Example Code Pattern

```rust
// Launch app
client.start_app("NFC").await?;

println!("⚠️  App is now running on Flipper");
println!("⚠️  Device is frozen - buttons won't work");
println!("⚠️  To exit: disconnect USB, press Back button");

// Drop connection to allow manual exit
drop(client);

println!("Waiting for user to exit app...");
println!("Press Enter after you've pressed Back on the Flipper...");
std::io::stdin().read_line(&mut String::new())?;

// Reconnect
let mut client = FlipperClient::new()?;
```

### Why Does This Happen?

**This is BY DESIGN, not a bug!**

Flipper Zero was designed with security in mind:
- Prevents malicious remote control of radio functions (RFID, NFC, Sub-GHz)
- Requires physical user interaction for transmit operations
- RPC mode is for file management and automation, NOT interactive app control

### What Should You Do Instead?

**Use file-based workflows:**

```rust
// ✅ GOOD: Generate files, user plays them manually
client.write_file("/ext/nfc/my_card.nfc", nfc_data).await?;
println!("File created! User can now play it via Flipper UI");

// ❌ BAD: Try to launch and control app
client.start_app("NFC").await?;  // App launches but you're stuck!
```

### Apps That Exhibit This Behavior

**Confirmed affected:**
- ✅ NFC
- ✅ Sub-GHz
- ✅ Infrared
- ✅ iButton
- ✅ GPIO

**Likely all built-in apps are affected** - this is system-wide behavior.

### Alternative: Custom RPC-Aware Apps

If you MUST have programmatic app control:
- Build custom `.fap` applications
- Implement `AppDataExchange` for RPC communication
- Your app runs in "RPC mode" and responds to commands
- Requires significant development effort

### For Connector Users

**When using flipper-connector tools:**

If a tool uses `AppStart` (rare, most don't):
- Tool will warn you about manual exit requirement
- Follow prompts to disconnect and press Back
- This is expected behavior, not a bug

**Most tools use file operations instead:**
- No app launching required
- No manual intervention needed
- Works completely via RPC
- This is the recommended approach

---

## Summary

| Action | Works via RPC? | Notes |
|--------|----------------|-------|
| Launch app | ✅ Yes | App displays on screen |
| Exit app | ❌ No | Must disconnect + press Back |
| Button control | ❌ No | All button APIs fail |
| File operations | ✅ Yes | Fully supported |
| App management | ✅ Yes | Install/remove via files |

**Bottom line:** Don't rely on `AppStart` unless you're prepared for manual user intervention. Use file-based workflows instead.

---

**Reference:** See WEEK1.5_FINDINGS.md for full technical details and testing results.
