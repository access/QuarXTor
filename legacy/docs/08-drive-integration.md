# QuarXDrive – Clients and Integration
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## CLI Surface

Examples (canonical entrypoint):

- `quarxtor import <DIR|FILE>`
- `quarxtor status`
- `quarxtor dashboard`

## Filesystem Integration (Read-only FUSE)

Drive exposes a **read-only** mount that presents imported content and repo
objects without exposing internal transports or on-disk layout. The mount is
strictly read-only; all writes are rejected.

### Path mapping (deterministic)

- `by-path/` mirrors the imported tree (stable snapshot, read-only).
- `by-id/` exposes objects by stable ID (`<object-id>/data` and `meta.json`).
- Listings are lexicographically ordered by stable, ASCII keys and are
  locale-independent.

### Dependencies

- Linux: `fuse3` (libfuse3 userspace).
- macOS: `macFUSE`.
- Windows: `WinFsp`.

### Unmount

- Linux: `fusermount -u <mountpoint>` or `umount <mountpoint>`.
- macOS: `umount <mountpoint>`.
- Windows: WinFsp unmount tools (e.g., `fsptool unmount <mountpoint>`), or
  `net use <drive> /delete` for drive-letter mounts.

## SDKs

Language bindings for:

- Python
- Go
- Rust

## ZNode / Proto

- ZNode/ZNodeKind are defined in the shared crate `quarxproto` (bincode ABI).
- `quarxdrive` writes/reads ZNode via this proto format (repo.znode helpers).
- Network path (`quarxnet` client/server) uses the same proto for `client_push_znode` / `client_get_znode`.
- Treat `quarxproto` as the single source of truth for ZNode payloads; avoid duplicating definitions in drive/net.
