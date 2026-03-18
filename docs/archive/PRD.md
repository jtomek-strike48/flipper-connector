# Product Requirements Document: Flipper Zero Connector

**Version:** 1.1
**Date:** 2026-02-25
**Status:** ✅ Week 0 Spike COMPLETE - Ready for Phase 1
**Author:** Claude Code + Jonathan Tomek

---

## Executive Summary

The Flipper Zero Connector is a Strike48 connector that enables AI agents in Prospector Studio to remotely control and leverage a Flipper Zero device for physical security testing. It exposes all Flipper Zero capabilities as discrete tools through the Strike48 SDK, allowing agents to perform RFID/NFC operations, Sub-GHz radio analysis, BadUSB attacks, infrared control, and more—similar to how the Kali Linux connector enables remote penetration testing.

---

## Goals

### Primary Goals
1. **Enable AI-Driven Physical Pentesting**: Allow Prospector Studio agents to autonomously conduct physical security assessments using Flipper Zero
2. **Comprehensive Capability Exposure**: Expose all Flipper Zero features (RFID, NFC, Sub-GHz, BadUSB, IR, GPIO, iButton, U2F) as Strike48 tools
3. **Bidirectional Data Flow**: Support both uploading payloads/assets to Flipper and retrieving captured data for analysis
4. **Application Management**: Enable installing, removing, listing, and auto-discovering applications on the Flipper Zero
5. **Asset Management**: Support uploading/downloading custom assets (Sub-GHz captures, IR remotes, BadUSB scripts, NFC dumps)

### Secondary Goals
- **Extensibility**: Architecture supports future additions (Bluetooth connectivity, firmware updates, multi-device support)
- **Reliability**: Robust error handling, auto-reconnect, and health monitoring
- **Observability**: Comprehensive audit logging for compliance and debugging
- **Developer Experience**: Clean API, well-documented, follows Rust best practices

---

## Non-Goals

### Explicitly Out of Scope
1. **Firmware Updates**: Not in initial version (Phase 2 feature)
2. **Bluetooth Connectivity**: USB only for MVP (Phase 2 feature)
3. **Multi-Device Support**: Single Flipper per connector instance initially
4. **Real-Time Event Streaming**: Simple request/response pattern initially
5. **Authorization Enforcement**: Trust Prospector Studio's authorization system (not enforced at connector level)
6. **Custom Firmware Development**: Connector works with existing firmware (official, Unleashed, Xtreme), no custom firmware creation

---

## User Personas

### Primary: AI Agent in Prospector Studio
- **Needs**: Autonomous access to Flipper Zero capabilities for pentesting engagements
- **Workflow**: Analyze environment → select appropriate tool → execute → analyze results → iterate
- **Expectations**: Reliable execution, clear error messages, structured results

### Secondary: Human Pentester
- **Needs**: Use Prospector Studio to orchestrate Flipper Zero operations during engagements
- **Workflow**: Guide agents with high-level objectives, review results, adjust strategy
- **Expectations**: Audit logs, safety metadata, ability to intervene

---

## Use Cases

### UC1: Physical Access Assessment - RFID Badge Cloning
**Actor:** AI Agent
**Flow:**
1. Agent requests RFID scan via `flipper_rfid_read`
2. Flipper captures badge data and returns to agent
3. Agent analyzes badge type and protocol
4. Agent requests emulation via `flipper_rfid_emulate`
5. Flipper emulates badge for access testing

### UC2: Wireless Attack Surface - Sub-GHz Signal Analysis
**Actor:** AI Agent
**Flow:**
1. Agent requests frequency scan via `flipper_subghz_scan` at 433MHz
2. Flipper captures signals and returns data
3. Agent identifies interesting signals (garage doors, key fobs)
4. Agent requests replay via `flipper_subghz_replay`
5. Results captured and analyzed for vulnerabilities

### UC3: Credential Harvesting - BadUSB Payload Delivery
**Actor:** AI Agent
**Flow:**
1. Agent generates Ducky script for credential harvesting
2. Agent uploads script via `flipper_badusb_upload`
3. Agent verifies upload via `flipper_file_list`
4. Agent executes payload via `flipper_badusb_execute`
5. Agent retrieves captured data via `flipper_file_read`

### UC4: Custom Tool Deployment
**Actor:** Human Pentester
**Flow:**
1. User downloads custom Flipper app from community
2. User uploads app via `flipper_app_install`
3. Connector auto-discovers new app and exposes it as a tool
4. Agent can now use custom tool in engagement

### UC5: Asset Management - IR Remote Database
**Actor:** AI Agent
**Flow:**
1. Agent uploads IR remote database via `flipper_upload_asset`
2. Agent lists available remotes via `flipper_ir_list`
3. Agent tests specific remote via `flipper_ir_send`
4. Results analyzed for environmental control vulnerabilities

---

## Technical Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Prospector Studio                         │
│                      (AI Agents)                             │
└────────────────────────┬────────────────────────────────────┘
                         │ Strike48 Protocol
                         │
