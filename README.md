# QuarxTor-DAG

QuarxTor-DAG is a fresh, DAG-governed reboot of QuarXTor.

## Control Model
- Structural gates: `docs/control/ROADMAP-MAP.md`
- Operational cursor: `docs/control/ROADMAP-EXEC.md`
- Append-only transitions: `docs/control/ROADMAP-LEDGER.ndjson`

## Current Seed Runtime
Active workspace crates:
- `core-rs`
- `quarxproto`
- `quarxdrive`
- `quarxnet`
- `quarxcloud`
- `quarxtor`
- `tools/quarxctl`
- `tools/quarx-inspect`

Current baseline user entrypoint:
- `quarxtor import`
- `quarxtor status`
- operational namespaces: `config`, `repo`, `service`, `mount`, `dashboard`, `cloud`, `compress`, `export`, `convert`, `version`, `diag`

## Validation
```bash
python3 tools/control_check.py
cargo test --workspace
```

## Codex Handoff
```bash
cd /home/jee/repos/QuarxTor-DAG
codex
```

`AGENTS.md` and `docs/control/*` in this repository are the active control surface.
Legacy planning/docs from the previous repo are preserved under `legacy/` for reference only.
