# Semantic Layers: Quarks, Local Meta, Cloud Semantics
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Status: PROPOSED (non-breaking direction)
Scope: Conceptual contract for future evolution (does not change current architecture)
Language: English-only (project rule)

## 0. Purpose

This document defines a **non-breaking semantic layering direction** for QuarXTor:
- **Core** owns *data physics* (quarks/blocks/graph/verification).
- **Drive** owns *local user semantics* (manifests, namespaces, “known files”, local tags).
- **Cloud** owns *aggregated / higher-order semantics* (global popularity, replication hints, cross-node “images”).

The goal is to enable richer behaviors (tiering decisions, prefetch, replication, “meta-drive” workflows) **without polluting core formats** and **without introducing hidden self-modifying intelligence**.

This is explicitly aligned with the existing architecture: strict layering, analysis as a separate subsystem, deterministic recovery, and opt-in analytics.

## 1. Core Principle: Meaning Lives Above Core

QuarXCore is **meaning-agnostic**. It provides:
- content-addressed quarks (blocks),
- typed frames (L0 / Multi / Z:* / Object),
- hash-graph traversal semantics,
- validate/verify invariants, ref-tracking, and deterministic recovery.

Any concept that is:
- user-defined,
- workload-defined,
- popularity-based,
- policy-dependent,
- or time-varying

**must not become a core invariant** and must not be required for correctness, recovery, GC, or verification.

## 2. Layer Responsibilities

### 2.1 QuarXCore (core-rs): Quarks and Correctness

**Owns (source of truth):**
- Quark identity, BlockKey ↔ BlockId mapping
- Frame decoding/encoding and stable kinds
- Graph correctness (link integrity, kind mismatches detection)
- Deterministic rebuildability (qroot rebuild from qblk, if applicable)
- Structural blocks (Z:*) as *data representation*, not as policy

**Must remain free of:**
- filesystem paths
- “file popularity”, “known file”, “user tags”, “cloud levels”
- replication strategies, prefetch heuristics, “global catalog”
- any automatic mutation triggered by analytics

**Permitted extension mechanism (safe):**
- read-only analysis hooks that compute deterministic metrics (entropy, repetition, structural motifs)
- exported as *metrics*, not as “meaning”

### 2.2 QuarXDrive: Local Meta and User Semantics

QuarXDrive owns **local meaning** and user-visible structure:
- object manifests (user-facing identity → root blocks / recipes)
- namespaces and local catalogs (“known files”, directory views)
- chunking parameters as part of object/recipe metadata (e.g., L0 chunk size used for that object)
- local tags (pin, retain, export groups), local policies

This layer can change rapidly and can be revised without reformatting the core store.

Drive meta must be:
- not required for core correctness (core must not depend on it)
- reconstructible or replaceable (at least to the extent that raw data remains intact)

### 2.3 QuarXCloud: Aggregated Semantics (“Images”)

QuarXCloud owns **cross-node, aggregated, higher-order semantics** derived from:
- telemetry snapshots,
- access patterns (hotness),
- replication state,
- cross-repo/object relationships.

Examples:
- global popularity level of an object
- replication desirability and placement hints
- “image-like” abstractions: cluster-wide canonical manifests, curated sets, global indexes
- recommended prefetch sets for a drive overlay

Cloud semantics are **not required for correctness**. They are advisory and can be stale or missing without breaking verification or recovery.

## 3. Semantic Levels (Recommended Model)

A minimal stable model is a small integer “level” + flags, scoped by layer:

- **L1 (Core-adjacent):** physical representation choices (quark size, structural encoding decisions)  
  *Note:* this is not stored as a core “meaning level”; it is manifested via frames/recipes or analysis metrics.

- **L2 (Drive):** locally known objects/manifests (user semantics)  
  Example: local file catalog entries, local pins, local “importance” levels.

- **L3 (Cloud):** aggregated semantics (“images”)  
  Example: globally popular objects, cluster-level “keep hot”, replication target class.

These levels are not a single global truth. They are **layer-scoped truths**:
- Drive level: local truth
- Cloud level: cluster truth
- Core: no semantic truth, only correctness truth

## 4. Data Model: MetaIndex (Non-Core, Explicitly Enabled)

Introduce a **MetaIndex** as a Drive/Cloud-owned structure (implementation detail):
- keys: `ObjectId`, `ManifestId`, `BlockId` when explicitly enabled (careful: block-level meaning tends to explode)
- values: `{ level: u8, flags: u32, hotness: u32, last_seen: u64, origin: enum }`

Storage placement options (non-breaking):
- Drive-local sidecar (preferred at first)
- drive-owned section inside `repo.qroot` (if qroot exists and policy allows)
- cloud-owned database (source of truth for L3), cached locally when enabled

**Rules:**
- MetaIndex must never be required for validate/verify/GC.
- Loss of MetaIndex must not imply data loss.
- MetaIndex can be rebuilt from telemetry and manifests (best-effort).

## 5. Protocol: Cheap Exchange of Semantics

Cloud/Drive synchronization should be a lightweight contract (quarxproto DTOs):
- `SemanticSnapshot` (periodic): counts, hotness, manifest refs
- `SemanticHints` (advisory): keep/prefetch/replicate suggestions
- `ManifestExchange` (explicitly enabled): canonical manifests for “images”

**Hard rule:** advisory semantics must be safe to ignore.

## 6. Determinism and Safety

To preserve QuarXTor invariants:
- Core persistent formats remain deterministic and versioned.
- Semantic/meta layers must not introduce nondeterministic iteration order into persistent core indexes.
- Analytics remains opt-in and read-only by default; no background “intelligence” that mutates storage state automatically.

Any policy action (prefetch, replicate, tier migration) must be:
- explicit (configured),
- auditable (logged),
- reversible (does not endanger correctness).

## 7. Implementation Guidance (Incremental, Non-Breaking)

Recommended order (safe staging):
1) Define DTOs for `SemanticSnapshot` and `SemanticHints` in quarxproto.
2) Implement Drive-local MetaIndex sidecar + basic CLI visibility.
3) Add Cloud aggregation service that consumes snapshots and emits hints.
4) Add Drive policy modules that apply hints (prefetch/pin/replicate) when enabled by config.
5) Persist selected Drive meta in qroot when explicitly enabled (still Drive-owned) once formats are stable.

## 8. Non-Goals

- No “global brain”, no autonomous self-modifying core.
- No semantics in core verification rules.
- No mandatory background daemons in MVP.
- No hidden coupling where “meaning” becomes required for recovery.

## 9. Rationale (Why This Works)

This direction preserves:
- strict layering (core → net → drive → cloud),
- stable correctness surface in core,
- fast-evolving semantics above core,
- cheap interoperability via DTO exchange,
- future extensibility (Z:* heuristics informed by analysis, without policy leaking into core).

End.
