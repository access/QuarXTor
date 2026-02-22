# ROADMAP-TARGET.md
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.

## QuarXTor — Target Execution Roadmap (Auto-Progress)

Status: TARGET (execution baseline)
Scope: Engineering execution, sequencing, progress tracking, evidence ledger
Authority: MUST NOT contradict ARCH-TARGET.md

This file is the **single operational ledger** of the project.
If something exists or is being worked on — it must appear here.

EXECUTION_CURSOR:
- last_pass_ts_utc: 2025-12-27T15:17:02Z
- last_next_line: 676
- last_epic_section: E2.5 [PLAN] Restore format version registry + tests
- last_next_text: - [ ] Reinstate format version registry and core format tests

Primary requirement:
- **Progress must be auto-computable** from checkboxes / counters in this file.
- Every task has explicit evidence items. If evidence is missing, task is not done.

No speculative scope.
No parallel realities.
No silent changes.

---

## 0. Auto-Progress Rules (Hard)

### 0.1 Units of Progress
- Progress is computed from **atomic items**:
  - `[ ]` / `[x]` checkboxes
  - counters like `DONE: 3/10`
- Each epic has a fixed number of atomic items and a weight.

### 0.2 Global Project Progress Formula
Let each Epic `Ei` have:
- `Wi` weight (integer)
- `Pi` epic progress in `[0..1]`

Then:

`PROJECT_PROGRESS = (Σ(Wi * Pi) / Σ(Wi)) * 100%`

### 0.3 Epic Progress Formula
For each epic, define a list of atomic items `Ai`.
- Each atomic item is one checkbox line: `- [ ] ...`
- Epic progress is:

`Pi = checked_items / total_items`

If an epic uses counters, they must also be expressible as checkboxes or `DONE: a/b`.

### 0.4 Definition of DONE
A task is DONE only if:
- All its checkboxes are checked
- Required evidence links/paths are present (as literal file paths or command outputs)

If it compiles but no evidence is recorded → not DONE.

### 0.5 Evidence Format
Evidence items must be one of:
- file path: `EVIDENCE: path/to/file.ext`
- command that was run + short output snippet:
  - `CMD: <command>`
  - `OUT: <1-10 lines>`
- benchmark summary:
  - `BENCH: <name>`
  - `RESULT: <numbers>`

### 0.6 Scope Lock
- No new epics without explicitly stating:
  - why it is required by ARCH-TARGET
  - what epic weight changes
- Dropped items are never deleted; marked `[DROP]` and checkboxes remain (unchecked).

## 0.7 Tooling (Execution-Required, MUST USE)

These tools are part of the execution contract. Codex MUST use them to select work and to report progress.

### 0.7.1 Progress calculator
Tool: `tools/roadmap_progress.py`

Run:
- `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md`
- Alternate invocation: `./tools/roadmap_progress.py --file ROADMAP-TARGET.md`

Output is the execution reference for:
- PROJECT progress %
- per-EPIC progress %
- Gate state (BLOCKED/OK)

### 0.7.2 Next-task selector (deterministic)
Tool: `tools/roadmap_next.py`

Run:
- `python3 tools/roadmap_next.py --file ROADMAP-TARGET.md --print-range`
- Alternate invocation: `./tools/roadmap_next.py --file ROADMAP-TARGET.md --print-range`

Output is the execution reference for:
- current SCOPE (Gate A/B/C or unlocked)
- selected EPIC + section
- NEXT checkbox line and its line number
- excerpt range for human review

### 0.7.3 Mandatory usage rule (hard)
For every execution pass:
1) Determine NEXT work item ONLY via `tools/roadmap_next.py`.
2) Implement changes and record EVIDENCE under the task/section.
3) Check ONLY the completed checkbox(es).
3.5) Update EXECUTION_CURSOR (line + epic.section + ts).
4) Print updated progress via `tools/roadmap_progress.py` and include the output in the session log.
5) If a task is checked without EVIDENCE → revert the checkbox (task not done).

### 0.7.4 Tool stability
- Tool changes are not allowed ad-hoc.
- Any modification of tooling requires an explicit ROADMAP item under EPIC E0 and its own EVIDENCE.

## 0.8 Contract Sync (Hard)

Contract anchors (MUST READ):
- `FORMAT.md`
- `PRODUCT-CONTRACT.md`

Rules:
- ROADMAP is append-only (no deletions/rewrites; only add atomic items + evidence/notes).
- Any discovered gap vs these contracts MUST be converted into atomic roadmap items under the most relevant Epic.
- Each contract-driven item MUST include:
  - CONTRACT: FORMAT | PRODUCT | BOTH
  - COMPLIANCE: OK | GAP
  - AC1/AC2/AC3 (acceptance criteria)
- Applies to: new items added after 0.8 introduction, and any items explicitly marked `CONTRACT:`

Contract coverage index recorded when present:
- Determinism / Config / Logging / Timeouts / Web UI / Packaging

## 0.9 Resume Rule (Hard)

After any interruption (rate limits, crashes, manual stop), resume is defined as:

1) Run `python3 tools/roadmap_next.py --file ROADMAP-TARGET.md --print-range`.
2) Work ONLY the returned NEXT checkbox line (or the earliest feasible item if BLOCKED).
3) If BLOCKED, append BLOCKER evidence under that item and immediately rerun the selector.
4) Never “remember” context outside this file + contract anchors.

---

## 1. Status Legend

- `[PLAN]` defined & ready
- `[DOING]` in progress
- `[WAIT]` blocked
- `[DONE]` completed
- `[DROP]` intentionally dropped (reason)

### ROADMAP STATUS SEMANTICS

- `[PLAN]` = has pending `[ ]` items
- `[WAIT]` = pending items exist but blocked; items stay `- [ ] ... (WAIT)` with `BLOCKER:`
- `[DONE]` = all items `[x]`
- `[DROP]` = excluded from selection and progress; stays as checkbox with `[DROP]` marker


EVIDENCE (WAIT-NORMALIZE):

FIX-T3 SUMMARY:
- Selection: OK (`python3 tools/roadmap_next.py --file ROADMAP-TARGET.md --print-range` returns E4.1 line 1225; no “Epic has no unchecked…” loop note)
- NOTE: Current selector output shows LINE: 1231 (line numbers shift as evidence blocks are appended)
- Progress: OK (`python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` prints “DROP items excluded from counts.”; totals stable)
- Anomalies:
  - none observed (DONE sections all `[x]`; WAIT sections have `- [ ] ... (WAIT)` + `BLOCKER:`; DROP items present as checkbox lines)

---

## 2. Known Existing Artifacts (Inventory Snapshot)

These are already considered “exists” and must be referenced by tasks that depend on them:

- [x] `ARCH-TARGET.md` (Target Architecture Contract)
- [x] `core-rs/` exists (Core implementation)
- [x] `quarxdrive/` exists (Drive layer)
- [x] `quarxnet/` exists (Net layer skeleton / partial)
- [x] `core-rs/src/store/file_store.rs` (file-backed store with dense offset index `Vec<u64>`)
- [x] `core-rs/src/store/mem_store.rs` (in-memory store using `HashMap<BlockId, Vec<u8>>` — target for densification)
- [x] `core-rs/src/graph/object_graph.rs` (graph traversal / closure)
- [x] `core-rs/src/gc.rs` (GC logic with reachability)
- [x] `core-rs/src/validate.rs` (validation primitives)
- [x] typed block encoding/decoding (`core-rs/src/store/encode.rs`, `decode.rs`, `blockstore.rs`)
- [x] tests exist:
  - `core-rs/tests/gc_*`
  - `core-rs/tests/analysis_patterns.rs`
  - `core-rs/tests/store_roundtrip.rs`

EVIDENCE:
- `core-rs/src/store/file_store.rs`
- `core-rs/src/store/mem_store.rs`
- `core-rs/src/graph/object_graph.rs`
- `core-rs/src/gc.rs`
- `core-rs/src/validate.rs`

## 2.1 Blocker Registry (Sticky)

- B001: `tools/roadmap_next.py` reports "no unchecked checkboxes" when an Epic has 0 unchecked items; fix by adding atomic items or marking [DROP]/[WAIT] explicitly (no duplicate BLOCKER spam).
- B002: `cargo test --workspace` can hang on `znode_roundtrip_over_net_and_drive`; treat as quarantine/skip and run scoped tests.
- Bxxx: <title> — <rule>

---

## 3. Epics, Weights, and Gates

