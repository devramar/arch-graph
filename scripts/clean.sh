#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"

DESKTOP="$PROJECT_ROOT/apps/desktop"

CLEAN_DEPS=false

for arg in "$@"; do
    case "$arg" in
        --deps)
            CLEAN_DEPS=true
            ;;

        -h|--help)
            cat <<EOF
Usage:
  ./scripts/clean.sh
  ./scripts/clean.sh --deps

Removes generated ArchGraph build artifacts.

Default:
  - Rust target/
  - frontend dist/
  - TypeScript *.tsbuildinfo
  - generated Tauri schemas

--deps:
  Also removes frontend node_modules/.
EOF
            exit 0
            ;;

        *)
            echo "Unknown argument: $arg"
            echo "Use --help for usage."
            exit 1
            ;;
    esac
done

echo "Cleaning ArchGraph..."

# Rust / Tauri build output
if [[ -d "$PROJECT_ROOT/target" ]]; then
    echo "  target/"
    rm -rf "$PROJECT_ROOT/target"
fi

# Vite output
if [[ -d "$DESKTOP/dist" ]]; then
    echo "  apps/desktop/dist/"
    rm -rf "$DESKTOP/dist"
fi

# TypeScript incremental build state
while IFS= read -r -d '' file; do
    echo "  ${file#"$PROJECT_ROOT/"}"
    rm -f "$file"
done < <(
    find "$PROJECT_ROOT" \
        -type f \
        -name '*.tsbuildinfo' \
        -not -path '*/node_modules/*' \
        -print0
)

# Tauri-generated schemas
if [[ -d "$DESKTOP/src-tauri/gen" ]]; then
    echo "  apps/desktop/src-tauri/gen/"
    rm -rf "$DESKTOP/src-tauri/gen"
fi

# Optional dependency reset
if [[ "$CLEAN_DEPS" == true && -d "$DESKTOP/node_modules" ]]; then
    echo "  apps/desktop/node_modules/"
    rm -rf "$DESKTOP/node_modules"
fi

echo
echo "Clean complete."

if [[ "$CLEAN_DEPS" == true ]]; then
    echo "Frontend dependencies were removed."
    echo "Run: cd apps/desktop && npm install"
fi