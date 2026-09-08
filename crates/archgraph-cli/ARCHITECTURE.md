# ArchGraph CLI

ARCH_NODE:ArchGraph CLI

ArchGraph CLI is the minimal command-line projection of ArchGraph Core. It accepts one project root, requests the core's default composed graph, serializes that graph as formatted JSON, and maps scan or serialization failures to process exit status.

The binary intentionally has no independent discovery, parsing, resolution, or configuration semantics.

## References

ARCH_REFERENCE:ArchGraph Core

ArchGraph CLI passes the requested project root directly to ArchGraph Core and treats the returned composed graph or scan error as authoritative rather than interpreting repository contents itself.

ARCH_REFERENCE:Graph Contract

ArchGraph CLI serializes Graph Contract output without adapting its architecture meaning, making the command-line interface a direct machine-readable view of the core model.
