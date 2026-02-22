# FRAG-0003 (Example: Architecture Change With Reopen)

This FRAG is an illustrative example of how an architecture change is made explicit and non-silent.

## Scenario
We change the primary entrypoint contract from “CLI-only” to “library-first API with an optional CLI wrapper”.

## Why this is not a silent change
This modifies a contract that other gates depend on. The prior closure of `G-02` is no longer applicable without re-validation.

## Required Governance Actions
- Append a ledger transition: `G-02` → `reopened` with a commit reference and rationale.
- Evaluate dependent gates and append reopen/regression transitions as required (commonly `G-10`, `G-20`).
- Create an ADR describing the new entrypoint contract and consequences.

## Evidence to Re-close
To close `G-02` again, the ledger entry must reference:
- the updated contract artifact(s),
- tests or validation runs proving it works as specified,
- a commit hash that contains the final form.

