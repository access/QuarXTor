# ROADMAP-EXEC (Execution Cursor)

This is the only live operational document.

Rules:
- Do not claim closures here.
- Closures are ledger transitions only.
- Update this file whenever cursor changes.

## Cursor
- Focus: post-`G-07` hardening (compat tests, UX refinement, production cloud transport).
- Last known good: commit where `tools/control_check.py` passes.
- Blockers: none recorded.

## Resume
1. Run `python3 tools/control_check.py`.
2. Run `cargo test --workspace`.
3. If either fails, fix before claiming progress.
4. Append ledger events for transitions/decisions.

## Expected Signals
- `tools/control_check.py` exits 0.
- Workspace tests pass for seed scope.
- Ledger stays valid NDJSON and append-only.

## Rollback
- If MAP/EXEC/LEDGER roles are mixed, restore to last good commit and re-apply via append-only ledger transitions.
