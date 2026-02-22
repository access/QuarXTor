# Block Model
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Block is the minimal unit of QuarXTor storage.

## Block Structure

Each block has:

- `id` — content hash (BLAKE3)
- `size` — size in bytes
- `tier` — RAM / SSD / HDD / remote
- `refs` — reference count
- `entropy` — approximate entropy (used for type inference)
- `meta` — flags or extended attributes when present

## Chunking

Prototype uses fixed-size blocks.
Later versions can introduce variable-size chunking.

## On-Disk Layout

Blocks are stored inside segment files:

- `segment_0001.dat`
- `segment_0002.dat`
- ...

Index maps:

- hash → (segment, offset, size)