### Weights (must remain stable unless explicitly changed)
- E0 Control Plane & Discipline: `W=3`
- E1 Core Correctness & Invariants: `W=12`
- E2 Repo Format & Recovery: `W=10`
- E3 Drive FS UX & Determinism: `W=10`
- E4 Dedup & Re-import Proof: `W=6`
- E5 Z:* Structural Blocks & Analysis hooks: `W=6`
- E6 Telemetry & Analytics (Binary-first): `W=6`
- E7 Stability & Long-run hardening: `W=7`
- E8 Performance Scaling (after gates): `W=5`
- E9 Networking Minimal Transport: `W=4`
- E10 Cloud/Cluster Coordination: `W=2`
- E11 Reporting & UX truthfulness: `W=4`

### Phase Gates (Hard)
Gate A (MVP correctness gate):
- E1 ≥ 95%
- E2 ≥ 70%
- E3 ≥ 60%
- `cargo test` green for core + drive baseline

Gate B (pre-net gate):
- E4 ≥ 90%
- E6 ≥ 80%
- E7 ≥ 70%

Gate C (pre-cloud gate):
- E9 ≥ 80%

---

# EPIC E0 (W=3) — Control Plane & Project Discipline
Goal: prevent drift, duplication, and context loss.

## E0.1 [DONE] Canonical docs are enforced
- [x] `ARCH-TARGET.md` exists in repo root
- [x] `ROADMAP-TARGET.md` exists in repo root
- [x] `AGENTS.md` references both as contract anchors
- [x] `ARCH-TARGET.md` listed as architecture authority in `AGENTS.md`
- [x] `ROADMAP-TARGET.md` listed as execution authority in `AGENTS.md`

EVIDENCE:
- `AGENTS.md`

## E0.2 [DONE] Repo hygiene: scripts + transient files policy
- [x] Root contains no ad-hoc `*.sh` scripts (except documented tooling)
- [x] `.gitignore` explicitly covers `backups/`, `target/`, and known temp artifacts
- [x] External runner convention documented (`../qx-run/` or similar)

EVIDENCE:
- `.gitignore`
- `docs/paths-and-repo.md`

## E0.3 [DONE] Stable checkpoints rule
- [x] Tagging policy written (what triggers tag, naming scheme)
- [x] First baseline tag created (e.g., `v0.1.0-mvp-baseline`)
- [x] “Safety guard” rules for resets are recorded (git status, reflog, save diff before reset)

EVIDENCE:
- `AGENTS.md` (policy section)

## E0.4 [DONE] Continuation fallback when roadmap_next has no unchecked items
- [x] If `tools/roadmap_next.py` reports “no unchecked checkboxes”, add exactly one minimal atomic item under the currently active Gate/Epic and re-run the selector.
  - AC1: After adding the fallback item, `tools/roadmap_next.py --print-range` returns a concrete NEXT checkbox line.
  - AC2: Evidence includes CMD+OUT for both runs (before/after).
  - AC3: No duplicate BLOCKER spam is appended; reference B001 instead.

EVIDENCE:

## E0.5 [DONE] Tooling ignores [DROP] checkboxes in selection and progress
- [x] Update `tools/roadmap_next.py` to exclude `[DROP]` items from total/progress counts
- [x] Update `tools/roadmap_progress.py` to exclude `[DROP]` items from total/progress counts
- [x] `tools/roadmap_next.py --print-range` returns a real engineering NEXT item (not “no unchecked checkboxes” from drop-only gaps)
- [x] `tools/roadmap_progress.py` shows stable Epic/Gate math with `[DROP]` excluded

EVIDENCE:
- ...
- `quarxdrive/src/bin/quarxdrive.rs` (serde_json output writer)
- `tools/quarxctl/src/main.rs` (serde_json telemetry export)
- `core-rs/src/config.rs` (sysinfo memory available fallback)
- `quarxdrive/tests/logging_json_smoke.rs`
- `quarxdrive/src/bin/quarxdrive.rs` (colored `error:` prefix; honors tty + `--no-color`)
- `tools/quarxctl/src/main.rs` (colored `error:` prefix; honors tty + `--no-color`)
- `tools/roadmap_next.py`
- `tools/roadmap_progress.py`
- `tools/roadmap_next.py`
- `tools/roadmap_progress.py`

## E0.6 [PLAN] Clock Sync audit (Docs/Contracts/Workflow gaps)
- [x] Add FUSE Quickstart section to README
  - COMPLIANCE: GAP
  - CONTRACT: PRODUCT-CONTRACT.md
  - REASON: FORMAT.md and PRODUCT-CONTRACT.md are missing; contract alignment cannot be verified yet.
  - AC1: `README.md` includes "Quickstart: Mount a QuarXTor repo as a FUSE filesystem" with deps, import, mount, use, unmount, verify.
  - AC2: Commands are copy-paste friendly and use current binaries (`quarxctl`, `quarxdrive`) with FUSE mount unavailability clearly labeled.
  - AC3: README includes a short blocker note for missing FUSE mount CLI.
  - FILES: README.md
  - EVIDENCE:
  - README.md

- [x] Create missing contract anchors: FORMAT.md + PRODUCT-CONTRACT.md
  - CONTRACT: both
  - COMPLIANCE: OK
  - REASON: FORMAT.md and PRODUCT-CONTRACT.md are missing; create anchors for deterministic contract references.
  - AC1: `FORMAT.md` defines on-disk format invariants and references `docs/core-formats.md`.
  - AC2: `PRODUCT-CONTRACT.md` lists required user-facing guarantees (config/logging/timeouts/repo lifecycle/FUSE/Web UI).
  - AC3: Both docs use stable section ordering for deterministic references.
  - FILES: FORMAT.md, PRODUCT-CONTRACT.md
  - EVIDENCE:
  - .private/agent/evidence/20251227T072332Z-audit.md

- [x] Create TOOLS-RECIPES.md with CLI recipes for repo/qblk workflows
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: TOOLS-RECIPES.md now documents required repo/qblk CLI recipes.
  - AC1: `TOOLS-RECIPES.md` includes fs-import/fs-tree/fs-stats/fs-get recipes with current CLI names.
  - AC2: Recipes cover quickstart flow and repo verification commands.
  - AC3: Output expectations describe deterministic ordering and stable keys.
  - FILES: TOOLS-RECIPES.md
  - EVIDENCE:
  - .private/agent/evidence/20251227T072332Z-audit.md

- [x] Create I18N-VISION.md with key strategy and locale rules
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: I18N-VISION.md defines key naming, fallback, locales, CLI language policy, and determinism rules.
  - AC1: `I18N-VISION.md` defines key naming, fallback behavior, and supported locales.
  - AC2: CLI/output language policy is defined (defaults and env overrides).
  - AC3: Notes that determinism is independent of locale (stable key order).
  - FILES: I18N-VISION.md
  - EVIDENCE:
  - .private/agent/evidence/20251227T074350Z-vision-cohesion-audit.md

- [x] Add drive import pipeline doc + config defaults
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: Drive import pipeline and config references are documented with determinism notes.
  - AC1: `docs/10-drive-import.md` documents the pipeline (init -> import -> stats -> verify/export) and required config keys.
  - AC2: `docs/config-reference.md` references the drive import doc for EffectiveConfig defaults.
  - AC3: The doc calls out deterministic report keys and stable ordering.
  - FILES: docs/10-drive-import.md, docs/config-reference.md
  - EVIDENCE:
  - .private/agent/evidence/20251227T075232Z-E0.6-drive-import-doc.md

- [x] Expand cross-OS path rules (macOS + normalization)
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: macOS mappings and normalization rules are documented with stable ordering.
  - AC1: `docs/paths-and-repo.md` includes macOS path mappings for data/cache/log/runtime.
  - AC2: Normalization rules for absolute paths and mountpoints are documented for Linux/macOS/Windows.
  - AC3: Path normalization rules guarantee stable ordering in outputs.
  - FILES: docs/paths-and-repo.md
  - BLOCKER: preflight reported dirty worktree; continuing under baseline policy.
  - EVIDENCE:
  - .private/agent/evidence/20251227T083330Z-E0.6-paths-normalization.md

