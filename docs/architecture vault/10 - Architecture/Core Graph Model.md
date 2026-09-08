# Core Graph Model

## Goal

The core graph format is independent of Cytoscape.js, Tauri, React, source languages, and presentation choices.

## Graph

```ts
interface ArchitectureGraph {
    version: 2;
    project: ProjectInfo;
    nodes: ArchitectureNode[];
    edges: ArchitectureEdge[];
    diagnostics: Diagnostic[];
}
```

## Node kinds

### Architecture

A documented subsystem declared by a configured architecture-node marker, canonically `ARCH_NODE:`.

Architecture nodes may include the raw architecture document for consumers such as the desktop inspector.

### Reference

A lightweight explicitly named concept with no unique architecture document target.

Reference nodes have one of two scopes:

- `shared` — created by a normal reference and merged globally by explicit name.
- `local` — created by a subreference; unique to the declaring architecture node and never merged or resolved by name.

A missing architecture document is normal for reference nodes and does not produce an unresolved diagnostic.

## Edges

Every edge is an explicitly authored reference and has a `referenceKind`:

- `reference`
- `subreference`

The edge description belongs to the relationship, not to the destination node.

## Resolution

For a normal reference:

1. exactly one matching architecture node → connect to it;
2. no matching architecture node → connect to a shared lightweight reference node;
3. multiple matching architecture nodes → connect to a shared reference node and emit an ambiguity diagnostic.

For a subreference:

- always create a source-local reference node;
- never resolve to an architecture node;
- never merge with another subreference, even when names match.

## Diagnostics

Diagnostics describe malformed or genuinely ambiguous architecture data. Missing reference documents are not errors.

Current codes:

- `ARCH001` duplicate architecture-node name
- `ARCH002` ambiguous reference caused by duplicate architecture-node names
- `ARCH004` architecture document without a node marker
- `ARCH005` malformed/empty marker
