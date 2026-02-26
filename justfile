# Hello World Connector - Build & Run Recipes
# Run `just --list` to see all available commands

# Default recipe shows help
default:
    @just --list

# ============ Development ============

# Check all code compiles
check:
    cargo check --workspace

# Run clippy lints
lint:
    cargo clippy --workspace -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Format check (CI)
fmt-check:
    cargo fmt --all -- --check

# Run tests
test:
    cargo test --workspace

# Build all crates
build:
    cargo build --workspace

# Build release
build-release:
    cargo build --workspace --release

# Clean build artifacts
clean:
    cargo clean

# ============ Run ============

# Run headless agent
run *ARGS:
    cargo run --package hello-headless -- {{ARGS}}

# Run headless agent (release)
run-release *ARGS:
    cargo run --package hello-headless --release -- {{ARGS}}

# ============ All ============

# Run all CI checks (check + fmt + clippy + test)
ci: check fmt-check lint test
    @echo "All CI checks passed!"
