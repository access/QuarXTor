# fs-import Report Block (Stable Contract)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Goal: a single, machine-readable report block with consistent fields and stable key ordering.

## JSON report (preferred)

Output constraints:

- Exactly **one** JSON object is emitted to stdout for the report.
- No other human text is written to stdout (logs go to stderr via the unified logging layer).

Top-level object (stable key order):

1) `report_type` (string) — `"fs_import"`
2) `host_dir` (string) — absolute or as provided on CLI
3) `repo_path` (string) — store path (e.g. `repo.qblk`)
4) `result` (object)
5) `metrics` (object)
6) `config_effective_ini` (string) — canonical INI (stable order)
   - Exact value should match `render_effective_config_ini(&EffectiveConfig)` for the config used by the operation.

### `result` (stable key order)

- `new_root_id` (u64)
- `files` (u64)
- `dirs` (u64)
- `bytes` (u64)
- `skipped` (object):
  - `zero` (u64)
  - `symlink` (u64)
  - `hidden` (u64)
  - `devices` (u64)
  - `special` (u64)
  - `other` (u64)
- `z_bytes` (u64, nullable)
- `z_ratio` (f64, nullable)

### `metrics` (stable key order)

- `elapsed_ms` (u64)
- `blocks_before` (u64)
- `blocks_after` (u64)
- `blocks_delta` (u64)
- `blocks_per_s` (f64)
- `mb_per_s` (f64)
- `cpu_norm` (f64, nullable)
- `rss_kb` (u64, nullable)
- `store_bytes` (u64, nullable)

## Sample (single line, stable keys)

```json
{"report_type":"fs_import","host_dir":"/data/src","repo_path":"/data/repo.qblk","result":{"new_root_id":129,"files":2,"dirs":1,"bytes":3,"skipped":{"zero":0,"symlink":0,"hidden":0,"devices":0,"special":0,"other":0},"z_bytes":20480,"z_ratio":0.1234},"metrics":{"elapsed_ms":12,"blocks_before":0,"blocks_after":130,"blocks_delta":130,"blocks_per_s":7.99,"mb_per_s":0.01,"cpu_norm":0.000,"rss_kb":6452,"store_bytes":174041},"config_effective_ini":"[core]\\n...\\n"}
```
