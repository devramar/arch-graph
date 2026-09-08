# System Overview

## Purpose

Provide a fast visual way to understand intentionally documented architecture in a repository or other project tree.

The default source of truth is a set of lightweight `ARCHITECTURE.md` files, with optional root `.archgraph` configuration to replace document names, marker vocabulary, app colour overrides, default view, and app-specific view settings.

## Components

### Rust Core

Scans configured architecture documents, resolves explicit named references, owns project configuration, and outputs a neutral graph.

### Desktop Application

Consumes the graph and normalized project configuration and renders them using Cytoscape.js.

## Separation

```text
Project Folder
    ↓
Rust Core
    ├─ Architecture Graph
    └─ Project Configuration
            ↓
        Any Consumer
        ├─ Desktop App
        ├─ CLI/tooling
        └─ Future viewers/exporters
```

## Important distinction

ArchGraph represents explicitly authored references. It is not an import graph and does not need support for any repository programming language.
