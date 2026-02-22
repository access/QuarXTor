# Hash Graph
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Hash Graph models structural and semantic relationships between blocks.

## Vertices

Vertices correspond to block IDs.

## Edges

Edges represent relationships:

- structural:
  - next block in a file
  - directory → file
- semantic:
  - versioning
  - snapshots

## Purpose

Graph enables:

- deduction of repeated structures
- global deduplication across datasets
- reconstruction of logical objects
- analytics and pattern detection

## Storage

Graph is stored as:

- adjacency lists
- lightweight metadata per edge
- compressed index
