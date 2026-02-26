# Deployment Guide - Flipper Zero Connector

## Overview

This guide covers deploying the Flipper Zero connector using Docker for production environments, including Strike48 platform integration.

---

## Prerequisites

### System Requirements

- **OS:** Linux (Ubuntu 20.04+, Debian 11+, RHEL 8+)
- **Docker:** 20.10+ with Docker Compose
- **CPU:** 2+ cores recommended
- **Memory:** 1GB RAM minimum, 2GB recommended
- **Storage:** 500MB for image, 5GB+ for logs (depends on usage)

### Flipper Zero Requirements

- Flipper Zero device connected via USB
- Official, Unleashed, or Xtreme firmware
- USB cable with data transfer capability

### Access Requirements

- Docker installed and running
- User in `docker` group (for non-root access)
- USB device access permissions

---

## Quick Start

### 1. Build the Docker Image

```bash
# Clone repository
git clone https://github.com/jtomek-strike48/flipper-connector.git
cd flipper-connector

# Build Docker image
docker build -t flipper-connector:latest .
```

### 2. Run with Docker Compose

```bash
# Create directories for logs and config
mkdir -p logs config

# Start the connector
docker-compose up -d

# View logs
docker-compose logs -f
```

### 3. Verify Deployment

```bash
# Check container status
docker-compose ps

# Check logs
docker-compose logs flipper-connector

# Verify Flipper Zero connection
docker-compose exec flipper-connector flipper-agent --version
```

---

## Docker Build

### Standard Build

```bash
docker build -t flipper-connector:latest .
```

### Build with Custom Tag

```bash
docker build -t flipper-connector:v1.2.0 .
```

### Build with Build Args

```bash
docker build \
  --build-arg RUST_VERSION=1.75 \
  -t flipper-connector:latest .
```

### Multi-Platform Build

```bash
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t flipper-connector:latest .
```

---

## Docker Run

### Basic Run

```bash
docker run -d \
  --name flipper-connector \
  --device /dev/bus/usb:/dev/bus/usb \
  -v $(pwd)/logs:/var/log/flipper \
  flipper-connector:latest
```

### With Environment Variables

```bash
docker run -d \
  --name flipper-connector \
  --device /dev/bus/usb:/dev/bus/usb \
  -v $(pwd)/logs:/var/log/flipper \
  -e RUST_LOG=debug \
  -e FLIPPER_AUDIT_ENABLED=true \
  -e FLIPPER_AUDIT_LOG=/var/log/flipper/audit.jsonl \
  flipper-connector:latest
```

### Interactive Mode (for testing)

```bash
docker run -it --rm \
  --name flipper-connector \
  --device /dev/bus/usb:/dev/bus/usb \
  flipper-connector:latest \
  /bin/bash
```

---

## Docker Compose

### Default Configuration

```yaml
# docker-compose.yml
version: '3.8'

services:
  flipper-connector:
    image: flipper-connector:latest
    container_name: flipper-connector
    restart: unless-stopped

    environment:
      - RUST_LOG=info
      - FLIPPER_AUDIT_ENABLED=true
      - FLIPPER_AUDIT_LOG=/var/log/flipper/audit.jsonl

    volumes:
      - ./logs:/var/log/flipper
      - ./config:/etc/flipper
      - /dev/bus/usb:/dev/bus/usb:rw

    devices:
      - /dev/bus/usb:/dev/bus/usb

    cap_add:
      - SYS_RAWIO
```

### Commands

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# Restart services
docker-compose restart

# View logs
docker-compose logs -f flipper-connector

# Execute command in container
docker-compose exec flipper-connector flipper-agent --help

# Scale (if needed)
docker-compose up -d --scale flipper-connector=2
```

---

## Environment Variables

### Application Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |
| `FLIPPER_LOG_LEVEL` | `info` | Flipper-specific log level |
| `FLIPPER_AUDIT_ENABLED` | `true` | Enable audit logging |
| `FLIPPER_AUDIT_LOG` | `/var/log/flipper/audit.jsonl` | Audit log file path |

### Strike48 Integration

| Variable | Default | Description |
|----------|---------|-------------|
| `STRIKE48_API_URL` | - | Strike48 API endpoint |
| `STRIKE48_API_KEY` | - | Strike48 API key |
| `STRIKE48_WORKSPACE` | - | Strike48 workspace ID |

### USB Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `FLIPPER_USB_VID` | `0x0483` | Flipper Zero USB Vendor ID |
| `FLIPPER_USB_PID` | `0x5740` | Flipper Zero USB Product ID |

### Example .env File

```bash
# .env
RUST_LOG=info
FLIPPER_AUDIT_ENABLED=true
FLIPPER_AUDIT_LOG=/var/log/flipper/audit.jsonl

