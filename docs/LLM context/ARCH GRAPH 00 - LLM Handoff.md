# ARCHITECTURE.md System — LLM Handoff

This document explains the project's architectural documentation convention.

Use it whenever you are asked to:
- add or update architectural documentation;
- understand how a feature is structured;
- describe relationships between subsystems;
- create `ARCHITECTURE.md` files;
- review whether existing architecture docs match the source.

The intent is to keep architecture documentation **close to the code**, concise, searchable, and useful to both humans and graph tooling.

## Core concept

Think of the codebase as a directed graph:

- documented systems are **nodes**;
- architecturally meaningful dependencies are **directed edges**;
- the text attached to a dependency explains **why that edge exists**.

The documentation should describe the **current system**, not aspirations or historical designs.

## When an `ARCHITECTURE.md` should exist

Create one only when a folder contains enough interacting structure that a new or returning developer would need to read multiple files to understand what the subsystem does.

Good reasons include:

- several files cooperate as one conceptual system;
- there is a meaningful public/internal boundary;
- data or control flows through several stages;
- important invariants exist;
- relationships are not obvious from individual files or imports;
- the folder represents a distinct feature or subsystem.

Do **not** create one simply because a folder exists.

A concise single-file utility normally does not need its own `ARCHITECTURE.md`.

## Canonical node declaration

Every architecture document represents one canonical architecture node.

Declare it near the top:

```text
ARCH_NODE:Canonical Name
```

Example:

```text
ARCH_NODE:EventStore
```

The canonical name should closely match the terminology already used in the source.

Do not invent branding that is disconnected from the code.

## Dependencies

Architecturally meaningful dependencies are declared with:

```text
ARCH_DEPENDENCY:Dependency Name
```

Example:

```markdown
ARCH_DEPENDENCY:EventAPIBridge

Provides real data from the API, in which EventStore manages and handles access for.
Users of EventStore should not know about EventAPIBridge's existence.
```

The prose beneath the declaration describes the **relationship**, not the dependency itself.

### Dependency targets do not need their own architecture document

A target may be:

- another `ARCH_NODE`;
- a concise TypeScript module;
- an exported symbol;
- an external system or library;
- another resolvable architectural concept.

For example, `EventAPIBridge` may be a small one-file module and still appear as:

```text
ARCH_DEPENDENCY:EventAPIBridge
```

It does not need an `ARCHITECTURE.md` solely because other systems depend on it.

This is intentional.

## Edge direction

```text
EventStore ─────▶ EventAPIBridge
```

means:

> EventStore depends on EventAPIBridge.

Do not declare reverse "dependent of" relationships.

## Recommended document structure

Use this structure when the sections are relevant:

```markdown
# Canonical Name

ARCH_NODE:Canonical Name

## Description

Short explanation of what the system is and its major components.

---

## Purpose

Why this system exists and what responsibility it owns.

---

## Intended Usage

How callers are expected to use it.

Include public entry points, common operations, and small examples when useful.

---

## Architecture

How the important internal pieces interact.

Describe control flow, data flow, ownership, boundaries, and major internal roles.

---

## Dependencies

ARCH_DEPENDENCY:Dependency Name

Explain how and why this system depends on it.

ARCH_DEPENDENCY:Another Dependency

Explain that relationship.

---

## Invariants

Important assumptions that must remain true.

---

## Relevant Files

`SomeFile.ts`
: Brief explanation of its role.
```

Not every section needs to exist, and if it should exist, keeping it simple is preferred.

Prefer concise, precise documentation over boilerplate.

## What belongs in the document

Good architecture documentation answers questions such as:

- What is this subsystem responsible for?
- Why does it exist?
- What are its public entry points?
- What are the important internal pieces?
- How does data/control move through them?
- What other systems does it depend on?
- Why do those dependencies exist?
- What assumptions must remain true?
- Which source files are the key implementation points?

## What does not belong

Do not turn `ARCHITECTURE.md` into:

- exhaustive API documentation;
- a changelog;
- a TODO list;
- a backlog;
- historical design notes;
- duplicated inline comments;
- a list of every import;
- speculative future architecture.

Inline TSDoc explains individual APIs.

`ARCHITECTURE.md` explains how the system fits together.

## Dependency selection

Only declare dependencies that matter architecturally.

An import alone is not sufficient reason to create an edge.