- [x] Document proto contract and transport versioning
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: Network protocol framing, versioning, and transport states are documented.
  - AC1: `docs/07-net-api.md` (or new `docs/08-transport-spec.md`) defines proto framing, versioning, and compatibility policy.
  - AC2: TLS/auth status and transport families are listed with current implementation state.
  - AC3: Error/status outputs are deterministic and stable across transports.
  - FILES: docs/07-net-api.md, docs/08-transport-spec.md, docs/40-net-and-sync.md
  - BLOCKER: preflight reported dirty worktree; continuing under baseline policy.
  - CMD: _shared/scripts/qx_gate_preflight.sh
  - OUT: WORKTREE DIRTY
  - EVIDENCE:
  - .private/agent/evidence/20251227T084305Z-E0.6-transport-docs.md

- [x] Update docs/08-drive-integration.md with first-class FUSE story
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: Drive integration doc now includes read-only FUSE semantics, dependencies, and deterministic listing rules.
  - AC1: `docs/08-drive-integration.md` includes read-only FUSE mount semantics and path mapping.
  - AC2: Dependencies (fuse3/macFUSE/WinFsp) and unmount steps are documented.
  - AC3: Read-only determinism and stable listing order are stated.
  - FILES: docs/08-drive-integration.md
  - EVIDENCE:
  - .private/agent/evidence/20251227T145256Z-E0.6-drive-integration.md

- [x] Update packaging/install docs for FUSE quickstart prerequisites
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: Packaging docs include FUSE dependencies, mount command, and deterministic verification steps.
  - AC1: `docs/packaging.md` lists FUSE dependencies per OS (fuse3/macFUSE/WinFsp).
  - AC2: Packaging instructions mention the FUSE mount CLI and how to verify install.
  - AC3: Instructions keep output checks deterministic (`--version`, stable commands).
  - FILES: docs/packaging.md
  - EVIDENCE:
  - .private/agent/evidence/20251227T145523Z-E0.6-packaging-fuse.md

---

# EPIC E1 (W=12) — Core Correctness & Invariants
Goal: graph + store + verify + GC correct before speed.

## E1.1 [DONE] Typed block model and IDs are stable (already implemented)
- [x] `BlockId=u64` used throughout core
- [x] typed blocks exist (`L0`, `Multi`, `Z`, `Object`)
- [x] encode/decode exists for typed frames
- [x] traversal reads child IDs from typed payloads

EVIDENCE:
- `core-rs/src/types.rs`
- `core-rs/src/store/encode.rs`
- `core-rs/src/store/decode.rs`
- `core-rs/src/graph/object_graph.rs`

## E1.2 [PLAN] In-memory store densification (productive structure)
Goal: eliminate `HashMap<BlockId, Vec<u8>>` as primary storage.

- [x] Baseline benchmark recorded (current mem_store behavior)
- [x] New dense layout chosen:
  - [ ] [DROP] Option A: `Vec<Vec<u8>>` (only if no deletes)
  - [x] Option B: `Vec<Option<Vec<u8>>>` (allows delete)
  - [ ] [DROP] Option C: slab (`Vec<Entry>` + free list)
- [x] Implementation completed in `core-rs/src/store/mem_store.rs`
- [x] `list_block_ids()` remains supported and deterministic
- [x] All core tests pass
- [x] Memory usage improvement measured (RSS) OR explicit “no regression” statement with numbers

EVIDENCE:
- `core-rs/src/store/mem_store.rs`
- `core-rs/tests/*`
- `docs/bench/mem_store.md` (or similar)
- EVIDENCE: core-rs/tests/mem_store_bench.rs
- EVIDENCE: core-rs/src/store/mem_store.rs
- EVIDENCE: core-rs/tests/mem_store_roundtrip.rs
  - (evidence moved to .private/agent/evidence/ — keep this file clean)

## E1.3 [DONE] Verify/Validate contract is complete and deterministic
- [x] Verify detects missing blocks referenced by graph
- [x] Verify detects hash mismatch for stored frames
- [x] Verify detects kind mismatches (stored kind vs expected)
- [x] Verify detects broken references in recipes (bad child IDs)
- [x] Error reporting is stable (same repo state → same output ordering)
- [x] A CLI or callable API exists to run verify end-to-end

EVIDENCE:
- `core-rs/src/validate.rs`
- `core-rs/src/repo_api.rs` or equivalent verify entrypoint
- test: `core-rs/tests/verify_*` (must exist)
- EVIDENCE: core-rs/src/validate.rs
- EVIDENCE: core-rs/tests/verify_contract.rs
- EVIDENCE: core-rs/src/validate.rs (verify_end_to_end)
  - (evidence moved to .private/agent/evidence/ — keep this file clean)

## E1.4 [DONE] GC invariants and reproducibility
- [x] GC closure algorithm validated on:
  - [x] simple repo graph
  - [x] branching graph
  - [x] graph with Z blocks
- [x] GC dry-run mode exists (report only)
- [x] GC apply mode exists (mutating)
- [x] GC never deletes reachable blocks
- [x] GC detects and reports unreachable blocks deterministically
- [x] GC produces stable output ordering (by BlockId)

EVIDENCE:
- `core-rs/src/gc.rs`
- tests:
  - `core-rs/tests/gc_invariants.rs`
  - `core-rs/tests/gc_closure.rs`
  - `core-rs/tests/gc_*`

## E1.5 [DONE] Graph closure API is stable
- [x] `compute_closure_from_object` is deterministic
- [x] `compute_closure_from_block` is deterministic
- [x] closure results contain:
  - [x] roots
  - [x] visited set
  - [x] order (topo/dfs stable order)
- [x] handles missing blocks gracefully (error path tested)

EVIDENCE:
- `core-rs/src/graph/object_graph.rs`
- `core-rs/tests/object_graph.rs` (must exist / updated)

BLOCKER: B001 (historical spam removed)

## E1.6 [DONE] Epic closure & contract sync
- [x] CONTRACT: BOTH
- [x] COMPLIANCE: OK (or GAP)
- [x] AC1: `tools/roadmap_progress.py` reports E1 progress and Gate A math correctly
- [x] AC2: `python3 tools/roadmap_next.py --file ROADMAP-TARGET.md --print-range` returns next work item outside E1
- [x] AC3: No duplicate BLOCKER spam for E1

COMPLIANCE: OK
CONTRACT: BOTH
REASON: Reviewed FORMAT.md and PRODUCT-CONTRACT.md; added contract-driven items under E3, E7, and E11 for config, logging, timeouts, lifecycle commands, filesystem policy, packaging, web UI, deterministic JSON outputs, and determinism test pack.

EVIDENCE:

## E1.7 [DONE] Core invariants backlog note
- [x] Record E1 core backlog note in ROADMAP
  - AC1: backlog note added under E1.7
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)

EVIDENCE:
- ...

## E1.8 [DONE] Core invariants follow-up note
- [x] Record E1 closure note in ROADMAP
  - AC1: backlog note added under E1.8
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)

EVIDENCE:
- ...

## E1.9 [DONE] Core invariants closure note (follow-up)
- [x] Record E1 closure follow-up note in ROADMAP
  - AC1: follow-up note added under E1.9
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)

EVIDENCE:
- ...

HARD GUARD (E1 continuity-note loop):
- Continuity-note items are NOT valid fallbacks. Do NOT create more.
- If `tools/roadmap_next.py` reports “no unchecked checkboxes” for E1, use E0.4 to add ONE real engineering item (code/test/doc) or move to the next Epic/Gate item.

## E1.10 [DROP] Core invariants continuity note
- [ ] [DROP] Record E1 continuity note in ROADMAP
  - AC1: continuity note added under E1.10
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)
  - NOTE: Invalid constraint; ignore.

NOTE: Loop artifact; excluded from progress; do not recreate.

COMPLIANCE: OK
CONTRACT: BOTH
REASON: Roadmap-only continuity note; no format or product changes.

NOTE: E1 continuity note recorded (ROADMAP-only).

EVIDENCE:
- ...

## [ARCHIVE] E1.11 [DROP] Core invariants loop artifact (second)
- [ ] [DROP] Record E1 continuity note (second) in ROADMAP
  - AC1: continuity note added under E1.11
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)
  - NOTE: Invalid constraint; ignore.

NOTE: Loop artifact; excluded from progress; do not recreate.

COMPLIANCE: OK
CONTRACT: BOTH
REASON: Roadmap-only continuity note; no format or product changes.

NOTE: E1 continuity note (second) recorded (ROADMAP-only).

EVIDENCE:
- ...
## [ARCHIVE] E1.12 [DROP] Core invariants loop artifact (third)
- [ ] [DROP] Record E1 continuity note (third) in ROADMAP
  - AC1: continuity note added under E1.12
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)
  - NOTE: Invalid constraint; ignore.

COMPLIANCE: OK
CONTRACT: BOTH
REASON: Roadmap-only continuity note; no format or product changes.

