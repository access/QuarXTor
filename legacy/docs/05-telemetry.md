# Telemetry
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Telemetry is a core feature of QuarXTor.

## Block-Level Metrics

- size
- entropy
- tier
- last access time
- access frequency
- reference count

## Engine-Level Metrics

- ingest throughput
- read latency
- dedup ratio
- RAM hit/miss ratio
- segment utilization

## Export Formats

- JSON
- streaming events
