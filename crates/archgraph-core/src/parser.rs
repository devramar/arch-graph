use crate::config::MarkerConfiguration;
use crate::model::{
    Diagnostic, DiagnosticCode, DiagnosticSeverity, ReferenceKind, SourceFormat, SourceLocation,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedArchitectureSource {
    pub source_file: PathBuf,
    pub node: Option<ParsedNode>,
    pub references: Vec<ParsedReference>,
    pub diagnostics: Vec<Diagnostic>,
    pub documentation: Option<String>,
    pub source_format: SourceFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedNode {
    pub name: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedReference {
    pub target: String,
    pub description: Option<String>,
    pub line: usize,
    pub kind: ReferenceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceParserKind {
    Markdown,
    DecoratedText,
}

impl SourceParserKind {
    fn for_path(path: &Path) -> Self {
        match path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase())
            .as_deref()
        {
            Some("md" | "markdown") => Self::Markdown,
            _ => Self::DecoratedText,
        }
    }

    fn parse(
        self,
        relative_path: &Path,
        contents: &str,
        markers: &MarkerConfiguration,
        layer_id: &str,
    ) -> Option<ParsedArchitectureSource> {
        match self {
            Self::Markdown => parse_markdown(relative_path, contents, markers, layer_id),
            Self::DecoratedText => parse_decorated_text(relative_path, contents, markers, layer_id),
        }
    }
}

pub(crate) fn parse_architecture_source(
    relative_path: &Path,
    contents: &str,
    markers: &MarkerConfiguration,
    layer_id: &str,
) -> Option<ParsedArchitectureSource> {
    SourceParserKind::for_path(relative_path).parse(relative_path, contents, markers, layer_id)
}

fn parse_markdown(
    relative_path: &Path,
    contents: &str,
    markers: &MarkerConfiguration,
    layer_id: &str,
) -> Option<ParsedArchitectureSource> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut node = None;
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();
    let mut saw_marker = false;

    for (index, raw_line) in lines.iter().enumerate() {
        let line = raw_line.trim();
        let line_number = index + 1;

        if let Some((marker, rest)) = marker_value(line, &markers.node) {
            saw_marker = true;
            let name = rest.trim();
            if name.is_empty() {
                diagnostics.push(malformed_marker(
                    relative_path,
                    line_number,
                    marker,
                    layer_id,
                ));
            } else if node.is_none() {
                node = Some(ParsedNode {
                    name: name.to_owned(),
                    line: line_number,
                });
            } else {
                diagnostics.push(multiple_node_marker(relative_path, line_number, layer_id));
            }
            continue;
        }

        let reference = reference_marker(line, markers);
        let Some((kind, marker, rest)) = reference else {
            continue;
        };
        saw_marker = true;
        let target = rest.trim();
        if target.is_empty() {
            diagnostics.push(malformed_marker(
                relative_path,
                line_number,
                marker,
                layer_id,
            ));
            continue;
        }

        references.push(ParsedReference {
            target: target.to_owned(),
            description: markdown_reference_description(&lines, index + 1, markers),
            line: line_number,
            kind,
        });
    }

    if !saw_marker {
        return None;
    }

    if node.is_none() {
        diagnostics.push(missing_node_marker(relative_path, layer_id, markers));
    }

    Some(ParsedArchitectureSource {
        source_file: relative_path.to_path_buf(),
        node,
        references,
        diagnostics,
        documentation: Some(contents.to_owned()),
        source_format: SourceFormat::Markdown,
    })
}

fn parse_decorated_text(
    relative_path: &Path,
    contents: &str,
    markers: &MarkerConfiguration,
    layer_id: &str,
) -> Option<ParsedArchitectureSource> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut node = None;
    let mut node_documentation = None;
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();
    let mut saw_marker = false;

    for (index, raw_line) in lines.iter().enumerate() {
        let Some(found) = decorated_marker(raw_line, markers) else {
            continue;
        };
        saw_marker = true;
        let line_number = index + 1;
        let target = found.rest.trim();

        match found.kind {
            MarkerKind::Node => {
                if target.is_empty() {
                    diagnostics.push(malformed_marker(
                        relative_path,
                        line_number,
                        found.marker,
                        layer_id,
                    ));
                } else if node.is_none() {
                    node = Some(ParsedNode {
                        name: target.to_owned(),
                        line: line_number,
                    });
                    node_documentation =
                        decorated_description(&lines, index + 1, &found.decoration, markers);
                } else {
                    diagnostics.push(multiple_node_marker(relative_path, line_number, layer_id));
                }
            }
            MarkerKind::Reference(kind) => {
                if target.is_empty() {
                    diagnostics.push(malformed_marker(
                        relative_path,
                        line_number,
                        found.marker,
                        layer_id,
                    ));
                    continue;
                }
                references.push(ParsedReference {
                    target: target.to_owned(),
                    description: decorated_description(
                        &lines,
                        index + 1,
                        &found.decoration,
                        markers,
                    ),
                    line: line_number,
                    kind,
                });
            }
        }
    }

    if !saw_marker {
        return None;
    }

    if node.is_none() {
        diagnostics.push(missing_node_marker(relative_path, layer_id, markers));
    }

    Some(ParsedArchitectureSource {
        source_file: relative_path.to_path_buf(),
        node,
        references,
        diagnostics,
        documentation: node_documentation,
        source_format: SourceFormat::DecoratedText,
    })
}

#[derive(Debug, Clone, Copy)]
enum MarkerKind {
    Node,
    Reference(ReferenceKind),
}

struct DecoratedMarker<'a> {
    kind: MarkerKind,
    marker: &'a str,
    rest: &'a str,
    decoration: String,
}

