use crate::config::{
    ConfigurationError, PROJECT_CONFIG_FILENAME, ProjectConfiguration,
    load_project_configuration_state_from_root,
};
use crate::model::{
    ArchitectureDeclaration, ArchitectureEdge, ArchitectureGraph, ArchitectureLayer,
    ArchitectureNode, Diagnostic, DiagnosticCode, DiagnosticSeverity, GRAPH_FORMAT_VERSION,
    GraphGroup, LayerGroup, NodeKind, ProjectInfo, ReferenceKind, ReferenceScope, SourceLocation,
};
use crate::parser::{ParsedArchitectureSource, parse_architecture_source};
use crate::resolver::{architecture_node_id, resolve_reference};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
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
    #[error(transparent)]
    Configuration(#[from] ConfigurationError),
}

#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    pub include_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectScan {
    pub graph: ArchitectureGraph,
    pub layers: Vec<ArchitectureLayer>,
    pub diagnostics: Vec<Diagnostic>,
    pub configuration: ProjectConfiguration,
    pub configuration_exists: bool,
}

pub fn scan_project(root: impl AsRef<Path>) -> Result<ArchitectureGraph, ScanError> {
    Ok(scan_project_state(root)?.graph)
}

pub fn scan_project_state(root: impl AsRef<Path>) -> Result<ProjectScan, ScanError> {
    scan_project_state_with_options(root, &ScanOptions::default())
}

pub fn scan_project_with_options(
    root: impl AsRef<Path>,
    options: &ScanOptions,
) -> Result<ArchitectureGraph, ScanError> {
    Ok(scan_project_state_with_options(root, options)?.graph)
}

pub fn scan_project_state_with_options(
    root: impl AsRef<Path>,
    options: &ScanOptions,
) -> Result<ProjectScan, ScanError> {
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
    let (configuration, configuration_exists) = load_project_configuration_state_from_root(&root)?;
    let ignored_paths = PathPatterns::new(&configuration.ignored_paths);
    let mut sources_by_layer: BTreeMap<String, Vec<ParsedArchitectureSource>> = configuration
        .layers
        .keys()
        .map(|layer_id| (layer_id.clone(), Vec::new()))
        .collect();
    let mut diagnostics = Vec::new();

    let filter_root = root.clone();
    let filter_ignored = ignored_paths.clone();
    let mut builder = WalkBuilder::new(&root);
    builder
        .hidden(!options.include_hidden)
        .follow_links(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .add_custom_ignore_filename(".archgraphignore")
        .filter_entry(move |entry| {
            let relative = entry
                .path()
                .strip_prefix(&filter_root)
                .unwrap_or(entry.path());
            !should_skip(relative) && !filter_ignored.matches_path(relative)
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
        if relative_path == Path::new(PROJECT_CONFIG_FILENAME)
            || should_skip(relative_path)
            || ignored_paths.matches_path(relative_path)
        {
            continue;
        }

        let matching_layers = configuration
            .layers
            .iter()
            .filter_map(|(layer_id, layer)| {
                let layer_relative_path = relative_to_layer_root(relative_path, &layer.path_root)?;
                if PathPatterns::new(&layer.ignored_paths).matches_path(layer_relative_path) {
                    return None;
                }
                layer
                    .files
                    .iter()
                    .any(|pattern| glob_matches_file(pattern, layer_relative_path))
                    .then_some((layer_id.as_str(), layer))
            })
            .collect::<Vec<_>>();

        if matching_layers.is_empty() {
            continue;
        }
        if matching_layers.len() > 1 {
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::AmbiguousLayerMatch,
                severity: DiagnosticSeverity::Warning,
                message: format!(
                    "Source {} matches multiple architecture layers and was skipped",
                    relative_path.display()
                ),
                source: Some(SourceLocation {
                    file: relative_path.to_path_buf(),
                    line: None,
                }),
                layer_id: None,
                group_id: None,
                candidates: matching_layers
                    .iter()
                    .map(|(layer_id, _)| (*layer_id).to_owned())
                    .collect(),
            });
            continue;
        }

        let (layer_id, layer) = matching_layers[0];
        let contents = read_utf8(path)?;
        let Some(parsed) =
            parse_architecture_source(relative_path, &contents, &layer.markers, layer_id)
        else {
            continue;
        };
        if let Some(sources) = sources_by_layer.get_mut(layer_id) {
            sources.push(parsed);
        }
    }

    let project = project_info(&root);
    let mut layers = Vec::new();
    for (layer_id, layer_configuration) in &configuration.layers {
        let sources = sources_by_layer.remove(layer_id).unwrap_or_default();
        layers.push(ArchitectureLayer {
            id: layer_id.clone(),
            display_name: layer_configuration.display_name.clone(),
            graph: build_layer_graph(
                &project,
                layer_id,
                &layer_configuration.display_name,
                sources,
            ),
        });
    }

    let all_layers = LayerGroup {
        id: "all".to_owned(),
        name: "All Layers".to_owned(),
        layer_ids: layers.iter().map(|layer| layer.id.clone()).collect(),
    };
    let graph = compose_project_layers(
        project,
        &layers,
        &diagnostics,
        std::slice::from_ref(&all_layers),
    );

    Ok(ProjectScan {
        graph,
        layers,
        diagnostics,
        configuration,
        configuration_exists,
    })
}

