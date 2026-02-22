# AGENTS.md (Codex Authority) -- QuarxTor-DAG (MANDATORY)

## 1) Role of Codex
- Codex is an execution agent operating under repository contracts.
- Codex must not invent side workflows outside the control system.
- Codex performs work only through the DAG control artifacts.

## 2) Mandatory Read Order
1. `docs/control/ROADMAP-EXEC.md`
2. `docs/control/GOVERNANCE.md`
3. `docs/control/ROADMAP-MAP.md`
4. `docs/control/STATUS-MODEL.md`
5. `docs/control/DEPENDENCY-REGISTRY.md`
6. `docs/control/GLOSSARY.md`

## 3) Hard Rules
- English-only engineering artifacts.
- Separation invariant:
  - MAP = structural gates only.
  - EXEC = operational cursor only.
  - LEDGER = append-only transitions/decisions.
- No "done" claims outside LEDGER transitions.
- `gate_transition` to `closed` requires non-empty `evidence[]` and `commit`.
- Contract changes require explicit ledger decision and reopen/regressed transitions.
- Dependency changes require `docs/control/DEPENDENCY-REGISTRY.md` update.

## 4) Repository-Only Constraint
- All Codex state/workflow files must remain inside this repository.
- Do not write global Codex state in user home.

## 5) Preflight on Every Run
1. `git status --porcelain=v1`
2. `git diff --name-only`
3. Follow `docs/control/ROADMAP-EXEC.md`
4. Run `python3 tools/control_check.py` before closure claims

## 6) Scope Rule
- Primary entrypoint is `quarxtor`.
- Other binaries/crates are internal modules until promoted by gate closure.

See `docs/control/CODEX-INTEGRATION.md` for operational examples.
