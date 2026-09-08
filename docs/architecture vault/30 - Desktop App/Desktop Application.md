# Desktop Application

## Purpose

Provide a PC-optimized interactive explorer for layered architecture graphs produced by the Rust core.

## Stack

- Tauri 2
- React
- TypeScript
- Cytoscape.js

## Primary workflow

1. Select or drop a project root.
2. Tauri asks the Rust core to scan it.
3. Receive independently scanned layers, default composed graph, diagnostics, normalized configuration, and whether `.archgraph` exists.
4. Restore desktop layer groups from `view_settings` when available; otherwise create one visible group per layer.
5. Ask Rust to compose enabled groups.
6. Render groups as separate soft regions in one Cytoscape canvas.
7. Drag layers between groups to change name-merging boundaries.

## Responsibilities

The desktop owns visualization, layout UI, search, filters, drag/drop grouping, inspector tabs, and interpretation of desktop-specific `view_settings`.

It does not own scanning or layer-composition semantics.

## Session persistence

If `.archgraph` exists, layer groups and the active layout are persisted after user changes.

If `.archgraph` is absent, changes are session-only. The sidebar can create `.archgraph` using the current session state.

## Reference styling

Shared references use deterministic colours derived from explicit names unless overridden.

Local subreferences are distinct IDs, smaller dashed ellipses, and shorter/stronger force-layout satellites.

Every edge arrow uses the primary colour of its destination node.

## Filesystem access

The frontend does not receive generic project read/write APIs. Source opening and `.archgraph` writes are narrow Tauri commands backed by Rust validation.
