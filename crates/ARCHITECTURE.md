# ArchGraph Rust Workspace

ARCH_NODE:ArchGraph Rust Workspace

The Rust workspace contains the reusable architecture engine and the command-line projection of that engine. It keeps graph semantics in one library boundary while allowing different consumers to expose those semantics without reimplementing them.

The workspace is intentionally asymmetric: `archgraph-core` owns project interpretation and graph construction, while binaries consume its public contract.

## References

ARCH_REFERENCE:ArchGraph Core

The workspace centralizes architecture scanning, configuration, reference semantics, layer composition, diagnostics, and graph construction in ArchGraph Core so every Rust consumer shares one implementation.

ARCH_REFERENCE:ArchGraph CLI

The workspace exposes ArchGraph CLI as a thin executable consumer that turns the core's composed graph into JSON for shell and automation use.

ARCH_REFERENCE:Core Validation

The workspace relies on Core Validation to exercise the public scanning and composition contract against representative project fixtures and configuration boundaries.