pub fn compose_project_layers(
    project: ProjectInfo,
    layers: &[ArchitectureLayer],
    scan_diagnostics: &[Diagnostic],
    groups: &[LayerGroup],
) -> ArchitectureGraph {
    let layer_by_id = layers
        .iter()
        .map(|layer| (layer.id.as_str(), layer))
        .collect::<HashMap<_, _>>();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut diagnostics = scan_diagnostics.to_vec();
    let mut graph_groups = Vec::new();

    for group in groups {
        let selected = group
            .layer_ids
            .iter()
            .filter_map(|layer_id| layer_by_id.get(layer_id.as_str()).copied())
            .collect::<Vec<_>>();
        if selected.is_empty() {
            continue;
        }

        graph_groups.push(GraphGroup {
            id: group.id.clone(),
            name: group.name.clone(),
            layer_ids: selected.iter().map(|layer| layer.id.clone()).collect(),
        });

        compose_group(group, &selected, &mut nodes, &mut edges, &mut diagnostics);
    }

    sort_graph_parts(&mut nodes, &mut edges, &mut diagnostics);

    ArchitectureGraph {
        version: GRAPH_FORMAT_VERSION,
        project,
        groups: graph_groups,
        nodes,
        edges,
        diagnostics,
    }
}

type ArchitectureCandidate<'a> = (&'a ArchitectureNode, &'a str);

