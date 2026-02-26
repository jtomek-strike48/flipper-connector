# Audit Logging - Flipper Zero Connector

## Overview

The Flipper Zero connector includes comprehensive audit logging for compliance, security monitoring, and debugging. All tool executions, connection events, and errors are logged in structured JSON format.

## Features

- **Comprehensive Coverage:** Logs all tool executions with full context
- **Structured Format:** JSON Lines format for easy parsing and analysis
- **Sensitive Data Protection:** Automatic sanitization of passwords, keys, tokens
- **Flexible Output:** File-based or stdout logging
- **Configurable:** Control what gets logged and how
- **Performance:** Minimal overhead with async logging

---

## Configuration

### Basic Configuration

```rust
use flipper_core::audit::AuditConfig;
use std::path::PathBuf;

let config = AuditConfig {
    enabled: true,
    output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl")),
    log_success: true,
    log_failures: true,
    sanitize_data: true,
    include_parameters: true,
    include_results: true,
};
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | bool | true | Enable audit logging |
| `output_path` | Option<PathBuf> | None | Log file path (None = stdout) |
| `log_success` | bool | true | Log successful operations |
| `log_failures` | bool | true | Log failed operations |
| `sanitize_data` | bool | true | Redact sensitive data |
| `include_parameters` | bool | true | Include request parameters |
| `include_results` | bool | true | Include result data |

### Creating Connector with Audit Logging

```rust
use flipper_core::connector::FlipperConnector;
use flipper_core::audit::AuditConfig;
use flipper_tools::create_tool_registry;
use std::path::PathBuf;

let registry = create_tool_registry();

let audit_config = AuditConfig {
    enabled: true,
    output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl")),
    ..Default::default()
};

let connector = FlipperConnector::with_audit_config(registry, Some(audit_config));
```

### Disabling Audit Logging

```rust
// Option 1: Use default constructor (no audit logging)
let connector = FlipperConnector::new(registry);

// Option 2: Explicitly disable
let audit_config = AuditConfig {
    enabled: false,
    ..Default::default()
};
let connector = FlipperConnector::with_audit_config(registry, Some(audit_config));
```

---

## Log Format

### JSON Lines Format

Each log entry is a single JSON object on one line (JSON Lines format):

```jsonl
{"event_id":"550e8400-e29b-41d4-a716-446655440000","timestamp":"2026-02-25T21:30:45.123Z","event_type":"tool_execution","tool_name":"flipper_nfc_read","parameters":{"path":"/ext/nfc/badge.nfc"},"success":true,"duration_ms":342,"result":{"device_type":"NTAG203","uid":"04 AA BB CC"},"context":{"connector_version":"0.1.0"}}
{"event_id":"550e8400-e29b-41d4-a716-446655440001","timestamp":"2026-02-25T21:31:12.456Z","event_type":"tool_execution","tool_name":"flipper_badusb_upload","parameters":{"filename":"test","script":"DELAY 1000\nSTRING test"},"success":true,"duration_ms":125,"context":{"connector_version":"0.1.0"}}
```

### Event Structure

```json
{
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2026-02-25T21:30:45.123Z",
  "event_type": "tool_execution",
  "tool_name": "flipper_nfc_read",
  "parameters": {
    "path": "/ext/nfc/badge.nfc"
  },
  "success": true,
  "duration_ms": 342,
  "result": {
    "device_type": "NTAG203",
    "uid": "04 AA BB CC DD EE FF"
  },
  "context": {
    "user_id": null,
    "session_id": null,
    "source_ip": null,
    "device_serial": null,
    "connector_version": "0.1.0"
  },
  "metadata": null
}
```

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `event_id` | string (UUID) | Unique event identifier |
| `timestamp` | string (RFC3339) | Event timestamp in ISO 8601 format |
| `event_type` | string | Event type (see Event Types below) |
| `tool_name` | string? | Tool that was executed (if applicable) |
| `parameters` | object? | Tool parameters (sanitized) |
| `success` | bool? | Whether operation succeeded |
| `duration_ms` | number? | Execution duration in milliseconds |
| `error` | string? | Error message if operation failed |
| `result` | object? | Tool result data (sanitized) |
| `context` | object? | Execution context (user, session, device) |
| `metadata` | object? | Additional metadata |

### Event Types

| Event Type | Description |
|------------|-------------|
| `tool_execution` | Tool was executed |
| `connection` | Connection established |
| `disconnection` | Connection closed |
| `error` | Error occurred |
| `connector_start` | Connector started |
| `connector_stop` | Connector stopped |

---

## Sensitive Data Sanitization

### Automatic Redaction

The audit logger automatically redacts sensitive fields containing:
- `password`
- `secret`
- `key`
- `token`
- `auth`

**Example:**

**Original:**
```json
{
  "username": "admin",
  "password": "secret123",
  "api_key": "sk-1234567890"
}
```

**Sanitized:**
```json
{
  "username": "admin",
  "password": "[REDACTED]",
  "api_key": "[REDACTED]"
}
```

### Disabling Sanitization

```rust
let audit_config = AuditConfig {
    sanitize_data: false,  // ⚠️ Use with caution!
    ..Default::default()
};
```

**⚠️ Warning:** Only disable sanitization in isolated testing environments. Never disable in production.

---

## Usage Examples

### Example 1: File-Based Logging

```rust
use flipper_core::audit::AuditConfig;
use std::path::PathBuf;

