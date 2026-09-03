use std::{path::{Path, PathBuf}, process::Command};

use archgraph_core::ArchitectureGraph;

#[tauri::command]
fn scan_project(root: String) -> Result<ArchitectureGraph, String> {
    archgraph_core::scan_project(root).map_err(|error| error.to_string())
}

#[tauri::command]
fn open_project_source(root: String, file: String) -> Result<(), String> {
    let root = PathBuf::from(root)
        .canonicalize()
        .map_err(|error| format!("Could not resolve project root: {error}"))?;
    let requested = root
        .join(file)
        .canonicalize()
        .map_err(|error| format!("Could not resolve source file: {error}"))?;

    if !requested.starts_with(&root) {
        return Err("Refusing to open a source outside the selected project root.".to_owned());
    }
    if !requested.is_file() {
        return Err("The requested declaration is not a file.".to_owned());
    }

    open_with_default_application(&requested)
}

#[cfg(target_os = "linux")]
fn open_with_default_application(path: &Path) -> Result<(), String> {
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open declaration with xdg-open: {error}"))
}

#[cfg(target_os = "macos")]
fn open_with_default_application(path: &Path) -> Result<(), String> {
    Command::new("open")
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open declaration with the default application: {error}"))
}

#[cfg(target_os = "windows")]
fn open_with_default_application(path: &Path) -> Result<(), String> {
    Command::new("cmd")
        .args(["/C", "start", ""])
        .arg(path)
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Could not open declaration with the default application: {error}"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn open_with_default_application(_path: &Path) -> Result<(), String> {
    Err("Opening declarations is not implemented for this platform.".to_owned())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![scan_project, open_project_source])
        .run(tauri::generate_context!())
        .expect("error while running ArchGraph");
}
