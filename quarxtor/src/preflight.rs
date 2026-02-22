use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use quarxtor_core::{
    config_contract, config_ini_path, parse_ini_sections, validate_config_path, ConfigKey,
    ConfigValueType, KeyRole,
};

/// Preflight status values are stable: ok, missing, invalid, disabled, unavailable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepStatus {
    Ok,
    Missing,
    Invalid,
    Disabled,
    Unavailable,
}

impl DepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DepStatus::Ok => "ok",
            DepStatus::Missing => "missing",
            DepStatus::Invalid => "invalid",
            DepStatus::Disabled => "disabled",
            DepStatus::Unavailable => "unavailable",
        }
    }
}

pub struct DepReport {
    pub name: &'static str,
    pub status: DepStatus,
    pub path: Option<PathBuf>,
    pub reason: String,
    pub required: bool,
    pub source: Option<String>,
}

impl DepReport {
    fn line(&self) -> String {
        let mut line = format!("{:<16}: {:<9}", self.name, self.status.as_str());
        if let Some(path) = &self.path {
            line.push_str(&format!(" path={}", path.display()));
        }
        let mut reason = self.reason.clone();
        if let Some(source) = &self.source {
            let human = source_human(source);
            if !reason.is_empty() {
                reason.push_str(&format!(" (from {human})"));
            } else {
                reason.push_str(&format!("from {human}"));
            }
        }
        if !reason.is_empty() {
            line.push_str(&format!(" reason={reason}"));
        }
        line
    }
}

pub struct PreflightReport {
    pub deps: Vec<DepReport>,
}

impl PreflightReport {
    pub fn required_ok(&self) -> bool {
        self.deps
            .iter()
            .filter(|d| d.required)
            .all(|d| d.status == DepStatus::Ok)
    }

    pub fn missing_required_only_repo(&self) -> bool {
        let mut missing_repo = false;
        for dep in &self.deps {
            if !dep.required {
                continue;
            }
            if dep.status == DepStatus::Ok {
                continue;
            }
            if dep.name == "repo" && dep.status == DepStatus::Missing {
                missing_repo = true;
                continue;
            }
            return false;
        }
        missing_repo
    }

    pub fn print_human(&self) {
        for dep in &self.deps {
            println!("{}", dep.line());
        }
    }

    pub fn to_json(&self) -> String {
        let mut dep_list = Vec::new();
        for dep in &self.deps {
            let name = json_escape(dep.name);
            let state = json_escape(dep.status.as_str());
            let path = dep
                .path
                .as_ref()
                .map(|p| json_escape(&p.display().to_string()))
                .unwrap_or_else(|| "null".to_string());
            let reason = json_escape(&dep.reason);
            let required = if dep.required { "true" } else { "false" };
            let source = dep
                .source
                .as_ref()
                .map(|s| json_escape(s))
                .unwrap_or_else(|| "null".to_string());

            dep_list.push(format!(
                "{{\"name\":{name},\"state\":{state},\"path\":{path},\"reason\":{reason},\"required\":{required},\"source\":{source}}}"
            ));
        }
        format!("{{\"dependencies\":[{}]}}", dep_list.join(","))
    }
}

pub struct PreflightOptions {
    pub repo_override: Option<PathBuf>,
    pub import_root_override: Option<PathBuf>,
}

pub struct PreflightResult {
    pub report: PreflightReport,
    pub repo_path: PathBuf,
}

#[derive(Clone, Debug)]
struct ResolvedKey {
    value: Option<String>,
    source: Option<String>,
    explicit: bool,
}

#[derive(Clone, Debug)]
struct ConfigValidation {
    unknown_keys: Vec<String>,
    missing_keys: Vec<String>,
    invalid_keys: BTreeMap<String, String>,
    resolved: BTreeMap<String, ResolvedKey>,
    transport: String,
}

impl ConfigValidation {
    fn has_errors(&self) -> bool {
        !self.unknown_keys.is_empty()
            || !self.missing_keys.is_empty()
            || !self.invalid_keys.is_empty()
    }

