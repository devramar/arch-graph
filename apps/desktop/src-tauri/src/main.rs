fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        // SAFETY: This runs before Tauri/GTK/WebKit initialization and before
        // the application starts any worker threads.
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    archgraph_desktop_lib::run();
}
