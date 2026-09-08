# MVP Scope

## Required

- standalone Rust scanning/composition core
- neutral schema-v3 graph model
- JSON serialization
- optional root `.archgraph`
- root-scoped logical layers
- per-layer file globs and marker aliases
- ignored-path globs
- Markdown source parser
- generic decorated-text source parser
- one source → at most one node declaration
- independently scanned layer graphs
- core-owned layer-group composition
- multi-declaration merged architecture nodes
- explicit shared references
- explicit non-merging local subreferences
- no programming-language semantic parsing
- Rust-owned `.archgraph` read/validate/write path
- desktop drag/drop project opening
- desktop drag/drop layer grouping
- simultaneous visible group regions
- inspector tabs for merged declarations
- session-only mode without `.archgraph`
- persisted desktop grouping/layout when `.archgraph` exists
- deterministic reference colours and overrides
- destination-coloured edge arrows
- search, filters, diagnostics, source opening, fully offline normal operation

## Explicitly not required

- inferred import edges
- source-language AST/type-system integration
- multiple architecture nodes in one source file
- nested `.archgraph` inheritance
- regex discovery rules
- language-specific parsers beyond generic Markdown/decorated dispatch
- editable palette definitions
- full layer-definition settings editor
- live filesystem watching
- Git integration
- hosted web service

## Later possibilities

- dedicated parsers for additional source/document formats
- multiple declarations per source file
- richer layer settings editor
- palette extension/replacement configuration
- graph export/import
- filesystem watching/incremental rescan
- persisted manual node positions
