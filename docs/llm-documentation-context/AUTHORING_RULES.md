# ArchGraph Authoring Rules

## Architecture nodes

An architecture source declares one named node with the layer's configured node marker, canonically:

```text
ARCH_NODE:Canonical Name
```

One source file may currently declare only one node.

Give the node a concise description that explains what the node is and its architectural responsibility. In Markdown, place this description directly after the node declaration before deeper structural sections where practical.

Architecture documentation should describe the target system itself: responsibilities, boundaries, important structures, data flow, invariants, and architectural relationships. Avoid turning `ARCHITECTURE.md` into an ArchGraph tutorial, build guide, changelog, implementation diary, or handover document.

## References are directional relationships

A normal reference is canonically:

```text
ARCH_REFERENCE:Target Name

Relationship description.
```

The relationship description belongs to the **source** node. It must explain how **this node** interacts with, uses, constrains, delegates to, receives from, exposes to, or otherwise relates to the target.

Good:

```text
ARCH_REFERENCE:Identity

Checkout asks Identity for the current customer before constructing an order.
```

Weak:

```text
ARCH_REFERENCE:Identity

Identity manages users and authentication.
```

The weak version describes the target instead of the source-to-target relationship. The target's own architecture declaration should explain what the target is.

A normal reference is mergeable by explicit name. When a unique same-name architecture declaration exists in the current composition group, the edge targets it. Otherwise ArchGraph creates a valid shared lightweight reference node. Missing declarations are not errors.

## Subreferences are local

A local subreference is canonically:

```text
ARCH_SUBREFERENCE:Local Concern

Relationship description.
```

A subreference always belongs only to its source declaration. It never resolves to an architecture declaration and never merges with same-named subreferences elsewhere.

Use subreferences for local architectural concerns whose repeated names should not imply a shared graph concept.

`ARCH_SUBREFERENCE` does not mean "child of the preceding reference".

## Markdown headings are non-semantic

Markers carry ArchGraph semantics. Headings such as `## References` are a readable convention, not parser control flow. Organize Markdown for humans without depending on headings for marker recognition.

## Decorated source declarations

Non-Markdown candidate files use decoration-prefix parsing. A marker must be the first meaningful content after a punctuation-only prefix, and adjacent documentation must keep that prefix:

```ts
/// ARCH_NODE:DateKey
///
/// Represents canonical YYYY-MM-DD date keys.
///
/// ARCH_REFERENCE:Clock
/// DateKey creation uses Clock when deriving the current date.
```

The surrounding programming language is irrelevant to ArchGraph. Do not infer nodes or references from classes, interfaces, imports, exports, symbols, packages, or ASTs.

## Layers and duplicate names

Layers are independent source collections. Same-name declarations can intentionally exist in different layers to describe one concept at different abstraction levels. They merge only when those layers are composed into the same group.

Do not rename a deliberate node merely because another layer contains the same name. Multiple declarations can be valuable because the desktop preserves them as separate inspector tabs.

Within a single layer, duplicate architecture declarations with the same name are ambiguous and should normally be avoided.

## Scope and quality

Prefer architecture-level information over incidental implementation detail. References should be useful even when viewed without source code. Keep names stable and descriptions concise enough to scan, while including enough structure for a reader to understand the node's role and interactions.
