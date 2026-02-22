# E4.2 — Re-import with partial changes
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Goal: re-importing after modifying a small fraction of inputs should only grow storage proportional to the modified content, not rewrite unchanged files.

## Canonical acceptance check (portable)

- Command:
  - `cargo test -p quarxdrive reimport_partial_changes_reuses_unchanged_files`

What the test does:
- creates a dataset of 10 files (`*.so`)
- imports once into a repo
- modifies 1 file (10%)
- imports again
- asserts `block_count_delta <= 12` (bounded growth; unchanged files are reused)