┌────────────────────────▼────────────────────────────────────┐
│              Flipper Zero Connector                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  flipper-core (BaseConnector, ToolRegistry)          │   │
│  └────────────┬────────────────────────────┬────────────┘   │
│               │                            │                 │
│  ┌────────────▼──────────┐   ┌────────────▼─────────────┐   │
│  │  flipper-tools        │   │  flipper-protocol        │   │
│  │  (Tool Implementations)│   │  (flipper-rpc wrapper)  │   │
│  └───────────────────────┘   └────────────┬─────────────┘   │
│                                            │                 │
└────────────────────────────────────────────┼─────────────────┘
                                             │ USB Serial
                                             │
                                  ┌──────────▼──────────┐
                                  │   Flipper Zero      │
                                  │    (USB-C)          │
                                  └─────────────────────┘
```

### Crate Structure

```
flipper-connector/
├── Cargo.toml                  # Workspace root
├── PRD.md                      # This document
├── README.md                   # Quick start guide
├── justfile                    # Build commands
├── crates/
│   ├── flipper-core/           # Core connector logic
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── error.rs        # Error types
│   │   │   ├── connector.rs    # BaseConnector impl
│   │   │   ├── registry.rs     # ToolRegistry
│   │   │   ├── traits.rs       # PentestTool trait
│   │   │   └── logging.rs      # Audit logging
│   │   └── tests/              # Integration tests
│   │
│   ├── flipper-protocol/       # Protocol abstraction
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client.rs       # Wrapper around flipper-rpc
│   │   │   ├── device.rs       # Device connection/health
│   │   │   └── cache.rs        # Metadata caching
│   │   └── Cargo.toml
│   │
│   ├── flipper-tools/          # Tool implementations
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── rfid.rs         # RFID tools
│   │   │   ├── nfc.rs          # NFC tools
│   │   │   ├── subghz.rs       # Sub-GHz tools
│   │   │   ├── badusb.rs       # BadUSB tools
│   │   │   ├── infrared.rs     # IR tools
│   │   │   ├── gpio.rs         # GPIO tools
│   │   │   ├── ibutton.rs      # iButton tools
│   │   │   ├── u2f.rs          # U2F tools
│   │   │   ├── apps.rs         # App management
│   │   │   ├── filesystem.rs   # File operations
│   │   │   └── device_info.rs  # Device info/status
│   │   └── Cargo.toml
│   │
│   └── flipper-agent/          # Headless binary
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
│
└── apps/                       # Future: UI if needed
```

### Key Dependencies

```toml
[dependencies]
# Strike48 SDK
strike48-connector = { path = "../../sdk-rs/crates/connector" }

# Flipper Zero Protocol
flipper-rpc = { version = "0.9.4", features = ["full"] }

# Async Runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error Handling
anyhow = "1"
thiserror = "1"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Time
chrono = "0.4"
```

### Connection Management

**Strategy:** Persistent connection with health monitoring

```rust
pub struct FlipperConnection {
    client: FlipperRpcClient,
    health_check_interval: Duration,
    last_health_check: Instant,
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
}

impl FlipperConnection {
    /// Maintains persistent connection, auto-reconnects on failure
    pub async fn ensure_connected(&mut self) -> Result<()>;

    /// Periodic health check via device info query
    pub async fn health_check(&mut self) -> Result<DeviceStatus>;

    /// Graceful disconnect
    pub async fn disconnect(&mut self) -> Result<()>;
}
```

### Caching Strategy

**Metadata cached with TTL:**
- Installed app list (TTL: 5 minutes)
- Device info (TTL: 30 seconds)
- Filesystem directory listings (TTL: 1 minute)

**Always fresh queries:**
- RFID/NFC reads
- Sub-GHz captures
- BadUSB execution status
- File contents

### Auto-Discovery of Custom Tools

```rust
pub async fn discover_apps(client: &FlipperRpcClient) -> Result<Vec<AppInfo>> {
    // 1. List apps in /ext/apps/
    // 2. Parse .fap metadata
    // 3. Generate tool schema dynamically
    // 4. Register in ToolRegistry
}

pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub category: AppCategory,
    pub capabilities: Vec<Capability>,
}
```

---

## Feature Specifications

### Phase 1: Core Functionality (MVP)

#### F1: Device Connection & Management
- **F1.1** Connect to Flipper Zero via USB serial
- **F1.2** Auto-detect serial port (or accept via config)
- **F1.3** Health monitoring (periodic ping)
- **F1.4** Auto-reconnect on disconnect
- **F1.5** Graceful shutdown

**Tools:**
- `flipper_device_info` - Get device info (firmware version, battery, storage)
- `flipper_device_status` - Check connection health

#### F2: Filesystem Operations
- **F2.1** List files/directories
- **F2.2** Read file contents
- **F2.3** Write/upload files
- **F2.4** Delete files/directories
- **F2.5** Create directories

**Tools:**
- `flipper_file_list` - List files in directory
- `flipper_file_read` - Read file contents
- `flipper_file_write` - Upload file
- `flipper_file_delete` - Delete file/directory
- `flipper_dir_create` - Create directory

#### F3: Application Management
- **F3.1** List installed applications
- **F3.2** Install application (.fap file)
- **F3.3** Remove application
- **F3.4** Auto-discover custom apps

**Tools:**
- `flipper_app_list` - List all installed apps
- `flipper_app_install` - Install app from file
- `flipper_app_remove` - Remove app

#### F4: RFID Operations (125kHz)
- **F4.1** Read RFID tag
- **F4.2** Save captured tag
- **F4.3** Emulate saved tag
- **F4.4** List saved tags

**Tools:**
- `flipper_rfid_read` - Read RFID tag
- `flipper_rfid_save` - Save captured tag
- `flipper_rfid_emulate` - Emulate tag
- `flipper_rfid_list` - List saved tags

#### F5: NFC Operations (13.56MHz)
- **F5.1** Read NFC tag/card
- **F5.2** Save captured data
- **F5.3** Emulate NFC tag
- **F5.4** List saved NFC data

**Tools:**
- `flipper_nfc_read` - Read NFC tag/card
- `flipper_nfc_save` - Save captured data
- `flipper_nfc_emulate` - Emulate tag
- `flipper_nfc_list` - List saved NFC data

#### F6: Sub-GHz Operations
- **F6.1** Scan frequency range
- **F6.2** Read/capture signal
- **F6.3** Save captured signal
- **F6.4** Replay/transmit signal
- **F6.5** List saved signals

**Tools:**
- `flipper_subghz_scan` - Scan frequency range
- `flipper_subghz_read` - Capture signal
- `flipper_subghz_save` - Save signal
- `flipper_subghz_replay` - Replay signal
- `flipper_subghz_list` - List saved signals

#### F7: BadUSB Operations
- **F7.1** Upload Ducky script
- **F7.2** Execute script
- **F7.3** List available scripts
- **F7.4** Delete script

**Tools:**
- `flipper_badusb_upload` - Upload Ducky script
- `flipper_badusb_execute` - Execute script
- `flipper_badusb_list` - List scripts
- `flipper_badusb_delete` - Delete script

#### F8: Audit Logging
- **F8.1** Log all tool executions (tool name, params, timestamp, duration, result)
- **F8.2** Structured JSON format
- **F8.3** Include in tool results (for Prospector Studio capture)
- **F8.4** Local log file with rotation

**Log Format:**
```json
{
  "timestamp": "2026-02-25T13:45:22.123Z",
  "tool": "flipper_rfid_read",
  "parameters": {"timeout": 30},
  "duration_ms": 2341,
  "status": "success",
  "result": {"tag_type": "EM4100", "data": "..."},
  "device_id": "flipper-abc123",
  "risk_level": "medium"
}
```

---

### Phase 2: Advanced Features

#### F9: Infrared Operations
- **F9.1** Learn IR signal
- **F9.2** Send IR signal
- **F9.3** Upload IR database
- **F9.4** List IR remotes

**Tools:**
- `flipper_ir_learn` - Learn IR signal
- `flipper_ir_send` - Send IR signal
- `flipper_ir_upload` - Upload IR database
- `flipper_ir_list` - List remotes

#### F10: GPIO Operations
- **F10.1** UART communication
- **F10.2** SPI communication
- **F10.3** I2C communication
- **F10.4** Read GPIO pins

**Tools:**
- `flipper_gpio_uart` - UART operations
- `flipper_gpio_spi` - SPI operations
- `flipper_gpio_i2c` - I2C operations
- `flipper_gpio_read` - Read pin state

#### F11: iButton Operations
- **F11.1** Read iButton key
- **F11.2** Emulate key
- **F11.3** Write key
- **F11.4** List saved keys

**Tools:**
- `flipper_ibutton_read` - Read key
- `flipper_ibutton_emulate` - Emulate key
- `flipper_ibutton_write` - Write key
- `flipper_ibutton_list` - List keys

#### F12: U2F Operations
- **F12.1** Register as security key
- **F12.2** Authenticate

**Tools:**
- `flipper_u2f_register` - Register key
- `flipper_u2f_authenticate` - Authenticate

#### F13: Bluetooth Connectivity
- **F13.1** Bluetooth LE connection
- **F13.2** Auto-pair/bond
- **F13.3** Same feature parity as USB

#### F14: Firmware Management
- **F14.1** Check firmware version
- **F14.2** Upload firmware
- **F14.3** Flash firmware
- **F14.4** Backup/restore

**Tools:**
- `flipper_firmware_check` - Check version
- `flipper_firmware_update` - Update firmware
- `flipper_firmware_backup` - Backup current firmware

---

## Tool Schema Example

Each tool follows this pattern:

```rust
pub struct FlipperRfidReadTool;

impl PentestTool for FlipperRfidReadTool {
    fn name(&self) -> &str {
        "flipper_rfid_read"
    }

