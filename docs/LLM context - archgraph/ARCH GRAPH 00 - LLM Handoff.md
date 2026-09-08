# ArchGraph — LLM Handoff

## Product definition

ArchGraph is an offline architecture-reference explorer. It discovers explicitly authored architecture declarations from configured source files, organises them into logical layers, and composes selected layer groups into directed graphs.

The core rule is:

> ArchGraph understands explicit architecture annotations, not programming languages.

Do not infer architecture from imports, exports, ASTs, package manifests, or language-server data.

## Source pipeline

```text
files -> declarations -> layers -> composition groups -> graph
```

A configured file glob only makes a file a candidate. A candidate containing no ArchGraph markers is ignored silently.

Markdown (`.md` / `.markdown`) uses the Markdown source parser. Every other file currently uses the generic decorated-text parser. Parser dispatch is centralized so more specialized source formats can be added later.

Decorated sources require markers to be the first meaningful content after a punctuation-only decoration prefix, for example:

```ts
/// ARCH_NODE:DateKey
///
/// Handles dates in canonical YYYY-MM-DD form.
///
/// ARCH_REFERENCE:Clock
/// Used when deriving today's date.
```

The parser does not know what `///` means in TypeScript or Rust. It only observes a decoration prefix and consumes adjacent prose that uses the same prefix. A string such as `const x = "ARCH_NODE:Fake"` is not a declaration.

One source file may currently declare at most one node.

## Canonical markers

```text
ARCH_NODE:Name
ARCH_REFERENCE:Name
ARCH_SUBREFERENCE:Name
```

Marker spellings are configured independently per layer. Markdown headings such as `## References` are authoring conventions only.

## Reference semantics

`ARCH_REFERENCE:X` creates a mergeable named reference. Within a composition group:

1. exactly one eligible architecture declaration named `X` -> target that architecture node;
2. none -> target one shared lightweight reference node named `X`;
3. duplicate declarations within a contributing layer -> do not guess; preserve ambiguity and emit a diagnostic.

Missing architecture declarations are normal and are not unresolved errors.

`ARCH_SUBREFERENCE:X` always creates a local lightweight satellite node. It never resolves or merges, even when names match. Same-name subreferences have distinct graph IDs but deliberately share deterministic colour identity in the desktop app.

## Layers and composition

`.archgraph` defines logical layers. The built-in `architecture` layer always exists and defaults to:

```text
display name: Architecture
files:        ARCHITECTURE.md
markers:      ARCH_NODE / ARCH_REFERENCE / ARCH_SUBREFERENCE
```

Additional layers may discover implementation files, overview documents, security notes, etc.

Layers are scanned independently. The Rust core also owns composition semantics. A desktop composition group can combine multiple layers; same-name architecture declarations in those layers become one graph node with multiple declarations. Separate enabled groups remain independent and may be displayed simultaneously.

Merged nodes retain every contributing declaration. The desktop inspector presents those declarations as tabs.

Subreferences never merge across layers or groups.

## Graph v3

Node kinds:

```text
architecture
reference
```

Reference scopes:

```text
shared
local
```

Edge kinds:

```text
reference
subreference
```

Architecture nodes contain `declarations[]`, each recording layer provenance, source location, source format, and extracted documentation. Graphs also contain composition `groups[]`; nodes/edges carry group provenance.

Diagnostics currently include:

```text
ARCH001 duplicate architecture node name
ARCH002 ambiguous normal reference
ARCH004 marker-bearing source without a valid node marker
ARCH005 malformed/empty marker
ARCH006 multiple node markers in one source
ARCH007 one source matched by multiple layers
```

## `.archgraph`

One optional strict-JSON `.archgraph` may exist at the selected project root. There is no nested/inherited configuration.

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
  },
  "app_colours": {
    "colour_overrides": {
      "references": { "Identity": "purple" },
      "subreferences": { "Password Management": "teal" }
    }
  },
  "default_view": "sticky",
  "view_settings": {
    "desktop": {
      "layer_groups": []
    },
    "sticky": {
      "reference_distance": 175,
      "subreference_distance": 68,
      "subreference_attraction": 1.8
    }
  }
}
```

File entries and `ignored_paths` use ArchGraph's glob matcher (`*`, `?`, `**`). Regex is not supported.

The Rust core owns configuration parsing, normalization, validation, and writes. `view_settings` is opaque application data.

## Desktop persistence

Without `.archgraph`, the desktop is session-only: layer grouping, visibility, and layout changes are not written to the project.

The user may explicitly choose **Create `.archgraph` · save this session**. After a configuration file exists, desktop layer groups and current view can be persisted through the Rust core.

## Desktop visual semantics

Enabled composition groups are shown simultaneously as soft labelled graph regions. A region represents a composition group, not a source layer. Dragging one layer onto another group's region makes them compose and therefore allows same-name declarations to merge.

Every edge arrow uses the primary colour of its destination node. Subreferences are smaller dashed ellipse nodes and stay closer to their owner in force layouts.

## Repository boundaries

`crates/archgraph-core`
: discovery, parsing, layers, composition, diagnostics, `.archgraph` I/O, graph semantics.

`crates/archgraph-cli`
: simple default-composed JSON graph consumer.

`apps/desktop/src-tauri`
: narrow native adapter around core operations.

`apps/desktop/src`
: React/Cytoscape presentation, drag/drop grouping, tabs, session persistence UX.

The frontend must not implement filesystem discovery or reference-composition semantics.
