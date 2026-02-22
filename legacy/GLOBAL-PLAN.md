# GLOBAL PLAN
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## Phase 1: Build (DONE)
- `quarxtor` binary builds and runs.
- `quarxtor import` (dir/file) and `quarxtor status` are runnable.
- Minimal i18n registry with en-US fallback is present.

## Phase 2: System (IN PROGRESS)
- Cross-OS path rules and normalization are documented.
- Import pipeline and config defaults are documented.
- Transport framing and versioning are documented (proto0).

## Phase 3: Ecosystem (UPCOMING)
- Daemon lifecycle wiring (net/cloud).
- Dashboard integration and status aggregation.
- Install and packaging alignment.
