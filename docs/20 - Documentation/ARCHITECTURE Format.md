# ARCHITECTURE Format

## When to create one

Create an `ARCHITECTURE.md` only when a folder contains enough interacting structure that understanding it requires more than reading one obvious file.

Good reasons include:

- several files cooperate as one conceptual system
- data or control flows through multiple stages
- a public/internal boundary exists
- important architectural invariants exist
- relationships are not obvious from imports alone

Do not create one merely because a folder exists.

## Recommended structure

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

## Description

Short explanation of the system.

---

## Purpose

Why it exists.

---

## Intended Usage

Supported entry points, public APIs, normal usage, and small examples.

---

## Architecture

Important internal components and their relationships.

---

## Dependencies

ARCH_DEPENDENCY:Dependency Name

Explanation of how and why this system depends on it.

ARCH_DEPENDENCY:Another Dependency

Explanation of that relationship.

---

## Invariants

Important assumptions that must remain true.

---

## Relevant Files

Short index of important implementation files.
```

## Scope

`ARCHITECTURE.md` explains system shape and relationships.

It should not attempt to replace:

- inline TypeScript/TSDoc
- generated API documentation
- changelogs
- TODO tracking
- exhaustive implementation notes

## Relationship descriptions

Dependency descriptions should explain the architectural relationship rather than merely restating the target name.

Good:

> Used as the canonical day representation when constructing synchronization windows.

Weak:

> This feature uses DateKey.
