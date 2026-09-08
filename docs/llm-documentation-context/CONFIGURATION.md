# `.archgraph` Discovery Context

Before creating or moving architecture declarations, inspect the target repository's root `.archgraph` when one exists.

The file is strict JSON and can define multiple logical layers. Each layer can change where and how architecture sources are discovered:

```json
{
  "ignored_paths": ["archive/**"],
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "path_root": "architecture",
      "ignored_paths": ["old/**"],
      "files": ["ARCHITECTURE.md"],
      "markers": {
        "ARCH_NODE": ["ARCH_NODE"],
        "ARCH_REFERENCE": ["ARCH_REFERENCE"],
        "ARCH_SUBREFERENCE": ["ARCH_SUBREFERENCE"]
      }
    },
    "implementation": {
      "display_name": "Implementation",
      "path_root": "src",
      "files": ["*.ts", "*.tsx"]
    }
  }
}
```

## Layer roots

`path_root` is a literal directory relative to the selected project root. File patterns and that layer's `ignored_paths` are evaluated relative to `path_root`.

Top-level `ignored_paths` are evaluated relative to the project root and affect every layer.

Do not place architecture material in excluded paths unless the task explicitly includes changing configuration.

When `path_root` is omitted it normalizes to the selected project root. Layer-local `ignored_paths` default to none.

## Candidate files

`files` contains simple globs using `*`, `?`, and `**`. A matching file is only a candidate; it becomes an architecture source only when it contains configured ArchGraph markers.

Markdown candidates use Markdown parsing. Other candidates use decorated-text parsing.

## Marker aliases

The JSON keys retain the canonical semantic names while their arrays define the accepted concrete spellings for that layer:

```json
{
  "layers": {
    "architecture": {
      "files": ["ARCHITECTURE.md"],
      "markers": {
        "ARCH_NODE": ["SYSTEM"],
        "ARCH_REFERENCE": ["USES", "CONNECTS_TO"],
        "ARCH_SUBREFERENCE": ["LOCAL"]
      }
    }
  }
}
```

If a marker list is configured, write one of those configured spellings. Do not assume canonical `ARCH_*` text remains accepted unless it appears in the list or the layer inherits the canonical defaults.

When a layer omits `markers`, the canonical `ARCH_NODE`, `ARCH_REFERENCE`, and `ARCH_SUBREFERENCE` spellings are inherited.

## Built-in architecture layer

ArchGraph always has an `architecture` layer. Without custom configuration it scans project-wide `ARCHITECTURE.md` files using the canonical markers.

A repository can override that layer's display name, root, ignores, files, and markers and can add more layers beside it.
