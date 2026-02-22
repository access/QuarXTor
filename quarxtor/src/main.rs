use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use quarxdrive::fs_import::{import_host_dir, ImportStats};
use quarxdrive::fs_root::{ensure_root_dir, set_root_dir_id};
use quarxdrive::{DirEntry, QuarxRepo};
use quarxtor_core::config::QuarxConfig;
use quarxtor_core::store::file_store::FileBlockStore;

mod i18n;
mod preflight;
use i18n::I18n;
use preflight::{run_preflight, PreflightOptions};

const PLANNED_NAMESPACES: &[(&str, &[&str])] = &[
    ("config", &["show", "effective", "explain", "set", "lint"]),
    ("repo", &["init", "verify", "doctor", "recover", "stats"]),
    ("service", &["start", "stop", "restart", "status", "logs"]),
    (
        "mount",
        &["create", "attach", "mount", "unmount", "list", "info"],
    ),
    ("dashboard", &["serve", "status", "snapshot"]),
    (
        "cloud",
        &["cluster", "peer", "discover", "sync", "policy", "stats"],
    ),
    (
        "compress",
        &["analyze", "plan", "apply", "rollback", "history", "tune"],
    ),
    ("export", &["snapshot", "bundle", "manifest"]),
    ("convert", &["to-disk", "to-nondisk", "verify"]),
    ("version", &["show", "check-compat"]),
    ("diag", &["preflight", "trace", "report"]),
];

fn main() {
    let i18n = I18n::load();
    let mut args: Vec<String> = env::args().collect();
    let bin = args
        .get(0)
        .cloned()
        .unwrap_or_else(|| "quarxtor".to_string());

    if args.len() <= 1 {
        usage(&i18n, &bin, false);
    }

    let cmd = args[1].clone();
    if cmd == "--help" || cmd == "-h" {
        usage(&i18n, &bin, true);
    }
    if cmd == "--version" || cmd == "-V" {
        println!("quarxtor {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let rest = args.split_off(2);
    match cmd.as_str() {
        "import" => cmd_import(&i18n, &bin, rest),
        "status" => cmd_status(&i18n, &bin, rest),
        "config" | "repo" | "service" | "mount" | "dashboard" | "cloud" | "compress"
        | "export" | "convert" | "version" | "diag" => cmd_planned_namespace(&cmd, &rest),
        _ => {
            eprintln!("{}: {}", i18n.t("cli.error.unknown_command", &[]), cmd);
            usage(&i18n, &bin, false);
        }
    }
}

fn usage(i18n: &I18n, bin: &str, success: bool) -> ! {
    let text = i18n.t("cli.usage.main", &[("bin", bin)]);
    if success {
        println!("{text}");
        exit(0);
    }
    eprintln!("{text}");
    exit(1);
}

fn cmd_import(i18n: &I18n, bin: &str, args: Vec<String>) {
    let (repo_override, rest) = parse_repo_arg(i18n, bin, &args);
    if rest.len() != 1 {
        eprintln!("{}: import", i18n.t("cli.error.missing_argument", &[]));
        usage(i18n, bin, false);
    }

    let source = PathBuf::from(&rest[0]);
    let preflight = run_preflight(PreflightOptions {
        repo_override,
        import_root_override: Some(source.clone()),
    });
    if !preflight.report.required_ok() {
        preflight.report.print_human();
        if !preflight.report.missing_required_only_repo() {
            exit(1);
        }
    }
    let meta = match fs::metadata(&source) {
        Ok(m) => m,
        Err(e) => {
            eprintln!(
                "{}: {} ({e})",
                i18n.t("cli.error.invalid_path", &[]),
                source.display()
            );
            exit(1);
        }
    };

    let mut cfg = QuarxConfig::load_default();
    cfg.import_root = Some(source.to_string_lossy().to_string());

    let repo_path = preflight.repo_path;

    let store = match FileBlockStore::open(repo_path.clone()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {e:?}", i18n.t("cli.error.open_repo", &[]));
            exit(1);
        }
    };
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);

    if meta.is_dir() {
        let (root_id, stats) = match import_host_dir(&cfg, &repo_path, &mut repo, &source, false) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {e}", i18n.t("cli.error.import_failed", &[]));
                exit(1);
            }
        };
        print_import_stats(&repo_path, &source, root_id, "dir", &stats);
        return;
    }

    if meta.is_file() {
        let data = match fs::read(&source) {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "{}: {} ({e})",
                    i18n.t("cli.error.invalid_path", &[]),
                    source.display()
                );
                exit(1);
            }
        };

        let name = match source.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => {
                eprintln!(
                    "{}: {}",
                    i18n.t("cli.error.invalid_path", &[]),
                    source.display()
                );
                exit(1);
            }
        };

        let root_id = match ensure_root_dir(&repo_path, &mut repo) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}: {e}", i18n.t("cli.error.import_failed", &[]));
                exit(1);
            }
        };

        let obj_id = match repo.put_bytes_simple(&data) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}: {e:?}", i18n.t("cli.error.import_failed", &[]));
                exit(1);
            }
        };

        let mut entries = match repo.read_dir(root_id) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {e:?}", i18n.t("cli.error.import_failed", &[]));
                exit(1);
            }
        };

        let mut updated = false;
        for e in &mut entries {
            if e.name == name {
                e.target = obj_id;
                updated = true;
                break;
            }
        }
        if !updated {
            entries.push(DirEntry {
                name,
                target: obj_id,
            });
        }

        let new_root_id = match repo.put_dir(&entries) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}: {e:?}", i18n.t("cli.error.import_failed", &[]));
                exit(1);
            }
        };

        if let Err(e) = set_root_dir_id(&repo_path, &mut repo, new_root_id) {
            eprintln!("{}: {e}", i18n.t("cli.error.import_failed", &[]));
            exit(1);
        }

        let mut stats = ImportStats::default();
        stats.files = 1;
        print_import_stats(&repo_path, &source, new_root_id, "file", &stats);
        return;
    }

    eprintln!(
        "{}: {}",
        i18n.t("cli.error.invalid_path", &[]),
        source.display()
    );
    exit(1);
}

