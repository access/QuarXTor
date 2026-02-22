# Crash/Recovery Drills
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## Truncated qblk (partial frame at EOF)

Command:
`cargo test -p quarxtor-core --test repo_truncate`

Result:
`test file_store_ignores_partial_frame_at_eof ... ok`

## qroot rebuild after loss

Command:
`cargo test -p quarxtor-core --test qroot_rebuild`

Result:
`test qroot_rebuild_is_deterministic ... ok`