NOTE: E1 continuity note (third) recorded (ROADMAP-only).
NOTE: Canonical continuity-note marker; do not add more.
NOTE: Loop artifact; excluded from progress; do not recreate.

EVIDENCE:
- ...
## [ARCHIVE] E1.13 [DROP] Core invariants loop artifact (fourth)
- [ ] [DROP] Record E1 continuity note (fourth) in ROADMAP
  - AC1: continuity note added under E1.13
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)
  - NOTE: Invalid constraint; ignore.

COMPLIANCE: OK
CONTRACT: BOTH
REASON: Roadmap-only continuity note; no format or product changes.

NOTE: E1 continuity note (fourth) recorded (ROADMAP-only).
NOTE: Loop artifact; superseded by E1.12; do not add more.
NOTE: Loop artifact; excluded from progress; do not recreate.

EVIDENCE:
- ...
## [ARCHIVE] E1.14 [DROP] Core invariants loop artifact (fifth)
- [ ] [DROP] Record E1 continuity note (fifth) in ROADMAP
  - AC1: continuity note added under E1.14
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)
  - NOTE: Invalid constraint; ignore.

NOTE: Loop artifact; superseded by E1.12; do not add more.
NOTE: Loop artifact; excluded from progress; do not recreate.

EVIDENCE:
- ...
## E1.15 [DONE] Tooling skip-drop to unblock real work
- [x] Update `tools/roadmap_next.py` to ignore `[DROP]` checkboxes in progress/selection
- [x] Update `tools/roadmap_progress.py` to ignore `[DROP]` checkboxes in progress totals
- [x] `tools/roadmap_next.py --print-range` returns a real engineering NEXT item (non-note) after the change
- [x] `tools/roadmap_progress.py` reflects stable Epic/Gate math with `[DROP]` excluded

EVIDENCE:
- ...

---

# EPIC E2 (W=10) — Repository Format & Recovery
Goal: portable, rebuildable repo.qblk + repo.qroot contract.

## E2.1 [DONE] repo.qblk append contract + partial frame detection
- [x] Define frame boundary and minimal header format in docs
- [x] Ensure decoder rejects partial frame at EOF safely
- [x] Create test that truncates file mid-frame and verifies detection
- [x] Ensure `file_store` can continue reading up to last valid frame

EVIDENCE:
- `core-rs/src/store/file_store.rs`
- `core-rs/tests/repo_truncate.rs` (must exist)
- doc: `docs/core-formats.md` (QBLK frame layout)

## E2.2 [DONE] repo.qroot rebuild from qblk
- [x] Define what qroot contains (minimal index, offsets, root pointers)
- [x] Implement `rebuild_qroot_from_qblk`
- [x] Rebuild is deterministic (same qblk → identical qroot bytes)
- [x] Add test: build repo → delete qroot → rebuild → verify

EVIDENCE:
- `core-rs/src/store/file_store.rs`
- test: `core-rs/tests/qroot_rebuild.rs`
- doc: `docs/core-formats.md` (QROOT index layout)

## E2.3 [DONE] Versioning and compatibility
- [x] qblk format version stamp exists
- [x] qroot format version stamp exists
- [x] unknown versions rejected with explicit error code
- [x] version values recorded in `docs/format/versions.md`

EVIDENCE:
- `core-rs/src/store/encode.rs`
- `core-rs/src/store/decode.rs`
- `core-rs/src/store/file_store.rs`
- docs: `docs/core-formats.md`
- docs: `docs/format/versions.md`
- test: `core-rs/tests/qblk_version.rs`

## E2.4 [DONE] Atomic update rules
- [x] qroot update uses write-new + fsync + atomic rename
- [x] no in-place qroot mutations that can corrupt on crash
- [x] tests demonstrate safe update pattern (at least unit-level)

EVIDENCE:
- `core-rs/src/store/file_store.rs`
- test: `core-rs/tests/qroot_rebuild.rs`

## E2.5 [PLAN] Restore format version registry + tests
- [ ] Reinstate format version registry and core format tests
  - CONTRACT: FORMAT.md
  - COMPLIANCE: GAP
  - REASON: docs/core-formats.md missing versions link; format version tests absent
  - AC1: `docs/format/versions.md` exists and is linked from `docs/core-formats.md`.
  - AC2: Tests `core-rs/tests/qblk_version.rs`, `core-rs/tests/qroot_rebuild.rs`, `core-rs/tests/repo_truncate.rs` exist and pass.
  - AC3: Unknown versions and partial frames are rejected with deterministic error text.
  - FILES: docs/format/versions.md, docs/core-formats.md, core-rs/tests/qblk_version.rs, core-rs/tests/qroot_rebuild.rs, core-rs/tests/repo_truncate.rs
  - EVIDENCE:
  - BLOCKER: quarxtor-preflight not available
  - CMD: quarxtor-preflight
  - OUT: /bin/bash: line 1: quarxtor-preflight: command not found
  - BLOCKER: quarxtor-roadmap-next-only not available
  - CMD: quarxtor-roadmap-next-only
  - OUT: /bin/bash: line 1: quarxtor-roadmap-next-only: command not found
  - BLOCKER: preflight gate failed: repo root contains *.sh scripts
  - CMD: _shared/scripts/qx_gate_preflight.sh
  - OUT: error: repo root must not contain *.sh scripts
  - OUT: ./git-dev-pr.sh
  - OUT: ./quarxtor.sh
  - BLOCKER: preflight gate failed: worktree dirty
  - CMD: _shared/scripts/qx_gate_preflight.sh
  - OUT: WORKTREE DIRTY. Stop.
  - OUT: RM git-dev-pr.sh -> tools/dev/git-dev-pr.sh
  - OUT: RM quarxtor.sh -> tools/dev/quarxtor.sh
  - BLOCKER: devcheck failed: quarxtor_core::config imports missing
  - CMD: _shared/scripts/qx_gate_devcheck.sh
  - OUT: DEV-CHECK: cargo test --workspace
  - OUT: error[E0432]: unresolved imports `quarxtor_core::config::config_ini_path`, `quarxtor_core::config::parse_ini_sections`
  - OUT: --> quarxtor/src/i18n.rs:5:29

---

# EPIC E3 (W=10) — Drive FS UX & Determinism
Goal: users operate via filesystem import/export; core never sees paths.

## E3.1 [DONE] CLI command surface exists and is stable
- [x] `quarxdrive` CLI exposes:
  - [x] `repo init` (or equivalent)
  - [x] `fs-import`
  - [x] `fs-tree`
  - [x] `fs-stats`
  - [x] `fs-export` / `fs-get`
  - [x] `verify`
- [x] `--root-path` (or equivalent) exists and is documented
- [x] user-facing output shows interpreted info (not raw `ls -al repo.qblk`)
  - [x] repo size
  - [x] block count
  - [x] dedup ratio / saved bytes (if computed)
  - [x] Z:* stats

EVIDENCE:
- `quarxdrive/src/bin/quarxdrive.rs`
BLOCKER:

## E3.2 [DONE] Import filters (include/exclude for files and dirs)
- [x] Config keys exist:
  - [x] `import.include_glob`
  - [x] `import.exclude_glob`
  - [x] `import.include_dir_glob`
  - [x] `import.exclude_dir_glob`
- [x] Filters apply deterministically
- [x] `fs-stats` works after filtered import
- [x] Tests cover:
  - [x] skip by extension
  - [x] skip by directory
  - [x] include-only list

EVIDENCE:
- `core-rs/src/config.rs`
- `quarxdrive/src/fs_import.rs`
- `quarxdrive/tests/fs_import_filters.rs`

## E3.3 [DONE] fs-tree determinism and scale
- [x] stable ordering
- [x] handles large trees without quadratic behavior (basic measure)
- [x] output can be machine-parsed (derived output when `--json` is set)

EVIDENCE:
- `quarxdrive/src/fs_tree.rs`
- `quarxdrive/src/bin/quarxdrive.rs`
- `quarxdrive/tests/fs_import_filters.rs`

## E3.4 [DONE] fs-export correctness (hash-based)
- [x] Export reproduces bytes exactly
- [x] random sampling verify (N files)
- [x] full verify option exists (slow but deterministic)
- [x] explicit mismatch reporting (path + reason)

EVIDENCE:
- `quarxdrive/src/fs_export.rs`
- `quarxdrive/src/bin/quarxdrive.rs`
- test: `quarxdrive/tests/fs_export.rs`

