#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "$SCRIPT_DIR/.." && pwd)"
DESKTOP_DIR="$ROOT_DIR/apps/desktop"

require_command() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Required command not found: $1" >&2
        exit 1
    fi
}

require_desktop_tools() {
    require_command npm
    require_command cargo

    if [[ ! -d "$DESKTOP_DIR/node_modules" ]]; then
        echo "Frontend dependencies are not installed." >&2
        echo "Run: (cd \"$DESKTOP_DIR\" && npm install)" >&2
        exit 1
    fi
}

run_tauri() {
    npm --prefix "$DESKTOP_DIR" run tauri -- "$@"
}
