# Architecture Document Template

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

## Description

Short description of the system.

---

## Purpose

Why it exists.

---

## Architecture

Broad internal structure and important flows.

---

## References

ARCH_REFERENCE:Shared Concept

Explain how this architecture uses, interacts with, constrains, or otherwise refers to the shared concept.

ARCH_SUBREFERENCE:Local Concept

Explain a local architectural concern that should not merge with same-name nodes elsewhere.

---

## Invariants

- Important property that must remain true.

---

## Relevant Files

`file`
: Why it matters.
```

`## References` is recommended for readability only. The configured marker tokens are what ArchGraph parses.
