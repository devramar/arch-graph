# MVP Scope

## Required

- standalone Rust scanning core
- neutral graph model
- JSON serialization
- desktop Tauri application
- drag/drop project root
- folder picker
- recursive project scan
- locate `ARCHITECTURE.md`
- parse `ARCH_NODE:`
- parse `ARCH_DEPENDENCY:`
- preserve dependency descriptions
- resolve architecture-to-architecture dependencies
- resolve useful TypeScript module/export targets
- diagnostics for unresolved and ambiguous dependencies
- Cytoscape.js graph rendering
- edge hover descriptions
- selectable nodes and edges
- right-side inspector
- search
- node-kind filters
- automatic graph layout
- fully offline normal operation

## Explicitly not required for MVP

- editing project files
- automatically generating architecture edges from every import
- full TypeScript language-server semantics
- live filesystem watching
- Git integration
- web-hosted version
- collaboration features
- telemetry

## Later possibilities

- graph export
- Mermaid export
- Graphviz export
- graph snapshots
- filesystem watching
- editor deep-links
- CLI commands
- standalone web viewer consuming pre-generated graph JSON
