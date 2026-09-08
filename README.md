# ArchGraph

ArchGraph is an offline desktop tool for exploring explicitly authored architecture as a graph.

Projects describe important systems with lightweight architecture documents. By default these are named `ARCHITECTURE.md`, but a root `.archgraph` file can replace the accepted document names and marker tokens.

```text
ARCH_NODE:EventSync
ARCH_REFERENCE:EventStore
ARCH_SUBREFERENCE:Password Management
```

ArchGraph does not infer architecture from imports, exports, or programming-language syntax. A normal reference connects to a matching documented architecture node when one exists; otherwise it becomes a valid lightweight shared reference node. A subreference is always local to the declaring architecture node and never merges by name.

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

```bash
cd apps/desktop
npm install
cd ../..
```

### 3. Validate the project

```bash
./scripts/check.sh
```

### 4. Run in development

```bash
cd apps/desktop
npm run tauri dev
```

### 5. Build a release

```bash
./scripts/build.sh native
```

See [`scripts/README.md`](scripts/README.md) for platform-specific build commands.

## `.archgraph`

A root `.archgraph` file is optional. Missing fields use ArchGraph defaults; configured alias lists replace the default spelling for that concept.

```json
{
  "aliasing": {
    "ARCHITECTURE.md": ["DOCUMENTATION.md"],
    "ARCH_NODE": ["SYS_NODE"],
    "ARCH_REFERENCE": ["SYS_REF", "ARCH_REF"],
    "ARCH_SUBREFERENCE": ["SYS_LOCAL"]
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

`view_settings` is intentionally application-defined data transported by the Rust core. The desktop currently understands `reference_distance`, `subreference_distance`, `node_spacing`, and `subreference_attraction` for its layouts.

## Documentation

- [`EXAMPLE_USAGE.md`](EXAMPLE_USAGE.md) — first-use walkthrough
- [`docs/architecture vault/20 - Documentation/ARCHITECTURE Format.md`](docs/architecture%20vault/20%20-%20Documentation/ARCHITECTURE%20Format.md) — architecture document format
- [`docs/architecture vault/20 - Documentation/Project Configuration.md`](docs/architecture%20vault/20%20-%20Documentation/Project%20Configuration.md) — `.archgraph` configuration
- [`docs/HANDOVER.md`](docs/HANDOVER.md) — validation handover for this refactor
