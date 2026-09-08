# Architecture Declaration Format

## Canonical markers

```text
ARCH_NODE:<name>
ARCH_REFERENCE:<name>
ARCH_SUBREFERENCE:<name>
```

`## References` is a recommended human-facing heading for Markdown, but headings do not define graph semantics. Configured marker tokens do.

## One source, one node

A participating source file may declare at most one node in this version. Additional node markers produce `ARCH006`; the first valid declaration is retained.

## Markdown sources

`.md` and `.markdown` files receive Markdown-specific parsing. Markers may appear directly in the document:

```markdown
ARCH_NODE:EventSync

Event synchronization coordinates remote reconciliation.

ARCH_REFERENCE:EventStore

Uses persisted local event state during reconciliation.
```

Dedicated Markdown declaration sources may contain arbitrary surrounding documentation. The complete Markdown source is preserved for the inspector.

A matching Markdown file with no ArchGraph markers is ignored silently.

## Decorated sources

All other file types use a language-agnostic decoration-prefix parser.

```ts
/// ARCH_NODE:DateKey
///
/// Handles dates in string form "YYYY-MM-DD".
///
/// ARCH_REFERENCE:Clock
/// Used when determining today.
export type DateKey = `${number}-${number}-${number}`;
```

ArchGraph discovers the punctuation prefix (`///` here), strips that same prefix from adjacent description lines, and stops the description when the decoration block ends.

Other examples include:

```text
# ARCH_NODE:ShellTask
-- ARCH_NODE:SqlRepository
* ARCH_NODE:BlockCommentNode
```

ArchGraph does not know which language those decorations belong to.

Markers inside ordinary code/string text are not recognized because the prefix before the marker must be a punctuation-only decoration.

## Candidate discovery

File globs in `.archgraph` only select candidates. A candidate with no markers contributes nothing to the graph and produces no diagnostic.

A source containing references/subreferences but no node marker produces `ARCH004`.

A source matching more than one configured layer is skipped with `ARCH007`.

## Normal references

`ARCH_REFERENCE:Name` creates an edge owned by the declaring source. Within one composition group, it resolves to a unique matching architecture node if one exists; otherwise it uses a shared lightweight reference node.

## Subreferences

`ARCH_SUBREFERENCE:Name` always creates a local lightweight node attached only to its declaring source. It never merges or resolves by name, even when another declaration or subreference has the same name.

Same-name subreferences may share deterministic visual styling in applications.
