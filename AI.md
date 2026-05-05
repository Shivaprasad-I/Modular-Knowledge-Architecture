# AI Knowledge Entry Point (MKA Protocol)

This project uses **Modular Knowledge Architecture (MKA)**. You MUST follow these rules for every task.

## 1. Core Mandates for **Efficiency**
- **Efficiency:** Use the `mka` utility to navigate based on intent. It provides context in TOON (Token Oriented Object Notation) which is highly token-efficient.

## 2. Technical Navigation (Workflows)
1. Run `mka features-list` to identify the functional workflow for your task.
2. Run `mka feature <id>` to get the technical "Mini-Map" (signatures, notes).
3. Run `mka feature <id> --view` if you need to understand the logic flow without reading the entire source file.
4. Run `mka get-method <path> <method>` to surgically extract logic for a specific function outside of defined workflows.

## 4. Project-Specific Triggers (Refer as Needed)
Refer to these files **ONLY** when your task matches the trigger:

- **Trigger:** The user explicitly asks to "Update Documentation" or "Finalize MKA."
  - **Refer:** `.MKA/Procedures/MkaMaintenance.md`

- **Trigger:** The user asks to add a new command or feature to the `mka` CLI.
  - **Refer:** `.MKA/Procedures/AddingNewCommands.md`

- **Trigger:** The user asks to build, release, or publish the `mka` binary for different platforms.
  - **Refer:** `.MKA/Procedures/PublishingBinary.md`

- **Trigger:** The user reports "Tree-sitter parser not found" or asks about language support.
  - **Refer:** `.MKA/Procedures/TreeSitterSetup.md`

---
*MKA is designed for token efficiency. Only read detailed workflow, procedure, or maintenance files when they are directly triggered by the current task.*