    fn reason(&self) -> String {
        let mut parts = Vec::new();
        if !self.unknown_keys.is_empty() {
            parts.push(format!("unknown keys: {}", join_list(&self.unknown_keys)));
        }
        if !self.invalid_keys.is_empty() {
            parts.push(format!(
                "invalid keys: {}",
                join_invalid(&self.invalid_keys)
            ));
        }
        if !self.missing_keys.is_empty() {
            parts.push(format!("missing keys: {}", join_list(&self.missing_keys)));
        }
        if parts.is_empty() {
            "ok".to_string()
        } else {
            parts.join("; ")
        }
    }

    fn resolved_key(&self, full_key: &str) -> Option<&ResolvedKey> {
        self.resolved.get(full_key)
    }

    fn invalid_reason(&self, full_key: &str) -> Option<&str> {
        self.invalid_keys.get(full_key).map(|s| s.as_str())
    }

    fn is_missing(&self, full_key: &str) -> bool {
        self.missing_keys.iter().any(|k| k == full_key)
    }
}

pub fn run_preflight(opts: PreflightOptions) -> PreflightResult {
    let (config_check, cfg) = check_config(opts.import_root_override.as_deref());
    let cfg_ref = cfg.as_ref();
    let config_path = config_check.path.clone();

    let (data_dir, data_source) = resolve_data_dir(cfg_ref);
    let (cache_dir, cache_source) = resolve_cache_dir(cfg_ref);
    let (runtime_dir, runtime_source) = resolve_runtime_dir(cfg_ref);

    let (repo_path, repo_source) = resolve_repo_path(opts.repo_override, cfg_ref);

    let mut deps = Vec::new();
    deps.push(config_check);
    deps.push(check_repo(&repo_path, repo_source));
    deps.push(check_dir("data_dir", &data_dir, true, data_source, true));
    deps.push(check_dir("cache_dir", &cache_dir, true, cache_source, true));
    deps.push(check_dir(
        "runtime_dir",
        &runtime_dir,
        true,
        runtime_source,
        true,
    ));
    deps.push(check_daemon_endpoint(cfg_ref));
    deps.push(check_net_stack(cfg_ref, config_path.as_deref()));
    deps.push(check_fuse());
    deps.push(check_drive_mountpoint(cfg_ref));
    deps.push(check_daemon_enable_netd(cfg_ref));
    deps.push(check_cloud_seed_nodes(cfg_ref));

    PreflightResult {
        report: PreflightReport { deps },
        repo_path,
    }
}

fn check_config(import_root_override: Option<&Path>) -> (DepReport, Option<ConfigValidation>) {
    let path = match config_ini_path() {
        Some(p) => p,
        None => {
            return (
                DepReport {
                    name: "config",
                    status: DepStatus::Missing,
                    path: None,
                    reason: "config base not resolved".to_string(),
                    required: true,
                    source: None,
                },
                None,
            );
        }
    };

    let md = match fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            let status = if e.kind() == std::io::ErrorKind::NotFound {
                DepStatus::Missing
            } else {
                DepStatus::Unavailable
            };
            return (
                DepReport {
                    name: "config",
                    status,
                    path: Some(path),
                    reason: if e.kind() == std::io::ErrorKind::NotFound {
                        "missing required config file".to_string()
                    } else {
                        e.to_string()
                    },
                    required: true,
                    source: None,
                },
                None,
            );
        }
    };

    if !md.is_file() {
        return (
            DepReport {
                name: "config",
                status: DepStatus::Invalid,
                path: Some(path),
                reason: "expected file".to_string(),
                required: true,
                source: None,
            },
            None,
        );
    }

    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            let status = DepStatus::Unavailable;
            return (
                DepReport {
                    name: "config",
                    status,
                    path: Some(path),
                    reason: e.to_string(),
                    required: true,
                    source: None,
                },
                None,
            );
        }
    };

    let validation = validate_config_text(&text, import_root_override);
    let status = if validation.has_errors() {
        DepStatus::Invalid
    } else {
        DepStatus::Ok
    };
    let reason = if status == DepStatus::Ok {
        "ok".to_string()
    } else {
        validation.reason()
    };

    (
        DepReport {
            name: "config",
            status,
            path: Some(path),
            reason,
            required: true,
            source: None,
        },
        Some(validation),
    )
}