struct DecoratedCandidate<'a> {
    index: usize,
    kind: MarkerKind,
    marker: &'a str,
    rest: &'a str,
    decoration: String,
}

fn decorated_marker<'a>(
    raw_line: &'a str,
    markers: &MarkerConfiguration,
) -> Option<DecoratedMarker<'a>> {
    let line = raw_line.trim_start();
    let mut candidates = Vec::new();

    collect_decorated_candidates(line, &markers.node, MarkerKind::Node, &mut candidates);
    collect_decorated_candidates(
        line,
        &markers.reference,
        MarkerKind::Reference(ReferenceKind::Reference),
        &mut candidates,
    );
    collect_decorated_candidates(
        line,
        &markers.subreference,
        MarkerKind::Reference(ReferenceKind::Subreference),
        &mut candidates,
    );

    candidates
        .into_iter()
        .min_by_key(|candidate| candidate.index)
        .map(|candidate| DecoratedMarker {
            kind: candidate.kind,
            marker: candidate.marker,
            rest: candidate.rest,
            decoration: candidate.decoration,
        })
}

fn collect_decorated_candidates<'a>(
    line: &'a str,
    aliases: &[String],
    kind: MarkerKind,
    candidates: &mut Vec<DecoratedCandidate<'a>>,
) {
    for alias in aliases {
        let needle = format!("{alias}:");
        let Some(index) = line.find(&needle) else {
            continue;
        };
        let prefix = line[..index].trim();
        if !valid_decoration(prefix) {
            continue;
        }
        let marker = &line[index..index + alias.len()];
        let rest = &line[index + needle.len()..];
        candidates.push(DecoratedCandidate {
            index,
            kind,
            marker,
            rest,
            decoration: prefix.to_owned(),
        });
    }
}

fn valid_decoration(prefix: &str) -> bool {
    !prefix.is_empty()
        && prefix.chars().all(|character| {
            !character.is_alphanumeric()
                && !character.is_whitespace()
                && !matches!(character, '"' | '\'' | '`')
        })
}

fn strip_decoration<'a>(raw_line: &'a str, decoration: &str) -> Option<&'a str> {
    let line = raw_line.trim_start();
    let remainder = line.strip_prefix(decoration)?;
    if decoration.ends_with('*') && remainder.trim_start().starts_with('/') {
        return None;
    }
    Some(remainder.strip_prefix(' ').unwrap_or(remainder).trim_end())
}

