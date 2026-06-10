#!/bin/bash
# scripts/verify-build.sh
# Run this script to verify that the MKA CLI builds correctly for your platform.
# This mirrors the logic used in GitHub Actions.

set -e

echo "Starting local build verification..."

# 1. Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Error: cargo is not installed. Please install Rust: https://rustup.rs/"
    exit 1
fi

# 2. Clean target directory (optional, but ensures a fresh build)
# echo "Cleaning target directory..."
# cargo clean --manifest-path mka-cli/Cargo.toml

# 3. Run check
echo "Running cargo check..."
cargo check --manifest-path mka-cli/Cargo.toml

# 4. Run tests
echo "Running unit tests..."
# Note: This will attempt to run semantic search tests if the model is installed locally.
cargo test --manifest-path mka-cli/Cargo.toml

# 5. Run release build
echo "Running release build..."
cargo build --release --manifest-path mka-cli/Cargo.toml

echo ""
echo "------------------------------------------------"
echo "VERIFICATION COMPLETE: Build is stable."
echo "Binary location: mka-cli/target/release/mka"
echo "------------------------------------------------"
