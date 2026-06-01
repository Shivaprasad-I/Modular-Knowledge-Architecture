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

*   **Workflow**: A high-level functional goal of the project (e.g., "User Login", "Sync Data").
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
- **`mka workflow-list`**: List all high-level workflows.
- **`mka workflow-get <id>`**: Display the technical map (nodes) for a specific workflow.
- **`mka workflow-get <id> --snippets`**: Explore specific workflows with token-efficient logic snippets.
- **`mka sync`**: Automatically heal broken paths in your workflows if files are moved.
- **`mka install <lang>`**: Install Tree-sitter parsers to the centralized MKA folder.

---

## 📁 The .MKA Directory Structure

MKA stores its knowledge in a `.MKA` folder at the project root:
- `index.mka.yaml`: The master index of all **Workflows**.
- `Workflows/`: Contains `.mka.yaml` files defining the specific method flows.
- `Procedures/`: Narrative Markdown files for complex team workflows (e.g., publishing, maintenance).
- `schema.json`: The JSON schema that ensures all **Workflows** are technically valid.

---

## 📄 License
MIT
