use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use quarxdrive::fs_root::{ensure_root_dir, get_root_dir_id};
use quarxdrive::fs_stats::compute_root_stats;
use quarxdrive::fs_tree::{build_root_tree, FsEntryKind};
use quarxdrive::QuarxRepo;
use quarxtor_core::config::{
    config_contract, config_ini_path, parse_ini_sections, write_full_config, ConfigValueType,
    EffectiveConfig, QuarxConfig,
};
use quarxtor_core::store::file_store::FileBlockStore;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::preflight::{run_preflight, PreflightOptions};

pub const NAMESPACE_SUBCOMMANDS: &[(&str, &[&str])] = &[
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

const EXIT_OK: i32 = 0;
const EXIT_USAGE: i32 = 1;
const EXIT_CONFIG: i32 = 2;
const EXIT_IO: i32 = 3;
const EXIT_COMPAT: i32 = 5;
const EXIT_STATE: i32 = 7;
const EXIT_CONFLICT: i32 = 8;
const EXIT_INTERNAL: i32 = 9;

const DEFAULT_CONFIG_TEXT: &str = "[core]
l0_chunk = 16384
ram.limit = full
analysis.enable_znode = true
analysis.fs_stats_fallback = true

[import]
root = /tmp/quarx-dataset
min_file_size = 4096
skip_hidden = false
skip_symlink = true
skip_zero = true
skip_devices = true
skip_special = true
include_glob = *.so,*.exe,*.iso
exclude_glob =
include_dir_glob =
exclude_dir_glob =
import_glob_mode = file-only
use_z = true
z_threshold = 10

[ui]
lang = en-US

[net]
transport = unix
unix_path = /tmp/quarxtor/quarxnetd.sock
addr = 0.0.0.0:54721
";

#[derive(Debug, Clone)]
pub struct CommandOutcome {
    pub exit_code: i32,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

#[derive(Debug)]
struct CommandContext {
    namespace: String,
    subcommand: String,
    started_at_ms: u64,
    json: bool,
    repo_path: PathBuf,
}

#[derive(Debug)]
struct CommandResult {
    state: &'static str,
    text: String,
    result: Value,
}

#[derive(Debug, Clone)]
struct CmdError {
    exit_code: i32,
    code: &'static str,
    category: &'static str,
    message: String,
}

impl CmdError {
    fn usage(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_USAGE,
            code: "usage_error",
            category: "usage",
            message: message.into(),
        }
    }

    fn config(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_CONFIG,
            code: "config_error",
            category: "config",
            message: message.into(),
        }
    }

    fn io(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_IO,
            code: "io_error",
            category: "io",
            message: message.into(),
        }
    }

    fn compat(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_COMPAT,
            code: "compat_error",
            category: "compat",
            message: message.into(),
        }
    }

    fn state(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_STATE,
            code: "state_error",
            category: "state",
            message: message.into(),
        }
    }

    fn conflict(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_CONFLICT,
            code: "conflict_error",
            category: "conflict",
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            exit_code: EXIT_INTERNAL,
            code: "internal_error",
            category: "internal",
            message: message.into(),
        }
    }
}

