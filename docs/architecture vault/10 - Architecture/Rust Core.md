# Rust Core

## Responsibility

The Rust core owns project scanning, `.archgraph` configuration, architecture-document parsing, graph construction, diagnostics, and project-configuration writes.

It does not depend on the desktop application or any programming language.

## Core operations

1. Validate and canonicalize the selected project root.
2. Read root `.archgraph` if present.
3. Normalize and validate aliases/configuration.
4. Walk the project tree without following symlinks.
5. Find configured architecture-document filenames.
6. Parse configured node/reference/subreference markers.
7. Resolve normal references only against documented architecture-node names.
8. Create shared or local lightweight reference nodes where appropriate.
9. Produce graph nodes, edges, and diagnostics.
10. Return normalized configuration to consumers that request project state.

## Source-language independence

The core intentionally does not parse TypeScript, Rust, Python, imports, exports, symbols, packages, or ASTs. Source files are irrelevant unless they are architecture documents under a configured filename.

## Configuration writes

Consumers can submit a typed `ProjectConfiguration` to the core. The core validates and canonicalizes it, serializes strict JSON, writes a completed temporary file, and replaces the root `.archgraph` file.

`view_settings` is transported as application-defined JSON values; the core does not interpret layout-specific keys.
