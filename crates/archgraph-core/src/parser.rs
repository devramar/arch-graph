use crate::config::AliasingConfiguration;
use crate::model::{Diagnostic, DiagnosticCode, DiagnosticSeverity, ReferenceKind, SourceLocation};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedArchitectureDocument {
    pub source_file: PathBuf,
    pub node: Option<ParsedNode>,
    pub references: Vec<ParsedReference>,
    pub diagnostics: Vec<Diagnostic>,
    pub documentation: String,
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

pub(crate) fn parse_architecture_document(
    relative_path: &Path,
    contents: &str,
    aliases: &AliasingConfiguration,
) -> ParsedArchitectureDocument {
    let lines: Vec<&str> = contents.lines().collect();
    let mut node = None;
    let mut references = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, raw_line) in lines.iter().enumerate() {
        let line = raw_line.trim();
        let line_number = index + 1;

        if let Some((marker, rest)) = marker_value(line, &aliases.node_markers) {
            let name = rest.trim();
            if name.is_empty() {
                diagnostics.push(malformed_marker(relative_path, line_number, marker));
            } else if node.is_none() {
                node = Some(ParsedNode {
                    name: name.to_owned(),
                    line: line_number,
                });
            }
            continue;
        }

        let reference = marker_value(line, &aliases.reference_markers)
            .map(|(marker, rest)| (ReferenceKind::Reference, marker, rest))
            .or_else(|| {
                marker_value(line, &aliases.subreference_markers)
                    .map(|(marker, rest)| (ReferenceKind::Subreference, marker, rest))
            });

        let Some((kind, marker, rest)) = reference else {
            continue;
        };
        let target = rest.trim();
        if target.is_empty() {
            diagnostics.push(malformed_marker(relative_path, line_number, marker));
            continue;
        }

        references.push(ParsedReference {
            target: target.to_owned(),
            description: reference_description(&lines, index + 1, aliases),
            line: line_number,
            kind,
        });
    }

    if node.is_none() {
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::MissingArchitectureNode,
            severity: DiagnosticSeverity::Warning,
            message: format!(
                "Architecture document does not declare a node marker ({})",
                aliases.node_markers.join(", ")
            ),
            source: Some(SourceLocation {
                file: relative_path.to_path_buf(),
                line: None,
            }),
            candidates: Vec::new(),
        });
    }

    ParsedArchitectureDocument {
        source_file: relative_path.to_path_buf(),
        node,
        references,
        diagnostics,
        documentation: contents.to_owned(),
    }
}

fn marker_value<'a>(line: &'a str, aliases: &[String]) -> Option<(&'a str, &'a str)> {
    aliases.iter().find_map(|alias| {
        let remainder = line.strip_prefix(alias)?;
        let rest = remainder.strip_prefix(':')?;
        Some((&line[..alias.len()], rest))
    })
}

fn reference_description(
    lines: &[&str],
    start_index: usize,
    aliases: &AliasingConfiguration,
) -> Option<String> {
    let mut collected = Vec::new();
    let mut started = false;

    for raw in lines.iter().skip(start_index) {
        let trimmed = raw.trim();

        if is_archgraph_marker(trimmed, aliases) || trimmed.starts_with("## ") || trimmed == "---" {
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

    while matches!(collected.last(), Some(last) if last.is_empty()) {
        collected.pop();
    }

    if collected.is_empty() {
        None
    } else {
        Some(collected.join("\n"))
    }
}

fn is_archgraph_marker(line: &str, aliases: &AliasingConfiguration) -> bool {
    marker_value(line, &aliases.node_markers).is_some()
        || marker_value(line, &aliases.reference_markers).is_some()
        || marker_value(line, &aliases.subreference_markers).is_some()
}

fn malformed_marker(path: &Path, line: usize, marker: &str) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::MalformedMarker,
        severity: DiagnosticSeverity::Warning,
        message: format!("{marker} marker has no target name"),
        source: Some(SourceLocation {
            file: path.to_path_buf(),
            line: Some(line),
        }),
        candidates: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nodes_references_subreferences_and_descriptions() {
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

        let parsed = parse_architecture_document(
            Path::new("ARCHITECTURE.md"),
            markdown,
            &AliasingConfiguration::default(),
        );

        assert_eq!(parsed.node.unwrap().name, "EventSync");
        assert_eq!(parsed.references.len(), 2);
        assert_eq!(parsed.references[0].kind, ReferenceKind::Reference);
        assert_eq!(parsed.references[1].kind, ReferenceKind::Subreference);
        assert_eq!(
            parsed.references[0].description.as_deref(),
            Some("Used as the canonical day representation when constructing sync windows.")
        );
    }

    #[test]
    fn configured_aliases_replace_canonical_markers() {
        let aliases = AliasingConfiguration {
            reference_markers: vec!["USES".to_owned()],
            ..AliasingConfiguration::default()
        };
        let markdown = "ARCH_NODE:Checkout\nARCH_REFERENCE:Legacy\nUSES:Identity\nRelationship.";

        let parsed = parse_architecture_document(Path::new("ARCHITECTURE.md"), markdown, &aliases);
        assert_eq!(parsed.references.len(), 1);
        assert_eq!(parsed.references[0].target, "Identity");
    }

    #[test]
    fn uses_configured_aliases_without_requiring_markdown_sections() {
        let aliases = AliasingConfiguration {
            node_markers: vec!["SYSTEM".to_owned()],
            reference_markers: vec!["USES".to_owned()],
            subreference_markers: vec!["LOCAL".to_owned()],
            ..AliasingConfiguration::default()
        };
        let markdown =
            "SYSTEM:Checkout\nUSES:Identity\nRelationship.\nLOCAL:PCI DSS\nLocal relationship.";

        let parsed = parse_architecture_document(Path::new("SYSTEM.md"), markdown, &aliases);
        assert_eq!(parsed.node.unwrap().name, "Checkout");
        assert_eq!(parsed.references.len(), 2);
    }
}