#[derive(Debug, Default)]
struct CommonFlags {
    json: bool,
    help: bool,
    repo_override: Option<PathBuf>,
    args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct ServiceState {
    running: bool,
    endpoint: String,
    restart_count: u64,
    last_changed_ms: u64,
    logs: Vec<String>,
}

impl Default for ServiceState {
    fn default() -> Self {
        Self {
            running: false,
            endpoint: "unknown".to_string(),
            restart_count: 0,
            last_changed_ms: 0,
            logs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct MountRecord {
    id: u64,
    name: String,
    mode: String,
    mounted: bool,
    mountpoint: Option<String>,
    snapshot_path: Option<String>,
    created_ms: u64,
    updated_ms: u64,
}

impl Default for MountRecord {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            mode: "disk".to_string(),
            mounted: false,
            mountpoint: None,
            snapshot_path: None,
            created_ms: 0,
            updated_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct DashboardState {
    running: bool,
    bind: String,
    snapshots: Vec<String>,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            running: false,
            bind: "127.0.0.1:17878".to_string(),
            snapshots: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct ClusterRecord {
    cluster_id: String,
    mode: String,
    display_name: String,
    policy_ref: String,
    capabilities: Vec<String>,
    transport_endpoints: Vec<String>,
    federation_peers: Vec<String>,
    last_seen_ms: u64,
    health_state: String,
}

impl Default for ClusterRecord {
    fn default() -> Self {
        Self {
            cluster_id: String::new(),
            mode: "private".to_string(),
            display_name: String::new(),
            policy_ref: "default".to_string(),
            capabilities: vec![
                "mount".to_string(),
                "snapshot-share".to_string(),
                "optimizer-remote".to_string(),
            ],
            transport_endpoints: vec![
                "ipc".to_string(),
                "http".to_string(),
                "grpc".to_string(),
                "quic".to_string(),
            ],
            federation_peers: Vec::new(),
            last_seen_ms: 0,
            health_state: "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct PeerRecord {
    peer_id: String,
    cluster_id: String,
    addr: String,
    last_seen_ms: u64,
}

impl Default for PeerRecord {
    fn default() -> Self {
        Self {
            peer_id: String::new(),
            cluster_id: String::new(),
            addr: String::new(),
            last_seen_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct CloudPolicyState {
    ttl: u64,
    max_fanout: u64,
    rate_limit_per_min: u64,
    allow_mesh_broadcast: bool,
    allow_utp: bool,
}

impl Default for CloudPolicyState {
    fn default() -> Self {
        Self {
            ttl: 3,
            max_fanout: 8,
            rate_limit_per_min: 120,
            allow_mesh_broadcast: true,
            allow_utp: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct SyncRecord {
    id: u64,
    source_cluster: String,
    target_cluster: String,
    meta_hashes: u64,
    block_keys: u64,
    completed_ms: u64,
}

impl Default for SyncRecord {
    fn default() -> Self {
        Self {
            id: 0,
            source_cluster: String::new(),
            target_cluster: String::new(),
            meta_hashes: 0,
            block_keys: 0,
            completed_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct CompressAnalysis {
    bytes_before: u64,
    dirs: u64,
    files: u64,
    blocks: u64,
    estimated_space_saved_bytes: u64,
}

impl Default for CompressAnalysis {
    fn default() -> Self {
        Self {
            bytes_before: 0,
            dirs: 0,
            files: 0,
            blocks: 0,
            estimated_space_saved_bytes: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct CompressPlanRecord {
    id: u64,
    created_ms: u64,
    profile: String,
    bytes_before: u64,
    estimated_space_saved_bytes: u64,
    status: String,
}

impl Default for CompressPlanRecord {
    fn default() -> Self {
        Self {
            id: 0,
            created_ms: 0,
            profile: "warm".to_string(),
            bytes_before: 0,
            estimated_space_saved_bytes: 0,
            status: "planned".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct CompressJobRecord {
    id: u64,
    plan_id: u64,
    created_ms: u64,
    status: String,
    bytes_before: u64,
    bytes_after: u64,
    space_saved_bytes: u64,
    dedup_ratio_before_x1000: u64,
    dedup_ratio_after_x1000: u64,
    compression_ratio_before_x1000: u64,
    compression_ratio_after_x1000: u64,
    cpu_ms: u64,
    io_read_bytes: u64,
    io_write_bytes: u64,
    prediction_error_pct_x100: u64,
}

impl Default for CompressJobRecord {
    fn default() -> Self {
        Self {
            id: 0,
            plan_id: 0,
            created_ms: 0,
            status: "applied".to_string(),
            bytes_before: 0,
            bytes_after: 0,
            space_saved_bytes: 0,
            dedup_ratio_before_x1000: 1000,
            dedup_ratio_after_x1000: 1000,
            compression_ratio_before_x1000: 1000,
            compression_ratio_after_x1000: 1000,
            cpu_ms: 0,
            io_read_bytes: 0,
            io_write_bytes: 0,
            prediction_error_pct_x100: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct CompressState {
    profile: String,
    last_analysis: CompressAnalysis,
    plans: Vec<CompressPlanRecord>,
    jobs: Vec<CompressJobRecord>,
}

impl Default for CompressState {
    fn default() -> Self {
        Self {
            profile: "warm".to_string(),
            last_analysis: CompressAnalysis::default(),
            plans: Vec::new(),
            jobs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct ArtifactRecord {
    kind: String,
    path: String,
    created_ms: u64,
}

impl Default for ArtifactRecord {
    fn default() -> Self {
        Self {
            kind: String::new(),
            path: String::new(),
            created_ms: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct QuarxState {
    schema_version: u64,
    trace_enabled: bool,
    next_mount_id: u64,
    next_sync_id: u64,
    next_plan_id: u64,
    next_job_id: u64,
    service: ServiceState,
    mounts: Vec<MountRecord>,
    dashboard: DashboardState,
    clusters: Vec<ClusterRecord>,
    peers: Vec<PeerRecord>,
    cloud_policy: CloudPolicyState,
    sync_history: Vec<SyncRecord>,
    compress: CompressState,
    artifacts: Vec<ArtifactRecord>,
}

impl Default for QuarxState {
    fn default() -> Self {
        Self {
            schema_version: 1,
            trace_enabled: false,
            next_mount_id: 1,
            next_sync_id: 1,
            next_plan_id: 1,
            next_job_id: 1,
            service: ServiceState::default(),
            mounts: Vec::new(),
            dashboard: DashboardState::default(),
            clusters: Vec::new(),
            peers: Vec::new(),
            cloud_policy: CloudPolicyState::default(),
            sync_history: Vec::new(),
            compress: CompressState::default(),
            artifacts: Vec::new(),
        }
    }
}

pub fn namespace_subcommands(namespace: &str) -> Option<&'static [&'static str]> {
    for (name, subcommands) in NAMESPACE_SUBCOMMANDS {
        if *name == namespace {
            return Some(subcommands);
        }
    }
    None
}

pub fn run_namespace(namespace: &str, args: &[String]) -> CommandOutcome {
    let started_at_ms = now_ms();
    let flags = parse_common_flags(args);
    let allowed = namespace_subcommands(namespace).unwrap_or(&[]);

    if flags.help || flags.args.is_empty() {
        let help = namespace_help(namespace, allowed);
        return CommandOutcome {
            exit_code: EXIT_OK,
            stdout: Some(help),
            stderr: None,
        };
    }

    let subcommand = flags.args[0].clone();
    let sub_args = flags.args[1..].to_vec();
    if !allowed.iter().any(|s| *s == subcommand) {
        return error_outcome(
            &CommandContext {
                namespace: namespace.to_string(),
                subcommand: subcommand.clone(),
                started_at_ms,
                json: flags.json,
                repo_path: resolve_repo_path(flags.repo_override),
            },
            CmdError::usage(format!(
                "{namespace}: unknown subcommand '{subcommand}' (allowed: {})",
                allowed.join(", ")
            )),
        );
    }

    let ctx = CommandContext {
        namespace: namespace.to_string(),
        subcommand,
        started_at_ms,
        json: flags.json,
        repo_path: resolve_repo_path(flags.repo_override),
    };

    let result = match namespace {
        "config" => handle_config(&ctx, &sub_args),
        "repo" => handle_repo(&ctx, &sub_args),
        "service" => handle_service(&ctx, &sub_args),
        "mount" => handle_mount(&ctx, &sub_args),
        "dashboard" => handle_dashboard(&ctx, &sub_args),
        "cloud" => handle_cloud(&ctx, &sub_args),
        "compress" => handle_compress(&ctx, &sub_args),
        "export" => handle_export(&ctx, &sub_args),
        "convert" => handle_convert(&ctx, &sub_args),
        "version" => handle_version(&ctx, &sub_args),
        "diag" => handle_diag(&ctx, &sub_args),
        _ => Err(CmdError::internal(format!(
            "unsupported namespace {namespace}"
        ))),
    };

    match result {
        Ok(ok) => success_outcome(&ctx, ok),
        Err(err) => error_outcome(&ctx, err),
    }
}

fn parse_common_flags(args: &[String]) -> CommonFlags {
    let mut out = CommonFlags::default();
    let mut i = 0usize;
    while i < args.len() {
        let a = &args[i];
        if a == "--help" || a == "-h" {
            out.help = true;
            i += 1;
            continue;
        }
        if a == "--json" || a == "--output=json" {
            out.json = true;
            i += 1;
            continue;
        }
        if a == "--output" {
            if i + 1 < args.len() && args[i + 1] == "json" {
                out.json = true;
                i += 2;
                continue;
            }
            if i + 1 < args.len() {
                out.args.push(a.clone());
                out.args.push(args[i + 1].clone());
                i += 2;
                continue;
            }
        }
        if a == "--repo" {
            if i + 1 < args.len() {
                out.repo_override = Some(PathBuf::from(&args[i + 1]));
                i += 2;
                continue;
            }
        }
        if let Some(v) = a.strip_prefix("--repo=") {
            out.repo_override = Some(PathBuf::from(v));
            i += 1;
            continue;
        }
        out.args.push(a.clone());
        i += 1;
    }
    out
}

fn namespace_help(namespace: &str, subcommands: &[&str]) -> String {
    format!(
        "Usage:\n  quarxtor {namespace} <subcommand> [--repo <path>] [--output json]\n\nAvailable subcommands:\n  {}",
        subcommands.join(", ")
    )
}

fn success_outcome(ctx: &CommandContext, result: CommandResult) -> CommandOutcome {
    if ctx.json {
        let envelope = envelope_json(ctx, result.state, result.result, None);
        return CommandOutcome {
            exit_code: EXIT_OK,
            stdout: Some(envelope),
            stderr: None,
        };
    }
    CommandOutcome {
        exit_code: EXIT_OK,
        stdout: Some(result.text),
        stderr: None,
    }
}

fn error_outcome(ctx: &CommandContext, err: CmdError) -> CommandOutcome {
    if ctx.json {
        let envelope = envelope_json(
            ctx,
            "error",
            Value::Null,
            Some(json!({
                "code": err.code,
                "category": err.category,
                "message": err.message
            })),
        );
        return CommandOutcome {
            exit_code: err.exit_code,
            stdout: Some(envelope),
            stderr: None,
        };
    }
    CommandOutcome {
        exit_code: err.exit_code,
        stdout: None,
        stderr: Some(format!(
            "{} {}: {}",
            ctx.namespace, ctx.subcommand, err.message
        )),
    }
}

fn envelope_json(ctx: &CommandContext, state: &str, result: Value, err: Option<Value>) -> String {
    let ended_at_ms = now_ms();
    let errors = match err {
        Some(e) => Value::Array(vec![e]),
        None => Value::Array(Vec::new()),
    };
    let payload = json!({
        "meta": {
            "namespace": ctx.namespace,
            "subcommand": ctx.subcommand,
            "state": state
        },
        "result": result,
        "errors": errors,
        "timing": {
            "started_at_ms": ctx.started_at_ms,
            "ended_at_ms": ended_at_ms,
            "duration_ms": ended_at_ms.saturating_sub(ctx.started_at_ms)
        },
        "operation_id": format!("{}-{}-{}", ctx.namespace, ctx.subcommand, ctx.started_at_ms)
    });
    payload.to_string()
}

fn handle_config(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    match ctx.subcommand.as_str() {
        "show" => {
            let path = ensure_config_file()?;
            let content = fs::read_to_string(&path).map_err(|e| CmdError::io(e.to_string()))?;
            Ok(CommandResult {
                state: "ok",
                text: format!("config_path={}\n{}", path.display(), content),
                result: json!({
                    "config_path": path.display().to_string(),
                    "content": content
                }),
            })
        }
        "effective" => {
            let cfg =
                EffectiveConfig::load_default().map_err(|e| CmdError::config(e.to_string()))?;
            let mut rows = Vec::new();
            for key in config_contract() {
                rows.push(json!({
                    "key": format!("{}.{}", key.section, key.key),
                    "value": effective_value_for_key(&cfg, key.section, key.key),
                }));
            }
            let mut text = String::new();
            for row in &rows {
                text.push_str(&format!(
                    "{}={}\n",
                    row["key"].as_str().unwrap_or(""),
                    row["value"].as_str().unwrap_or("")
                ));
            }
            Ok(CommandResult {
                state: "ok",
                text,
                result: json!({
                    "keys": rows
                }),
            })
        }
        "explain" => {
            if args.is_empty() {
                return Err(CmdError::usage(
                    "config explain requires <section.key> argument",
                ));
            }
            let key = &args[0];
            let Some((section, name)) = split_config_key(key) else {
                return Err(CmdError::usage("config key must be section.key"));
            };
            let mut found = None;
            for entry in config_contract() {
                if entry.section == section && entry.key == name {
                    found = Some(entry);
                    break;
                }
            }
            let Some(entry) = found else {
                return Err(CmdError::config(format!("unknown config key: {key}")));
            };
            let value_type = match entry.value_type {
                ConfigValueType::Bool => "bool".to_string(),
                ConfigValueType::U64 => "u64".to_string(),
                ConfigValueType::String => "string".to_string(),
                ConfigValueType::Path => "path".to_string(),
                ConfigValueType::Enum(values) => format!("enum({})", values.join("|")),
            };
            let desc = entry.description.join(" ");
            let env = entry.env_override.unwrap_or("");
            let text = format!(
                "key={}.{}\ndefault={}\ntype={}\nstatus={}\nenv_override={}\ndescription={}",
                entry.section,
                entry.key,
                entry.default_value,
                value_type,
                entry.status.as_str(),
                env,
                desc
            );
            Ok(CommandResult {
                state: "ok",
                text,
                result: json!({
                    "key": format!("{}.{}", entry.section, entry.key),
                    "default": entry.default_value,
                    "type": value_type,
                    "status": entry.status.as_str(),
                    "env_override": env,
                    "description": desc
                }),
            })
        }
        "set" => {
            if args.len() < 2 {
                return Err(CmdError::usage("config set requires <section.key> <value>"));
            }
            let key = &args[0];
            let value = args[1..].join(" ");
            let Some((section, name)) = split_config_key(key) else {
                return Err(CmdError::usage("config key must be section.key"));
            };
            if !config_contract()
                .iter()
                .any(|entry| entry.section == section && entry.key == name)
            {
                return Err(CmdError::config(format!("unknown config key: {key}")));
            }
            let path = ensure_config_file()?;
            let text = fs::read_to_string(&path).unwrap_or_default();
            let mut parsed = parse_ini_sections(&text).sections;
            parsed
                .entry(section.to_string())
                .or_default()
                .insert(name.to_string(), value.clone());
            let next_text = render_ini_sections(&parsed);
            EffectiveConfig::from_ini_str(&next_text)
                .map_err(|e| CmdError::config(e.to_string()))?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| CmdError::io(e.to_string()))?;
            }
            fs::write(&path, &next_text).map_err(|e| CmdError::io(e.to_string()))?;
            Ok(CommandResult {
                state: "ok",
                text: format!("updated {}={}", key, value),
                result: json!({
                    "config_path": path.display().to_string(),
                    "key": key,
                    "value": value
                }),
            })
        }
        "lint" => {
            let path = ensure_config_file()?;
            let text = fs::read_to_string(&path).map_err(|e| CmdError::io(e.to_string()))?;
            let parsed = parse_ini_sections(&text);
            let strict = EffectiveConfig::from_ini_str_strict(&text);
            let (ok, message) = match strict {
                Ok(_) => (true, "ok".to_string()),
                Err(e) => (false, e.to_string()),
            };
            let text_out = format!(
                "config_path={}\nstrict_ok={}\nunknown_keys={}\nmessage={}",
                path.display(),
                ok,
                parsed.unknown_keys.len(),
                message
            );
            if !ok {
                return Err(CmdError::config(text_out));
            }
            Ok(CommandResult {
                state: "ok",
                text: text_out,
                result: json!({
                    "config_path": path.display().to_string(),
                    "strict_ok": ok,
                    "unknown_keys": parsed.unknown_keys,
                    "message": message
                }),
            })
        }
        _ => Err(CmdError::internal("unsupported config subcommand")),
    }
}

fn handle_repo(ctx: &CommandContext, _args: &[String]) -> Result<CommandResult, CmdError> {
    match ctx.subcommand.as_str() {
        "init" => repo_init(&ctx.repo_path),
        "verify" => repo_verify(&ctx.repo_path),
        "doctor" => repo_doctor(&ctx.repo_path),
        "recover" => repo_recover(&ctx.repo_path),
        "stats" => repo_stats(&ctx.repo_path),
        _ => Err(CmdError::internal("unsupported repo subcommand")),
    }
}

fn repo_init(repo_path: &Path) -> Result<CommandResult, CmdError> {
    if let Some(parent) = repo_path.parent() {
        fs::create_dir_all(parent).map_err(|e| CmdError::io(e.to_string()))?;
    }
    let store = FileBlockStore::open(repo_path.to_path_buf())
        .map_err(|e| CmdError::io(format!("{e:?}")))?;
    let before = store.len() as u64;
    let cfg = QuarxConfig::load_default();
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);
    let root_id = ensure_root_dir(repo_path, &mut repo).map_err(CmdError::state)?;
    let store = repo.into_store();
    let after = store.len() as u64;
    let sidecar = with_extension(repo_path, "qroot");
    Ok(CommandResult {
        state: "ok",
        text: format!(
            "repo_path={}\nroot_id={}\nblocks_before={}\nblocks_after={}\nroot_sidecar={}",
            repo_path.display(),
            root_id,
            before,
            after,
            sidecar.display()
        ),
        result: json!({
            "repo_path": repo_path.display().to_string(),
            "root_id": root_id,
            "blocks_before": before,
            "blocks_after": after,
            "root_sidecar": sidecar.display().to_string()
        }),
    })
}

fn repo_verify(repo_path: &Path) -> Result<CommandResult, CmdError> {
    let store = FileBlockStore::open(repo_path.to_path_buf())
        .map_err(|e| CmdError::io(format!("{e:?}")))?;
    let blocks = store.len() as u64;
    let cfg = QuarxConfig::load_default();
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);
    let root_id = get_root_dir_id(repo_path, &mut repo).map_err(CmdError::state)?;
    let entries = repo
        .read_dir(root_id)
        .map_err(|e| CmdError::state(format!("read_dir failed: {e:?}")))?;
    Ok(CommandResult {
        state: "ok",
        text: format!(
            "repo_path={}\nverify=ok\nroot_id={}\nroot_entries={}\nblocks={}",
            repo_path.display(),
            root_id,
            entries.len(),
            blocks
        ),
        result: json!({
            "repo_path": repo_path.display().to_string(),
            "verify": "ok",
            "root_id": root_id,
            "root_entries": entries.len(),
            "blocks": blocks
        }),
    })
}

fn repo_doctor(repo_path: &Path) -> Result<CommandResult, CmdError> {
    let preflight = run_preflight(PreflightOptions {
        repo_override: Some(repo_path.to_path_buf()),
        import_root_override: None,
    });
    let verify = repo_verify(repo_path);
    let verify_ok = verify.is_ok();
    let mut issues = Vec::new();
    if !preflight.report.required_ok() {
        for dep in &preflight.report.deps {
            if dep.required && dep.status.as_str() != "ok" {
                issues.push(format!("{}={}", dep.name, dep.status.as_str()));
            }
        }
    }
    if let Err(err) = &verify {
        issues.push(err.message.clone());
    }
    let status = if issues.is_empty() { "ok" } else { "degraded" };
    let text = format!(
        "repo_path={}\nstatus={}\nrequired_ok={}\nverify_ok={}\nissues={}",
        repo_path.display(),
        status,
        preflight.report.required_ok(),
        verify_ok,
        issues.join("; ")
    );
    Ok(CommandResult {
        state: "ok",
        text,
        result: json!({
            "repo_path": repo_path.display().to_string(),
            "status": status,
            "required_ok": preflight.report.required_ok(),
            "verify_ok": verify_ok,
            "issues": issues
        }),
    })
}

fn repo_recover(repo_path: &Path) -> Result<CommandResult, CmdError> {
    if let Some(parent) = repo_path.parent() {
        fs::create_dir_all(parent).map_err(|e| CmdError::io(e.to_string()))?;
    }
    let store = FileBlockStore::open(repo_path.to_path_buf())
        .map_err(|e| CmdError::io(format!("{e:?}")))?;
    let cfg = QuarxConfig::load_default();
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);
    let root_id = ensure_root_dir(repo_path, &mut repo).map_err(CmdError::state)?;
    let text = format!(
        "repo_path={}\nrecover=ok\nroot_id={}\nroot_sidecar={}",
        repo_path.display(),
        root_id,
        with_extension(repo_path, "qroot").display()
    );
    Ok(CommandResult {
        state: "ok",
        text,
        result: json!({
            "repo_path": repo_path.display().to_string(),
            "recover": "ok",
            "root_id": root_id,
            "root_sidecar": with_extension(repo_path, "qroot").display().to_string()
        }),
    })
}

fn repo_stats(repo_path: &Path) -> Result<CommandResult, CmdError> {
    let store = FileBlockStore::open(repo_path.to_path_buf())
        .map_err(|e| CmdError::io(format!("{e:?}")))?;
    let blocks = store.len() as u64;
    let cfg = QuarxConfig::load_default();
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);
    let (root_id, stats) =
        compute_root_stats(repo_path, &mut repo, &cfg).map_err(CmdError::state)?;
    let text = format!(
        "repo_path={}\nroot_id={}\nblocks={}\ndirs={}\nfiles={}\nbytes={}",
        repo_path.display(),
        root_id,
        blocks,
        stats.dirs,
        stats.files,
        stats.bytes
    );
    Ok(CommandResult {
        state: "ok",
        text,
        result: json!({
            "repo_path": repo_path.display().to_string(),
            "root_id": root_id,
            "blocks": blocks,
            "dirs": stats.dirs,
            "files": stats.files,
            "bytes": stats.bytes
        }),
    })
}

fn handle_service(ctx: &CommandContext, _args: &[String]) -> Result<CommandResult, CmdError> {
    let mut state = load_state(&ctx.repo_path)?;
    let now = now_ms();
    match ctx.subcommand.as_str() {
        "start" => {
            let endpoint = service_endpoint();
            state.service.running = true;
            state.service.endpoint = endpoint.clone();
            state.service.last_changed_ms = now;
            state
                .service
                .logs
                .push(format!("{now}: start endpoint={endpoint}"));
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("service=running\nendpoint={endpoint}"),
                result: json!({
                    "running": true,
                    "endpoint": endpoint
                }),
            })
        }
        "stop" => {
            state.service.running = false;
            state.service.last_changed_ms = now;
            state.service.logs.push(format!("{now}: stop"));
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: "service=stopped".to_string(),
                result: json!({
                    "running": false,
                    "endpoint": state.service.endpoint
                }),
            })
        }
        "restart" => {
            let endpoint = service_endpoint();
            state.service.running = true;
            state.service.restart_count = state.service.restart_count.saturating_add(1);
            state.service.endpoint = endpoint.clone();
            state.service.last_changed_ms = now;
            state
                .service
                .logs
                .push(format!("{now}: restart endpoint={endpoint}"));
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "service=running\nendpoint={}\nrestart_count={}",
                    endpoint, state.service.restart_count
                ),
                result: json!({
                    "running": true,
                    "endpoint": endpoint,
                    "restart_count": state.service.restart_count
                }),
            })
        }
        "status" => Ok(CommandResult {
            state: "ok",
            text: format!(
                "service_running={}\nendpoint={}\nrestart_count={}\nlast_changed_ms={}",
                state.service.running,
                state.service.endpoint,
                state.service.restart_count,
                state.service.last_changed_ms
            ),
            result: json!({
                "running": state.service.running,
                "endpoint": state.service.endpoint,
                "restart_count": state.service.restart_count,
                "last_changed_ms": state.service.last_changed_ms
            }),
        }),
        "logs" => {
            let logs = tail(&state.service.logs, 32);
            Ok(CommandResult {
                state: "ok",
                text: logs.join("\n"),
                result: json!({
                    "entries": logs
                }),
            })
        }
        _ => Err(CmdError::internal("unsupported service subcommand")),
    }
}

fn handle_mount(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    let mut state = load_state(&ctx.repo_path)?;
    match ctx.subcommand.as_str() {
        "create" => {
            let parsed = parse_options(args);
            if parsed.positionals.is_empty() {
                return Err(CmdError::usage("mount create requires <name>"));
            }
            let name = parsed.positionals[0].clone();
            if state.mounts.iter().any(|m| m.name == name) {
                return Err(CmdError::conflict(format!(
                    "mount already exists: {}",
                    name
                )));
            }
            let mode = parsed
                .options
                .get("mode")
                .cloned()
                .unwrap_or_else(|| "disk".to_string());
            if mode != "disk" && mode != "non-disk" {
                return Err(CmdError::usage("mount mode must be disk|non-disk"));
            }
            let mountpoint = parsed.options.get("mountpoint").cloned();
            let now = now_ms();
            let rec = MountRecord {
                id: state.next_mount_id,
                name,
                mode,
                mounted: false,
                mountpoint,
                snapshot_path: None,
                created_ms: now,
                updated_ms: now,
            };
            state.next_mount_id = state.next_mount_id.saturating_add(1);
            state.mounts.push(rec.clone());
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "mount_id={}\nname={}\nmode={}\nmounted={}",
                    rec.id, rec.name, rec.mode, rec.mounted
                ),
                result: json!(rec),
            })
        }
        "attach" => {
            let parsed = parse_options(args);
            if parsed.positionals.is_empty() {
                return Err(CmdError::usage("mount attach requires <snapshot-path>"));
            }
            let snapshot_path = PathBuf::from(&parsed.positionals[0]);
            let snapshot = read_json_file(&snapshot_path)?;
            let mode = snapshot
                .get("mode")
                .and_then(|v| v.as_str())
                .unwrap_or("disk")
                .to_string();
            let name = parsed
                .options
                .get("name")
                .cloned()
                .or_else(|| {
                    snapshot
                        .get("mount_name")
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string())
                })
                .unwrap_or_else(|| format!("attached-{}", state.next_mount_id));
            if state.mounts.iter().any(|m| m.name == name) {
                return Err(CmdError::conflict(format!(
                    "mount already exists: {}",
                    name
                )));
            }
            let now = now_ms();
            let rec = MountRecord {
                id: state.next_mount_id,
                name,
                mode,
                mounted: false,
                mountpoint: parsed.options.get("mountpoint").cloned(),
                snapshot_path: Some(snapshot_path.display().to_string()),
                created_ms: now,
                updated_ms: now,
            };
            state.next_mount_id = state.next_mount_id.saturating_add(1);
            state.mounts.push(rec.clone());
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "attached_mount_id={}\nname={}\nmode={}",
                    rec.id, rec.name, rec.mode
                ),
                result: json!(rec),
            })
        }
        "mount" => {
            let parsed = parse_options(args);
            if parsed.positionals.is_empty() {
                return Err(CmdError::usage("mount mount requires <name|id>"));
            }
            let target = &parsed.positionals[0];
            let Some(rec) = find_mount_mut(&mut state.mounts, target) else {
                return Err(CmdError::state(format!("mount not found: {target}")));
            };
            rec.mounted = true;
            if let Some(mp) = parsed.options.get("mountpoint") {
                rec.mountpoint = Some(mp.clone());
            }
            rec.updated_ms = now_ms();
            let output = rec.clone();
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "mounted=true\nname={}\nid={}\nmountpoint={}",
                    output.name,
                    output.id,
                    output.mountpoint.clone().unwrap_or_default()
                ),
                result: json!(output),
            })
        }
        "unmount" => {
            if args.is_empty() {
                return Err(CmdError::usage("mount unmount requires <name|id>"));
            }
            let target = &args[0];
            let Some(rec) = find_mount_mut(&mut state.mounts, target) else {
                return Err(CmdError::state(format!("mount not found: {target}")));
            };
            rec.mounted = false;
            rec.updated_ms = now_ms();
            let output = rec.clone();
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("mounted=false\nname={}\nid={}", output.name, output.id),
                result: json!(output),
            })
        }
        "list" => {
            let mounts = state.mounts.clone();
            let mut text = String::new();
            for m in &mounts {
                text.push_str(&format!(
                    "id={} name={} mode={} mounted={} mountpoint={}\n",
                    m.id,
                    m.name,
                    m.mode,
                    m.mounted,
                    m.mountpoint.clone().unwrap_or_default()
                ));
            }
            if mounts.is_empty() {
                text = "no mounts".to_string();
            }
            Ok(CommandResult {
                state: "ok",
                text,
                result: json!({ "mounts": mounts }),
            })
        }
        "info" => {
            if args.is_empty() {
                return Err(CmdError::usage("mount info requires <name|id>"));
            }
            let target = &args[0];
            let Some(rec) = find_mount(&state.mounts, target) else {
                return Err(CmdError::state(format!("mount not found: {target}")));
            };
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "id={}\nname={}\nmode={}\nmounted={}\nmountpoint={}\nsnapshot={}",
                    rec.id,
                    rec.name,
                    rec.mode,
                    rec.mounted,
                    rec.mountpoint.clone().unwrap_or_default(),
                    rec.snapshot_path.clone().unwrap_or_default()
                ),
                result: json!(rec),
            })
        }
        _ => Err(CmdError::internal("unsupported mount subcommand")),
    }
}

fn handle_dashboard(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    let mut state = load_state(&ctx.repo_path)?;
    match ctx.subcommand.as_str() {
        "serve" => {
            let parsed = parse_options(args);
            let bind = parsed
                .options
                .get("bind")
                .cloned()
                .unwrap_or_else(|| "127.0.0.1:17878".to_string());
            state.dashboard.running = true;
            state.dashboard.bind = bind.clone();
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("dashboard_running=true\nbind={}", bind),
                result: json!({
                    "running": true,
                    "bind": bind
                }),
            })
        }
        "status" => Ok(CommandResult {
            state: "ok",
            text: format!(
                "dashboard_running={}\nbind={}\nsnapshot_count={}",
                state.dashboard.running,
                state.dashboard.bind,
                state.dashboard.snapshots.len()
            ),
            result: json!({
                "running": state.dashboard.running,
                "bind": state.dashboard.bind,
                "snapshot_count": state.dashboard.snapshots.len()
            }),
        }),
        "snapshot" => {
            let parsed = parse_options(args);
            let out_path = parsed
                .options
                .get("out")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    default_artifact_path(&ctx.repo_path, "dashboard-snapshot", "json")
                });
            let report = json!({
                "format": "quarxtor.dashboard.snapshot.v1",
                "created_at_ms": now_ms(),
                "repo_path": ctx.repo_path.display().to_string(),
                "service": state.service,
                "dashboard": {
                    "running": state.dashboard.running,
                    "bind": state.dashboard.bind
                },
                "mounts_count": state.mounts.len(),
                "clusters_count": state.clusters.len(),
                "compress_profile": state.compress.profile
            });
            write_json_file(&out_path, &report)?;
            state
                .dashboard
                .snapshots
                .push(out_path.display().to_string());
            state.artifacts.push(ArtifactRecord {
                kind: "dashboard_snapshot".to_string(),
                path: out_path.display().to_string(),
                created_ms: now_ms(),
            });
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("dashboard_snapshot={}", out_path.display()),
                result: json!({
                    "snapshot_path": out_path.display().to_string()
                }),
            })
        }
        _ => Err(CmdError::internal("unsupported dashboard subcommand")),
    }
}

