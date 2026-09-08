# Extended ArchGraph Examples

These examples show different ways a repository can partition and author architecture without introducing language-specific parsing.

## Separate architectural roots with the same convention

A monorepo can keep identical `ARCHITECTURE.md` conventions while limiting each layer to a different subtree:

```json
{
  "layers": {
    "architecture": {
      "display_name": "Core Architecture",
      "path_root": "crates",
      "ignored_paths": ["archgraph-core/tests/fixtures/**"],
      "files": ["ARCHITECTURE.md"]
    },
    "desktop": {
      "display_name": "Desktop Architecture",
      "path_root": "apps/desktop",
      "files": ["ARCHITECTURE.md"]
    }
  }
}
```

`files` and layer-level `ignored_paths` are evaluated relative to `path_root`. Source locations in the graph remain project-relative.

This is the configuration pattern used by the ArchGraph repository itself.

## Architecture plus implementation declarations

A broad documentation layer can coexist with an implementation layer that enrols source files:

```json
{
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "path_root": ".",
      "files": ["ARCHITECTURE.md"]
    },
    "implementation": {
      "display_name": "Implementation",
      "path_root": "src",
      "ignored_paths": ["generated/**", "vendor/**"],
      "files": ["*.ts", "*.tsx"]
    }
  }
}
```

Most matching TypeScript files can remain completely unaware of ArchGraph. Only files containing configured markers become architecture sources:

```ts
/// ARCH_NODE:EventStore
///
/// Owns the persisted event state used by the application.
///
/// ARCH_REFERENCE:Event Database
/// EventStore reads and writes durable event records through Event Database.
///
/// ARCH_REFERENCE:DateKey
/// EventStore uses DateKey as the canonical partition identifier.
export class EventStore {
    // implementation
}
```

ArchGraph does not interpret `export class`, imports, interfaces, or any TypeScript syntax. The file is selected by the layer glob; the decorated architecture block supplies all graph meaning.

The same decorated form can be used in any candidate text file when its comment convention produces a stable punctuation prefix:

```rust
/// ARCH_NODE:EventStore
///
/// Owns persisted event state.
```

```python
# ARCH_NODE:EventStore
#
# Owns persisted event state.
```

```sql
-- ARCH_NODE:EventStore
--
-- Owns persisted event state.
```

## Multiple abstraction levels for one concept

Two layers can deliberately declare the same name at different abstraction levels.

A Markdown architecture source might contain:

```text
ARCH_NODE:Identity

Identity is the system boundary for establishing and carrying user identity.
```

An implementation source might contain:

```ts
/// ARCH_NODE:Identity
///
/// Concrete identity adapter used by the desktop application.
```

When the Architecture and Implementation layers are placed in the same desktop group, the declarations merge into one `Identity` node. The inspector retains each declaration as a separate tab. If the layers are placed in different groups, two independent `Identity` nodes remain visible.

## Custom project vocabulary

Marker aliases are configured per layer, allowing a repository to use domain-specific terminology without changing ArchGraph semantics:

```json
{
  "layers": {
    "architecture": {
      "display_name": "System Design",
      "path_root": "design",
      "files": ["SYSTEM.md"],
      "markers": {
        "ARCH_NODE": ["SYSTEM"],
        "ARCH_REFERENCE": ["USES", "CONNECTS_TO"],
        "ARCH_SUBREFERENCE": ["LOCAL_CONCERN"]
      }
    }
  }
}
```

A matching source can then contain:

```text
SYSTEM:Checkout

Checkout coordinates the customer purchase path.

USES:Identity

Checkout asks Identity for the current customer before creating an order.

LOCAL_CONCERN:PCI Boundary

Checkout keeps payment-card handling inside this local architectural concern.
```

Configured alias lists replace the canonical spelling for that layer. Include `ARCH_REFERENCE` in the list as well if both canonical and custom spellings should be accepted.

## Global and layer-local exclusions

Top-level ignores are project-root-relative and apply to every layer. Layer-local ignores are relative to that layer's `path_root`:

