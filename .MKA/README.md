# Modular Knowledge Architecture (MKA) - Local Registry

## Mission: LLM-First Indexing
This directory is a **High-Density Technical Index** designed for LLM navigation. Its goal is to provide an AI agent with the minimum required "Artifact Pointers" to understand a feature without exhausting the context window.

## Protocol: The Planar List
- **Flat Mapping:** Do not use complex logic gates (if/else) or strict sequential steps.
- **Bag of Pointers:** List the "nodes" (Files/Methods) involved in a workflow.
- **Truth in Code:** Assume the LLM will use the `nodes` to read the actual code.

## 🚀 How to Bootstrap an AI Session
To ensure your AI agent (Gemini, Claude, ChatGPT, etc.) understands this architecture, use **one** of the following methods at the start of a session:

### Option A: The Bootstrap Prompt (Recommended)
Paste this as your first message:
> "Please read `.MKA/AI.md` to understand the project's technical indexing protocol before performing any tasks."

### Option B: Tool-Specific Configuration
If your tool supports persistent instructions (e.g., `.cursorrules`, `.clinerules`, or Custom Instructions), point it to `.MKA/AI.md` as the primary source of truth.

### Option C: Manual Context Attachment
Attach `.MKA/AI.md` as a file to your session.

## Directory Structure
- `AI.md`: The entry point for all AI agents.
- `Instructions.md`: Maintenance rules for the AI.
- `schema.json`: The structural source of truth.
- `index.mka.yaml`: The master registry of workflows.
- `Workflows/`: Surgical detail files using the `.mka.yaml` extension.

---
*MKA is an experimental project indexing system. Maintain technical precision above all else.*
