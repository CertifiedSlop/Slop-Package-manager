#!/bin/bash
# Post-create script for dev container setup

set -e

echo "🦀 Setting up Rust development environment..."

# Install rustfmt and clippy
rustup component add rustfmt clippy

# Install cargo extensions
cargo install cargo-edit cargo-watch cargo-audit 2>/dev/null || true

# Generate shell completions
echo "Generating shell completions..."
mkdir -p completions
cargo run -- --help > /dev/null 2>&1 || true

echo "✅ Development environment ready!"
echo ""
echo "Available commands:"
echo "  cargo build     - Build the project"
echo "  cargo test      - Run tests"
echo "  cargo clippy    - Run linter"
echo "  cargo watch     - Watch for changes and rebuild"
echo "  cargo audit     - Check for security vulnerabilities"