fn compose_group(
    group: &LayerGroup,
    layers: &[&ArchitectureLayer],
    nodes: &mut Vec<ArchitectureNode>,
    edges: &mut Vec<ArchitectureEdge>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut architecture_by_name: HashMap<String, Vec<ArchitectureCandidate<'_>>> = HashMap::new();
    for layer in layers {
        for node in layer
            .graph
            .nodes
            .iter()
            .filter(|node| node.kind == NodeKind::Architecture)
        {
            architecture_by_name
                .entry(node.name.clone())
                .or_default()
                .push((node, layer.id.as_str()));
        }

        diagnostics.extend(
            layer
                .graph
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.code != DiagnosticCode::AmbiguousReference)
                .cloned()
                .map(|mut diagnostic| {
                    diagnostic.group_id = Some(group.id.clone());
                    diagnostic
                }),
        );
    }

    let mut source_id_map = HashMap::new();
    let mut unique_architecture_targets = HashMap::new();
    let mut ambiguous_architecture_targets: HashMap<String, Vec<String>> = HashMap::new();

    for (name, candidates) in &architecture_by_name {
        let mut counts_by_layer: HashMap<&str, usize> = HashMap::new();
        for (_, layer_id) in candidates {
            *counts_by_layer.entry(*layer_id).or_default() += 1;
        }
        let ambiguous = counts_by_layer.values().any(|count| *count > 1);

        if ambiguous {
            let candidate_files = candidates
                .iter()
                .flat_map(|(candidate, _)| candidate.declarations.iter())
                .map(|declaration| declaration.source.file.display().to_string())
                .collect::<Vec<_>>();
            ambiguous_architecture_targets.insert(name.clone(), candidate_files);

            for (candidate, layer_id) in candidates {
                let mut node = (**candidate).clone();
                node.id = format!("group:{}:layer:{layer_id}:{}", group.id, candidate.id);
                node.group_id = group.id.clone();
                source_id_map.insert(
                    ((*layer_id).to_owned(), candidate.id.clone()),
                    node.id.clone(),
                );
                nodes.push(node);
            }
            continue;
        }

        let id = format!("group:{}:architecture:{name}", group.id);
        let mut declarations = candidates
            .iter()
            .flat_map(|(candidate, _)| candidate.declarations.iter().cloned())
            .collect::<Vec<_>>();
        declarations.sort_by(|a, b| {
            a.layer_id
                .cmp(&b.layer_id)
                .then(a.source.file.cmp(&b.source.file))
                .then(a.source.line.cmp(&b.source.line))
        });

        nodes.push(ArchitectureNode {
            id: id.clone(),
            name: name.clone(),
            kind: NodeKind::Architecture,
            group_id: group.id.clone(),
            reference_scope: None,
            declarations,
        });
        unique_architecture_targets.insert(name.clone(), id.clone());
        for (candidate, layer_id) in candidates {
            source_id_map.insert(((*layer_id).to_owned(), candidate.id.clone()), id.clone());
        }
    }

    let mut known_reference_nodes = HashSet::new();
    for layer in layers {
        for edge in &layer.graph.edges {
            let Some(source) = source_id_map
                .get(&(layer.id.clone(), edge.source.clone()))
                .cloned()
            else {
                continue;
            };

            let target = if edge.reference_kind == ReferenceKind::Subreference {
                let id = format!("group:{}:subref:{}:{}", group.id, layer.id, edge.id);
                if known_reference_nodes.insert(id.clone()) {
                    nodes.push(reference_node(
                        id.clone(),
                        &edge.target_name,
                        ReferenceScope::Local,
                        &group.id,
                    ));
                }
                id
            } else if let Some(target) = unique_architecture_targets.get(&edge.target_name) {
                target.clone()
            } else {
                let id = format!("group:{}:reference:{}", group.id, edge.target_name);
                if known_reference_nodes.insert(id.clone()) {
                    nodes.push(reference_node(
                        id.clone(),
                        &edge.target_name,
                        ReferenceScope::Shared,
                        &group.id,
                    ));
                }

                if let Some(candidates) = ambiguous_architecture_targets.get(&edge.target_name) {
                    diagnostics.push(Diagnostic {
                        code: DiagnosticCode::AmbiguousReference,
                        severity: DiagnosticSeverity::Warning,
                        message: format!(
                            "Reference {:?} matches multiple architecture declarations in group {:?}",
                            edge.target_name, group.name
                        ),
                        source: Some(edge.source_location.clone()),
                        layer_id: Some(layer.id.clone()),
                        group_id: Some(group.id.clone()),
                        candidates: candidates.clone(),
                    });
                }
                id
            };

            edges.push(ArchitectureEdge {
                id: format!("group:{}:{}:{}", group.id, layer.id, edge.id),
                source,
                target,
                target_name: edge.target_name.clone(),
                group_id: group.id.clone(),
                layer_id: layer.id.clone(),
                description: edge.description.clone(),
                source_location: edge.source_location.clone(),
                reference_kind: edge.reference_kind,
            });
        }
    }
}