let audit_config = AuditConfig {
    enabled: true,
    output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl")),
    log_success: true,
    log_failures: true,
    sanitize_data: true,
    include_parameters: true,
    include_results: true,
};

let connector = FlipperConnector::with_audit_config(registry, Some(audit_config));
```

**Log file content:**
```jsonl
{"event_id":"...","timestamp":"2026-02-25T21:30:45.123Z","event_type":"tool_execution","tool_name":"flipper_device_info",...}
{"event_id":"...","timestamp":"2026-02-25T21:31:12.456Z","event_type":"tool_execution","tool_name":"flipper_nfc_read",...}
```

### Example 2: Stdout Logging

```rust
let audit_config = AuditConfig {
    enabled: true,
    output_path: None,  // Log to stdout
    ..Default::default()
};

let connector = FlipperConnector::with_audit_config(registry, Some(audit_config));
```

### Example 3: Failures Only

```rust
let audit_config = AuditConfig {
    enabled: true,
    output_path: Some(PathBuf::from("/var/log/flipper/failures.jsonl")),
    log_success: false,  // Don't log successful operations
    log_failures: true,   // Only log failures
    ..Default::default()
};
```

### Example 4: Minimal Logging

```rust
let audit_config = AuditConfig {
    enabled: true,
    output_path: Some(PathBuf::from("/var/log/flipper/minimal.jsonl")),
    log_success: true,
    log_failures: true,
    sanitize_data: true,
    include_parameters: false,  // Don't log parameters
    include_results: false,      // Don't log results
};
```

**Output:**
```json
{
  "event_id": "...",
  "timestamp": "2026-02-25T21:30:45.123Z",
  "event_type": "tool_execution",
  "tool_name": "flipper_nfc_read",
  "success": true,
  "duration_ms": 342
}
```

---

## Log Analysis

### Using jq

**Count total events:**
```bash
wc -l /var/log/flipper/audit.jsonl
```

**Count by event type:**
```bash
jq -r '.event_type' audit.jsonl | sort | uniq -c
```

**Find failed operations:**
```bash
jq 'select(.success == false)' audit.jsonl
```

**Find slow operations (> 1 second):**
```bash
jq 'select(.duration_ms > 1000)' audit.jsonl
```

**Count by tool:**
```bash
jq -r '.tool_name' audit.jsonl | sort | uniq -c | sort -nr
```

**Extract errors:**
```bash
jq 'select(.error != null) | {timestamp, tool_name, error}' audit.jsonl
```

**Average duration by tool:**
```bash
jq -s 'group_by(.tool_name) | map({tool: .[0].tool_name, avg_ms: (map(.duration_ms) | add / length)})' audit.jsonl
```

### Using grep

**Find specific tool executions:**
```bash
grep '"tool_name":"flipper_nfc_read"' audit.jsonl
```

**Find today's logs:**
```bash
grep "$(date +%Y-%m-%d)" audit.jsonl
```

**Find errors:**
```bash
grep '"success":false' audit.jsonl
```

### Using Python

```python
import json

# Read audit log
with open('/var/log/flipper/audit.jsonl', 'r') as f:
    events = [json.loads(line) for line in f]

# Count by tool
from collections import Counter
tool_counts = Counter(e.get('tool_name') for e in events if e.get('tool_name'))
print(tool_counts.most_common(10))

# Find errors
errors = [e for e in events if not e.get('success', True)]
print(f"Found {len(errors)} errors")

# Calculate average duration
durations = [e.get('duration_ms', 0) for e in events if e.get('duration_ms')]
avg_duration = sum(durations) / len(durations) if durations else 0
print(f"Average duration: {avg_duration:.2f}ms")
```

---

## Compliance & Security

### Compliance Benefits

**1. Audit Trail**
- Complete record of all operations
- Timestamp and user context for every action
- Immutable log format (JSON Lines)

**2. Security Monitoring**
- Detect unauthorized access attempts
- Track suspicious patterns
- Alert on failed operations

**3. Forensics**
- Investigate security incidents
- Trace tool execution chains
- Identify root causes

**4. Compliance Requirements**
- HIPAA, SOX, PCI-DSS compliance
- Regulatory audit support
- Evidence preservation

### Log Retention

**Recommended Retention Periods:**
- **Active logs:** 90 days minimum
- **Archived logs:** 1-7 years (depending on compliance requirements)
- **Security incidents:** Indefinite retention

**Rotation Strategy:**
```bash
# Rotate daily
/var/log/flipper/audit.jsonl -> audit-2026-02-25.jsonl

