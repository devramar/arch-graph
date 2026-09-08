# ArchGraph Authoring Checklist

## Before adding a document

- Add architecture documentation only when a folder represents a meaningful system/concept.
- Choose one stable explicit architecture-node name.
- Identify relationships that are architecturally useful rather than mechanically exhaustive.

## Markers

- Declare one configured node marker, canonically `ARCH_NODE:`.
- Use `ARCH_REFERENCE:` for shared references that should converge by explicit name.
- Use `ARCH_SUBREFERENCE:` for local concepts that must never merge by name.
- Put the relationship explanation immediately beneath the reference marker.
- Do not depend on a particular Markdown heading for parsing.

## Reference choice

Use a normal reference when:

- another architecture document already describes the target; or
- multiple systems should visibly converge on one shared named concept.

Use a subreference when:

- merging the name globally would create a misleading hub;
- the concept is explanatory context local to one architecture node;
- same-name occurrences should look related but remain disconnected from each other.

## Avoid

- declaring every import/reference in source code;
- relying on TypeScript/Rust/Python symbol names for resolution;
- treating missing architecture documentation as an error;
- using `ARCH_SUBREFERENCE` as though it means a child of the preceding reference.

## Final check

- every architecture name is deliberate and stable;
- every reference name is explicit and meaningful;
- every edge description explains the interaction;
- shared vs local scope is intentional;
- custom project vocabulary is represented in root `.archgraph` rather than parser-specific hacks.
