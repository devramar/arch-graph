# ArchGraph Authoring Checklist

## Choose the layer

- Put broad system documentation in the built-in `architecture` layer unless another layer is more appropriate.
- Add implementation/overview/security/etc. layers only when they provide a useful alternate abstraction.
- Keep file globs mutually exclusive; a source claimed by multiple layers produces `ARCH007` rather than being guessed.
- Use `ignored_paths` for fixtures, generated material, archives, or other paths that should not participate in discovery.

## Declare a source

- A matching file is only a candidate; without ArchGraph markers it is ignored.
- Declare exactly one configured node marker per source, canonically `ARCH_NODE:`.
- Markdown may use bare markers.
- Non-Markdown sources must use a consistent punctuation decoration prefix, e.g. `///`, `#`, `--`, or `*`.
- Do not rely on source-language syntax or imports for resolution.

## References

- Use `ARCH_REFERENCE:` for shared names that may converge onto a documented node in the same composition group.
- Use `ARCH_SUBREFERENCE:` for local concepts that must never merge by name.
- Put the relationship explanation immediately beneath the marker using the same decoration prefix in decorated sources.
- Markdown headings such as `## References` are for humans and do not control parsing.

## Layer composition

- Same-name declarations merge only when their layers are placed in the same composition group.
- A merged node may therefore expose several declaration tabs (for example Architecture + Implementation).
- Separate enabled groups remain visible simultaneously and do not merge with each other.
- Subreferences never merge, regardless of group membership.

## Avoid

- annotating every implementation detail mechanically;
- overlapping file globs without a deliberate reason;
- multiple `ARCH_NODE` declarations in one source;
- treating an undocumented normal reference as an error;
- using `ARCH_SUBREFERENCE` as a child-of-previous-reference marker;
- adding language-specific parsing merely to discover symbols automatically.

## Final check

- node names are deliberate and stable;
- layer boundaries represent useful abstraction levels;
- references explain architectural interactions rather than implementation trivia;
- edge prose is concise and useful;
- source globs and ignored paths are intentional;
- custom vocabulary lives in layer marker configuration rather than parser hacks.
