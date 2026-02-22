# Node Registry & Heartbeat (QuarXCloud)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Goal: provide minimal cluster coordination primitives without becoming a source of truth for storage.

## Ownership & layering

- Cloud (`quarxcloud`) owns coordination state and node liveness.
- Net/Drive/Core remain source of truth for storage formats and deterministic artifacts.

## Determinism requirements

- Node listing must be stable and deterministic (lexicographic by `node_id`).
- Registry state is not persisted to repo formats; it is coordination-only.

## Current implementation (in-memory)

- `quarxcloud/src/registry.rs` provides `NodeRegistry`:
  - `register(node_id, addr)` upserts a node record
  - `heartbeat(node_id, now_ms)` updates last-seen time (caller-provided)
  - storage uses `BTreeMap` to keep listing deterministic

Next items in ROADMAP E10.1:
- stable node list output/serialization for APIs/CLI
