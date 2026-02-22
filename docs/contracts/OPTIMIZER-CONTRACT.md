# OPTIMIZER-CONTRACT

## Architecture
- `analyzer`
- `planner`
- `executor`

## Scheduling
Dual-queue score scheduler:
- `quick-win`
- `high-impact`

## Cost Model
Two-stage estimation:
- stage A: cheap heuristic estimation
- stage B: exact pre-apply estimation

## Safety
- Plan-required apply.
- Chunked transactional batches.
- Verify-after-batch.
- Rollback checkpoints.
- Anti-thrashing: cooldown + min-gain thresholds.

## Scoring
`net_value_score` includes:
- space gain
- CPU/IO cost
- restore penalty
- network/replication cost

## Efficiency Metrics
Every optimizer job records:
- `bytes_before`, `bytes_after`, `space_saved_bytes`
- `dedup_ratio_before`, `dedup_ratio_after`
- `compression_ratio_before`, `compression_ratio_after`
- `cpu_ms`, `io_read_bytes`, `io_write_bytes`
- `prediction_error_pct` (planned vs actual gain)

## Tier Policy
Tier-aware profiles:
- hot: latency-safe
- warm: balanced
- cold: space-dense

## API and CLI
Job-oriented API and CLI lifecycle:
- `analyze`
- `plan`
- `apply`
- `rollback`
- `history`
- `tune`

## Validation
Required pack:
- determinism
- performance bounds
- safety/rollback correctness
- predicted-vs-actual audit loop
- metric schema stability for dashboard/API consumers