    fn description(&self) -> &str {
        "Read RFID tag (125kHz) using Flipper Zero"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: self.name().to_string(),
            description: self.description().to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "timeout": {
                        "type": "integer",
                        "description": "Read timeout in seconds",
                        "default": 30
                    },
                    "save_as": {
                        "type": "string",
                        "description": "Optional filename to save captured tag"
                    }
                },
                "required": []
            }),
            metadata: ToolMetadata {
                risk_level: RiskLevel::Medium,
                requires_device: true,
                category: "RFID".to_string(),
                tags: vec!["physical", "125khz", "access-control"],
            }
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let timeout = params["timeout"].as_u64().unwrap_or(30);
        let save_as = params["save_as"].as_str();

        // Implementation using flipper-protocol
        let mut conn = get_connection().await?;
        let tag_data = conn.rfid_read(Duration::from_secs(timeout)).await?;

        if let Some(filename) = save_as {
            conn.rfid_save(&tag_data, filename).await?;
        }

        Ok(ToolResult {
            status: "success",
            output: serde_json::to_value(tag_data)?,
            duration_ms: /* tracked by execute_timed */,
        })
    }
}
```

---

## Implementation Phases

### Week 0: Validation Sprint (3-5 days) ⚡ **CRITICAL**

**Goal:** De-risk the project by validating `flipper-rpc` crate capabilities

**Why Week 0?**
We're betting heavily on `flipper-rpc` v0.9.4 without knowing if it supports all our requirements. Its features list shows filesystem operations (`fs-*`), but it's unclear if it supports RFID, NFC, Sub-GHz, BadUSB operations. **This spike validates our technical approach before committing to the timeline.**

**🎯 Success Criteria:**
- [ ] Successfully connect to Flipper Zero device via USB
- [ ] Execute basic operations (device info, filesystem)
- [ ] **CRITICAL**: Determine if `flipper-rpc` supports RFID/NFC/Sub-GHz operations
- [ ] Document API ergonomics and limitations
- [ ] Make Go/No-Go decision on using `flipper-rpc`

**📋 Tasks:**

**Day 1-2: Setup & Basic Validation**
- [ ] Create spike workspace (`spike/` directory)
- [ ] Add `flipper-rpc = "0.9.4"` dependency with `full` features
- [ ] Connect Flipper Zero via USB
- [ ] Implement basic connection test
- [ ] Test device info query
- [ ] Test filesystem operations (list, read, write)
- [ ] Document findings

**Day 2-3: Protocol Deep Dive**
- [ ] Review `flipper-rpc` source code on GitHub
- [ ] Identify what operations are exposed
- [ ] **Critical Test**: Attempt RFID read operation
  - If supported by crate: ✅ Great!
  - If not: Can we send custom protobuf messages?
- [ ] Test NFC read (if possible)
- [ ] Test Sub-GHz scan (if possible)
- [ ] Document gaps

**Day 3-4: Decision & Planning**
- [ ] Create findings document with:
  - What `flipper-rpc` provides out-of-box
  - What we need to extend/fork
  - What we need to build from scratch
  - API quality assessment
- [ ] **Go/No-Go Decision Matrix:**
  - ✅ **GO**: `flipper-rpc` supports most operations → Proceed with Phase 1
  - ⚠️ **ADJUST**: `flipper-rpc` supports some operations → Adjust timeline, plan extensions
  - ❌ **PIVOT**: `flipper-rpc` only does filesystem → Use protobuf directly, add 2-3 weeks
- [ ] Update PRD with findings
- [ ] Adjust Phase 1 timeline if needed

**Day 4-5: Proof of Concept**
- [ ] Build minimal PoC connector:
  - Connect to device
  - Implement 2-3 tools (device_info, file_list, rfid_read)
  - Test Strike48 SDK integration
  - Measure performance baseline
- [ ] Document PoC architecture
- [ ] Identify technical risks for Phase 1

**🚦 Exit Criteria:**
1. **Connection verified**: We can reliably connect to Flipper Zero
2. **Capability assessment**: We know exactly what `flipper-rpc` provides
3. **Gaps identified**: We know what we need to build ourselves
4. **Decision made**: Clear path forward (GO/ADJUST/PIVOT)
5. **Timeline validated**: Confidence in Phase 1 estimates

**📊 Deliverables:**
- `spike/` directory with PoC code
- `WEEK0_FINDINGS.md` with detailed assessment
- Updated PRD with timeline adjustments (if needed)
- Confidence to start Phase 1

---

### Phase 1: MVP (Weeks 1-4)
**Goal:** Functional connector with highest-value tools

**Prerequisites:** Week 0 complete with "GO" decision

**Week 1: Foundation**
- [ ] Set up workspace structure
- [ ] Integrate `flipper-rpc` dependency
- [ ] Implement `flipper-protocol` crate (connection wrapper)
- [ ] Implement `flipper-core` (BaseConnector, ToolRegistry)
- [ ] Basic device connection and health checks

**Week 2: Filesystem & App Management**
- [ ] Filesystem operations tools (F2)
- [ ] Application management tools (F3)
- [ ] Auto-discovery of apps
- [ ] Integration tests

**Week 3: Core Pentest Tools**
- [ ] RFID tools (F4)
- [ ] NFC tools (F5)
- [ ] Sub-GHz tools (F6)
- [ ] Integration tests with real hardware

**Week 4: BadUSB & Polish**
- [ ] BadUSB tools (F7)
- [ ] Audit logging (F8)
- [ ] Documentation
- [ ] End-to-end testing with Prospector Studio
- [ ] Docker image

**Deliverables:**
- Functional connector with 30+ tools
- Docker image ready for deployment
- Documentation (README, API docs)
- Integration with Prospector Studio verified

---

### Phase 2: Advanced Features (Weeks 5-8)

**Week 5-6: Additional Categories**
- [ ] Infrared tools (F9)
- [ ] GPIO tools (F10)
- [ ] iButton tools (F11)
- [ ] U2F tools (F12)

**Week 7: Bluetooth Support**
- [ ] Bluetooth LE connection (F13)
- [ ] Pairing/bonding
- [ ] Feature parity testing

**Week 8: Firmware Management**
- [ ] Firmware update tools (F14)
- [ ] Safety checks
- [ ] Backup/restore
- [ ] Comprehensive testing

**Deliverables:**
- Complete feature set (all 8 categories)
- Bluetooth connectivity
- Firmware management
- Updated documentation

---

### Phase 3: Enhancements (Weeks 9-10)

**Potential Additions:**
- [ ] Real-time event streaming
- [ ] Multi-device support
- [ ] Advanced caching strategies
- [ ] Performance optimizations
- [ ] Enhanced error recovery
- [ ] Telemetry/metrics

---

## Success Metrics

### Technical Metrics
- **Tool Execution Success Rate**: >95% for stable operations
- **Connection Uptime**: >99% during active sessions
- **Auto-Reconnect Success**: >90% on first attempt
- **Tool Execution Latency**: <2s for most operations (excluding long captures)
- **Test Coverage**: >80% unit test coverage, >70% integration test coverage

### Product Metrics
- **Tool Adoption**: Agents successfully use >15 different tools in real engagements
- **Custom App Discovery**: Auto-discovery works with >90% of community apps
- **Error Rate**: <5% of tool executions result in unhandled errors
- **Agent Satisfaction**: Agents can complete physical pentest workflows autonomously

### Operational Metrics
- **Docker Image Size**: <500MB
- **Memory Usage**: <200MB during idle, <500MB under load
- **Audit Log Integrity**: 100% of executions logged
- **Documentation Coverage**: All public APIs documented

---

## Dependencies

### External Dependencies
- **Strike48 SDK**: Core connector framework
- **flipper-rpc crate**: Flipper Zero protocol implementation
- **Flipper Zero firmware**: Requires compatible firmware (official, Unleashed, Xtreme)
- **USB permissions**: Requires appropriate udev rules or privileged Docker container

### Hardware Dependencies
- **Flipper Zero device**: Physical hardware required for testing and operation
- **USB-C cable**: For USB connectivity
- **Linux host**: Primary target platform (Docker on Linux)

### Integration Dependencies
- **Prospector Studio**: Must be compatible with Strike48 connector protocol
- **Docker**: For containerized deployment

---

## Risks & Mitigations

### Technical Risks

**R1: flipper-rpc Incompatibility**
- **Risk**: Crate doesn't support all needed operations
- **Likelihood**: Medium
- **Impact**: High
- **Mitigation**: Evaluate crate thoroughly in Week 1, prepare to fork/extend if needed

**R2: Firmware Fragmentation**
- **Risk**: Different firmware versions break compatibility
- **Likelihood**: High
- **Impact**: Medium
- **Mitigation**: Test against official + top 2 community firmwares, document compatibility matrix

**R3: USB Stability Issues**
- **Risk**: Serial connection drops/hangs
- **Likelihood**: Medium
- **Impact**: High
- **Mitigation**: Robust auto-reconnect, health monitoring, timeouts on all operations

**R4: Auto-Discovery Failures**
- **Risk**: Custom apps can't be auto-discovered
- **Likelihood**: Medium
- **Impact**: Low
- **Mitigation**: Fall back to manual registration, document app metadata requirements

### Operational Risks

**R5: USB Permissions in Docker**
- **Risk**: Connector can't access USB device in container
- **Likelihood**: Low
- **Impact**: High
- **Mitigation**: Document privileged mode or device mapping requirements

**R6: Legal/Ethical Misuse**
- **Risk**: Tool used for unauthorized pentesting
- **Likelihood**: Medium
- **Impact**: Critical
- **Mitigation**: Clear disclaimers, rely on Prospector Studio authorization, audit logging

---

## Open Questions

### Resolved in Discussion
1. ✅ USB vs Bluetooth priority? → USB first
2. ✅ What is Prospector Studio? → Strike48 AI agent platform
3. ✅ App management scope? → Apps + assets, no firmware updates initially
4. ✅ Which categories? → All, phased implementation
5. ✅ Protocol approach? → Use flipper-rpc crate
6. ✅ Use cases? → All scenarios, upload/retrieve, custom tools
7. ✅ Architecture? → Single device, persistent connection, caching, auto-discovery
8. ✅ Security model? → Trust Strike48 auth, audit logging, risk metadata

### Outstanding Questions
None at this time. All key decisions finalized.

---

## Appendix A: Flipper Zero Protocol Reference

### Protobuf Messages (via flipper-rpc)
- Storage operations: read, write, delete, mkdir, stat
- Application operations: start, stop, list
- Property operations: get (device info, battery, firmware version)
- System operations: ping, reboot

### Serial Communication
- Baud rate: 230400
- Protocol: Protobuf over serial
- Framing: Length-delimited messages

---

## Appendix B: Tool Categories & Risk Levels

| Category | Tool Count | Risk Level | Priority |
|----------|-----------|------------|----------|
| Device Info | 2 | Low | Phase 1 |
| Filesystem | 5 | Low | Phase 1 |
| App Management | 3 | Medium | Phase 1 |
| RFID | 4 | Medium | Phase 1 |
| NFC | 4 | Medium | Phase 1 |
| Sub-GHz | 5 | High | Phase 1 |
| BadUSB | 4 | High | Phase 1 |
| Infrared | 4 | Low | Phase 2 |
| GPIO | 4 | Medium | Phase 2 |
| iButton | 4 | Medium | Phase 2 |
| U2F | 2 | Low | Phase 2 |
| Firmware | 4 | High | Phase 2 |

**Total: ~45 tools** (Phase 1: ~30 tools, Phase 2: ~15 tools)

---

## Appendix C: Example Agent Workflow

```
Agent: "I need to assess physical access controls at the target facility"

