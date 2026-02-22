# QuarXCloud – Cloud / Multi-Node Topology
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## Nodes

Roles:

- storage
- index
- coordinator

## Replication

Simple replication (2 copies) in current versions.

## Health

- node heartbeat
- replication backlog
- missing blocks detection

## Deployment Models

- single node
- local cluster (3–5 nodes)
- Kubernetes cluster (later)
