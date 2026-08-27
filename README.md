# let-interop

Open-format test harness (STEP/DXF/IPC-2581 round-trip; IFC parse + IfcDiff). Not computer-aided design (CAD). Not a translator clone.

An engineer (or continuous integration (CI)) on Linux proves that a tool chain can handle published geometry and electronics formats without silent data loss. For STEP, Drawing Exchange Format (DXF), and IPC-2581, **round-trip** means: write the file out, read it back, and check counts, units, and bounds. For Industry Foundation Classes (IFC), the harness **parses** with IfcOpenShell and **compares** two files with [IfcDiff](https://docs.ifcopenshell.org/ifcdiff.html). IfcOpenShell has no non-IFC native model, so this is not a kernel import/export. Geometry rewrite, if needed, is Bonsai.

Organization terms: [community TERMS.md](https://github.com/linux-engineering-tools/community/blob/main/TERMS.md). Spell a term out on first use in a document, then use the short form.

| | |
|---|---|
| Spec | [community `specs/let-interop`](https://github.com/linux-engineering-tools/community/blob/main/specs/let-interop/README.md) |
| RFC | [community#21](https://github.com/linux-engineering-tools/community/issues/21) |
| Requirements | [#8](https://github.com/linux-engineering-tools/community/issues/8), [#14](https://github.com/linux-engineering-tools/community/issues/14), [#15](https://github.com/linux-engineering-tools/community/issues/15) |
| License | Apache-2.0 |
| GUI | None |

This repository is **docs-first incubation**: a command-line interface (CLI) contract, JavaScript Object Notation (JSON) schema, fixtures (in-tree sample files), and a stub that `--dry-run`s without calling FreeCAD, KiCad, or a geometry kernel. Kernels stay **upstream** (Open CASCADE Technology, IfcOpenShell, KiCad). New **capabilities** still go to [community](https://github.com/linux-engineering-tools/community) issues. This repo’s issues are for **bugs in this harness**.

Binary name is `let-interop`, never `let` (shell builtin).

## Non-goals

- Reimplementing Open CASCADE Technology (OCCT), IfcOpenShell, or KiCad
- Editing vendor binaries (`.rvt`, `.pln`, `.adb`, `.nxasm`)
- A graphical user interface (GUI)

## Build / test

```
cargo build
cargo test
./target/debug/let-interop --help
./target/debug/let-interop check --in fixtures/dxf/rect.dxf --expect fixtures/dxf/rect.json
./target/debug/let-interop roundtrip --dry-run --in fixtures/dxf/rect.dxf --expect fixtures/dxf/rect.json --report /tmp/report.json
```

GitHub Actions was not added in the first push (the token lacks `workflow` scope). Add `.github/workflows/ci.yml` later: `cargo test`, `cargo build`, `--help`, and the DXF `check` command above.

v0 `--dry-run` / `check` on Drawing Exchange Format (DXF) counts entities in-tree. ISO 10303 STEP, Industry Foundation Classes (IFC), and IPC-2581 (printed circuit board manufacturing interchange) dry-run only checks that the input file looks like the format and that the expect JSON matches [`schema/report.schema.json`](schema/report.schema.json) required fields. Kernel round-trip and IfcDiff are not implemented yet.

## Command-line interface (CLI)

See [`docs/cli.md`](docs/cli.md). Exit codes: 0 pass, 2 input/schema, 3 drift, 4 host/kernel, 64 usage (`--help` is 0).

## Human accountable

Joseph Woolley / [@calledtoconstruct](https://github.com/calledtoconstruct)
