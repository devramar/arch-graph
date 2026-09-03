# System Overview

## Purpose

Provide a fast visual way to understand the architecture of an unfamiliar or long-unvisited codebase.

The source of truth is a set of lightweight `ARCHITECTURE.md` files placed only in folders with enough structure to justify architectural documentation.

## Components

### Rust Core

See [[Rust Core]].

Scans a project directory, parses architecture documents, resolves named dependencies, and outputs a neutral graph representation.

### Desktop Application

See [[../30 - Desktop App/Desktop Application]].

Consumes the graph produced by the Rust core and renders it using Cytoscape.js.

## Separation

The Rust core must remain independently usable.

The desktop app is one consumer of its output, not the owner of the graph format.

```text
Project Folder
    ↓
Rust Core
    ↓
Architecture Graph
    ↓
Any Consumer
    ├─ Desktop App
    ├─ CLI tooling
    ├─ Web viewer
    ├─ Graphviz exporter
    └─ Other tools
```

## Important distinction

The architecture graph represents intentionally documented architectural relationships.

It is not an automatic import graph.
