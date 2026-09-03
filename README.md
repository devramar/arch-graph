# ArchGraph

ArchGraph is an offline desktop tool for exploring a codebase as an architecture graph.

Projects describe important systems with lightweight `ARCHITECTURE.md` files. ArchGraph scans those documents, resolves their declared dependencies, and renders the result as an interactive graph.

```text
ARCH_NODE:EventSync
ARCH_DEPENDENCY:DateKey
```

The Rust scanner is standalone and produces a neutral graph model. The desktop app is a Tauri + React + Cytoscape.js viewer for that model.

## Quick start

### 1. Requirements

You need:

- Rust with `cargo`
- Node.js 22+
- npm
- the normal Tauri 2 prerequisites for your operating system

Linux additionally needs the WebKitGTK/GTK development packages required by Tauri.

### 2. Install frontend dependencies

From the repository root:

```bash
cd apps/desktop
npm install
cd ../..
```

### 3. Validate the project

```bash
./scripts/check.sh
```

This runs the Rust workspace tests and the frontend production build.

### 4. Run in development

```bash
cd apps/desktop
npm run tauri dev
```

### 5. Build a release

For the current machine:

```bash
./scripts/build.sh native
```

The native Linux executable is produced at:

```text
target/release/archgraph-desktop
```

Platform-specific builds are also available:

```bash
./scripts/build.sh linux x86_64
./scripts/build.sh windows x86_64
./scripts/build.sh macos universal
./scripts/build.sh core
```

See [`scripts/README.md`](scripts/README.md) for the full build guide.

## Try ArchGraph on this repository

This repository contains its own `ARCHITECTURE.md` files, so it can be used as a sample project.

1. Build or run ArchGraph.
2. Open the desktop app.
3. Drag the **repository root folder** into the window.
4. Explore the resulting architecture graph.

See [`EXAMPLE_USAGE.md`](EXAMPLE_USAGE.md) for a complete clone → build → scan walkthrough.

## Linux local installation

After `./scripts/build.sh native`:

```bash
./scripts/install.sh
```

After rebuilding a newer version:

```bash
./scripts/update.sh
```

To remove the installed copy:

```bash
./scripts/uninstall.sh
```

These scripts install only for the current user under `~/.local` and do not require `sudo`.

## Documentation

- [`EXAMPLE_USAGE.md`](EXAMPLE_USAGE.md) — first-use walkthrough
- [`scripts/README.md`](scripts/README.md) — build targets and scripts
- [`RUNNING.md`](RUNNING.md) — development and runtime notes
- [`docs/00 - Home.md`](docs/00%20-%20Home.md) — architecture documentation index
- [`docs/20 - Documentation/ARCHITECTURE Format.md`](docs/20%20-%20Documentation/ARCHITECTURE%20Format.md) — how to document a project for ArchGraph
