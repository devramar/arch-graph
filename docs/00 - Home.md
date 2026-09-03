# Architecture Graph Explorer

Working documentation checkpoint for the architecture graph tooling.

The system has two deliberately separate parts:

1. A standalone Rust core that scans a project and produces a neutral architecture graph.
2. A desktop application that consumes that graph and provides an interactive Cytoscape.js explorer.

The graph is intentionally based on explicit architectural documentation rather than inferred import graphs.

## Start here

- [[10 - Architecture/System Overview]]
- [[10 - Architecture/Rust Core]]
- [[10 - Architecture/Core Graph Model]]
- [[20 - Documentation/ARCHITECTURE Format]]
- [[30 - Desktop App/Desktop Application]]
- [[30 - Desktop App/Interaction Model]]
- [[40 - Constraints/Security and Offline]]
- [[50 - Scope/MVP Scope]]
- [[60 - Implementation/Build and Packaging]]
- [[90 - Decisions/Decisions]]

## Core idea

Documented systems are graph nodes.

Dependencies declared by those systems are directed graph edges.

Small modules may appear as graph nodes without needing their own `ARCHITECTURE.md`.

The prose attached to an `ARCH_DEPENDENCY:` declaration belongs to the edge and explains why that relationship exists.

## Implementation

- [[60 - Implementation/Repository Map]]
- [[60 - Implementation/Validation Checklist]]
- [[../RUNNING]]
- [[../HANDOVER]]
