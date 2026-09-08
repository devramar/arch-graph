# ArchGraph Core

ARCH_NODE:ArchGraph Core

## Description

Standalone Rust engine that discovers explicit architecture declarations, organizes them into logical layers, composes selected layer groups, and emits a presentation-independent directed graph.

---
## Purpose

Own project scanning, `.archgraph` configuration, source discovery, declaration parsing, reference semantics, layer composition, diagnostics, and graph construction independently of any UI or programming language.

---
## Intended Usage

Use `scan_project(root)` for the default all-layer composition, `scan_project_state(root)` when a consumer needs independently scanned layers/configuration, and `compose_project_layers(...)` when an app needs custom layer groups without rescanning the filesystem.

---
## Architecture

The scanner reads root `.archgraph`, applies built-in/Git/custom ignored paths, matches source candidates against per-layer file globs, and assigns each matched source to exactly one layer.

Markdown sources use the Markdown parser. Every other source type uses the language-agnostic decorated-text parser. Candidate files without markers are ignored.

Each layer is scanned independently. Composition then merges same-name architecture declarations only inside explicitly supplied layer groups. Different groups remain independent even when names match.

Undocumented ordinary references become shared lightweight reference nodes. Subreferences always become source-local lightweight nodes and never merge or resolve by name.

---
## References

ARCH_REFERENCE:ArchGraph Desktop

Consumes independently scanned layers and asks the core to compose desktop layer groups rather than reimplementing merge semantics in TypeScript.

---
## Invariants

- Architectural edges only originate from explicit reference/subreference markers.
- Candidate source globs never imply graph membership without ArchGraph markers.
- Source-code imports, exports, modules, and language syntax are never parsed for graph meaning.
- One source file may declare at most one architecture node.
- A source matching multiple layers is skipped rather than assigned arbitrarily.
- Same-name declarations merge only inside the same composition group.
- Missing ordinary targets are valid shared reference nodes, not diagnostics.
- Subreferences are always local to their declaration and never merge by name.
- Project traversal does not follow symlinks by default.
- `.archgraph` is root-scoped and parsed/validated/written by the core.
- The serialized graph remains independent of Cytoscape.js and Tauri.

---
## Relevant Files

`src/config.rs`
: `.archgraph` layers, ignored paths, markers, app settings, validation, and writes.

`src/model.rs`
: Neutral graph-v3, declaration, layer, and composition contracts.

`src/parser.rs`
: Extensible source-parser dispatch with Markdown and decorated-text parsers.

`src/resolver.rs`
: Per-layer architecture/reference/subreference target resolution.

`src/scanner.rs`
: Filesystem traversal, glob matching, layer scans, and core-owned group composition.
