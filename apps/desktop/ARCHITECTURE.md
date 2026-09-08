# ArchGraph Desktop

ARCH_NODE:ArchGraph Desktop

## Description

Desktop-first Tauri application for interactively exploring layered graphs produced by ArchGraph Core.

---
## Purpose

Provide a local PC-optimized graph explorer with project drag/drop, layer grouping, simultaneous graph regions, search, filtering, reference documentation, source opening, configurable layouts, and multi-source inspection.

---
## Intended Usage

Drop or open one project root, then arrange discovered layers into visible groups. Layers inside one group merge matching node names; different enabled groups remain visible simultaneously as separate regions.

The frontend does not directly traverse or modify the project filesystem.

---
## Architecture

The Tauri adapter invokes ArchGraph Core and receives independently scanned layers, normalized configuration, configuration-presence state, and a default graph. React owns session/group UI but asks Rust to compose layer groups so graph semantics remain core-owned.

Merged architecture nodes preserve every declaration. The inspector exposes declaration tabs, allowing architecture-level and implementation-level sources to describe the same node without discarding either source.

The desktop uses deterministic reference colours. `.archgraph` may override colour names separately for references and subreferences. Edge arrows use the destination node primary colour. Local subreferences render as smaller dashed satellite nodes and use shorter force-layout distances than normal references.

Enabled layer groups render as soft labeled graph regions. Separate groups are packed apart while remaining visible in the same canvas.

---
## References

ARCH_REFERENCE:ArchGraph Core

Provides scanning, `.archgraph` parsing/writing, layer discovery, composition semantics, diagnostics, and canonical graph/configuration data.

ARCH_REFERENCE:ArchitectureGraph

The frontend consumes the neutral graph contract rather than embedding scanner or merge semantics in Cytoscape-specific structures.

---
## Invariants

- Normal operation is offline.
- The webview does not receive generic filesystem traversal or write APIs.
- Project configuration writes go through the Rust core.
- Without a root `.archgraph`, desktop group/layout changes are session-only.
- Creating `.archgraph` promotes the current session state into persisted desktop view settings.
- Several layer groups may remain enabled and visible simultaneously.
- Same-name declarations merge only within one group.
- Merged nodes retain all declaration sources and expose them as inspector tabs.
- Source opening is confined to files beneath the selected project root.
- Reference descriptions remain edge-owned documentation.
- Same-name subreferences remain distinct graph nodes while sharing deterministic visual colour identity.

---
## Relevant Files

`src/App.tsx`
: Project loading, group composition requests, session persistence, and top-level UI state.

`src/components/Sidebar.tsx`
: Layer-group drag/drop, enable/disable controls, config-persistence state, filters, diagnostics.

`src/components/GraphCanvas.tsx`
: Cytoscape rendering, destination colours, group regions, layouts, and graph interaction.

`src/components/Inspector.tsx`
: Node/reference inspection and multi-declaration tabs.

`src/components/MarkdownDocument.tsx`
: Safe Markdown-oriented declaration renderer.

`src-tauri/src/lib.rs`
: Thin adapter around scan, compose, configuration-write, and root-confined source-open operations.
