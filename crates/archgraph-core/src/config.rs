use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub const PROJECT_CONFIG_FILENAME: &str = ".archgraph";

pub type ViewSettings = BTreeMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectConfiguration {
    pub aliasing: AliasingConfiguration,
    pub app_colours: AppColoursConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    pub view_settings: ViewSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct AliasingConfiguration {
    #[serde(rename = "ARCHITECTURE.md")]
    pub architecture_files: Vec<String>,
    #[serde(rename = "ARCH_NODE")]
    pub node_markers: Vec<String>,
    #[serde(rename = "ARCH_REFERENCE")]
    pub reference_markers: Vec<String>,
    #[serde(rename = "ARCH_SUBREFERENCE")]
    pub subreference_markers: Vec<String>,
}

impl Default for AliasingConfiguration {
    fn default() -> Self {
        Self {
            architecture_files: vec!["ARCHITECTURE.md".to_owned()],
            node_markers: vec!["ARCH_NODE".to_owned()],
            reference_markers: vec!["ARCH_REFERENCE".to_owned()],
            subreference_markers: vec!["ARCH_SUBREFERENCE".to_owned()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(default, deny_unknown_fields)]
pub struct AppColoursConfiguration {
    pub colour_overrides: ColourOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(default, deny_unknown_fields)]
pub struct ColourOverrides {
    pub references: BTreeMap<String, String>,
    pub subreferences: BTreeMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("project root does not exist: {0}")]
    RootMissing(PathBuf),
    #[error("project root is not a directory: {0}")]
    RootNotDirectory(PathBuf),
    #[error("failed to canonicalize project root {path}: {source}")]
    Canonicalize { path: PathBuf, source: io::Error },
    #[error("failed to read project configuration {path}: {source}")]
    Read { path: PathBuf, source: io::Error },
    #[error("failed to parse project configuration {path}: {source}")]
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("invalid project configuration: {0}")]
    Invalid(String),
    #[error("failed to serialize project configuration: {0}")]
    Serialize(serde_json::Error),
    #[error("failed to write project configuration {path}: {source}")]
    Write { path: PathBuf, source: io::Error },
}

pub fn load_project_configuration(
    root: impl AsRef<Path>,
) -> Result<ProjectConfiguration, ConfigurationError> {
    let root = canonical_project_root(root.as_ref())?;
    load_project_configuration_from_root(&root)
}

pub fn write_project_configuration(
    root: impl AsRef<Path>,
    configuration: &ProjectConfiguration,
) -> Result<ProjectConfiguration, ConfigurationError> {
    let root = canonical_project_root(root.as_ref())?;
    let configuration = normalize_and_validate(configuration.clone())?;
    let path = root.join(PROJECT_CONFIG_FILENAME);
    validate_configuration_path(&path)?;
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temporary = root.join(format!(".archgraph.tmp-{}-{suffix}", std::process::id()));

    let mut bytes =
        serde_json::to_vec_pretty(&configuration).map_err(ConfigurationError::Serialize)?;
    bytes.push(b'\n');

    let write_result = (|| -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&temporary)?;
        file.write_all(&bytes)?;
        file.sync_all()?;

        replace_configuration_file(&temporary, &path)
    })();

    if let Err(source) = write_result {
        let _ = fs::remove_file(&temporary);
        return Err(ConfigurationError::Write { path, source });
    }

    Ok(configuration)
}

pub(crate) fn load_project_configuration_from_root(
    root: &Path,
) -> Result<ProjectConfiguration, ConfigurationError> {
    let path = root.join(PROJECT_CONFIG_FILENAME);
    if !validate_configuration_path(&path)? {
        return Ok(ProjectConfiguration::default());
    }

    let contents = fs::read_to_string(&path).map_err(|source| ConfigurationError::Read {
        path: path.clone(),
        source,
    })?;
    let parsed = serde_json::from_str::<ProjectConfiguration>(&contents).map_err(|source| {
        ConfigurationError::Parse {
            path: path.clone(),
            source,
        }
    })?;
    normalize_and_validate(parsed)
}

