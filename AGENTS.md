# Agent instructions — linux-engineering-tools/let-interop

Read [community AGENTS.md](https://github.com/linux-engineering-tools/community/blob/main/AGENTS.md) first. This repo is the **harness**, not the requirements board. Spell out a term on first use, or link [community TERMS.md](https://github.com/linux-engineering-tools/community/blob/main/TERMS.md).

## What this repo is

CLI + fixtures to round-trip published formats (STEP, IFC, DXF, IPC-2581) and report JSON. Not a CAD. Kernels stay upstream.

## Hard rules

1. Capability language. No clone of a named commercial translator.
2. No proprietary IP (source, binaries, leaked docs, vendor CAD as fixtures).
3. Do not reimplement OCCT, IfcOpenShell, or KiCad here.
4. No GUI in v0. If a GUI is proposed later, the Omarchy contract in community applies.
5. DCO on every commit: `Signed-off-by: Full Name <email>`.
6. Binary is `let-interop`, never `let`.
7. New capabilities: file in [community](https://github.com/linux-engineering-tools/community). Issues here are harness bugs only.
8. Human accountable on agent-authored work: name them.

## Prove a change

```
cargo test
cargo build
./target/debug/let-interop --help
./target/debug/let-interop check --in fixtures/dxf/rect.dxf --expect fixtures/dxf/rect.json
```
