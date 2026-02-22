# ARCH-TARGET.md

Status: TARGET

## Layering
`quarxtor (entrypoint)` -> `quarxdrive` -> `quarxproto` -> `core-rs`

Future modules (`cloud`, extended transports, optimizer) are integrated through gate closure, not assumed by default.

## Core Invariants
- Single primary entrypoint: `quarxtor`.
- Deterministic outputs and stable contracts.
- On-disk format owned by core.
- Cloud/P2P contracts are additive and version-gated.

## Planned Contract Axes
- CLI I/O universal envelope and exit-code taxonomy.
- Cloud/P2P cluster model: ClusterSpec, discovery triad, bounded broadcast.
- Compression optimizer model: analyzer/planner/executor.
- GUI journey contract including disk and smart non-disk archive.