fn build_layer_graph(
    project: &ProjectInfo,
    layer_id: &str,
    layer_name: &str,
    sources: Vec<ParsedArchitectureSource>,
) -> ArchitectureGraph {
    let mut diagnostics = Vec::new();
    let mut architecture_by_name: HashMap<String, Vec<ArchitectureNode>> = HashMap::new();

    for source in &sources {
        diagnostics.extend(source.diagnostics.clone());
        let Some(parsed_node) = &source.node else {
            continue;
        };

        let node = ArchitectureNode {
            id: architecture_node_id(layer_id, &source.source_file, &parsed_node.name),
            name: parsed_node.name.clone(),
            kind: NodeKind::Architecture,
            group_id: layer_id.to_owned(),
            reference_scope: None,
            declarations: vec![ArchitectureDeclaration {
                layer_id: layer_id.to_owned(),
                layer_name: layer_name.to_owned(),
                source: SourceLocation {
                    file: source.source_file.clone(),
                    line: Some(parsed_node.line),
                },
                documentation: source.documentation.clone(),
                source_format: source.source_format,
            }],
        };

        architecture_by_name
            .entry(node.name.clone())
            .or_default()
            .push(node);
    }

    for (name, candidates) in &architecture_by_name {
        if candidates.len() > 1 {
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::DuplicateArchitectureNode,
                severity: DiagnosticSeverity::Warning,
                message: format!("Duplicate architecture node name {name:?} in layer {layer_id:?}"),
                source: candidates
                    .first()
                    .and_then(|node| node.declarations.first())
                    .map(|declaration| declaration.source.clone()),
                layer_id: Some(layer_id.to_owned()),
                group_id: Some(layer_id.to_owned()),
                candidates: candidates
                    .iter()
                    .flat_map(|node| node.declarations.iter())
                    .map(|declaration| declaration.source.file.display().to_string())
                    .collect(),
            });
        }
    }

    let mut nodes = architecture_by_name
        .values()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    let mut known_node_ids = nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<HashSet<_>>();
    let mut edges = Vec::new();

    for source in &sources {
        let Some(parsed_node) = &source.node else {
            continue;
        };
        let Some(source_node) =
            architecture_by_name
                .get(&parsed_node.name)
                .and_then(|candidates| {
                    candidates.iter().find(|candidate| {
                        candidate.declarations.first().is_some_and(|declaration| {
                            declaration.source.file == source.source_file
                        })
                    })
                })
        else {
            continue;
        };

        for (ordinal, reference) in source.references.iter().enumerate() {
            let source_location = SourceLocation {
                file: source.source_file.clone(),
                line: Some(reference.line),
            };
            let resolved = resolve_reference(
                &reference.target,
                reference.kind,
                &architecture_by_name,
                &source_node.id,
                ordinal,
                &source_location,
                layer_id,
            );

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
                target_name: reference.target.clone(),
                group_id: layer_id.to_owned(),
                layer_id: layer_id.to_owned(),
                description: reference.description.clone(),
                source_location,
                reference_kind: reference.kind,
            });
        }
    }

    sort_graph_parts(&mut nodes, &mut edges, &mut diagnostics);

    ArchitectureGraph {
        version: GRAPH_FORMAT_VERSION,
        project: project.clone(),
        groups: vec![GraphGroup {
            id: layer_id.to_owned(),
            name: layer_name.to_owned(),
            layer_ids: vec![layer_id.to_owned()],
        }],
        nodes,
        edges,
        diagnostics,
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

fn sort_graph_parts(
    nodes: &mut [ArchitectureNode],
    edges: &mut [ArchitectureEdge],
    diagnostics: &mut [Diagnostic],
) {
    nodes.sort_by(|a, b| {
        a.group_id
            .cmp(&b.group_id)
            .then(a.name.cmp(&b.name))
            .then(
                reference_scope_order(a.reference_scope)
                    .cmp(&reference_scope_order(b.reference_scope)),
            )
            .then(a.id.cmp(&b.id))
    });
    edges.sort_by(|a, b| {
        a.group_id
            .cmp(&b.group_id)
            .then(a.source.cmp(&b.source))
            .then(a.target.cmp(&b.target))
            .then(a.id.cmp(&b.id))
    });
    diagnostics.sort_by(|a, b| {
        a.group_id
            .cmp(&b.group_id)
            .then(a.layer_id.cmp(&b.layer_id))
            .then(a.message.cmp(&b.message))
    });
}

fn reference_scope_order(scope: Option<ReferenceScope>) -> u8 {
    match scope {
        None => 0,
        Some(ReferenceScope::Shared) => 1,
        Some(ReferenceScope::Local) => 2,
    }
}

fn project_info(root: &Path) -> ProjectInfo {
    ProjectInfo {
        name: root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("project")
            .to_owned(),
        root: root.to_path_buf(),
    }
}

fn read_utf8(path: &Path) -> Result<String, ScanError> {
    fs::read_to_string(path).map_err(|source| ScanError::ReadFile {
        path: path.to_path_buf(),
        source,
    })
}

fn should_skip(path: &Path) -> bool {
    const EXCLUDED: &[&str] = &[
        ".git",
        "node_modules",
        "dist",
        "build",
        "target",
        ".expo",
        ".next",
    ];
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|component| EXCLUDED.contains(&component))
    })
}

fn relative_to_layer_root<'a>(path: &'a Path, path_root: &str) -> Option<&'a Path> {
    if path_root == "." {
        Some(path)
    } else {
        path.strip_prefix(Path::new(path_root)).ok()
    }
}

#[derive(Clone)]
struct PathPatterns {
    patterns: Vec<String>,
}