fn decorated_description(
    lines: &[&str],
    start_index: usize,
    decoration: &str,
    markers: &MarkerConfiguration,
) -> Option<String> {
    let mut collected = Vec::new();
    let mut started = false;

    for raw_line in lines.iter().skip(start_index) {
        let Some(content) = strip_decoration(raw_line, decoration) else {
            break;
        };
        if is_archgraph_marker(content.trim(), markers) {
            break;
        }

        let content = content.trim();
        if content.is_empty() {
            if started {
                collected.push(String::new());
            }
            continue;
        }

        started = true;
        collected.push(content.to_owned());
    }

    finish_description(collected)
}

fn markdown_reference_description(
    lines: &[&str],
    start_index: usize,
    markers: &MarkerConfiguration,
) -> Option<String> {
    let mut collected = Vec::new();
    let mut started = false;

    for raw in lines.iter().skip(start_index) {
        let trimmed = raw.trim();

        if is_archgraph_marker(trimmed, markers) || trimmed.starts_with("## ") || trimmed == "---" {
            break;
        }

        if trimmed.is_empty() {
            if started {
                collected.push(String::new());
            }
            continue;
        }

        started = true;
        collected.push(trimmed.to_owned());
    }

    finish_description(collected)
}

fn finish_description(mut collected: Vec<String>) -> Option<String> {
    while matches!(collected.last(), Some(last) if last.is_empty()) {
        collected.pop();
    }

    if collected.is_empty() {
        None
    } else {
        Some(collected.join("\n"))
    }
}

fn reference_marker<'a>(
    line: &'a str,
    markers: &MarkerConfiguration,
) -> Option<(ReferenceKind, &'a str, &'a str)> {
    marker_value(line, &markers.reference)
        .map(|(marker, rest)| (ReferenceKind::Reference, marker, rest))
        .or_else(|| {
            marker_value(line, &markers.subreference)
                .map(|(marker, rest)| (ReferenceKind::Subreference, marker, rest))
        })
}

fn marker_value<'a>(line: &'a str, aliases: &[String]) -> Option<(&'a str, &'a str)> {
    aliases.iter().find_map(|alias| {
        let remainder = line.strip_prefix(alias)?;
        let rest = remainder.strip_prefix(':')?;
        Some((&line[..alias.len()], rest))
    })
}

fn is_archgraph_marker(line: &str, markers: &MarkerConfiguration) -> bool {
    marker_value(line, &markers.node).is_some()
        || marker_value(line, &markers.reference).is_some()
        || marker_value(line, &markers.subreference).is_some()
}

fn malformed_marker(path: &Path, line: usize, marker: &str, layer_id: &str) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::MalformedMarker,
        severity: DiagnosticSeverity::Warning,
        message: format!("{marker} marker has no target name"),
        source: Some(SourceLocation {
            file: path.to_path_buf(),
            line: Some(line),
        }),
        layer_id: Some(layer_id.to_owned()),
        group_id: None,
        candidates: Vec::new(),
    }
}

fn multiple_node_marker(path: &Path, line: usize, layer_id: &str) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::MultipleArchitectureNodes,
        severity: DiagnosticSeverity::Warning,
        message: "A source may declare only one architecture node".to_owned(),
        source: Some(SourceLocation {
            file: path.to_path_buf(),
            line: Some(line),
        }),
        layer_id: Some(layer_id.to_owned()),
        group_id: None,
        candidates: Vec::new(),
    }
}

