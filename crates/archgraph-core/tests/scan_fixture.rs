use archgraph_core::{
    LayerGroup, NodeKind, ReferenceKind, ReferenceScope, SourceFormat, compose_project_layers,
    scan_project, scan_project_state,
};
use std::path::PathBuf;

#[test]
fn scans_explicit_references_without_source_language_resolution() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/basic");
    let graph = scan_project(fixture).expect("fixture should scan");

    assert_eq!(graph.version, 3);
    let event_sync = graph
        .nodes
        .iter()
        .find(|node| node.name == "EventSync")
        .expect("EventSync architecture node");
    assert_eq!(event_sync.kind, NodeKind::Architecture);
    assert!(
        event_sync
            .declarations
            .first()
            .and_then(|declaration| declaration.documentation.as_deref())
            .is_some_and(|documentation| documentation.contains("ARCH_REFERENCE:DateKey"))
    );

    let date_key = graph
        .nodes
        .iter()
        .find(|node| node.name == "DateKey")
        .expect("DateKey should be represented as a reference");
    assert_eq!(date_key.kind, NodeKind::Reference);
    assert_eq!(date_key.reference_scope, Some(ReferenceScope::Shared));
    assert!(
        date_key.declarations.is_empty(),
        "TypeScript source must not be indexed without a matching layer glob"
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
fn scans_layers_globs_decorated_sources_and_ignored_paths() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/aliases");
    let scan = scan_project_state(fixture).expect("layered fixture should scan");

    assert!(scan.configuration_exists);
    assert_eq!(scan.layers.len(), 2);
    assert_eq!(scan.configuration.layers["architecture"].path_root, ".");
    assert_eq!(
        scan.configuration.layers["architecture"].ignored_paths,
        vec!["ignored/**".to_owned()]
    );
    assert_eq!(
        scan.configuration.layers["implementation"].path_root,
        "implementation"
    );
    assert!(
        !scan
            .graph
            .nodes
            .iter()
            .any(|node| node.name == "ShouldNotAppear")
    );
    assert!(
        !scan
            .graph
            .nodes
            .iter()
            .any(|node| node.name == "ordinaryCode")
    );
    assert!(
        !scan
            .graph
            .nodes
            .iter()
            .any(|node| node.name == "Outside Implementation Root")
    );

    let identity = scan
        .graph
        .nodes
        .iter()
        .find(|node| node.name == "Identity")
        .expect("Identity should merge across layers");
    assert_eq!(identity.kind, NodeKind::Architecture);
    assert_eq!(identity.declarations.len(), 2);
    assert!(identity.declarations.iter().any(|declaration| {
        declaration.layer_id == "implementation"
            && declaration.source_format == SourceFormat::DecoratedText
            && declaration
                .documentation
                .as_deref()
                .is_some_and(|documentation| {
                    documentation.contains("Concrete identity implementation")
                })
    }));

    assert!(scan.graph.nodes.iter().any(|node| {
        node.name == "Password Management" && node.reference_scope == Some(ReferenceScope::Local)
    }));
    assert!(
        scan.graph
            .nodes
            .iter()
            .any(|node| node.name == "Token Store")
    );

    let implementation_detail = scan
        .graph
        .nodes
        .iter()
        .find(|node| node.name == "Implementation Detail")
        .expect("implementation-only architecture declaration");
    assert_eq!(implementation_detail.kind, NodeKind::Architecture);
    assert!(scan.graph.edges.iter().any(|edge| {
        edge.target_name == "Implementation Detail" && edge.target == implementation_detail.id
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
}

#[test]
fn composition_only_merges_matching_nodes_inside_the_same_group() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/aliases");
    let scan = scan_project_state(fixture).expect("layered fixture should scan");

    let separate = compose_project_layers(
        scan.graph.project.clone(),
        &scan.layers,
        &scan.diagnostics,
        &[
            LayerGroup {
                id: "architecture".to_owned(),
                name: "Architecture".to_owned(),
                layer_ids: vec!["architecture".to_owned()],
            },
            LayerGroup {
                id: "implementation".to_owned(),
                name: "Implementation".to_owned(),
                layer_ids: vec!["implementation".to_owned()],
            },
        ],
    );

    let identities = separate
        .nodes
        .iter()
        .filter(|node| node.name == "Identity" && node.kind == NodeKind::Architecture)
        .collect::<Vec<_>>();
    assert_eq!(identities.len(), 2);
    assert_ne!(identities[0].group_id, identities[1].group_id);

    let architecture_detail_target = separate
        .edges
        .iter()
        .find(|edge| edge.group_id == "architecture" && edge.target_name == "Implementation Detail")
        .expect("architecture reference to implementation-only name");
    let target_node = separate
        .nodes
        .iter()
        .find(|node| node.id == architecture_detail_target.target)
        .expect("reference target node");
    assert_eq!(target_node.kind, NodeKind::Reference);
    assert_eq!(target_node.reference_scope, Some(ReferenceScope::Shared));
}
