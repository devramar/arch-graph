# Decisions

A concise record of decisions already made.

## Documentation

`ARCHITECTURE.md` is used only for sufficiently structured systems.

Architecture documents use explicit searchable markers.

```text
ARCH_NODE:Name
ARCH_DEPENDENCY:Name
```

Dependency prose documents the edge.

## Graph model

Small modules may appear as graph nodes without having their own architecture document.

Reverse dependents are derived from incoming edges rather than declared separately.

The graph format is presentation-independent.

## Core

The scanner is a standalone Rust component.

The desktop application consumes the core rather than embedding architecture semantics in the UI.

## Desktop

Primary implementation:

- Tauri 2
- React
- TypeScript
- Cytoscape.js

The product is desktop-first and fully functional offline.

## Interaction

Graph panning should work with:

- left-mouse drag on graph space
- middle-mouse drag
- Space + drag

Edges have hover documentation and selectable detailed inspection.

## Parsing philosophy

Architecture relationships are intentionally authored.

Source parsing exists to resolve those declarations, not to replace them with a noisy automatically inferred import graph.
