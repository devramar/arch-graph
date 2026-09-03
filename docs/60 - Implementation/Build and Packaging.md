# Build and Packaging

## Entry points

Repository build automation lives in [[../../scripts/README|scripts/README.md]].

The general dispatcher is:

```bash
./scripts/build.sh <target>
```

Supported target families are `native`, `linux`, `windows`, `macos`, and `core`.

## Native builds

Prefer native host builds for release artifacts when practical. Tauri desktop bundles depend on platform-native webview and packaging systems.

## Windows cross-builds

The provided Windows helper follows Tauri's `cargo-xwin` route and targets the MSVC Rust ABI. This is a convenience route rather than the preferred release environment.

## Application icon

The canonical editable source belongs at:

```text
assets/app-icon.png
```

Use a square 1024x1024 transparent PNG where practical. A square SVG is also accepted.

Generate the complete Tauri icon suite with:

```bash
./scripts/generate-icons.sh
```

Generated outputs live in `apps/desktop/src-tauri/icons/` and are referenced by the existing Tauri bundle configuration.
