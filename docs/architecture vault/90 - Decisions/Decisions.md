# Decisions

## Explicit markers remain the semantic contract

Canonical markers are:

```text
ARCH_NODE:Name
ARCH_REFERENCE:Name
ARCH_SUBREFERENCE:Name
```

Markdown headings such as `## References` are human convention only.

## Language independence

ArchGraph does not parse imports, exports, symbols, ASTs, or type systems.

File globs discover candidates. Markdown has a dedicated parser; all other file types use generic punctuation decoration-prefix parsing. This keeps source annotations language-agnostic while leaving parser dispatch extensible.

## One source, one declaration

A participating source may declare at most one architecture node in this version. Multiple node markers are diagnostic rather than implicitly creating subgraphs.

## Layers

A project may expose several logical architecture layers. Each source belongs to exactly one layer; overlap is diagnostic rather than guessed.

The `architecture` layer is always present and defaults to `ARCHITECTURE.md` plus canonical markers.

## Composition

Same-name declarations merge only inside one layer group. Different groups form independent namespaces and may be rendered simultaneously.

Merged nodes preserve every declaration instead of selecting one source as canonical.

## Reference semantics

Normal references resolve by explicit name inside their composition group. Missing targets are valid shared reference nodes.

Subreferences remain source-local, never merge, and never resolve by name.

## Configuration and persistence

One optional root `.archgraph` strict-JSON file controls ignored paths, layers, discovery globs, marker vocabularies, colour overrides, default view, and opaque app view settings.

Without `.archgraph`, desktop session changes are not persisted. The app may explicitly create the file through the Rust core.

Nested configuration inheritance is deferred.

## Graph model

Graph format version `3` adds composition groups and multiple declaration provenance per architecture node.

The graph remains presentation-independent.

## Desktop visuals

Enabled layer groups render as separate soft regions. Reference colours are deterministic from names unless overridden. Edge arrows use destination primary colours. Subreferences are smaller dashed satellites with shorter force-layout distances.
