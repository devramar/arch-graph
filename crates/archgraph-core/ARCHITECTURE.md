# ArchGraph Core

ARCH_NODE:ArchGraph Core

## Description

Standalone Rust engine that turns explicit architecture documentation and named references into a presentation-independent directed graph.

---
## Purpose

Own project scanning, `.archgraph` configuration, reference semantics, diagnostics, and graph construction independently of any UI or programming language.

---
## Intended Usage

Use `scan_project(root)` from Rust consumers, `scan_project_state(root)` when the consumer also needs project configuration, or consume the graph through `archgraph-cli` when JSON is preferable.

---
## Architecture

The scanner reads the root `.archgraph` configuration first, walks the confined project root for configured architecture document filenames, parses configured marker aliases, resolves ordinary references only against documented architecture-node names, and assembles the neutral graph.

Undocumented ordinary references become globally shared lightweight reference nodes. Subreferences always become source-local lightweight nodes and never merge or resolve by name.

---
## Invariants

- Architectural edges only originate from explicit reference/subreference markers.
- Source-code imports, exports, modules, and language syntax are never scanned for graph targets.
- An ordinary reference resolves to a documented architecture node only when exactly one matching `ARCH_NODE` exists.
- Missing ordinary targets are valid shared reference nodes, not diagnostics.
- Subreferences are always local to their declaration and never merge by name.
- Project traversal does not follow symlinks by default.
- `.archgraph` is root-scoped and parsed/validated by the core.
- The serialized graph remains independent of Cytoscape.js and Tauri.

---
## Relevant Files

`src/config.rs`
: `.archgraph` parsing, validation, defaults, and writes.

`src/model.rs`
: Neutral serialized graph contract.

`src/parser.rs`
: Configurable architecture marker parser.

`src/resolver.rs`
: Architecture/reference/subreference target resolution.

`src/scanner.rs`
: Filesystem traversal and graph assembly.
