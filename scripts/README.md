# Build Scripts

All scripts resolve paths relative to the repository, so spaces in the checkout path are supported.

## Common commands

```bash
./scripts/check.sh
./scripts/build.sh core
./scripts/build.sh native
```

## Linux

Build for the host architecture:

```bash
./scripts/build.sh linux
```

Or select the Rust target architecture:

```bash
./scripts/build.sh linux x86_64
./scripts/build.sh linux aarch64
```

Linux bundles require the normal Tauri/WebKitGTK packaging dependencies for the build host and target architecture.

## Windows

Native Windows builds can always use:

```bash
./scripts/build.sh native
```

For Linux/macOS cross-build environments, the Windows script follows Tauri's `cargo-xwin` path and emits an NSIS bundle:

```bash
./scripts/build.sh windows x86_64
```

This requires `cargo-xwin` and the additional Windows cross-compilation prerequisites described by Tauri. Cross-building should be treated as a convenience path; a Windows host or CI runner is the preferred release environment when available.

## macOS

Run on a macOS host:

```bash
./scripts/build.sh macos native
./scripts/build.sh macos universal
./scripts/build.sh macos aarch64
./scripts/build.sh macos x86_64
```

The universal build requires both Apple Rust targets to be installed.

## Application icon

Put the master icon at:

```text
assets/app-icon.png
```

Recommended source: square **1024x1024 PNG with transparency**. Tauri also accepts a square SVG.

Then run:

```bash
./scripts/generate-icons.sh
```

The generated platform-specific files are written to `apps/desktop/src-tauri/icons/`, which is already referenced by `tauri.conf.json`.
