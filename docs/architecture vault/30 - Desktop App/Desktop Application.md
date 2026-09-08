# Desktop Application

## Purpose

Provide a PC-optimized interactive explorer for architecture graphs produced by the Rust core.

## Stack

- Tauri 2
- React
- TypeScript
- Cytoscape.js

## Primary workflow

1. Select or drop a project root.
2. Tauri asks the Rust core to scan it.
3. Receive `{ graph, configuration }`.
4. Convert graph nodes/edges into Cytoscape elements.
5. Apply project colour/view configuration.
6. Explore the graph.

## Responsibilities

The desktop owns visualization, layouts, search, filters, hover previews, inspectors, desktop interaction, and interpretation of desktop-specific `view_settings`.

It does not own project parsing or reference-resolution semantics.

## Reference styling

Shared references use deterministic colours derived from their explicit names unless overridden by `.archgraph`.

Local subreferences use the same deterministic name-colour rule but remain distinct graph node identities. They render as smaller dashed ellipses and sit closer to their declaring architecture node in force layouts.

Every edge arrow uses the primary colour of its destination node.

## Filesystem access

The frontend does not receive generic project read/write APIs. Source opening and `.archgraph` writes are narrow Tauri commands backed by validation in Rust.
