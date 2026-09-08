# MVP Scope

## Required

- standalone Rust scanning core
- neutral schema-v2 graph model
- JSON serialization
- optional root `.archgraph` configuration
- configurable architecture document filenames
- configurable node/reference/subreference marker aliases
- Rust-owned `.archgraph` read/validate/write path
- explicit shared references
- explicit non-merging local subreferences
- no programming-language source resolution
- desktop Tauri application
- drag/drop project root and folder picker
- reference descriptions preserved on edges
- duplicate/ambiguous architecture diagnostics
- Cytoscape rendering
- deterministic reference colours and project colour overrides
- destination-coloured edge arrows
- subreference visual/layout distinction
- search, filters, inspector, and fully offline normal operation

## Explicitly not required

- inferred import edges
- source-language AST/type-system integration
- nested `.archgraph` inheritance
- editable palette definitions in `.archgraph`
- full settings UI
- live filesystem watching
- Git integration
- hosted web service

## Later possibilities

- settings editor using the existing configuration-write command
- palette extension/replacement configuration
- graph export/import
- filesystem watching and incremental rescan
- persisted manual node positions
