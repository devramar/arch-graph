use crate::model::{Diagnostic, DiagnosticCode, DiagnosticSeverity, SourceLocation};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedArchitectureDocument {
    pub source_file: PathBuf,
    pub node: Option<ParsedNode>,
    pub dependencies: Vec<ParsedDependency>,
    pub diagnostics: Vec<Diagnostic>,
    pub documentation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedNode {
    pub name: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedDependency {
    pub target: String,
    pub description: Option<String>,
    pub line: usize,
}

pub(crate) fn parse_architecture_document(
    relative_path: &Path,
    contents: &str,
) -> ParsedArchitectureDocument {
    let lines: Vec<&str> = contents.lines().collect();
    let mut node = None;
    let mut dependencies = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, raw_line) in lines.iter().enumerate() {
        let line = raw_line.trim();
        let line_number = index + 1;

        if let Some(rest) = line.strip_prefix("ARCH_NODE:") {
            let name = rest.trim();
            if name.is_empty() {
                diagnostics.push(malformed_marker(relative_path, line_number, "ARCH_NODE"));
            } else if node.is_none() {
                node = Some(ParsedNode {
                    name: name.to_owned(),
                    line: line_number,
                });
            }
        }

        if let Some(rest) = line.strip_prefix("ARCH_DEPENDENCY:") {
            let target = rest.trim();
            if target.is_empty() {
                diagnostics.push(malformed_marker(
                    relative_path,
                    line_number,
                    "ARCH_DEPENDENCY",
                ));
                continue;
            }

            dependencies.push(ParsedDependency {
                target: target.to_owned(),
                description: dependency_description(&lines, index + 1),
                line: line_number,
            });
        }
    }

    if node.is_none() {
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::MissingArchitectureNode,
            severity: DiagnosticSeverity::Warning,
            message: "ARCHITECTURE.md does not declare an ARCH_NODE".to_owned(),
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
        dependencies,
        diagnostics,
        documentation: contents.to_owned(),
    }
}

fn dependency_description(lines: &[&str], start_index: usize) -> Option<String> {
    let mut collected = Vec::new();
    let mut started = false;

    for raw in lines.iter().skip(start_index) {
        let trimmed = raw.trim();

        if trimmed.starts_with("ARCH_DEPENDENCY:")
            || trimmed.starts_with("ARCH_NODE:")
            || trimmed.starts_with("## ")
            || trimmed == "---"
        {
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
    fn parses_node_dependencies_and_edge_descriptions() {
        let markdown = r#"
# Event Sync

ARCH_NODE:EventSync

## Dependencies

ARCH_DEPENDENCY:DateKey

Used as the canonical day representation when constructing sync windows.

ARCH_DEPENDENCY:EventStore
Provides persisted event state.

---
## Invariants
"#;

        let parsed = parse_architecture_document(Path::new("ARCHITECTURE.md"), markdown);

        assert_eq!(parsed.node.unwrap().name, "EventSync");
        assert_eq!(parsed.dependencies.len(), 2);
        assert_eq!(
            parsed.dependencies[0].description.as_deref(),
            Some("Used as the canonical day representation when constructing sync windows.")
        );
        assert_eq!(
            parsed.dependencies[1].description.as_deref(),
            Some("Provides persisted event state.")
        );
    }
}
