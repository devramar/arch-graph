# Desktop Application

## Purpose

Provide a PC-optimized interactive explorer for architecture graphs produced by the Rust core.

## Stack

Planned stack:

- Tauri 2
- React
- TypeScript
- Cytoscape.js

## Primary workflow

1. Launch application.
2. Drag a project root folder into the window, or choose a folder.
3. Pass the selected root to the Rust core.
4. Receive the neutral graph model.
5. Convert that model into Cytoscape elements.
6. Explore the graph.

## Responsibilities

The desktop app owns:

- visualization
- graph layouts
- search
- filters
- hover previews
- inspectors
- desktop interaction
- display of diagnostics

It does not own project parsing semantics.

## Filesystem access

The frontend should not receive generic unrestricted filesystem APIs.

The desktop shell should expose narrow operations that invoke the Rust core against a user-selected root.

## Offline behavior

The complete normal workflow must function without network access.
