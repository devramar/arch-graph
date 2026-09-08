# ArchGraph Documentation Authoring Context

This folder is intended to be copied or zipped and supplied to an LLM together with a target repository when asking it to create or revise ArchGraph architecture documentation.

Its purpose is narrow: give the model the semantic rules needed to produce correct ArchGraph declarations without requiring the full ArchGraph source repository.

The model should inspect the target repository before writing architecture material. Existing `.archgraph` configuration controls layer roots, candidate files, ignored paths, and marker spellings and must be respected.

Use the other files in this folder as the authoring contract:

- `AUTHORING_RULES.md` defines node, reference, subreference, layer, and source semantics.
- `CONFIGURATION.md` explains the parts of `.archgraph` that affect documentation discovery.
- `TEMPLATE.md` provides canonical Markdown and decorated-source shapes.

The intended output is architecture documentation about the target system. It should not explain how to use ArchGraph unless the target system itself is ArchGraph.
