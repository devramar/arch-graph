use crate::model::{
    ArchitectureNode, Diagnostic, DiagnosticCode, DiagnosticSeverity, NodeKind, ReferenceKind,
    ReferenceScope, SourceLocation,
};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub(crate) struct Resolution {
    pub node: ArchitectureNode,
    pub diagnostic: Option<Diagnostic>,
}

pub(crate) fn resolve_reference(
    target: &str,
    kind: ReferenceKind,
    architectures: &HashMap<String, Vec<ArchitectureNode>>,
    source_node_id: &str,
    ordinal: usize,
    source: &SourceLocation,
) -> Resolution {
    if kind == ReferenceKind::Subreference {
        return Resolution {
            node: reference_node(
                format!("subref:{source_node_id}:{ordinal}#{target}"),
                target,
                ReferenceScope::Local,
            ),
            diagnostic: None,
        };
    }

    if let Some(candidates) = architectures.get(target) {
        if candidates.len() == 1 {
            return Resolution {
                node: candidates[0].clone(),
                diagnostic: None,
            };
        }

        if candidates.len() > 1 {
            return Resolution {
                node: reference_node(
                    format!("reference:{target}"),
                    target,
                    ReferenceScope::Shared,
                ),
                diagnostic: Some(Diagnostic {
                    code: DiagnosticCode::AmbiguousReference,
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "Reference {target:?} matches multiple architecture documents"
                    ),
                    source: Some(source.clone()),
                    candidates: candidates
                        .iter()
                        .filter_map(|candidate| candidate.source.as_ref())
                        .map(|location| location.file.display().to_string())
                        .collect(),
                }),
            };
        }
    }

    Resolution {
        node: reference_node(
            format!("reference:{target}"),
            target,
            ReferenceScope::Shared,
        ),
        diagnostic: None,
    }
}

fn reference_node(id: String, name: &str, scope: ReferenceScope) -> ArchitectureNode {
    ArchitectureNode {
        id,
        name: name.to_owned(),
        kind: NodeKind::Reference,
        reference_scope: Some(scope),
        source: None,
        summary: None,
        documentation: None,
    }
}

pub(crate) fn architecture_node_id(relative_architecture_file: &Path, name: &str) -> String {
    format!("arch:{}#{name}", relative_architecture_file.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undocumented_references_merge_by_explicit_name() {
        let architectures = HashMap::new();
        let source = SourceLocation {
            file: "ARCHITECTURE.md".into(),
            line: Some(4),
        };

        let first = resolve_reference(
            "Authentication",
            ReferenceKind::Reference,
            &architectures,
            "arch:a#A",
            0,
            &source,
        );
        let second = resolve_reference(
            "Authentication",
            ReferenceKind::Reference,
            &architectures,
            "arch:b#B",
            0,
            &source,
        );

        assert_eq!(first.node.id, second.node.id);
        assert_eq!(first.node.reference_scope, Some(ReferenceScope::Shared));
        assert!(first.diagnostic.is_none());
        assert!(second.diagnostic.is_none());
    }

    #[test]
    fn ordinary_reference_resolves_only_to_a_unique_architecture_node() {
        let source = SourceLocation {
            file: "ARCHITECTURE.md".into(),
            line: Some(4),
        };
        let architecture = ArchitectureNode {
            id: "arch:identity/ARCHITECTURE.md#Identity".to_owned(),
            name: "Identity".to_owned(),
            kind: NodeKind::Architecture,
            reference_scope: None,
            source: Some(SourceLocation {
                file: "identity/ARCHITECTURE.md".into(),
                line: Some(3),
            }),
            summary: None,
            documentation: None,
        };
        let architectures = HashMap::from([("Identity".to_owned(), vec![architecture.clone()])]);

        let resolved = resolve_reference(
            "Identity",
            ReferenceKind::Reference,
            &architectures,
            "arch:checkout#Checkout",
            0,
            &source,
        );

        assert_eq!(resolved.node, architecture);
        assert!(resolved.diagnostic.is_none());
    }

    #[test]
    fn ambiguous_architecture_names_are_not_guessed() {
        let source = SourceLocation {
            file: "ARCHITECTURE.md".into(),
            line: Some(4),
        };
        let architecture = |id: &str, file: &str| ArchitectureNode {
            id: id.to_owned(),
            name: "Identity".to_owned(),
            kind: NodeKind::Architecture,
            reference_scope: None,
            source: Some(SourceLocation {
                file: file.into(),
                line: Some(3),
            }),
            summary: None,
            documentation: None,
        };
        let architectures = HashMap::from([(
            "Identity".to_owned(),
            vec![
                architecture("arch:a#Identity", "a/ARCHITECTURE.md"),
                architecture("arch:b#Identity", "b/ARCHITECTURE.md"),
            ],
        )]);

        let resolved = resolve_reference(
            "Identity",
            ReferenceKind::Reference,
            &architectures,
            "arch:checkout#Checkout",
            0,
            &source,
        );

        assert_eq!(resolved.node.kind, NodeKind::Reference);
        assert_eq!(resolved.node.reference_scope, Some(ReferenceScope::Shared));
        assert_eq!(
            resolved
                .diagnostic
                .as_ref()
                .map(|diagnostic| diagnostic.code),
            Some(DiagnosticCode::AmbiguousReference)
        );
    }

    #[test]
    fn subreferences_never_merge() {
        let architectures = HashMap::new();
        let source = SourceLocation {
            file: "ARCHITECTURE.md".into(),
            line: Some(4),
        };

        let first = resolve_reference(
            "Password Management",
            ReferenceKind::Subreference,
            &architectures,
            "arch:a#A",
            0,
            &source,
        );
        let second = resolve_reference(
            "Password Management",
            ReferenceKind::Subreference,
            &architectures,
            "arch:b#B",
            0,
            &source,
        );

        assert_ne!(first.node.id, second.node.id);
        assert_eq!(first.node.name, second.node.name);
        assert_eq!(first.node.reference_scope, Some(ReferenceScope::Local));
    }
}
