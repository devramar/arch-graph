# ArchGraph Documentation Templates

## Markdown architecture source

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

Concise description of the node and its architectural responsibility.

## Boundary

High-level responsibilities, owned state, important structures, or invariants that belong to this node.

## Data flow

High-level information about how data or control moves through this node when relevant.

## References

ARCH_REFERENCE:Shared Concept

Explain how this node interacts with Shared Concept. Describe the relationship from this node's perspective rather than explaining Shared Concept itself.

ARCH_SUBREFERENCE:Local Concern

Explain how this local concern relates to this node and why it should remain source-local rather than merge by name.
```

Sections are illustrative. Use headings that fit the target system. `## References` is a human-readable convention only; configured marker tokens are what ArchGraph parses.

## Decorated source declaration

```ts
/// ARCH_NODE:Canonical Name
///
/// Concise architecture-level description of this implementation concept.
///
/// ARCH_REFERENCE:Shared Concept
/// Explain how this source concept interacts with Shared Concept.
///
/// ARCH_SUBREFERENCE:Local Concern
/// Explain the source-local architectural concern.
export class CanonicalName {
    // implementation
}
```

Preserve the file's natural comment decoration. The marker and following documentation lines must share the same decoration prefix.

Do not add programming-language constructs solely for ArchGraph. The declaration block is sufficient.
