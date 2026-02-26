# Multi-stage Dockerfile for Flipper Zero Connector
# Optimized for minimal image size and security

# =============================================================================
# Builder Stage: Compile Rust application
# =============================================================================
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libudev-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY apps ./apps

# Build the application in release mode
RUN cargo build --release --package flipper-agent

# Strip debug symbols for smaller binary
RUN strip /app/target/release/flipper-agent

# =============================================================================
# Runtime Stage: Minimal runtime environment
# =============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libudev1 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for running the application
RUN useradd -m -u 1000 -s /bin/bash flipper

# Create directories for logs and config
RUN mkdir -p /var/log/flipper /etc/flipper && \
    chown -R flipper:flipper /var/log/flipper /etc/flipper

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/flipper-agent /usr/local/bin/flipper-agent

# Copy documentation (optional, for reference)
COPY --chown=flipper:flipper docs /app/docs

# Set up volumes for logs and configuration
VOLUME ["/var/log/flipper", "/etc/flipper"]

# Switch to non-root user
USER flipper

# Set working directory
WORKDIR /app

# Environment variables (can be overridden)
ENV RUST_LOG=info
ENV FLIPPER_AUDIT_ENABLED=true
ENV FLIPPER_AUDIT_LOG=/var/log/flipper/audit.jsonl
ENV FLIPPER_LOG_LEVEL=info

# Expose port for Strike48 communication (if applicable)
# EXPOSE 8080

# Health check (if flipper-agent supports it)
# HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
#   CMD flipper-agent health || exit 1

# Run the application
CMD ["/usr/local/bin/flipper-agent"]
