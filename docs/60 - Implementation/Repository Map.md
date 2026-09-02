# Repository Map

## Rust core

`crates/archgraph-core`
: Owns project walking, `ARCHITECTURE.md` parsing, TypeScript symbol indexing, resolution, diagnostics, and the neutral graph schema.

Important files:

- `src/model.rs` — serialized graph contract
- `src/parser.rs` — architecture marker and edge-description parser
- `src/resolver.rs` — TypeScript module symbol index and target resolution
- `src/scanner.rs` — root-confined project scan and graph assembly

## CLI

`crates/archgraph-cli`
: Thin standalone consumer that scans one root and emits graph JSON to stdout.

## Desktop

`apps/desktop`
: React/Cytoscape frontend.

`apps/desktop/src-tauri`
: Thin Tauri adapter exposing the core scanner through one `scan_project` command and the native folder dialog.

The frontend does not directly enumerate the filesystem.

## Fixtures

`crates/archgraph-core/tests/fixtures/basic`
: Small project fixture covering architecture nodes, module targets, relationship descriptions, and an unresolved dependency.
