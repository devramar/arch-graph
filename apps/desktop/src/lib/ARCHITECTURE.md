# Desktop Services

ARCH_NODE:Desktop Services

Desktop Services is the small frontend integration layer in `src/lib`. It isolates React code from Tauri invocation details and supplies the browser-only demo graph used when the native runtime is unavailable.

The native service functions map frontend contract types directly onto named Tauri commands for scanning, composition, configuration writes, and source opening. Folder selection and webview drag/drop event registration are also centralized here so the rest of the application can remain runtime-agnostic at call sites.

## References

ARCH_REFERENCE:Native Desktop Adapter

Desktop Services invokes Native Desktop Adapter commands with typed project, layer, configuration, and source-location payloads and returns the resulting core-owned graph/configuration values to Desktop Application.

ARCH_REFERENCE:Desktop Application

Desktop Services supplies Desktop Application with a narrow asynchronous API for native capabilities, keeping runtime detection and Tauri-specific invocation mechanics outside application orchestration code.

ARCH_REFERENCE:Graph Contract

Desktop Services transports Graph Contract values across the Tauri bridge without altering node, edge, layer, declaration, or diagnostic semantics.

ARCH_SUBREFERENCE:Browser Demo Graph

Desktop Services provides a static in-memory scan for non-Tauri browser preview so the graph workspace can be exercised without pretending browser mode can scan arbitrary local projects.
