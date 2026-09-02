# Core Graph Model

## Goal

The core graph format must be independent of Cytoscape.js, Tauri, React, and any other presentation layer.

## Project

Conceptually:

```ts
interface ArchitectureGraph
{
    version: number;
    project: ProjectInfo;
    nodes: ArchitectureNode[];
    edges: ArchitectureEdge[];
    diagnostics: Diagnostic[];
}
```

This is illustrative rather than a frozen schema.

## Node kinds

### Architecture

A documented subsystem declared by an `ARCH_NODE:` marker.

### Module

A concise code module or exported symbol that is useful as a dependency target but does not need its own architecture document.

Example:

```text
DateKey
```

### External

A dependency outside the scanned project.

Examples may include frameworks, libraries, or external services.

### Unresolved

A named dependency that could not be resolved uniquely.

Unresolved nodes are preserved so the graph remains inspectable even when documentation contains errors.

## Edge

An edge represents an intentionally documented dependency.

Conceptually:

```ts
interface ArchitectureEdge
{
    source: NodeId;
    target: NodeId;

    description?: string;

    sourceFile: string;
    sourceLine?: number;
}
```

The description belongs to the relationship.

For example:

```text
EventSync ──uses DateKey to define sync windows──▶ DateKey
```

## Diagnostics

Diagnostics should report issues without preventing all useful graph output.

Examples:

- ambiguous dependency
- unresolved dependency
- duplicate architecture node name
- malformed marker