fn validate_configuration_path(path: &Path) -> Result<bool, ConfigurationError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(source) => {
            return Err(ConfigurationError::Read {
                path: path.to_path_buf(),
                source,
            });
        }
    };

    if metadata.file_type().is_symlink() {
        return Err(ConfigurationError::Invalid(format!(
            "{} may not be a symbolic link",
            path.display()
        )));
    }
    if !metadata.is_file() {
        return Err(ConfigurationError::Invalid(format!(
            "{} must be a regular file",
            path.display()
        )));
    }

    Ok(true)
}

fn replace_configuration_file(temporary: &Path, destination: &Path) -> io::Result<()> {
    match fs::rename(temporary, destination) {
        Ok(()) => return Ok(()),
        Err(error) if !destination.exists() => return Err(error),
        Err(_) => {}
    }

    // Windows does not replace an existing destination with rename(). Preserve
    // the old file as a temporary backup so a failed second rename can be rolled
    // back rather than leaving the project without configuration.
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let backup =
        destination.with_file_name(format!(".archgraph.backup-{}-{suffix}", std::process::id()));

    fs::rename(destination, &backup)?;
    match fs::rename(temporary, destination) {
        Ok(()) => {
            let _ = fs::remove_file(backup);
            Ok(())
        }
        Err(error) => {
            let _ = fs::rename(&backup, destination);
            Err(error)
        }
    }
}

fn canonical_project_root(root: &Path) -> Result<PathBuf, ConfigurationError> {
    if !root.exists() {
        return Err(ConfigurationError::RootMissing(root.to_path_buf()));
    }
    if !root.is_dir() {
        return Err(ConfigurationError::RootNotDirectory(root.to_path_buf()));
    }

    root.canonicalize()
        .map_err(|source| ConfigurationError::Canonicalize {
            path: root.to_path_buf(),
            source,
        })
}

fn normalize_and_validate(
    mut configuration: ProjectConfiguration,
) -> Result<ProjectConfiguration, ConfigurationError> {
    normalize_list(&mut configuration.aliasing.architecture_files);
    normalize_list(&mut configuration.aliasing.node_markers);
    normalize_list(&mut configuration.aliasing.reference_markers);
    normalize_list(&mut configuration.aliasing.subreference_markers);

    validate_non_empty_list(
        "aliasing.ARCHITECTURE.md",
        &configuration.aliasing.architecture_files,
    )?;
    validate_non_empty_list("aliasing.ARCH_NODE", &configuration.aliasing.node_markers)?;
    validate_non_empty_list(
        "aliasing.ARCH_REFERENCE",
        &configuration.aliasing.reference_markers,
    )?;
    validate_non_empty_list(
        "aliasing.ARCH_SUBREFERENCE",
        &configuration.aliasing.subreference_markers,
    )?;

    for filename in &configuration.aliasing.architecture_files {
        if filename == "."
            || filename == ".."
            || filename == PROJECT_CONFIG_FILENAME
            || filename.contains('/')
            || filename.contains('\\')
        {
            return Err(ConfigurationError::Invalid(format!(
                "architecture document alias must be a filename, got {filename:?}"
            )));
        }
    }

    let marker_groups = [
        ("ARCH_NODE", &configuration.aliasing.node_markers),
        ("ARCH_REFERENCE", &configuration.aliasing.reference_markers),
        (
            "ARCH_SUBREFERENCE",
            &configuration.aliasing.subreference_markers,
        ),
    ];
    let mut owners: HashMap<&str, &str> = HashMap::new();
    for (semantic_name, markers) in marker_groups {
        for marker in markers {
            if marker.contains(':') || marker.contains('\n') || marker.contains('\r') {
                return Err(ConfigurationError::Invalid(format!(
                    "marker alias {marker:?} for {semantic_name} may not contain ':' or a newline"
                )));
            }
            if let Some(existing) = owners.insert(marker.as_str(), semantic_name) {
                return Err(ConfigurationError::Invalid(format!(
                    "marker alias {marker:?} is assigned to both {existing} and {semantic_name}"
                )));
            }
        }
    }

    normalize_colour_overrides(&mut configuration.app_colours.colour_overrides.references)?;
    normalize_colour_overrides(&mut configuration.app_colours.colour_overrides.subreferences)?;

    if let Some(default_view) = &mut configuration.default_view {
        *default_view = default_view.trim().to_owned();
        if default_view.is_empty() {
            return Err(ConfigurationError::Invalid(
                "default_view may not be empty".to_owned(),
            ));
        }
    }

    Ok(configuration)
}