```json
{
  "ignored_paths": [
    "vendor/**",
    "archive/**"
  ],
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "path_root": ".",
      "ignored_paths": ["crates/archgraph-core/tests/fixtures/**"],
      "files": ["ARCHITECTURE.md"]
    },
    "implementation": {
      "display_name": "Implementation",
      "path_root": "src",
      "ignored_paths": ["generated/**"],
      "files": ["*.ts"]
    }
  }
}
```

The implementation rule above excludes `src/generated/**`, while the global rules exclude `vendor/**` and `archive/**` regardless of layer.

ArchGraph also honours its built-in excluded directories, Git ignore rules, and `.archgraphignore`.

## Overview and implementation visible at the same time

Layers do not need to represent increasingly detailed versions of the same architecture. They can provide parallel views:

```json
{
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "path_root": "architecture",
      "files": ["ARCHITECTURE.md"]
    },
    "implementation": {
      "display_name": "Implementation",
      "path_root": "src",
      "files": ["*.rs"]
    },
    "overview": {
      "display_name": "Overview",
      "path_root": "docs/overview",
      "files": ["*.md"],
      "markers": {
        "ARCH_NODE": ["OVERVIEW_NODE"],
        "ARCH_REFERENCE": ["OVERVIEW_REFERENCE"],
        "ARCH_SUBREFERENCE": ["OVERVIEW_SUBREFERENCE"]
      }
    }
  }
}
```

The desktop can show an Architecture + Implementation composition group alongside a separate Overview group. Matching names merge only inside each group, so displaying several abstraction spaces does not force them into one graph identity space.

## Shared references and local subreferences

Use a normal reference when a name represents a concept that should converge when a matching architecture declaration exists:

```text
ARCH_REFERENCE:Authentication

Checkout asks Authentication to establish the current customer identity.
```

If several nodes reference `Authentication`, they share one lightweight reference node until an `ARCH_NODE:Authentication` declaration is available in the same composition group.

Use a subreference when the visual concept belongs only to the declaring node:

```text
ARCH_SUBREFERENCE:Password Management

This service delegates password storage to its own managed-secret boundary.
```

Another source can also declare `ARCH_SUBREFERENCE:Password Management`; the two nodes remain distinct even though the desktop gives same-named subreferences the same deterministic colour family.

## Persisted desktop grouping

Desktop layer grouping and layout state can be stored inside the otherwise application-opaque `view_settings` object:

```json
{
  "layers": {
    "architecture": {
      "display_name": "Architecture",
      "path_root": "architecture",
      "files": ["ARCHITECTURE.md"]
    },
    "implementation": {
      "display_name": "Implementation",
      "path_root": "src",
      "files": ["*.ts"]
    },
    "overview": {
      "display_name": "Overview",
      "path_root": "overview",
      "files": ["*.md"]
    }
  },
  "default_view": "sticky",
  "view_settings": {
    "desktop": {
      "layer_groups": [
        {
          "id": "system",
          "name": "System",
          "enabled": true,
          "layer_ids": ["architecture", "implementation"]
        },
        {
          "id": "overview",
          "name": "Overview",
          "enabled": false,
          "layer_ids": ["overview"]
        }
      ]
    },
    "sticky": {
      "reference_distance": 180,
      "subreference_distance": 70,
      "subreference_attraction": 1.8
    }
  }
}
```

Without a root `.archgraph`, these desktop changes remain in-memory for the current session. The desktop can explicitly create `.archgraph` when persistent project state is wanted.

## Reference colour overrides

The desktop has a built-in deterministic palette. Explicit names can be mapped to built-in colour families independently for references and subreferences:

```json
{
  "app_colours": {
    "colour_overrides": {
      "references": {
        "Authentication": "purple"
      },
      "subreferences": {
        "Password Management": "teal"
      }
    }
  }
}
```

Every edge arrow uses the primary colour of its destination node. Same-name subreferences remain separate nodes but share their colour identity.

The desktop currently exposes these built-in colour names for overrides: `purple`, `blue`, `teal`, `green`, `orange`, `pink`, `red`, and `indigo`.
