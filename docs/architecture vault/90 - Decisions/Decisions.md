# Decisions

## Documentation vocabulary

Canonical markers are:

```text
ARCH_NODE:Name
ARCH_REFERENCE:Name
ARCH_SUBREFERENCE:Name
```

`## References` is recommended Markdown structure but does not gate marker recognition; Markdown boundaries only delimit adjacent prose.

## Reference semantics

Normal references merge by explicit name when undocumented and resolve to a unique matching architecture node when documented.

Subreferences are source-local, never merge, and never resolve by name.

Missing architecture documents are normal reference-node states rather than unresolved errors.

## Language independence

ArchGraph does not parse programming-language imports, exports, modules, symbols, ASTs, or type systems. Explicit architecture documents are the only semantic input.

## Configuration

One optional root `.archgraph` strict-JSON file controls accepted filename/marker aliases, reference/subreference colour overrides, default view, and opaque app view settings.

Alias lists replace defaults. Nested configuration inheritance is deferred.

The Rust core owns configuration parsing, validation, normalization, and writes.

## Graph model

Graph format version 2 contains architecture/reference node kinds, shared/local reference scopes, and reference/subreference edge kinds.

The graph remains presentation-independent.

## Desktop visuals

Reference colours are deterministic from names unless overridden. Same-name subreferences therefore share visual identity while retaining separate node IDs.

Edge arrows use the destination node primary colour.

Subreferences use smaller dashed ellipse styling and shorter force-layout distances.
