# Core Engine

ARCH_NODE:Core Engine

Core Engine is the internal pipeline implemented by `config.rs`, `model.rs`, `parser.rs`, `resolver.rs`, and `scanner.rs`. Together those modules turn a project root into independently scanned architecture layers and then into one or more composed graph regions.

`config.rs` establishes the project contract and safe write path. `scanner.rs` applies filesystem and layer boundaries, dispatches source parsing, builds per-layer graph fragments, and coordinates composition. `parser.rs` extracts one declaration and its references from Markdown or decorated text without interpreting the host language. `resolver.rs` resolves normal references within a layer scan while preserving local subreference identity. `model.rs` defines the serialized graph, declaration, diagnostic, layer, and composition types shared across the crate boundary.

The modules deliberately exchange explicit data structures rather than source-language concepts. A candidate file matters only when a configured marker declares architecture.

## References

ARCH_REFERENCE:Project Configuration

Core Engine reads Project Configuration before traversal and uses its normalized layer roots, ignore globs, candidate-file globs, and marker aliases to determine which sources may contribute declarations.

ARCH_REFERENCE:Source Parsing

Core Engine sends matching candidate sources through Source Parsing and consumes only explicit node, reference, subreference, documentation, source-format, and diagnostic results; candidate files without markers disappear from the graph pipeline.

ARCH_REFERENCE:Reference Resolution

Core Engine passes parsed reference declarations through Reference Resolution so unique architecture targets are preferred, missing normal targets become shared lightweight references, and subreferences remain source-local identities.

ARCH_REFERENCE:Layer Composition

Core Engine hands independent layer graphs to Layer Composition, which merges compatible same-name declarations inside each requested group and re-resolves normal references against the group's combined declaration set.

ARCH_REFERENCE:Graph Contract

Core Engine constructs Graph Contract values from the normalized project identity, composition groups, architecture and reference nodes, documented edges, declaration provenance, and diagnostics produced by the pipeline.

ARCH_SUBREFERENCE:Filesystem Discovery

Core Engine constrains traversal through canonical project roots, built-in excluded directories, Git ignore data, `.archgraphignore`, global ignores, per-layer roots, and per-layer ignores before any source is parsed.

ARCH_SUBREFERENCE:Glob Matching

Core Engine evaluates configured candidate-file and ignore patterns in the coordinate space of each layer root so discovery remains generic without introducing language-specific file handling.