# Compress old logs
gzip /var/log/flipper/audit-*.jsonl

# Archive after 90 days
mv audit-2025-*.jsonl.gz /archive/flipper/
```

### Access Control

**Recommended Permissions:**
```bash
# Log directory
chmod 750 /var/log/flipper/
chown flipper:audit /var/log/flipper/

# Log files
chmod 640 /var/log/flipper/audit.jsonl
chown flipper:audit /var/log/flipper/audit.jsonl
```

**Read access should be limited to:**
- Connector service account
- Security team
- Compliance auditors
- System administrators

---

## Performance Impact

### Overhead

Audit logging is designed to be lightweight:
- **Write latency:** < 1ms per event (file I/O)
- **Memory overhead:** Minimal (buffered writes)
- **CPU overhead:** < 0.1% (async logging)

### Optimization Tips

**1. Use file output instead of stdout**
- Faster writes
- Better buffering
- Log rotation support

**2. Disable unnecessary fields**
```rust
let audit_config = AuditConfig {
    include_parameters: false,  // Smaller logs
    include_results: false,      // Faster writes
    ..Default::default()
};
```

**3. Filter by success/failure**
```rust
let audit_config = AuditConfig {
    log_success: false,  // Only log failures
    log_failures: true,
    ..Default::default()
};
```

**4. Use log rotation**
- Prevents single large file
- Improves write performance
- Easier to manage

---

## Troubleshooting

### Logs Not Appearing

**Check 1: Audit logging enabled**
```rust
let config = AuditConfig {
    enabled: true,  // Must be true
    ..Default::default()
};
```

**Check 2: Output path valid**
```rust
// Ensure directory exists
std::fs::create_dir_all("/var/log/flipper")?;

let config = AuditConfig {
    output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl")),
    ..Default::default()
};
```

**Check 3: File permissions**
```bash
# Check if writable
touch /var/log/flipper/audit.jsonl
ls -l /var/log/flipper/audit.jsonl
```

**Check 4: Disk space**
```bash
df -h /var/log/flipper/
```

### Incomplete Log Entries

**Issue:** Fields missing from log entries

**Solution:** Check configuration
```rust
let config = AuditConfig {
    include_parameters: true,   // Include parameters
    include_results: true,       // Include results
    ..Default::default()
};
```

### Performance Issues

**Issue:** Audit logging slowing down operations

**Solutions:**
1. Use file output instead of stdout
2. Disable result logging for large responses
3. Use log rotation to prevent large files
4. Check disk I/O performance

---

## Best Practices

### 1. Always Enable in Production

```rust
let audit_config = AuditConfig {
    enabled: true,  // Always on in production
    output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl")),
    sanitize_data: true,  // Always sanitize
    ..Default::default()
};
```

### 2. Use File-Based Logging

File-based logging is more reliable and performant than stdout:
```rust
output_path: Some(PathBuf::from("/var/log/flipper/audit.jsonl"))
```

### 3. Implement Log Rotation

```bash
# logrotate configuration
/var/log/flipper/audit.jsonl {
    daily
    rotate 90
    compress
    delaycompress
    missingok
    notifempty
    create 640 flipper audit
}
```

### 4. Monitor Log Files

```bash
# Watch for errors
tail -f /var/log/flipper/audit.jsonl | jq 'select(.success == false)'

# Alert on failures
watch -n 60 'jq "select(.success == false)" /var/log/flipper/audit.jsonl | tail -10'
```

### 5. Regular Analysis

```bash
# Weekly summary
jq -s 'group_by(.tool_name) | map({tool: .[0].tool_name, count: length, failures: map(select(.success == false)) | length})' audit.jsonl
```

### 6. Secure Log Access

```bash
# Restrict access
chmod 640 /var/log/flipper/audit.jsonl
chown flipper:audit /var/log/flipper/audit.jsonl

# Audit log access
auditctl -w /var/log/flipper/audit.jsonl -p war -k flipper_audit_access
```

---

## Integration Examples

### Elasticsearch

```python
from elasticsearch import Elasticsearch
import json

es = Elasticsearch(['localhost:9200'])

with open('/var/log/flipper/audit.jsonl', 'r') as f:
    for line in f:
        event = json.loads(line)
        es.index(index='flipper-audit', body=event)
```

### Splunk

```bash
# Add data input
[monitor:///var/log/flipper/audit.jsonl]
sourcetype = _json
index = flipper

# Query
index=flipper success=false | stats count by tool_name
```

### Grafana Loki

```yaml
# promtail configuration
- job_name: flipper-audit
  static_configs:
    - targets:
        - localhost
      labels:
        job: flipper-audit
        __path__: /var/log/flipper/audit.jsonl
  pipeline_stages:
    - json:
        expressions:
          tool_name: tool_name
          success: success
```

---

## Support

For issues or questions:
- Documentation: `/docs/` directory
- GitHub Issues: https://github.com/jtomek-strike48/flipper-connector/issues
