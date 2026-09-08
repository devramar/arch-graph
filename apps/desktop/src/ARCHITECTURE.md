# Desktop Application

ARCH_NODE:Desktop Application

Desktop Application is the React state and orchestration boundary centred on `App.tsx`, with `types.ts` defining the frontend view of the core contract and the application shell coordinating the graph workspace.

It owns the loaded `ProjectScan`, active composed graph, layer groups, layout, search/filter state, selection, project refresh state, configuration-save queue, and session-aware versus session-only behaviour. It composes enabled layer groups through the native service boundary rather than joining nodes itself.

Opening a project establishes state from the scan's configuration. Refreshing a configured project reloads persisted settings; refreshing a project without `.archgraph` rescans source data while retaining the current in-memory grouping and layout. Creating `.archgraph` promotes the current desktop session into persisted project state.

## References

ARCH_REFERENCE:Desktop Services

Desktop Application sends scan, composition, configuration-write, source-open, folder-picker, and drag/drop-runtime operations through Desktop Services instead of invoking native APIs throughout the component tree.

ARCH_REFERENCE:Graph Workspace

Desktop Application supplies Graph Workspace with the current composed graph, configuration, filters, layout, and selection callbacks, and receives layer-group, node, edge, and source-navigation interactions back into application state.

ARCH_REFERENCE:Graph Contract

Desktop Application stores and passes Graph Contract objects without changing node/reference meaning, using the contract as the authoritative representation for graph rendering and inspection.

ARCH_REFERENCE:Layer Composition

Desktop Application converts enabled desktop layer groups into Layer Composition requests and replaces the displayed graph with the core-produced composition result whenever grouping changes.

ARCH_REFERENCE:Project Configuration

Desktop Application derives the default layout and saved desktop grouping from Project Configuration, and writes updated view state only when configuration persistence is enabled or explicitly created.

ARCH_SUBREFERENCE:Session Persistence Queue

Desktop Application serializes `.archgraph` view-state writes through a promise queue so rapid grouping or layout changes preserve write order and update the active configuration state after each successful native write.
