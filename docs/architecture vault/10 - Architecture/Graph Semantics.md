# Graph Semantics

## Architecture nodes

A configured node marker establishes a canonical documented node:

```text
ARCH_NODE:EventSync
```

## References

```text
ARCH_REFERENCE:EventStore
```

creates a directed edge from the current architecture node to the explicit name `EventStore`.

If exactly one `ARCH_NODE:EventStore` exists, the edge targets that architecture node. Otherwise, when no document exists, ArchGraph creates one globally shared lightweight `EventStore` reference node.

ArchGraph does not search imports, exports, modules, packages, ASTs, or language-server metadata to resolve the name.

## Subreferences

```text
ARCH_SUBREFERENCE:Password Management
```

creates a local satellite reference node attached only to the declaring architecture node.

Same-name subreferences remain distinct graph identities:

```text
Service A → Password Management #1
Service B → Password Management #2
```

They may share visual styling, but they never merge and never resolve to `ARCH_NODE:Password Management`.

## Direction

References are directional. Reverse relationships are derived from incoming edges rather than authored separately.

## Progressive documentation

A shared reference can exist before it has architecture documentation. Adding a unique matching `ARCH_NODE` later automatically converts future scans to target that architecture node without rewriting the existing reference declarations.
