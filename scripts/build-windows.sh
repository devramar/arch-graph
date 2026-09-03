#!/usr/bin/env bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

require_desktop_tools
require_command cargo-xwin

ARCH="${1:-x86_64}"
shift || true

case "$ARCH" in
    x86_64) RUST_TARGET="x86_64-pc-windows-msvc" ;;
    aarch64|arm64) RUST_TARGET="aarch64-pc-windows-msvc" ;;
    *) echo "Usage: ./scripts/build-windows.sh [x86_64|aarch64] [extra tauri args...]" >&2; exit 2 ;;
esac

run_tauri build --runner cargo-xwin --target "$RUST_TARGET" --bundles nsis "$@"
