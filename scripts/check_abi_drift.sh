#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
exec uv run --no-project python "$ROOT/tools/abi_manifest.py" check "$@"
