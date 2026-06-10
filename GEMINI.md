# AI Knowledge Entry Point (MKA Protocol)

This project uses **Modular Knowledge Architecture (MKA)**. You MUST follow these rules for every task.

## 1. Core Mandates for **Efficiency**
- **Efficiency:** Use the `mka` MCP server tools (via `call_mcp_tool`) as your primary toolset to navigate based on intent. It provides context in TOON (Token Oriented Object Notation) which is highly token-efficient.
- **Deferred Maintenance:** MKA documentation is an end-of-cycle effort. **DO NOT** update `.mka.yaml` files or `.MKA/` documentation during active iterative coding. **ONLY** update Workflows when the user explicitly commands: "update session changes to mka" or "Update documentation".

## 2. Technical Navigation (The Cache/Fallback Protocol)
MKA documents may lag behind active development (Documentation Drift). You must follow this sequence for discovery:
1. **Cache Check:** Always invoke the `mka_list_workflows` tool first (via `call_mcp_tool` with `ServerName: "mka"`) to see if a high-level Workflow exists for the task.
2. **Semantic Search:** If you need to search for a concept, feature, or workflow, ALWAYS use `mka_workflow_search` first. Avoid standard file search tools (like grep/ripgrep) unless you cannot find the relevant workflow or files through the MKA tools.
3. **Hit:** If a workflow exists, invoke the `mka_get_workflow` tool (via `call_mcp_tool` with `ServerName: "mka"`, passing the workflow ID) to locate the code and understand the logic flow.
4. **Miss / Drift:** If the workflow does not exist, OR if the file/method in the workflow has been changed due to recent iteration, seamlessly fall back to standard search tools (`grep_search`, `glob`) to find the logic.
5. **Extraction:** Run the `mka get-method <path> <method>` command to surgically extract logic for a specific function outside of defined Workflows if needed.

## 3. Project-Specific Workflows (Refer as Needed)
Refer to these files **ONLY** when your task matches the trigger:

- **Trigger:** The user explicitly asks to "Update Documentation" or "update session changes to mka."
  - **Refer:** `.MKA/Procedures/MkaMaintenance.md`

- **Trigger:** The user asks to add a new command or workflow to the `mka` CLI.
  - **Refer:** `.MKA/Procedures/AddingNewCommands.md`

- **Trigger:** The user asks to build, release, or publish the `mka` binary for different platforms.
  - **Refer:** `.MKA/Procedures/PublishingBinary.md`

- **Trigger:** The user reports "Tree-sitter parser not found" or asks about language support.
  - **Refer:** `.MKA/Procedures/TreeSitterSetup.md`

- **Trigger:** The user asks how to integrate MKA with other AI tools like Claude, Copilot, or Cursor.
  - **Refer:** `.MKA/Procedures/IntegratingOtherAIs.md`

---
*MKA is designed for token efficiency. Only read detailed workflow, procedure, or maintenance files when they are directly triggered by the current task.*