Agent → flipper_device_status
← Connected, battery 87%, firmware v1.4.3

Agent → flipper_rfid_read (timeout: 60)
← Tag detected: EM4100, UID: 0123456789AB

Agent: "Standard EM4100 proximity card detected. Testing for cloning vulnerability."

Agent → flipper_rfid_save (name: "target_badge_001")
← Saved to /ext/rfid/target_badge_001.rfid

Agent → flipper_rfid_emulate (file: "target_badge_001")
← Emulating... (press BACK to stop)

Agent: "Badge successfully cloned and emulated. Physical access vulnerability confirmed."

Agent → Creates pentest report with findings
```

---

## Appendix D: Long-Running Operations & Cancellation

### Challenge
Many Flipper operations require physical interaction and are inherently long-running:
- **RFID read**: "Hold Flipper near tag" - could take 60s or never complete
- **Sub-GHz capture**: "Press button when signal transmits" - timing unknown
- **NFC emulation**: "Emulating... press BACK to stop" - runs until cancelled

### Design Requirements

**1. Mandatory Timeouts**
- ALL tools that involve waiting MUST have `timeout` parameter
- Default timeouts should be reasonable (30-60s for reads, 5-10min for scans)
- Timeout should be configurable by agent

**2. Cancellation Support**
```rust
// Tool: flipper_cancel_operation
// Cancels any currently running operation
pub async fn cancel_operation() -> Result<ToolResult>
```

**3. Operation Status Queries**
```rust
// Tool: flipper_operation_status
// Returns: {status: "idle" | "running", operation: "rfid_read", elapsed_ms: 5420}
pub async fn operation_status() -> Result<ToolResult>
```

**4. Tool Schema Metadata**
```json
{
  "execution_model": "blocking",
  "typical_duration_sec": 30,
  "max_duration_sec": 300,
  "requires_user_interaction": true,
  "cancellable": true
}
```

---

## Appendix E: Error Handling & Agent-Friendly Messages

### Error Taxonomy

```rust
#[derive(Debug, thiserror::Error)]
pub enum FlipperError {
    // Device/Connection Errors
    #[error("Flipper Zero not connected. Check USB connection and ensure device is powered on.")]
    DeviceNotConnected,

