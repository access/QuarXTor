# QuarXCore API
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


QuarXCore exposes a local API for the engine.

## Functions

### ingest(path)
Split file into blocks, store blocks, return object ID.

### fetch(object_id)
Return stream of blocks forming the object.

### stats()
Return engine stats (dedup, block count, etc).

### delete(object_id)
Remove references and update refcounts.

## Stability

API will be formalized in milestone 0.5.