fn handle_cloud(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    let mut state = load_state(&ctx.repo_path)?;
    match ctx.subcommand.as_str() {
        "cluster" => cloud_cluster(&ctx.repo_path, &mut state, args),
        "peer" => cloud_peer(&ctx.repo_path, &mut state, args),
        "discover" => cloud_discover(&ctx.repo_path, &state),
        "sync" => cloud_sync(&ctx.repo_path, &mut state, args),
        "policy" => cloud_policy(&ctx.repo_path, &mut state, args),
        "stats" => cloud_stats(&state),
        _ => Err(CmdError::internal("unsupported cloud subcommand")),
    }
}

fn cloud_cluster(
    repo_path: &Path,
    state: &mut QuarxState,
    args: &[String],
) -> Result<CommandResult, CmdError> {
    if args.is_empty() || args[0] == "list" {
        return Ok(CommandResult {
            state: "ok",
            text: format!("clusters={}", state.clusters.len()),
            result: json!({ "clusters": state.clusters }),
        });
    }
    match args[0].as_str() {
        "add" => {
            if args.len() < 3 {
                return Err(CmdError::usage(
                    "cloud cluster add requires <cluster_id> <public|private>",
                ));
            }
            let cluster_id = args[1].clone();
            let mode = args[2].clone();
            if mode != "public" && mode != "private" {
                return Err(CmdError::usage("cluster mode must be public|private"));
            }
            let display_name = if args.len() > 3 {
                args[3..].join(" ")
            } else {
                cluster_id.clone()
            };
            if let Some(existing) = state
                .clusters
                .iter_mut()
                .find(|c| c.cluster_id == cluster_id)
            {
                existing.mode = mode.clone();
                existing.display_name = display_name.clone();
                existing.last_seen_ms = now_ms();
                let output = existing.clone();
                save_state(repo_path, state)?;
                return Ok(CommandResult {
                    state: "ok",
                    text: format!("cluster_updated={}", cluster_id),
                    result: json!(output),
                });
            }
            let record = ClusterRecord {
                cluster_id: cluster_id.clone(),
                mode,
                display_name,
                policy_ref: "default".to_string(),
                capabilities: vec![
                    "mount".to_string(),
                    "snapshot-share".to_string(),
                    "optimizer-remote".to_string(),
                    "gateway-auth".to_string(),
                ],
                transport_endpoints: vec![
                    "ipc".to_string(),
                    "http".to_string(),
                    "grpc".to_string(),
                    "quic".to_string(),
                ],
                federation_peers: Vec::new(),
                last_seen_ms: now_ms(),
                health_state: "healthy".to_string(),
            };
            state.clusters.push(record.clone());
            save_state(repo_path, state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("cluster_added={}", cluster_id),
                result: json!(record),
            })
        }
        "remove" => {
            if args.len() < 2 {
                return Err(CmdError::usage(
                    "cloud cluster remove requires <cluster_id>",
                ));
            }
            let before = state.clusters.len();
            state.clusters.retain(|c| c.cluster_id != args[1]);
            let removed = before.saturating_sub(state.clusters.len());
            save_state(repo_path, state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("clusters_removed={removed}"),
                result: json!({ "removed": removed }),
            })
        }
        "show" => {
            if args.len() < 2 {
                return Err(CmdError::usage("cloud cluster show requires <cluster_id>"));
            }
            let Some(cluster) = state.clusters.iter().find(|c| c.cluster_id == args[1]) else {
                return Err(CmdError::state(format!("cluster not found: {}", args[1])));
            };
            Ok(CommandResult {
                state: "ok",
                text: format!("cluster_id={}\nmode={}", cluster.cluster_id, cluster.mode),
                result: json!(cluster),
            })
        }
        _ => Err(CmdError::usage(
            "cloud cluster supports list|add|remove|show",
        )),
    }
}

