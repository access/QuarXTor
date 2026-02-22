# FORMAT.md

This file anchors on-disk format invariants.

## Authority
Detailed format specification lives in `docs/core-formats.md`.

## Invariants
- On-disk frames are versioned ABI.
- Unknown versions are rejected deterministically.
- Extensions are additive and compatibility-gated.
