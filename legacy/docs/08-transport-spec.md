# Transport Spec (Proto0)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


This document defines the currently implemented transport framing for QuarXNet.

## Framing

Proto0 uses an 8-byte header:

- bytes 0..3: ASCII magic `QXRT`
- byte 4: version (u8) = 0
- byte 5: opcode (u8)
- bytes 6..7: payload length (u16, little-endian)

Opcodes:
- 1: PING (len=0)
- 2: PONG (len=0)

## Versioning and compatibility

- Version 0 is the only supported wire version.
- Any other version is rejected with `bad version`.
- Future versions must bump the header version byte and document forward/backward compatibility explicitly.

## Transport mapping

- TCP: implemented (`quarxnetd` binds to `net.addr`).
- Unix domain socket: implemented on Unix when `net.transport=unix`.
- Windows: Unix sockets are not available; TCP is the supported transport.

## Security status

- TLS/auth are not implemented; transport is plaintext.

## Determinism

- Header parsing is fixed-size and deterministic.
- Error strings are stable tokens and locale-independent.
