# Core Validation

ARCH_NODE:Core Validation

Core Validation defines the behavioural contract around scanning, configuration, parsing, layer boundaries, composition, and serialization. Its fixtures model authored architecture rather than source-language analysis, allowing tests to assert the same rules exposed to real repositories.

Validation covers configuration replacement semantics, source candidates that should disappear silently, decorated declarations, rooted layer discovery, ignored paths, shared references, source-local subreferences, cross-layer resolution, merged declarations, ambiguous names, and diagnostics.

## References

ARCH_REFERENCE:ArchGraph Core

Core Validation exercises ArchGraph Core through its public scanning and composition surface so regressions are detected at the same boundary used by external consumers.

ARCH_REFERENCE:Core Engine

Core Validation drives Core Engine through fixture repositories that force its configuration, discovery, parser dispatch, reference handling, and composition stages to interact as one pipeline.

ARCH_REFERENCE:Project Configuration

Core Validation supplies alternate Project Configuration values to verify alias replacement, layer roots, ignored paths, file globs, normalization, and persisted configuration behaviour.

ARCH_REFERENCE:Source Parsing

Core Validation uses Markdown and decorated source fixtures to verify Source Parsing only enrols explicit declarations and preserves declaration/reference documentation without understanding host-language syntax.

ARCH_REFERENCE:Layer Composition

Core Validation separates and recombines fixture layers to verify Layer Composition changes name resolution only at the requested composition boundary while retaining source provenance.

ARCH_SUBREFERENCE:Fixture Corpus

Core Validation uses a deliberately small fixture corpus to express graph semantics directly, including ignored fixture subtrees that must not leak into scans of the ArchGraph repository itself.
