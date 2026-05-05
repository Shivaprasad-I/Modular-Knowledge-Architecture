# Modular Knowledge Architecture (MKA) Maintenance

As an AI assistant, you are responsible for maintaining the **MKA Technical Index**. 

## 1. Defining a Workflow
Do not index every file or class. MKA is strictly for **Functional Workflows**. A workflow is defined as:
- A sequence of events that fulfills a specific goal (e.g., "User Login", "Video Playback").
- A path that typically spans multiple files or layers (e.g., UI -> Service -> Database).
- **Prohibited:** Do not create MKA files for utility classes, standalone constants, or isolated helper functions.

## 2. Documentation Lifecycle
- **New Workflows:** When a new end-to-end feature is added, create a `.mka.yaml` file.
- **Maintenance:** Update existing workflows only when the functional path changes.

## 3. The "Planar List" Protocol
- **Nodes Only:** List the files and methods involved. Do not write narrative logic.
- **Validation:** Provide a pointer to the `test_file` that verifies the feature.

## 4. Fallback Responsibility
MKA does not replace your standard engineering tools. 
- If the MKA index is missing information, you MUST use `grep_search`, `glob`, or other discovery tools to find the truth in the code.
- Once you discover a missing path via traditional search, you are expected to **add it to the MKA index** to help future sessions.

## 5. MKA File Structure
- **Extension:** `.mka.yaml`
- **Pathing:** All paths MUST be relative to the project root.
- **Schema:** Adhere strictly to `.MKA/schema.json`.
