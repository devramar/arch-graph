# ArchGraph v3 Layered Sources — Handover

This handover covers the layered-source/discovery pass built on top of the previously validated v2 reference model.

The previous reference-model revision was locally validated by the project owner with `cargo fmt`, `cargo test --workspace`, and Clippy with `-D warnings`. This pass changes the scanner/model/parser/desktop substantially enough that the Rust and frontend validation below should be run again.

## What changed

ArchGraph's source pipeline is now:

```text
candidate files -> architecture declarations -> layers -> composition groups -> graph
```

The core remains language-agnostic. It never inspects imports, exports, classes, interfaces, ASTs, or package metadata.

### Graph contract

`ArchitectureGraph.version` is now `3`.

The v3 graph adds:

- composition `groups`;
- layer/group provenance on nodes, edges, and diagnostics;
- `declarations[]` on architecture nodes;
- declaration source format (`markdown` or `decoratedText`);
- support for several declarations contributing to one merged architecture node.

A merged architecture node retains every contributing declaration rather than discarding one source. The desktop inspector displays these declarations as tabs.

### Logical layers

`.archgraph` now uses `layers` instead of the old v2 top-level `aliasing` object.

A built-in layer named `architecture` always exists. Its defaults are:

```text
Display name: Architecture
Files:        ARCHITECTURE.md
Markers:      ARCH_NODE / ARCH_REFERENCE / ARCH_SUBREFERENCE
```

Layers may override their own file globs and marker lists and may add additional logical representations such as `implementation` or `overview`.

Example:

```json
{
  "ignored_paths": [
    "crates/archgraph-core/tests/fixtures/**"
  ],
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "files": ["ARCHITECTURE.md"]
    },
    "implementation": {
      "display_name": "Implementation",
      "files": ["*.ts", "*.tsx", "*.rs"]
    },
    "overview": {
      "display_name": "Overview",
      "files": ["OVERVIEW.md"],
      "markers": {
        "ARCH_NODE": ["OVERVIEW_NODE"],
        "ARCH_REFERENCE": ["OVERVIEW_REFERENCE"],
        "ARCH_SUBREFERENCE": ["OVERVIEW_SUBREFERENCE"]
      }
    }
  }
}
```

Missing marker lists use canonical defaults. If the `architecture` layer is omitted, the built-in layer is restored automatically.

File patterns use ArchGraph's glob matcher (`*`, `?`, `**`); regex is deliberately unsupported.

A source matched by more than one layer is not guessed. It is skipped and emits `ARCH007`.

### Ignored paths

Root `.archgraph` may now contain:

```json
{
  "ignored_paths": [
    "crates/archgraph-core/tests/fixtures/**",
    "**/generated/**"
  ]
}
```

These exclusions are combined with existing built-in ignores, Git ignore behaviour, and `.archgraphignore`.

The ArchGraph repository itself does **not** gain a root `.archgraph` in this change. This is deliberate: merely opening an arbitrary repository should remain session-only and should not create project files. If desired, the fixture path above is the intended configuration for excluding this repository's own core fixtures.

### Candidate-file behaviour

A file glob only enrols a file as a scan candidate.

- candidate with no ArchGraph markers -> silently ignored;
- candidate with ArchGraph markers but no valid node marker -> `ARCH004`;
- more than one valid node marker -> `ARCH006` and only the first declaration is retained;
- one file -> one architecture node remains an explicit limitation in this version.

This means `*.ts` can be enabled safely without every TypeScript file becoming a graph node.

### Source parsers

Parser selection is centralized and extensible.

Current parsers:

```text
.md / .markdown -> Markdown parser
all other files -> generic decorated-text parser
```

Future specialized formats can be added through this dispatch point without changing scanner semantics.

#### Markdown

Markdown continues to accept bare configured markers and preserves its architecture text as Markdown documentation.

#### Decorated text

Non-Markdown files require a punctuation-only decoration prefix. For example:

```ts
/// ARCH_NODE:DateKey
///
/// Handles canonical YYYY-MM-DD date keys.
///
/// ARCH_REFERENCE:Clock
/// Used when deriving today's date.
export type DateKey = string;
```

ArchGraph does not know this is TypeScript or that `///` is a comment form. It sees a decoration prefix, strips it from adjacent matching lines, and stops consuming prose when the decoration no longer lines up.

Examples such as `#`, `--`, `///`, and `*` work under the same generic rule.

Markers embedded later in executable/text content are intentionally rejected, e.g.:

```ts
const example = "ARCH_NODE:Fake";
```

### Layer composition

