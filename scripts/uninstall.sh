#!/usr/bin/env bash
set -euo pipefail

BIN="$HOME/.local/bin/archgraph"
DESKTOP="$HOME/.local/share/applications/archgraph.desktop"
ICON="$HOME/.local/share/icons/hicolor/256x256/apps/archgraph.png"

echo "Removing ArchGraph..."

rm -f "$BIN"
rm -f "$DESKTOP"
rm -f "$ICON"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$HOME/.local/share/applications" >/dev/null 2>&1 || true
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache "$HOME/.local/share/icons/hicolor" >/dev/null 2>&1 || true
fi

if command -v kbuildsycoca6 >/dev/null 2>&1; then
    kbuildsycoca6 >/dev/null 2>&1 || true
fi

echo
echo "ArchGraph removed."