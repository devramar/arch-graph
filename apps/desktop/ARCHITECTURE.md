# ArchGraph Desktop

ARCH_NODE:ArchGraph Desktop

ArchGraph Desktop is the Tauri, React, and Cytoscape application that presents ArchGraph Core project scans as an interactive layered architecture workspace.

## System boundary

The frontend owns presentation state, project interaction, layer-group arrangement, graph layouts, search and filtering, selection, inspector state, and desktop persistence UX. It does not traverse project files or implement reference-resolution and layer-composition rules.

A narrow Tauri adapter exposes project scanning, layer composition, normalized `.archgraph` updates, and project-confined source opening.

## Project lifecycle

A project can be opened through the native folder picker or desktop drag-and-drop. The resulting scan contains normalized project configuration, configuration-presence state, independently scanned layers, diagnostics, and a default composed graph.

Refresh Project rescans the currently loaded root through the same core scan path. Projects with `.archgraph` reload persisted configuration. Projects without `.archgraph` keep their in-memory layer grouping and layout while refreshing source data.

Without `.archgraph`, desktop grouping and layout state is session-only. Creating `.archgraph` from the desktop promotes the current session into persisted project configuration; subsequent changes are written through the core.

## Layer workspace

Discovered layers can be moved between composition groups. Layers in the same group allow same-name declarations to merge; separate enabled groups remain independent and can be visible at the same time.

Each enabled composition group is rendered as a soft labelled graph region. The region represents the composition boundary rather than an individual source layer.

Merged architecture nodes retain all source declarations. The inspector presents those declarations as tabs so multiple architectural descriptions of one concept remain independently readable.

## Graph presentation

Cytoscape renders architecture nodes, shared references, and local subreferences from the neutral core graph. Reference colour selection is deterministic by name unless overridden by `.archgraph`. Every edge arrow uses the primary colour of its destination node.

Local subreferences use a smaller dashed elliptical treatment and shorter force-layout distances, visually keeping them closer to their source node than ordinary references.

Directed, organic, and sticky layouts consume application-specific values from `view_settings` without moving rendering semantics into the Rust graph model.

## References

ARCH_REFERENCE:ArchGraph Core

The desktop requests project scans, composition of the currently enabled layer groups, and validated `.archgraph` writes through the native adapter; it consumes the resulting graph contracts instead of recreating core architecture semantics in TypeScript.
