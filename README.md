# Modular Knowledge Architecture (MKA) - Local Registry

## The Mission: Precision over Volume
Modern Large Language Models (LLMs) are capable of processing entire codebases, but **capability is not efficiency.** MKA is a surgical indexing protocol designed to bridge the gap between "reading everything" and "understanding what matters."

## Why MKA is Necessary
Even with massive context windows, AI-driven development faces critical challenges that MKA solves:

### 1. The "Lost in the Middle" Effect
When an AI is fed an entire project, the most important technical nuances often get lost in the noise. MKA provides a **High-Density Mini-Map** that forces the AI's attention onto the exact artifacts that define a feature's behavior.

### 2. Context-Aware Dynamic Scaling (Anti-Bloat)
One of the greatest wastes of tokens is "Instruction Overload"—forcing an AI to read naming conventions and build rules for tasks that don't require them. MKA implements **Just-in-Time Instructions** via **Procedures**. The AI is aware of "Trigger Points" and only pulls detailed project procedures into its active memory when the task explicitly requires them.

### 3. Documentation Decay (The Multi-Developer Problem)
Traditional documentation dies the moment "Developer B" modifies a feature written by "Developer A." MKA solves this by **decoupling the "What" from the "How."** 
- We document the **"Where"** (the files and methods).
- We leave the **"Logic"** (the conditions and branches) in the code.
- This ensures the AI always sees the current implementation truth, even if the internals change under the hood.

### 4. Eliminating Logical Redundancy
LLMs are native speakers of both English and Code. MKA removes "Translation Layer" bloat. We provide the entry points (the Nodes), and we let the AI's natural reasoning engine interpret the source code directly.

### 5. On-Demand Maintenance (Developer Mode vs. Librarian Mode)
To prevent annoying overhead, MKA does not require documentation updates during active development. The AI focuses 100% on code until you explicitly command it to **"Update Documentation."** Only then does the AI switch to "Librarian Mode" to synchronize the MKA registry.

---

## 🏗️ Project Procedures & Standards
Project-specific "House Rules" (building, publishing, naming conventions) are handled through a **Trigger-Based Reference System**.

### Why separate Procedures?
Instead of a single giant "Rules" file that the AI must read every turn, we split instructions into modular files. This keeps the AI's "working memory" clean for the actual code.

### When to use them?
- **Strict Rules:** Global, non-negotiable mandates are stored directly in `AI.md` under **Core Mandates**.
- **Case-Specific Procedures:** Instructions for specific tasks (e.g., "How to publish") are stored in the `Procedures/` directory.

### How it works:
1. The AI starts the session by reading `AI.md`.
2. `AI.md` contains **Triggers** (e.g., "When adding a database file").
3. The AI only reads the corresponding procedure file in `Procedures/` if its current task matches that trigger.

---

## 🚀 How to Bootstrap
Provide `.MKA/AI.md` as the primary context for the AI.

### Implementation Workflow:
1. **Develop:** The AI uses the MKA index to navigate but focuses entirely on implementation.
2. **Finalize:** Once code changes are done, tell the AI: **"Update the MKA documentation."**
3. **Review:** The AI will read `.MKA/Instructions.md` and surgically update the registry before you commit.

## Directory Structure
- `AI.md`: The central orchestrator (Must-read for AI).
- `.MKA/Procedures/MkaMaintenance.md`: The "Librarian's Manual" (Triggered only on-demand).
- `index.mka.yaml`: Master registry for Functional Workflows.
- `Workflows/`: Surgical YAML maps of execution paths.
- `Procedures/`: Narrative markdown files for case-specific project standards.

---
*MKA is an experimental project indexing system. It assumes that the code is the truth, and the documentation is the map.*
