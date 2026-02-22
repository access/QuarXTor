# QuarXNet API (RPC / HTTP)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


QuarXNet exposes network interfaces around QuarXCore. The current implementation is a minimal framed RPC (proto0). HTTP/gRPC surfaces are not implemented in the current build.

## HTTP endpoints (not implemented)

### PUT /object
Upload object -> returns object ID.

### GET /object/<id>
Download object.

### GET /stats
Return engine statistics.

### GET /block/<hash>
Return info / raw block.

## Proto / ZNode

- ZNode/ZNodeKind live in the shared crate `quarxproto` (bincode format).
- Client helpers `client_push_znode` / `client_get_znode` in `quarxnet` use this format unchanged.
- Treat proto types as part of the stable surface between `quarxnet` and `quarxdrive`; no ad-hoc structs in net.

## Proto0 framing (implemented)

The current wire format is proto0 with an 8-byte fixed header:

- bytes 0..3: ASCII magic `QXRT`
- byte 4: version (u8) = 0
- byte 5: opcode (u8)
- bytes 6..7: payload length (u16, little-endian)

Opcodes:
- 1: PING (len=0)
- 2: PONG (len=0)

## Versioning and compatibility

- The header version byte is canonical. Proto0 is version 0.
- Current behavior rejects non-zero versions.
- Future versions must increment the header version and define explicit compatibility rules.
- `quarxproto` bincode types are stable within a given version and must not change without a version bump.

## Transport families and security status

- HttpApi (HTTPS+JSON): not implemented.
- FramedRpc: implemented (proto0 over TCP).
- LocalIpc: implemented on Unix via domain sockets; not available on Windows.
- GrpcCompat: not implemented.
- TLS/auth: not implemented; current transports are plaintext.

## Deterministic error/status outputs

- Proto0 error strings are stable tokens: `bad magic`, `bad version`, `unexpected opcode/len`.
- Status outputs use fixed keys and stable ordering; they are locale-independent.
