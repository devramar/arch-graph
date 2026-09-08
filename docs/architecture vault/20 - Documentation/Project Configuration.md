# Project Configuration

## File

ArchGraph optionally reads strict JSON from root `.archgraph`.

Without the file, canonical defaults are used and the desktop keeps grouping/layout changes in memory only.

## Example

```json
{
  "ignored_paths": [
    "crates/archgraph-core/tests/fixtures/**",
    "**/generated/**"
  ],
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "files": ["DOCUMENTATION.md"],
      "markers": {
        "ARCH_NODE": ["SYS_NODE"],
        "ARCH_REFERENCE": ["SYS_REF", "ARCH_REF"],
        "ARCH_SUBREFERENCE": ["SYS_LOCAL"]
      }
    },
    "implementation": {
      "display_name": "Implementation",
      "files": ["*.ts", "*.tsx"]
    }
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
    },
    "desktop": {
      "layer_groups": [
        {
          "id": "system",
          "name": "Architecture + Implementation",
          "enabled": true,
          "layer_ids": ["architecture", "implementation"]
        }
      ]
    }
  }
}
```

## Layers

`layers` maps stable layer IDs to discovery configuration.

Each layer supports:

- `display_name`
- `files`: candidate globs
- `markers.ARCH_NODE`
- `markers.ARCH_REFERENCE`
- `markers.ARCH_SUBREFERENCE`

The `architecture` layer always exists. If it is omitted, ArchGraph restores:

```text
display_name = Architecture
files        = ARCHITECTURE.md
markers      = canonical ARCH_* tokens
```

Other layers must define at least one file glob. Missing marker lists use canonical marker names.

Marker aliases replace the canonical list for that layer when explicitly supplied. Aliases cannot collide across node/reference/subreference concepts inside one layer.

## Globs

The current matcher supports simple path globs:

- `*` — zero or more characters within one path segment
- `?` — one character within one path segment
- `**` — zero or more path segments

Examples:

```text
*.ts
src/**/*.rs
crates/**/ARCHITECTURE.md
```

Basename-only file patterns such as `*.ts` match at any project depth.

A source matching more than one layer is skipped with `ARCH007` rather than guessed.

## Ignored paths

`ignored_paths` uses the same simple glob matcher and combines with:

- built-in exclusions (`.git`, `node_modules`, `dist`, `build`, `target`, `.expo`, `.next`)
- Git ignore rules
- `.archgraphignore`

## App colours

`colour_overrides.references` and `colour_overrides.subreferences` map explicit names to application palette names.

The desktop currently supplies: `purple`, `blue`, `teal`, `green`, `orange`, `pink`, `red`, `indigo`.

## Views and desktop session state

`default_view` is an application-consumed string.

`view_settings` is deliberately opaque to the Rust core. The desktop currently stores:

- layout tuning (`reference_distance`, `subreference_distance`, `node_spacing`, `subreference_attraction`)
- `desktop.layer_groups` for persisted grouping/visibility

Without `.archgraph`, the desktop does not write session state. Its Create `.archgraph` action writes the current configuration plus current grouping/layout through the core.

## Writes

Apps should write `.archgraph` through `write_project_configuration(...)` / the Tauri configuration command. The core normalizes and validates strict JSON before atomically replacing the file.
