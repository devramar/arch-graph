# Core Graph Model

## Goal

The core graph format is independent of Cytoscape.js, Tauri, React, source languages, and presentation choices.

Graph format version `3` adds declaration provenance and layer-composition groups.

```ts
interface ArchitectureGraph {
    version: 3;
    project: ProjectInfo;
    groups: GraphGroup[];
    nodes: ArchitectureNode[];
    edges: ArchitectureEdge[];
    diagnostics: Diagnostic[];
}
```

## Architecture declarations

An architecture node may be backed by one or more declarations:

```ts
interface ArchitectureDeclaration {
    layerId: string;
    layerName: string;
    source: SourceLocation;
    documentation?: string;
    sourceFormat: 'markdown' | 'decoratedText';
}
```

A merged node therefore retains every contributing source rather than selecting one winner.

## Node kinds

### Architecture

A named architectural concept established by an explicit configured node marker.

Within one composition group, matching names from different layers merge when each contributing layer has a unique declaration for that name.

### Reference

A lightweight explicitly named concept with no unique architecture target in its group.

Reference scopes:

- `shared` — normal references merge by explicit name inside the group.
- `local` — subreferences remain unique to their declaring source.

Missing architecture declarations are normal for shared reference nodes.

## Groups

Every graph node/edge belongs to a composition `groupId`.

Layers inside one group participate in name matching. Different groups are isolated semantic namespaces and may be rendered simultaneously.

## Edges

Every edge is explicitly authored and has:

- `referenceKind`: `reference` or `subreference`
- `layerId`: layer containing the source declaration
- `groupId`: composition group in which the edge was materialized
- edge-owned description and source location

## Diagnostics

Current codes:

- `ARCH001` duplicate architecture-node name inside one layer
- `ARCH002` ambiguous reference
- `ARCH004` marker-bearing source without a node marker
- `ARCH005` malformed/empty marker
- `ARCH006` more than one architecture node marker in one source
- `ARCH007` source matched more than one configured layer
