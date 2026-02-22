use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use quarxtor_core::{config_ini_path, parse_ini_sections};

pub const REQUIRED_KEYS: &[&str] = &[
    "cli.usage.main",
    "cli.error.unknown_command",
    "cli.error.missing_argument",
    "cli.error.invalid_path",
    "cli.error.open_repo",
    "cli.error.import_failed",
    "cli.error.status_failed",
];

#[derive(Debug, Clone)]
pub struct I18n {
    map: HashMap<String, String>,
}

impl I18n {
    pub fn load() -> Self {
        let mut fallback = parse_locale(include_str!("../../i18n/en-US.toml"));
        if fallback.is_empty() {
            fallback = default_fallback_map();
        }

        let locale = desired_locale().unwrap_or_else(|| "en-US".to_string());
        if locale != "en-US" {
            if let Some(extra) = load_locale_file(&locale) {
                for (k, v) in extra {
                    fallback.insert(k, v);
                }
            }
        }

        ensure_keys(&mut fallback);
        Self { map: fallback }
    }

    pub fn t(&self, key: &str, vars: &[(&str, &str)]) -> String {
        let template = self
            .map
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string());
        render(&template, vars)
    }
}

fn ensure_keys(map: &mut HashMap<String, String>) {
    for key in REQUIRED_KEYS {
        map.entry((*key).to_string())
            .or_insert_with(|| (*key).to_string());
    }
}

fn desired_locale() -> Option<String> {
    if let Some(v) = config_locale() {
        return Some(v);
    }
    if let Ok(v) = std::env::var("QUARX_LANG") {
        let v = normalize_locale(&v);
        if !v.is_empty() {
            return Some(v);
        }
    }
    if let Ok(v) = std::env::var("LANG") {
        let v = normalize_locale(&v);
        if !v.is_empty() {
            return Some(v);
        }
    }
    None
}

fn config_locale() -> Option<String> {
    let path = config_ini_path()?;
    let text = fs::read_to_string(path).ok()?;
    let parsed = parse_ini_sections(&text);
    let raw = parsed.sections.get("ui")?.get("lang")?;
    let normalized = normalize_locale(raw);
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn normalize_locale(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let base = trimmed.split('.').next().unwrap_or("");
    base.replace('_', "-")
}

fn load_locale_file(locale: &str) -> Option<HashMap<String, String>> {
    let path = locale_path(locale);
    let text = fs::read_to_string(path).ok()?;
    let parsed = parse_locale(&text);
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

fn locale_path(locale: &str) -> PathBuf {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("i18n");
    base.join(format!("{locale}.toml"))
}

fn parse_locale(text: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let value = match text.parse::<toml::Value>() {
        Ok(v) => v,
        Err(_) => return map,
    };

    let table = match value {
        toml::Value::Table(t) => t,
        _ => return map,
    };

    for (k, v) in table {
        if let toml::Value::String(s) = v {
            map.insert(k, s);
        }
    }

    map
}

fn render(template: &str, vars: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (key, value) in vars {
        let token = format!("{{{key}}}");
        out = out.replace(&token, value);
    }
    out
}

fn default_fallback_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(
        "cli.usage.main".to_string(),
        "Usage:\n  {bin} --version\n  {bin} import <DIR|FILE> [--repo <path>]\n  {bin} status [--repo <path>] [--json]\n\nPlanned namespaces:\n  {bin} config|repo|service|mount|dashboard|cloud|compress|export|convert|version|diag\n\nFor planned namespace details:\n  {bin} <namespace> --help\n"
            .to_string(),
    );
    map.insert(
        "cli.error.unknown_command".to_string(),
        "Unknown command".to_string(),
    );
    map.insert(
        "cli.error.missing_argument".to_string(),
        "Missing argument".to_string(),
    );
    map.insert(
        "cli.error.invalid_path".to_string(),
        "Invalid path".to_string(),
    );
    map.insert(
        "cli.error.open_repo".to_string(),
        "Failed to open repo".to_string(),
    );
    map.insert(
        "cli.error.import_failed".to_string(),
        "Import failed".to_string(),
    );
    map.insert(
        "cli.error.status_failed".to_string(),
        "Status failed".to_string(),
    );
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_files_parse() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("i18n");
        let entries = fs::read_dir(base).expect("i18n dir missing");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|v| v.to_str()) != Some("toml") {
                continue;
            }
            let text = fs::read_to_string(&path).expect("locale read failed");
            let parsed = parse_locale(&text);
            assert!(
                !parsed.is_empty(),
                "locale {} did not parse",
                path.display()
            );
        }
    }

    #[test]
    fn default_locale_has_required_keys() {
        let parsed = parse_locale(include_str!("../../i18n/en-US.toml"));
        for key in REQUIRED_KEYS {
            assert!(parsed.contains_key(*key), "missing key in en-US: {key}");
        }
    }

    #[test]
    fn token_replacement_works() {
        let i18n = I18n {
            map: default_fallback_map(),
        };
        let rendered = i18n.t("cli.usage.main", &[("bin", "quarxtor")]);
        assert!(!rendered.contains("{bin}"));
        assert!(rendered.contains("quarxtor --version"));
    }

    #[test]
    fn locale_keys_are_namespaced() {
        let parsed = parse_locale(include_str!("../../i18n/en-US.toml"));
        for key in parsed.keys() {
            assert!(key.contains('.'), "key not namespaced: {key}");
        }
    }
}
