# Interaction Model

## Input philosophy

The application is optimized for mouse and keyboard use on a desktop.

## Panning

All of the following should drag/pan the graph:

- left mouse button held on empty graph space
- middle mouse button held
- Space held while dragging

Node dragging remains available when directly dragging a node.

Exact conflict behavior between left-drag panning and node dragging should be resolved in favor of intuitive desktop behavior.

## Zoom

Mouse wheel controls graph zoom.

## Selection

Single click selects a node or edge.

Selected relationships open in the inspector.

## Edge hover

Hovering an edge should show a compact preview containing:

- source node
- target node
- dependency description

## Edge inspector

Clicking an edge should expose:

- source
- target
- full dependency description
- architecture source file
- source line when available

## Search

Search should support finding nodes by canonical name and rapidly isolating their local dependency neighborhood.

## Useful keyboard interactions

Initial candidates:

- `/` focuses search
- `Esc` clears current selection
- `F` fits the visible graph

These are not yet frozen.

## Layout

The graph should support automatic layout plus manual repositioning.

Likely automatic strategies:

- hierarchical layout for suitable directed graphs
- force-directed layout for cyclic or irregular graphs