    #[error("Device disconnected during operation. Auto-reconnecting... (attempt {attempt}/{max})")]
    DeviceDisconnected { attempt: u32, max: u32 },

    #[error("Connection timeout after {timeout_sec}s. Device may be unresponsive.")]
    ConnectionTimeout { timeout_sec: u64 },

    // Operation Errors
    #[error("Operation timed out after {timeout_sec}s. {context}")]
    OperationTimeout {
        timeout_sec: u64,
        context: String // e.g., "No RFID tag detected. Ensure Flipper is in range."
    },

    #[error("Invalid parameters: {details}")]
    InvalidParameters { details: String },

    #[error("Operation failed: {reason}. Suggestion: {suggestion}")]
    OperationFailed { reason: String, suggestion: String },

    // Compatibility Errors
    #[error("Operation requires firmware v{required}+. Current: v{current}. Please update firmware.")]
    FirmwareIncompatible { required: String, current: String },

    #[error("Operation not supported by current firmware ({firmware_type} v{version})")]
    OperationNotSupported { firmware_type: String, version: String },

    // Concurrency Errors
    #[error("Device busy: {current_operation} in progress. Wait or cancel operation.")]
    DeviceBusy { current_operation: String },

    // Protocol Errors
    #[error("Protocol error: {details}. This may indicate a bug or firmware issue.")]
    ProtocolError { details: String },
}
```

### Error Response Format

```json
{
  "status": "error",
  "error": {
    "type": "OperationTimeout",
    "message": "Operation timed out after 30s. No RFID tag detected. Ensure Flipper is in range.",
    "code": "OPERATION_TIMEOUT",
    "details": {
      "timeout_sec": 30,
      "operation": "rfid_read"
    },
    "suggestion": "Try increasing timeout or verify RFID tag is within range (1-10cm)",
    "recoverable": true
  },
  "duration_ms": 30042
}
```

---

## Appendix F: Testing Strategy & Hardware Requirements

### Three-Tier Testing Approach

#### Tier 1: Unit Tests (No Hardware)
```rust
// Mock flipper-rpc client
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        FlipperClient {
            async fn rfid_read(&self, timeout: Duration) -> Result<RfidData>;
        }
    }

    #[tokio::test]
    async fn test_rfid_read_tool_success() {
        let mut mock_client = MockFlipperClient::new();
        mock_client.expect_rfid_read()
            .returning(|_| Ok(RfidData { ... }));

        let tool = FlipperRfidReadTool::new(mock_client);
        let result = tool.execute(json!({"timeout": 30})).await.unwrap();
        assert_eq!(result.status, "success");
    }
}
```

**Coverage target**: >80%

#### Tier 2: Integration Tests (Mocked Device)
```rust
// Simulate real device responses for full workflow testing
pub struct MockFlipperDevice {
    responses: HashMap<String, Vec<u8>>, // protobuf responses
}