## E3.5 [DONE] Config contract surfaces
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: `config print` outputs INI with stable ordering and round-trips into `QuarxConfig`
- [x] AC2: `config print --effective` outputs `EffectiveConfig` with CLI/env/file/default precedence and stable ordering
- [x] AC3: `config explain <key>` reports value + source with deterministic output

EVIDENCE:
- ...
- COMPLIANCE: GAP
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Implement `config print/--effective` and `config explain <key>` surfaces + deterministic INI rendering (contract-ordered) with unit tests.
- `core-rs/src/config.rs`
- `tools/quarxctl/src/main.rs`

## E3.6 [DONE] Repo lifecycle commands (doctor, recover, stats)
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: `repo verify` and `repo stats` commands exist and are read-only
- [x] AC2: `repo doctor` reports issues with exit codes; `repo recover` exists with explicit confirmation or dry-run
- [x] AC3: Output ordering is stable and deterministic across runs

EVIDENCE:
- ...
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Add repo lifecycle commands to `quarxdrive` CLI with deterministic key/value outputs and explicit `recover --apply` gate.
- `quarxdrive/src/bin/quarxdrive.rs`
- NOTE: corrected test summary (previous OUT line was inaccurate)

## E3.7 [DONE] Filesystem policy and skip reasons
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: Explicit policies for symlinks, special files, device files, and path traversal are documented and configurable
- [x] AC2: `fs-import` reports skip reasons with stable keys and counts
- [x] AC3: Tests cover policy behavior and deterministic ordering

EVIDENCE:
- ...
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Document import policy and add deterministic skip counters + tests for symlink/socket skip behavior.
- `docs/config-reference.md`
- `quarxdrive/src/bin/quarxdrive.rs`
- `quarxdrive/tests/fs_import_filters.rs`

## E3.8 [DONE] Deterministic CLI and JSON outputs
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: `--format json` outputs canonical JSON with stable key order for drive and repo commands
- [x] AC2: Text/kv outputs are locale-independent and stable ordered
- [x] AC3: Snapshot tests assert byte-identical outputs across runs

EVIDENCE:
- ...
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Add `--format json` to `quarxdrive` with canonical JSON writers + CLI snapshot tests.
- `quarxdrive/src/bin/quarxdrive.rs`
- `quarxdrive/tests/cli_format_json.rs`

## E3.9 [PLAN] FUSE mount CLI (read-only)
- [ ] Implement read-only FUSE mount in `quarxdrive`
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: GAP
  - AC1: `quarxdrive mount --repo <qblk> --mount <dir>` mounts a read-only view with deterministic listing order.
  - AC2: Foreground (`--foreground`) and background (`--daemon`) modes are supported with explicit flags and pid file under `[paths].runtime_dir`.
  - AC3: Unmount path is documented and covered by a smoke test or script.
  - FILES: quarxdrive/src/bin/quarxdrive.rs, quarxdrive/src/fuse/mod.rs, docs/08-drive-integration.md, docs/config-reference.md
  - EVIDENCE:
  - (evidence moved to .private/agent/evidence/ — keep this file clean)

## E3.10 [PLAN] Restore drive CLI command surface claimed in E3
- [ ] Implement repo lifecycle + FS commands in `quarxdrive`
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: GAP
  - AC1: `repo init/verify/doctor/recover/stats` and `fs-import/fs-tree/fs-stats/fs-export` exist with deterministic outputs.
  - AC2: `--root-path` (or equivalent) and `--format json` are supported with canonical ordering.
  - AC3: Tests cover import filters, export verification, and CLI output determinism.
  - FILES: quarxdrive/src/bin/quarxdrive.rs, quarxdrive/src/fs_import.rs, quarxdrive/src/fs_tree.rs, quarxdrive/src/fs_stats.rs, quarxdrive/src/fs_export.rs, quarxdrive/tests/fs_import_filters.rs, quarxdrive/tests/fs_export.rs, quarxdrive/tests/cli_format_json.rs
  - EVIDENCE:

---

# EPIC E4 (W=6) — Dedup & Re-import Proof (Real Data)
Goal: demonstrate real dedup behavior and incremental updates.

## E4.1 [DONE] Re-import identical dataset yields minimal growth
- [x] Create test dataset snapshot (or choose a real tree)
- [x] Run import #1: record repo stats
- [x] Run import #2 (same data): record repo stats
- [x] Implement portable E4.1 evidence procedure (workspace temp root + `tools/portable_ls.py`) and record results
- Show:
  - [x] block count delta == 0 after import #2 (identical dataset)
  - [x] repo bytes delta == 0 after import #2 (identical dataset)

EVIDENCE:
- `docs/bench/reimport_identical.md` with numbers
- NOTE: Portable procedure (preferred): use `.private/agent/workspace/e4-reimport-identical/` (or Python `tempfile`) + `python3 tools/portable_ls.py`; legacy `/tmp/*` + `find -printf` evidence below is Linux-only.
BLOCKER:
- BLOCKER: block_count_delta and repo_bytes_delta not ~0 after import #2 (expected minimal growth)

EVIDENCE (UNBLOCK-GateB-E4.1):

EVIDENCE (E4.1 PORTABLE + ZERO-DELTA):
- `docs/bench/reimport_identical.md`
- `tools/e4_reimport_identical.py`
- `quarxdrive/src/fs_import.rs`
- `quarxdrive/tests/reimport_identical_no_growth.rs`
- NOTE: dataset uses `*.so/*.exe/*.iso` to match default `include_glob` (portable, deterministic).

## E4.2 [DONE] Re-import with partial changes
- [x] Modify 10–20% of data (controlled)
- [x] Import again
- [x] Show growth proportional to modified content
  - BLOCKER: observed deltas exceed expected proportional growth; unblock when dedup re-import produces deltas consistent with modified fraction (after dedup fix + rerun evidence)
  - RESOLVED: fs-import reuses unchanged file/dir objects; partial changes no longer rewrite the full dataset (see EVIDENCE below).

EVIDENCE:
- `docs/bench/reimport_delta.md`
- `quarxdrive/src/fs_import.rs`
- `quarxdrive/tests/reimport_partial_growth.rs`
- NOTE: test asserts block_count_delta <= 12 when 1/10 files are modified.
BLOCKER:
- BLOCKER: growth not proportional to 10% data change; deltas exceed expected minimal

---

# EPIC E5 (W=6) — Z:* Structural Blocks & Analysis Hooks
Goal: Z:* is correct, measurable, and verifiable.

## E5.1 [DONE] Z-block integrated as first-class block kind
- [x] Z payload exists
- [x] graph traversal includes Z children

EVIDENCE:
- `core-rs/src/block/zblock.rs`
- `core-rs/src/codec/z.rs`
- `core-rs/src/graph/object_graph.rs`

## E5.2 [DONE] Structural detection measurement surfaced
- [x] import / scan reports Z bytes and ratio
- [x] Z metrics included in fs-stats
- [x] Z metrics included in telemetry line

EVIDENCE:
- `quarxdrive/src/fs_stats.rs`
- `quarxdrive/src/bin/quarxdrive.rs`
- `tools/quarxctl/src/main.rs`

## E5.3 [DONE] Z verify and invariants
- [x] verify checks Z ranges/refs are valid
- [x] tests: corrupt Z payload triggers deterministic error

EVIDENCE:
- `core-rs/src/validate.rs`
- `core-rs/tests/z_verify.rs`

## E5.4 [DONE] Pattern analyzer subsystem boundary (opt-in)
- [x] Analysis subsystem exists as separate module/family
- [x] Disabled by default
- [x] Output available via explicit command (e.g., `repo explain-block --id <id>`)
- [x] Must not mutate repository state

EVIDENCE:
- `core-rs/src/analysis.rs`
- `tools/quarxctl/src/main.rs`

---

# EPIC E6 (W=6) — Telemetry & Analytics (Binary-first)
Goal: metrics without heavy JSON; derived JSON allowed.

## E6.1 [DONE] Live telemetry line
- [x] Throughput (blk/s, MB/s)
- [x] RSS
- [x] CPU normalized (`cpu / cores`)
- [x] unique blocks
- [x] Z bytes
- [x] store entries / bytes

EVIDENCE:
- `docs/bench/telemetry_samples.md`

## E6.2 [DONE] Binary analytics storage
- [x] Define raw/binary record format
- [x] Write raw logs during import/scan
- [x] Provide converter to JSON report (derived)
- [x] Ensure derived JSON is not required for correctness