fn normalize_list(values: &mut Vec<String>) {
    for value in values.iter_mut() {
        *value = value.trim().to_owned();
    }
    values.retain(|value| !value.is_empty());
    values.sort();
    values.dedup();
}

fn validate_non_empty_list(name: &str, values: &[String]) -> Result<(), ConfigurationError> {
    if values.is_empty() {
        return Err(ConfigurationError::Invalid(format!(
            "{name} must contain at least one alias"
        )));
    }
    Ok(())
}

fn normalize_colour_overrides(
    overrides: &mut BTreeMap<String, String>,
) -> Result<(), ConfigurationError> {
    let original = std::mem::take(overrides);
    for (raw_name, raw_colour) in original {
        let name = raw_name.trim().to_owned();
        let colour = raw_colour.trim().to_owned();
        if name.is_empty() || colour.is_empty() {
            return Err(ConfigurationError::Invalid(
                "colour override names and colour names may not be empty".to_owned(),
            ));
        }
        if overrides.insert(name.clone(), colour).is_some() {
            return Err(ConfigurationError::Invalid(format!(
                "colour override name {name:?} is duplicated after normalization"
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_use_canonical_archgraph_tokens() {
        let config = ProjectConfiguration::default();
        assert_eq!(
            config.aliasing.architecture_files,
            vec!["ARCHITECTURE.md".to_owned()]
        );
        assert_eq!(config.aliasing.node_markers, vec!["ARCH_NODE".to_owned()]);
        assert_eq!(
            config.aliasing.reference_markers,
            vec!["ARCH_REFERENCE".to_owned()]
        );
        assert_eq!(
            config.aliasing.subreference_markers,
            vec!["ARCH_SUBREFERENCE".to_owned()]
        );
    }

    #[test]
    fn rejects_marker_alias_collisions() {
        let config = ProjectConfiguration {
            aliasing: AliasingConfiguration {
                reference_markers: vec!["REF".to_owned()],
                subreference_markers: vec!["REF".to_owned()],
                ..AliasingConfiguration::default()
            },
            ..ProjectConfiguration::default()
        };

        assert!(matches!(
            normalize_and_validate(config),
            Err(ConfigurationError::Invalid(_))
        ));
    }

    #[test]
    fn rejects_unknown_configuration_fields() {
        let json = r#"{ "unknown": true }"#;
        assert!(serde_json::from_str::<ProjectConfiguration>(json).is_err());
    }

    #[test]
    fn writes_and_reloads_normalized_configuration() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "archgraph-config-test-{}-{suffix}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("temp project root");

        let mut config = ProjectConfiguration {
            aliasing: AliasingConfiguration {
                reference_markers: vec![" SYS_REF ".to_owned(), "ARCH_REF".to_owned()],
                ..AliasingConfiguration::default()
            },
            default_view: Some(" sticky ".to_owned()),
            ..ProjectConfiguration::default()
        };
        config
            .app_colours
            .colour_overrides
            .subreferences
            .insert(" Password Management ".to_owned(), " purple ".to_owned());

        let written = write_project_configuration(&root, &config).expect("configuration write");
        let loaded = load_project_configuration(&root).expect("configuration reload");
        assert_eq!(written, loaded);
        assert_eq!(loaded.default_view.as_deref(), Some("sticky"));
        assert_eq!(
            loaded.aliasing.reference_markers,
            vec!["ARCH_REF".to_owned(), "SYS_REF".to_owned()]
        );
        assert_eq!(
            loaded
                .app_colours
                .colour_overrides
                .subreferences
                .get("Password Management")
                .map(String::as_str),
            Some("purple")
        );

        fs::remove_dir_all(root).expect("temp project cleanup");
    }
}
