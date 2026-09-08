# ArchGraph

ArchGraph is an offline desktop tool for exploring explicitly authored architecture as a graph.

Its core rule is simple:

> ArchGraph understands explicit architecture declarations and references, not programming languages.

By default, ArchGraph scans `ARCHITECTURE.md` files for:

```text
ARCH_NODE:EventSync
ARCH_REFERENCE:EventStore
ARCH_SUBREFERENCE:Password Management
```

A normal reference connects to a matching declared architecture node when one exists. Otherwise it becomes a valid lightweight shared reference node. A subreference is always local to its declaring node and never merges by name.

A root `.archgraph` file can add logical **layers**, file globs, marker aliases, ignored paths, colour overrides, and app-specific view settings. This lets a repository describe both broad architecture and implementation-level concepts without ArchGraph needing TypeScript/Rust/Python/etc. support.

## Sources and layers

Every matching file is only a **candidate**. A candidate with no ArchGraph markers is ignored silently.

Markdown files (`.md` / `.markdown`) use the Markdown parser. Other file types use the decorated-text parser. For example, an implementation layer can enrol `*.ts` files while only annotated files become graph nodes:

```ts
/// ARCH_NODE:DateKey
///
/// Handles dates in string form "YYYY-MM-DD".
///
/// ARCH_REFERENCE:Clock
/// Used when determining today.
export type DateKey = `${number}-${number}-${number}`;
```

The decoration prefix is discovered lexically (`///`, `#`, `--`, `*`, etc.). ArchGraph does not parse the surrounding language.

One source file may declare at most one `ARCH_NODE` in this version.

## Quick start

### Requirements

- Rust with `cargo`
- Node.js 22+
- npm
- normal Tauri 2 prerequisites for your operating system

Linux additionally needs the WebKitGTK/GTK development packages required by Tauri.

### Install and validate

```bash
cd apps/desktop
npm install
cd ../..
./scripts/check.sh
```

### Run

```bash
cd apps/desktop
npm run tauri dev
```

### Build

```bash
./scripts/build.sh native
```

See [`scripts/README.md`](scripts/README.md) for platform-specific build commands.

## `.archgraph`

`.archgraph` is optional strict JSON. Without it, ArchGraph uses the built-in `architecture` layer and the desktop treats view/group changes as session-only state.

```json
{
  "ignored_paths": [
    "crates/archgraph-core/tests/fixtures/**",
    "**/generated/**"
  ],
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "files": ["DOCUMENTATION.md"],
      "markers": {
        "ARCH_NODE": ["SYS_NODE"],
        "ARCH_REFERENCE": ["SYS_REF", "ARCH_REF"],
        "ARCH_SUBREFERENCE": ["SYS_LOCAL"]
      }
    },
    "implementation": {
      "display_name": "Implementation",
      "files": ["*.ts", "*.tsx"]
    }
  },
  "app_colours": {
    "colour_overrides": {
      "references": {
        "Identity": "purple"
      },
      "subreferences": {
        "Password Management": "teal"
      }
    }
  },
  "default_view": "sticky",
  "view_settings": {
    "sticky": {
      "reference_distance": 175,
      "subreference_distance": 68,
      "subreference_attraction": 1.8
    }
  }
}
```

The `architecture` layer always exists. If it is omitted from `.archgraph`, canonical `ARCHITECTURE.md` discovery is restored automatically. Additional layers must declare at least one file glob.

File patterns support simple path globs: `*`, `?`, and `**`. A source matching more than one layer is skipped with `ARCH007` rather than being assigned arbitrarily.

`ignored_paths` combines with ArchGraph's built-in exclusions, Git ignore rules, and `.archgraphignore`.

`view_settings` is intentionally application-defined data transported by the Rust core. The desktop currently understands layout tuning plus its persisted layer-group session state.

## Desktop layer groups

The desktop can show several layer groups simultaneously. Each enabled group gets a soft graph region.

- Layers in the **same** group merge matching node names.
- Layers in **different** groups remain separate and can be viewed at the same time.
- Merged nodes keep every source declaration; the inspector displays them as tabs.
- Dragging a layer onto another group merges them. Dragging it onto the "new group" drop target splits it again.
- If `.archgraph` exists, grouping and layout changes are persisted in `view_settings` / `default_view`.
- If `.archgraph` does not exist, the desktop is session-only until **Create `.archgraph` · save this session** is used.

## Documentation

- [`EXAMPLE_USAGE.md`](EXAMPLE_USAGE.md) — first-use walkthrough
- [`docs/architecture vault/20 - Documentation/ARCHITECTURE Format.md`](docs/architecture%20vault/20%20-%20Documentation/ARCHITECTURE%20Format.md) — declaration/reference format
- [`docs/architecture vault/20 - Documentation/Project Configuration.md`](docs/architecture%20vault/20%20-%20Documentation/Project%20Configuration.md) — `.archgraph` configuration
- [`docs/HANDOVER.md`](docs/HANDOVER.md) — validation handover for the current change set
