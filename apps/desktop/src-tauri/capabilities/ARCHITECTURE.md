# Desktop Capability Boundary

ARCH_NODE:Desktop Capability Boundary

Desktop Capability Boundary defines the native permission envelope granted to the ArchGraph main window. The current capability set keeps the webview on Tauri's default core surface and permits folder-dialog opening while native command handlers retain responsibility for project scanning, configuration writes, and guarded source opening.

The boundary is intentionally smaller than the application's functional surface: filesystem-sensitive operations are expressed as application commands rather than broad frontend filesystem permissions.

## References

ARCH_REFERENCE:Native Desktop Adapter

Desktop Capability Boundary constrains the window in which Native Desktop Adapter and its plugins operate, granting only the permissions required by the current desktop interaction model.

ARCH_SUBREFERENCE:Folder Dialog Permission

Desktop Capability Boundary grants the main window permission to open the native folder dialog used to choose project roots without granting a general-purpose frontend filesystem API.
