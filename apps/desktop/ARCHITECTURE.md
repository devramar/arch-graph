# ArchGraph Desktop

ARCH_NODE:ArchGraph Desktop

## Description

Desktop-first Tauri application for interactively exploring graphs produced by ArchGraph Core.

---
## Purpose

Provide a local PC-optimized graph explorer with project drag/drop, search, filtering, relationship hover documentation, source opening, force-directed settling, and detailed inspection.

---
## Intended Usage

Drop or open one project root, then explore its generated graph. The frontend does not directly traverse the project filesystem.

---
## Architecture

The Tauri adapter invokes the standalone Rust core and returns its neutral graph. React translates that model into Cytoscape.js elements and owns presentation and interaction only. Architecture-node graph records include their raw architecture document so the inspector can display documentation without obtaining direct filesystem read access.

---
## Dependencies

ARCH_DEPENDENCY:ArchGraph Core

Provides scanning, dependency resolution, diagnostics, and the canonical graph data returned to the frontend.

ARCH_DEPENDENCY:ArchitectureGraph

The frontend consumes the neutral graph contract rather than embedding scanner semantics in Cytoscape-specific structures.

---
## Invariants

- Normal operation is offline.
- The webview does not receive generic filesystem traversal APIs.
- Left background drag, middle drag, and Space+drag all pan the graph.
- Normal direct node drag remains available.
- Source opening is confined to files beneath the selected project root and delegates to the system default application through Tauri's opener integration.
- Architecture document viewing consumes document content already present in the core graph model; the frontend does not read source files itself.
- Relationship descriptions remain edge-owned documentation and may be surfaced at the cursor, in the inspector, or in the graph description panel.

---
## Relevant Files

`src/App.tsx`
: Desktop UI orchestration and project loading.

`src/components/GraphCanvas.tsx`
: Cytoscape rendering and desktop graph interaction.

`src/components/Inspector.tsx`
: Node/relationship inspection, source actions, and architecture-document presentation.

`src/components/MarkdownDocument.tsx`
: Small safe Markdown-oriented architecture document renderer.

`src-tauri/src/lib.rs`
: Thin Tauri adapter around ArchGraph Core.
