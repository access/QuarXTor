# CLOUD-P2P-CONTRACT

## Cluster Model
Typed `ClusterSpec` includes:
- `cluster_id`
- `mode` (`public` or `private`)
- failure domains
- transport policy
- replication policy
- auth policy
- federation peers

Cluster registry view is represented as `ClusterCatalog`:
- `cluster_id`
- `display_name`
- `capabilities` (`mount`, `snapshot-share`, `optimizer-remote`, `gateway-auth`)
- `transport_endpoints[]` (`ipc/http/grpc/quic`)
- `last_seen_at`
- `health_state`
- `policy_ref`

## Discovery
Discovery triad:
- `announce`
- `query`
- `resolve`

Discovery resolution order:
1. local registry cache
2. configured private control-plane
3. bounded mesh query (when allowed by policy)
4. optional gateway federation query

## Data Addressing Units
Canonical units:
- `BlockKey(hash,size)` for physical chunk ownership.
- `ObjectRootHash` for DAG roots.
- `MetaHashSet` for snapshot manifests and policy refs.

Ownership exchange protocol:
1. compare compact sketch/bloom for `BlockKey` candidate set.
2. exchange exact ranges for uncertain windows.
3. request missing blocks by priority class (`meta-first`, then data chunks).

## Ownership Exchange
- Fast path: manifest bloom/sketch.
- Exact path: targeted exact index.
- Canonical ownership unit: `BlockKey(hash,size)` + object root refs.

## Broadcast Rules
Mesh-wide broadcast is allowed only as bounded broadcast:
- TTL
- max fanout
- request-id dedupe
- rate limits
- ACL policy

Broadcast classes allowed:
- control/discovery only
- no data payload replication via broadcast

## Transport Matrix
Baseline:
- `ipc`, `http`, `grpc`, `quic`
- no `uTP` in baseline

Optional extension:
- `uTP` can be enabled only behind explicit transport policy flag and interoperability tests.
- Baseline behavior and tests must pass without `uTP`.

## Security
- Private mode: mTLS + capability-scoped ACL.
- Public mode federation: policy peering with trust-level controls.
- User login/password allowed at gateway plane only.

## Replication
- Bounded replication with configurable `R/W/N` quorum.
- Partition behavior: safety-first degraded mode.
- Resync: snapshot anchor + delta sync + verify checkpoints.
