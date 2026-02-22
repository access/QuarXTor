# Drive import pipeline
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## 1. Scope
This document defines the drive import pipeline and its deterministic outputs.
It describes the canonical user-facing flow through `quarxtor` and the config
keys that control import behavior.

## 2. Pipeline stages (ordered)
1) init
   - Discover or create the repository at `core.repo_root`.
   - Validate path invariants (repo root vs import root vs paths.*).
2) import
   - Ingest a directory or a single file (`DIR|FILE`) into the repository.
   - Update the graph and root metadata in `repo.qblk` + `repo.qroot`.
3) stats
   - Compute import counts and repo statistics (bytes, dedup savings, sizes).
4) verify/export
   - Verify repository invariants after import.
   - Export views or file copies are enabled explicitly by the CLI surface.

## 3. Inputs and outputs

### 3.1 User command surface
Canonical entrypoint commands:
- `quarxtor import <DIR>`
- `quarxtor import <FILE>`
- `quarxtor status` (reports system-wide state and repo metrics)

### 3.2 Internal components (high-level)
- `quarxtor` orchestrates the flow and hides daemon details.
- `quarxnetd` is the system daemon when enabled; local import can run in-process
  or via the daemon without exposing ports/protocols.
- `quarxtor_core` owns the repository format and verification logic.

### 3.3 Repository artifacts
- `repo.qblk`: primary data container.
- `repo.qroot`: metadata sidecar; rebuildable from `repo.qblk`.

## 4. Required configuration keys

### 4.1 Repository and path discovery
- `paths.data_dir`, `paths.cache_dir`, `paths.log_dir`, `paths.runtime_dir`
- `core.repo_root`
See `docs/paths-and-repo.md` for OS defaults and path invariants.

### 4.2 Import options
- `import.root`
- `import.min_file_size`
- `import.skip_hidden`
- `import.skip_symlink`
- `import.skip_zero`
- `import.skip_devices`
- `import.skip_special`
- `import.include_glob`
- `import.exclude_glob`
- `import.include_dir_glob`
- `import.exclude_dir_glob`
- `import.import_glob_mode`
- `import.use_z`
- `import.z_threshold`

### 4.3 Core analysis and chunking
- `core.l0_chunk`
- `core.analysis.enable_znode`
- `core.analysis.fs_stats_fallback`

### 4.4 Drive policy (reserved, FUTURE)
These keys exist but are not required for import today:
- `drive.mountpoint`
- `drive.strategy`
- `drive.fs_cache_bytes`

### 4.5 Daemon toggle (reserved, FUTURE)
- `daemon.enable_netd`

## 5. Determinism and output stability
- Stable report keys are required for all machine-readable outputs.
- Stable ordering is invariant and independent of locale.
- `--json` outputs use canonical key order and formatting.

## 6. Examples
```bash
quarxtor import /data/source-dir
quarxtor import /data/source-file.bin
quarxtor status --json
```

## 7. GAP (tracked)
- The canonical `quarxtor` binary entrypoint is tracked in ROADMAP-TARGET.md
  (E11.7) and must replace the current script wrapper.
