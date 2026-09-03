#!/usr/bin/env bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

if [[ "$(uname -s)" != "Linux" ]]; then
    echo "Linux Tauri bundles should be produced on a Linux host." >&2
    exit 1
fi

require_desktop_tools

ARCH="${1:-}"
if [[ -n "$ARCH" ]]; then
    shift
else
    case "$(uname -m)" in
        x86_64) ARCH="x86_64" ;;
        aarch64|arm64) ARCH="aarch64" ;;
        *) echo "Unsupported host architecture: $(uname -m)" >&2; exit 1 ;;
    esac
fi

case "$ARCH" in
    x86_64) RUST_TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64|arm64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    *) echo "Usage: ./scripts/build-linux.sh [x86_64|aarch64] [extra tauri args...]" >&2; exit 2 ;;
esac

run_tauri build --target "$RUST_TARGET" --bundles deb,rpm,appimage "$@"
