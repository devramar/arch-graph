# Interaction Model

## Panning and zoom

- left mouse drag on empty graph space — pan
- middle mouse drag — pan
- Space + drag — pan
- mouse wheel — zoom
- Shift + wheel — stronger zoom

Directly dragging a node moves the node unless Space is held.

## Selection

Single click selects a node or reference edge. Selection styling does not alter node geometry.

## Declarations

Architecture nodes with source declarations and reference edges with declaration locations can be opened with the operating system's default application. The desktop adapter validates that the file remains inside the selected project root.

## Reference hover

Hovering an edge shows:

- source node
- target node
- reference description

The optional Descriptions panel shows the same edge-owned prose in a stable surface.

## Reference nodes

Selecting an undocumented shared reference explains that no architecture document describes the explicit name and lists architectures that reference it.

Selecting a local subreference explains that it is declaration-scoped and never merges by name.

## Search

Search finds nodes by canonical/explicit name and isolates their local graph neighborhood.

## Layouts

- **Directed** — left-to-right hierarchical layout
- **Organic** — force-directed fCoSE layout
- **Sticky** — force-directed layout with incremental settling after manual repositioning

Normal reference spacing is deliberately larger than the previous implementation. Subreference edges use shorter ideal lengths and stronger attraction so local satellites remain visually close to their owner.

Desktop-specific layout tuning may be supplied through `.archgraph` `view_settings`.
