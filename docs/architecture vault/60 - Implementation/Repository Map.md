# Repository Map

## Rust core

`crates/archgraph-core`
: Owns root `.archgraph` configuration, architecture document discovery/parsing, explicit reference resolution, diagnostics, and the neutral graph schema.

Important files:

- `src/config.rs` — configuration defaults, parsing, validation, and writes
- `src/model.rs` — serialized graph-v2 contract
- `src/parser.rs` — configurable marker and edge-description parser
- `src/resolver.rs` — architecture/shared/local reference resolution
- `src/scanner.rs` — root-confined project scan and graph assembly

## CLI

`crates/archgraph-cli`
: Thin standalone consumer that scans one root and emits graph JSON.

## Desktop

`apps/desktop`
: React/Cytoscape frontend.

`apps/desktop/src-tauri`
: Thin Tauri adapter exposing project scan, configuration write, and source-open commands.

## Fixtures

`crates/archgraph-core/tests/fixtures/basic`
: Proves normal references, architecture resolution, subreferences, and that matching TypeScript files are ignored.

`crates/archgraph-core/tests/fixtures/aliases`
: Proves root `.archgraph` filename/marker aliases plus configuration transport.
