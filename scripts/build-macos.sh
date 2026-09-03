#!/usr/bin/env bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "macOS application bundles must be produced on a macOS host." >&2
    exit 1
fi

require_desktop_tools

ARCH="${1:-native}"
shift || true

case "$ARCH" in
    native)
        run_tauri build "$@"
        ;;
    universal)
        run_tauri build --target universal-apple-darwin "$@"
        ;;
    x86_64)
        run_tauri build --target x86_64-apple-darwin "$@"
        ;;
    aarch64|arm64)
        run_tauri build --target aarch64-apple-darwin "$@"
        ;;
    *)
        echo "Usage: ./scripts/build-macos.sh [native|universal|x86_64|aarch64] [extra tauri args...]" >&2
        exit 2
        ;;
esac
