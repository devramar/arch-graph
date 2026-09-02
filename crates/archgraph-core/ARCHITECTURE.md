# ArchGraph Core

ARCH_NODE:ArchGraph Core

## Description

Standalone Rust engine that turns explicit architecture documentation and resolvable source modules into a presentation-independent directed graph.

---
## Purpose

Own all project scanning and graph semantics independently of any UI or desktop shell.

---
## Intended Usage

Use `scan_project(root)` from Rust consumers, or consume the same core through `archgraph-cli` when a JSON representation is preferable.

---
## Architecture

The scanner walks a confined project root, parses architecture documents, indexes useful TypeScript exports, resolves explicitly authored dependencies, and assembles nodes, edges, and diagnostics.

---
## Invariants

- Architectural edges only originate from explicit `ARCH_DEPENDENCY:` declarations.
- Project traversal does not follow symlinks by default.
- Ambiguous targets are reported rather than guessed.
- The serialized model remains independent of Cytoscape.js and Tauri.

---
## Relevant Files

`src/model.rs`
: Neutral serialized graph contract.

`src/parser.rs`
: Architecture document parser.

`src/resolver.rs`
: Lightweight TypeScript export indexing and dependency resolution.

`src/scanner.rs`
: Filesystem traversal and graph assembly.