fn cloud_peer(
    repo_path: &Path,
    state: &mut QuarxState,
    args: &[String],
) -> Result<CommandResult, CmdError> {
    if args.is_empty() || args[0] == "list" {
        return Ok(CommandResult {
            state: "ok",
            text: format!("peers={}", state.peers.len()),
            result: json!({ "peers": state.peers }),
        });
    }
    match args[0].as_str() {
        "add" => {
            if args.len() < 4 {
                return Err(CmdError::usage(
                    "cloud peer add requires <peer_id> <cluster_id> <addr>",
                ));
            }
            let peer = PeerRecord {
                peer_id: args[1].clone(),
                cluster_id: args[2].clone(),
                addr: args[3].clone(),
                last_seen_ms: now_ms(),
            };
            state.peers.retain(|p| p.peer_id != peer.peer_id);
            state.peers.push(peer.clone());
            save_state(repo_path, state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("peer_added={}", peer.peer_id),
                result: json!(peer),
            })
        }
        "remove" => {
            if args.len() < 2 {
                return Err(CmdError::usage("cloud peer remove requires <peer_id>"));
            }
            let before = state.peers.len();
            state.peers.retain(|p| p.peer_id != args[1]);
            let removed = before.saturating_sub(state.peers.len());
            save_state(repo_path, state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("peers_removed={removed}"),
                result: json!({ "removed": removed }),
            })
        }
        _ => Err(CmdError::usage("cloud peer supports list|add|remove")),
    }
}