fn validate_config_text(text: &str, import_root_override: Option<&Path>) -> ConfigValidation {
    let parsed = parse_ini_sections(text);
    let mut unknown_keys = parsed.unknown_keys;
    unknown_keys.sort();
    unknown_keys.dedup();

    let mut resolved: BTreeMap<String, ResolvedKey> = BTreeMap::new();
    for key in config_contract() {
        let full_key = format!("{}.{}", key.section, key.key);
        let resolved_key = resolve_key_value(&parsed.sections, key, import_root_override);
        resolved.insert(full_key, resolved_key);
    }

    let mut missing_keys: Vec<String> = Vec::new();
    let mut invalid_keys: BTreeMap<String, String> = BTreeMap::new();

    for key in config_contract() {
        let full_key = format!("{}.{}", key.section, key.key);
        let resolved_key = resolved.get(&full_key).expect("resolved key missing");

        if matches!(key.role, KeyRole::Required) && !resolved_key.explicit {
            missing_keys.push(full_key.clone());
        }

        if resolved_key.explicit {
            if let Some(value) = resolved_key.value.as_deref() {
                if let Err(reason) = validate_value(key, value) {
                    invalid_keys.insert(full_key.clone(), reason);
                }
            }
        }
    }

    let transport = resolved
        .get("net.transport")
        .and_then(|r| r.value.as_deref())
        .unwrap_or("unix")
        .to_string();

    if transport == "tcp" {
        let net_addr = resolved.get("net.addr");
        let net_addr_explicit = net_addr.map(|r| r.explicit).unwrap_or(false);
        if !net_addr_explicit {
            missing_keys.push("net.addr".to_string());
        } else if let Some(value) = net_addr.and_then(|r| r.value.as_deref()) {
            if !is_host_port(value) {
                invalid_keys.insert("net.addr".to_string(), "must be host:port".to_string());
            }
        }
    }

    missing_keys.sort();
    missing_keys.dedup();

    ConfigValidation {
        unknown_keys,
        missing_keys,
        invalid_keys,
        resolved,
        transport,
    }
}

fn resolve_key_value(
    sections: &BTreeMap<String, BTreeMap<String, String>>,
    key: &ConfigKey,
    import_root_override: Option<&Path>,
) -> ResolvedKey {
    if key.section == "import" && key.key == "root" {
        if let Some(override_path) = import_root_override {
            return ResolvedKey {
                value: Some(override_path.display().to_string()),
                source: Some(arg_source("import.root")),
                explicit: true,
            };
        }
    }

    if let Some(env_key) = key.env_override {
        if let Some(val) = env_var(env_key) {
            return ResolvedKey {
                value: Some(val),
                source: Some(env_source(env_key)),
                explicit: true,
            };
        }
    }

    if let Some(sec) = sections.get(key.section) {
        if let Some(val) = sec.get(key.key) {
            return ResolvedKey {
                value: Some(val.clone()),
                source: Some(config_source(key.section, key.key)),
                explicit: true,
            };
        }
    }

    if matches!(key.role, KeyRole::RequiredWithDefault) && !key.default_value.is_empty() {
        return ResolvedKey {
            value: Some(key.default_value.to_string()),
            source: None,
            explicit: false,
        };
    }

    ResolvedKey {
        value: None,
        source: None,
        explicit: false,
    }
}

