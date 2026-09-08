# ArchGraph v2 Reference Model — Handover

This handover describes the reference-model refactor in this repository and the validation still required on a machine with the Rust and frontend toolchains installed.

## What changed

ArchGraph is now explicitly language-agnostic. The Rust core no longer scans TypeScript exports or attempts to infer source-code modules. Architectural relationships come only from authored markers in architecture documents.

The graph contract is now schema version `2`.

### Reference semantics

The canonical markers are:

```text
ARCH_NODE:<name>
ARCH_REFERENCE:<name>
ARCH_SUBREFERENCE:<name>
```

`ARCH_REFERENCE` creates a normal named reference. If exactly one architecture document declares the same node name, the reference targets that architecture node. If no architecture document declares that name, ArchGraph creates one globally shared lightweight reference node. Missing architecture documentation is valid and no longer produces an unresolved-reference diagnostic.

`ARCH_SUBREFERENCE` always creates a source-local lightweight reference node. It never resolves to an architecture document and never merges with an identically named node from another source. Same-name subreferences remain visually related by deterministic colour selection in the desktop app.

Reference descriptions remain edge documentation. Markdown headings such as `## References` are an authoring convention and do not gate marker recognition; configured marker aliases are the machine-readable contract. Markdown section boundaries are used only to delimit adjacent prose.

### Removed v1 behaviour

- `ARCH_DEPENDENCY` is no longer a built-in marker.
- TypeScript export/module indexing has been removed.
- `module`, `external`, and `unresolved` graph-node semantics have been removed from the v2 graph contract.
- `ARCH003` unresolved dependency diagnostics have been removed.
- Edge `resolution` state has been replaced by `referenceKind: "reference" | "subreference"`.

Projects that temporarily need the old spelling can explicitly include `ARCH_DEPENDENCY` as an alias for `ARCH_REFERENCE` in `.archgraph`; it is not enabled implicitly.

## `.archgraph` project configuration

A project root can optionally contain strict JSON in `.archgraph`. If the file is absent, the canonical defaults are used.

Example:

```json
{
  "aliasing": {
    "ARCHITECTURE.md": ["DOCUMENTATION.md", "architecture.md"],
    "ARCH_NODE": ["SYS_NODE"],
    "ARCH_REFERENCE": ["SYS_REF", "ARCH_REF"],
    "ARCH_SUBREFERENCE": ["SYS_LOCAL"]
  },
  "app_colours": {
    "colour_overrides": {
      "references": {
        "Authentication": "blue"
      },
      "subreferences": {
        "Password Management": "purple"
      }
    }
  },
  "default_view": "sticky",
  "view_settings": {
    "sticky": {
      "reference_distance": 175,
      "subreference_distance": 68,
      "node_spacing": 62,
      "subreference_attraction": 1.8
    }
  }
}
```

Alias arrays replace the corresponding defaults. To accept both a canonical spelling and an organisation-specific spelling, list both explicitly.

Configuration is root-scoped only in this version; nested `.archgraph` inheritance is not implemented.

The Rust core owns loading, normalization, validation, and writing through:

- `load_project_configuration(...)`
- `write_project_configuration(...)`
- `scan_project_state(...)` / `scan_project_state_with_options(...)` when callers need both graph and configuration

The Tauri layer exposes `update_project_configuration` so desktop/future app settings UIs do not need direct filesystem writes. There is intentionally no settings editor UI in this change set yet. Callers should rescan after a successful write because filename/marker aliases can change the graph itself.

`view_settings` is deliberately opaque to the Rust core. Apps interpret their own keys. The current desktop understands the four Sticky/Organic/Directed tuning keys shown above. Higher `subreference_attraction` values mean stronger local attraction in the force-directed layouts.

## Desktop behaviour

The desktop now distinguishes:

- architecture nodes
- shared reference nodes
- local subreference nodes

Subreferences are smaller dashed ellipse nodes and are laid out closer to their declaring architecture node. Normal references use more separation than before.

Reference/subreference colours are selected deterministically from the desktop palette using the explicit reference name. Same-name references therefore keep the same visual family across rescans. `colour_overrides.references` and `colour_overrides.subreferences` can select a named desktop palette colour per explicit name.