# Strike48 Configuration
STRIKE48_API_URL=https://api.strike48.com
STRIKE48_API_KEY=your-api-key-here
STRIKE48_WORKSPACE=workspace-id
```

---

## Volume Mounts

### Log Directory

```bash
# Host directory for logs
mkdir -p logs
chmod 755 logs

# Mount in docker-compose.yml
volumes:
  - ./logs:/var/log/flipper
```

**Contents:**
- `audit.jsonl` - Audit log (if enabled)
- `flipper.log` - Application log
- `error.log` - Error log

### Configuration Directory

```bash
# Host directory for config
mkdir -p config
chmod 755 config

# Mount in docker-compose.yml
volumes:
  - ./config:/etc/flipper
```

**Configuration Files:**
- `config.toml` - Connector configuration (if used)
- `audit.conf` - Audit logging configuration
- `tools.json` - Tool-specific configuration

### USB Device Access

```bash
# Mount USB devices
volumes:
  - /dev/bus/usb:/dev/bus/usb:rw

# Or specific device
devices:
  - /dev/bus/usb:/dev/bus/usb
```

---

## USB Device Access

### Linux USB Permissions

**Option 1: udev Rules (Recommended)**

Create `/etc/udev/rules.d/99-flipper.rules`:

```bash
# Flipper Zero
SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="5740", MODE="0666", GROUP="plugdev"
```

Reload udev rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

**Option 2: Add User to dialout Group**

```bash
sudo usermod -aG dialout $USER
sudo usermod -aG plugdev $USER
```

**Option 3: Docker Privileged Mode (Not Recommended)**

```yaml
services:
  flipper-connector:
    privileged: true
```

### Verify USB Access

```bash
# List USB devices
lsusb | grep Flipper

# Check permissions
ls -l /dev/bus/usb/001/00*

# Test in container
docker-compose exec flipper-connector lsusb
```

---

## Strike48 Integration

### Configuration

**1. Environment Variables**

```bash
export STRIKE48_API_URL=https://api.strike48.com
export STRIKE48_API_KEY=your-api-key
export STRIKE48_WORKSPACE=workspace-id
```

**2. Docker Compose**

```yaml
services:
  flipper-connector:
    environment:
      - STRIKE48_API_URL=${STRIKE48_API_URL}
      - STRIKE48_API_KEY=${STRIKE48_API_KEY}
      - STRIKE48_WORKSPACE=${STRIKE48_WORKSPACE}
```

### Deployment to Strike48 Platform

**1. Build and Push Image**

```bash
# Tag for registry
docker tag flipper-connector:latest registry.strike48.com/flipper-connector:v1.2.0

# Push to Strike48 registry
docker push registry.strike48.com/flipper-connector:v1.2.0
```

**2. Deploy via Strike48 CLI**

```bash
strike48 deploy flipper-connector \
  --image registry.strike48.com/flipper-connector:v1.2.0 \
  --workspace your-workspace \
  --device-path /dev/bus/usb
```

**3. Verify Deployment**

```bash
# Check connector status
strike48 connector status flipper-connector

# List available tools
strike48 connector tools flipper-connector

# Test tool execution
strike48 connector exec flipper-connector flipper_device_info
```

---

## Production Deployment

### Best Practices

**1. Use Docker Secrets (not environment variables)**

```yaml
services:
  flipper-connector:
    secrets:
      - strike48_api_key

secrets:
  strike48_api_key:
    file: ./secrets/api_key.txt
```

**2. Enable Resource Limits**

```yaml
deploy:
  resources:
    limits:
      cpus: '2'
      memory: 1G
    reservations:
      cpus: '0.5'
      memory: 256M
