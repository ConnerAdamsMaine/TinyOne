#!/usr/bin/env bash
# _common.sh -- sourced by compile_* leaves and consumer dispatchers
# Single source for ROOT/TARGET/BUILD and PKGBUILD-style common flag sets.

set -euo pipefail

export t=$'\t'

_usage() {
  local _self
  _self="$(basename "${BASH_SOURCE[0]:-$0}")"
  printf "%s\n" "$(cat <<EOF
Usage: source ${_self}  # or: source scripts/_common.sh

Sourced helper -- not executed directly.
Sets: ROOT_DIR, TARGET_DIR, BUILD_DIR, _common_gcc_flags, _common_msvc_flags, _rustc_base_args
For LLM: source this file, do not execute. No args expected.

Example:
${t}source \${BASH_SOURCE[0]}
EOF
)"
}

case "${1:-}" in
  -h|--help|help) _usage; exit 0 ;;
esac

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="${CARGO_TARGET_DIR:-${ROOT_DIR}/target}"
BUILD_DIR="${TARGET_DIR}/debug"

mkdir -p "${BUILD_DIR}"

# Single common sets -- only -std / extension vary per-file below
_common_gcc_flags=(
  -Wall
  -Wextra
  -Werror
  -I"${ROOT_DIR}"
)

_common_msvc_flags=(
  /nologo
  /W4
  /WX
  /I "${ROOT_DIR}"
)

_rustc_base_args=(
  --edition=2024
  --emit=metadata
)
