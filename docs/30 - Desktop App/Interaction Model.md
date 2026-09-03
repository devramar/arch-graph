# Interaction Model

## Input philosophy

The application is optimized for mouse and keyboard use on a desktop.

## Panning

All of the following pan the graph:

- left mouse drag on empty graph space
- middle mouse drag
- Space + drag

Directly dragging a node moves the node unless Space is held.

## Zoom

Mouse wheel controls graph zoom at a deliberately strong desktop sensitivity.

Shift + wheel applies a larger zoom step while preserving the cursor as the zoom focal point.

## Selection

Single click selects a node or relationship.

Selection styling must not alter node geometry.

## Declarations

Nodes with source declarations and relationships with declaration locations can be opened with the operating system's default application.

Access is available from:

- right-click context menu on a graph element
- **Open in editor** beside Source/Declaration in the inspector

The desktop adapter validates that the requested file remains inside the currently selected project root before opening it.

## Relationship hover

Hovering a relationship shows a cursor-following preview containing:

- source node
- target node
- dependency description

An optional **Descriptions** panel at the bottom of the graph presents the same documentation in a larger stable surface. It follows the hovered relationship and falls back to the selected relationship when nothing is hovered.

## Search

Search should support finding nodes by canonical name and rapidly isolating their local dependency neighborhood.

## Keyboard interactions

- `/` focuses search
- `Esc` clears current selection/context menu
- `F` fits the visible graph
- `Space + drag` pans
- `Shift + wheel` zooms faster

## Layout

Three layout modes are available:

- **Dependency** — hierarchical directed layout
- **Organic** — force-directed fCoSE layout
- **Sticky** — fCoSE layout that performs an incremental settling pass after a node is manually repositioned, approximating the sticky/simulation behavior of tools such as Obsidian graph view
