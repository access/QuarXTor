# Parallel Hashing & Safe Parallel Regions
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Goal: increase throughput without breaking determinism (stable outputs, stable on-disk artifacts).

This note documents where parallelism is safe in the current architecture, and where it is not.

## Determinism invariants (must hold)

- Any *user-visible output* (text/JSON) must be produced in a stable order (sort before print).
- Any *persistent artifact* must not depend on HashMap iteration order or task scheduling.
- `BlockId` assignment must remain deterministic (never depend on race order).
- Parallel work must not introduce nondeterministic log-driven side effects (avoid per-chunk logs).

## Safe parallel regions (pure / order-free compute)

These are safe to parallelize because each unit of work is independent and produces a value that
can be merged deterministically:

- **Payload hashing** (`blake3(payload)`) for blocks where the payload bytes are already available.
  - Safe pattern: parallel hash compute → collect (with stable indices) → sequential commit.
  - References: `core-rs/src/store/blockstore.rs` (`hash_payload`, `make_frame_*`).
- **Verification hashing** for already-addressable blocks (re-hash payload and compare).
  - Safe pattern: parallel hash compute over a *stable list of BlockIds* (sorted / ascending).
  - References: `core-rs/src/validate.rs` (`verify_hash_mismatch`).
- **Per-block analytics** (entropy estimation, size stats) when results are aggregated via
  deterministic reductions (e.g., sum/count) or sorted vectors.

## Parallel regions that require a “parallel compute, sequential commit” split

These can use parallelism only if *the commit phase stays single-threaded* and ordered:

- **Import chunking pipeline**
  - Parallel: compute hash / frame bytes for chunks (indexed by `(path, chunk_index)`).
  - Sequential: assign `BlockId`s and append frames to `repo.qblk` in a stable order.
- **Batch encoding** (multi/object/z frames)
  - Parallel: encode payloads + compute hashes.
  - Sequential: assign IDs + write frames.

The key rule: anything that calls `next_id()` or appends to `repo.qblk` must be deterministic and
order-preserving.

References:
- `core-rs/src/store/file_store.rs` (`append_frame`, `next_id`)
- `core-rs/src/store/mem_store.rs` (`next_id`)

## Not safe to parallelize (order-sensitive / stateful)

- **Appending to `repo.qblk`** (affects offsets and `BlockId` order).
- **Any code that assigns IDs** (`next_id` / `index.len()` driven).
- **Any output built from unordered maps/sets** unless keys are sorted first.
- **Per-chunk tracing in parallel loops** if tests compare stderr logs across runs (ordering is not
  guaranteed under parallel scheduling).

## Implementation constraints (recommended patterns)

- Use `rayon` for CPU-bound parallelism and keep the thread pool controlled (config-driven).
  - Prefer a dedicated `rayon::ThreadPool` and `pool.install(|| ...)` for parallel regions.
- Preserve deterministic order by construction:
  - Use `IndexedParallelIterator` (e.g., slice `par_iter()`) and collect into a `Vec` (stable index).
  - If order cannot be guaranteed by the iterator type, attach a stable index and sort before commit.
- Treat logging as *observability*, not part of deterministic output:
  - Log at operation granularity (start/end/duration), not per-chunk.

## Next steps (tracked in ROADMAP E8.1)

- Add a worker-count config key (effective config) to control hashing parallelism.
- Implement parallel hashing in import/verify with sequential commits.
- Add determinism tests: same inputs with `workers=1` and `workers=N` produce identical outputs
  (stdout + on-disk artifacts where applicable).

