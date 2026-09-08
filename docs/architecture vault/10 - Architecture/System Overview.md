# System Overview

ArchGraph turns explicit architectural declarations and named references into a navigable graph.

The default project has one logical `architecture` layer scanning `ARCHITECTURE.md`. Optional root `.archgraph` configuration can add layers, file globs, marker vocabularies, ignored paths, app colour overrides, and view settings.

```text
Project files
   ↓
source discovery by layer
   ↓
Markdown / decorated-text parsing
   ↓
independent layer graphs
   ↓
core-owned layer-group composition
   ↓
ArchitectureGraph v3
   ↓
CLI / Tauri desktop / future consumers
```

The architecture is intentionally language-agnostic. Source files participate only through explicit configured markers.