fn validate_value(key: &ConfigKey, value: &str) -> Result<(), String> {
    match key.value_type {
        ConfigValueType::Bool => {
            if parse_bool_value(value).is_some() {
                Ok(())
            } else {
                Err("invalid bool".to_string())
            }
        }
        ConfigValueType::U64 => {
            if parse_u64_value(value).is_some() {
                Ok(())
            } else {
                Err("invalid u64".to_string())
            }
        }
        ConfigValueType::Enum(options) => {
            if options.iter().any(|o| *o == value) {
                Ok(())
            } else {
                Err(format!("must be one of: {}", options.join("|")))
            }
        }
        ConfigValueType::Path => {
            validate_config_path(value, &format!("{}.{}", key.section, key.key))
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        ConfigValueType::String => Ok(()),
    }
}

fn check_repo(path: &Path, source: Option<String>) -> DepReport {
    if let Some(parent) = path.parent() {
        if fs::metadata(parent).is_err() {
            if let Err(e) = fs::create_dir_all(parent) {
                return DepReport {
                    name: "repo",
                    status: DepStatus::Unavailable,
                    path: Some(path.to_path_buf()),
                    reason: e.to_string(),
                    required: true,
                    source,
                };
            }
        }
    }

    let md = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            let status = if e.kind() == std::io::ErrorKind::NotFound {
                DepStatus::Missing
            } else {
                DepStatus::Unavailable
            };
            return DepReport {
                name: "repo",
                status,
                path: Some(path.to_path_buf()),
                reason: if e.kind() == std::io::ErrorKind::NotFound {
                    "missing required repo file".to_string()
                } else {
                    e.to_string()
                },
                required: true,
                source,
            };
        }
    };

    if !md.is_file() {
        return DepReport {
            name: "repo",
            status: DepStatus::Invalid,
            path: Some(path.to_path_buf()),
            reason: "expected file".to_string(),
            required: true,
            source,
        };
    }

    match quarxtor_core::store::file_store::FileBlockStore::open(path.to_path_buf()) {
        Ok(_) => DepReport {
            name: "repo",
            status: DepStatus::Ok,
            path: Some(path.to_path_buf()),
            reason: "ok".to_string(),
            required: true,
            source,
        },
        Err(quarxtor_core::store::blockstore::StoreError::Io(e)) => DepReport {
            name: "repo",
            status: DepStatus::Unavailable,
            path: Some(path.to_path_buf()),
            reason: e.to_string(),
            required: true,
            source,
        },
        Err(e) => DepReport {
            name: "repo",
            status: DepStatus::Invalid,
            path: Some(path.to_path_buf()),
            reason: format!("{e:?}"),
            required: true,
            source,
        },
    }
}

fn check_dir(
    name: &'static str,
    path: &Path,
    required: bool,
    source: Option<String>,
    create_if_missing: bool,
) -> DepReport {
    let mut created = false;
    let md = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) => {
            if create_if_missing && e.kind() == std::io::ErrorKind::NotFound {
                match fs::create_dir_all(path) {
                    Ok(()) => {
                        created = true;
                        match fs::metadata(path) {
                            Ok(m) => m,
                            Err(e) => {
                                return DepReport {
                                    name,
                                    status: DepStatus::Unavailable,
                                    path: Some(path.to_path_buf()),
                                    reason: e.to_string(),
                                    required,
                                    source,
                                };
                            }
                        }
                    }
                    Err(e) => {
                        let status = DepStatus::Unavailable;
                        return DepReport {
                            name,
                            status,
                            path: Some(path.to_path_buf()),
                            reason: format!("create failed: {e}"),
                            required,
                            source,
                        };
                    }
                }
            } else {
                let status = if e.kind() == std::io::ErrorKind::NotFound {
                    DepStatus::Missing
                } else {
                    DepStatus::Unavailable
                };
                return DepReport {
                    name,
                    status,
                    path: Some(path.to_path_buf()),
                    reason: e.to_string(),
                    required,
                    source,
                };
            }
        }
    };

    if !md.is_dir() {
        return DepReport {
            name,
            status: DepStatus::Invalid,
            path: Some(path.to_path_buf()),
            reason: "expected directory".to_string(),
            required,
            source,
        };
    }

    match fs::read_dir(path) {
        Ok(_) => DepReport {
            name,
            status: DepStatus::Ok,
            path: Some(path.to_path_buf()),
            reason: if created {
                "created".to_string()
            } else {
                "ok".to_string()
            },
            required,
            source,
        },
        Err(e) => DepReport {
            name,
            status: DepStatus::Unavailable,
            path: Some(path.to_path_buf()),
            reason: e.to_string(),
            required,
            source,
        },
    }
}