Good dependency:

```text
ARCH_DEPENDENCY:EventStore
```

because synchronization relies on the event store as a system boundary.

Usually bad dependency:

```text
ARCH_DEPENDENCY:Array
```

because using a language/runtime primitive does not help explain the architecture.

The graph should remain useful rather than becoming an import graph.

## Relationship descriptions

Prefer descriptions that explain the role of the dependency.

Good:

> Provides the local event state against which synchronization results are reconciled.

Weak:

> EventStore uses EventAPIBridge.

Good:

> Converts UTC event timestamps using the user's configured timezone before presentation.

Weak:

> Used for timezone stuff.

## Invariants

Use invariants for assumptions that are important but may not be obvious from types.

Examples:

- `EventStore` should be the single source of truth for event information.
- UI components do not directly perform persistence operations.

Do not invent invariants that cannot be supported by the source.

## Relevant Files

List only files that materially help a reader understand the subsystem.

Example:

```markdown
## Relevant Files

`event-sync.ts`
: Main synchronization coordinator.

`event-store.ts`
: Owns local persistence and retrieval.

`sync-window.ts`
: Computes the range of days considered by a synchronization operation.
```

Do not list every file simply for completeness.

## Source-grounding rules for LLMs

When creating or updating architecture documentation:

1. Read the relevant source first.
2. Identify the actual public entry points and major internal roles.
3. Trace meaningful data/control flow.
4. Identify architectural dependencies from real usage.
5. Preserve existing names whenever possible.
6. Do not fabricate responsibilities, invariants, or relationships.
7. Describe the current implementation.
8. If something is unclear from the source, say so rather than guessing.
9. Keep descriptions concise enough to remain maintainable.
10. Prefer updating an existing architecture node over creating overlapping nodes.

## Scope rule

Documentation should generally live at the **smallest folder scope that represents a coherent subsystem**.

Examples:

```text
features/
    events/
        feed/
            ARCHITECTURE.md
        bridge/
            ARCHITECTURE.md
        sync/
            ARCHITECTURE.md
```

if each folder contains a distinct multi-file subsystem.

Do not create one giant top-level document if that would hide useful boundaries.

Likewise, do not split one coherent subsystem into multiple architecture nodes merely because it has several files.

## Reviewing an existing `ARCHITECTURE.md`

Check that:

- `ARCH_NODE:` matches the actual subsystem;
- the description reflects current source;
- intended usage matches real public APIs;
- architecture describes real interactions;
- each `ARCH_DEPENDENCY:` is meaningful and real;
- each dependency description explains the relationship;
- invariants are actually true;
- relevant files still exist;
- obsolete implementation details have been removed.

## Graph-tooling compatibility

The architecture graph tooling parses:

```text
ARCH_NODE:Name
ARCH_DEPENDENCY:Name
```

The marker names and canonical dependency names should therefore remain stable and globally searchable.

Dependency descriptions are treated as documentation on graph edges.

A dependency target can resolve to a documented architecture node or to a smaller code module/symbol without its own architecture document.

## Minimal example

```markdown
# Event Synchronization

ARCH_NODE:EventStore

## Description

Coordinates synchronization of changed remote event days with locally stored event data.

---

## Purpose

Keeps local event state current while allowing consumers to work against stable local representations.

---

## Intended Usage

Consumers invoke the synchronization coordinator rather than reconciling remote and local state directly.

---

## Architecture

The coordinator receives changed day information, determines which local days require refresh, fetches required event data, and reconciles the result through the event store.

---

## Dependencies

ARCH_DEPENDENCY:EventAPIBridge

Provides the canonical day representation used to construct synchronization windows and identify changed days.

ARCH_DEPENDENCY:EventStore

Provides the local event state against which synchronization results are reconciled.

---

## Invariants

- Synchronization uses canonical EventAPIBridges for day identity.
- Callers do not directly mutate synchronization persistence.

---

## Relevant Files

`EventStore.ts`
: Main synchronization coordinator.
```

## Final authoring principle

A good `ARCHITECTURE.md` should let someone open a folder after months away and quickly answer:

> What is this system, why does it exist, how do its important pieces interact, and what does it rely on?

If the document does that without forcing the reader through unnecessary detail, it is doing its job.

See also:
- [[10 - Authoring Checklist]]
- [[20 - Template]]
- [[30 - Example]]
