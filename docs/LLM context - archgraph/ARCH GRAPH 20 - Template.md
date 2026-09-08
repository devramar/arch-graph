# ArchGraph Source Templates

## Markdown architecture source

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

## Description

Short description of the system or concept.

## References

ARCH_REFERENCE:Shared Concept

Explain how this node uses, interacts with, constrains, or otherwise refers to the shared concept.

ARCH_SUBREFERENCE:Local Concept

Explain a local concern that should remain attached only to this node.
```

`## References` is recommended for readability only. Configured markers control parsing.

## Decorated implementation source

```ts
/// ARCH_NODE:Canonical Name
///
/// Short architecture-level description of this implementation concept.
///
/// ARCH_REFERENCE:Shared Concept
/// Explain the relationship.
///
/// ARCH_SUBREFERENCE:Local Concept
/// Explain the local concern.
export class CanonicalName {
  // implementation
}
```

ArchGraph does not interpret `export class` or TypeScript syntax. It only discovers a configured candidate file and extracts the consistently decorated architecture block.

One source currently supports one architecture node.
