# Rust Core

## Responsibility

The Rust core owns project scanning and graph construction.

It must not depend on the desktop application.

## Inputs

Primary input:

- a project root directory

The core scans within that root for relevant architecture documents and source files.

## Outputs

Primary output:

- a neutral architecture graph data structure

The same data should be serializable as JSON so any external program can consume it.

See [[Core Graph Model]].

## Core operations

1. Walk the project tree.
2. Find `ARCHITECTURE.md` files.
3. Parse `ARCH_NODE:` declarations.
4. Parse `ARCH_DEPENDENCY:` declarations and their descriptions.
5. Index resolvable TypeScript modules and exported symbols.
6. Resolve dependency names.
7. Produce graph nodes, edges, and diagnostics.

## Filesystem boundaries

The scanner receives one explicit root.

It must not intentionally read outside that root.

Symlinks should not be followed by default.

## TypeScript resolution

For the first version, TypeScript parsing exists to resolve explicitly documented dependencies such as:

```text
ARCH_DEPENDENCY:DateKey
```

It should not automatically convert every TypeScript import into an architecture edge.

## Standalone use

A future CLI should be able to expose the same engine, for example:

```text
archgraph scan ./project --json
```

The exact CLI syntax is not yet fixed.
