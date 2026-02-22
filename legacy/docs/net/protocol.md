# QuarXNet — Minimal Protocol Surface (E9)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


This document defines the minimal, versioned-on-wire protocol surface required by Epic E9.
Status: LEGACY. This framing is non-default and is kept for historical reference.
Canonical proto0 framing is defined in `docs/08-transport-spec.md`.

## Frame transport (legacy)

Legacy framing notes are kept for historical context; proto0 framing is defined in `docs/08-transport-spec.md`.

Each message is a `Frame`:

- header: 8 bytes (proto0, canonical; see `docs/08-transport-spec.md`)
  - bytes 0..3: ASCII magic `QXRT`
  - byte 4: version (u8) = 0
  - byte 5: opcode (u8)
  - bytes 6..7: payload length (u16, little-endian)
- payload: `length` bytes

Legacy (deprecated, non-default):
- 6-byte header (`kind`, `flags`, `length` as u32 big-endian) is historical and must not be used for proto0.

## Block identity

External identity is `BlockKey = (hash, size)` where:

- `hash` is the 32-byte payload hash embedded in the `QBLK` frame (`frame[12..44]`)
- `size` is the payload length of the `QBLK` frame (`u32`, `frame[8..12]`)

## GetBlock / PushBlock

### `GetBlock` (FrameKind = 9)

Payload:
- `hash` (32 bytes)
- `size` (u32, big-endian)

### `PushBlock` (FrameKind = 10)

Payload:
- raw `QBLK` frame bytes (including its header + payload)
- empty payload can indicate “not found” in minimal implementations

## PutBlock

### `PutBlock` (FrameKind = 11)

Payload:
- raw `QBLK` frame bytes (including its header + payload)

Notes:
- minimal servers can reply with `PushBlock` containing the stored frame as an ACK
- servers MUST reject invalid `QBLK` frames before storing (bad magic/length/version or payload-hash mismatch)

## Roots / snapshots (minimal)

### `ListRoots` (FrameKind = 12)

Payload: empty.

### `RootsList` (FrameKind = 13)

Payload:
- repeated `BlockKey` entries:
  - `hash` (32 bytes)
  - `size` (u32, big-endian)

## Errors (deterministic)

### `Error` (FrameKind = 14)

Payload:
- `code` (u16, big-endian)

Defined codes (minimal set):
- `1` = invalid `QBLK` frame (structure or payload-hash mismatch)
- `2` = not found
