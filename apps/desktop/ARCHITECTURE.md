# ArchGraph Desktop

ARCH_NODE:ArchGraph Desktop

ArchGraph Desktop is the interactive Tauri application that turns ArchGraph project scans into a layered architecture workspace. It combines a React presentation layer with a narrow native adapter while preserving ArchGraph Core as the authority for project interpretation and graph composition.

The desktop boundary owns project interaction, layer grouping, layouts, search, filters, graph selection, declaration inspection, source navigation, and opt-in session persistence. It does not implement filesystem scanning or reference-resolution semantics in TypeScript.

## References

ARCH_REFERENCE:Desktop Application

ArchGraph Desktop delegates user-facing project state, refresh behaviour, layer-group arrangement, layout choice, search/filter state, and persistence UX to Desktop Application.

ARCH_REFERENCE:Graph Workspace

ArchGraph Desktop presents composed architecture through Graph Workspace, which renders graph regions and relationships while exposing selection, layer controls, and declaration inspection.

ARCH_REFERENCE:Native Desktop Adapter

ArchGraph Desktop crosses the filesystem and Rust boundary through Native Desktop Adapter for project scans, layer composition, configuration writes, folder selection support, and project-confined source opening.
