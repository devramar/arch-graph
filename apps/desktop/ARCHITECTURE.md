# ArchGraph Desktop

ARCH_NODE:ArchGraph Desktop

## Description

Desktop-first Tauri application for interactively exploring graphs produced by ArchGraph Core.

---
## Purpose

Provide a local PC-optimized graph explorer with project drag/drop, search, filtering, reference hover documentation, source opening, configurable layouts, and detailed inspection.

---
## Intended Usage

Drop or open one project root, then explore its generated graph. The frontend does not directly traverse or modify the project filesystem.

---
## Architecture

The Tauri adapter invokes the standalone Rust core and returns both its neutral graph and normalized `.archgraph` project configuration. React translates that graph into Cytoscape.js elements and owns presentation and interaction only. Architecture-node records include their raw architecture document so the inspector can display documentation without direct filesystem reads.

The desktop uses a deterministic built-in reference colour palette. `.archgraph` may override colour names separately for shared references and subreferences. Edge arrows use the destination node primary colour. Local subreferences render as smaller dashed satellite nodes and use shorter force-layout distances than ordinary references.

---
## References

ARCH_REFERENCE:ArchGraph Core

Provides scanning, `.archgraph` parsing/writing, reference resolution, diagnostics, and the canonical graph/configuration data returned to the frontend.

ARCH_REFERENCE:ArchitectureGraph

The frontend consumes the neutral graph contract rather than embedding scanner semantics in Cytoscape-specific structures.

---
## Invariants

- Normal operation is offline.
- The webview does not receive generic filesystem traversal or write APIs.
- Project configuration writes go through the Rust core.
- Left background drag, middle drag, and Space+drag all pan the graph.
- Normal direct node drag remains available.
- Source opening is confined to files beneath the selected project root.
- Architecture document viewing consumes content already present in the core graph model.
- Reference descriptions remain edge-owned documentation.
- Same-name subreferences remain distinct graph nodes while sharing deterministic visual colour identity.

---
## Relevant Files

`src/App.tsx`
: Desktop UI orchestration and project loading.

`src/components/GraphCanvas.tsx`
: Cytoscape rendering, colours, layouts, and desktop graph interaction.

`src/components/Inspector.tsx`
: Node/reference inspection, source actions, and architecture-document presentation.

`src/components/MarkdownDocument.tsx`
: Small safe Markdown-oriented architecture document renderer.

`src-tauri/src/lib.rs`
: Thin Tauri adapter around ArchGraph Core, including configuration writes.