Every edge arrow uses the primary colour of its destination node. The destination node border uses the same primary colour and its fill uses the corresponding secondary colour.

Selecting an undocumented shared reference reports that no architecture document describes the explicit name and shows its incoming source nodes. Ambiguous duplicate architecture documents remain a diagnostic rather than being guessed.

## Important fixtures

`crates/archgraph-core/tests/fixtures/basic` intentionally still contains `DateKey.ts` and `EventStore.ts`. Their presence is a regression guard: the v2 core must **not** discover them as source-language graph nodes. Their authored references should appear as lightweight shared references unless an architecture document with that exact name exists.

The same fixture now contains a documented `RemoteChanges` architecture node and a local `Password Management` subreference.

`crates/archgraph-core/tests/fixtures/aliases` exercises custom architecture filenames, marker aliases, colour configuration, default view, and opaque view settings.

## Validation performed in this environment

The supplied environment does not contain `cargo`, `rustc`, or `rustfmt`. Per the project request, no attempt was made to install a Rust toolchain.

`node`/`npm` and a global TypeScript compiler are available, but `apps/desktop/node_modules` was not supplied. A dependency install attempt was stopped after timing out, so a dependency-backed frontend build could not be performed here. A direct `tsc -b` invocation consequently reports missing React/Cytoscape/Tauri packages and is not a meaningful application compile result.

The handover/package pass performed non-toolchain checks instead:

- both JSON Schemas parse and pass Draft 2020-12 schema validation;
- the aliased `.archgraph` fixture validates against `project-configuration.schema.json`;
- all 12 desktop `.ts`/`.tsx` source files parse successfully with the available TypeScript parser without module resolution;
- a repository-wide legacy-token sweep found old dependency/source-resolution vocabulary only in deliberate migration/history prose;
- `git diff --check` passes.

## Please run locally

From the repository root, first validate the Rust workspace:

```bash
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Then validate the desktop using the existing npm lockfile:

```bash
cd apps/desktop
npm ci
npm run build
```

You can also exercise the CLI serialization directly, for example:

```bash
cargo run -p archgraph-cli -- crates/archgraph-core/tests/fixtures/basic
```

If those pass, launch the desktop:

```bash
npm run tauri dev
```

The repository also contains `scripts/check.sh`; after dependencies are installed, running it is useful as an additional project-level check if it matches your local workflow.

## Runtime checks worth doing

1. Open `crates/archgraph-core/tests/fixtures/basic` as a project. Confirm `DateKey` and `EventStore` are shared reference nodes despite the adjacent `.ts` files.
2. Confirm `RemoteChanges` resolves to its architecture document.
3. Confirm `Password Management` is a small dashed local subreference node and sits materially closer to `EventSync` in Sticky view.
4. Add the same `ARCH_SUBREFERENCE:Password Management` under another architecture node. Confirm the two nodes remain distinct but receive the same colour family.
5. Confirm each edge arrow matches the primary/border colour of its destination node.
6. Open `crates/archgraph-core/tests/fixtures/aliases` and confirm `DOCUMENTATION.md`, `SYS_NODE`, `ARCH_REF`/`SYS_REF`, and `SYS_LOCAL` are recognized according to its `.archgraph` file.
7. Change a reference/subreference colour override in that fixture and rescan; confirm the named desktop palette colour is applied.
8. Change Sticky `reference_distance` / `subreference_distance` values and confirm the desktop consumes them without any Rust-side knowledge of those fields.

## If validation fails

For a follow-up pass, send the complete output from the first failing command, especially:

- Rust compiler/test/clippy diagnostics with file and line numbers
- `npm run build` TypeScript diagnostics
- any Tauri runtime console error when loading one of the fixtures

Please include which fixture/project was open for visual or runtime behaviour failures.

## Supplied-tree changes preserved

The uploaded repository already contained working-tree changes including the desktop `package.json` clean-script change, staged LLM-context directory renames, and an untracked `docs/LLM context - archgraph.zip`. This refactor preserves those supplied changes. The existing LLM-context ZIP was not regenerated as part of this work.
