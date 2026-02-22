# I/O Policy Controls (Drive-Owned)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## Ownership boundary (ARCH-TARGET.md invariant)

Direct I/O (and any OS-specific filesystem I/O strategy such as `O_DIRECT`, `posix_fadvise`, readahead
controls, mmap vs read/write, etc.) is **Drive-owned**.

Core (`core-rs`) remains:

- filesystem-agnostic
- OS-agnostic
- format/invariant owner only (serialization, ordering, verification)

Drive (`quarxdrive`) owns:

- host filesystem traversal and import/export behavior
- I/O policy selection and tuning
- user-facing knobs for performance vs resource tradeoffs

## Determinism constraints

I/O policy changes MUST NOT change:

- `repo.qblk` / `repo.qroot` bytes for the same logical inputs
- ordering of CLI outputs (except stderr logs when explicitly enabled)

I/O policies are allowed to change:

- throughput
- CPU utilization
- memory usage
- latency

## Knobs tracked in ROADMAP E8.2 (not implemented)

- Drive config toggle: `[drive] io.direct = off|auto|on` (where supported).
- Measurements recorded with fixed datasets, with determinism validated (byte-identical repo + outputs).

## Measurements (baseline)

*Note:* `io.direct` is currently a config-only toggle; it does not change runtime behavior yet.

Dataset:
- 1 file: `large.bin` (256 MiB, zero-filled)
- Config included: `[import] include_glob = *.bin` (to avoid default include filters)

Command (both runs):
- `quarxdrive --log-level error --format json --root-path <repo>.qblk fs-import <dataset>`

Results (Arch Linux, local /tmp):
- `io.direct=off`: `1.211369s`
- `io.direct=on`:  `1.219616s`
