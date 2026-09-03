#!/usr/bin/env bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

require_command npm

ICON_SOURCE="${1:-$ROOT_DIR/assets/app-icon.png}"
if [[ ! -f "$ICON_SOURCE" ]]; then
    echo "Icon source not found: $ICON_SOURCE" >&2
    echo "Place a square transparent PNG at assets/app-icon.png (1024x1024 recommended)," >&2
    echo "or pass a square PNG/SVG path as the first argument." >&2
    exit 1
fi

ICON_SOURCE="$(cd -- "$(dirname -- "$ICON_SOURCE")" && pwd)/$(basename -- "$ICON_SOURCE")"

if [[ ! -d "$DESKTOP_DIR/node_modules" ]]; then
    echo "Frontend dependencies are not installed." >&2
    echo "Run: (cd \"$DESKTOP_DIR\" && npm install)" >&2
    exit 1
fi

run_tauri icon "$ICON_SOURCE" --output "$DESKTOP_DIR/src-tauri/icons"

echo "Generated Tauri icons in apps/desktop/src-tauri/icons/."