fn check_daemon_endpoint(cfg: Option<&ConfigValidation>) -> DepReport {
    let default_transport = if cfg!(target_os = "windows") {
        "tcp"
    } else {
        "unix"
    };
    if let Some(reason) = cfg.and_then(|c| c.invalid_reason("net.transport")) {
        return DepReport {
            name: "daemon_endpoint",
            status: DepStatus::Invalid,
            path: None,
            reason: reason.to_string(),
            required: false,
            source: cfg
                .and_then(|c| c.resolved_key("net.transport"))
                .and_then(|r| r.source.clone()),
        };
    }
    let transport = cfg
        .map(|c| c.transport.as_str())
        .unwrap_or(default_transport);

    if transport == "unix" {
        if cfg!(target_os = "windows") {
            return DepReport {
                name: "daemon_endpoint",
                status: DepStatus::Unavailable,
                path: None,
                reason: "unix sockets unsupported".to_string(),
                required: false,
                source: None,
            };
        }

        let (path, source, invalid_reason) = config_path_value(cfg, "net.unix_path");
        if let Some(reason) = invalid_reason {
            return DepReport {
                name: "daemon_endpoint",
                status: DepStatus::Invalid,
                path: path.map(PathBuf::from),
                reason,
                required: false,
                source,
            };
        }

        let path = path
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/tmp/quarxtor/quarxnetd.sock"));
        if path.exists() {
            DepReport {
                name: "daemon_endpoint",
                status: DepStatus::Ok,
                path: Some(path),
                reason: "ok".to_string(),
                required: false,
                source,
            }
        } else {
            DepReport {
                name: "daemon_endpoint",
                status: DepStatus::Missing,
                path: Some(path),
                reason: "socket not found".to_string(),
                required: false,
                source,
            }
        }
    } else if transport == "tcp" {
        let (addr, source, invalid_reason) = config_value(cfg, "net.addr");
        if let Some(reason) = invalid_reason {
            return DepReport {
                name: "daemon_endpoint",
                status: DepStatus::Invalid,
                path: None,
                reason,
                required: false,
                source,
            };
        }
        if cfg.map(|c| c.is_missing("net.addr")).unwrap_or(false) {
            return DepReport {
                name: "daemon_endpoint",
                status: DepStatus::Invalid,
                path: None,
                reason: "missing net.addr".to_string(),
                required: false,
                source,
            };
        }
        let addr = addr.unwrap_or_else(|| "0.0.0.0:54721".to_string());
        DepReport {
            name: "daemon_endpoint",
            status: DepStatus::Unavailable,
            path: None,
            reason: format!("tcp endpoint not probed ({addr})"),
            required: false,
            source,
        }
    } else {
        DepReport {
            name: "daemon_endpoint",
            status: DepStatus::Invalid,
            path: None,
            reason: "invalid transport".to_string(),
            required: false,
            source: None,
        }
    }
}

fn check_net_stack(cfg: Option<&ConfigValidation>, config_path: Option<&Path>) -> DepReport {
    let default_transport = if cfg!(target_os = "windows") {
        "tcp"
    } else {
        "unix"
    };
    let transport = cfg
        .map(|c| c.transport.as_str())
        .unwrap_or(default_transport);
    let source = cfg
        .and_then(|c| c.resolved_key("net.transport"))
        .and_then(|r| r.source.clone());

    if let Some(reason) = cfg.and_then(|c| c.invalid_reason("net.transport")) {
        return DepReport {
            name: "net_stack",
            status: DepStatus::Invalid,
            path: config_path.map(PathBuf::from),
            reason: reason.to_string(),
            required: false,
            source,
        };
    }

    if transport != "tcp" && transport != "unix" {
        return DepReport {
            name: "net_stack",
            status: DepStatus::Invalid,
            path: config_path.map(PathBuf::from),
            reason: "invalid transport".to_string(),
            required: false,
            source,
        };
    }

    if transport == "unix" {
        let status = if cfg!(target_os = "windows") {
            DepStatus::Unavailable
        } else {
            DepStatus::Ok
        };
        let reason = if cfg!(target_os = "windows") {
            "unix sockets unsupported".to_string()
        } else if cfg.is_some() {
            "unix transport configured".to_string()
        } else {
            "unix transport default".to_string()
        };
        return DepReport {
            name: "net_stack",
            status,
            path: config_path.map(PathBuf::from),
            reason,
            required: false,
            source,
        };
    }

    let addr_invalid = cfg
        .and_then(|c| c.invalid_reason("net.addr"))
        .map(|s| s.to_string());
    if let Some(reason) = addr_invalid {
        return DepReport {
            name: "net_stack",
            status: DepStatus::Invalid,
            path: config_path.map(PathBuf::from),
            reason,
            required: false,
            source,
        };
    }

    if cfg.map(|c| c.is_missing("net.addr")).unwrap_or(false) {
        return DepReport {
            name: "net_stack",
            status: DepStatus::Invalid,
            path: config_path.map(PathBuf::from),
            reason: "missing net.addr".to_string(),
            required: false,
            source,
        };
    }

    let reason = if cfg.is_some() {
        "tcp transport configured".to_string()
    } else {
        "tcp transport default".to_string()
    };

    DepReport {
        name: "net_stack",
        status: DepStatus::Ok,
        path: config_path.map(PathBuf::from),
        reason,
        required: false,
        source,
    }
}

