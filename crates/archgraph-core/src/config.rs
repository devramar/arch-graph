use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub const PROJECT_CONFIG_FILENAME: &str = ".archgraph";
pub const DEFAULT_LAYER_ID: &str = "architecture";

pub type ViewSettings = BTreeMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct ProjectConfiguration {
    pub ignored_paths: Vec<String>,
    pub layers: BTreeMap<String, LayerConfiguration>,
    pub app_colours: AppColoursConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    pub view_settings: ViewSettings,
}

impl Default for ProjectConfiguration {
    fn default() -> Self {
        Self {
            ignored_paths: Vec::new(),
            layers: BTreeMap::from([(
                DEFAULT_LAYER_ID.to_owned(),
                LayerConfiguration::default_architecture(),
            )]),
            app_colours: AppColoursConfiguration::default(),
            default_view: None,
            view_settings: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(default, deny_unknown_fields)]
pub struct LayerConfiguration {
    pub display_name: String,
    pub path_root: String,
    pub ignored_paths: Vec<String>,
    pub files: Vec<String>,
    pub markers: MarkerConfiguration,
}

impl LayerConfiguration {
    fn default_architecture() -> Self {
        Self {
            display_name: "Architecture".to_owned(),
            path_root: ".".to_owned(),
            ignored_paths: Vec::new(),
            files: vec!["ARCHITECTURE.md".to_owned()],
            markers: MarkerConfiguration::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct MarkerConfiguration {
    #[serde(rename = "ARCH_NODE")]
    pub node: Vec<String>,
    #[serde(rename = "ARCH_REFERENCE")]
    pub reference: Vec<String>,
    #[serde(rename = "ARCH_SUBREFERENCE")]
    pub subreference: Vec<String>,
}

impl Default for MarkerConfiguration {
    fn default() -> Self {
        Self {
            node: vec!["ARCH_NODE".to_owned()],
            reference: vec!["ARCH_REFERENCE".to_owned()],
            subreference: vec!["ARCH_SUBREFERENCE".to_owned()],
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
    Ok(load_project_configuration_state_from_root(&root)?.0)
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

pub(crate) fn load_project_configuration_state_from_root(
    root: &Path,
) -> Result<(ProjectConfiguration, bool), ConfigurationError> {
    let path = root.join(PROJECT_CONFIG_FILENAME);
    if !validate_configuration_path(&path)? {
        return Ok((ProjectConfiguration::default(), false));
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
    Ok((normalize_and_validate(parsed)?, true))
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
    normalize_path_list(&mut configuration.ignored_paths);

    configuration
        .layers
        .entry(DEFAULT_LAYER_ID.to_owned())
        .or_insert_with(LayerConfiguration::default_architecture);

    for (layer_id, layer) in &mut configuration.layers {
        let normalized_id = layer_id.trim();
        if normalized_id.is_empty() || normalized_id != layer_id {
            return Err(ConfigurationError::Invalid(format!(
                "layer id {layer_id:?} must be non-empty and may not have surrounding whitespace"
            )));
        }
        if layer_id.chars().any(char::is_whitespace) {
            return Err(ConfigurationError::Invalid(format!(
                "layer id {layer_id:?} may not contain whitespace"
            )));
        }

        layer.display_name = layer.display_name.trim().to_owned();
        if layer.display_name.is_empty() {
            layer.display_name = if layer_id == DEFAULT_LAYER_ID {
                "Architecture".to_owned()
            } else {
                humanize_layer_id(layer_id)
            };
        }

        layer.path_root = normalize_layer_path_root(&layer.path_root)?;
        normalize_path_list(&mut layer.ignored_paths);
        for pattern in &layer.ignored_paths {
            validate_glob_pattern(pattern, &format!("layers.{layer_id}.ignored_paths"))?;
        }

        normalize_path_list(&mut layer.files);
        if layer.files.is_empty() && layer_id == DEFAULT_LAYER_ID {
            layer.files.push("ARCHITECTURE.md".to_owned());
        }
        validate_non_empty_list(&format!("layers.{layer_id}.files"), &layer.files)?;
        for pattern in &layer.files {
            validate_glob_pattern(pattern, &format!("layers.{layer_id}.files"))?;
            if pattern == PROJECT_CONFIG_FILENAME {
                return Err(ConfigurationError::Invalid(format!(
                    "{} may not be enrolled as an architecture source",
                    PROJECT_CONFIG_FILENAME
                )));
            }
        }

        normalize_string_list(&mut layer.markers.node);
        normalize_string_list(&mut layer.markers.reference);
        normalize_string_list(&mut layer.markers.subreference);
        validate_non_empty_list(
            &format!("layers.{layer_id}.markers.ARCH_NODE"),
            &layer.markers.node,
        )?;
        validate_non_empty_list(
            &format!("layers.{layer_id}.markers.ARCH_REFERENCE"),
            &layer.markers.reference,
        )?;
        validate_non_empty_list(
            &format!("layers.{layer_id}.markers.ARCH_SUBREFERENCE"),
            &layer.markers.subreference,
        )?;
        validate_marker_groups(layer_id, &layer.markers)?;
    }

    for pattern in &configuration.ignored_paths {
        validate_glob_pattern(pattern, "ignored_paths")?;
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

fn validate_marker_groups(
    layer_id: &str,
    markers: &MarkerConfiguration,
) -> Result<(), ConfigurationError> {
    let marker_groups = [
        ("ARCH_NODE", &markers.node),
        ("ARCH_REFERENCE", &markers.reference),
        ("ARCH_SUBREFERENCE", &markers.subreference),
    ];
    let mut owners: HashMap<&str, &str> = HashMap::new();
    for (semantic_name, aliases) in marker_groups {
        for marker in aliases {
            if marker.contains(':') || marker.contains('\n') || marker.contains('\r') {
                return Err(ConfigurationError::Invalid(format!(
                    "marker alias {marker:?} for layer {layer_id:?} {semantic_name} may not contain ':' or a newline"
                )));
            }
            if let Some(existing) = owners.insert(marker.as_str(), semantic_name) {
                return Err(ConfigurationError::Invalid(format!(
                    "marker alias {marker:?} in layer {layer_id:?} is assigned to both {existing} and {semantic_name}"
                )));
            }
        }
    }
    Ok(())
}

fn validate_glob_pattern(pattern: &str, field: &str) -> Result<(), ConfigurationError> {
    if pattern.is_empty() {
        return Err(ConfigurationError::Invalid(format!(
            "{field} may not contain an empty glob"
        )));
    }
    if pattern.contains('\0') || pattern.contains('\n') || pattern.contains('\r') {
        return Err(ConfigurationError::Invalid(format!(
            "glob {pattern:?} in {field} contains an unsupported control character"
        )));
    }
    Ok(())
}

fn humanize_layer_id(layer_id: &str) -> String {
    let mut output = String::with_capacity(layer_id.len());
    let mut capitalize = true;
    for character in layer_id.chars() {
        if character == '-' || character == '_' {
            output.push(' ');
            capitalize = true;
        } else if capitalize {
            output.extend(character.to_uppercase());
            capitalize = false;
        } else {
            output.push(character);
        }
    }
    output
}

fn normalize_string_list(values: &mut Vec<String>) {
    for value in values.iter_mut() {
        *value = value.trim().to_owned();
    }
    values.retain(|value| !value.is_empty());
    values.sort();
    values.dedup();
}

fn normalize_path_list(values: &mut Vec<String>) {
    for value in values.iter_mut() {
        *value = value.trim().replace('\\', "/");
    }
    values.retain(|value| !value.is_empty());
    values.sort();
    values.dedup();
}

fn normalize_layer_path_root(path_root: &str) -> Result<String, ConfigurationError> {
    let normalized = path_root.trim().replace('\\', "/");
    if normalized.is_empty() || normalized == "." || normalized == "./" {
        return Ok(".".to_owned());
    }
    if normalized.starts_with('/')
        || normalized
            .as_bytes()
            .get(1)
            .is_some_and(|separator| *separator == b':')
    {
        return Err(ConfigurationError::Invalid(format!(
            "layer path_root {path_root:?} must be relative to the project root"
        )));
    }
    if normalized.contains('*') || normalized.contains('?') {
        return Err(ConfigurationError::Invalid(format!(
            "layer path_root {path_root:?} must be a literal directory path, not a glob"
        )));
    }
    if normalized.contains('\0') || normalized.contains('\n') || normalized.contains('\r') {
        return Err(ConfigurationError::Invalid(format!(
            "layer path_root {path_root:?} contains an unsupported control character"
        )));
    }

    let mut segments = Vec::new();
    for segment in normalized.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                return Err(ConfigurationError::Invalid(format!(
                    "layer path_root {path_root:?} must stay within the project root"
                )));
            }
            _ => segments.push(segment),
        }
    }

    if segments.is_empty() {
        Ok(".".to_owned())
    } else {
        Ok(segments.join("/"))
    }
}

fn validate_non_empty_list(name: &str, values: &[String]) -> Result<(), ConfigurationError> {
    if values.is_empty() {
        return Err(ConfigurationError::Invalid(format!(
            "{name} must contain at least one value"
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
    fn defaults_include_the_architecture_layer() {
        let config = ProjectConfiguration::default();
        let architecture = config
            .layers
            .get(DEFAULT_LAYER_ID)
            .expect("architecture layer");
        assert_eq!(architecture.path_root, ".");
        assert!(architecture.ignored_paths.is_empty());
        assert_eq!(architecture.files, vec!["ARCHITECTURE.md".to_owned()]);
        assert_eq!(architecture.markers.node, vec!["ARCH_NODE".to_owned()]);
        assert_eq!(
            architecture.markers.reference,
            vec!["ARCH_REFERENCE".to_owned()]
        );
        assert_eq!(
            architecture.markers.subreference,
            vec!["ARCH_SUBREFERENCE".to_owned()]
        );
    }

    #[test]
    fn layer_path_roots_are_normalized_and_may_not_escape_the_project() {
        let config = ProjectConfiguration {
            layers: BTreeMap::from([(
                DEFAULT_LAYER_ID.to_owned(),
                LayerConfiguration {
                    path_root: " ./crates/archgraph-core/ ".to_owned(),
                    files: vec!["ARCHITECTURE.md".to_owned()],
                    ..LayerConfiguration::default()
                },
            )]),
            ..ProjectConfiguration::default()
        };
        let normalized = normalize_and_validate(config).expect("valid rooted layer");
        assert_eq!(
            normalized.layers[DEFAULT_LAYER_ID].path_root,
            "crates/archgraph-core"
        );

        let escaping = ProjectConfiguration {
            layers: BTreeMap::from([(
                DEFAULT_LAYER_ID.to_owned(),
                LayerConfiguration {
                    path_root: "../outside".to_owned(),
                    files: vec!["ARCHITECTURE.md".to_owned()],
                    ..LayerConfiguration::default()
                },
            )]),
            ..ProjectConfiguration::default()
        };
        assert!(matches!(
            normalize_and_validate(escaping),
            Err(ConfigurationError::Invalid(_))
        ));

        let absolute = ProjectConfiguration {
            layers: BTreeMap::from([(
                DEFAULT_LAYER_ID.to_owned(),
                LayerConfiguration {
                    path_root: "/outside".to_owned(),
                    files: vec!["ARCHITECTURE.md".to_owned()],
                    ..LayerConfiguration::default()
                },
            )]),
            ..ProjectConfiguration::default()
        };
        assert!(matches!(
            normalize_and_validate(absolute),
            Err(ConfigurationError::Invalid(_))
        ));
    }

    #[test]
    fn missing_architecture_layer_is_restored_during_normalization() {
        let config = ProjectConfiguration {
            layers: BTreeMap::from([(
                "implementation".to_owned(),
                LayerConfiguration {
                    files: vec!["*.ts".to_owned()],
                    ..LayerConfiguration::default()
                },
            )]),
            ..ProjectConfiguration::default()
        };
        let normalized = normalize_and_validate(config).expect("valid config");
        assert!(normalized.layers.contains_key(DEFAULT_LAYER_ID));
        assert!(normalized.layers.contains_key("implementation"));
    }

    #[test]
    fn rejects_marker_alias_collisions_inside_a_layer() {
        let config = ProjectConfiguration {
            layers: BTreeMap::from([(
                DEFAULT_LAYER_ID.to_owned(),
                LayerConfiguration {
                    files: vec!["ARCHITECTURE.md".to_owned()],
                    markers: MarkerConfiguration {
                        reference: vec!["REF".to_owned()],
                        subreference: vec!["REF".to_owned()],
                        ..MarkerConfiguration::default()
                    },
                    ..LayerConfiguration::default()
                },
            )]),
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
            ignored_paths: vec![" tests/fixtures/** ".to_owned()],
            default_view: Some(" sticky ".to_owned()),
            ..ProjectConfiguration::default()
        };
        let architecture = config
            .layers
            .get_mut(DEFAULT_LAYER_ID)
            .expect("architecture layer");
        architecture.path_root = " ./crates/core/ ".to_owned();
        architecture.ignored_paths = vec![" generated/** ".to_owned()];
        architecture.markers.reference = vec![" SYS_REF ".to_owned(), "ARCH_REF".to_owned()];
        config
            .app_colours
            .colour_overrides
            .subreferences
            .insert(" Password Management ".to_owned(), " purple ".to_owned());

        let written = write_project_configuration(&root, &config).expect("configuration write");
        let loaded = load_project_configuration(&root).expect("configuration reload");
        assert_eq!(written, loaded);
        assert_eq!(loaded.default_view.as_deref(), Some("sticky"));
        assert_eq!(loaded.ignored_paths, vec!["tests/fixtures/**".to_owned()]);
        assert_eq!(loaded.layers[DEFAULT_LAYER_ID].path_root, "crates/core");
        assert_eq!(
            loaded.layers[DEFAULT_LAYER_ID].ignored_paths,
            vec!["generated/**".to_owned()]
        );
        assert_eq!(
            loaded.layers[DEFAULT_LAYER_ID].markers.reference,
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
