# Modular Knowledge Architecture (MKA)

A standardized protocol for LLM-ready documentation designed to solve the "Context Inflation" problem through surgical, indexed retrieval.

## Why MKA?
Traditional documentation is often too verbose or too vague for AI agents, leading to wasted context and hallucinated implementation details. MKA replaces monolithic context files with a three-layer surgical pattern.

## The Architecture
1.  **Bootstrap (`AI.md`)**: The AI entry point that redirects agents to the functional index.
2.  **Functional Index (`.ProjInfo/WorkFlows.md`)**: A central registry mapping features to their technical specifications.
3.  **Surgical Detail (`.ProjInfo/Workflows/*.md`)**: Technical documentation anchored to specific code artifacts (files, methods, data flows).

## Standards
- **Zero Generics**: Avoid vague descriptions; use explicit artifact references.
- **Artifact Anchors**: Mandatory inclusion of full file paths and symbols.
- **Data Flow**: Step-by-step tracing from trigger to execution.

## Getting Started
To implement MKA in your project, copy the `.ProjInfo` directory structure and the `AI.md` bootstrap file to your root directory.

---
*MKA v1.0 - Apache License 2.0*
