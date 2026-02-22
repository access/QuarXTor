# GOVERNANCE (Rules for Changing Rules)

This document defines how QuarxTor-DAG changes contracts and gates.

## Working Language
All engineering artifacts are English-only.

## Artifact Separation (non-negotiable)
- MAP (`docs/control/ROADMAP-MAP.md`): structural truth only.
- EXEC (`docs/control/ROADMAP-EXEC.md`): operational cursor only.
- LEDGER (`docs/control/ROADMAP-LEDGER.ndjson`): append-only transitions and decisions.
- FRAG/ADR: context and design decisions.

## Evidence-Driven Closure
A gate can be closed only by a ledger `gate_transition` with:
- `to: "closed"`
- non-empty `evidence[]`
- non-empty `commit`

## Change Classes
- Constitutional: changes to governance, statuses, artifact roles.
- Architectural: changes to major contracts/layer boundaries.
- Incremental: changes inside existing contracts.

## Reopen Matrix
- Entry point contract changes reopen `G-02`, and downstream gates.
- Runtime seed scope changes reopen `G-03` and downstream gates.
- CLI I/O contract changes reopen `G-04` and downstream gates.
- Cloud/P2P contract changes reopen `G-05` and dependent gates.
- Optimizer contract changes reopen `G-06` and dependent gates.
- GUI journey contract changes reopen `G-07`.
- Control semantics changes reopen `G-00` and impacted gates.

## Gate Status Semantics
See `docs/control/STATUS-MODEL.md`.

## Ledger Discipline
`docs/control/ROADMAP-LEDGER.ndjson` is append-only.
Corrections are new events, never edits.

## Minimum Automation
`python3 tools/control_check.py` must pass before closure claims.
