use archgraph_core::ArchitectureGraph;

#[tauri::command]
fn scan_project(root: String) -> Result<ArchitectureGraph, String> {
    archgraph_core::scan_project(root).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![scan_project])
        .run(tauri::generate_context!())
        .expect("error while running ArchGraph");
}
