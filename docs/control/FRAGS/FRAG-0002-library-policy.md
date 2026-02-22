# FRAG-0002 (Library Policy & Dependency Discipline)

## Intent
Prevent dependency sprawl while avoiding bespoke reinvention of standard infrastructure.

## Default Policy
- Prefer mature libraries for logging, configuration, serialization, CLI parsing, metrics, and testing.
- Avoid bespoke implementations in these areas unless a FRAG documents why and governance accepts the exception.
- Treat any “key dependency” as an architectural contract and record it in `docs/control/DEPENDENCY-REGISTRY.md`.

## What Requires a Registry Entry
Create/update a dependency registry entry when a dependency:
- is used across multiple modules/layers,
- shapes data formats or protocol parsing,
- impacts security posture,
- is difficult to replace later.

## Evidence Expectations
For a gate that depends on a key dependency:
- record the selection rationale and alternatives,
- document risks and a basic exit plan,
- link evidence in the closing ledger transition (tests/CI/benchmarks as applicable).