fn cmd_status(i18n: &I18n, bin: &str, args: Vec<String>) {
    let (repo_override, json, rest) = parse_status_args(i18n, bin, &args);
    if !rest.is_empty() {
        eprintln!("{}: status", i18n.t("cli.error.missing_argument", &[]));
        usage(i18n, bin, false);
    }

    let preflight = run_preflight(PreflightOptions {
        repo_override,
        import_root_override: None,
    });

    if json {
        println!("{}", preflight.report.to_json());
    } else {
        preflight.report.print_human();
    }

    if preflight.report.required_ok() || preflight.report.missing_required_only_repo() {
        exit(0);
    }
    exit(1);
}

fn parse_repo_arg(i18n: &I18n, bin: &str, args: &[String]) -> (Option<PathBuf>, Vec<String>) {
    let mut repo: Option<PathBuf> = None;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let a = &args[i];
        if a == "--repo" {
            if i + 1 >= args.len() {
                eprintln!("{}: --repo", i18n.t("cli.error.missing_argument", &[]));
                usage(i18n, bin, false);
            }
            repo = Some(PathBuf::from(&args[i + 1]));
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--repo=") {
            repo = Some(PathBuf::from(v));
            i += 1;
            continue;
        }
        if a == "--help" || a == "-h" {
            usage(i18n, bin, true);
        }
        rest.push(a.clone());
        i += 1;
    }

    (repo, rest)
}

fn parse_status_args(
    i18n: &I18n,
    bin: &str,
    args: &[String],
) -> (Option<PathBuf>, bool, Vec<String>) {
    let mut repo: Option<PathBuf> = None;
    let mut json = false;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let a = &args[i];
        if a == "--repo" {
            if i + 1 >= args.len() {
                eprintln!("{}: --repo", i18n.t("cli.error.missing_argument", &[]));
                usage(i18n, bin, false);
            }
            repo = Some(PathBuf::from(&args[i + 1]));
            i += 2;
            continue;
        }
        if let Some(v) = a.strip_prefix("--repo=") {
            repo = Some(PathBuf::from(v));
            i += 1;
            continue;
        }
        if a == "--json" {
            json = true;
            i += 1;
            continue;
        }
        if a == "--help" || a == "-h" {
            usage(i18n, bin, true);
        }
        rest.push(a.clone());
        i += 1;
    }

    (repo, json, rest)
}

fn print_import_stats(
    repo_path: &Path,
    source_path: &Path,
    root_id: u64,
    kind: &str,
    stats: &ImportStats,
) {
    println!("import=ok");
    println!("import_kind={kind}");
    println!("repo_path={}", repo_path.display());
    println!("source_path={}", source_path.display());
    println!("root_id={root_id}");
    println!("files={}", stats.files);
    println!("dirs={}", stats.dirs);
    println!("skipped_zero={}", stats.skipped_zero);
    println!("skipped_symlink={}", stats.skipped_symlink);
    println!("skipped_hidden={}", stats.skipped_hidden);
    println!("skipped_devices={}", stats.skipped_devices);
    println!("skipped_special={}", stats.skipped_special);
    println!("skipped_small={}", stats.skipped_small);
    println!("skipped_other={}", stats.skipped_other);
}

