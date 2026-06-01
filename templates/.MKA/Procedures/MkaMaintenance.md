# Procedure: Maintaining MKA Knowledge

MKA knowledge is **surgical**. It must be kept lean and technically accurate to ensure AI efficiency.

## 1. Defining a Workflow
Do not index every file or class. MKA is strictly for **Functional Workflows**. A workflow is defined as:
- A sequence of surface-level methods that data travels through to complete a **Functional Goal**.
- A collection of "Entry Points" that an AI needs to understand to modify a specific logic flow.

## 2. Maintenance Lifecycle
- **New Workflows:** When a new high-level capability is added, create a `.mka.yaml` file in `Workflows/`.
- **Maintenance:** Update existing workflows only when the functional path changes (e.g., a method is renamed or the data flow is rerouted).
- **Automation:** Run `mka sync` regularly to detect and heal broken file paths in your workflows.

## 3. Best Practices
- **Atomic Intents:** Each workflow should have one clear `intent` string.
- **Workflow Nodes Only:** List the files and methods involved. Do not write narrative logic in the YAML—MKA will extract **Snippets** automatically.
- **Cross-Referencing:** If a node triggers another complete Workflow, use the `workflow: <id>` field instead of `file: <path>`. This allows the AI to traverse modular logic graphs without duplicating `.mka.yaml` file paths.
- **Validation:** Provide a pointer to the `test_file` that verifies the action.

## 4. Finalizing Changes
Before committing changes to the `.MKA` directory:
1.  Verify the YAML against `schema.json`.
2.  Run `mka workflow-list` to ensure the new workflow appears in the index.
3.  Run `mka workflow-get <id> --snippets` to confirm the logic extraction is working as expected.
