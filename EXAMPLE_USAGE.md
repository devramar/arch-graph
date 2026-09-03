# Example Usage: Scan ArchGraph with ArchGraph

This repository documents its own major systems, making it a useful first project to open in ArchGraph.

## 1. Get the repository

Clone it with Git:

```bash
git clone <repository-url> archgraph
cd archgraph
```

Or download and extract the repository ZIP, then open a terminal in the extracted root folder.

The repository root is the directory containing files such as:

```text
Cargo.toml
README.md
apps/
crates/
scripts/
```

## 2. Install the frontend dependencies

```bash
cd apps/desktop
npm install
cd ../..
```

This only needs to be repeated when the frontend dependency set changes or `node_modules` has been removed.

## 3. Check the project

```bash
./scripts/check.sh
```

A successful check validates:

- the Rust workspace and scanner tests
- TypeScript compilation
- the frontend production build

## 4. Build ArchGraph

Build for the machine you are currently using:

```bash
./scripts/build.sh native
```

On Linux, the raw release executable is:

```text
target/release/archgraph-desktop
```

You can run it directly:

```bash
./target/release/archgraph-desktop
```

Alternatively, during development you can launch through Tauri:

```bash
cd apps/desktop
npm run tauri dev
```

## 5. Open this repository in ArchGraph

When ArchGraph opens:

1. Drag the **ArchGraph repository root folder** onto the application window.
2. ArchGraph scans the project locally.
3. The graph should include documented architecture nodes for the Rust core and desktop application.
4. Select a node to read its architecture document in the inspector.
5. Hover or select a relationship to read the dependency description.
6. Right-click a declaration or use **Open in editor** from the inspector to open its source file in the system's default application.

The relevant self-documenting files include:

```text
crates/archgraph-core/ARCHITECTURE.md
apps/desktop/ARCHITECTURE.md
```

A test fixture also contains a small example architecture document:

```text
crates/archgraph-core/tests/fixtures/basic/src/features/events/sync/ARCHITECTURE.md
```

## 6. Try the graph controls

Useful desktop controls:

- left-drag empty graph space — pan
- middle-drag — pan
- Space + drag — pan
- drag a node — reposition it
- mouse wheel — zoom
- Shift + mouse wheel — stronger zoom
- `/` — focus search
- `F` — fit graph
- `Esc` — clear selection

Try **Sticky** layout if you want nearby nodes to settle around manually repositioned nodes.

## 7. Install it locally on Linux

After a successful native release build:

```bash
./scripts/install.sh
```

ArchGraph is installed for the current user under `~/.local` and should appear in the desktop application launcher.

For later source updates:

```bash
./scripts/build.sh native
./scripts/update.sh
```

To uninstall:

```bash
./scripts/uninstall.sh
```

## Next step: document another project

Add an `ARCHITECTURE.md` only to folders whose structure benefits from architectural explanation.

A minimal document looks like:

```markdown
# Event Synchronization

ARCH_NODE:EventSync

## Description

Coordinates synchronization of remote event data with local state.

## Dependencies

ARCH_DEPENDENCY:DateKey

Used as the canonical representation of calendar days when constructing synchronization windows.
```

Then drag that project's root folder into ArchGraph and the declared relationship becomes part of the graph.

For the complete format, see [`docs/20 - Documentation/ARCHITECTURE Format.md`](docs/20%20-%20Documentation/ARCHITECTURE%20Format.md).
