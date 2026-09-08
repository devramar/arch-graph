# Tauri Command Surface

ARCH_NODE:Tauri Command Surface

Tauri Command Surface is the native RPC contract implemented in `src-tauri/src/lib.rs`. Four commands bridge the webview to the Rust engine and the operating system: project scan, layer composition, normalized project-configuration update, and project-confined source opening.

Scan and composition return core model values directly. Configuration updates pass through core validation and normalized writing. Source opening canonicalizes the requested path and rejects any file that escapes the selected project root before invoking the system opener.

## References

ARCH_REFERENCE:ArchGraph Core

Tauri Command Surface calls ArchGraph Core for project scans, composition of selected layers, and validated `.archgraph` writes, exposing those operations without duplicating their semantics.

ARCH_REFERENCE:Project Configuration

Tauri Command Surface routes configuration updates through Project Configuration validation and normalized persistence before returning the written configuration to the frontend.

ARCH_REFERENCE:Layer Composition

Tauri Command Surface accepts explicit layer-group definitions from the desktop and delegates Layer Composition to the core using the already scanned layer graphs and diagnostics.

ARCH_REFERENCE:Graph Contract

Tauri Command Surface transports Graph Contract project scans and composed graphs across Tauri serialization as the shared data model between Rust and TypeScript.

ARCH_SUBREFERENCE:Project-Confined Source Opening

Tauri Command Surface canonicalizes both project root and requested declaration path, verifies containment and regular-file status, then asks the operating system to open the declaration only after those checks pass.
