# AI Knowledge Entry Point (MKA Protocol)

This project uses **Modular Knowledge Architecture (MKA)**. You MUST follow these rules for every task.

## 1. Core Mandates for **Efficiency**
- **Efficiency:** Use the `mka` utility to navigate based on intent. It provides context in TOON (Token Oriented Object Notation) which is highly token-efficient.
- **Deferred Maintenance:** MKA documentation is an end-of-cycle effort. **DO NOT** update `.mka.yaml` files or `.MKA/` documentation during active iterative coding. **ONLY** update Trigger Maps when the user explicitly commands: "update session changes to mka" or "Update documentation".

## 2. Technical Navigation (The Cache/Fallback Protocol)
MKA documents may lag behind active development (Documentation Drift). You must follow this sequence for discovery:
1. **Cache Check:** Always run `mka actions` first to see if a high-level Action exists for the task.
2. **Hit:** If a map exists, run `mka trigger-map <id> --snippets` to locate the code and understand the logic flow.
3. **Miss / Drift:** If the map does not exist, OR if the file/method in the map has been changed due to recent iteration, seamlessly fall back to standard search tools (`grep_search`, `glob`) to find the logic.
4. **Extraction:** Run `mka get-method <path> <method>` to surgically extract logic for a specific function outside of defined Trigger Maps.

## 3. Project-Specific Triggers (Refer as Needed)
Refer to these files **ONLY** when your task matches the trigger:

- **Trigger:** The user explicitly asks to "Update Documentation" or "update session changes to mka."
  - **Refer:** `.MKA/Procedures/MkaMaintenance.md`

- **Trigger:** The user asks to add a new command or action to the `mka` CLI.
  - **Refer:** `.MKA/Procedures/AddingNewCommands.md`

- **Trigger:** The user asks to build, release, or publish the `mka` binary for different platforms.
  - **Refer:** `.MKA/Procedures/PublishingBinary.md`

- **Trigger:** The user reports "Tree-sitter parser not found" or asks about language support.
  - **Refer:** `.MKA/Procedures/TreeSitterSetup.md`

- **Trigger:** The user asks how to integrate MKA with other AI tools like Claude, Copilot, or Cursor.
  - **Refer:** `.MKA/Procedures/IntegratingOtherAIs.md`

---
*MKA is designed for token efficiency. Only read detailed trigger-map, procedure, or maintenance files when they are directly triggered by the current task.*
