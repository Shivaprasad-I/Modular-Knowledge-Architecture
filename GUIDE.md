# MKA MCP Integration Guide

This guide covers how to integrate the **Modular Knowledge Architecture (MKA)** MCP server into various AI agentic workflows. By connecting MKA as an MCP server, you enable AI assistants to autonomously navigate your codebase using high-density TOON maps.

---

## 🚀 1. Installation

First, ensure you have the latest version of MKA installed on your system:

```bash
cd mka-cli
cargo install --path .
```

Verify the installation:
```bash
mka --help
```
*You should see `workflow-search` and `mcp` in the commands list.*

---

## 🤖 2. Gemini CLI

Gemini CLI has native support for MCP servers.

### Fast Setup
Run this command from your terminal:
```bash
gemini mcp add mka mka mcp --trust
```

### Manual Configuration
Add MKA to your `~/.gemini/settings.json`:
```json
{
  "mcpServers": {
    "mka": {
      "command": "mka",
      "args": ["mcp"],
      "trust": true
    }
  }
}
```

---

## 🛸 3. Antigravity CLI

Antigravity CLI (`agy`) supports native plugins, making it extremely simple to install and configure MCP servers with a single command.

### Fast Setup
Run this command from the repository root:
```bash
agy plugin install templates/mka-plugin
```

### Manual Configuration
Alternatively, you can manually configure it by creating or editing your global `~/.gemini/antigravity-cli/mcp_config.json` file:
```json
{
  "mcpServers": {
    "mka": {
      "command": "mka",
      "args": ["mcp"],
      "trust": true
    }
  }
}
```

---

## 🎭 4. Claude Desktop

Claude Desktop allows you to use MKA tools directly in the chat interface.

1. Open your Claude Desktop configuration file:
   - **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

2. Add MKA to the `mcpServers` section:
```json
{
  "mcpServers": {
    "mka": {
      "command": "mka",
      "args": ["mcp"]
    }
  }
}
```
3. Restart Claude Desktop. You will see a 🔌 icon indicating the server is connected.

---

## 🖱️ 5. Cursor

Cursor supports MCP servers to enhance its codebase indexing and agentic capabilities.

1. Go to **Settings** > **Cursor Settings** > **General** > **MCP**.
2. Click **+ Add New MCP Server**.
3. Fill in the details:
   - **Name**: `mka`
   - **Type**: `stdio`
   - **Command**: `mka mcp`
4. Click **Save**. Cursor's Composer and Agent will now use MKA to find logic locations.

---

## 🐱 6. GitHub Copilot (Extensions)

For GitHub Copilot, MCP integration is typically handled via the **Copilot Extensions** or local proxy tools like `mcp-bridge`.

### Using with local agent
If you are using a local agent wrapper that supports MCP, point it to:
- **Executable**: `mka`
- **Arguments**: `mcp`

---

## 🧠 How it Works: The "Map-First" Strategy

MKA is designed to be your AI's **Navigation Map**. Instead of the AI guessing where code lives or grepping the entire project, it will:

1. **Semantic Search / Listing**: Call `mka_workflow_search` to find relevant workflows using natural language, or set `list_all: true` to list all available workflows.
2. **Location Discovery**: Call `mka_get_workflow` to get a technical map of files and methods.
3. **Contextual Read**: Perform standard file reads on the exact targets identified by MKA.

This approach significantly reduces token noise and prevents the AI from getting lost in large repositories.
