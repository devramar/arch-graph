# Graph Semantics

## Declarations

A configured node marker establishes a declaration:

```text
ARCH_NODE:EventSync
```

A source file may declare at most one node in this version.

## References

```text
ARCH_REFERENCE:EventStore
```

creates a directed edge from the current declaration to explicit name `EventStore`.

Inside one composition group:

1. a unique architecture target with that name → connect to it;
2. no architecture target → connect to one shared lightweight reference node;
3. ambiguous architecture declarations → use the shared reference node and emit a diagnostic.

ArchGraph never uses imports, exports, modules, packages, ASTs, or language-server metadata to resolve names.

## Subreferences

```text
ARCH_SUBREFERENCE:Password Management
```

creates a source-local satellite node. Same-name subreferences share visual identity but never graph identity.

## Layers

Layers are independently discovered architecture representations of one project, such as:

- Architecture
- Implementation
- Overview
- Infrastructure

A file belongs to exactly one layer. Candidate globs only enroll a source when ArchGraph markers are actually present.

## Composition groups

Layers can be composed together. Same-name architecture declarations merge only inside the same group.

```text
Architecture + Implementation   -> DateKey (merged, 2 declarations)
Overview                        -> DateKey (separate node)
```

Several groups can be visible simultaneously.

## Progressive documentation

A shared reference can exist before any matching declaration exists. Adding a declaration later—or composing in another layer that declares the name—allows the reference to resolve without rewriting the original marker.