fn check_fuse() -> DepReport {
    DepReport {
        name: "fuse",
        status: DepStatus::Unavailable,
        path: None,
        reason: "not enabled at build time".to_string(),
        required: false,
        source: None,
    }
}

fn check_drive_mountpoint(cfg: Option<&ConfigValidation>) -> DepReport {
    let (value, source, invalid_reason) = config_path_value(cfg, "drive.mountpoint");
    if let Some(reason) = invalid_reason {
        return DepReport {
            name: "drive_mountpoint",
            status: DepStatus::Invalid,
            path: value.map(PathBuf::from),
            reason,
            required: false,
            source,
        };
    }
    if let Some(path) = value {
        DepReport {
            name: "drive_mountpoint",
            status: DepStatus::Disabled,
            path: Some(PathBuf::from(path)),
            reason: "reserved in schema".to_string(),
            required: false,
            source,
        }
    } else {
        DepReport {
            name: "drive_mountpoint",
            status: DepStatus::Disabled,
            path: None,
            reason: "not configured".to_string(),
            required: false,
            source: None,
        }
    }
}

fn check_daemon_enable_netd(cfg: Option<&ConfigValidation>) -> DepReport {
    let (value, source, invalid_reason) = config_value(cfg, "daemon.enable_netd");
    if let Some(reason) = invalid_reason {
        return DepReport {
            name: "daemon_netd",
            status: DepStatus::Invalid,
            path: None,
            reason,
            required: false,
            source,
        };
    }
    if let Some(raw) = value {
        let enabled = parse_bool_value(&raw).unwrap_or(false);
        let reason = if enabled {
            "reserved in schema".to_string()
        } else {
            "disabled in config".to_string()
        };
        DepReport {
            name: "daemon_netd",
            status: DepStatus::Disabled,
            path: None,
            reason,
            required: false,
            source,
        }
    } else {
        DepReport {
            name: "daemon_netd",
            status: DepStatus::Disabled,
            path: None,
            reason: "not configured".to_string(),
            required: false,
            source: None,
        }
    }
}

fn check_cloud_seed_nodes(cfg: Option<&ConfigValidation>) -> DepReport {
    let (value, source, invalid_reason) = config_value(cfg, "cloud.seed_nodes");
    if let Some(reason) = invalid_reason {
        return DepReport {
            name: "cloud_seed_nodes",
            status: DepStatus::Invalid,
            path: None,
            reason,
            required: false,
            source,
        };
    }
    if let Some(raw) = value {
        if raw.trim().is_empty() {
            return DepReport {
                name: "cloud_seed_nodes",
                status: DepStatus::Disabled,
                path: None,
                reason: "not configured".to_string(),
                required: false,
                source: None,
            };
        }
        DepReport {
            name: "cloud_seed_nodes",
            status: DepStatus::Disabled,
            path: None,
            reason: "reserved in schema".to_string(),
            required: false,
            source,
        }
    } else {
        DepReport {
            name: "cloud_seed_nodes",
            status: DepStatus::Disabled,
            path: None,
            reason: "not configured".to_string(),
            required: false,
            source: None,
        }
    }
}

fn resolve_repo_path(
    repo_override: Option<PathBuf>,
    cfg: Option<&ConfigValidation>,
) -> (PathBuf, Option<String>) {
    if let Some(path) = repo_override {
        return (
            absolutize(normalize_repo_root_path(path)),
            Some(arg_source("--repo")),
        );
    }

    if let Some(v) = env_var("QUARX_REPO_ROOT") {
        return (
            absolutize(normalize_repo_root_path(PathBuf::from(v))),
            Some(env_source("QUARX_REPO_ROOT")),
        );
    }

    if let (Some(value), source, _) = config_value(cfg, "core.repo_root") {
        return (
            absolutize(normalize_repo_root_path(PathBuf::from(value))),
            source,
        );
    }

    if let Some(v) = env_var("QUARX_DATA_DIR") {
        return (
            absolutize(PathBuf::from(v).join("repo.qblk")),
            Some(env_source("QUARX_DATA_DIR")),
        );
    }
    if let (Some(value), source, _) = config_value(cfg, "paths.data_dir") {
        return (absolutize(PathBuf::from(value).join("repo.qblk")), source);
    }

    (
        absolutize(default_data_dir().join("quarxtor").join("repo.qblk")),
        None,
    )
}