Layers are scanned independently by Rust. The core also owns composition through `compose_project_layers(...)`.

Within one composition group:

- same-name architecture declarations from different layers merge into one architecture node;
- the merged node carries all declarations;
- normal reference edges are resolved again against the composed declaration set, so an Architecture-layer reference can resolve to an Implementation-layer declaration when those layers are grouped;
- duplicate same-name declarations within one contributing layer remain ambiguous;
- subreferences never merge.

Different enabled composition groups remain independent and may be displayed simultaneously.

The CLI still uses `scan_project(...)` and prints the default all-layers composition, so simple graph JSON export remains uncomplicated. Rich clients use `scan_project_state(...)` to receive independent layers/configuration and can ask Rust to compose arbitrary groups without rescanning files.

## Desktop behaviour

### Group regions

The desktop displays every enabled layer composition group simultaneously.

Each composition group is represented by one soft labelled region behind its graph nodes. Regions represent **merge boundaries**, not individual layers:

- layers inside one region are composed and same-name declarations can merge;
- layers in separate regions stay independent;
- several regions can remain enabled at once.

### Drag/drop grouping

The sidebar shows layer groups and draggable layer chips.

- drag a layer onto another group to compose them;
- drop a layer into the new-group target to split it out again;
- check/uncheck groups to show several, one, or none simultaneously.

Group names are derived from their participating layer display names for now.

### Multiple declarations

When a merged node has several declarations, the Inspector presents one tab per declaration/layer. Each tab shows source provenance and that declaration's extracted documentation.

Markdown declarations retain Markdown rendering. Decorated-text declarations render their extracted architecture prose as plain pre-wrapped text.

### Persistence model

This change deliberately distinguishes two project modes.

**No `.archgraph`: session-only**

- drag/drop grouping works;
- enabled/disabled groups work;
- layout changes work;
- nothing is written to the repository.

The sidebar labels this state `session only`.

**`.archgraph` exists: session-aware**

Desktop grouping and current view are persisted into the opaque `view_settings.desktop.layer_groups` / `default_view` data through the Rust core.

If no `.archgraph` exists, the sidebar exposes:

> Create `.archgraph` · save this session

That explicit action writes the normalized configuration plus current desktop session state and switches the open project into persistent mode. The frontend never writes project files directly.

### Existing visual semantics retained

- edge arrow colour = destination node primary colour;
- deterministic reference/subreference colour families;
- explicit `colour_overrides.references` and `.subreferences`;
- subreferences are small dashed ellipses;
- Sticky/Organic keep subreferences materially closer to their owner than ordinary reference nodes.

## Configuration migration from v2

The v2 configuration form:

```json
{
  "aliasing": {
    "ARCHITECTURE.md": ["DOCUMENTATION.md"],
    "ARCH_NODE": ["SYS_NODE"]
  }
}
```

is intentionally replaced rather than silently supported.

The v3 equivalent is:

```json
{
  "layers": {
    "architecture": {
      "files": ["DOCUMENTATION.md"],
      "markers": {
        "ARCH_NODE": ["SYS_NODE"],
        "ARCH_REFERENCE": ["ARCH_REFERENCE"],
        "ARCH_SUBREFERENCE": ["ARCH_SUBREFERENCE"]
      }
    }
  }
}
```

This clean break is required because discovery aliases now belong to specific logical layers rather than one project-global scanner vocabulary.

## Important fixtures

### `crates/archgraph-core/tests/fixtures/basic`

Has no `.archgraph` and therefore exercises default Architecture-only discovery. Its adjacent `.ts` files must remain invisible because no implementation layer opts into them.

### `crates/archgraph-core/tests/fixtures/aliases`

Despite the historical directory name, this is now the layered-source fixture.

Its `.archgraph` defines:

- an Architecture layer using `DOCUMENTATION.md` and custom `SYS_*` markers;
- an Implementation layer using `*.ts` and canonical markers;
- `ignored/**` as an ignored path;
- existing colour/layout configuration.

Important contents:

- Architecture `Identity` declaration in `identity/DOCUMENTATION.md`;
- decorated Implementation `Identity` declaration in `implementation/Identity.ts`;
- an unannotated `implementation/no-architecture.ts`, which must disappear silently;
- `ignored/DOCUMENTATION.md` declaring `ShouldNotAppear`, which must never appear.

The shared `Identity` name provides the main cross-layer composition/tab regression case.

## Validation performed in this environment

This environment still has no Rust toolchain. Per project instruction, no attempt was made to install one.

Before packaging, the following non-Rust checks were performed or should be considered part of the packaging gate:

