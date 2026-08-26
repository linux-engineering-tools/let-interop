use serde_json::{json, Map, Value};
use std::fs;
use std::io;
use std::path::Path;

pub const EXIT_OK: i32 = 0;
pub const EXIT_INPUT: i32 = 2;
pub const EXIT_DRIFT: i32 = 3;
pub const EXIT_HOST: i32 = 4;
pub const EXIT_USAGE: i32 = 64;

pub const HELP: &str = "\
let-interop — open-format round-trip harness (not a CAD)

Usage:
  let-interop --help
  let-interop check --in FILE --expect FILE [--report FILE]
  let-interop roundtrip --dry-run --in FILE --expect FILE [--report FILE]

Exit: 0 pass, 2 input/schema, 3 drift, 4 host/kernel, 64 usage
(--help exits 0)

v0: DXF entity counts in-tree. STEP/IFC/IPC-2581 dry-run checks
file shape + expect JSON only. No FreeCAD/KiCad/OCCT subprocess.
";

#[derive(Debug)]
pub struct Outcome {
    pub exit: i32,
    pub report: Value,
}

pub fn check(input: &Path, expect_path: &Path, dry_run: bool) -> Outcome {
    let expect = match load_expect(expect_path) {
        Ok(v) => v,
        Err(o) => return o,
    };
    let input_text = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(e) => {
            return input_error(
                expect.get("fixture").and_then(|x| x.as_str()).unwrap_or(""),
                format!("cannot read {}: {e}", input.display()),
            )
        }
    };
    let format = expect
        .get("format")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();

    match format.as_str() {
        "dxf" => check_dxf(&input_text, expect, dry_run),
        "step-ap242" | "step-ap214" => check_stepish(&input_text, expect, dry_run, "ISO-10303-21"),
        "ifc" => check_stepish(&input_text, expect, dry_run, "ISO-10303-21"),
        "ipc-2581" => check_ipc2581(&input_text, expect, dry_run),
        other => input_error(
            expect.get("fixture").and_then(|x| x.as_str()).unwrap_or(""),
            format!("unknown format {other}"),
        ),
    }
}

pub fn host_not_implemented() -> Outcome {
    Outcome {
        exit: EXIT_HOST,
        report: json!({
            "ok": false,
            "fixture": "",
            "format": "dxf",
            "units": "millimetre",
            "counts": { "solids": 0, "instances": 0 },
            "error": {
                "code": "host-not-implemented",
                "message": "kernel round-trip is not implemented; use --dry-run"
            }
        }),
    }
}

fn load_expect(path: &Path) -> Result<Value, Outcome> {
    let text = fs::read_to_string(path).map_err(|e| {
        input_error("", format!("cannot read expect {}: {e}", path.display()))
    })?;
    let v: Value = serde_json::from_str(&text).map_err(|e| {
        input_error("", format!("expect JSON: {e}"))
    })?;
    for key in ["ok", "fixture", "format", "counts", "units"] {
        if v.get(key).is_none() {
            return Err(input_error("", format!("expect missing {key}")));
        }
    }
    let counts = v.get("counts").and_then(|c| c.as_object()).ok_or_else(|| {
        input_error("", "expect counts must be an object".into())
    })?;
    if !counts.contains_key("solids") || !counts.contains_key("instances") {
        return Err(input_error("", "expect counts needs solids and instances".into()));
    }
    Ok(v)
}

fn input_error(fixture: impl AsRef<str>, message: String) -> Outcome {
    let fixture = fixture.as_ref();
    Outcome {
        exit: EXIT_INPUT,
        report: json!({
            "ok": false,
            "fixture": fixture,
            "format": "dxf",
            "units": "millimetre",
            "counts": { "solids": 0, "instances": 0 },
            "error": { "code": "input", "message": message }
        }),
    }
}