fn normalize_repo_root_path(path: PathBuf) -> PathBuf {
    if let Ok(md) = fs::metadata(&path) {
        if md.is_dir() {
            return path.join("repo.qblk");
        }
        if md.is_file() {
            return path;
        }
    }
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if name.ends_with(".qblk") || name.ends_with(".hgss") {
        return path;
    }
    path.join("repo.qblk")
}

fn resolve_data_dir(cfg: Option<&ConfigValidation>) -> (PathBuf, Option<String>) {
    if let Some(v) = env_var("QUARX_DATA_DIR") {
        return (
            absolutize(PathBuf::from(v)),
            Some(env_source("QUARX_DATA_DIR")),
        );
    }
    if let (Some(value), source, _) = config_value(cfg, "paths.data_dir") {
        return (absolutize(PathBuf::from(value)), source);
    }
    (absolutize(default_data_dir().join("quarxtor")), None)
}

fn resolve_cache_dir(cfg: Option<&ConfigValidation>) -> (PathBuf, Option<String>) {
    if let Some(v) = env_var("QUARX_CACHE_DIR") {
        return (
            absolutize(PathBuf::from(v)),
            Some(env_source("QUARX_CACHE_DIR")),
        );
    }
    if let (Some(value), source, _) = config_value(cfg, "paths.cache_dir") {
        return (absolutize(PathBuf::from(value)), source);
    }
    (absolutize(default_cache_dir().join("quarxtor")), None)
}

fn resolve_runtime_dir(cfg: Option<&ConfigValidation>) -> (PathBuf, Option<String>) {
    if let Some(v) = env_var("QUARX_RUNTIME_DIR") {
        return (
            absolutize(PathBuf::from(v)),
            Some(env_source("QUARX_RUNTIME_DIR")),
        );
    }
    if let (Some(value), source, _) = config_value(cfg, "paths.runtime_dir") {
        return (absolutize(PathBuf::from(value)), source);
    }
    (absolutize(default_runtime_dir()), None)
}

fn config_value(
    cfg: Option<&ConfigValidation>,
    full_key: &str,
) -> (Option<String>, Option<String>, Option<String>) {
    match cfg {
        Some(c) => (
            c.resolved_key(full_key).and_then(|r| r.value.clone()),
            c.resolved_key(full_key).and_then(|r| r.source.clone()),
            c.invalid_reason(full_key).map(|s| s.to_string()),
        ),
        None => (None, None, None),
    }
}

fn config_path_value(
    cfg: Option<&ConfigValidation>,
    full_key: &str,
) -> (Option<String>, Option<String>, Option<String>) {
    config_value(cfg, full_key)
}

fn env_var(key: &str) -> Option<String> {
    let v = std::env::var(key).ok()?;
    let v = v.trim();
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

fn default_data_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(v) = env_var("LOCALAPPDATA").or_else(|| env_var("APPDATA")) {
            return PathBuf::from(v);
        }
    }

    if cfg!(target_os = "macos") {
        if let Some(home) = home_dir() {
            return home.join("Library").join("Application Support");
        }
    }

    if let Some(v) = env_var("XDG_DATA_HOME") {
        return PathBuf::from(v);
    }
    if let Some(home) = home_dir() {
        return home.join(".local").join("share");
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn default_cache_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(v) = env_var("LOCALAPPDATA").or_else(|| env_var("APPDATA")) {
            return PathBuf::from(v);
        }
    }

    if cfg!(target_os = "macos") {
        if let Some(home) = home_dir() {
            return home.join("Library").join("Caches");
        }
    }

    if let Some(v) = env_var("XDG_CACHE_HOME") {
        return PathBuf::from(v);
    }
    if let Some(home) = home_dir() {
        return home.join(".cache");
    }

    std::env::temp_dir().join("quarxtor").join("cache")
}

