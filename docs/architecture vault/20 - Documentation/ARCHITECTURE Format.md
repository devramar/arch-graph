# Architecture Document Format

## When to create one

Create an architecture document only when a folder contains enough interacting structure that understanding it benefits from broad architectural explanation.

By default the filename is `ARCHITECTURE.md`; root `.archgraph` configuration can replace that accepted filename list.

## Canonical markers

```text
ARCH_NODE:Canonical Name
ARCH_REFERENCE:Shared Reference Name
ARCH_SUBREFERENCE:Local Reference Name
```

Configured aliases can replace these spellings.

## Recommended structure

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

## Description

Short explanation of the system.

---

## Architecture

Important internal components and relationships.

---

## References

ARCH_REFERENCE:Another System

Explanation of how and why this architecture references that system.

ARCH_SUBREFERENCE:Password Management

Explanation of a concept that should remain local to this architecture node.

---

## Invariants

Important assumptions that must remain true.
```

`## References` is a human-facing convention and does not gate recognition. ArchGraph recognizes configured marker tokens wherever they appear; Markdown section boundaries are used only to delimit adjacent prose.

## Reference descriptions

The prose beneath a reference marker belongs to the edge and should explain the interaction or reason for the reference rather than merely restating its name.

## Shared references

A normal reference merges by explicit name when no architecture document exists. If a unique matching architecture node is later added, the edge resolves to it automatically.

## Subreferences

A subreference is scoped to the declaring architecture node. It is not a child of the previous reference. It never merges or resolves by name, even when another node or architecture document has the same name.