fn cloud_discover(repo_path: &Path, state: &QuarxState) -> Result<CommandResult, CmdError> {
    let _ = repo_path;
    let clusters: Vec<String> = state
        .clusters
        .iter()
        .map(|c| c.cluster_id.clone())
        .collect();
    let text = format!(
        "discovery_order=local_registry_cache,private_control_plane,bounded_mesh_query,gateway_federation\nclusters_found={}",
        clusters.join(",")
    );
    Ok(CommandResult {
        state: "ok",
        text,
        result: json!({
            "order": [
                "local_registry_cache",
                "private_control_plane",
                "bounded_mesh_query",
                "gateway_federation"
            ],
            "clusters_found": clusters
        }),
    })
}

fn cloud_sync(
    repo_path: &Path,
    state: &mut QuarxState,
    args: &[String],
) -> Result<CommandResult, CmdError> {
    if args.len() < 2 {
        return Err(CmdError::usage(
            "cloud sync requires <source_cluster> <target_cluster>",
        ));
    }
    let source = args[0].clone();
    let target = args[1].clone();
    let stats = collect_repo_stats(repo_path).unwrap_or_default();
    let meta_hashes = (stats.files.saturating_add(stats.dirs)).max(1);
    let block_keys = stats.blocks.max(1);
    let rec = SyncRecord {
        id: state.next_sync_id,
        source_cluster: source.clone(),
        target_cluster: target.clone(),
        meta_hashes,
        block_keys,
        completed_ms: now_ms(),
    };
    state.next_sync_id = state.next_sync_id.saturating_add(1);
    state.sync_history.push(rec.clone());
    save_state(repo_path, state)?;
    Ok(CommandResult {
        state: "ok",
        text: format!(
            "sync_id={}\nsource={}\ntarget={}\nmeta_hashes={}\nblock_keys={}",
            rec.id, source, target, rec.meta_hashes, rec.block_keys
        ),
        result: json!(rec),
    })
}

fn cloud_policy(
    repo_path: &Path,
    state: &mut QuarxState,
    args: &[String],
) -> Result<CommandResult, CmdError> {
    if args.is_empty() || args[0] == "show" {
        return Ok(CommandResult {
            state: "ok",
            text: format!(
                "ttl={}\nmax_fanout={}\nrate_limit_per_min={}\nallow_mesh_broadcast={}\nallow_utp={}",
                state.cloud_policy.ttl,
                state.cloud_policy.max_fanout,
                state.cloud_policy.rate_limit_per_min,
                state.cloud_policy.allow_mesh_broadcast,
                state.cloud_policy.allow_utp
            ),
            result: json!(state.cloud_policy),
        });
    }
    if args[0] != "set" {
        return Err(CmdError::usage("cloud policy supports show|set"));
    }
    let parsed = parse_options(&args[1..]);
    if let Some(v) = parsed
        .options
        .get("ttl")
        .and_then(|v| v.parse::<u64>().ok())
    {
        state.cloud_policy.ttl = v;
    }
    if let Some(v) = parsed
        .options
        .get("fanout")
        .and_then(|v| v.parse::<u64>().ok())
    {
        state.cloud_policy.max_fanout = v;
    }
    if let Some(v) = parsed
        .options
        .get("rate")
        .and_then(|v| v.parse::<u64>().ok())
    {
        state.cloud_policy.rate_limit_per_min = v;
    }
    if let Some(v) = parsed.options.get("mesh") {
        state.cloud_policy.allow_mesh_broadcast =
            parse_bool(v, state.cloud_policy.allow_mesh_broadcast);
    }
    if let Some(v) = parsed.options.get("utp") {
        state.cloud_policy.allow_utp = parse_bool(v, state.cloud_policy.allow_utp);
    }
    save_state(repo_path, state)?;
    Ok(CommandResult {
        state: "ok",
        text: "cloud policy updated".to_string(),
        result: json!(state.cloud_policy),
    })
}

fn cloud_stats(state: &QuarxState) -> Result<CommandResult, CmdError> {
    let public_clusters = state.clusters.iter().filter(|c| c.mode == "public").count();
    let private_clusters = state.clusters.len().saturating_sub(public_clusters);
    Ok(CommandResult {
        state: "ok",
        text: format!(
            "clusters_total={}\nclusters_public={}\nclusters_private={}\npeers_total={}\nsync_runs={}",
            state.clusters.len(),
            public_clusters,
            private_clusters,
            state.peers.len(),
            state.sync_history.len()
        ),
        result: json!({
            "clusters_total": state.clusters.len(),
            "clusters_public": public_clusters,
            "clusters_private": private_clusters,
            "peers_total": state.peers.len(),
            "sync_runs": state.sync_history.len()
        }),
    })
}

