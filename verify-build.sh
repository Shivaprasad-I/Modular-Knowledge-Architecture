#!/bin/bash
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

# 2. Set up isolated environment variables to simulate clean CI container
# This avoids modifying or moving your real ~/.mka or config directories on the filesystem.
ENV_DIR="$PWD/target/verify_build_env"

# Preserve existing Cargo and Rustup homes so cargo can run and we don't redownload/recompile dependencies
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"

# Redirect home and config directories to the isolated environment directory
export HOME="$ENV_DIR"
export USERPROFILE="$ENV_DIR"
export XDG_CONFIG_HOME="$ENV_DIR/.config"
export APPDATA="$ENV_DIR/AppData/Roaming"

# Clean up the isolated environment directory on exit
cleanup() {
    echo "Cleaning up isolated build verification environment..."
    rm -rf "$ENV_DIR"
}
trap cleanup EXIT

# Re-create fresh isolated directories
rm -rf "$ENV_DIR"
mkdir -p "$ENV_DIR"

# 3. Run check
echo "Running cargo check..."
cargo check --manifest-path mka-cli/Cargo.toml

# 4. Prepare Environment (Tree-sitter Parsers)
# Since the environment is stashed, we compile the parsers before running tests (matching CI logic)
echo "Pre-installing Tree-sitter parsers to verify environment preparation..."
cargo run --manifest-path mka-cli/Cargo.toml -- install rust
cargo run --manifest-path mka-cli/Cargo.toml -- install python
cargo run --manifest-path mka-cli/Cargo.toml -- install javascript
cargo run --manifest-path mka-cli/Cargo.toml -- install typescript
cargo run --manifest-path mka-cli/Cargo.toml -- install c-sharp


# 5. Run tests in a loop with high thread count to catch flaky race conditions
echo "Running unit tests in a loop (5 iterations, 8 threads) to catch parallel race conditions..."
export MKA_STRICT_TESTS=1
for i in {1..5}; do
    echo "Iteration $i..."
    cargo test --manifest-path mka-cli/Cargo.toml -- --test-threads=8
done

# 6. Run release build
echo "Running release build..."
cargo build --release --manifest-path mka-cli/Cargo.toml

echo ""
echo "------------------------------------------------"
echo "VERIFICATION COMPLETE: Build is stable."
echo "Binary location: mka-cli/target/release/mka"
echo "------------------------------------------------"