fn check_dxf(text: &str, expect: Value, dry_run: bool) -> Outcome {
    let got = match count_dxf_entities(text) {
        Ok(n) => n,
        Err(e) => return input_error(fixture_name(&expect), e),
    };
    let want = expect
        .pointer("/counts/dxf_entities")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let tol = expect
        .pointer("/tolerance/count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let drift = got.abs_diff(want) > tol;
    finish(expect, dry_run, json!({ "dxf_entities": got }), drift, "dxf_entities")
}

fn check_stepish(text: &str, expect: Value, dry_run: bool, magic: &str) -> Outcome {
    if !text.contains(magic) {
        return input_error(
            fixture_name(&expect),
            format!("input does not contain {magic}"),
        );
    }
    // v0: no kernel; counts stay as expected.
    let mut extra = Map::new();
    extra.insert("solids".into(), expect.pointer("/counts/solids").cloned().unwrap_or(json!(0)));
    extra.insert(
        "instances".into(),
        expect.pointer("/counts/instances").cloned().unwrap_or(json!(0)),
    );
    finish(expect, dry_run, Value::Object(extra), false, "")
}

fn check_ipc2581(text: &str, expect: Value, dry_run: bool) -> Outcome {
    let lower = text.to_ascii_lowercase();
    if !lower.contains("<ipc-2581") && !lower.contains("ipc-2581") {
        return input_error(fixture_name(&expect), "input does not look like IPC-2581 XML".into());
    }
    finish(expect, dry_run, json!({}), false, "")
}

fn finish(mut expect: Value, dry_run: bool, counted: Value, drift: bool, path: &str) -> Outcome {
    if let Some(obj) = expect.as_object_mut() {
        obj.insert("command".into(), json!(if dry_run { "roundtrip" } else { "check" }));
        obj.insert("dry_run".into(), json!(dry_run));
        if let Some(counts) = obj.get_mut("counts").and_then(|c| c.as_object_mut()) {
            if let Some(m) = counted.as_object() {
                for (k, v) in m {
                    counts.insert(k.clone(), v.clone());
                }
            }
        }
        if drift {
            obj.insert("ok".into(), json!(false));
            obj.insert(
                "error".into(),
                json!({
                    "code": "drift",
                    "message": "count outside tolerance",
                    "path": path
                }),
            );
        } else {
            obj.insert("ok".into(), json!(true));
        }
    }
    Outcome {
        exit: if drift { EXIT_DRIFT } else { EXIT_OK },
        report: expect,
    }
}

fn fixture_name(expect: &Value) -> String {
    expect
        .get("fixture")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Count DXF entities in the ENTITIES section (group 0 names other than ENDSEC).
pub fn count_dxf_entities(text: &str) -> Result<u64, String> {
    let lines: Vec<&str> = text.lines().map(|l| l.trim()).collect();
    let mut i = 0;
    let mut in_entities = false;
    let mut count = 0u64;
    while i + 1 < lines.len() {
        let code = lines[i];
        let val = lines[i + 1];
        i += 2;
        if code == "0" && val == "SECTION" {
            continue;
        }
        if code == "2" && val == "ENTITIES" {
            in_entities = true;
            continue;
        }
        if !in_entities {
            continue;
        }
        if code == "0" && val == "ENDSEC" {
            in_entities = false;
            continue;
        }
        if code == "0" {
            count += 1;
        }
    }
    if count == 0 && !text.to_ascii_uppercase().contains("ENTITIES") {
        return Err("DXF has no ENTITIES section".into());
    }
    Ok(count)
}

pub fn write_report(path: Option<&Path>, report: &Value) -> io::Result<()> {
    let s = serde_json::to_string_pretty(report)? + "\n";
    print!("{s}");
    if let Some(p) = path {
        fs::write(p, s)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn dxf_rect_counts_one_entity() {
        let p = root().join("fixtures/dxf/rect.dxf");
        let text = fs::read_to_string(p).unwrap();
        assert_eq!(count_dxf_entities(&text).unwrap(), 1);
    }

    #[test]
    fn check_dxf_rect_ok() {
        let o = check(
            &root().join("fixtures/dxf/rect.dxf"),
            &root().join("fixtures/dxf/rect.json"),
            true,
        );
        assert_eq!(o.exit, EXIT_OK);
        assert_eq!(o.report["ok"], true);
        assert_eq!(o.report["dry_run"], true);
        assert_eq!(o.report["counts"]["dxf_entities"], 1);
    }

    #[test]
    fn missing_input_is_2() {
        let o = check(
            Path::new("/no/such/file.dxf"),
            &root().join("fixtures/dxf/rect.json"),
            false,
        );
        assert_eq!(o.exit, EXIT_INPUT);
    }

    #[test]
    fn bad_expect_is_2() {
        let o = load_expect(Path::new("/no/such.json"));
        assert!(o.is_err());
        assert_eq!(o.err().unwrap().exit, EXIT_INPUT);
    }
}