fn missing_node_marker(path: &Path, layer_id: &str, markers: &MarkerConfiguration) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::MissingArchitectureNode,
        severity: DiagnosticSeverity::Warning,
        message: format!(
            "Architecture source contains ArchGraph markers but does not declare a valid node marker ({})",
            markers.node.join(", ")
        ),
        source: Some(SourceLocation {
            file: path.to_path_buf(),
            line: None,
        }),
        layer_id: Some(layer_id.to_owned()),
        group_id: None,
        candidates: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_markdown_nodes_references_subreferences_and_descriptions() {
        let markdown = r#"
# Event Sync

ARCH_NODE:EventSync

## References

ARCH_REFERENCE:DateKey

Used as the canonical day representation when constructing sync windows.

ARCH_SUBREFERENCE:Password Management
Provides password handling local to EventSync.

---
## Invariants
"#;

        let parsed = parse_architecture_source(
            Path::new("ARCHITECTURE.md"),
            markdown,
            &MarkerConfiguration::default(),
            "architecture",
        )
        .expect("markers should enroll markdown");

        assert_eq!(parsed.node.expect("node").name, "EventSync");
        assert_eq!(parsed.references.len(), 2);
        assert_eq!(parsed.references[0].kind, ReferenceKind::Reference);
        assert_eq!(parsed.references[1].kind, ReferenceKind::Subreference);
        assert_eq!(
            parsed.references[0].description.as_deref(),
            Some("Used as the canonical day representation when constructing sync windows.")
        );
        assert_eq!(parsed.source_format, SourceFormat::Markdown);
    }

    #[test]
    fn ignores_candidate_markdown_without_any_markers() {
        let parsed = parse_architecture_source(
            Path::new("README.md"),
            "# Just documentation\n\nNothing to see here.",
            &MarkerConfiguration::default(),
            "overview",
        );
        assert!(parsed.is_none());
    }

    #[test]
    fn parses_decorated_source_without_understanding_the_language() {
        let source = r#"
/// ARCH_NODE:DateKey
///
/// Handles dates in string form "YYYY-MM-DD".
///
/// ARCH_REFERENCE:Clock
/// Used when determining today.
export type DateKey = `${number}-${number}-${number}`;
"#;

        let parsed = parse_architecture_source(
            Path::new("src/datekey.ts"),
            source,
            &MarkerConfiguration::default(),
            "implementation",
        )
        .expect("decorated declaration");

        assert_eq!(parsed.node.expect("node").name, "DateKey");
        assert_eq!(parsed.references.len(), 1);
        assert_eq!(parsed.references[0].target, "Clock");
        assert_eq!(
            parsed.documentation.as_deref(),
            Some("Handles dates in string form \"YYYY-MM-DD\".")
        );
        assert_eq!(
            parsed.references[0].description.as_deref(),
            Some("Used when determining today.")
        );
        assert_eq!(parsed.source_format, SourceFormat::DecoratedText);
    }

    #[test]
    fn decorated_parser_rejects_markers_inside_code_or_explanatory_comments() {
        let source = r#"
const example = "ARCH_NODE:Fake";
/// An example marker is ARCH_NODE:AlsoFake
export const value = 1;
"#;
        assert!(
            parse_architecture_source(
                Path::new("src/example.ts"),
                source,
                &MarkerConfiguration::default(),
                "implementation",
            )
            .is_none()
        );
    }

    #[test]
    fn decorated_parser_supports_block_comment_decoration() {
        let source = r#"
/**
 * ARCH_NODE:Repository
 *
 * Persists account state.
 *
 * ARCH_REFERENCE:Database
 * Stores rows in the database.
 */
class Repository {}
"#;
        let parsed = parse_architecture_source(
            Path::new("Repository.java"),
            source,
            &MarkerConfiguration::default(),
            "implementation",
        )
        .expect("decorated declaration");
        assert_eq!(parsed.node.expect("node").name, "Repository");
        assert_eq!(
            parsed.documentation.as_deref(),
            Some("Persists account state.")
        );
        assert_eq!(
            parsed.references[0].description.as_deref(),
            Some("Stores rows in the database.")
        );
    }

    #[test]
    fn configured_markers_replace_canonical_markers() {
        let markers = MarkerConfiguration {
            reference: vec!["USES".to_owned()],
            ..MarkerConfiguration::default()
        };
        let markdown = "ARCH_NODE:Checkout\nARCH_REFERENCE:Legacy\nUSES:Identity\nRelationship.";

        let parsed = parse_architecture_source(
            Path::new("ARCHITECTURE.md"),
            markdown,
            &markers,
            "architecture",
        )
        .expect("node marker should enroll file");
        assert_eq!(parsed.references.len(), 1);
        assert_eq!(parsed.references[0].target, "Identity");
    }

    #[test]
    fn multiple_nodes_in_one_source_are_diagnosed() {
        let source = "ARCH_NODE:One\nARCH_NODE:Two\n";
        let parsed = parse_architecture_source(
            Path::new("ARCHITECTURE.md"),
            source,
            &MarkerConfiguration::default(),
            "architecture",
        )
        .expect("source should be enrolled");
        assert_eq!(parsed.node.expect("first node").name, "One");
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == DiagnosticCode::MultipleArchitectureNodes)
        );
    }
}
