# let-interop

Open-format **round-trip test harness**. Not a CAD. Not a translator clone.

An engineer (or CI) on Linux proves that a tool chain can import and export published geometry and electronics formats without silent data loss.

| | |
|---|---|
| Spec | [community `specs/let-interop`](https://github.com/linux-engineering-tools/community/blob/main/specs/let-interop/README.md) |
| RFC | [community#21](https://github.com/linux-engineering-tools/community/issues/21) |
| Requirements | [#8](https://github.com/linux-engineering-tools/community/issues/8), [#14](https://github.com/linux-engineering-tools/community/issues/14), [#15](https://github.com/linux-engineering-tools/community/issues/15) |
| License | Apache-2.0 |
| GUI | None |

This repository is **docs-first incubation**: CLI contract, JSON schema, fixtures, and a stub that `--dry-run`s without calling FreeCAD, KiCad, or a geometry kernel. Kernels stay upstream (OpenCASCADE, IfcOpenShell, KiCad). New **capabilities** still go to [community](https://github.com/linux-engineering-tools/community) issues. This repo’s issues are for **bugs in this harness**.

Binary name is `let-interop`, never `let` (shell builtin).

## Non-goals

- Reimplementing OCCT, IfcOpenShell, or KiCad
- Editing vendor binaries (`.rvt`, `.pln`, `.adb`, `.nxasm`)
- A GUI

## Build / test

```
cargo build
cargo test
./target/debug/let-interop --help
./target/debug/let-interop check --in fixtures/dxf/rect.dxf --expect fixtures/dxf/rect.json
./target/debug/let-interop roundtrip --dry-run --in fixtures/dxf/rect.dxf --expect fixtures/dxf/rect.json --report /tmp/report.json
```

v0 `--dry-run` / `check` on DXF counts entities in-tree. STEP/IFC/IPC-2581 dry-run only checks that the input file looks like the format and that the expect JSON matches [`schema/report.schema.json`](schema/report.schema.json) required fields. Kernel round-trip is not implemented yet.

## CLI

See [`docs/cli.md`](docs/cli.md). Exit codes: 0 pass, 2 input/schema, 3 drift, 4 host/kernel, 64 usage (`--help` is 0).

## Human accountable

Joseph Woolley / [@calledtoconstruct](https://github.com/calledtoconstruct)
