# Interaction Model

## Layer groups

Each discovered layer appears in exactly one desktop group.

- drag a layer onto another group → merge their matching names
- drag a layer onto the new-group drop area → split it into its own group
- uncheck a group → hide the entire region
- keep several groups enabled → view them simultaneously

Each enabled group renders as a soft labeled rectangle/region. The graph layout packs independent regions apart for clarity.

## Merged nodes

When several layers in one group declare the same unique name, the graph contains one architecture node with multiple declarations.

The inspector renders one tab per declaration, labelled by layer display name. Each tab has its own source path and documentation.

## Persistence

A project without `.archgraph` is session-only. Grouping and layout changes do not touch the project filesystem.

The sidebar action `Create .archgraph · save this session` creates root configuration through the Rust core and enables future persistence.

## Panning and zoom

- left mouse drag on empty graph space — pan
- middle mouse drag — pan
- Space + drag — pan
- mouse wheel — zoom
- Shift + wheel — stronger zoom

Direct node drag remains available.

## Reference interaction

Hovering an edge shows its source, target, and edge-owned reference description. Selecting a reference node explains whether it is an undocumented shared reference or declaration-local subreference.

## Layouts

- **Directed** — left-to-right hierarchical
- **Organic** — force-directed fCoSE
- **Sticky** — force-directed with incremental settling after manual repositioning

Subreferences use shorter ideal edge lengths and stronger attraction than normal references.
