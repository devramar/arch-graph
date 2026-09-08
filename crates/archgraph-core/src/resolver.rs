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
    layer_id: &str,
) -> Resolution {
    if kind == ReferenceKind::Subreference {
        return Resolution {
            node: reference_node(
                format!("subref:{layer_id}:{source_node_id}:{ordinal}#{target}"),
                target,
                ReferenceScope::Local,
                layer_id,
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
                    format!("reference:{layer_id}:{target}"),
                    target,
                    ReferenceScope::Shared,
                    layer_id,
                ),
                diagnostic: Some(Diagnostic {
                    code: DiagnosticCode::AmbiguousReference,
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "Reference {target:?} matches multiple architecture declarations in layer {layer_id:?}"
                    ),
                    source: Some(source.clone()),
                    layer_id: Some(layer_id.to_owned()),
                    group_id: Some(layer_id.to_owned()),
                    candidates: candidates
                        .iter()
                        .flat_map(|candidate| candidate.declarations.iter())
                        .map(|declaration| declaration.source.file.display().to_string())
                        .collect(),
                }),
            };
        }
    }

    Resolution {
        node: reference_node(
            format!("reference:{layer_id}:{target}"),
            target,
            ReferenceScope::Shared,
            layer_id,
        ),
        diagnostic: None,
    }
}

fn reference_node(
    id: String,
    name: &str,
    scope: ReferenceScope,
    group_id: &str,
) -> ArchitectureNode {
    ArchitectureNode {
        id,
        name: name.to_owned(),
        kind: NodeKind::Reference,
        group_id: group_id.to_owned(),
        reference_scope: Some(scope),
        declarations: Vec::new(),
    }
}

pub(crate) fn architecture_node_id(layer_id: &str, relative_source: &Path, name: &str) -> String {
    format!("arch:{layer_id}:{}#{name}", relative_source.display())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ArchitectureDeclaration, SourceFormat};

    fn source() -> SourceLocation {
        SourceLocation {
            file: "ARCHITECTURE.md".into(),
            line: Some(4),
        }
    }

    fn architecture(id: &str, file: &str) -> ArchitectureNode {
        ArchitectureNode {
            id: id.to_owned(),
            name: "Identity".to_owned(),
            kind: NodeKind::Architecture,
            group_id: "architecture".to_owned(),
            reference_scope: None,
            declarations: vec![ArchitectureDeclaration {
                layer_id: "architecture".to_owned(),
                layer_name: "Architecture".to_owned(),
                source: SourceLocation {
                    file: file.into(),
                    line: Some(3),
                },
                documentation: None,
                source_format: SourceFormat::Markdown,
            }],
        }
    }

    #[test]
    fn undocumented_references_merge_by_explicit_name_within_a_layer() {
        let architectures = HashMap::new();
        let first = resolve_reference(
            "Authentication",
            ReferenceKind::Reference,
            &architectures,
            "arch:a#A",
            0,
            &source(),
            "architecture",
        );
        let second = resolve_reference(
            "Authentication",
            ReferenceKind::Reference,
            &architectures,
            "arch:b#B",
            0,
            &source(),
            "architecture",
        );

        assert_eq!(first.node.id, second.node.id);
        assert_eq!(first.node.reference_scope, Some(ReferenceScope::Shared));
        assert!(first.diagnostic.is_none());
        assert!(second.diagnostic.is_none());
    }

    #[test]
    fn ordinary_reference_resolves_to_a_unique_architecture_declaration() {
        let identity = architecture("arch:identity#Identity", "identity/ARCHITECTURE.md");
        let architectures = HashMap::from([("Identity".to_owned(), vec![identity.clone()])]);

        let resolved = resolve_reference(
            "Identity",
            ReferenceKind::Reference,
            &architectures,
            "arch:checkout#Checkout",
            0,
            &source(),
            "architecture",
        );

        assert_eq!(resolved.node, identity);
        assert!(resolved.diagnostic.is_none());
    }

    #[test]
    fn ambiguous_architecture_names_are_not_guessed() {
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
            &source(),
            "architecture",
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
        let first = resolve_reference(
            "Password Management",
            ReferenceKind::Subreference,
            &architectures,
            "arch:a#A",
            0,
            &source(),
            "architecture",
        );
        let second = resolve_reference(
            "Password Management",
            ReferenceKind::Subreference,
            &architectures,
            "arch:b#B",
            0,
            &source(),
            "architecture",
        );

        assert_ne!(first.node.id, second.node.id);
        assert_eq!(first.node.name, second.node.name);
        assert_eq!(first.node.reference_scope, Some(ReferenceScope::Local));
    }
}
