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
mod ops;
mod preflight;
use i18n::I18n;
use ops::run_namespace;
use preflight::{run_preflight, PreflightOptions};

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
        "config" | "repo" | "service" | "mount" | "dashboard" | "cloud" | "compress" | "export"
        | "convert" | "version" | "diag" => cmd_namespace(&cmd, &rest),
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

fn cmd_namespace(namespace: &str, args: &[String]) -> ! {
    let outcome = run_namespace(namespace, args);
    if let Some(out) = outcome.stdout {
        println!("{out}");
    }
    if let Some(err) = outcome.stderr {
        eprintln!("{err}");
    }
    exit(outcome.exit_code);
}
