#!/usr/bin/env bash
set -euo pipefail
source "$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/_common.sh"

require_command cargo
cd "$ROOT_DIR"
cargo build --release -p archgraph-core -p archgraph-cli "$@"
