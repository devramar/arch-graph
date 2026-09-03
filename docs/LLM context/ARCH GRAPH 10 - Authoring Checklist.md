# Authoring Checklist

Use this when creating or reviewing an `ARCHITECTURE.md`.

## Before writing

- Read the source in the folder.
- Identify the subsystem's actual responsibility.
- Identify its public entry points.
- Identify the important internal pieces.
- Trace major data/control flow.
- Identify architectural dependencies.
- Check whether the folder is complex enough to justify an architecture document.

## While writing

- Use source terminology.
- Add one `ARCH_NODE:` declaration.
- Keep descriptions concise.
- Document current behavior.
- Explain relationships, not imports.
- Add `ARCH_DEPENDENCY:` only for meaningful architectural edges.
- Put the relationship explanation directly beneath the dependency marker.
- Record important invariants.
- List only relevant implementation files.

## Avoid

- speculative design;
- historical notes;
- exhaustive API reference;
- every import as a dependency;
- duplicate or overlapping architecture nodes;
- invented invariants;
- generic descriptions that add no information.

## Final review

A reader should be able to answer:

1. What does this subsystem do?
2. Why does it exist?
3. How should it be used?
4. How do its important parts interact?
5. What systems does it depend on?
6. Why does each dependency matter?
7. What assumptions must remain true?
8. Which files should I inspect next?