#[tokio::test]
async fn test_full_rfid_workflow() {
    let mock_device = MockFlipperDevice::new()
        .with_rfid_tag("EM4100", "0123456789AB");

    let connector = FlipperConnector::new(mock_device);

    // Test full workflow: read → save → emulate
    let read_result = connector.execute_tool("flipper_rfid_read", params).await?;
    let save_result = connector.execute_tool("flipper_rfid_save", params).await?;
    let emulate_result = connector.execute_tool("flipper_rfid_emulate", params).await?;

    // Verify all succeeded
}
```

**Coverage target**: >70%

#### Tier 3: Hardware E2E Tests (Manual)
```markdown
# Hardware Test Checklist

## Required Equipment
- [ ] Flipper Zero (official firmware v1.3+)
- [ ] Flipper Zero (Unleashed firmware)
- [ ] USB-C cable
- [ ] Test RFID card (EM4100 or HID Prox)
- [ ] Test NFC card (Mifare Classic or NTAG)
- [ ] 433MHz remote (for Sub-GHz)
- [ ] IR remote (for infrared testing)

## Test Procedures
### TP-01: Device Connection
1. Power on Flipper Zero
2. Connect via USB
3. Run: `flipper_device_info`
4. **Expected**: Device info returned within 2s

### TP-02: RFID Read
1. Run: `flipper_rfid_read(timeout=60)`
2. Hold test RFID card near Flipper
3. **Expected**: Tag data captured within 5s

### TP-03: Connection Recovery
1. Start long operation (Sub-GHz scan)
2. Disconnect USB during operation
3. Reconnect USB
4. **Expected**: Auto-reconnect within 5s, operation resumes or fails gracefully

[... more test procedures ...]
```

### CI/CD Strategy
- **Unit tests**: Run on every commit (GitHub Actions)
- **Integration tests**: Run on every PR
- **Hardware tests**: Run manually before release

---

## Appendix G: Concurrency & Operation Queue

### Problem
Flipper Zero can only execute **one operation at a time**. If two agents try to use it simultaneously, we need to handle conflicts gracefully.

### Solution: Operation Queue with Mutex

```rust
pub struct FlipperConnection {
    client: Arc<Mutex<FlipperRpcClient>>,
    current_operation: Arc<RwLock<Option<OperationInfo>>>,
}

pub struct OperationInfo {
    operation: String,
    started_at: Instant,
    cancellation_token: CancellationToken,
}

impl FlipperConnection {
    pub async fn execute_operation<F, T>(&self, op_name: &str, f: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        // Try to acquire lock
        let _guard = self.client.lock().await;

        // Check if operation is already running
        if self.current_operation.read().await.is_some() {
            return Err(FlipperError::DeviceBusy {
                current_operation: self.current_operation.read().await.as_ref().unwrap().operation.clone()
            });
        }

        // Mark operation as running
        *self.current_operation.write().await = Some(OperationInfo {
            operation: op_name.to_string(),
            started_at: Instant::now(),
            cancellation_token: CancellationToken::new(),
        });

        // Execute operation
        let result = f.await;

        // Clear operation
        *self.current_operation.write().await = None;

        result
    }
}
```

### Alternative: Queuing (Phase 2)
```rust
pub struct OperationQueue {
    queue: Arc<Mutex<VecDeque<PendingOperation>>>,
    max_queue_size: usize,
}

// Operations are queued and processed sequentially
// Allows agents to "fire and forget" without getting DeviceBusy errors
```

**Phase 1**: Return `DeviceBusy` error (simple, explicit)
**Phase 2**: Optional queuing mode (more user-friendly)

---

## Appendix H: Configuration Reference

### Configuration File Format

**File**: `config.toml`
**Location**: `/etc/flipper-connector/config.toml` or `./config.toml`

```toml
# Flipper Zero Connector Configuration

[device]
# Serial port path. Use "auto" for auto-detection.
port = "auto"  # or "/dev/ttyACM0"

# Connection timeout in milliseconds
timeout_ms = 5000

# Number of reconnection attempts before giving up
reconnect_attempts = 3

# Time between reconnection attempts (ms)
reconnect_delay_ms = 1000

# Health check interval (seconds). Set to 0 to disable.
health_check_interval_sec = 30

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Audit log file path
audit_file = "/var/log/flipper-connector/audit.jsonl"

