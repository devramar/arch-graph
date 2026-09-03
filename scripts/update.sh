#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"

BUILD="$PROJECT_ROOT/target/release/archgraph-desktop"

BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"

BIN="$BIN_DIR/archgraph"
DESKTOP="$APP_DIR/archgraph.desktop"
ICON="$ICON_DIR/archgraph.png"

SOURCE_ICON="$PROJECT_ROOT/apps/desktop/src-tauri/icons/128x128@2x.png"

if [[ ! -f "$BUILD" ]]; then
    echo "ArchGraph release build not found:"
    echo "  $BUILD"
    echo
    echo "Build it first."
    exit 1
fi

if [[ ! -e "$BIN" ]]; then
    echo "ArchGraph is not currently installed."
    echo "Use scripts/install.sh first."
    exit 1
fi

echo "Updating ArchGraph..."

install -Dm755 "$BUILD" "$BIN"

if [[ -f "$SOURCE_ICON" ]]; then
    install -Dm644 "$SOURCE_ICON" "$ICON"
fi

mkdir -p "$APP_DIR"

cat > "$DESKTOP" <<EOF
[Desktop Entry]
Type=Application
Name=ArchGraph
Comment=Explore documented project architecture
Exec=$BIN
Icon=archgraph
Terminal=false
Categories=Development;
StartupNotify=true
EOF

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APP_DIR" >/dev/null 2>&1 || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache "$HOME/.local/share/icons/hicolor" >/dev/null 2>&1 || true
fi

if command -v kbuildsycoca6 >/dev/null 2>&1; then
    kbuildsycoca6 >/dev/null 2>&1 || true
fi

echo
echo "ArchGraph updated."