fn default_runtime_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Some(v) = env_var("LOCALAPPDATA").or_else(|| env_var("APPDATA")) {
            return PathBuf::from(v).join("QuarXTor").join("runtime");
        }
    }

    if cfg!(target_os = "macos") {
        if let Some(home) = home_dir() {
            return home
                .join("Library")
                .join("Application Support")
                .join("quarxtor")
                .join("run");
        }
    }

    if let Some(v) = env_var("XDG_RUNTIME_DIR") {
        return PathBuf::from(v).join("quarxtor");
    }

    std::env::temp_dir().join("quarxtor").join("run")
}

fn home_dir() -> Option<PathBuf> {
    if let Some(v) = env_var("HOME") {
        return Some(PathBuf::from(v));
    }
    env_var("USERPROFILE").map(PathBuf::from)
}

fn absolutize(path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        return path;
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(path),
        Err(_) => path,
    }
}

fn parse_bool_value(raw: &str) -> Option<bool> {
    let v = raw.trim().to_ascii_lowercase();
    match v.as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn parse_u64_value(raw: &str) -> Option<u64> {
    raw.trim().parse::<u64>().ok()
}

fn is_host_port(addr: &str) -> bool {
    let parts: Vec<&str> = addr.split(':').collect();
    if parts.len() < 2 {
        return false;
    }
    let port = parts.last().unwrap_or(&"");
    port.parse::<u16>().is_ok()
}

fn join_list(list: &[String]) -> String {
    list.join(",")
}

fn join_invalid(invalid: &BTreeMap<String, String>) -> String {
    let mut parts = Vec::new();
    for (key, reason) in invalid {
        parts.push(format!("{key} ({reason})"));
    }
    parts.join(",")
}

fn json_escape(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

fn source_human(source: &str) -> &str {
    source.strip_prefix("config:").unwrap_or(source)
}

fn config_source(section: &str, key: &str) -> String {
    format!("config:[{section}].{key}")
}

fn env_source(env: &str) -> String {
    format!("env:{env}")
}

fn arg_source(arg: &str) -> String {
    format!("arg:{arg}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_transport_missing_addr_is_ok() {
        let root = std::env::temp_dir().join("quarxtor-test-root");
        let sock = std::env::temp_dir().join("quarxtor.sock");
        let text = format!(
            "\
[import]
root={}
[net]
transport=unix
unix_path={}
",
            root.display(),
            sock.display()
        );
        let validation = validate_config_text(&text, None);
        assert!(validation.missing_keys.is_empty());
        assert!(validation.invalid_keys.is_empty());
        assert!(validation.unknown_keys.is_empty());
    }

    #[test]
    fn tcp_transport_requires_addr() {
        let root = std::env::temp_dir().join("quarxtor-test-root");
        let text = format!(
            "\
[import]
root={}
[net]
transport=tcp
",
            root.display()
        );
        let validation = validate_config_text(&text, None);
        assert!(validation.missing_keys.contains(&"net.addr".to_string()));

        let text = format!(
            "\
[import]
root={}
[net]
transport=tcp
addr=not-a-host
",
            root.display()
        );
        let validation = validate_config_text(&text, None);
        assert!(validation.invalid_keys.contains_key("net.addr"));
    }

    #[test]
    fn unknown_key_is_invalid() {
        let root = std::env::temp_dir().join("quarxtor-test-root");
        let text = format!(
            "\
[import]
root={}
[core]
bogus=1
",
            root.display()
        );
        let validation = validate_config_text(&text, None);
        assert_eq!(validation.unknown_keys, vec!["core.bogus".to_string()]);
    }

    #[test]
    fn missing_keys_are_sorted_and_deterministic() {
        let text = "\
[net]
transport=tcp
";
        let validation = validate_config_text(text, None);
        assert_eq!(
            validation.missing_keys,
            vec!["import.root".to_string(), "net.addr".to_string()]
        );
    }

    #[test]
    fn status_json_has_stable_keys_and_source() {
        let report = PreflightReport {
            deps: vec![DepReport {
                name: "config",
                status: DepStatus::Ok,
                path: None,
                reason: "ok".to_string(),
                required: true,
                source: Some("config:[core].l0_chunk".to_string()),
            }],
        };
        let json = report.to_json();
        let expected = "{\"dependencies\":[{\"name\":\"config\",\"state\":\"ok\",\"path\":null,\"reason\":\"ok\",\"required\":true,\"source\":\"config:[core].l0_chunk\"}]}";
        assert_eq!(json, expected);
    }
}
