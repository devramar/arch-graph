use crate::model::{
    ArchitectureNode, Diagnostic, DiagnosticCode, DiagnosticSeverity, EdgeResolution, NodeKind,
    SourceLocation,
};
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub(crate) struct ModuleSymbol {
    pub name: String,
    pub file: PathBuf,
    pub line: usize,
}

#[derive(Debug, Default)]
pub(crate) struct SymbolIndex {
    by_name: HashMap<String, Vec<ModuleSymbol>>,
}

#[derive(Debug)]
pub(crate) struct Resolution {
    pub node: ArchitectureNode,
    pub resolution: EdgeResolution,
    pub diagnostic: Option<Diagnostic>,
}

impl SymbolIndex {
    pub fn add_source(&mut self, relative_path: &Path, contents: &str) {
        for symbol in extract_exported_symbols(relative_path, contents) {
            let candidates = self.by_name.entry(symbol.name.clone()).or_default();

            // TypeScript declaration merging (for example `type DateKey` plus
            // `namespace DateKey`) should still represent one graph target when
            // the declarations live in the same module.
            if candidates.iter().all(|candidate| candidate.file != symbol.file) {
                candidates.push(symbol);
            }
        }
    }

    pub fn resolve(
        &self,
        target: &str,
        architectures: &HashMap<String, Vec<ArchitectureNode>>,
        source: &SourceLocation,
    ) -> Resolution {
        if let Some(candidates) = architectures.get(target) {
            if candidates.len() == 1 {
                return Resolution {
                    node: candidates[0].clone(),
                    resolution: EdgeResolution::Architecture,
                    diagnostic: None,
                };
            }

            if candidates.len() > 1 {
                return ambiguous_resolution(target, candidates, source);
            }
        }

        if let Some(symbols) = self.by_name.get(target) {
            if symbols.len() == 1 {
                let symbol = &symbols[0];
                return Resolution {
                    node: ArchitectureNode {
                        id: module_node_id(&symbol.file, &symbol.name),
                        name: symbol.name.clone(),
                        kind: NodeKind::Module,
                        source: Some(SourceLocation {
                            file: symbol.file.clone(),
                            line: Some(symbol.line),
                        }),
                        summary: None,
                        documentation: None,
                    },
                    resolution: EdgeResolution::Module,
                    diagnostic: None,
                };
            }

            let candidates = symbols
                .iter()
                .map(|symbol| format!("{}#{}", symbol.file.display(), symbol.name))
                .collect::<Vec<_>>();

            return unresolved_node(
                target,
                EdgeResolution::Ambiguous,
                Some(Diagnostic {
                    code: DiagnosticCode::AmbiguousDependency,
                    severity: DiagnosticSeverity::Warning,
                    message: format!("Ambiguous dependency: {target}"),
                    source: Some(source.clone()),
                    candidates,
                }),
            );
        }

        unresolved_node(
            target,
            EdgeResolution::Unresolved,
            Some(Diagnostic {
                code: DiagnosticCode::UnresolvedDependency,
                severity: DiagnosticSeverity::Warning,
                message: format!("Could not resolve dependency: {target}"),
                source: Some(source.clone()),
                candidates: Vec::new(),
            }),
        )
    }
}

fn extract_exported_symbols(relative_path: &Path, contents: &str) -> Vec<ModuleSymbol> {
    static EXPORT_RE: OnceLock<Regex> = OnceLock::new();
    let regex = EXPORT_RE.get_or_init(|| {
        Regex::new(
            r"(?x)^\s*export\s+(?:declare\s+)?(?:default\s+)?(?:abstract\s+)?(?:async\s+)?(?:class|interface|type|enum|namespace|function|const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)",
        )
        .expect("valid export regex")
    });

    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            regex.captures(line).and_then(|captures| {
                captures.get(1).map(|name| ModuleSymbol {
                    name: name.as_str().to_owned(),
                    file: relative_path.to_path_buf(),
                    line: index + 1,
                })
            })
        })
        .collect()
}

fn ambiguous_resolution(
    target: &str,
    candidates: &[ArchitectureNode],
    source: &SourceLocation,
) -> Resolution {
    unresolved_node(
        target,
        EdgeResolution::Ambiguous,
        Some(Diagnostic {
            code: DiagnosticCode::AmbiguousDependency,
            severity: DiagnosticSeverity::Warning,
            message: format!("Ambiguous architecture dependency: {target}"),
            source: Some(source.clone()),
            candidates: candidates
                .iter()
                .filter_map(|candidate| candidate.source.as_ref())
                .map(|location| location.file.display().to_string())
                .collect(),
        }),
    )
}

fn unresolved_node(
    target: &str,
    resolution: EdgeResolution,
    diagnostic: Option<Diagnostic>,
) -> Resolution {
    Resolution {
        node: ArchitectureNode {
            id: format!("unresolved:{target}"),
            name: target.to_owned(),
            kind: NodeKind::Unresolved,
            source: None,
            summary: None,
            documentation: None,
        },
        resolution,
        diagnostic,
    }
}

pub(crate) fn architecture_node_id(relative_architecture_file: &Path, name: &str) -> String {
    format!("arch:{}#{name}", relative_architecture_file.display())
}

fn module_node_id(relative_file: &Path, name: &str) -> String {
    format!("module:{}#{name}", relative_file.display())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_common_typescript_exports() {
        let source = r#"
export type DateKey = string;
export namespace DateKey {}
export interface EventStore {}
export class Thing {}
const hidden = 1;
"#;
        let mut index = SymbolIndex::default();
        index.add_source(Path::new("src/types.ts"), source);

        assert_eq!(index.by_name["EventStore"].len(), 1);
        assert_eq!(index.by_name["Thing"].len(), 1);
        assert!(!index.by_name.contains_key("hidden"));
        assert_eq!(index.by_name["DateKey"].len(), 1);
    }
}
