#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
TARGET="${1:-native}"
shift || true

case "$TARGET" in
    native)
        exec "$SCRIPT_DIR/build-native.sh" "$@"
        ;;
    linux)
        exec "$SCRIPT_DIR/build-linux.sh" "$@"
        ;;
    windows)
        exec "$SCRIPT_DIR/build-windows.sh" "$@"
        ;;
    macos)
        exec "$SCRIPT_DIR/build-macos.sh" "$@"
        ;;
    core)
        exec "$SCRIPT_DIR/build-core.sh" "$@"
        ;;
    *)
        cat >&2 <<USAGE
Usage: ./scripts/build.sh <target> [target options]

Targets:
  native              Build a Tauri bundle for the current host.
  linux [arch]        Build Linux bundles (x86_64 or aarch64).
  windows [arch]      Build Windows NSIS using cargo-xwin (x86_64 or aarch64).
  macos [arch]        Build on macOS (native, universal, x86_64, or aarch64).
  core                Build the standalone Rust core and CLI.
USAGE
        exit 2
        ;;
esac
