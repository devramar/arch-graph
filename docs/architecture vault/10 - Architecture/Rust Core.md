# Rust Core

## Responsibility

The Rust core owns project scanning, `.archgraph` configuration, source discovery/parsing, layer construction, group composition, diagnostics, and configuration writes.

It does not depend on the desktop application or any programming language.

## Core operations

1. Validate and canonicalize the project root.
2. Read root `.archgraph` if present.
3. Normalize layers, markers, ignored paths, colour/view settings.
4. Walk the project tree without following symlinks.
5. Apply built-in ignores, Git ignores, `.archgraphignore`, and configured `ignored_paths`.
6. Match files against per-layer globs.
7. Skip files matching multiple layers with `ARCH007`.
8. Parse Markdown sources with the Markdown parser; parse all other source types with decorated-text semantics.
9. Ignore candidate sources that contain no markers.
10. Build one graph per layer.
11. Compose supplied layer groups with core-owned name matching.
12. Return graph/layers/configuration state to consumers.

## Source-language independence

The decorated parser recognizes a punctuation decoration prefix before configured markers, for example `///`, `#`, `--`, or `*`. It strips the same prefix from adjacent declaration/reference prose.

This is lexical annotation handling, not TypeScript/Rust/Python/etc. parsing.

## Configuration writes

Consumers submit typed `ProjectConfiguration` values to the core. The core validates and normalizes them, writes completed strict JSON to a temporary file, then replaces root `.archgraph`.

`view_settings` remains opaque application JSON. Rust transports it without understanding desktop-specific layer-group or layout keys.
