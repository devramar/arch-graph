use archgraph_core::{NodeKind, ReferenceKind, ReferenceScope, scan_project, scan_project_state};
use std::path::PathBuf;

#[test]
fn scans_explicit_references_without_source_language_resolution() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/basic");
    let graph = scan_project(fixture).expect("fixture should scan");

    assert_eq!(graph.version, 2);
    assert!(
        graph
            .nodes
            .iter()
            .any(|node| { node.name == "EventSync" && node.kind == NodeKind::Architecture })
    );
    assert!(
        graph
            .nodes
            .iter()
            .find(|node| node.name == "EventSync")
            .and_then(|node| node.documentation.as_deref())
            .is_some_and(|documentation| documentation.contains("ARCH_REFERENCE:DateKey"))
    );

    let date_key = graph
        .nodes
        .iter()
        .find(|node| node.name == "DateKey")
        .expect("DateKey should be represented as a reference");
    assert_eq!(date_key.kind, NodeKind::Reference);
    assert_eq!(date_key.reference_scope, Some(ReferenceScope::Shared));
    assert_eq!(
        date_key.source, None,
        "TypeScript source must not be indexed"
    );

    let remote_changes = graph
        .nodes
        .iter()
        .find(|node| node.name == "RemoteChanges")
        .expect("documented target should exist");
    assert_eq!(remote_changes.kind, NodeKind::Architecture);

    let password_management = graph
        .nodes
        .iter()
        .find(|node| node.name == "Password Management")
        .expect("subreference should exist");
    assert_eq!(password_management.kind, NodeKind::Reference);
    assert_eq!(
        password_management.reference_scope,
        Some(ReferenceScope::Local)
    );

    assert!(graph.edges.iter().any(|edge| {
        edge.target_name == "RemoteChanges" && edge.reference_kind == ReferenceKind::Reference
    }));
    assert!(graph.edges.iter().any(|edge| {
        edge.target_name == "Password Management"
            && edge.reference_kind == ReferenceKind::Subreference
    }));
    assert!(graph.diagnostics.is_empty());
}

#[test]
fn applies_root_archgraph_aliases_and_exposes_app_configuration() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/aliases");
    let scan = scan_project_state(fixture).expect("aliased fixture should scan");

    assert!(scan.graph.nodes.iter().any(|node| node.name == "Checkout"));
    assert!(scan.graph.nodes.iter().any(|node| node.name == "Identity"));
    assert!(scan.graph.nodes.iter().any(|node| {
        node.name == "Password Management" && node.reference_scope == Some(ReferenceScope::Local)
    }));
    assert_eq!(scan.configuration.default_view.as_deref(), Some("sticky"));
    assert_eq!(
        scan.configuration
            .app_colours
            .colour_overrides
            .references
            .get("Identity")
            .map(String::as_str),
        Some("purple")
    );
    assert!(scan.configuration.view_settings.contains_key("sticky"));
}
