# ArchGraph — LLM Handoff

## Product definition

ArchGraph is an offline architecture-reference explorer. It scans explicitly authored architecture documents and turns explicitly authored named references into a directed graph.

The core rule is:

> ArchGraph understands architecture documentation, not programming languages.

Do not infer architecture from imports, exports, modules, package manifests, ASTs, or language-server data.

## Canonical document markers

```text
ARCH_NODE:Name
ARCH_REFERENCE:Name
ARCH_SUBREFERENCE:Name
```

The markers may be replaced through root `.archgraph` aliases. `## References` is only a recommended human-facing section and does not gate marker recognition; Markdown boundaries are used only to delimit adjacent prose.

## Architecture nodes

`ARCH_NODE:EventSync` establishes a documented architecture node named `EventSync`.

Architecture-node names are the only names normal references may resolve to as documented targets.

## Normal references

```text
ARCH_REFERENCE:EventStore
```

creates one authored edge from the declaring architecture node to the explicit name `EventStore`.

Resolution semantics:

1. exactly one architecture document declares `ARCH_NODE:EventStore` → target that architecture node;
2. no architecture document declares it → create/use one globally shared lightweight reference node named `EventStore`;
3. multiple architecture documents declare it → target a lightweight reference node and emit an ambiguity diagnostic rather than guessing.

Missing architecture documentation is normal. It is not an unresolved error.

The prose beneath the marker describes the edge:

```markdown
ARCH_REFERENCE:EventStore

Provides persisted local event state used during reconciliation.
```

## Subreferences

```text
ARCH_SUBREFERENCE:Password Management
```

creates a lightweight local satellite node owned by that declaration's source architecture node.

Subreference invariants:

- never resolve to an `ARCH_NODE`;
- never merge with another subreference by name;
- never merge with a normal shared reference by name;
- two same-name subreferences have different graph node IDs;
- same-name subreferences deliberately share deterministic visual colour identity in the desktop app.

`ARCH_SUBREFERENCE` does not mean a child of the previous reference. It means source-local/non-merging reference.

## Graph v2

Node kinds:

```text
architecture
reference
```

Reference-node scopes:

```text
shared
local
```

Edge kinds:

```text
reference
subreference
```

There are no module, external, or unresolved node kinds in graph v2.

There is no edge-resolution field. Every edge is an authored reference; the destination node type tells the consumer what it currently points at.

Current diagnostics:

```text
ARCH001 duplicate architecture node name
ARCH002 ambiguous normal reference due to duplicate architecture nodes
ARCH004 architecture document missing a node marker
ARCH005 malformed/empty marker
```

## `.archgraph`

One optional strict-JSON `.archgraph` file may exist at the selected project root. There is no nested/inherited configuration in this version.

Example:

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

Alias lists replace defaults. If `ARCH_REFERENCE` is configured as `["SYS_REF"]`, the canonical `ARCH_REFERENCE:` spelling is no longer accepted unless it is also listed.

The Rust core owns configuration parsing, validation, normalization, and writes.

`view_settings` is intentionally opaque application data. The core transports it without understanding layout-specific keys.

## Desktop colour semantics

The desktop has a fixed built-in pretty palette. Shared references and subreferences choose a colour deterministically from their explicit names unless `.archgraph` gives a named override.

The current built-in names are:

```text
purple
blue
teal
green
orange
pink
red
indigo
```

For all edges:

> target arrow colour = destination node primary colour

Architecture nodes use the architecture colour pair. Reference/subreference nodes use their deterministic/overridden pair.

Subreferences are smaller dashed ellipse nodes.

## Layout semantics

Desktop views:

```text
directed
organic
sticky
```

Normal reference spacing is intentionally larger than before. Local subreference edges use shorter ideal lengths; force layouts also apply stronger attraction so subreferences remain satellite-like around their owner.

Desktop `view_settings` currently understands:

```text
reference_distance
subreference_distance
node_spacing
subreference_attraction
```

## Repository boundaries

`crates/archgraph-core`
: project/configuration scanning and graph semantics.

`crates/archgraph-cli`
: standalone graph JSON consumer.

`apps/desktop/src-tauri`
: narrow native adapter around the core.

`apps/desktop/src`
: React/Cytoscape presentation and interaction.

The frontend must not implement filesystem scanning or reference resolution.

## Important Rust files

`src/config.rs`
: `.archgraph` defaults, strict parsing, alias validation, normalization, writes.

`src/model.rs`
: graph v2 data contract.

`src/parser.rs`
: configurable token parsing and reference prose extraction.

`src/resolver.rs`
: architecture/shared/local reference resolution.

`src/scanner.rs`
: project traversal and graph assembly.

## Important desktop files

`src/types.ts`
: TypeScript representation of graph v2 and project configuration.

`src/components/GraphCanvas.tsx`
: deterministic colours, destination arrow colours, layouts, search/filter rendering, interactions.

`src/components/Inspector.tsx`
: architecture/reference/subreference inspection.

`src/lib/desktop.ts`
: Tauri calls including scan and configuration update.

`src-tauri/src/lib.rs`
: native commands backed by the Rust core.

## Authoring philosophy

Use a normal reference when multiple architecture nodes truly refer to the same named concept and convergence is useful.

Use a subreference when the concept is local explanatory context and graph-wide merging would create misleading hubs.

Examples of useful reference names are not limited to code modules:

```text
Identity
PCI DSS
Data Retention Policy
Password Management
Observability
AWS Region
Incident Management
Customer Data
```

The system is deliberately suitable for application code, infrastructure repositories, systems documentation, or mixed repositories without needing any language plugin.
