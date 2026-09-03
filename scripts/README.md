# Build Scripts

Run build scripts from the repository root. They resolve their own paths, so the repository can live in a directory containing spaces.

## First-time setup

Install the frontend dependencies once:

```bash
cd apps/desktop
npm install
cd ../..
```

You also need Rust/Cargo and the Tauri 2 prerequisites for the platform you are building on.

## Recommended local workflow

Validate everything:

```bash
./scripts/check.sh
```

Build a release for the current machine:

```bash
./scripts/build.sh native
```

Build only the standalone Rust scanner and CLI:

```bash
./scripts/build.sh core
```

## Build targets

The general entry point is:

```text
./scripts/build.sh <target> [target options]
```

### Native

```bash
./scripts/build.sh native
```

Builds the Tauri application for the current host using its native target.

On Linux, the raw executable is normally:

```text
target/release/archgraph-desktop
```

Tauri may also create platform bundles under `target/release/bundle/`.

### Linux

```bash
./scripts/build.sh linux
./scripts/build.sh linux x86_64
./scripts/build.sh linux aarch64
```

The Linux helper requests `.deb`, `.rpm`, and AppImage bundles. A packaging tool may fail even after the release executable has built successfully; in that case, check the target release directory before assuming compilation failed.

Cross-architecture builds require the appropriate Rust target and system libraries.

### Windows

On a Windows host, use the native build:

```bash
./scripts/build.sh native
```

The cross-build helper is also available where `cargo-xwin` is configured:

```bash
./scripts/build.sh windows x86_64
./scripts/build.sh windows aarch64
```

It produces an NSIS bundle.

### macOS

Run these on a macOS host:

```bash
./scripts/build.sh macos native
./scripts/build.sh macos universal
./scripts/build.sh macos x86_64
./scripts/build.sh macos aarch64
```

The universal build requires both Apple Rust targets.

## Checks

```bash
./scripts/check.sh
```

This runs:

```text
cargo test --workspace
npm --prefix apps/desktop run build
```

If frontend dependencies are not installed, the Rust tests still run and the frontend build is skipped.

## Cleaning

Remove generated build output:

```bash
./scripts/clean.sh
```

Also remove `node_modules`:

```bash
./scripts/clean.sh --deps
```

## Linux local install

These scripts expect `target/release/archgraph-desktop` to already exist.

Install:

```bash
./scripts/install.sh
```

Update an existing installation after rebuilding:

```bash
./scripts/update.sh
```

Remove it:

```bash
./scripts/uninstall.sh
```

The installation is per-user under `~/.local` and does not require `sudo`.

## Application icon

Put the master icon at:

```text
assets/app-icon.png
```

A square 1024×1024 transparent PNG is recommended. A square SVG is also accepted.

Generate Tauri's platform-specific icon files with:

```bash
./scripts/generate-icons.sh
```

They are written to:

```text
apps/desktop/src-tauri/icons/
```