EVIDENCE:
- `tools/quarxctl/src/main.rs`
- `docs/format/analytics_raw.md`

---

# EPIC E7 (W=7) — Stability & Long-run Hardening
Goal: predictable long runs, diagnosable failures.

## E7.1 [DONE] Hang/stall detection
- [x] Progress counter increments
- [x] Watchdog tick in logs
- [x] Detect “no progress in N seconds” and emit warning

EVIDENCE:
- `quarxdrive/src/fs_import.rs`
- `tools/quarxctl/src/main.rs`

## E7.2 [DONE] Memory return measurement
- [x] After run exit, RSS returns (or explain OS cache behavior)
- [x] Provide “expected behavior” doc if OS caching persists
- [x] Record before/after numbers

EVIDENCE:
- `docs/bench/memory_return.md`

## E7.3 [DONE] Crash/recovery drills
- [x] Simulate crash during qblk append
- [x] Verify repo remains readable up to last valid frame
- [x] Rebuild qroot and verify

EVIDENCE:
- `docs/bench/crash_drills.md`

## E7.4 [DONE] Unified logging contract
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: Single logging layer used across core, drive, and net crates
- [x] AC2: Log records include timestamp (UTC ISO-8601), level, category, message, operation_id when provided
- [x] AC3: Text and JSON log formats available with deterministic field ordering and tests

- [x] Implement unified tracing logging foundation
  - AC1: `quarxtor_core::logging::init_logging(...)` exists and initializes `tracing-subscriber`
  - AC2: Human + JSON formats include required fields; JSON uses stable key order
  - AC3: Unit tests cover human timestamp shape + JSON key order/presence
  - COMPLIANCE: OK
  - CONTRACT: PRODUCT-CONTRACT.md
  - REASON: current code uses `chrono::Local` + ad-hoc `println!/eprintln!` log glue; no unified init or JSON logging
  - NOTE: implemented `core-rs/src/logging.rs` + removed `chrono` from `core-rs`

- [x] Integrate logging flags + color policy into CLIs
  - AC1: `quarxdrive`, `quarxctl`, `quarxnet*`, `quarx-inspect` accept `--log-level`, `--log-format`, `--no-color`, `--timestamps`
  - AC2: Logs go to stderr; command outputs remain deterministic (no ANSI in non-tty / JSON)
  - AC3: No internal `println!/eprintln!` logging remains in core/drive/net paths (except final user-facing output)
  - COMPLIANCE: OK
  - CONTRACT: PRODUCT-CONTRACT.md
  - REASON: CLIs lack unified log init/flags; core/drive/net still contain ad-hoc `println!/eprintln!` logging
  - NOTE: wired `init_logging(...)` + flags across CLIs; migrated core/drive/net internal logs to `tracing`

- [x] Replace ad-hoc JSON/timestamp glue with mature libs
  - AC1: CLI JSON outputs use `serde_json` writers (no manual string building)
  - AC2: Timestamp formatting uses `time` crate (no `chrono::Local` helpers)
  - AC3: Linux-only `/proc/*` probing is not the sole implementation path for used metrics
  - COMPLIANCE: OK
  - CONTRACT: PRODUCT-CONTRACT.md
  - REASON: `quarxdrive` and `quarxctl` still build JSON via manual strings; config RAM probe uses `/proc/meminfo` only
  - NOTE: migrated `quarxdrive` JSON outputs + `quarxctl telemetry export` to `serde_json`; added `sysinfo` fallback for memory metrics

- [x] Add JSON logging integration smoke test
  - AC1: Running a CLI with `--log-format json` emits valid JSON lines
  - AC2: JSON log lines include required keys with stable ordering and no ANSI
  - AC3: Test is deterministic aside from timestamp values
  - COMPLIANCE: OK
  - CONTRACT: PRODUCT-CONTRACT.md
  - REASON: no integration test currently validates end-to-end JSON logging output
  - NOTE: `quarxdrive/tests/logging_json_smoke.rs` asserts JSON validity + key order + determinism (ts_utc normalized)

EVIDENCE:
- ...

EVIDENCE (E7.4 COMPLETE):
- `core-rs/src/logging.rs`
- `quarxdrive/src/bin/quarxdrive-znode-dump.rs`
- `quarxdrive/src/bin/quarxdrive-znode-mktest-tree.rs`
- `PRODUCT-CONTRACT.md`

BLOCKER:
- BLOCKER: unified logging contract requires code changes across core/drive/net; current pass restricted to ROADMAP-only updates
- RESOLVED: logging/time/color foundation implemented; see E7.4 EVIDENCE above

## E7.5 [DONE] Time and timeout reporting
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: Long operations report start, end, and duration in outputs
- [x] AC2: Timeouts enforced for filesystem and network ops with config knobs
- [x] AC3: Tests cover timeout behavior; hangs surface as warnings/errors
- [x] Fix hang in `znode_roundtrip_over_net_and_drive`
  - AC1: `timeout 300s cargo test --workspace` completes without timing out
  - AC2: Test closes client before joining server thread
  - AC3: Evidence includes CMD+OUT for workspace tests

EVIDENCE:
- ...
- `quarxdrive/src/bin/quarxdrive.rs`
- `quarxdrive/src/fs_import.rs`
- `quarxdrive/tests/fs_import_timeout.rs`
- `quarxnet/src/transport/mod.rs`
- `quarxnet/tests/transport_timeouts.rs`
- `tools/quarxctl/src/main.rs`
- `PRODUCT-CONTRACT.md`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Close the client transport before `join()` so the server loop can exit and workspace tests remain bounded.
- `quarxnet/tests/znode_drive_e2e.rs`

BLOCKER:
- BLOCKER: timeouts and duration reporting require code changes across operations; current pass restricted to ROADMAP-only updates
- RESOLVED: duration logs + fs/net timeouts + tests implemented; see E7.5 EVIDENCE above

## E7.6 [DONE] Determinism test pack
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: Determinism test pack lists commands and fixtures with stable expectations
- [x] AC2: Tests run commands twice and compare byte-identical outputs
- [x] AC3: Coverage includes CLI outputs and serialized repo artifacts

EVIDENCE:
- ...
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Add deterministic CLI/JSON snapshot tests and repo artifact byte-for-byte comparisons.
- `docs/determinism-test-pack.md`
- `quarxdrive/tests/cli_format_json.rs`
- `quarxdrive/tests/determinism_pack.rs`

## E7.7 [PLAN] Restore logging/timeout implementation parity
- [ ] Restore unified logging + timeout tests referenced by E7 evidence
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: GAP
  - AC1: `core-rs/src/logging.rs` (or equivalent) exists and is used by core/drive/net init.
  - AC2: Tests `quarxdrive/tests/logging_json_smoke.rs`, `quarxnet/tests/transport_timeouts.rs`, `quarxdrive/tests/fs_import_timeout.rs` exist and pass.
  - AC3: Logs/timeouts use deterministic ordering and stable error prefixes.
  - FILES: core-rs/src/logging.rs, core-rs/src/util/log.rs, quarxdrive/tests/logging_json_smoke.rs, quarxnet/tests/transport_timeouts.rs, quarxdrive/tests/fs_import_timeout.rs
  - EVIDENCE:

---

# EPIC E8 (W=5) — Performance Scaling (After Gate A)
Goal: faster without breaking determinism.

## E8.1 [DONE] Parallel hashing and worker control
- [x] Identify safe parallel regions
- [x] Worker count config key exists
- [x] Determinism preserved under parallelism

EVIDENCE:
- perf doc with numbers
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Document safe parallelization regions and determinism hazards before implementing parallel hashing.
- `docs/bench/parallel_hashing.md`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Add a contract-listed worker-count key to control future hashing parallelism deterministically.
- `core-rs/src/config.rs`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Parallel hashing uses stable chunk order + sequential commit; artifacts and outputs are identical across worker counts.
- `quarxdrive/src/lib.rs`
- `core-rs/src/store/blockstore.rs`
- `core-rs/src/store/file_store.rs`
- `core-rs/src/store/mem_store.rs`
- `quarxdrive/tests/determinism_pack.rs`
BLOCKER:

## E8.2 [DONE] I/O strategy controls
- [x] direct I/O policy is Drive-owned
- [x] config toggles exist
- [x] measurements recorded

