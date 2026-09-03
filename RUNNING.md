# Running ArchGraph

## Requirements

### Desktop app

- Node.js 22 or newer
- npm
- Rust toolchain with `cargo` and `rustc`
- Tauri 2 platform prerequisites for your OS

On Linux, Tauri also requires the normal WebKit/GTK system dependencies documented by Tauri for your distribution. ArchGraph disables WebKitGTK's DMABUF renderer before Tauri initialization to avoid the observed Wayland protocol crash on affected systems.

## Install frontend dependencies

```bash
cd apps/desktop
npm install
```

## Run the desktop app

From `apps/desktop`:

```bash
npm run tauri dev
```

The application opens to a project picker. Either:

- drag a project root folder into the window, or
- select **Open Folder**.

The selected root is sent to the standalone Rust scanner and the resulting graph is rendered locally.

## Build the desktop app

The repository-level build scripts are the preferred entry point:

```bash
./scripts/build.sh native
```

Target-specific helpers are also available:

```bash
./scripts/build.sh linux x86_64
./scripts/build.sh windows x86_64
./scripts/build.sh macos universal
./scripts/build.sh core
```

See `scripts/README.md` for host requirements and cross-build notes. Tauri places bundles beneath the repository `target/<target>/release/bundle/` tree (or `target/release/bundle/` for a native build).

## Application icon

Put the master icon at:

```text
assets/app-icon.png
```

A square 1024x1024 transparent PNG is recommended; a square SVG is also accepted. Then generate all platform assets with:

```bash
./scripts/generate-icons.sh
```

The generated files replace the contents of `apps/desktop/src-tauri/icons/`, which is already wired into `tauri.conf.json`.

## Run only the browser UI

The React frontend has a demo graph specifically so its graph interaction can be tested without Tauri:

```bash
cd apps/desktop
npm run dev
```

Open the Vite URL and choose **Load Demo Graph**.

Browser preview cannot scan arbitrary project folders. Project scanning intentionally lives behind the Tauri/Rust boundary.

## Run the standalone scanner

From repository root:

```bash
cargo run -p archgraph-cli -- /path/to/project > graph.json
```

This scans the project and writes the neutral graph model as JSON to stdout.

## Run checks

```bash
./scripts/check.sh
```

Or run the Rust workspace directly:

```bash
cargo test --workspace
```

## Frontend type/build validation

```bash
cd apps/desktop
npm run build
```

This runs TypeScript compilation followed by the Vite production build.