# Enable audit log encryption (requires encryption_key)
encrypt_audit_log = false

# Redact sensitive data in logs (show only partial data)
redact_sensitive_data = true

# Log rotation: max file size in MB before rotation
max_log_size_mb = 100

# Number of rotated log files to keep
max_log_files = 10

[cache]
# Time-to-live for cached app list (seconds)
app_list_ttl_sec = 300

# Time-to-live for cached device info (seconds)
device_info_ttl_sec = 30

# Time-to-live for filesystem directory listings (seconds)
fs_cache_ttl_sec = 60

[tools]
# Directory containing custom tool manifests
custom_manifests_dir = "/etc/flipper-connector/custom-tools"

# Enable auto-discovery of apps (Phase 1: list only, Phase 2: expose as tools)
auto_discover_apps = true

# Default timeout for operations (seconds)
default_timeout_sec = 30

# Maximum timeout allowed (seconds)
max_timeout_sec = 600

[security]
# Require TLS for Strike48 SDK connection
require_tls = true

# Path to audit log encryption key (if encrypt_audit_log = true)
# encryption_key = "/etc/flipper-connector/audit.key"

# Data retention policy (days). Logs older than this are deleted.
audit_log_retention_days = 90

[observability]
# Enable Prometheus metrics endpoint
enable_metrics = false

# Metrics endpoint address
metrics_addr = "0.0.0.0:9090"

# Enable health check HTTP endpoint
enable_health_check = true

# Health check endpoint address
health_check_addr = "0.0.0.0:8080"
```

### Environment Variable Overrides

All config values can be overridden with environment variables:

```bash
# Format: FLIPPER_<SECTION>_<KEY>
export FLIPPER_DEVICE_PORT="/dev/ttyACM0"
export FLIPPER_LOGGING_LEVEL="debug"
export FLIPPER_CACHE_APP_LIST_TTL_SEC=600
```

### Validation

Configuration is validated on startup with clear error messages:

```
Error: Invalid configuration
  - device.timeout_ms must be between 1000 and 60000 (got: 100)
  - logging.audit_file parent directory does not exist: /var/log/flipper-connector
  - cache.app_list_ttl_sec must be positive (got: -1)

Fix these errors in /etc/flipper-connector/config.toml and restart.
```

---

## Appendix I: Docker Deployment Guide

### Dockerfile

```dockerfile
FROM rust:1.86-slim as builder

WORKDIR /build
COPY . .

RUN cargo build --release --package flipper-agent

FROM debian:bookworm-slim

# Install required libraries
RUN apt-get update && apt-get install -y \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /build/target/release/flipper-agent /usr/local/bin/

# Create config directory
RUN mkdir -p /etc/flipper-connector /var/log/flipper-connector

# Copy default config
COPY config.toml /etc/flipper-connector/config.toml

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["flipper-agent"]
```

### docker-compose.yml

```yaml
version: '3.8'

services:
  flipper-connector:
    build: .
    image: flipper-connector:latest
    container_name: flipper-connector

    # USB device passthrough
    devices:
      - /dev/ttyACM0:/dev/ttyACM0

    # Alternative: privileged mode (less secure)
    # privileged: true

    volumes:
      - ./config.toml:/etc/flipper-connector/config.toml:ro
      - ./logs:/var/log/flipper-connector
      - ./custom-tools:/etc/flipper-connector/custom-tools:ro

    environment:
      - FLIPPER_LOGGING_LEVEL=info
      - RUST_LOG=flipper_connector=debug

    ports:
      - "8080:8080"  # Health check
      - "9090:9090"  # Metrics (optional)

    restart: unless-stopped

    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

### USB Permissions (Linux)

**Option 1: udev rules (recommended)**

Create `/etc/udev/rules.d/99-flipper-zero.rules`:

```
# Flipper Zero USB device
SUBSYSTEM=="tty", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="5740", MODE="0666", GROUP="plugdev", SYMLINK+="flipper0"
```

Reload rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

**Option 2: Add user to dialout group**
```bash
sudo usermod -a -G dialout $USER
# Log out and back in
```

**Option 3: Privileged container (not recommended for production)**
```yaml
privileged: true
```

### Troubleshooting

**Issue**: Container can't access USB device

```bash
# Check if device is visible on host
ls -l /dev/ttyACM*

# Check device permissions
ls -l /dev/ttyACM0

# Check vendor/product ID
lsusb | grep Flipper

# Test inside container
docker exec -it flipper-connector ls -l /dev/ttyACM0
```

**Issue**: Device keeps disconnecting

- Check USB cable quality (use official cable)
- Check USB power (some ports don't provide enough power)
- Disable USB autosuspend:
  ```bash
  echo 'on' | sudo tee /sys/bus/usb/devices/*/power/control
  ```

---

## Sign-Off

**Approved by:**
- [ ] Product Owner
- [ ] Technical Lead
- [ ] Security Lead
- [ ] Prospector Studio Team

**Next Steps:**
1. ✅ Review and approve PRD (v1.1 with Week 0)
2. 🚀 **Execute Week 0 Spike** (IN PROGRESS)
3. Make Go/No-Go decision based on spike findings
4. Update timeline if needed
5. Begin Phase 1 Week 1
