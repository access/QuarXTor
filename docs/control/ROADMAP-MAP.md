# ROADMAP-MAP (Structural Gates)

This file defines the stable DAG of readiness gates for QuarxTor-DAG.

Role:
- Gate IDs and dependencies.
- Definition of Done (DoD).
- Required evidence for closure.
- References to contracts and ADR/FRAG context.

Non-role:
- No execution cursor.
- No timestamps or history notes.
- No command playbooks.

Gate status is tracked only in `docs/control/ROADMAP-LEDGER.ndjson`.

## Gate DAG (overview)
- `G-00` CONTROL-CONSTITUTION
- `G-01` CONTRACT-REBASE (depends on `G-00`)
- `G-02` ENTRYPOINT-CONTRACT (depends on `G-01`)
- `G-03` RUNTIME-SEED-INTEGRITY (depends on `G-01`, `G-02`)
- `G-04` CLI-IO-CONTRACT (depends on `G-02`, `G-03`)
- `G-05` CLOUD-P2P-CONTRACT (depends on `G-02`, `G-04`)
- `G-06` OPTIMIZER-CONTRACT (depends on `G-03`, `G-04`)
- `G-07` GUI-JOURNEY-CONTRACT (depends on `G-04`, `G-05`, `G-06`)

---

## G-00 CONTROL-CONSTITUTION
Depends on: --

DoD:
- `docs/control/*` control set exists and passes validator.
- MAP/EXEC/LEDGER separation is explicit and enforced.
- Ledger closure rule (`closed` requires evidence+commit) is documented.

Evidence:
- `python3 tools/control_check.py` passes.
- Ledger contains a closing `gate_transition` for `G-00`.

## G-01 CONTRACT-REBASE
Depends on: `G-00`

DoD:
- Contract set exists and is internally consistent with seed runtime scope.
- Contract files define: architecture, product guarantees, format, config.
- Conflicting legacy claims are removed from active contracts.

Evidence:
- Contract files present and reviewed.
- Ledger close transition references reviewed files.

## G-02 ENTRYPOINT-CONTRACT
Depends on: `G-01`

DoD:
- Single primary entrypoint (`quarxtor`) is defined.
- Command namespace taxonomy is documented.
- Backward-compatibility/deprecation policy is documented.

Evidence:
- Entry point contract file and tests/CLI verification references.

## G-03 RUNTIME-SEED-INTEGRITY
Depends on: `G-01`, `G-02`

DoD:
- Seed workspace compiles.
- Seed workspace tests pass deterministically.
- Carried modules are explicitly listed and justified.

Evidence:
- `cargo test --workspace` output referenced in ledger.
- Seed inventory document reference.

## G-04 CLI-IO-CONTRACT
Depends on: `G-02`, `G-03`

DoD:
- Universal JSON envelope and text contract are defined.
- Stable exit-code taxonomy is defined.
- Snapshot/schema compatibility test policy is defined and implemented.

Evidence:
- Contract docs and test artifacts referenced in ledger.

## G-05 CLOUD-P2P-CONTRACT
Depends on: `G-02`, `G-04`

DoD:
- ClusterSpec, discovery triad, bounded broadcast policy are defined.
- Transport matrix (IPC/HTTP/gRPC/QUIC; no uTP baseline) is defined.
- Public/private cluster access model and auth contract are defined.

Evidence:
- Contract docs and protocol tests references in ledger.

## G-06 OPTIMIZER-CONTRACT
Depends on: `G-03`, `G-04`

DoD:
- Analyzer/planner/executor model is defined.
- Cost model, anti-thrashing, apply/rollback safety are defined.
- Optimizer CLI/API and test gates are defined.

Evidence:
- Contract docs and optimizer test references in ledger.

## G-07 GUI-JOURNEY-CONTRACT
Depends on: `G-04`, `G-05`, `G-06`

DoD:
- Role/OS journeys are documented (`weak/user/power/admin` + desktop/server + Android consumer-first).
- `disk` and `non-disk` UX is defined.
- Snapshot share/import fallback behavior is defined.

Evidence:
- Journey spec and acceptance checks referenced in ledger.
