# Fixtures

Public formats only. Do not commit proprietary CAD.

Expect JSON must include `ok`, `fixture`, `format`, `counts` (`solids`, `instances`), and `units` per [`../schema/report.schema.json`](../schema/report.schema.json).

| Id | Input | Expect | v0 |
|---|---|---|---|
| `dxf-rect` | `dxf/rect.dxf` | `dxf/rect.json` | In-tree. Closed LWPOLYLINE rectangle, millimetres. |
| `step-two-instances` | generate `step/two-instances.step` | `step/two-instances.json` | Expect only until OCCT/CadQuery generate the STEP. |
| `ifc-beam-column` | generate `ifc/beam-column.ifc` | `ifc/beam-column.json` | Expect only until IfcOpenShell generates the IFC. Parse with IfcOpenShell; compare with IfcDiff, not a native-model round-trip. |
| `ipc2581-two-layer` | generate `pcb/two-layer.xml` | `pcb/two-layer.json` | Expect only; prefer `kicad-cli pcb export ipc2581`. |

## Generate later (not required for `cargo test`)

```
# STEP: CadQuery or OCCT — two solid instances, millimetres
# IFC: IfcOpenShell — IfcBeam + IfcColumn + connection + property set; compare with python -m ifcdiff
# IPC-2581: kicad-cli pcb export ipc2581 fixture.kicad_pcb
```
