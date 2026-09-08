# Validation Checklist

See `docs/HANDOVER.md` for the current validation round.

Minimum acceptance checks:

- Rust workspace formats, compiles, clippy-checks, and tests.
- Frontend TypeScript production build passes.
- Tauri app starts.
- default `ARCH_NODE`, `ARCH_REFERENCE`, and `ARCH_SUBREFERENCE` markers scan correctly.
- `.archgraph` filename/marker replacement aliases scan correctly.
- a matching TypeScript source symbol does not affect reference resolution.
- undocumented normal references render without unresolved diagnostics.
- unique documented targets receive normal reference edges.
- same-name subreferences remain separate nodes with matching colour identity.
- destination arrow colours match destination node borders.
- Sticky layout keeps subreferences substantially closer than normal references.
- reference/subreference colour overrides are respected.
- source opening remains root-confined.
- configuration writes update only root `.archgraph` through the Rust core.
