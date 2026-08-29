#!/usr/bin/env bash
set -euo pipefail

# Windows MSVC C smoke: build cdylib + link tinylang_consumer.c via cl.exe against .dll.lib
# Requires setup-msvc (msvc-dev-cmd + PATH filtered). Fail hard; no fallback.
# Candidate-chain dispatch (array + centralized switch -> helper) for toolchain probing.

export t=$'\t'

_usage() {
  local _self
  _self="$(basename "${BASH_SOURCE[0]:-$0}")"
  printf "%s\n" "$(cat <<EOF
Usage: ${_self} [help|-h|--help]
  or: bash ${BASH_SOURCE[0]} [help|-h|--help]

Builds cdylib (cargo build), links tests/consumers/tinylang_consumer.c via cl.exe
against the cdylib import lib (*.dll.lib), and executes the binary.
Requires: cl.exe on PATH (setup-msvc). No arguments expected.

Examples:
${t}bash ${BASH_SOURCE[0]}
${t}bash ${BASH_SOURCE[0]} --help

For LLM: no args required. Missing cl.exe is explicit failure; no fallback.
EOF
)"
}

case "${1:-}" in
  -h|--help|help) _usage; exit 0 ;;
esac

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="${CARGO_TARGET_DIR:-${ROOT_DIR}/target}"
BUILD_DIR="${TARGET_DIR}/debug"

cargo build

# Find import lib: tinylang.dll.lib or tinyone.dll.lib depending on crate name
IMPORT_LIB="$(ls -1 "${BUILD_DIR}"/*.dll.lib 2>/dev/null | head -n 1 || true)"
if [[ -z "${IMPORT_LIB}" ]]; then
  printf "consumer_smoke_msvc: import library *.dll.lib not found in %s\n" "${BUILD_DIR}" >&2
  ls -la "${BUILD_DIR}" 2>&1 | head -n 30 >&2 || true
  exit 1
fi

OUT="${BUILD_DIR}/tinylang_consumer.exe"
SRC="${ROOT_DIR}/tests/consumers/tinylang_consumer.c"

_base_msvc_args=(
  /nologo
  /std:c11
  /W4
  /WX
  /I "${ROOT_DIR}"
)

_link_msvc() {
  printf "Linking %s + %s -> %s (cl.exe)\n" "${SRC}" "${IMPORT_LIB}" "${OUT}"
  _msvc_link_args=(
    "${_base_msvc_args[@]}"
    "${SRC}"
    "${IMPORT_LIB}"
    /link
    "/OUT:${OUT}"
  )

  cl.exe "${_msvc_link_args[@]}"
  "${OUT}"
}

# Candidate chains: probe bin -> helper
_candidates=(
  "cl.exe:_link_msvc"
)

_dispatched=0
for _entry in "${_candidates[@]}"; do
  IFS=":" read -r _bin _helper <<< "${_entry}"
  if command -v "${_bin}" >/dev/null 2>&1; then
    "${_helper}"
    _dispatched=1
    break
  fi
done

if [[ "${_dispatched}" -eq 0 ]]; then
  printf "consumer_smoke_msvc: required toolchain not found (need: cl.exe; setup-msvc missing?)\n" >&2
  _usage >&2
  exit 1
fi