fn handle_compress(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    let mut state = load_state(&ctx.repo_path)?;
    match ctx.subcommand.as_str() {
        "analyze" => {
            let analysis = compute_compress_analysis(&ctx.repo_path)?;
            state.compress.last_analysis = analysis.clone();
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "bytes_before={}\nfiles={}\ndirs={}\nblocks={}\nestimated_space_saved_bytes={}",
                    analysis.bytes_before,
                    analysis.files,
                    analysis.dirs,
                    analysis.blocks,
                    analysis.estimated_space_saved_bytes
                ),
                result: json!(analysis),
            })
        }
        "plan" => {
            let analysis = compute_compress_analysis(&ctx.repo_path)?;
            state.compress.last_analysis = analysis.clone();
            let plan = CompressPlanRecord {
                id: state.next_plan_id,
                created_ms: now_ms(),
                profile: state.compress.profile.clone(),
                bytes_before: analysis.bytes_before,
                estimated_space_saved_bytes: analysis.estimated_space_saved_bytes,
                status: "planned".to_string(),
            };
            state.next_plan_id = state.next_plan_id.saturating_add(1);
            state.compress.plans.push(plan.clone());
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "plan_id={}\nprofile={}\nbytes_before={}\nestimated_space_saved_bytes={}",
                    plan.id, plan.profile, plan.bytes_before, plan.estimated_space_saved_bytes
                ),
                result: json!(plan),
            })
        }
        "apply" => {
            let plan_id = if let Some(first) = args.first() {
                first
                    .parse::<u64>()
                    .map_err(|_| CmdError::usage("compress apply expects numeric plan_id"))?
            } else {
                let Some(last) = state.compress.plans.last() else {
                    return Err(CmdError::state(
                        "no plan available; run compress plan first",
                    ));
                };
                last.id
            };
            let Some(plan) = state.compress.plans.iter_mut().find(|p| p.id == plan_id) else {
                return Err(CmdError::state(format!("plan not found: {plan_id}")));
            };
            if plan.status == "applied" {
                return Err(CmdError::conflict(format!(
                    "plan already applied: {plan_id}"
                )));
            }
            plan.status = "applied".to_string();
            let before = plan.bytes_before;
            let saved = plan.estimated_space_saved_bytes.min(before);
            let after = before.saturating_sub(saved);
            let job = CompressJobRecord {
                id: state.next_job_id,
                plan_id,
                created_ms: now_ms(),
                status: "applied".to_string(),
                bytes_before: before,
                bytes_after: after,
                space_saved_bytes: saved,
                dedup_ratio_before_x1000: 1000,
                dedup_ratio_after_x1000: if before > 0 {
                    1000 + (saved * 1000 / before)
                } else {
                    1000
                },
                compression_ratio_before_x1000: 1000,
                compression_ratio_after_x1000: if before > 0 {
                    1000 + (saved * 700 / before)
                } else {
                    1000
                },
                cpu_ms: 250 + (before / 1024 / 1024),
                io_read_bytes: before,
                io_write_bytes: after,
                prediction_error_pct_x100: 0,
            };
            state.next_job_id = state.next_job_id.saturating_add(1);
            state.compress.jobs.push(job.clone());
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "job_id={}\nplan_id={}\nbytes_before={}\nbytes_after={}\nspace_saved_bytes={}",
                    job.id, job.plan_id, job.bytes_before, job.bytes_after, job.space_saved_bytes
                ),
                result: json!(job),
            })
        }
        "rollback" => {
            let target_id = if let Some(first) = args.first() {
                first
                    .parse::<u64>()
                    .map_err(|_| CmdError::usage("compress rollback expects numeric job_id"))?
            } else {
                let Some(job) = state
                    .compress
                    .jobs
                    .iter()
                    .rev()
                    .find(|j| j.status == "applied")
                else {
                    return Err(CmdError::state("no applied job to rollback"));
                };
                job.id
            };
            let Some(job) = state.compress.jobs.iter_mut().find(|j| j.id == target_id) else {
                return Err(CmdError::state(format!("job not found: {target_id}")));
            };
            if job.status == "rolled_back" {
                return Err(CmdError::conflict(format!(
                    "job already rolled back: {target_id}"
                )));
            }
            job.status = "rolled_back".to_string();
            let output = job.clone();
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("rollback_job_id={}", target_id),
                result: json!(output),
            })
        }
        "history" => {
            let plans = state.compress.plans.clone();
            let jobs = state.compress.jobs.clone();
            Ok(CommandResult {
                state: "ok",
                text: format!("plans={}\njobs={}", plans.len(), jobs.len()),
                result: json!({
                    "profile": state.compress.profile,
                    "plans": plans,
                    "jobs": jobs
                }),
            })
        }
        "tune" => {
            if args.is_empty() {
                return Err(CmdError::usage("compress tune requires <hot|warm|cold>"));
            }
            let profile = args[0].as_str();
            if !matches!(profile, "hot" | "warm" | "cold") {
                return Err(CmdError::usage("compress profile must be hot|warm|cold"));
            }
            state.compress.profile = profile.to_string();
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("compress_profile={}", profile),
                result: json!({ "profile": profile }),
            })
        }
        _ => Err(CmdError::internal("unsupported compress subcommand")),
    }
}

fn handle_export(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    let mut state = load_state(&ctx.repo_path)?;
    match ctx.subcommand.as_str() {
        "snapshot" => {
            let parsed = parse_options(args);
            let mode = parsed
                .options
                .get("mode")
                .cloned()
                .unwrap_or_else(|| "disk".to_string());
            if mode != "disk" && mode != "non-disk" {
                return Err(CmdError::usage(
                    "export snapshot --mode must be disk|non-disk",
                ));
            }
            let stats = collect_repo_stats(&ctx.repo_path)?;
            let snapshot = json!({
                "format": "quarxtor.snapshot.v1",
                "mode": mode,
                "repo_path": ctx.repo_path.display().to_string(),
                "root_id": stats.root_id,
                "created_at_ms": now_ms(),
                "mounts": state.mounts,
                "cloud_policy": state.cloud_policy,
                "compress_profile": state.compress.profile
            });
            let out_path = parsed
                .options
                .get("out")
                .map(PathBuf::from)
                .unwrap_or_else(|| {
                    default_artifact_path(&ctx.repo_path, "snapshot", "qxsnap.json")
                });
            write_json_file(&out_path, &snapshot)?;
            state.artifacts.push(ArtifactRecord {
                kind: "snapshot".to_string(),
                path: out_path.display().to_string(),
                created_ms: now_ms(),
            });
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("snapshot_path={}", out_path.display()),
                result: json!({
                    "snapshot_path": out_path.display().to_string(),
                    "mode": mode
                }),
            })
        }
        "manifest" => {
            let parsed = parse_options(args);
            let manifest = build_manifest(&ctx.repo_path)?;
            let mut text = format!("repo_path={}", ctx.repo_path.display());
            if let Some(out) = parsed.options.get("out").map(PathBuf::from) {
                write_json_file(&out, &manifest)?;
                text.push_str(&format!("\nmanifest_path={}", out.display()));
                state.artifacts.push(ArtifactRecord {
                    kind: "manifest".to_string(),
                    path: out.display().to_string(),
                    created_ms: now_ms(),
                });
                save_state(&ctx.repo_path, &state)?;
            }
            Ok(CommandResult {
                state: "ok",
                text,
                result: manifest,
            })
        }
        "bundle" => {
            let parsed = parse_options(args);
            let snapshot_value = {
                let stats = collect_repo_stats(&ctx.repo_path)?;
                json!({
                    "format": "quarxtor.snapshot.v1",
                    "mode": "non-disk",
                    "repo_path": ctx.repo_path.display().to_string(),
                    "root_id": stats.root_id,
                    "created_at_ms": now_ms()
                })
            };
            let manifest = build_manifest(&ctx.repo_path)?;
            let bundle = json!({
                "format": "quarxtor.bundle.v1",
                "created_at_ms": now_ms(),
                "snapshot": snapshot_value,
                "manifest": manifest
            });
            let out_path = parsed
                .options
                .get("out")
                .map(PathBuf::from)
                .unwrap_or_else(|| default_artifact_path(&ctx.repo_path, "bundle", "qbundle.json"));
            write_json_file(&out_path, &bundle)?;
            state.artifacts.push(ArtifactRecord {
                kind: "bundle".to_string(),
                path: out_path.display().to_string(),
                created_ms: now_ms(),
            });
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("bundle_path={}", out_path.display()),
                result: json!({ "bundle_path": out_path.display().to_string() }),
            })
        }
        _ => Err(CmdError::internal("unsupported export subcommand")),
    }
}

fn handle_convert(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    match ctx.subcommand.as_str() {
        "to-disk" => convert_snapshot_mode(args, "disk"),
        "to-nondisk" => convert_snapshot_mode(args, "non-disk"),
        "verify" => convert_verify(args),
        _ => Err(CmdError::internal("unsupported convert subcommand")),
    }
}

fn convert_snapshot_mode(args: &[String], mode: &str) -> Result<CommandResult, CmdError> {
    let parsed = parse_options(args);
    if parsed.positionals.is_empty() {
        return Err(CmdError::usage("convert command requires <snapshot-path>"));
    }
    let in_path = PathBuf::from(&parsed.positionals[0]);
    let mut snapshot = read_json_file(&in_path)?;
    if !snapshot.is_object() {
        return Err(CmdError::compat("snapshot must be a JSON object"));
    }
    snapshot["mode"] = Value::String(mode.to_string());
    let out_path = parsed
        .options
        .get("out")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let suffix = if mode == "disk" { "disk" } else { "nondisk" };
            with_suffix(&in_path, suffix)
        });
    write_json_file(&out_path, &snapshot)?;
    Ok(CommandResult {
        state: "ok",
        text: format!("converted_path={}\nmode={}", out_path.display(), mode),
        result: json!({
            "input": in_path.display().to_string(),
            "output": out_path.display().to_string(),
            "mode": mode
        }),
    })
}

fn convert_verify(args: &[String]) -> Result<CommandResult, CmdError> {
    if args.is_empty() {
        return Err(CmdError::usage("convert verify requires <snapshot-path>"));
    }
    let path = PathBuf::from(&args[0]);
    let snapshot = read_json_file(&path)?;
    let format = snapshot
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mode = snapshot.get("mode").and_then(|v| v.as_str()).unwrap_or("");
    let repo_path = snapshot
        .get("repo_path")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let format_ok = matches!(format, "quarxtor.snapshot.v1" | "quarxtor.bundle.v1");
    let mode_ok = matches!(mode, "disk" | "non-disk");
    let repo_ok = !repo_path.is_empty() || format == "quarxtor.bundle.v1";
    let ok = format_ok && mode_ok && repo_ok;
    if !ok {
        return Err(CmdError::compat(format!(
            "verify failed: format_ok={format_ok} mode_ok={mode_ok} repo_ok={repo_ok}"
        )));
    }
    Ok(CommandResult {
        state: "ok",
        text: format!("verify=ok\npath={}", path.display()),
        result: json!({
            "verify": "ok",
            "path": path.display().to_string(),
            "format": format,
            "mode": mode
        }),
    })
}

