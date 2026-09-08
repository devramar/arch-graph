# Example Usage: Scan ArchGraph with ArchGraph

This repository documents its own major systems, making it a useful first project to open in ArchGraph.

## Build and run

```bash
cd apps/desktop
npm install
cd ../..
./scripts/check.sh
cd apps/desktop
npm run tauri dev
```

## Default architecture declarations

Without `.archgraph`, ArchGraph scans `ARCHITECTURE.md` using the canonical markers:

```markdown
# Event Synchronization

ARCH_NODE:EventSync

## References

ARCH_REFERENCE:EventStore

Uses the event store as the local source of persisted event state.

ARCH_SUBREFERENCE:Password Management

Password handling is intentionally represented only as a local concern of this architecture node.
```

`## References` is human-facing convention only. Markers provide the machine semantics.

A normal reference without a matching declaration is a lightweight reference node, not an unresolved error. A subreference is a smaller local node and never merges by name.

## Add an implementation layer

Create a root `.archgraph`:

```json
{
  "layers": {
    "implementation": {
      "display_name": "Implementation",
      "files": ["*.ts", "*.tsx"]
    }
  }
}
```

The built-in `architecture` layer remains active automatically.

Now source files can opt in without ArchGraph understanding TypeScript:

```ts
/// ARCH_NODE:DateKey
///
/// Canonical YYYY-MM-DD representation.
///
/// ARCH_REFERENCE:Clock
/// Used to derive today's date.
export type DateKey = `${number}-${number}-${number}`;
```

Unannotated matching `.ts` files are ignored silently.

## Compose layers in the desktop

With `Architecture` and `Implementation` visible as separate groups, two `DateKey` declarations remain separate nodes. Drag `Implementation` onto `Architecture` and matching names merge inside one region.

Selecting a merged node exposes one inspector tab per source declaration, so broad architecture documentation and implementation-level documentation can coexist without overwriting each other.

Disable a group to hide it while leaving other groups visible.

If `.archgraph` exists, grouping and layout changes are saved. If it does not exist, all desktop changes are session-only and the sidebar offers a button to create `.archgraph` from the current session.

## Ignore paths

Repositories can exclude fixtures/generated trees from discovery:

```json
{
  "ignored_paths": [
    "crates/archgraph-core/tests/fixtures/**",
    "**/generated/**"
  ]
}
```

See the architecture-format and project-configuration documents under `docs/architecture vault/20 - Documentation/` for the complete rules.
