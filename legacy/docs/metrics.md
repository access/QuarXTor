# Metrics & Equations (Canonical Definitions)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


This document defines the canonical meanings and equations for user-facing metrics.

## Byte accounting

### `logical_bytes`

Total bytes of the *logical input* being processed.

Examples:
- For `fs-import`: sum of imported file sizes (after filters).
- For `import-l0`: `stats.total_input_bytes`.

### `stored_bytes`

Total bytes physically stored for the operation’s output *excluding* the input source.

Examples:
- For `import-l0`: `stats.unique_bytes` (unique chunk bytes written once).
- For repo-size reports: on-disk bytes of repo artifacts being counted (`repo.qblk` and, when applicable, `repo.qroot`).

### `saved_bytes`

Bytes avoided due to deduplication/compression:

`saved_bytes = max(logical_bytes - stored_bytes, 0)`

### `dedup_factor`

How many times larger the logical input is compared to stored bytes:

If `stored_bytes > 0`:

`dedup_factor = logical_bytes / stored_bytes`

Else:

`dedup_factor = 1.0`

## Z-block metrics

### `z_bytes`

Total bytes represented by Z-structural blocks for the selected root (logical coverage, not on-disk size).

### `z_ratio`

`z_ratio = z_bytes / logical_bytes` (when `logical_bytes > 0`, otherwise `0.0`)

## Consistency checks (must hold)

- `saved_bytes + stored_bytes >= logical_bytes` (due to saturation)
- `dedup_factor >= 1.0`
- `0.0 <= z_ratio <= 1.0`