fn handle_version(_ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    match _ctx.subcommand.as_str() {
        "show" => Ok(CommandResult {
            state: "ok",
            text: format!(
                "quarxtor_version={}\nformat_contract=v1\nsnapshot_formats=quarxtor.snapshot.v1,quarxtor.bundle.v1",
                env!("CARGO_PKG_VERSION")
            ),
            result: json!({
                "quarxtor_version": env!("CARGO_PKG_VERSION"),
                "format_contract": "v1",
                "snapshot_formats": ["quarxtor.snapshot.v1", "quarxtor.bundle.v1"]
            }),
        }),
        "check-compat" => {
            if args.is_empty() {
                return Ok(CommandResult {
                    state: "ok",
                    text: "compat=ok\nsupported_formats=quarxtor.snapshot.v1,quarxtor.bundle.v1".to_string(),
                    result: json!({
                        "compat": "ok",
                        "supported_formats": ["quarxtor.snapshot.v1", "quarxtor.bundle.v1"]
                    }),
                });
            }
            let path = PathBuf::from(&args[0]);
            let value = read_json_file(&path)?;
            let format = value.get("format").and_then(|v| v.as_str()).unwrap_or("");
            let compatible = matches!(format, "quarxtor.snapshot.v1" | "quarxtor.bundle.v1");
            if !compatible {
                return Err(CmdError::compat(format!("unsupported format: {format}")));
            }
            Ok(CommandResult {
                state: "ok",
                text: format!("compat=ok\npath={}\nformat={}", path.display(), format),
                result: json!({
                    "compat": "ok",
                    "path": path.display().to_string(),
                    "format": format
                }),
            })
        }
        _ => Err(CmdError::internal("unsupported version subcommand")),
    }
}

fn handle_diag(ctx: &CommandContext, args: &[String]) -> Result<CommandResult, CmdError> {
    match ctx.subcommand.as_str() {
        "preflight" => {
            let preflight = run_preflight(PreflightOptions {
                repo_override: Some(ctx.repo_path.clone()),
                import_root_override: None,
            });
            let deps_json: Value =
                serde_json::from_str(&preflight.report.to_json()).unwrap_or_else(|_| json!({}));
            Ok(CommandResult {
                state: "ok",
                text: format!(
                    "required_ok={}\nmissing_required_only_repo={}",
                    preflight.report.required_ok(),
                    preflight.report.missing_required_only_repo()
                ),
                result: json!({
                    "required_ok": preflight.report.required_ok(),
                    "missing_required_only_repo": preflight.report.missing_required_only_repo(),
                    "report": deps_json
                }),
            })
        }
        "trace" => {
            let mut state = load_state(&ctx.repo_path)?;
            if args.is_empty() || args[0] == "status" {
                return Ok(CommandResult {
                    state: "ok",
                    text: format!("trace_enabled={}", state.trace_enabled),
                    result: json!({ "trace_enabled": state.trace_enabled }),
                });
            }
            match args[0].as_str() {
                "on" => state.trace_enabled = true,
                "off" => state.trace_enabled = false,
                _ => {
                    return Err(CmdError::usage("diag trace supports on|off|status"));
                }
            }
            save_state(&ctx.repo_path, &state)?;
            Ok(CommandResult {
                state: "ok",
                text: format!("trace_enabled={}", state.trace_enabled),
                result: json!({ "trace_enabled": state.trace_enabled }),
            })
        }
        "report" => {
            let parsed = parse_options(args);
            let preflight = run_preflight(PreflightOptions {
                repo_override: Some(ctx.repo_path.clone()),
                import_root_override: None,
            });
            let state = load_state(&ctx.repo_path)?;
            let repo_stats = collect_repo_stats(&ctx.repo_path).unwrap_or_default();
            let report = json!({
                "format": "quarxtor.diag.report.v1",
                "created_at_ms": now_ms(),
                "repo_path": ctx.repo_path.display().to_string(),
                "preflight": {
                    "required_ok": preflight.report.required_ok(),
                    "missing_required_only_repo": preflight.report.missing_required_only_repo(),
                    "dependencies": serde_json::from_str::<Value>(&preflight.report.to_json()).unwrap_or_else(|_| json!({}))
                },
                "repo_stats": {
                    "root_id": repo_stats.root_id,
                    "blocks": repo_stats.blocks,
                    "dirs": repo_stats.dirs,
                    "files": repo_stats.files,
                    "bytes": repo_stats.bytes
                },
                "state_summary": {
                    "service_running": state.service.running,
                    "mounts": state.mounts.len(),
                    "clusters": state.clusters.len(),
                    "peers": state.peers.len(),
                    "compress_plans": state.compress.plans.len(),
                    "compress_jobs": state.compress.jobs.len(),
                    "trace_enabled": state.trace_enabled
                }
            });
            if let Some(out) = parsed.options.get("out").map(PathBuf::from) {
                write_json_file(&out, &report)?;
                return Ok(CommandResult {
                    state: "ok",
                    text: format!("diag_report={}", out.display()),
                    result: json!({
                        "report_path": out.display().to_string(),
                        "report": report
                    }),
                });
            }
            Ok(CommandResult {
                state: "ok",
                text: "diag report generated".to_string(),
                result: report,
            })
        }
        _ => Err(CmdError::internal("unsupported diag subcommand")),
    }
}

#[derive(Debug, Clone, Default)]
struct RepoStats {
    root_id: u64,
    blocks: u64,
    dirs: u64,
    files: u64,
    bytes: u64,
}

fn collect_repo_stats(repo_path: &Path) -> Result<RepoStats, CmdError> {
    let store = FileBlockStore::open(repo_path.to_path_buf())
        .map_err(|e| CmdError::io(format!("{e:?}")))?;
    let blocks = store.len() as u64;
    let cfg = QuarxConfig::load_default();
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);
    let (root_id, stats) =
        compute_root_stats(repo_path, &mut repo, &cfg).map_err(CmdError::state)?;
    Ok(RepoStats {
        root_id,
        blocks,
        dirs: stats.dirs,
        files: stats.files,
        bytes: stats.bytes,
    })
}

fn build_manifest(repo_path: &Path) -> Result<Value, CmdError> {
    let store = FileBlockStore::open(repo_path.to_path_buf())
        .map_err(|e| CmdError::io(format!("{e:?}")))?;
    let blocks = store.len() as u64;
    let cfg = QuarxConfig::load_default();
    let mut repo = QuarxRepo::with_chunk(store, cfg.l0_chunk as usize);
    let (root_id, entries) = build_root_tree(repo_path, &mut repo).map_err(CmdError::state)?;
    let (_, stats) = compute_root_stats(repo_path, &mut repo, &cfg).map_err(CmdError::state)?;
    let entries_json: Vec<Value> = entries
        .into_iter()
        .map(|entry| {
            let kind = match entry.kind {
                FsEntryKind::File => "file",
                FsEntryKind::Dir => "dir",
                FsEntryKind::Object => "object",
                FsEntryKind::Unknown => "unknown",
            };
            json!({
                "uid": entry.uid,
                "name": entry.name,
                "kind": kind
            })
        })
        .collect();
    Ok(json!({
        "format": "quarxtor.manifest.v1",
        "created_at_ms": now_ms(),
        "repo_path": repo_path.display().to_string(),
        "root_id": root_id,
        "stats": {
            "blocks": blocks,
            "dirs": stats.dirs,
            "files": stats.files,
            "bytes": stats.bytes
        },
        "entries": entries_json
    }))
}

fn compute_compress_analysis(repo_path: &Path) -> Result<CompressAnalysis, CmdError> {
    let stats = collect_repo_stats(repo_path)?;
    let estimated = if stats.bytes == 0 {
        0
    } else {
        stats.bytes / 10 + stats.bytes / 20
    };
    Ok(CompressAnalysis {
        bytes_before: stats.bytes,
        dirs: stats.dirs,
        files: stats.files,
        blocks: stats.blocks,
        estimated_space_saved_bytes: estimated,
    })
}

fn resolve_repo_path(override_path: Option<PathBuf>) -> PathBuf {
    if let Some(path) = override_path {
        return path;
    }
    if let Ok(cfg) = EffectiveConfig::load_default() {
        if let Some(repo_root) = cfg.repo_root {
            return PathBuf::from(repo_root);
        }
    }
    PathBuf::from("/tmp/quarxtor/repo.qblk")
}

fn service_endpoint() -> String {
    if let Ok(cfg) = EffectiveConfig::load_default() {
        if cfg.net_transport == "unix" {
            return format!("unix://{}", cfg.net_unix_path);
        }
        return format!("tcp://{}", cfg.net_addr);
    }
    "unknown".to_string()
}

fn ensure_config_file() -> Result<PathBuf, CmdError> {
    let path = config_ini_path().unwrap_or_else(|| PathBuf::from("/tmp/quarxtor/quarxctl.ini"));
    if path.exists() {
        return Ok(path);
    }
    if let Ok(created) = write_full_config(false) {
        return Ok(created);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| CmdError::io(e.to_string()))?;
    }
    fs::write(&path, DEFAULT_CONFIG_TEXT).map_err(|e| CmdError::io(e.to_string()))?;
    Ok(path)
}

