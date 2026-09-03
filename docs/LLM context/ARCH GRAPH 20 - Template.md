# ARCHITECTURE.md Template

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

## Description

Short explanation of what this system is and its major components.

---

## Purpose

Why this system exists and what responsibility it owns.

---

## Intended Usage

Describe the supported way callers interact with the system.

Include important public entry points and small examples where useful.

---

## Architecture

Describe the important internal components and how data/control moves between them.

---

## Dependencies

ARCH_DEPENDENCY:Dependency Name

Explain how and why this system depends on the target.

ARCH_DEPENDENCY:Another Dependency

Explain that relationship.

---

## Invariants

- Important assumption that must remain true.
- Another important architectural assumption.

---

## Relevant Files

`File.ts`
: Role of this file.

`AnotherFile.ts`
: Role of this file.
```

Sections may be omitted when they genuinely add no value.

Do not fill sections with boilerplate merely to preserve the template.