- configuration and graph JSON files/schemas parse as JSON;
- JSON Schema Draft 2020-12 validation for the v3 graph schema, configuration schema, and layered `.archgraph` fixture;
- desktop TS/TSX syntax parsing with the globally available TypeScript compiler API;
- repository-wide searches for stale v2 runtime concepts and marker-model assumptions;
- shell syntax/check-whitespace checks;
- ZIP integrity check.

This repository does not contain an npm lockfile, so `npm ci` is not applicable. A bounded `npm install --ignore-scripts --no-audit --no-fund` attempt timed out in this environment; partial `node_modules`/lockfile output was removed. A dependency-backed `npm run build` therefore remains part of the local validation request.

## Please run locally

From the repository root:

```bash
cargo fmt --all
cargo fmt --all --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Then:

```bash
cd apps/desktop
npm install
npm run build
npm run tauri dev
```

The most useful feedback is the complete output from the **first** command that fails.

## Manual runtime checks

### Layered fixture

Open:

```text
crates/archgraph-core/tests/fixtures/aliases
```

1. Confirm Architecture and Implementation initially appear as separate enabled regions/groups.
2. Confirm there are initially two separate `Identity` architecture nodes, one in each group.
3. Drag Implementation onto Architecture.
4. Confirm the two `Identity` nodes become one node.
5. Select merged `Identity` and confirm the Inspector has two source tabs: Architecture and Implementation.
6. Confirm the Implementation tab contains the prose extracted from `///` decoration lines rather than the entire TypeScript file.
7. Confirm Checkout's normal reference to `Identity` now targets the merged node.
8. Confirm the Implementation declaration's `Token Store` reference is present.
9. Confirm `ShouldNotAppear` is absent because `ignored/**` is configured.
10. Confirm `no-architecture.ts` creates neither a node nor a diagnostic.
11. Split Implementation back into a new group and confirm two independent `Identity` nodes return while both regions remain visible.
12. Disable/re-enable either group and confirm the other remains independently visible.
13. Because this fixture already has `.archgraph`, change grouping/layout, reload the project, and confirm the session state persists.

### Session-only project

Use a temporary copy of a project/fixture that has **no** `.archgraph` so validation does not dirty the repository fixture itself.

1. Open it and confirm the sidebar reports `session only`.
2. Change layout/group state and confirm no `.archgraph` appears.
3. Use **Create `.archgraph` · save this session**.
4. Confirm the file is created at the selected root.
5. Reload and confirm the grouping/layout state restores.
6. Inspect the generated JSON and confirm writes are normalized/pretty-printed by the Rust core.

### Existing reference visuals

1. Confirm every edge arrow still matches its destination node's primary/border colour.
2. Confirm local subreferences remain dashed ellipses and sit materially closer to their source in Sticky view.
3. Confirm same-name subreferences remain distinct while sharing colour identity.

## Known intentional limitations

- one source file -> one architecture node;
- no nested `.archgraph` files/inheritance;
- no language-specific parsers or AST discovery;
- no regex file matching;
- ArchGraph's glob matcher supports the required `*`, `?`, and `**` behaviour but is intentionally smaller than a complete gitignore implementation;
- a source claimed by multiple layers is diagnosed/skipped instead of interpreted twice;
- composition groups are flat; no nested group hierarchy;
- group names are currently generated from layer names rather than being a richer workspace-management concept.

## Main implementation areas touched

Core:

- `crates/archgraph-core/src/config.rs`
- `crates/archgraph-core/src/model.rs`
- `crates/archgraph-core/src/parser.rs`
- `crates/archgraph-core/src/resolver.rs`
- `crates/archgraph-core/src/scanner.rs`
- `crates/archgraph-core/src/lib.rs`
- `crates/archgraph-core/tests/scan_fixture.rs`
- layered test fixtures under `crates/archgraph-core/tests/fixtures/aliases`

Desktop/Tauri:

- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/lib/desktop.ts`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/components/Sidebar.tsx`
- `apps/desktop/src/components/GraphCanvas.tsx`
- `apps/desktop/src/components/Inspector.tsx`
- `apps/desktop/src/styles.css`

Contracts/docs:

- `schema/architecture-graph.schema.json`
- `schema/project-configuration.schema.json`
- README/example/architecture-vault/LLM-context documentation

## Supplied-tree changes preserved

The uploaded repository already contained working-tree changes, including the desktop `package.json` clean-script change, staged LLM-context directory renames, and an untracked `docs/LLM context - archgraph.zip`. These were preserved. The existing LLM-context ZIP is intentionally not regenerated/overwritten by this pass.
