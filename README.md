# Modular Knowledge Architecture (MKA)

**The Surgical Logic Protocol for AI-First Engineering.**

MKA is a protocol and CLI utility designed to solve the "Context Noise" problem in Large Language Model (LLM) software engineering. It decouples the **Intent** (what a feature does) from the **Implementation** (how it's built) by providing a high-density "Mini-Map" of codebases.

---

## 🚀 The Core Philosophy

### 1. Context Efficiency (TOON)
When an AI is fed an entire project, the most important technical nuances often get lost in the noise. MKA provides a **High-Density Mini-Map** that forces the AI's attention onto the exact artifacts that define a **Workflow's** behavior. We use **TOON (Token Oriented Object Notation)** to maximize the signal-to-token ratio.

### 2. Decoupled Documentation
Traditional documentation dies the moment "Developer B" modifies code written by "Developer A." MKA solves this by **decoupling the "What" from the "How."** Instead of writing long READMEs, you define **Workflows**—structured lists of files and methods involved in a specific data flow.

### 3. Surgical Analysis
LLMs are native speakers of both English and Code. MKA removes "Translation Layer" bloat. We provide the entry points (the **Workflow Nodes**), and we let the AI's natural reasoning engine interpret the source code directly using extracted **Snippets**.

---

## 🛠️ Key Terminology

*   **Workflow**: A high-level functional goal of the project (e.g., "User Login", "User Authentication").
*   **Workflow Map**: The technical "map" or sequence of surface-level methods that data flows through to complete a Workflow.
*   **Workflow Node**: A specific file and method within a Workflow.
*   **Snippet**: A surgically extracted, minified version of a method's logic, optimized for AI context windows.

---

## 💻 CLI Usage

### Installation
```bash
# Clone the repo and build
cd mka-cli
cargo build --release
cp target/release/mka /usr/local/bin/
```

### Navigation
- **`mka workflow-search <query>`**: Search workflows semantically.
- **`mka workflow-search --listAll`**: List all available workflows.
- **`mka workflow-get <id>`**: Display the technical map (nodes) for a specific workflow (without snippets by default).
- **`mka workflow-get <id> --snippets`**: Display the workflow map including token-efficient logic snippets.
- **`mka install <lang>`**: Install Tree-sitter parsers to the centralized MKA folder.

---

## 📁 The .MKA Directory Structure

MKA stores its knowledge in a `.MKA` folder at the project root:
- `index.mka.yaml`: The master index of all **Workflows**.
- `Workflows/`: Contains `.mka.yaml` files defining the specific method flows.
- `Procedures/`: Narrative Markdown files for complex team workflows (e.g., publishing, maintenance).
- `schema.json`: The JSON schema that ensures all **Workflows** are technically valid.

---

## ⚙️ Configuration

MKA supports both user-level (global) and project-level configuration files (`config.yaml`).

* **Global Configuration:** Located at `~/.config/mka/config.yaml` (Linux/macOS) or `%APPDATA%\mka\config.yaml` (Windows).
* **Project Configuration:** Located at `.MKA/config.yaml` at the root of your project (overrides global settings).

> [!NOTE]
> All configuration parameters are optional. If you only define a subset of options (for example, just `parsers_enabled: true`), all omitted fields will automatically use their default fallback values listed below.

### Configuration Options

Below are the available configuration fields and default values:

```yaml
# Set to true to enable code parsers for rich symbol and logic snippet extraction
parsers_enabled: false

# Git repository URL for MKA templates (used during init)
# repo_url: "https://github.com/Shivaprasad-I/Modular-Knowledge-Architecture.git"

# Directory inside template repo containing MKA folders
# template_dir: "templates"

# Temporary directory used during git sparse checkout
# temp_dir: ".mka_temp"

# URL to download the semantic search ONNX model
# model_url: "https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/onnx/model.onnx"

# URL to download the tokenizer config
# tokenizer_url: "https://huggingface.co/Xenova/all-MiniLM-L6-v2/resolve/main/tokenizer.json"

# Custom directory for compiled Tree-sitter parser binaries (.so)
# treesitter_dir: "~/.mka/treesitter"

# Custom path to the semantic search ONNX model file
# model_path: "~/.local/share/mka/models/all-MiniLM-L6-v2.onnx"

# Custom path to the tokenizer.json file
# tokenizer_path: "~/.local/share/mka/models/tokenizer.json"

# Custom path/name for the workflow index file
# index_file: ".MKA/index.mka.yaml"

# Custom path/name for the schema file
# schema_file: ".MKA/schema.json"
```

---

## 📊 Performance & Token Optimization

MKA is designed to drastically reduce "context noise" and save tokens during AI-assisted development. By navigating codebases using a map-first workflow protocol, the LLM avoids exploratory directory scraping.

A profiling run comparing **Antigravity CLI (`agy`)** connected over Model Context Protocol (MCP) using `Gemini 3.5 Flash` shows:
* **33.2% Reduction** in the active model footprint (generation & tool call costs).
* **70.4% Reduction** in exploratory tool call tokens (saving time and API calls).
* **15.4% Reduction** in overall active session context size.

For step-by-step reproduction steps, detailed analysis, and visual TUI benchmarks, see [Metrics/README.md](https://github.com/Shivaprasad-I/Modular-Knowledge-Architecture/blob/main/Metrics/README.md).

---

## Notes

* MKA was born and raised on Linux. That's where it has seen the most testing and real-world use.
* Windows and macOS are supported, but they have mostly been validated through unit tests. If you're on Windows, you may be participating in a scientific experiment.
* If you find a bug, please open an issue. Reproducing it is optional; dramatic screenshots are appreciated.

## Contributing

* Feature requests, issues, and pull requests are welcome.
* AI-generated commits are allowed.
* The code should be tested by at least one intelligent entity before submission.

## Disclaimer

I know just enough Rust to be dangerous.

The architecture was designed by me.
The code was mostly written by AI agents.
The bugs were a collaborative effort.

## 📄 License
MIT
