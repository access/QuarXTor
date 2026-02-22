# Analytics Raw Format
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Binary telemetry records are written to a log file. The raw log is the
canonical source; JSON is derived for inspection only.

## File layout

Header:
- magic: 4 bytes `QXTR`
- version: u32 (little endian), currently `1`

Records:
- fixed-size records appended after the header
- record size: 72 bytes
- encoding: little endian

## Record layout (v1)

Offsets are from the start of the record payload (after the header).

- 0x00 u64 `ts_unix_ms`
- 0x08 f64 `blk_s`
- 0x10 f64 `mb_s`
- 0x18 u64 `rss_kb`
- 0x20 f64 `cpu_norm`
- 0x28 u64 `unique_blocks`
- 0x30 u64 `z_bytes`
- 0x38 u64 `store_entries`
- 0x40 u64 `store_bytes`

## Missing values

- u64 fields use `u64::MAX` as a sentinel for missing values.
- f64 fields use `NaN` as a sentinel for missing values.

## JSON export (derived)

`quarxctl telemetry export` reads the raw log and emits JSON. The JSON output
is derived and must not be required for correctness.