impl PathPatterns {
    fn new(patterns: &[String]) -> Self {
        Self {
            patterns: patterns.to_vec(),
        }
    }

    fn matches_path(&self, path: &Path) -> bool {
        self.patterns
            .iter()
            .any(|pattern| glob_matches_path(pattern, path))
    }
}

fn glob_matches_file(pattern: &str, path: &Path) -> bool {
    let normalized = normalize_pattern(pattern);
    if normalized.contains('/') {
        return glob_path_segments_match(&split_pattern(&normalized), &path_segments(path));
    }

    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| wildcard_segment_match(&normalized, name))
}

fn glob_matches_path(pattern: &str, path: &Path) -> bool {
    let normalized = normalize_pattern(pattern);
    let path_segments = path_segments(path);
    if normalized.contains('/') {
        return glob_path_segments_match(&split_pattern(&normalized), &path_segments);
    }

    path_segments
        .iter()
        .any(|segment| wildcard_segment_match(&normalized, segment))
}

fn normalize_pattern(pattern: &str) -> String {
    pattern
        .trim()
        .trim_start_matches("./")
        .trim_matches('/')
        .replace('\\', "/")
}

fn split_pattern(pattern: &str) -> Vec<&str> {
    pattern
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn path_segments(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| component.as_os_str().to_str().map(str::to_owned))
        .collect()
}

fn glob_path_segments_match(pattern: &[&str], path: &[String]) -> bool {
    fn matches(pattern: &[&str], path: &[String], pattern_index: usize, path_index: usize) -> bool {
        if pattern_index == pattern.len() {
            return path_index == path.len();
        }
        if pattern[pattern_index] == "**" {
            return matches(pattern, path, pattern_index + 1, path_index)
                || (path_index < path.len()
                    && matches(pattern, path, pattern_index, path_index + 1));
        }
        path_index < path.len()
            && wildcard_segment_match(pattern[pattern_index], &path[path_index])
            && matches(pattern, path, pattern_index + 1, path_index + 1)
    }

    matches(pattern, path, 0, 0)
}

fn wildcard_segment_match(pattern: &str, text: &str) -> bool {
    let pattern = pattern.chars().collect::<Vec<_>>();
    let text = text.chars().collect::<Vec<_>>();
    let mut memo = HashMap::new();

    fn matches(
        pattern: &[char],
        text: &[char],
        pattern_index: usize,
        text_index: usize,
        memo: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(result) = memo.get(&(pattern_index, text_index)) {
            return *result;
        }
        let result = if pattern_index == pattern.len() {
            text_index == text.len()
        } else if pattern[pattern_index] == '*' {
            matches(pattern, text, pattern_index + 1, text_index, memo)
                || (text_index < text.len()
                    && matches(pattern, text, pattern_index, text_index + 1, memo))
        } else {
            text_index < text.len()
                && (pattern[pattern_index] == '?' || pattern[pattern_index] == text[text_index])
                && matches(pattern, text, pattern_index + 1, text_index + 1, memo)
        };
        memo.insert((pattern_index, text_index), result);
        result
    }

    matches(&pattern, &text, 0, 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_globs_match_basenames_and_recursive_paths() {
        assert!(glob_matches_file("*.ts", Path::new("src/core/datekey.ts")));
        assert!(glob_matches_file(
            "src/**/*.ts",
            Path::new("src/core/datekey.ts")
        ));
        assert!(!glob_matches_file(
            "src/**/*.rs",
            Path::new("src/core/datekey.ts")
        ));
    }

    #[test]
    fn ignored_path_globs_match_directories_and_descendants() {
        assert!(glob_matches_path(
            "crates/archgraph-core/tests/fixtures/**",
            Path::new("crates/archgraph-core/tests/fixtures/basic/ARCHITECTURE.md")
        ));
        assert!(glob_matches_path(
            "fixtures",
            Path::new("tests/fixtures/basic/ARCHITECTURE.md")
        ));
    }

    #[test]
    fn layer_roots_change_the_path_coordinate_space() {
        let source = Path::new("apps/desktop/src/App.tsx");
        let relative = relative_to_layer_root(source, "apps/desktop").expect("inside layer root");
        assert_eq!(relative, Path::new("src/App.tsx"));
        assert!(glob_matches_file("src/**/*.tsx", relative));
        assert!(relative_to_layer_root(source, "crates").is_none());
    }
}
