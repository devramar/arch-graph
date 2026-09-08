# ArchGraph

ArchGraph is an offline desktop application and Rust core for exploring explicitly authored architecture as a graph.

ArchGraph does not infer architecture from imports, exports, ASTs, package manifests, or any particular programming language. A project declares named architecture nodes and explains directional references between them. The graph is built only from those declarations.

```text
ARCH_NODE:EventStore

Owns durable event state and coordinates persistence boundaries.

ARCH_REFERENCE:Event Database

EventStore reads and writes persisted event records through Event Database.

ARCH_SUBREFERENCE:Retention Policy

EventStore applies this local retention concern without merging it with same-named subreferences elsewhere.
```

A reference description belongs to the source node and explains how **this** node interacts with the referenced concept. A normal reference can converge on a matching declared node. A subreference is always local to its source declaration and never merges by name.

## How ArchGraph reads a repository

ArchGraph scans one or more logical **layers**. Each layer defines a project-relative `path_root`, source file globs, optional ignored paths, and marker aliases.

The built-in `architecture` layer defaults to `ARCHITECTURE.md` files using `ARCH_NODE`, `ARCH_REFERENCE`, and `ARCH_SUBREFERENCE`.

Markdown files are parsed as Markdown architecture sources. Other configured file types use a language-agnostic decorated-text parser, so architecture can sit beside implementation without ArchGraph understanding the surrounding language:

```ts
/// ARCH_NODE:DateKey
///
/// Represents canonical YYYY-MM-DD date keys.
///
/// ARCH_REFERENCE:Clock
/// DateKey creation uses Clock when deriving the current date.
export type DateKey = `${number}-${number}-${number}`;
```

A file glob only makes a file a candidate. Files with no ArchGraph markers are ignored. One source file can declare one architecture node in the current format.

Layers are scanned independently and can be composed into desktop groups. Same-name declarations merge only when their layers are in the same group; merged nodes retain all contributing declarations and expose them as tabs in the inspector. Multiple groups can remain visible simultaneously.

## `.archgraph`

A root `.archgraph` file is optional strict JSON. Without it, ArchGraph uses its built-in architecture layer and desktop grouping/layout changes remain session-only.

This repository uses two architecture layers with the same `ARCHITECTURE.md` convention but different filesystem boundaries:

```json
{
  "layers": {
    "architecture": {
      "display_name": "Core Architecture",
      "path_root": "crates",
      "ignored_paths": ["archgraph-core/tests/fixtures/**"],
      "files": ["ARCHITECTURE.md"]
    },
    "desktop": {
      "display_name": "Desktop Architecture",
      "path_root": "apps/desktop",
      "files": ["ARCHITECTURE.md"]
    }
  }
}
```

`path_root` is a literal directory beneath the selected project root. A layer's `files` and `ignored_paths` globs are evaluated relative to that directory. Top-level `ignored_paths` remain project-root-relative. Graph source locations are always reported relative to the project root.

Marker names, reference/subreference colour overrides, default view, and application-specific `view_settings` are also configurable. See [`docs/extended_examples.md`](docs/extended_examples.md) for larger configurations and source-annotation patterns.

## Build prerequisites

ArchGraph uses a Rust workspace and a Tauri 2 + React desktop application. A development machine needs:

- Rust and Cargo;
- Node.js 22 or newer and npm;
- the normal Tauri 2 platform prerequisites for the host operating system.

Install frontend dependencies once:

```bash
cd apps/desktop
npm install
cd ../..
```

Run the repository checks:

```bash
./scripts/check.sh
```

`check.sh` runs the Rust workspace tests and, when `apps/desktop/node_modules` exists, the frontend TypeScript/Vite build.

For the stricter Rust gate used during development:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Run locally

Start the Tauri development application from the repository root with:

```bash
cd apps/desktop
npm run tauri dev
```

The browser-only Vite preview can be started with `npm run dev`, but filesystem scanning and native source opening require the Tauri runtime.

## Build

The build scripts are run from the repository root and resolve their own paths.

Build the desktop application for the current host:

```bash
./scripts/build.sh native
```

Build only the Rust core and CLI:

```bash
./scripts/build.sh core
```

Platform helpers are also available:

```bash
./scripts/build.sh linux [x86_64|aarch64]
./scripts/build.sh windows [x86_64|aarch64]
./scripts/build.sh macos [native|universal|x86_64|aarch64]
```

Cross-platform helpers require the corresponding Rust targets, system libraries, and packaging tools. Tauri bundles are written under the Cargo `target` directory.

## Local Linux installation

The repository includes per-user Linux install scripts. Build the native release first:

```bash
./scripts/build.sh native
./scripts/install.sh
```

This installs the ArchGraph binary and desktop entry under `~/.local` without `sudo`.

After rebuilding, update the installed binary with:

```bash
./scripts/update.sh
```

Remove the local installation with:

```bash
./scripts/uninstall.sh
```

Generated build output can be removed with `./scripts/clean.sh`; `./scripts/clean.sh --deps` also removes frontend dependencies.

## Repository map

- `crates/archgraph-core` — scanning, configuration, parsing, layer composition, graph semantics, and diagnostics.
- `crates/archgraph-cli` — standalone JSON graph consumer.
- `apps/desktop` — Tauri, React, and Cytoscape desktop application.
- `schema` — serialized graph and `.archgraph` JSON schemas.
- `docs/extended_examples.md` — larger `.archgraph` and source-integration examples.
- `docs/llm-documentation-context` — a self-contained folder intended to be zipped and supplied to an LLM when asking it to author ArchGraph documentation for another repository.
