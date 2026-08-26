use let_interop::{check, host_not_implemented, write_report, EXIT_INPUT, EXIT_USAGE, HELP};
use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{HELP}");
        process::exit(0);
    }

    let mut cmd = "";
    let mut input: Option<PathBuf> = None;
    let mut expect: Option<PathBuf> = None;
    let mut report: Option<PathBuf> = None;
    let mut dry_run = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "check" | "roundtrip" => {
                cmd = args[i].as_str();
                i += 1;
            }
            "--in" => {
                input = args.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--expect" => {
                expect = args.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--report" => {
                report = args.get(i + 1).map(PathBuf::from);
                i += 2;
            }
            "--out" => {
                i += 2; // reserved; v0 ignores
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            other => {
                eprintln!("unknown argument: {other}\n");
                eprint!("{HELP}");
                process::exit(EXIT_USAGE);
            }
        }
    }

    if cmd.is_empty() || input.is_none() || expect.is_none() {
        eprint!("{HELP}");
        process::exit(EXIT_USAGE);
    }

    let outcome = if cmd == "roundtrip" && !dry_run {
        host_not_implemented()
    } else {
        check(
            input.as_ref().unwrap(),
            expect.as_ref().unwrap(),
            cmd == "roundtrip" && dry_run,
        )
    };

    if let Err(e) = write_report(report.as_deref(), &outcome.report) {
        eprintln!("report: {e}");
        process::exit(EXIT_INPUT);
    }
    process::exit(outcome.exit);
}
