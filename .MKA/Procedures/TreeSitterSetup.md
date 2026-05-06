# Procedure: Setting up Tree-sitter Parsers for MKA

MKA uses Tree-sitter for surgical code analysis. It expects language parsers to be available as shared libraries (`.so` files) on your system.

## 1. Supported Search Paths
The `mka` utility searches for parsers (e.g., `rust.so`, `python.so`) in a single centralized location managed by the internal `Config` class:
1.  **Release Build:** `~/.mka/treesitter/`
2.  **Debug Build:** `~/.mka/treesitter-debug/`

## 2. Installation via MKA (Recommended)
The `mka` utility includes a built-in installer that clones and compiles grammars for you.
```bash
mka install <language>
```
This will automatically:
1.  Clone the grammar repository from GitHub.
2.  Detect your C/C++ compiler (gcc, g++, or cl.exe).
3.  Compile the parser as a shared library.
4.  Move it to your centralized MKA folder (e.g., `~/.mka/treesitter/` for release builds).

## 3. Troubleshooting
If you get a `Tree-sitter parser for '<lang>' not found` error:
- Verify the parser file exists in your centralized MKA folder.
- Ensure the file name matches the expected name (e.g., `rust.so` for Rust on Linux/macOS, `rust.dll` on Windows).
- Check that the C compiler used to build the parser is compatible with your architecture.
