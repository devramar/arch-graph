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

```bash
cd apps/desktop
npm run tauri build
```

Tauri will place platform-specific bundles under its normal `src-tauri/target` output directories.

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

## Run Rust tests

```bash
cargo test --workspace
```

## Frontend type/build validation

```bash
cd apps/desktop
npm run build
```

This runs TypeScript compilation followed by the Vite production build.
