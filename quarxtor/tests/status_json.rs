use std::env;
use std::fs;
use std::sync::Mutex;

use quarxtor::preflight::{run_preflight, PreflightOptions};

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn status_json_includes_source_and_stable_keys() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let text = r#"
[core]
repo_root = /tmp/quarxtor/repo.qblk

[import]
root = /tmp/quarxtor/data

[net]
transport = unix
unix_path = /tmp/quarxtor/net.sock
"#;

    let temp_root = env::temp_dir().join(format!("quarxtor-test-{}", std::process::id()));
    let xdg_root = temp_root.join("xdg");
    let config_dir = xdg_root.join("quarxtor");
    fs::create_dir_all(&config_dir).expect("create config dir");
    fs::write(config_dir.join("quarxctl.ini"), text).expect("write config");

    let prev = env::var("XDG_CONFIG_HOME").ok();
    env::set_var("XDG_CONFIG_HOME", &xdg_root);

    let result = run_preflight(PreflightOptions {
        repo_override: None,
        import_root_override: None,
    });

    if let Some(value) = prev {
        env::set_var("XDG_CONFIG_HOME", value);
    } else {
        env::remove_var("XDG_CONFIG_HOME");
    }

    let json = result.report.to_json();
    let value: serde_json::Value = serde_json::from_str(&json).expect("status json");
    let deps = value["dependencies"]
        .as_array()
        .expect("dependencies array");

    assert_eq!(deps[0]["name"], "config");
    assert!(deps[0]["source"].is_string() || deps[0]["source"].is_null());

    let obj = deps[0].as_object().expect("dependency object");
    for key in ["name", "state", "path", "reason", "required", "source"] {
        assert!(obj.contains_key(key), "missing key {key}");
    }

    let name_pos = json.find("\"name\"").expect("name in json");
    let state_pos = json.find("\"state\"").expect("state in json");
    assert!(name_pos < state_pos);
}
