# AI Knowledge Entry Point (MKA Protocol)

This project uses **Modular Knowledge Architecture (MKA)**. You MUST follow these rules for every task.

## 1. Core Mandates for **Efficiency**
- **Efficiency:** Rely on the MKA index to navigate based on intent, before reading the entire codebase.

## 2. Technical Navigation (Workflows)
1. Read `.MKA/index.mka.yaml` to identify the functional workflow for your task.
2. Follow the `path` to the specific `.mka.yaml` file for the technical "Mini-Map."

## 4. Fallback Responsibility
MKA does not replace your standard engineering tools.
- If the MKA index is missing information, you MUST use `grep_search`, `glob`, or other discovery tools to find the truth in code.
- On discovering missing paths via traditional search, you are expected to **add that workflow to MKA** to take advantage in future sessions.

## 3. Project-Specific Triggers (Refer as Needed)
Refer to these files **ONLY** when your task matches the trigger:

- **Trigger:** The user explicitly asks to "Update Documentation" or "Finalize MKA."
  - **Refer:** `.MKA/Procedures/MkaMaintenance.md`

---
*MKA is designed for token efficiency. Only read detailed workflow, procedure, or maintenance files when they are directly triggered by the current task.*
