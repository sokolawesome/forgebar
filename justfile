# List available tasks
list:
    @just --list

# Build the application binary (debug)
build:
    @echo "Building forgebar (debug)..."
    @cargo build

# Build the application binary for release (optimized)
build-release:
    @echo "Building forgebar (release)..."
    @cargo build --release

# Run the application using a debug build
run:
    @echo "Running forgebar..."
    @cargo run

# Run the application using a release build
run-release: build-release
    @echo "Running optimized forgebar..."
    @./target/release/forgebar

# Format all Rust source files
fmt:
    @echo "Formatting code..."
    @cargo fmt

# Check the codebase for errors without building an executable
check:
    @echo "Checking packages and dependencies..."
    @cargo check

# Lint the codebase using Clippy (Rust's linter)
clippy:
    @echo "Linting code with Clippy..."
    @cargo clippy --all-targets -- -D warnings

# Test the codebase
test:
    @echo "Testing code..."
    @cargo test

# Clean all build artifacts
clean:
    @echo "Cleaning build artifacts..."
    @cargo clean
