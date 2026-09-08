# ArchGraph Core

ARCH_NODE:ArchGraph Core

ArchGraph Core is the language-agnostic Rust engine that interprets explicitly authored architecture declarations and produces presentation-neutral layered graphs.

Its public boundary covers project configuration, source discovery, declaration parsing, reference semantics, layer construction, graph composition, diagnostics, and safe `.archgraph` persistence. It does not infer architecture from imports, exports, package metadata, ASTs, or programming-language semantics.

The crate exposes high-level scan and composition operations while keeping the parsing, resolution, path handling, and graph-building machinery internal.

## References

ARCH_REFERENCE:Core Engine

ArchGraph Core delegates project interpretation and graph construction to Core Engine, which coordinates configuration, discovery, parsing, per-layer resolution, and final composition behind the crate's exported API.

ARCH_REFERENCE:Project Configuration

ArchGraph Core loads and validates Project Configuration before discovery so layer roots, ignored paths, file globs, marker aliases, application colour overrides, and opaque view settings are applied consistently across consumers.

ARCH_REFERENCE:Graph Contract

ArchGraph Core emits Graph Contract values as the stable presentation-neutral result of scanning and composition, allowing CLI and desktop consumers to share the same node, edge, diagnostic, layer, and declaration semantics.

ARCH_REFERENCE:Layer Composition

ArchGraph Core composes independently scanned layers through Layer Composition so same-name declarations merge only inside explicit groups and references are resolved against the declarations visible in each group.
