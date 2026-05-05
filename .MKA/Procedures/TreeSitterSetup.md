# Procedure: Setting up Tree-sitter Parsers for MKA

MKA uses Tree-sitter for surgical code analysis. It expects language parsers to be available as shared libraries (`.so` files) on your system.

## 1. Supported Search Paths
The `mka` utility searches for parsers (e.g., `rust.so`, `python.so`) in the following locations (in order):
1.  **Neovim (lazy.nvim):** `~/.local/share/nvim/lazy/nvim-treesitter/parser/`
2.  **Neovim (packer.nvim):** `~/.local/share/nvim/site/pack/packer/start/nvim-treesitter/parser/`
3.  **Local Cache:** `~/.cache/tree-sitter/lib/`
4.  **System Global:** `/usr/local/lib/tree-sitter-<lang>.so`
5.  **Project Local:** `./parsers/`

## 2. Installation via MKA (Recommended)
The `mka` utility includes a built-in installer that clones and compiles grammars for you.
```bash
mka install <language>
```
This will automatically:
1.  Clone the grammar repository from GitHub.
2.  Detect your C/C++ compiler (gcc, g++, or cl.exe).
3.  Compile the parser as a shared library.
4.  Move it to your local cache (`~/.cache/tree-sitter/lib/` or `%LOCALAPPDATA%\tree-sitter\lib`).

## 3. Installation via Neovim
If you use Neovim with `nvim-treesitter`, you can also install grammars within Neovim:
```vim
:TSInstall <language>
```
MKA is configured to automatically search your Neovim parser directories on both Linux and Windows.

## 4. Manual Installation (Global/Cache)
If the automated installer fails, you can build the parser manually:
1.  Clone the grammar: `git clone https://github.com/tree-sitter/tree-sitter-<lang>.git`
2.  Compile:
    - **Linux/macOS:** `gcc -O3 -shared -fPIC -I./src src/parser.c [src/scanner.c] -o ~/.cache/tree-sitter/lib/<lang>.so`
    - **Windows (CMD/PowerShell):** `cl.exe /LD /Isrc src/parser.c [src/scanner.c] /Fe:python.dll`

## 4. Troubleshooting
If you get a `Tree-sitter parser for '<lang>' not found` error:
- Verify the `.so` file name matches the expected name (e.g., `rust.so` for Rust).
- Ensure the file is in one of the search paths listed in Section 1.
- Check that the C compiler used to build the parser is compatible with your architecture.
