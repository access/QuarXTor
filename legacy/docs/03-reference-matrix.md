# Reference Matrix
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Reference Matrix tracks relationships between logical objects and blocks.

## Conceptual Model

Rows: logical objects (files, datasets, snapshots)  
Columns: block IDs

Cell: 1 if object depends on that block.

## Physical Implementation

Optimized representation:

- compressed bitsets
- index → object
- index → blocks
- delta-updates on object creation/removal

## Purpose

Reference Matrix is used for:

- deduplication
- refcount management
- object deletion
- structural analysis
