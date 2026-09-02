use crate::model::{
    ArchitectureEdge, ArchitectureGraph, ArchitectureNode, Diagnostic, DiagnosticCode,
    DiagnosticSeverity, NodeKind, ProjectInfo, SourceLocation, GRAPH_FORMAT_VERSION,
};
use crate::parser::{ParsedArchitectureDocument, parse_architecture_document};
use crate::resolver::{SymbolIndex, architecture_node_id};
use ignore::WalkBuilder;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("project root does not exist: {0}")]
    RootMissing(PathBuf),
    #[error("project root is not a directory: {0}")]
    RootNotDirectory(PathBuf),
    #[error("failed to canonicalize project root {path}: {source}")]
    Canonicalize { path: PathBuf, source: io::Error },
    #[error("failed to read {path}: {source}")]
    ReadFile { path: PathBuf, source: io::Error },
}

#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub architecture_filename: String,
    pub include_hidden: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            architecture_filename: "ARCHITECTURE.md".to_owned(),
            include_hidden: false,
        }
    }
}

pub fn scan_project(root: impl AsRef<Path>) -> Result<ArchitectureGraph, ScanError> {
    scan_project_with_options(root, &ScanOptions::default())
}

pub fn scan_project_with_options(
    root: impl AsRef<Path>,
    options: &ScanOptions,
) -> Result<ArchitectureGraph, ScanError> {
    let supplied_root = root.as_ref();
    if !supplied_root.exists() {
        return Err(ScanError::RootMissing(supplied_root.to_path_buf()));
    }
    if !supplied_root.is_dir() {
        return Err(ScanError::RootNotDirectory(supplied_root.to_path_buf()));
    }

    let root = supplied_root
        .canonicalize()
        .map_err(|source| ScanError::Canonicalize {
            path: supplied_root.to_path_buf(),
            source,
        })?;

    let mut documents = Vec::new();
    let mut symbols = SymbolIndex::default();

    let filter_root = root.clone();
    let mut builder = WalkBuilder::new(&root);
    builder
        .hidden(!options.include_hidden)
        .follow_links(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .add_custom_ignore_filename(".archgraphignore")
        .filter_entry(move |entry| {
            let relative = entry.path().strip_prefix(&filter_root).unwrap_or(entry.path());
            !should_skip(relative)
        });

    for result in builder.build() {
        let Ok(entry) = result else { continue };
        let path = entry.path();
        if path == root || !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }

        let Ok(relative_path) = path.strip_prefix(&root) else {
            continue;
        };
        if should_skip(relative_path) {
            continue;
        }

        if relative_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == options.architecture_filename)
        {
            let contents = read_utf8(path)?;
            documents.push(parse_architecture_document(relative_path, &contents));
        } else if is_typescript_source(relative_path) {
            let contents = read_utf8(path)?;
            symbols.add_source(relative_path, &contents);
        }
    }

    Ok(build_graph(&root, documents, symbols))
}

fn build_graph(
    root: &Path,
    documents: Vec<ParsedArchitectureDocument>,
    symbols: SymbolIndex,
) -> ArchitectureGraph {
    let mut diagnostics = Vec::new();
    let mut architecture_by_name: HashMap<String, Vec<ArchitectureNode>> = HashMap::new();

    for document in &documents {
        diagnostics.extend(document.diagnostics.clone());
        let Some(parsed_node) = &document.node else { continue };

        let node = ArchitectureNode {
            id: architecture_node_id(&document.source_file, &parsed_node.name),
            name: parsed_node.name.clone(),
            kind: NodeKind::Architecture,
            source: Some(SourceLocation {
                file: document.source_file.clone(),
                line: Some(parsed_node.line),
            }),
            summary: None,
        };

        architecture_by_name
            .entry(node.name.clone())
            .or_default()
            .push(node);
    }

    for (name, nodes) in &architecture_by_name {
        if nodes.len() > 1 {
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::DuplicateArchitectureNode,
                severity: DiagnosticSeverity::Warning,
                message: format!("Duplicate ARCH_NODE name: {name}"),
                source: nodes.first().and_then(|node| node.source.clone()),
                candidates: nodes
                    .iter()
                    .filter_map(|node| node.source.as_ref())
                    .map(|source| source.file.display().to_string())
                    .collect(),
            });
        }
    }

    let mut nodes = architecture_by_name
        .values()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    let mut known_node_ids = nodes.iter().map(|node| node.id.clone()).collect::<HashSet<_>>();
    let mut edges = Vec::new();

    for document in &documents {
        let Some(parsed_node) = &document.node else { continue };
        let Some(source_node) = architecture_by_name
            .get(&parsed_node.name)
            .and_then(|candidates| {
                candidates
                    .iter()
                    .find(|candidate| candidate.source.as_ref().is_some_and(|source| source.file == document.source_file))
            })
        else {
            continue;
        };

        for (ordinal, dependency) in document.dependencies.iter().enumerate() {
            let source_location = SourceLocation {
                file: document.source_file.clone(),
                line: Some(dependency.line),
            };
            let resolved = symbols.resolve(&dependency.target, &architecture_by_name, &source_location);

            if known_node_ids.insert(resolved.node.id.clone()) {
                nodes.push(resolved.node.clone());
            }
            if let Some(diagnostic) = resolved.diagnostic {
                diagnostics.push(diagnostic);
            }

            edges.push(ArchitectureEdge {
                id: format!("edge:{}:{}:{ordinal}", source_node.id, resolved.node.id),
                source: source_node.id.clone(),
                target: resolved.node.id,
                target_name: dependency.target.clone(),
                description: dependency.description.clone(),
                source_location,
                resolution: resolved.resolution,
            });
        }
    }

    nodes.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
    edges.sort_by(|a, b| a.source.cmp(&b.source).then(a.target.cmp(&b.target)));
    diagnostics.sort_by(|a, b| a.message.cmp(&b.message));

    ArchitectureGraph {
        version: GRAPH_FORMAT_VERSION,
        project: ProjectInfo {
            name: root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("project")
                .to_owned(),
            root: root.to_path_buf(),
        },
        nodes,
        edges,
        diagnostics,
    }
}

fn read_utf8(path: &Path) -> Result<String, ScanError> {
    fs::read_to_string(path).map_err(|source| ScanError::ReadFile {
        path: path.to_path_buf(),
        source,
    })
}

fn is_typescript_source(path: &Path) -> bool {
    let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    if name.ends_with(".d.ts") {
        return false;
    }

    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("ts" | "tsx" | "mts" | "cts")
    )
}

fn should_skip(path: &Path) -> bool {
    const EXCLUDED: &[&str] = &[".git", "node_modules", "dist", "build", "target", ".expo", ".next"];
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|component| EXCLUDED.contains(&component))
    })
}
