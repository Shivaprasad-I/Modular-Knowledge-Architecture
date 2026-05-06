# Procedure: Integrating MKA with Other AI Assistants

Modular Knowledge Architecture (MKA) is designed to make any AI agent token-efficient and accurate. While some CLIs (like Gemini CLI) have native "Skill" plugins, you can easily instruct other popular AI tools (Claude, GitHub Copilot, Cursor) to follow the MKA Protocol.

## 1. Claude Desktop / Claude Code
Claude relies heavily on "System Prompts" or custom instructions provided at the start of a project.

**How to integrate:**
1. Create a `claude.json` or `.clauderc` file in your project root (if supported by your specific Claude wrapper) or paste the following into the project's custom instructions:
2. **The Prompt:**
   ```text
   This project uses the Modular Knowledge Architecture (MKA).
   1. DO NOT read files blindly to discover logic.
   2. ALWAYS run `mka actions` to see available workflows.
   3. ALWAYS run `mka trigger-map <id>` to get the line-numbered logic snippets.
   4. If you need to edit a file, use the line numbers from the MKA snippet to read the exact bounds of the file, then apply your edit.
   5. DO NOT update `.mka.yaml` files unless explicitly asked to "update session changes to mka".
   ```

## 2. GitHub Copilot Workspace / Chat
GitHub Copilot respects instructions placed in a `.github/copilot-instructions.md` file.

**How to integrate:**
1. Create `.github/copilot-instructions.md` in your repository.
2. Add the following content:
   ```markdown
   # MKA Navigation Protocol
   When answering questions about the codebase or planning edits:
   - Use the terminal to run `mka actions` to find relevant features.
   - Run `mka trigger-map <id>` to understand the flow before suggesting code changes.
   - MKA snippets are minified; you must read the original file using the provided line numbers to do exact replacements.
   ```

## 3. Cursor IDE
Cursor uses `.cursorrules` files in the project root to guide its agentic behavior.

**How to integrate:**
1. Create a `.cursorrules` file in the root of your project.
2. Add the following rules:
   ```text
   Rules for navigating this repository:
   - Command Execution: Always run `mka actions` to discover intents.
   - Logic Discovery: Run `mka trigger-map <id>` instead of searching via regex.
   - File Editing: Use the line numbers provided in the MKA snippet to locate the exact code block for editing.
   - Documentation: Do not modify `.MKA/` folder contents during iterative coding.
   ```

## 4. General Custom GPTs / Assistants
If you are using a web-based LLM or a generic agent, paste the contents of `AI.md` (or `GEMINI.md`) into its system prompt or first message to ensure it respects the Cache/Fallback protocol and defers documentation maintenance.
