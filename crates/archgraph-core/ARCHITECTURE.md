# ArchGraph Core

ARCH_NODE:ArchGraph Core

ArchGraph Core is the language-agnostic Rust engine that turns explicitly authored architecture declarations into independently scanned layers and presentation-neutral composed graphs.

## System boundary

The core owns project configuration, filesystem discovery, source parser selection, architecture declaration extraction, reference semantics, layer construction, graph composition, diagnostics, and `.archgraph` writes. Consumers receive serialized graph data and do not need to reproduce those semantics.

The core does not infer architecture from source imports, exports, symbols, package metadata, or language-specific syntax.

## Project configuration

A single optional `.archgraph` file at the selected project root defines global ignored paths, logical layers, colour-name overrides, a default view, and opaque application view settings. Configuration is normalized and validated by the core before scanning or writing.

Each layer has a stable id, display name, literal project-relative `path_root`, layer-relative ignored-path globs, source file globs, and marker aliases. The built-in `architecture` layer is restored when omitted and defaults to project-wide `ARCHITECTURE.md` discovery.

Configuration writes use a normalized JSON representation and a temporary-file replacement path so applications do not write project settings directly.

## Discovery and parsing

Project traversal respects built-in excluded directories, Git ignore data, `.archgraphignore`, global configured ignores, layer roots, and layer-local ignores. Symlinks are not followed.

A source file is considered only inside a matching layer root and only when a configured file glob matches its path relative to that root. A source claimed by more than one layer is skipped with a diagnostic rather than assigned implicitly. Candidate files containing no ArchGraph markers are ignored.

Parser dispatch is source-format based. Markdown files use the Markdown parser. Other files use decorated-text parsing, which recognizes marker lines behind a consistent punctuation decoration prefix and extracts adjacent text with the same decoration. Parser dispatch is centralized so additional source-format parsers can be introduced without changing layer or graph semantics.

One source currently owns at most one architecture declaration.

## References

Normal references are explicit, mergeable named relationships. During composition they target a unique same-name architecture declaration in the current group when one exists; otherwise they target a shared lightweight reference node. Missing targets are valid graph content rather than unresolved errors.

Subreferences are source-local relationships. Their target nodes remain unique to the originating declaration, never resolve onto architecture nodes, and never merge by name.

Reference prose is edge documentation: it describes how the source declaration interacts with the referenced target.

## Layers and composition

Layers are scanned independently into graph fragments with declaration provenance. Composition groups are then built from selected layers without rescanning the filesystem.

Same-name architecture declarations merge only inside the same composition group. A merged node retains every contributing declaration, including its layer, project-relative source location, source format, and extracted documentation. Identical names in separate groups remain separate graph nodes.

Reference resolution is repeated at composition boundaries so a reference authored in one layer can resolve to a declaration supplied by another layer when both participate in the same group.

## Graph contract

The serialized graph separates architecture nodes from lightweight reference nodes and records reference versus subreference edges. Nodes and edges carry composition-group provenance; architecture declarations carry layer provenance.

Diagnostics remain part of the graph result so malformed or ambiguous architecture data can be inspected without making every documentation issue fatal to scanning.
