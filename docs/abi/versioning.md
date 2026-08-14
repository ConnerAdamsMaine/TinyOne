# ABI Versioning and Stability

**Current ABI status: STABLE (version 1).** Callers should compare
`tinyone_abi_version()` with `TINYONE_ABI_VERSION` before using the interface.

## What Constitutes a Breaking Change

The following changes break binary or source compatibility for callers:

**Function-level breaks:**
- Removing or renaming an entry point declared in `tinylang.h`
- Changing the type or order of any parameter
- Changing the return type of any entry point

**Response-level breaks:**
- Removing a key from a success `value` object
- Changing the type of an existing key in any response shape
- Removing one of the four envelope shapes (`ok/value`, `compile`,
  `runtime`, `panic`)
- Changing the meaning of `"kind"` values

**Bytecode-level breaks:**
- Reordering or removing any opcode in `Op` ordinal positions 1–29
- Reordering or removing any Phase-1 builtin in slots 0–34 of `BUILTINS`
- Changing the JSON artifact `"format"` or `"version"` field values

## What Is Not a Breaking Change

- Adding new entry points to `tinylang.h`
- Adding new opcode ordinals above the frozen Phase-1 range
- Adding new Phase-2 builtin slots above index 34
- Changing internal implementation details with no observable effect on
  inputs or outputs
- Changing error message text within the `"error"` field (do not parse
  error strings)

## Current Stability Status

| Area | Status | Notes |
| --- | --- | --- |
| Function signatures in `tinylang.h` | STABLE | Frozen for ABI version 1 |
| Response envelope shape (4 kinds) | STABLE | Frozen now |
| `value` object keys per endpoint | STABLE | Frozen by `tinyone-response-schema.json` |
| `memory` array encoding | STABLE | Frozen by `tinyone-response-schema.json` |
| Phase-1 opcode ordinals (1–29) | STABLE | Frozen; artifact round-trips depend on them |
| Phase-2 opcode ordinals (30+) | STABLE | Frozen for the v1 artifact format |
| Phase-1 builtin slots (0–34) | STABLE | Frozen |
| Phase-2 builtin slots (35+) | STABLE | Frozen for the v1 release line |
| Artifact `format`/`version` fields | STABLE | `"tinyone-bytecode"` / `1` |

## v1 Stability Declaration

The following surfaces are stable and will not change without a major version
bump or, for the C boundary, a new ABI version:

1. All function signatures in `tinylang.h`
2. All four response envelope shapes
3. All `value` object keys for every entry point
4. The `memory` array encoding
5. Phase-1 opcode ordinals, Phase-2 opcode ordinals, and Phase-2 builtin slot order

The [v1 roadmap](../v1-roadmap.md) items 1–5 were resolved by:

- **Item 1:** JSON response schema audit and contract tests committed
- **Item 2:** `Program` field visibility scoped to `pub(crate)`
- **Item 3:** `VerifiedProgram` adopted on all execution paths
- **Item 4:** `tinyone_free_string` wrapped in `catch_unwind`
- **Item 5:** Void `extern "C"` entry point policy decided and documented

## Decay Policy

After v1 is declared, deprecated features will be marked in `tinylang.h`
with a `// DEPRECATED(vX.Y): reason` comment and kept for at least one
minor version cycle before removal. Removals require a major version
bump.