#[derive(Debug, PartialEq, Eq)]
struct PlannedDispatch {
    exit_code: i32,
    stdout: Option<String>,
    stderr: Option<String>,
}

fn cmd_planned_namespace(cmd: &str, args: &[String]) -> ! {
    let dispatch = dispatch_planned_namespace(cmd, args);
    if let Some(out) = dispatch.stdout {
        println!("{out}");
    }
    if let Some(err) = dispatch.stderr {
        eprintln!("{err}");
    }
    exit(dispatch.exit_code);
}

fn dispatch_planned_namespace(cmd: &str, args: &[String]) -> PlannedDispatch {
    let mut json = false;
    let mut help = false;
    let mut subcommand: Option<String> = None;

    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if a == "--help" || a == "-h" {
            help = true;
            i += 1;
            continue;
        }
        if a == "--json" || a == "--output=json" {
            json = true;
            i += 1;
            continue;
        }
        if a == "--output" {
            if i + 1 < args.len() && args[i + 1] == "json" {
                json = true;
                i += 2;
                continue;
            }
            i += 1;
            continue;
        }
        if !a.starts_with('-') && subcommand.is_none() {
            subcommand = Some(a.clone());
        }
        i += 1;
    }

    let allowed = planned_namespace_subcommands(cmd).unwrap_or(&[]);
    if help {
        return PlannedDispatch {
            exit_code: 0,
            stdout: Some(format!(
                "Usage:\n  quarxtor {cmd} <subcommand> [--output json]\n\nAvailable subcommands:\n  {}",
                allowed.join(", ")
            )),
            stderr: None,
        };
    }

    if let Some(ref sub) = subcommand {
        if !allowed.iter().any(|candidate| candidate == sub) {
            return PlannedDispatch {
                exit_code: 1,
                stdout: None,
                stderr: Some(format!(
                    "{cmd}: unknown subcommand '{sub}' (allowed: {})",
                    allowed.join(", ")
                )),
            };
        }
    }

    if json {
        let sub_json = match subcommand.as_deref() {
            Some(sub) => format!("\"{sub}\""),
            None => "null".to_string(),
        };
        let payload = format!(
            "{{\"meta\":{{\"namespace\":\"{}\",\"subcommand\":{},\"state\":\"planned\"}},\"result\":null,\"errors\":[{{\"code\":\"not_implemented\",\"category\":\"state\",\"message\":\"namespace is planned and not implemented yet\"}}],\"timing\":{{\"started_at\":\"n/a\",\"ended_at\":\"n/a\",\"duration_ms\":0}},\"operation_id\":\"planned-{}\"}}",
            cmd, sub_json, cmd
        );
        return PlannedDispatch {
            exit_code: 2,
            stdout: Some(payload),
            stderr: None,
        };
    }

    let suffix = subcommand
        .as_deref()
        .map(|sub| format!(":{sub}"))
        .unwrap_or_default();
    PlannedDispatch {
        exit_code: 2,
        stdout: None,
        stderr: Some(format!(
            "{cmd}{suffix}: namespace is planned and not implemented yet"
        )),
    }
}

fn planned_namespace_subcommands(namespace: &str) -> Option<&'static [&'static str]> {
    for (name, subcommands) in PLANNED_NAMESPACES {
        if *name == namespace {
            return Some(subcommands);
        }
    }
    None
}

#[cfg(test)]
mod planned_dispatch_tests {
    use super::*;

    #[test]
    fn planned_json_response_is_stable() {
        let args = vec!["mount".to_string(), "--output".to_string(), "json".to_string()];
        let dispatch = dispatch_planned_namespace("mount", &args);
        assert_eq!(dispatch.exit_code, 2);
        let out = dispatch.stdout.expect("json payload");
        assert!(out.contains("\"namespace\":\"mount\""));
        assert!(out.contains("\"subcommand\":\"mount\""));
        assert!(out.contains("\"code\":\"not_implemented\""));
    }

    #[test]
    fn unknown_subcommand_is_usage_error() {
        let args = vec!["oops".to_string()];
        let dispatch = dispatch_planned_namespace("cloud", &args);
        assert_eq!(dispatch.exit_code, 1);
        assert!(dispatch.stdout.is_none());
        let err = dispatch.stderr.expect("stderr");
        assert!(err.contains("unknown subcommand"));
    }

    #[test]
    fn help_lists_subcommands() {
        let args = vec!["--help".to_string()];
        let dispatch = dispatch_planned_namespace("compress", &args);
        assert_eq!(dispatch.exit_code, 0);
        let out = dispatch.stdout.expect("help");
        assert!(out.contains("analyze"));
        assert!(out.contains("rollback"));
    }
}
