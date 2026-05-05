# Procedure: Publishing MKA-CLI Binaries

This document outlines the process for building and publishing the `mka` utility for various operating systems and architectures.

## 1. Prerequisites
- [Rust](https://rustup.rs/) (Stable)
- [cross](https://github.com/cross-rs/cross) (Optional, for simplified cross-compilation)
  ```bash
  cargo install cross --git https://github.com/cross-rs/cross
  ```

## 2. Target Architectures
The following targets are officially supported for `mka-cli`:

| OS | Architecture | Target Triple |
| :--- | :--- | :--- |
| **Linux** | x86_64 | `x86_64-unknown-linux-gnu` |
| **Linux** | ARM64 | `aarch64-unknown-linux-gnu` |
| **macOS** | Intel | `x86_64-apple-darwin` |
| **macOS** | M1/M2/M3 | `aarch64-apple-darwin` |
| **Windows** | x86_64 | `x86_64-pc-windows-msvc` |

## 3. Build Process

### Standard Build (Current OS)
```bash
cd mka-cli
cargo build --release
```

### Cross-Compilation (Using `cross`)
Using `cross` is recommended as it handles the linker and dependency setup via Docker.

```bash
# Example: Build for Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu
```

### Manual Cross-Compilation (Using `cargo`)
Ensure you have added the target via `rustup target add <target-triple>`.

```bash
# Example: Build for macOS Apple Silicon from Intel Mac
cargo build --release --target aarch64-apple-darwin
```

## 4. Packaging
Once built, the binaries are located in `mka-cli/target/<target-triple>/release/mka` (or `mka.exe` for Windows).

### Recommended Naming Convention
When uploading to GitHub Releases or a CDN:
- `mka-linux-x86_64`
- `mka-linux-aarch64`
- `mka-macos-x86_64`
- `mka-macos-aarch64`
- `mka-windows-x86_64.exe`

## 5. Verification
Always verify the binary on the target architecture before finalized publishing.
```bash
./mka --version
```
