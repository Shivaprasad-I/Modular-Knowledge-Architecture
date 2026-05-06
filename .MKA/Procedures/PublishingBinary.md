# Procedure: Publishing MKA-CLI Binaries

This document outlines the process for building and publishing the `mka` utility for various operating systems and architectures.

## 1. Automated Publishing (Recommended)
This project uses GitHub Actions to automate the release process. 

### Triggering a Release
To trigger a new release and build binaries for all supported platforms:
1.  Update the version in `mka-cli/Cargo.toml`.
2.  Commit and push the change.
3.  Create and push a new git tag starting with `v` (e.g., `v0.1.0`):
    ```bash
    git tag v0.1.0
    git push origin v0.1.0
    ```

The workflow defined in `.github/workflows/publish.yml` will automatically build the binaries and attach them to a new GitHub Release.

## 2. Prerequisites (Manual Build)
- [Rust](https://rustup.rs/) (Stable)
- [cross](https://github.com/cross-rs/cross) (Optional, for simplified cross-compilation)
  ```bash
  cargo install cross --git https://github.com/cross-rs/cross
  ```

## 3. Target Architectures
The following targets are officially supported for `mka-cli`:

| OS | Architecture | Target Triple |
| :--- | :--- | :--- |
| **Linux** | x86_64 | `x86_64-unknown-linux-gnu` |
| **Linux** | ARM64 | `aarch64-unknown-linux-gnu` |
| **macOS** | Intel | `x86_64-apple-darwin` |
| **macOS** | M1/M2/M3 | `aarch64-apple-darwin` |
| **Windows** | x86_64 | `x86_64-pc-windows-msvc` |

## 4. Manual Build Process

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

## 5. Packaging
Once built, the binaries are located in `mka-cli/target/<target-triple>/release/mka` (or `mka.exe` for Windows).

### Recommended Naming Convention
When uploading to GitHub Releases or a CDN:
- `mka-linux-x86_64`
- `mka-linux-aarch64`
- `mka-macos-x86_64`
- `mka-macos-aarch64`
- `mka-windows-x86_64.exe`

## 6. Verification
Always verify the binary on the target architecture before finalized publishing.
```bash
./mka --version
```
