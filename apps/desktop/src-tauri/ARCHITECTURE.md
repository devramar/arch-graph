# Native Desktop Adapter

ARCH_NODE:Native Desktop Adapter

Native Desktop Adapter is the Tauri-side boundary between the webview and ArchGraph Core. It exists to expose a deliberately small set of native operations while keeping architecture semantics inside the reusable core crate.

The adapter owns Tauri application setup, plugin registration, command exposure, and the native capability boundary required by the desktop shell. It does not maintain an alternate graph model or parse project files itself.

## References

ARCH_REFERENCE:Tauri Command Surface

Native Desktop Adapter exposes Tauri Command Surface as the concrete set of webview-callable operations for scanning, composition, configuration persistence, and source opening.

ARCH_REFERENCE:ArchGraph Core

Native Desktop Adapter delegates project interpretation and composition to ArchGraph Core and converts only Rust errors into command-safe strings at the application boundary.

ARCH_REFERENCE:Desktop Capability Boundary

Native Desktop Adapter runs within Desktop Capability Boundary so the main window receives only the native permissions required by the current application surface.

ARCH_SUBREFERENCE:Tauri Plugin Registration

Native Desktop Adapter registers the dialog and opener plugins used by the command surface and frontend integration without making those plugins part of the graph semantics.