```

**3. Configure Health Checks**

```yaml
healthcheck:
  test: ["CMD", "flipper-agent", "health"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 5s
```

**4. Use Named Volumes**

```yaml
volumes:
  flipper-logs:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: /var/lib/flipper/logs
```

**5. Implement Log Rotation**

```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

### Security Hardening

**1. Run as Non-Root User** (already configured)

```dockerfile
USER flipper
```

**2. Read-Only Filesystem**

```yaml
services:
  flipper-connector:
    read_only: true
    tmpfs:
      - /tmp
```

**3. Drop Capabilities**

```yaml
cap_drop:
  - ALL
cap_add:
  - SYS_RAWIO  # Only for USB access
```

**4. Network Isolation**

```yaml
networks:
  flipper:
    driver: bridge
    internal: true
```

---

## Monitoring

### Container Logs

```bash
# Follow logs
docker-compose logs -f

# Tail last 100 lines
docker-compose logs --tail=100

# Filter by service
docker-compose logs flipper-connector

# Export logs
docker-compose logs > flipper-logs.txt
```

### Audit Logs

```bash
# View audit log
tail -f logs/audit.jsonl

# Parse JSON
tail -f logs/audit.jsonl | jq '.'

# Filter failed operations
tail -f logs/audit.jsonl | jq 'select(.success == false)'
```

### Container Stats

```bash
# Resource usage
docker stats flipper-connector

# Inspect container
docker inspect flipper-connector

# Container processes
docker top flipper-connector
```

---

## Troubleshooting

### Container Won't Start

**Check logs:**
```bash
docker-compose logs flipper-connector
```

**Common issues:**
- USB device not accessible → Check udev rules
- Permission denied → Add user to docker group
- Port already in use → Change port mapping

### Flipper Zero Not Detected

**Verify USB connection:**
```bash
# On host
lsusb | grep Flipper

# In container
docker-compose exec flipper-connector lsusb
```

**Check permissions:**
```bash
ls -l /dev/bus/usb/00*/00*
```

**Restart container with privileged mode (temporary):**
```bash
docker run --privileged -it flipper-connector:latest /bin/bash
```

### Audit Logs Not Writing

**Check directory permissions:**
```bash
ls -ld logs/
```

**Check environment variable:**
```bash
docker-compose exec flipper-connector env | grep AUDIT
```

**Verify volume mount:**
```bash
docker inspect flipper-connector | jq '.[0].Mounts'
```

### High Memory Usage

**Check resource limits:**
```bash
docker stats flipper-connector
```

**Adjust limits in docker-compose.yml:**
```yaml
deploy:
  resources:
    limits:
      memory: 512M
```

---

## Backup & Recovery

### Backup Audit Logs

```bash
# Copy logs from container
docker cp flipper-connector:/var/log/flipper ./backup/

# Or from mounted volume
tar -czf flipper-logs-$(date +%Y%m%d).tar.gz logs/
```

### Backup Configuration

```bash
# Backup config directory
tar -czf flipper-config-$(date +%Y%m%d).tar.gz config/
```

### Restore

```bash
# Extract logs
tar -xzf flipper-logs-20260225.tar.gz -C ./

# Restart container
docker-compose restart flipper-connector
```

---

## Upgrading

### Update to New Version

```bash
# Pull new image
docker pull flipper-connector:v1.3.0

# Stop current version
docker-compose down

# Update docker-compose.yml with new version
# image: flipper-connector:v1.3.0

# Start new version
docker-compose up -d

# Verify
docker-compose logs -f
```

### Rollback

```bash
# Stop current version
docker-compose down

# Revert to previous version in docker-compose.yml
# image: flipper-connector:v1.2.0

# Start previous version
docker-compose up -d
```

---

## Maintenance

### Log Rotation

```bash
# Rotate audit logs
mv logs/audit.jsonl logs/audit-$(date +%Y%m%d).jsonl
docker-compose exec flipper-connector kill -USR1 1
```

### Cleanup

```bash
# Remove stopped containers
docker container prune

# Remove unused images
docker image prune

# Remove unused volumes
docker volume prune

# Full cleanup
docker system prune -a
```

---

## Support

For issues or questions:
- Documentation: `/docs/` directory
- GitHub Issues: https://github.com/jtomek-strike48/flipper-connector/issues
- Strike48 Support: support@strike48.com
