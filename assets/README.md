# Application Assets

## App icon

Place the editable/master ArchGraph icon here as:

```text
app-icon.png
```

Recommended format:

- square PNG
- 1024x1024
- transparent background where appropriate
- artwork kept comfortably inside the canvas so platform masks do not crop important details

A square SVG is also accepted if you prefer to keep the master artwork as vector data.

Generate every Tauri platform icon from the master with:

```bash
./scripts/generate-icons.sh
```

Generated icons live under `apps/desktop/src-tauri/icons/` and are the files referenced by the Tauri bundle configuration.
