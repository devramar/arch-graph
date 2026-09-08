# Graph Workspace

ARCH_NODE:Graph Workspace

Graph Workspace is the interactive presentation boundary formed by `GraphCanvas`, `Sidebar`, `Inspector`, and the constrained Markdown renderer. It turns a composed graph into spatial structure and gives users direct access to the architecture declarations and reference prose carried by that graph.

`GraphCanvas` maps neutral graph nodes and edges into Cytoscape elements, applies deterministic reference colours, colours every arrow from its destination node, runs directed/organic/sticky layouts, packs independent composition groups, and draws soft labelled regions around those groups. `Sidebar` controls graph filtering and layer-group membership. `Inspector` exposes node metrics, edge documentation, source locations, and declaration tabs for merged nodes. `MarkdownDocument` renders embedded architecture Markdown without becoming a general-purpose browser.

## References

ARCH_REFERENCE:Desktop Application

Graph Workspace receives current graph/configuration state and mutation callbacks from Desktop Application, reporting selection, filter, layer-group, and source-open interactions upward rather than owning project lifecycle state.

ARCH_REFERENCE:Graph Contract

Graph Workspace renders Graph Contract node kinds, reference scopes, group provenance, declaration sources, and edge documentation directly into visual and inspector behaviour.

ARCH_REFERENCE:Layer Composition

Graph Workspace treats Layer Composition groups as visible regions: declarations merged by the core appear as one node, while enabled groups kept separate by the user remain distinct spatial regions at the same time.

ARCH_SUBREFERENCE:Cytoscape Rendering

Graph Workspace translates each graph node and edge into Cytoscape data, preserving destination colour on arrows and applying distinct treatments for architecture nodes, shared references, and local subreferences.

ARCH_SUBREFERENCE:Declaration Tabs

Graph Workspace presents each contributing declaration of a merged architecture node as an independent tab so layer-specific descriptions remain readable after composition.

ARCH_SUBREFERENCE:Layer Group Drag and Drop

Graph Workspace lets users move layers between composition groups in the sidebar; the resulting group definition is sent back to Desktop Application for core-owned recomposition.