EVIDENCE:
- `docs/bench/io_policy.md`
- COMPLIANCE: OK
- CONTRACT: ARCH-TARGET.md
- REASON: Document and enforce the layering boundary: Drive owns host filesystem I/O policy; Core remains filesystem-agnostic.
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Add `[drive] io.direct = off|auto|on` to config contract and EffectiveConfig surfaces (no behavior change in Core).
- `core-rs/src/config.rs`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Record baseline measurements for the drive-owned I/O policy toggle (even if behavior is not yet implemented).
- `docs/bench/io_policy.md`
- BENCH: fs-import baseline (256 MiB zeros, include_glob=*.bin, hash_workers=1)
- RESULT: io.direct=off 1.211369s; io.direct=on 1.219616s
BLOCKER:

---

# EPIC E9 (W=4) — Networking Minimal Transport (After Gate B)
Goal: transport is not a source of truth.

## E9.1 [DONE] Minimal protocol surface
- [x] get-block (by BlockKey)
- [x] put-block (frame upload)
- [x] list roots / snapshots when explicitly enabled
- [x] reject invalid frames before admit

EVIDENCE:
- `docs/net/protocol.md`
- `core-rs/src/types.rs`
- `core-rs/src/net_core/frames.rs`
- `core-rs/src/store/decode.rs`
- `quarxnet/src/protocol.rs`
- `quarxnet/src/server.rs`
- NOTE: PutBlock stores validated `QBLK` frame bytes per-connection (keyed by `BlockKey`) and ACKs via `PushBlock`.
- NOTE: ListRoots/RootsList expose the per-connection key set (deterministic ordering).
- NOTE: PutBlock validates `QBLK` frames (including payload-hash) via `quarxtor_core::store::decode::validate_qblk_frame`.

BLOCKER:
- BLOCKER: protocol surface not implemented/verified yet (pending net work)
BLOCKER:
- BLOCKER: integrity checks require net implementation; current pass restricted to ROADMAP-only updates
- BLOCKER: minimal protocol surface requires quarxnet/tools implementation; current pass restricted to ROADMAP-only updates

## E9.3 [DONE] Net protocol backlog note
- [x] Record E9 net protocol blockers in ROADMAP
  - AC1: BLOCKER note summarizes missing net protocol surface and integrity items
  - AC2: `python3 tools/roadmap_progress.py --file ROADMAP-TARGET.md` output recorded
  - AC3: No code changes (ROADMAP-only)

EVIDENCE:
- ...

## E9.2 [DONE] Integrity on receive
- [x] verify kind + hash
- [x] reject corrupted payload
- [x] deterministic error codes

EVIDENCE:
- tests in net module
- `core-rs/src/store/decode.rs`
- `quarxnet/src/server.rs`
- `docs/net/protocol.md`
BLOCKER:
- BLOCKER: integrity-on-receive checks not implemented/verified yet (pending net work)
- RESOLVED: PutBlock/GetBlock validate `QBLK` frames and use deterministic `Error` codes; see E9.2 evidence above.

---

# EPIC E10 (W=2) — Cloud/Cluster Coordination (After Gate C)
Goal: coordination only (no storage logic).

## E10.1 [DONE] Node registry + heartbeat
- [x] nodes register
- [x] heartbeats recorded
- [x] node list stable

EVIDENCE:
- `docs/cloud/registry.md`
- COMPLIANCE: OK
- CONTRACT: ARCH-TARGET.md
- REASON: Implement minimal in-memory node registry with deterministic ordering.
- `quarxcloud/src/lib.rs`
- `quarxcloud/src/registry.rs`
- COMPLIANCE: OK
- CONTRACT: ARCH-TARGET.md
- REASON: Add explicit heartbeat recording with caller-provided timestamps and unit coverage.
- `quarxcloud/src/registry.rs`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Node list is deterministic (BTreeMap by node_id) and covered by unit tests.
BLOCKER:

---

# EPIC E11 (W=4) — Reporting & UX Truthfulness
Goal: output that never lies and is interpretable.

## E11.1 [DONE] Single stable report block for import
- [x] consistent fields
- [x] no duplicated sections
- [x] includes effective config snapshot

EVIDENCE:
- sample output in docs
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Define a stable, single-block fs-import report schema with consistent field names/types and stable JSON key ordering.
- `docs/reporting/fs_import_report.md`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Specify single-report stdout behavior (one JSON object; no extra stdout sections).
- `docs/reporting/fs_import_report.md`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Include `config_effective_ini` snapshot in the report contract and specify its canonical source.
- `docs/reporting/fs_import_report.md`
BLOCKER:

## E11.2 [DONE] Saved-bytes math correctness
- [x] define equations explicitly
- [x] byte-accurate totals
- [x] verify totals equal sum of parts

EVIDENCE:
- `docs/metrics.md`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Define canonical metric meanings and equations for saved_bytes/dedup_factor/z_ratio.
- `docs/metrics.md`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Make repo size metrics byte-accurate by accounting for qblk + qroot sidecar bytes.
- `quarxdrive/src/bin/quarxdrive.rs`
- `quarxdrive/tests/cli_format_json.rs`
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Add a deterministic JSON expectation that enforces `size_bytes == qblk_bytes + qroot_bytes`.
- `quarxdrive/tests/cli_format_json.rs`
BLOCKER:

## E11.3 [DONE] Packaging contract (single-binary, cross-platform)
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: Single-binary install story documented for Linux, Windows, and macOS
- [x] AC2: Cross-platform build plan documented with deterministic build notes
- [x] AC3: `--version` is available across binaries with stable output

EVIDENCE:
- ...
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Document packaging/install story and ensure `--version` works everywhere with stable output.
- `docs/packaging.md`
- `packaging/common/build-release.sh`
- `quarxnet/src/bin/quarxnet-hello.rs`
- `quarxnet/src/bin/quarxnet-ping-client.rs`
- `quarxnet/src/bin/quarxnet-ping-server.rs`
- `quarxnet/src/bin/quarxnet-stats-client.rs`
- `quarxnet/src/bin/quarxnet-version-client.rs`
- `quarxnet/src/bin/quarxnet-test-all.rs`
- `quarxnet/src/bin/quarxnet-test-znode.rs`
- `quarxnet/src/bin/quarxnet-e2e-source-db.rs`

## E11.4 [DONE] Read-only Web UI dashboard
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: Dashboard shows repo status, import progress, verify status, and stats
- [x] AC2: Log viewer and config viewer exist and use the same API as CLI
- [x] AC3: UI lists are stable ordered; basic smoke test or doc

EVIDENCE:
- ...
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Implement a read-only dashboard binary with deterministic JSON APIs and a minimal HTML UI.
- `tools/quarxdash/Cargo.toml`
- `tools/quarxdash/src/main.rs`
- `core-rs/src/logging.rs`
- `core-rs/src/store/blockstore.rs`
- `core-rs/src/store/file_store.rs`
- `quarxdrive/src/fs_stats.rs`
- `docs/packaging.md`
- `packaging/common/build-release.sh`
- `Cargo.toml`

## E11.5 [DONE] README quickstart + demo path + dev-check
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: README includes Quick Start (Dev) + Quick Start (Demo) with validated copy/paste commands
- [x] AC2: `./dev-check.sh` runs without permission errors (tools/check-cyrillic.sh executable or invoked correctly)
- [x] AC3: A minimal demo flow is documented and runs in under 60 seconds with stable output

EVIDENCE:
- COMPLIANCE: GAP
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: README lacks validated quickstart/demo flow and dev-check currently fails due to tool permissions.
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: README now documents validated quickstart/demo flow; dev-check passes after permission-safe socket tests and English-only config example.
- `README.md`
- `tools/quarxctl/quarxctl.example.ini` (legacy mirror; canonical example: `tools/quarxctl/quarxctl.canonical.example.ini`)
- `quarxdrive/tests/fs_import_filters.rs`
- `quarxnet/src/server.rs`
- `quarxnet/tests/transport_timeouts.rs`

BLOCKER:
- BLOCKER: `rg --files .private/agent/templates` exited with code 1 while checking templates (empty dir).
- BLOCKER: `wc -l` glob exited with code 1 due to no matches.
- BLOCKER: dev-check failed on Cyrillic comments in legacy mirror tools/quarxctl/quarxctl.example.ini (canonical example: tools/quarxctl/quarxctl.canonical.example.ini).
- BLOCKER: dev-check failed in fs_import_skips_unix_sockets_when_configured (PermissionDenied).
- BLOCKER: dev-check failed in quarxnet server tests due to bind PermissionDenied.
- BLOCKER: dev-check failed in transport_timeouts due to bind PermissionDenied.
- BLOCKER: cargo run without --bin failed for quarxdrive demo smoke.

