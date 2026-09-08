# Validation Checklist

See `docs/HANDOVER.md` for the current validation round.

Minimum acceptance checks:

- Rust workspace formats, tests, and passes strict Clippy.
- Frontend production build passes.
- Tauri app starts.
- default `ARCHITECTURE.md` / `ARCH_*` markers scan correctly.
- candidate globs ignore files without markers.
- decorated `///`, `#`, `--`, and block-comment-style declarations parse without language semantics.
- configured ignored paths suppress matching source trees.
- sources matching multiple layers produce `ARCH007` and are skipped.
- one-file/multiple-node declarations produce `ARCH006`.
- cross-layer matching names merge only when layers share a composition group.
- separate enabled groups retain same-name nodes separately and remain simultaneously visible.
- merged nodes expose all source declarations in inspector tabs.
- undocumented normal references render without unresolved diagnostics.
- same-name subreferences remain separate nodes with matching colour identity.
- destination arrow colours match destination node borders.
- Sticky layout keeps subreferences closer than normal references.
- projects without `.archgraph` do not persist desktop grouping/layout.
- Create `.archgraph` persists current session through the Rust core.
- projects with `.archgraph` restore/persist desktop layer groups.
- source opening remains root-confined.
