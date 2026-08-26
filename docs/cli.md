# CLI

```
let-interop --help
let-interop check --in FILE --expect FILE [--report FILE]
let-interop roundtrip --dry-run --in FILE --expect FILE [--report FILE]
```

`--help` / `-h` / no arguments: print this contract, exit 0.

## Commands

**check** — parse `--in`, compare to `--expect` JSON. No kernel subprocess.

**roundtrip --dry-run** — same parse + expect check. Writes a report with `"dry_run": true`. Does not invoke FreeCAD, KiCad, IfcConvert, or OCCT.

**roundtrip** without `--dry-run` is not implemented in v0 (exit 4, JSON `error.code` = `host-not-implemented`).

## Flags

| Flag | Meaning |
|---|---|
| `--in` | Input: `.step`/`.stp`, `.ifc`, `.dxf`, or IPC-2581 `.xml` |
| `--expect` | Expect JSON (required fields as in `schema/report.schema.json`) |
| `--report` | Write the JSON report here as well as stdout |

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Pass |
| 2 | Input missing, unreadable, or schema-invalid |
| 3 | Drift beyond tolerance (counts) |
| 4 | Host/kernel subprocess failed or not implemented |
| 64 | Usage (`--help` is 0) |

Failures print JSON on stdout (and `--report` if set).
