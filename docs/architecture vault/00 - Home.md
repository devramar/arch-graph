# Architecture Graph Explorer

Working documentation for ArchGraph.

The system has two deliberately separate parts:

1. A standalone Rust core that scans a project and produces a neutral architecture graph plus normalized project configuration.
2. A desktop application that consumes those outputs and provides an interactive Cytoscape.js explorer.

The graph is intentionally based on explicit architectural documentation rather than inferred source-code relationships.

## Start here

- [[10 - Architecture/System Overview]]
- [[10 - Architecture/Rust Core]]
- [[10 - Architecture/Core Graph Model]]
- [[10 - Architecture/Graph Semantics]]
- [[20 - Documentation/ARCHITECTURE Format]]
- [[20 - Documentation/Project Configuration]]
- [[30 - Desktop App/Desktop Application]]
- [[30 - Desktop App/Interaction Model]]
- [[40 - Constraints/Security and Offline]]
- [[50 - Scope/MVP Scope]]
- [[60 - Implementation/Build and Packaging]]
- [[90 - Decisions/Decisions]]

## Core idea

Documented systems are architecture nodes.

Explicit `ARCH_REFERENCE:` declarations are directed graph edges. If the target has no architecture document, ArchGraph creates a valid shared lightweight reference node.

Explicit `ARCH_SUBREFERENCE:` declarations create source-local lightweight nodes that never merge by name.

The prose attached to a reference declaration belongs to the edge and explains why that relationship exists.
