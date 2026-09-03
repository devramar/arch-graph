#!/usr/bin/env bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

require_command cargo
require_command npm

cd "$ROOT_DIR"
cargo test --workspace

if [[ ! -d "$DESKTOP_DIR/node_modules" ]]; then
    echo "Skipping frontend build: apps/desktop/node_modules is missing." >&2
    exit 0
fi

npm --prefix "$DESKTOP_DIR" run build
