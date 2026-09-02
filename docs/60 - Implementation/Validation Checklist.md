# Validation Checklist

See [[../../HANDOVER]] for the complete first-run validation process.

Minimum acceptance checks:

- Rust workspace compiles and tests pass.
- Frontend TypeScript build passes.
- Tauri app starts.
- Folder drag/drop scans a root.
- `ARCH_NODE:` creates architecture nodes.
- bare `ARCH_DEPENDENCY:` resolves architecture and TypeScript module targets.
- dependency prose appears on edge hover and edge inspection.
- ambiguity/unresolved states produce diagnostics rather than silent guesses.
- left background drag, middle drag, and Space+drag pan.
- direct node drag still moves nodes.
- all core functionality works offline.
