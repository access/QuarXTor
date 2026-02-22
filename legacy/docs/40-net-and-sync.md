# Net and Sync (Current State)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


This document summarizes the current network and sync surface.

## Implemented

- `quarxnetd` provides a minimal framed RPC (proto0) with PING/PONG.
- Transports:
  - TCP is supported.
  - Unix domain sockets are supported on Unix when configured.

## Not implemented

- Replication, sync, and object transfer APIs are not implemented.
- TLS/auth and transport-level security are not implemented.

## Determinism

- Proto0 framing is fixed-size and versioned (see `docs/08-transport-spec.md`).
- Error/status strings are stable tokens and locale-independent.