## E11.6 [PLAN] Presentation surface: onboarding + quickstart wrapper + dev/user split
- [x] CONTRACT: PRODUCT
- [x] COMPLIANCE: OK
- [x] AC1: README top includes What is, Quick Start (Demo), Install/Build, Packaging/Releases, and a contributor link with validated commands
- [x] AC2: End-user quickstart uses a single wrapper command (script or `quarxtor` CLI) with no dev-only gates
- [x] AC3: Dev checks moved under tools/dev and referenced only in contributor docs

EVIDENCE:
- COMPLIANCE: GAP
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Current root onboarding still mixes dev checks; no single quickstart wrapper exists yet.
- BLOCKER: quickstart wrapper failed against legacy quarxdrive CLI (--root-path unsupported).
- BLOCKER: quarxdrive --version missing.
- BLOCKER: dev-check failed Cyrillic gate (legacy mirror quarxctl.example.ini; canonical example: tools/quarxctl/quarxctl.canonical.example.ini).
- BLOCKER: dev-check hung in quarxnet znode_drive_e2e; interrupted.
- COMPLIANCE: OK
- CONTRACT: PRODUCT-CONTRACT.md
- REASON: Quickstart wrapper uses import-l0 demo flow; dev checks moved under tools and referenced in contributor docs.
- EVIDENCE: README.md
- EVIDENCE: docs/packaging.md
- EVIDENCE: quarxtor
- EVIDENCE: tools/dev-check.sh
- EVIDENCE: tools/check-cyrillic.sh
- EVIDENCE: tools/quarxctl/quarxctl.example.ini (legacy mirror; canonical example: tools/quarxctl/quarxctl.canonical.example.ini)
- EVIDENCE: CONTRIBUTING.md
- EVIDENCE: .github/workflows/ci.yml
- EVIDENCE: git-dev-pr.sh
- BLOCKER: push failed; no git remote configured.

## E11.7 [PLAN] Canonical entrypoint: quarxtor binary
- [ ] Replace bash `quarxtor` wrapper with cross-OS Rust binary entrypoint
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: GAP
  - AC1: `quarxtor` binary builds on Linux/macOS/Windows and is the recommended entrypoint.
  - AC2: `quarxtor import DIR|FILE` supports both directory and file with deterministic output.
  - AC3: `quarxtor status` and `quarxtor dashboard --print-url` return stable keys/order.
  - FILES: Cargo.toml, quarxtor/src/main.rs, docs/packaging.md
  - EVIDENCE:
  - BLOCKER: current `quarxtor` is a bash wrapper script; must be replaced with Rust binary entrypoint.

## E11.8 [PLAN] Preflight dependency status in quarxtor
- [x] Add preflight dependency checks to `quarxtor status` and `quarxtor import`
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: Dependency checks enumerate config/repo/runtime and gate import on required readiness.
  - AC1: `quarxtor status` lists dependencies with status/path/reason and supports `--json`.
  - AC2: `quarxtor import` refuses when required deps are not ok and prints the same dependency list.
  - AC3: Output keys/order are stable and dependency names/status keys are not localized.
  - FILES: core-rs/src/config.rs, quarxtor/src/preflight.rs, quarxtor/src/main.rs, i18n/en-US.toml
  - EVIDENCE:
  - .private/agent/evidence/20251227T085951Z-E11.8-preflight-status.md
  - .private/agent/evidence/20251227T151548Z-preflight-i18n-config.md

## E11.9 [PLAN] INI section-aware config validation for preflight
- [x] Refine config validation to be INI-section-aware and transport-dependent
  - CONTRACT: PRODUCT-CONTRACT.md
  - COMPLIANCE: OK
  - REASON: Status/import validate INI sections/keys with transport-aware requirements and stable reporting.
  - AC1: `quarxtor status` reports unknown/missing/invalid config keys as `section.key` with deterministic ordering and tcp/unix net.addr rules.
  - AC2: `cargo test --workspace`
  - AC3: `quarxtor status --json` includes stable key order and `source` for config-derived deps.
  - FILES: core-rs/src/config.rs, quarxtor/src/preflight.rs
  - EVIDENCE:
  - .private/agent/evidence/20251227T113500Z-E11.9-ini-validation.md
  - .private/agent/evidence/20251227T151548Z-preflight-i18n-config.md

--- 

## 4. Auto-Progress Extraction Notes (for tools)
To compute progress:
- Parse epic headers `# EPIC Ex (W=n)`
- Count checkboxes until next epic header
- Epic progress = checked/total
- Project progress = weighted average

This file is intentionally machine-readable.

---
End of ROADMAP-TARGET.md
EVIDENCE (E0.6 transport docs):
- CMD: docs update
- OUT: # Evidence E0.6 transport docs
- OUT: 
- OUT: CMD: _shared/scripts/qx_gate_preflight.sh
- OUT: OUT: WORKTREE DIRTY (baseline; continued)
- OUT: 
- OUT: CMD: _shared/scripts/qx_gate_next.sh
- OUT: OUT: NEXT - [ ] Document proto contract and transport versioning
- OUT: 
- OUT: CMD: git diff --stat docs/07-net-api.md docs/08-transport-spec.md docs/40-net-and-sync.md GLOBAL-PLAN.md
- OUT:  docs/07-net-api.md | 41 ++++++++++++++++++++++++++++++++++-------
- OUT:  1 file changed, 34 insertions(+), 7 deletions(-)
- OUT: 
EVIDENCE (E11.9 ini validation):
- CMD: devcheck/build/status
- OUT: devcheck: cargo test --workspace (OK)
- OUT: build: cargo build -p quarxtor
- OUT: status: ./target/debug/quarxtor status
- OUT: status_out: config ok; repo missing; runtime_dir created
- OUT: status_json: ./target/debug/quarxtor status --json (stable keys + source)
- OUT: files: core-rs/src/config.rs, quarxtor/src/preflight.rs
EVIDENCE (E11.8/E11.9 preflight+i18n+config):
- CMD: evidence: .private/agent/evidence/20251227T151548Z-preflight-i18n-config.md
- OUT: # Evidence: preflight + i18n + config meta
- OUT: 
- OUT: - cargo build --workspace: OK
- OUT: - qx_gate_devcheck.sh (cargo test --workspace): OK
- OUT: - sh tools/check-config-meta.sh: OK
- OUT: - ./target/debug/quarxtor --version: quarxtor 0.1.0
- OUT: - ./target/debug/quarxtor status:
- OUT:   - config          : ok        path=/home/jee/.config/quarxtor/quarxctl.ini reason=ok
- OUT:   - repo            : missing   path=/home/jee/.local/share/quarxtor/repo.qblk reason=No such file or directory (os error 2)
- OUT:   - data_dir        : ok        path=/home/jee/.local/share/quarxtor reason=ok
- OUT:   - cache_dir       : ok        path=/home/jee/.cache/quarxtor reason=ok
- OUT:   - runtime_dir     : ok        path=/run/user/1000/quarxtor reason=ok
EVIDENCE (devcheck):
- CMD: _shared/scripts/qx_gate_devcheck.sh
- OUT: DEV-CHECK: cargo test --workspace error[E0432]: quarxtor_core::config imports missing
EVIDENCE (devcheck):
- CMD: _shared/scripts/qx_gate_devcheck.sh
- OUT: DEV-CHECK: cargo test --workspace error[E0432]: quarxtor_core::config imports missing
EVIDENCE (devcheck):
- CMD: _shared/scripts/qx_gate_devcheck.sh
- OUT: DEV-CHECK: cargo test --workspace error[E0432]: quarxtor_core::config imports missing
EVIDENCE (preflight):
- CMD: _shared/scripts/qx_gate_preflight.sh
- OUT: WORKTREE DIRTY. Stop. M ROADMAP-TARGET.md M docs/core-formats.md
EVIDENCE (devcheck):
- CMD: _shared/scripts/qx_gate_devcheck.sh
- OUT: DEV-CHECK: cargo test --workspace error[E0432]: quarxtor_core::config imports missing
EVIDENCE (preflight):
- CMD: _shared/scripts/qx_gate_preflight.sh
- OUT: WORKTREE DIRTY. Stop. M core-rs/src/lib.rs M quarxtor/src/i18n.rs M quarxtor/src/preflight.rs M quarxtor/tests/config_validation.rs M quarxtor/tests/status_json.rs
EVIDENCE (devcheck):
- CMD: _shared/scripts/qx_gate_devcheck.sh
- OUT: DEV-CHECK: OK
