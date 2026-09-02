use archgraph_core::{DiagnosticCode, EdgeResolution, NodeKind, scan_project};
use std::path::PathBuf;

#[test]
fn scans_architecture_and_typescript_targets() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/basic");
    let graph = scan_project(fixture).expect("fixture should scan");

    assert_eq!(graph.version, 1);
    assert!(graph.nodes.iter().any(|node| node.name == "EventSync" && node.kind == NodeKind::Architecture));
    assert!(graph.nodes.iter().any(|node| node.name == "DateKey" && node.kind == NodeKind::Module));
    assert!(graph.edges.iter().any(|edge| edge.target_name == "DateKey" && edge.resolution == EdgeResolution::Module));
    assert!(graph.edges.iter().any(|edge| edge.target_name == "RemoteChanges" && edge.resolution == EdgeResolution::Unresolved));
    assert!(graph.diagnostics.iter().any(|diagnostic| diagnostic.code == DiagnosticCode::UnresolvedDependency));
}
