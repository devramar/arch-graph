# Graph Semantics

## Architectural nodes

Any documented subsystem may declare:

```text
ARCH_NODE:EventSync
```

This creates a canonical architecture node.

## Lightweight module nodes

A dependency target does not need its own `ARCH_NODE:` declaration.

For example:

```text
ARCH_DEPENDENCY:DateKey
```

may resolve directly to a TypeScript module or exported symbol named `DateKey`.

This allows concise utilities to participate in the graph without forcing unnecessary documentation.

## Dependencies

A declaration:

```text
ARCH_DEPENDENCY:DateKey
```

creates a directed edge from the current architecture node to `DateKey`.

The prose immediately associated with that declaration describes the edge.

## Name resolution

Bare names should be the normal form:

```text
ARCH_DEPENDENCY:DateKey
```

When a name is ambiguous, a more explicit target may be used later, for example:

```text
ARCH_DEPENDENCY:@/core/date/DateKey
```

or:

```text
ARCH_DEPENDENCY:@/core/date/DateKey#DateKey
```

The exact qualified syntax can be finalized during implementation.

## Ambiguity

The resolver must not silently guess when multiple targets match.

Ambiguous dependencies should remain visible in the graph and produce a diagnostic.

## Direction

Dependencies are directional.

```text
EventSync ─────▶ DateKey
```

means `EventSync` depends on `DateKey`.

Reverse dependents can be calculated from incoming edges and do not need duplicate declarations.
