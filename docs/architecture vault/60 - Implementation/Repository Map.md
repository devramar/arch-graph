# Repository Map

## Rust core

`crates/archgraph-core`
: Owns `.archgraph`, source discovery, parser dispatch, per-layer scans, explicit reference resolution, group composition, diagnostics, and graph schema.

Important files:

- `src/config.rs` — layers, globs, ignored paths, markers, app configuration, writes
- `src/model.rs` — graph-v3, declaration, layer, and composition contracts
- `src/parser.rs` — Markdown/decorated-text parser dispatch
- `src/resolver.rs` — per-layer reference/subreference resolution
- `src/scanner.rs` — traversal, simple glob matcher, layer scans, group composition

## CLI

`crates/archgraph-cli`
: Thin standalone consumer that scans one root and emits the default all-layer graph JSON.

## Desktop

`apps/desktop`
: React/Cytoscape frontend with layer-group UI, regions, declaration tabs, search/filter/layout interaction.

`apps/desktop/src-tauri`
: Thin Tauri adapter exposing scan, compose, configuration-write, and source-open commands.

## Fixtures

`crates/archgraph-core/tests/fixtures/basic`
: Proves canonical Markdown declarations and that ordinary `.ts` files remain irrelevant without an implementation layer.

`crates/archgraph-core/tests/fixtures/aliases`
: Proves layers, file globs, per-layer marker aliases, ignored paths, decorated TypeScript annotations, cross-layer name merging, colours, and view configuration.
