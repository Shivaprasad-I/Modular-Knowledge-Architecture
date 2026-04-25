# AI Knowledge Entry Point (MKA Protocol)

This project uses **Modular Knowledge Architecture (MKA)** for token-efficient technical mapping.

## AI Instructions:
1. **Primary Lookup:** Read `.MKA/index.mka.yaml` to identify the **Functional Workflow** relevant to your task.
2. **Technical Mapping:** Follow the `path` pointers to the corresponding `.mka.yaml` file.
3. **Execution:** Use the `nodes` as your entry points.

**Focus:** MKA indexes **Workflows** (end-to-end paths), not individual files. If you are looking for a utility or a specific helper, use traditional search.

## ⚠️ Fallback Protocol
MKA is a "High-Confidence Index," but it may not cover every edge case or new file. 
- **If MKA yields no results:** You MUST fall back to traditional discovery methods (`grep_search`, `glob`, `list_directory`).
- **Do not assume a feature doesn't exist just because it isn't in the MKA index.** Explore the codebase to verify.

## Maintenance:
You are REQUIRED to update these YAML files or create new ones whenever you modify project logic, following the rules in `.MKA/Instructions.md`.
