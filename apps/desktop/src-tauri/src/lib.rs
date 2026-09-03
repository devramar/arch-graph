use std::path::PathBuf;

use archgraph_core::ArchitectureGraph;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
fn scan_project(root: String) -> Result<ArchitectureGraph, String> {
    archgraph_core::scan_project(root).map_err(|error| error.to_string())
}

#[tauri::command]
fn open_project_source(app: AppHandle, root: String, file: String) -> Result<(), String> {
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

    app.opener()
        .open_path(requested.to_string_lossy().to_string(), None::<String>)
        .map_err(|error| format!("Could not open declaration with the system default application: {error}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_project, open_project_source])
        .run(tauri::generate_context!())
        .expect("error while running ArchGraph");
}
