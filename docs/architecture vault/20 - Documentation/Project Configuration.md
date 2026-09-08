# Project Configuration

A project may define one root `.archgraph` strict-JSON file. Nested configuration/inheritance is not supported in this version.

If the file is absent, built-in defaults apply.

## Example

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

## Aliasing semantics

Each configured list replaces the default accepted spelling for that semantic concept.

For example:

```json
"ARCH_REFERENCE": ["SYS_REF", "ARCH_REF"]
```

means `ARCH_REFERENCE:` is no longer recognized unless it is explicitly included in the list.

Aliases are normalized and must not collide across node/reference/subreference concepts. Architecture document aliases must be filenames rather than paths, and `.archgraph` itself is reserved for project configuration.

## App colours

`colour_overrides.references` and `colour_overrides.subreferences` map explicit node names to application palette names.

The desktop currently supplies a fixed pretty palette (`purple`, `blue`, `teal`, `green`, `orange`, `pink`, `red`, `indigo`). Unknown names fall back to deterministic name hashing.

## Views

`default_view` is an application-consumed string.

`view_settings` is deliberately an opaque object from the Rust core's perspective. Each app may define the keys it understands. The desktop currently understands:

- `reference_distance`
- `subreference_distance`
- `node_spacing`
- `subreference_attraction`

for `directed`, `organic`, and `sticky` view objects where applicable. Higher `subreference_attraction` values pull local subreferences more strongly in force-directed layouts.

## Writes

Apps should write `.archgraph` through the Rust core rather than directly accessing the project filesystem. The core validates and normalizes configuration before replacing the file. Apps should rescan after a successful write because syntax aliases can change which documents and markers participate in the graph.
