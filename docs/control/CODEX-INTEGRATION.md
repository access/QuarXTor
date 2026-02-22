# CODEX-INTEGRATION

This repository uses DAG control artifacts for Codex collaboration.

## Startup
1. Read `AGENTS.md`.
2. Read `docs/control/ROADMAP-EXEC.md`.
3. Validate with `python3 tools/control_check.py`.

## Working Rules
- Update structural intent only in MAP.
- Update operational cursor only in EXEC.
- Record transitions/decisions only in LEDGER.

## Validation
- `python3 tools/control_check.py`
- `cargo test --workspace` for runtime changes

## Ledger Event Types
- `gate_transition`
- `decision`
- `note`
