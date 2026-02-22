use quarxtor::config::{EffectiveConfig, EffectiveConfigError};

#[test]
fn unix_transport_missing_net_addr_is_ok() {
    let text = r#"
[core]
repo_root = /tmp/quarxtor/repo.qblk

[import]
root = /tmp/quarxtor/data

[net]
transport = unix
unix_path = /tmp/quarxtor/net.sock
"#;
    let cfg = EffectiveConfig::from_ini_str_strict(text).expect("unix transport config");
    assert_eq!(cfg.net_transport, "unix");
}

#[test]
fn tcp_transport_missing_net_addr_is_invalid() {
    let text = r#"
[import]
root = /tmp/quarxtor/data

[net]
transport = tcp
addr =
"#;
    match EffectiveConfig::from_ini_str_strict(text) {
        Err(EffectiveConfigError::InvalidNetAddr(msg)) => {
            assert_eq!(msg, "net.addr must be host:port");
        }
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn unknown_key_is_invalid() {
    let text = r#"
[core]
l0_chunk = 16384
unknown_key = 1

[import]
root = /tmp/quarxtor/data
"#;
    match EffectiveConfig::from_ini_str_strict(text) {
        Err(EffectiveConfigError::UnknownKeys(keys)) => {
            assert!(keys.contains(&"core.unknown_key".to_string()));
        }
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn multi_missing_required_keys_are_sorted() {
    let text = r#"
[import]
root =

[net]
transport = tcp
addr =
"#;
    match EffectiveConfig::from_ini_str_strict(text) {
        Err(EffectiveConfigError::MissingDatasetRoot) => {}
        other => panic!("unexpected result: {other:?}"),
    }
}
