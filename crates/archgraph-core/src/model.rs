use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const GRAPH_FORMAT_VERSION: u32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureGraph {
    pub version: u32,
    pub project: ProjectInfo,
    #[serde(default)]
    pub groups: Vec<GraphGroup>,
    pub nodes: Vec<ArchitectureNode>,
    pub edges: Vec<ArchitectureEdge>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub name: String,
    pub root: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GraphGroup {
    pub id: String,
    pub name: String,
    pub layer_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureLayer {
    pub id: String,
    pub display_name: String,
    pub graph: ArchitectureGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LayerGroup {
    pub id: String,
    pub name: String,
    pub layer_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureNode {
    pub id: String,
    pub name: String,
    pub kind: NodeKind,
    pub group_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_scope: Option<ReferenceScope>,
    #[serde(default)]
    pub declarations: Vec<ArchitectureDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureDeclaration {
    pub layer_id: String,
    pub layer_name: String,
    pub source: SourceLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    pub source_format: SourceFormat,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum SourceFormat {
    Markdown,
    DecoratedText,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum NodeKind {
    Architecture,
    Reference,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceScope {
    Shared,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArchitectureEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub target_name: String,
    pub group_id: String,
    pub layer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub source_location: SourceLocation,
    pub reference_kind: ReferenceKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceKind {
    Reference,
    Subreference,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticCode {
    #[serde(rename = "ARCH001")]
    DuplicateArchitectureNode,
    #[serde(rename = "ARCH002")]
    AmbiguousReference,
    #[serde(rename = "ARCH004")]
    MissingArchitectureNode,
    #[serde(rename = "ARCH005")]
    MalformedMarker,
    #[serde(rename = "ARCH006")]
    MultipleArchitectureNodes,
    #[serde(rename = "ARCH007")]
    AmbiguousLayerMatch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticSeverity {
    Warning,
    Error,
}