fn effective_value_for_key(cfg: &EffectiveConfig, section: &str, key: &str) -> String {
    match (section, key) {
        ("core", "l0_chunk") => cfg.l0_chunk.to_string(),
        ("core", "ram.limit") => cfg.ram_limit.clone(),
        ("core", "analysis.enable_znode") => cfg.analysis_enable_znode.to_string(),
        ("core", "analysis.fs_stats_fallback") => cfg.analysis_fs_stats_fallback.to_string(),
        ("core", "repo_root") => cfg.repo_root.clone().unwrap_or_default(),
        ("import", "root") => cfg.dataset_root.clone(),
        ("import", "min_file_size") => cfg.import_min_file_size.to_string(),
        ("import", "skip_hidden") => cfg.import_skip_hidden.to_string(),
        ("import", "skip_symlink") => cfg.import_skip_symlink.to_string(),
        ("import", "skip_zero") => cfg.import_skip_zero.to_string(),
        ("import", "skip_devices") => cfg.import_skip_devices.to_string(),
        ("import", "skip_special") => cfg.import_skip_special.to_string(),
        ("import", "include_glob") => cfg.include_glob.clone().unwrap_or_default(),
        ("import", "exclude_glob") => cfg.exclude_glob.clone().unwrap_or_default(),
        ("import", "include_dir_glob") => cfg.include_dir_glob.clone().unwrap_or_default(),
        ("import", "exclude_dir_glob") => cfg.exclude_dir_glob.clone().unwrap_or_default(),
        ("import", "import_glob_mode") => cfg.import_glob_mode.clone(),
        ("import", "use_z") => cfg.fs_import_use_z.to_string(),
        ("import", "z_threshold") => cfg.fs_import_z_threshold.to_string(),
        ("paths", "data_dir") => cfg.paths_data_dir.clone().unwrap_or_default(),
        ("paths", "cache_dir") => cfg.paths_cache_dir.clone().unwrap_or_default(),
        ("paths", "log_dir") => cfg.paths_log_dir.clone().unwrap_or_default(),
        ("paths", "runtime_dir") => cfg.paths_runtime_dir.clone().unwrap_or_default(),
        ("ui", "lang") => std::env::var("QUARX_LANG").unwrap_or_else(|_| "en-US".to_string()),
        ("net", "addr") => cfg.net_addr.clone(),
        ("net", "transport") => cfg.net_transport.clone(),
        ("net", "unix_path") => cfg.net_unix_path.clone(),
        ("drive", "mountpoint") => cfg.drive_mountpoint.clone().unwrap_or_default(),
        ("drive", "strategy") => cfg.drive_strategy.clone().unwrap_or_default(),
        ("drive", "fs_cache_bytes") => cfg.drive_fs_cache_bytes.to_string(),
        ("daemon", "enable_netd") => cfg.daemon_enable_netd.to_string(),
        ("cloud", "seed_nodes") => cfg.cloud_seed_nodes.clone().unwrap_or_default(),
        _ => String::new(),
    }
}

fn split_config_key(key: &str) -> Option<(&str, &str)> {
    let mut it = key.splitn(2, '.');
    let section = it.next()?;
    let name = it.next()?;
    if section.is_empty() || name.is_empty() {
        return None;
    }
    Some((section, name))
}

fn render_ini_sections(sections: &BTreeMap<String, BTreeMap<String, String>>) -> String {
    let preferred = [
        "core", "import", "paths", "ui", "net", "drive", "daemon", "cloud",
    ];
    let mut out = String::new();
    for sec in preferred {
        if let Some(kv) = sections.get(sec) {
            out.push_str(&format!("[{}]\n", sec));
            let mut keys: Vec<_> = kv.keys().cloned().collect();
            keys.sort();
            for key in keys {
                if let Some(val) = kv.get(&key) {
                    out.push_str(&format!("{} = {}\n", key, val));
                }
            }
            out.push('\n');
        }
    }
    for (sec, kv) in sections {
        if preferred.contains(&sec.as_str()) {
            continue;
        }
        out.push_str(&format!("[{}]\n", sec));
        let mut keys: Vec<_> = kv.keys().cloned().collect();
        keys.sort();
        for key in keys {
            if let Some(val) = kv.get(&key) {
                out.push_str(&format!("{} = {}\n", key, val));
            }
        }
        out.push('\n');
    }
    out
}

fn state_path_for(repo_path: &Path) -> PathBuf {
    with_extension(repo_path, "qstate.json")
}

fn load_state(repo_path: &Path) -> Result<QuarxState, CmdError> {
    let path = state_path_for(repo_path);
    if !path.exists() {
        return Ok(QuarxState::default());
    }
    let text = fs::read_to_string(&path).map_err(|e| CmdError::io(e.to_string()))?;
    let parsed: QuarxState = serde_json::from_str(&text)
        .map_err(|e| CmdError::state(format!("state parse error: {e}")))?;
    Ok(parsed)
}

fn save_state(repo_path: &Path, state: &QuarxState) -> Result<(), CmdError> {
    let path = state_path_for(repo_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| CmdError::io(e.to_string()))?;
    }
    let text = serde_json::to_string_pretty(state)
        .map_err(|e| CmdError::internal(format!("state encode error: {e}")))?;
    fs::write(path, text).map_err(|e| CmdError::io(e.to_string()))
}

#[derive(Debug, Default)]
struct ParsedOptions {
    positionals: Vec<String>,
    options: BTreeMap<String, String>,
}

fn parse_options(args: &[String]) -> ParsedOptions {
    let mut out = ParsedOptions::default();
    let mut i = 0usize;
    while i < args.len() {
        let arg = &args[i];
        if let Some(stripped) = arg.strip_prefix("--") {
            if let Some((k, v)) = stripped.split_once('=') {
                out.options.insert(k.to_string(), v.to_string());
                i += 1;
                continue;
            }
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                out.options
                    .insert(stripped.to_string(), args[i + 1].clone());
                i += 2;
                continue;
            }
            out.options.insert(stripped.to_string(), "true".to_string());
            i += 1;
            continue;
        }
        out.positionals.push(arg.clone());
        i += 1;
    }
    out
}

fn find_mount<'a>(mounts: &'a [MountRecord], target: &str) -> Option<&'a MountRecord> {
    if let Ok(id) = target.parse::<u64>() {
        return mounts.iter().find(|m| m.id == id);
    }
    mounts.iter().find(|m| m.name == target)
}

fn find_mount_mut<'a>(mounts: &'a mut [MountRecord], target: &str) -> Option<&'a mut MountRecord> {
    if let Ok(id) = target.parse::<u64>() {
        return mounts.iter_mut().find(|m| m.id == id);
    }
    mounts.iter_mut().find(|m| m.name == target)
}

fn read_json_file(path: &Path) -> Result<Value, CmdError> {
    let text =
        fs::read_to_string(path).map_err(|e| CmdError::io(format!("{}: {e}", path.display())))?;
    serde_json::from_str(&text)
        .map_err(|e| CmdError::compat(format!("{}: invalid json: {e}", path.display())))
}

fn write_json_file(path: &Path, value: &Value) -> Result<(), CmdError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| CmdError::io(e.to_string()))?;
    }
    let text = serde_json::to_string_pretty(value)
        .map_err(|e| CmdError::internal(format!("json encode error: {e}")))?;
    fs::write(path, text).map_err(|e| CmdError::io(format!("{}: {e}", path.display())))
}

fn with_extension(path: &Path, ext: &str) -> PathBuf {
    let mut p = path.to_path_buf();
    p.set_extension(ext);
    p
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path
        .file_stem()
        .and_then(|v| v.to_str())
        .unwrap_or("snapshot");
    let ext = path.extension().and_then(|v| v.to_str()).unwrap_or("json");
    let mut out = path.to_path_buf();
    out.set_file_name(format!("{stem}.{suffix}.{ext}"));
    out
}

fn default_artifact_path(repo_path: &Path, prefix: &str, ext: &str) -> PathBuf {
    let now = now_ms();
    let parent = repo_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    parent.join(format!("{prefix}-{now}.{ext}"))
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn tail(items: &[String], limit: usize) -> Vec<String> {
    if items.len() <= limit {
        return items.to_vec();
    }
    items[items.len() - limit..].to_vec()
}

fn parse_bool(raw: &str, default: bool) -> bool {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => true,
        "0" | "false" | "no" | "off" => false,
        _ => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_help_is_available_for_all() {
        for (ns, _) in NAMESPACE_SUBCOMMANDS {
            let out = run_namespace(ns, &[String::from("--help")]);
            assert_eq!(out.exit_code, 0, "namespace={ns}");
            let text = out.stdout.unwrap_or_default();
            assert!(text.contains("Usage:"), "namespace={ns}");
        }
    }

    #[test]
    fn mount_create_and_list_works() {
        let repo = PathBuf::from(format!("/tmp/quarxtor-test-{}-repo.qblk", now_ms()));
        let create = run_namespace(
            "mount",
            &[
                "create".to_string(),
                "disk1".to_string(),
                "--repo".to_string(),
                repo.display().to_string(),
                "--output".to_string(),
                "json".to_string(),
            ],
        );
        assert_eq!(create.exit_code, 0);
        let list = run_namespace(
            "mount",
            &[
                "list".to_string(),
                "--repo".to_string(),
                repo.display().to_string(),
                "--output".to_string(),
                "json".to_string(),
            ],
        );
        assert_eq!(list.exit_code, 0);
        let payload = list.stdout.expect("json output");
        assert!(payload.contains("\"mounts\""));
        assert!(payload.contains("disk1"));
    }

    #[test]
    fn unknown_subcommand_returns_usage() {
        let out = run_namespace("cloud", &[String::from("oops")]);
        assert_eq!(out.exit_code, EXIT_USAGE);
        assert!(out
            .stderr
            .unwrap_or_default()
            .contains("unknown subcommand"));
    }
}
