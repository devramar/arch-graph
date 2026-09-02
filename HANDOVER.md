# Handover

## Current implementation

The repository contains three runtime boundaries:

1. `crates/archgraph-core` — standalone Rust scanner and neutral graph model.
2. `crates/archgraph-cli` — JSON CLI consumer of the core.
3. `apps/desktop` — Tauri + React + Cytoscape desktop consumer of the same core.

## Validation completed in the build environment

- Repository structure created and versioned with Git.
- Git phase commits created throughout implementation.
- Node.js/npm availability confirmed.
- Current Tauri 2 drag/drop API and current Cytoscape/Tauri package versions checked against upstream documentation/package metadata.
- Source-level review performed after each implementation phase.

## Validation not completed here

The build environment did not contain `rustc` or `cargo`. Per project instruction, no Rust installation was attempted.

`npm install` was attempted once for the desktop project but the registry request did not complete within the execution environment. It was not repeatedly retried.

Therefore the following must be run on a normal development machine:

```bash
cargo test --workspace
```

and:

```bash
cd apps/desktop
npm install
npm run build
npm run tauri dev
```

Any compile/runtime failures found by those commands should be returned with their complete output for the next repair round.

## First manual validation pass

After `npm run tauri dev` succeeds:

1. Drop `crates/archgraph-core/tests/fixtures/basic` into the app.
2. Confirm `EventSync`, `DateKey`, `EventStore`, and unresolved `RemoteChanges` appear.
3. Hover the `EventSync → DateKey` edge and confirm its relationship description appears.
4. Click the edge and confirm the right inspector shows its source file/line.
5. Pan with left-drag on graph background.
6. Pan with middle-drag.
7. Hold Space and drag to pan, including beginning the drag over a node.
8. Drag a node normally without Space and confirm the node itself moves.
9. Type `DateKey` in search and confirm unrelated graph elements fade.
10. Disable **Modules** and confirm module nodes and attached edges disappear.
11. Switch between **Dependency** and **Organic** layouts.
12. Confirm diagnostics shows `ARCH003` for `RemoteChanges`.

## Known MVP limitations

- Qualified dependency syntax such as `@/core/date/DateKey#DateKey` is documented as a future escape hatch but is not implemented yet.
- TypeScript exported-symbol discovery is intentionally lightweight. It recognizes common direct `export class/interface/type/enum/namespace/function/const/let/var` declarations. It does not implement the TypeScript type system or barrel re-export semantics.
- `External` node resolution is represented in the graph schema/UI but the scanner does not yet automatically classify package/service dependencies as external.
- Source navigation currently displays source location; it does not yet launch the user's editor.
- Live filesystem watching is not part of the MVP.

## Useful next implementation rounds

High-value next steps after first runtime validation:

- qualified dependency target resolution
- TypeScript barrel/re-export resolution
- editor deep-links
- graph JSON import/export in the desktop UI
- filesystem watch + incremental rescan
- persisted manual node positions
