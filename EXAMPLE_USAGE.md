# Example Usage: Scan ArchGraph with ArchGraph

This repository documents its own major systems, making it a useful first project to open in ArchGraph.

## Build and run

```bash
cd apps/desktop
npm install
cd ../..
./scripts/check.sh
cd apps/desktop
npm run tauri dev
```

## Open this repository

Drag the repository root into ArchGraph. The graph should include documented architecture nodes for the Rust core and desktop application. Select an architecture node to read its document; select or hover an edge to read its reference description.

A normal reference without a matching architecture document is shown as a lightweight reference node rather than an unresolved error. Source code with a matching symbol name is intentionally ignored.

A subreference is displayed as a smaller dashed local node and never merges with another node of the same name.

## Minimal document

```markdown
# Event Synchronization

ARCH_NODE:EventSync

## References

ARCH_REFERENCE:EventStore

Uses the event store as the local source of persisted event state.

ARCH_SUBREFERENCE:Password Management

Password handling is intentionally represented only as a local concern of this architecture node.
```

`## References` is a recommended human-facing heading, but it does not gate recognition. Explicit markers are recognized wherever they appear; Markdown section boundaries are used only to keep adjacent prose from being attached to the wrong reference.

## Custom vocabulary

A project can use a root `.archgraph` file to replace filenames and marker spellings. For example, `DOCUMENTATION.md`, `SYS_NODE`, and `SYS_REF` can be used without changing the graph semantics.

See the architecture-format and project-configuration documents under `docs/architecture vault/20 - Documentation/`.
