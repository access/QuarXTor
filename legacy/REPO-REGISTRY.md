# QuarXTor - REPO REGISTRY (Single Source of Structural Truth)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.

Status: DRAFT (Phase B)
Scope: Repo structure, authority, governance, inventory, and dependency edges

## A) HUMAN CONTRACT LAYER

### Purpose and Scope
REPO-REGISTRY.md is the single source of structural truth for the QuarXTor information system.
It is a declarative map of what exists, where it lives, what it means, who owns authority,
how conflicts are resolved, and how agents must behave in this repository.
It is not a plan, a roadmap, a refactor tool, a cleanup script, or a future vision.

### Authority Model
Kinds: CONTRACT, POLICY, SPEC, PLAN, WORKFLOW, GUIDE, CODE, TEST, TOOLING, CONFIG, I18N, DATA/EVIDENCE
Authority levels: HIGHEST, HIGH, MEDIUM, LOW, NONE
Notes:
- source_of_truth is per-topic, not global.
- Authority conflicts are resolved ONLY via explicit OVERRIDES edges, never by implied precedence.

### Registry Field Definitions
- visibility: semantic audience boundary (public = repo/public docs/contracts/specs; private = internal-only material under .private/ or explicitly internal); do not infer from vcs_state or local git excludes.
- vcs_state: git index state in this checkout (mechanically derived):
  - tracked: git ls-files --error-unmatch <path> succeeds
  - untracked: exists but not tracked and not ignored/generated/external
  - ignored: untracked and (git check-ignore -q <path> true OR path under .private/)
  - generated: paths under target/ or target/tmp/ or other build outputs explicitly listed as generated in this registry
  - external: absolute path or symlink outside repo root
- git_tracked is not a field and must not be added to this registry.
- git push/publishability is not modeled as an artifact field.
- local git excludes (.git/info/exclude) do NOT define semantic visibility or policy.
- HARD RULES (vcs_state vs visibility):
  - visibility is semantic audience boundary (policy); NEVER inferred from vcs_state or local git excludes.
  - vcs_state is per-checkout mechanical state; not publishability, not policy, not "what will be pushed".
  - .git/info/exclude / local ignores MUST NOT affect visibility semantics or policy.
  - if visibility=public with vcs_state=ignored (or vice versa) is intentional, add a one-line note in notes explaining why; otherwise set evidence_gap: true with reason "visibility/vcs_state mismatch; needs human decision".
- tags: navigation buckets (5-12), derived from concepts; minimize overlap; ASCII only; forbid command lines, flags, and shebangs; list syntax [a, b, c].
- concepts: content-derived keys (identifiers/paths allowed), 8-80; ASCII only; forbid command lines and shebangs; list syntax [a, b, c].
- generated artifacts are informational only and NEVER authority-bearing; they cannot be canonical config/spec sources.
- visibility and vcs_state are orthogonal; do not infer one from the other.
- if they appear inconsistent and there is no explicit one-line note explaining why, set evidence_gap: true with reason "visibility/vcs_state mismatch; needs human decision".

### PRODUCT INVARIANTS (LOCKED)
- Cross-OS product requirement: canonical entrypoint is cross-OS binary `quarxtor`. Not a shell wrapper.
- Primary UX invariant: `quarxtor import DIR|FILE` must support both directory and file.
- Single user entrypoint concept: other binaries can exist but do not replace `quarxtor`.
- All axes are main and equal. No silent disablement. FUSE is a mandatory direction (even if unavailable on some platforms).
- There are zero disableable components in QuarXTor. Every axis is mandatory.
- Absence is explicit and reported deterministically; there is no implicit disablement.
- FUSE "when unavailable" behavior must be deterministic and documented.
- Root hygiene: no *.sh in repo root. Dev tooling scripts must live under tools/dev/** or another explicit dev-only area.
- Config format: product config is INI (NOT TOML). Canonical runtime config is quarxctl.ini (platform-equivalent path).
- i18n is mandatory: separate locale dictionaries, default en-US; locale selection is deterministic (QUARX_LANG -> LANG -> ini [ui].lang -> en-US).
- i18n keyspace must be collision-safe: keys MUST be namespaced per component; forbid generic cross-component keys (like "usage") without component namespace.
- Transport framing canonicalization: if there is a header-size conflict, registry must declare a single canonical framing and mark others as legacy notes only.
- Plans are NOT authoritative contracts (plans cannot override contracts or specs).

### REGISTRY GOVERNANCE (CONTRACT)
1) How artifacts are ADDED
- requires file existence
- requires explicit description
- requires authority classification
- requires evidence-based semantics

2) How artifacts are MODIFIED
- modifications MUST preserve history
- semantic meaning changes MUST be documented
- authority changes MUST be explicit
- append-only memory rule: prior registry entries are never removed, only marked legacy or deprecated

3) How artifacts are DEPRECATED
- status change only
- never removed
- replacement must be referenced

4) How artifacts are REMOVED
- THEY ARE NOT
- historical presence is permanent

5) How conflicts are resolved
- ONLY via OVERRIDES edges
- higher authority wins
- conflicts MUST be listed explicitly

6) Codex behavioral constraints
- registry MUST be read before any task
- registry violations are STOP conditions
- registry silence != permission

### Known Conflicts Index
#### C01 Entrypoint naming and import UX
Topic: quarxtor vs quarxctl/quarx entrypoints and import command shape
Conflicting artifacts: PRODUCT-CONTRACT.md, README.md
Evidence:
- PRODUCT-CONTRACT.md: "- `quarxtor` is the canonical, cross-OS binary entrypoint."
- PRODUCT-CONTRACT.md: "- `quarxtor import DIR|FILE` is the primary workflow."
- README.md: `./target/release/quarxctl "${DEMO_ROOT}/repo.qblk" fs-import "${DEMO_ROOT}/data"`
Current canonical stance: product invariants require `quarxtor` as the canonical entrypoint and `quarxtor import DIR|FILE`.
Migration notes: update README and any doc/workflow examples that treat quarxctl/quarx as the primary user entrypoint.

#### C02 Root hygiene (no root .sh)
Topic: root scripts forbidden vs root wrappers present
Conflicting artifacts: AUTHORITY-MAP.md, quarxtor.sh, git-dev-pr.sh
Evidence:
- AUTHORITY-MAP.md: "- Repo root contains no `*.sh` (and no dev-only helpers)."
Current canonical stance: root *.sh is forbidden; wrappers are legacy only and must not be product entrypoints.
Migration notes: move or replace root wrappers under tools/dev/** or packaging/** and mark legacy as needed.

#### C03 Config canonical example resolution (single canonical)
Topic: canonical example selection resolved; legacy mirrors marked non-canonical
Related artifacts (canonical + legacy mirrors): CONTRIBUTING.md, quarxctl.example.ini, tools/quarxctl/quarxctl.example.ini, tools/quarxctl/quarxctl.canonical.example.ini
Evidence:
- CONTRIBUTING.md: "- `tools/quarxctl/quarxctl.canonical.example.ini` is the canonical example config."
- quarxctl.example.ini: "# QuarXTor example config (legacy mirror; canonical example is tools/quarxctl/quarxctl.canonical.example.ini)."
- tools/quarxctl/quarxctl.example.ini: "# QuarXTor example config (legacy mirror; canonical example is tools/quarxctl/quarxctl.canonical.example.ini)."
- tools/quarxctl/quarxctl.canonical.example.ini: "# QuarXTor canonical config (v2)"
Current canonical stance: canonical example is tools/quarxctl/quarxctl.canonical.example.ini; tools/quarxctl/quarxctl.example.ini and quarxctl.example.ini are legacy/mirrors and MUST NOT claim canonical status (see REGISTRY::CANONICAL_CONFIG).
Resolution note: legacy mirrors and docs are labeled non-canonical and point to the canonical example.

#### C04 FUSE axis mandatory and availability semantics
Topic: FS-view mandatory axis and availability semantics
Conflicting artifacts: AUTHORITY-MAP.md, PRODUCT-CONTRACT.md, ROADMAP.md
Evidence:
- AUTHORITY-MAP.md: "- FS-view (FUSE/FS backend) is a first-class main axis of the product, mandatory."
- PRODUCT-CONTRACT.md: "- A read-only FUSE drive view is a mandatory axis; mounting is explicitly enabled."
- ROADMAP.md: "- FUSE mount implementation (read-only)."
Current canonical stance: FUSE is a mandatory axis; availability is conditional but must be deterministic when unavailable.
Migration notes: reconcile any remaining language to match the mandatory axis model.

#### C05 Transport framing (6-byte vs 8-byte header)
Topic: proto0 header size references
Conflicting artifacts: docs/08-transport-spec.md, docs/net/protocol.md
Evidence:
- docs/08-transport-spec.md: "Proto0 uses an 8-byte header:"
- docs/net/protocol.md: "- 6-byte header (`kind`, `flags`, `length` as u32 big-endian) is historical and must not be used for proto0."
Current canonical stance: 8-byte header is canonical; 6-byte framing is legacy-only and must be labeled as such.
Migration notes: keep legacy note clearly labeled and avoid implying dual support.

#### C06 Public vs private boundary
Topic: public docs referencing .private paths
Conflicting artifacts: AUTHORITY-MAP.md, AGENTS.md, docs/bench/reimport_identical.md
Evidence:
- AUTHORITY-MAP.md: "- Public docs must not depend on `.private` existence."
- AGENTS.md: "1) Read `.private/agent/entry.md` (authoritative)."
- docs/bench/reimport_identical.md: "Root: `.private/agent/workspace/e4-reimport-identical/data`"
Current canonical stance: public docs must not depend on .private; any .private references must be non-required developer workflow only.
Migration notes: move or rewrite public references to .private into private workflow docs.

#### C07 i18n key namespace and locale selection
Topic: namespace model and locale selection sources
Conflicting artifacts: I18N-VISION.md, AUTHORITY-MAP.md, docs/config-reference.md
Evidence:
- I18N-VISION.md: "- `cli.usage.main`"
- I18N-VISION.md: "- `cli.error.unknown_command`"
- AUTHORITY-MAP.md: "- `quarxtor.usage.*`"
- AUTHORITY-MAP.md: "- `quarxctl.*`"
- I18N-VISION.md: "- Environment overrides: `QUARX_LANG`, then `LANG` as a fallback."
- docs/config-reference.md: "| ui.lang | string | en-US | `QUARX_LANG` | IMPLEMENTED | Locale tag for CLI/UI output (BCP-47). |"
Current canonical stance: component-scoped keys are required; locale selection order is QUARX_LANG -> LANG -> [ui].lang -> en-US.
Migration notes: align namespace examples and document locale selection order consistently.

#### C08 Conflict handling (stop vs no-stop)
Topic: stop-and-report conflicts vs never stop execution
Conflicting artifacts: AUTHORITY-MAP.md, AGENTS.override.md
Evidence:
- AUTHORITY-MAP.md: "- STOP and report conflicts"
- AGENTS.override.md: "- NEVER stop execution due to git status, dirty tree, permissions, missing tools/deps, failing commands/tests, or environment limits."
Current canonical stance: registry governance requires STOP on registry violations; conflicts must be listed and resolved via OVERRIDES edges.
Migration notes: reconcile workflow docs to avoid simultaneous stop and no-stop mandates.

### Legacy Provisions (from prior registry; preserved, superseded where noted)

#### Legacy phased work rule (superseded by current governance but preserved)
- Phase A: audit-only (read-only)
- Phase B: draft changes (limited scope)
- Phase C: ratify (raise authority + wire into startup)
- Phase D: enforce (tests, lints, skills)
Phases MUST NOT be mixed in one run.

#### Legacy governance notes (superseded by REGISTRY GOVERNANCE (CONTRACT))
Adding an artifact requires:
1) Create or modify the artifact itself (out of scope for this phase)
2) Add ARTIFACT entry in this registry
3) Add EDGES: DEPENDS_ON, OVERRIDES, IMPLEMENTS
4) Update Acceptance Checklist section

Removing an artifact requires:
1) Mark deprecated in this registry with replacement
2) Remove references to it
3) Delete it in a later changeset

Changing authority or source_of_truth requires:
- Update OVERRIDES edges
- Update conflict notes where relevant

#### Legacy milestones (relative)
- T0: draft exists
- T1: draft ratified
- T2: startup reads registry
- T3: lint or enforcement exists
- T4: docs aligned and conflicts closed

#### Legacy acceptance checklist
- Inventory coverage: all authority-bearing *.md across root, docs, .private, tools are listed.
- Every source_of_truth topic has explicit OVERRIDES edges.
- Every major component has defined entrypoints and config ownership.
- No contradictions inside the registry itself.
- Public and private boundary declared and no public docs depend on .private.

#### Legacy gaps (record only; can be resolved by current registry)
- i18n dictionaries (non-md artifacts) not represented as ARTIFACT entries
- code-level ownership edges for runtime behavior (code artifacts not listed here)

## B) MACHINE-READABLE REGISTRY


```text
REGISTRY::MANIFEST
registry_version: 1
scope: repo structure, authority, governance, inventory, and dependency edges
non_goals: plan, roadmap, refactor, cleanup, future vision
behavioral_guards: registry-read-before-task; stop-on-violation; registry-silence-not-permission
```

```text
REGISTRY::ARTIFACTS
ARTIFACT
id: root_agents_md
path: AGENTS.md
exists: yes
vcs_state: ignored
kind: WORKFLOW
authority: HIGH
scope: dev
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: AGENTS.md (FULL, updated)
tags: [AGENTS (Local Only), Startup sequence (mandatory), .private/agent/entry.md, ROADMAP-TARGET.md, ARCH-TARGET.md]
concepts: [AGENTS (Local Only), Startup sequence (mandatory), .private/agent/entry.md, ROADMAP-TARGET.md, ARCH-TARGET.md, ARCH.md, docs/, Cleanliness (hard rules), codex, .codex, autopilot, CodexPrompt, .private/agent/**, .git/info/exclude, .gitignore, Repository hygiene and checkpoints, tools/dev/**, .private/**, tools/, packaging/, ../qx-run/, Repo snapshot & invariants, quarxproto]
evidence_gap: true
notes: visibility/vcs_state mismatch: needs human decision

ARTIFACT
id: root_agents_override_md
path: AGENTS.override.md
exists: yes
vcs_state: ignored
kind: WORKFLOW
authority: HIGH
scope: dev
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: AGENTS Override (Local Only)
tags: [AGENTS Override (Local Only), Mandatory skill workflow (dev only), tools/dev/check-root-hygiene.sh, *.sh, codex]
concepts: [AGENTS Override (Local Only), Mandatory skill workflow (dev only), tools/dev/check-root-hygiene.sh, *.sh, codex, chatgpt, ChatGPT, ChatGPT5, ChatGPT-5, *chatgpt*, .codex, autopilot, CodexPrompt, .gitignore, .git/info/exclude, .private/, AGENTS*.md, tools/dev/**, .private/**]
evidence_gap: true
notes: visibility/vcs_state mismatch: needs human decision

ARTIFACT
id: root_arch_target_md
path: ARCH-TARGET.md
exists: yes
vcs_state: tracked
kind: CONTRACT
authority: HIGHEST
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: architecture_targets, layering_invariants
summary: ARCH-TARGET.md
tags: [ARCH-TARGET.md, 0. Purpose, 1. Terminology & Naming, u64, L0]
concepts: [ARCH-TARGET.md, 0. Purpose, 1. Terminology & Naming, u64, L0, Multi, Z, Object, 2. High-Level System Goal, 3. Layered Architecture (Hard Invariant), Invariants, core-rs, 4. Repository Model, 4.1 `repo.qblk`, repo.qblk, 4.2 `repo.qroot`, repo.qroot, Guarantees, 5.1 External Identity vs Internal IDs, BlockId, 5.2 Index Principle, Vec, 5.3 Determinism, 6. Block & Graph Model]
evidence_gap: false
notes:

ARTIFACT
id: root_arch_md
path: ARCH.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor Architecture
tags: [QuarXTor Architecture, QuarXTor/QuarXTor, Components and layering, core-rs/, quarxnet/]
concepts: [QuarXTor Architecture, QuarXTor/QuarXTor, Components and layering, core-rs/, quarxnet/, quarxdrive/, quarxcloud/, quarxproto/, tools/, core-py/, core-rs, quarxnet, quarxdrive, quarxcloud, tools, quarxproto, 1. Design Goals, 2. High-Level Components, 2.1 QuarXCore (Engine), docs/core-formats.md, repo_root, [import].root, [paths].data_dir/cache_dir/log_dir/runtime_dir]
evidence_gap: false
notes:

ARTIFACT
id: root_authority_map_md
path: AUTHORITY-MAP.md
exists: yes
vcs_state: tracked
kind: POLICY
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: yes
truth_for: authority_order, protected_paths, boundary_policy
summary: AUTHORITY-MAP.md
tags: [quarxtor, AUTHORITY-MAP.md, 0) Core Product Invariants (Non-Negotiable), 0.1 Single user entrypoint, *.sh]
concepts: [AUTHORITY-MAP.md, 0) Core Product Invariants (Non-Negotiable), 0.1 Single user entrypoint, quarxtor, *.sh, quarxctl, 0.2 Import UX invariant, 0.3 Cross-OS is mandatory, 0.4 FUSE / FS-view is a mandatory axis, 0.5 Multilingual (i18n) is a mandatory invariant, en-US, 0.6 No root scripts / clean root policy, tools/dev/**, .private/**, 1) Authority Order (Conflict Resolution), 1.1 Target architecture & immutable product contract, ARCH-TARGET.md, PRODUCT-CONTRACT.md, FORMAT.md, docs/08-transport-spec.md, 1.2 Supporting "must-align" documents, I18N-VISION.md, docs/**, 1.3 Agent/operator workflow rules]
evidence_gap: false
notes:

ARTIFACT
id: root_contributing_md
path: CONTRIBUTING.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: dev
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Overview
tags: [Overview, docs/, Configuration invariants, QuarxConfig::validate(), repo_root]
concepts: [Overview, docs/, Configuration invariants, QuarxConfig::validate(), repo_root, [import].root, [paths].data_dir/cache_dir/log_dir/runtime_dir, docs/config-reference.md, docs/paths-and-repo.md, Example config, quarxctl.example.ini, QuarxConfig, core-rs/src/config.rs, Dev workflow, .rs, docs/*.md, .ini, .toml, CI, .github/workflows/ci.yml, push, pull_request]
evidence_gap: false
notes:

ARTIFACT
id: root_format_md
path: FORMAT.md
exists: yes
vcs_state: tracked
kind: CONTRACT
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: format_invariants
summary: FORMAT.md
tags: [FORMAT.md, 1. Scope and Authority, docs/core-formats.md, 2. Canonical Source, core-rs/src/block/*]
concepts: [FORMAT.md, 1. Scope and Authority, docs/core-formats.md, 2. Canonical Source, core-rs/src/block/*, core-rs/src/codec/*, core-rs/src/store/*, 3. Invariant Index (Stable Order), 3.1 FORMAT-CORE-ABI, 3.2 FORMAT-BLOCK-KINDS, BlockKind, 3.3 FORMAT-L0, 3.4 FORMAT-MULTI, 3.5 FORMAT-Z, 3.6 FORMAT-OBJECT, 3.7 FORMAT-STORE, 3.8 FORMAT-EXTENSION]
evidence_gap: false
notes:

ARTIFACT
id: root_global_plan_md
path: GLOBAL-PLAN.md
exists: yes
vcs_state: tracked
kind: PLAN
authority: LOW
scope: dev
visibility: public
status: legacy
source_of_truth: no
truth_for:
summary: GLOBAL PLAN
tags: [quarxtor, GLOBAL PLAN, Phase 1: Build (DONE), quarxtor import, quarxtor status]
concepts: [GLOBAL PLAN, Phase 1: Build (DONE), quarxtor, quarxtor import, quarxtor status, Phase 2: System (IN PROGRESS), Phase 3: Ecosystem (UPCOMING)]
evidence_gap: true
notes: concepts_insufficient: 1

ARTIFACT
id: root_i18n_vision_md
path: I18N-VISION.md
exists: yes
vcs_state: tracked
kind: POLICY
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: i18n_policy, locale_rules, key_namespace
summary: I18N-VISION.md
tags: [I18N-VISION.md, 1. Purpose, 2. Key Naming Conventions, cli.usage.main, cli.error.unknown_command]
concepts: [I18N-VISION.md, 1. Purpose, 2. Key Naming Conventions, cli.usage.main, cli.error.unknown_command, status.check.config.label, status.state.ok, usage, error, 3. Supported Locales Policy, en-US, et-EE, ru-RU, 4. Fallback Behavior, 5. CLI Output Language Policy, [ui].lang, QUARX_LANG, LANG, 6. Determinism and Output Stability]
evidence_gap: false
notes:

ARTIFACT
id: root_product_contract_md
path: PRODUCT-CONTRACT.md
exists: yes
vcs_state: tracked
kind: CONTRACT
authority: HIGHEST
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: product_ux, entrypoint, import_ux, config_contract
summary: PRODUCT-CONTRACT.md
tags: [quarxtor, PRODUCT-CONTRACT.md, 1. Purpose and Authority, 2. Canonical Entrypoint, 3. Primary UX Invariant]
concepts: [PRODUCT-CONTRACT.md, 1. Purpose and Authority, 2. Canonical Entrypoint, quarxtor, 3. Primary UX Invariant, 4. Daemon Model and Lifecycle, quarxnetd, quarxcloudd, 5. Status Contract, quarxtor status, 6. Dashboard Contract, quarxtor dashboard, 7. Filesystem and Import Contract, mount, 8. Determinism Contract, 9. Config Contract, docs/paths-and-repo.md, quarxtor config print, 10. Logging Contract, operation_id, 11. Time and Timeouts, 12. Repo Lifecycle, 13. Filesystem Layer Policies]
evidence_gap: false
notes:

ARTIFACT
id: root_readme_md
path: README.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor — Hash-Graph Storage Engine
tags: [quarxtor, Quickstart: Mount a QuarXTor repo as a FUSE filesystem, create/open repo.qblk + repo.qroot, mount via FUSE (unavailable in this build), verify repo status]
concepts: [Quickstart: Mount a QuarXTor repo as a FUSE filesystem, create/open repo.qblk + repo.qroot, mount via FUSE (unavailable in this build), verify repo status, Onboarding, What is QuarXTor, Quick Start (Demo), quarxtor, Install / Build, Packaging / Releases, docs/packaging.md, Contributor / Dev, CONTRIBUTING.md, tools/dev-check.sh, Deduplication that *actually* works, Verified storage savings, Predictable on-disk ABI, docs/core-formats.md, High ingest throughput, Transparent reads, quarxdrive/, quarxnet/, ARCH.md]
evidence_gap: false
notes:

ARTIFACT
id: root_repo_registry_md
path: REPO-REGISTRY.md
exists: yes
vcs_state: tracked
kind: POLICY
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: yes
truth_for: registry_map, governance, inventory, edges
summary: QuarXTor - REPO REGISTRY (Single Source of Structural Truth)
tags: [quarxtor, QuarXTor - REPO REGISTRY (Single Source of Structural Truth), A) HUMAN CONTRACT LAYER, Purpose and Scope, Authority Model]
concepts: [QuarXTor - REPO REGISTRY (Single Source of Structural Truth), A) HUMAN CONTRACT LAYER, Purpose and Scope, Authority Model, Registry Field Definitions, PRODUCT INVARIANTS (LOCKED), quarxtor, REGISTRY GOVERNANCE (CONTRACT), Known Conflicts Index, C01 Entrypoint naming and import UX, C02 Root hygiene (no root .sh), *.sh, C03 Config canonical example resolution (single canonical), quarxctl.example.ini, C04 FUSE axis mandatory, C05 Transport framing (6-byte vs 8-byte header), kind, flags, length, C06 Public vs private boundary, .private, .private/agent/entry.md, .private/agent/workspace/e4-reimport-identical/data]
evidence_gap: false
notes:

ARTIFACT
id: root_roadmap_target_md
path: ROADMAP-TARGET.md
exists: yes
vcs_state: tracked
kind: WORKFLOW
authority: HIGH
scope: dev
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: ROADMAP-TARGET.md
tags: [ROADMAP-TARGET.md, 0. Auto-Progress Rules (Hard), 0.1 Units of Progress, [ ], [x]]
concepts: [ROADMAP-TARGET.md, 0. Auto-Progress Rules (Hard), 0.1 Units of Progress, [ ], [x], DONE: 3/10, 0.2 Global Project Progress Formula, Ei, Wi, Pi, [0..1], 0.3 Epic Progress Formula, Ai, Pi = checked_items / total_items, DONE: a/b, 0.4 Definition of DONE, 0.5 Evidence Format, EVIDENCE: path/to/file.ext, 0.6 Scope Lock]
evidence_gap: false
notes:

ARTIFACT
id: root_roadmap_md
path: ROADMAP.md
exists: yes
vcs_state: tracked
kind: PLAN
authority: LOW
scope: dev
visibility: public
status: legacy
source_of_truth: no
truth_for:
summary: QuarXTor Roadmap
tags: [QuarXTor Roadmap, quarxproto, QuarXCore, list-blocks, list-files]
concepts: [QuarXTor Roadmap, quarxproto, QuarXCore, list-blocks, list-files, stats, QuarXNet, PUT /object, GET /object, GET /stats, QuarXDrive, quarx stats, QuarXCloud, Tracking, ARCH.md, docs/]
evidence_gap: false
notes:

ARTIFACT
id: root_tools_recipes_md
path: TOOLS-RECIPES.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: dev
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: TOOLS-RECIPES.md
tags: [TOOLS-RECIPES.md, Conventions, QXCTL, quarxctl, STORE]
concepts: [TOOLS-RECIPES.md, Conventions, QXCTL, quarxctl, STORE, .qblk, .qroot, SRC, OUT, fs-get, Recipe: fs-import (directory), Recipe: fs-tree (root listing), UID, TYPE, NAME, Recipe: fs-stats (repo verification summary), Recipe: fs-get (retrieve a file), Deterministic output expectations, fs-import, fs-stats, dirs, files, bytes]
evidence_gap: false
notes:

ARTIFACT
id: docs_00_overview_md
path: docs/00-overview.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Overview
tags: [Overview]
concepts: [Overview]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: docs_01_block_model_md
path: docs/01-block-model.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Block Model
tags: [Block Model, Block Structure, id, size, tier]
concepts: [Block Model, Block Structure, id, size, tier, refs, entropy, meta, Chunking, On-Disk Layout, segment_0001.dat, segment_0002.dat]
evidence_gap: false
notes:

ARTIFACT
id: docs_02_hash_graph_md
path: docs/02-hash-graph.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Hash Graph
tags: [Hash Graph, Vertices, Edges, Purpose, Storage]
concepts: [Hash Graph, Vertices, Edges, Purpose, Storage]
evidence_gap: true
notes: concepts_insufficient: 3

ARTIFACT
id: docs_03_reference_matrix_md
path: docs/03-reference-matrix.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Reference Matrix
tags: [Reference Matrix, Conceptual Model, Physical Implementation, Purpose]
concepts: [Reference Matrix, Conceptual Model, Physical Implementation, Purpose]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: docs_04_tiering_md
path: docs/04-tiering.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Tiering (RAM / SSD / HDD)
tags: [Tiering (RAM / SSD / HDD), Tiers, Promotion, Demotion]
concepts: [Tiering (RAM / SSD / HDD), Tiers, Promotion, Demotion]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: docs_05_telemetry_md
path: docs/05-telemetry.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Telemetry
tags: [Telemetry, Block-Level Metrics, Engine-Level Metrics, Export Formats]
concepts: [Telemetry, Block-Level Metrics, Engine-Level Metrics, Export Formats]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: docs_06_core_api_md
path: docs/06-core-api.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: QuarXCore API
tags: [QuarXCore API, Functions, ingest(path), fetch(object_id), stats()]
concepts: [QuarXCore API, Functions, ingest(path), fetch(object_id), stats(), delete(object_id), Stability]
evidence_gap: true
notes: concepts_insufficient: 1

ARTIFACT
id: docs_07_net_api_md
path: docs/07-net-api.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: yes
truth_for: net_api, transport_families
summary: QuarXNet API (RPC / HTTP)
tags: [QuarXNet API (RPC / HTTP), HTTP endpoints (not implemented), PUT /object, GET /stats, Proto / ZNode]
concepts: [QuarXNet API (RPC / HTTP), HTTP endpoints (not implemented), PUT /object, GET /stats, Proto / ZNode, quarxproto, client_push_znode, client_get_znode, quarxnet, quarxdrive, Proto0 framing (implemented), QXRT, Versioning and compatibility, Transport families and security status, Deterministic error/status outputs, bad magic, bad version, unexpected opcode/len]
evidence_gap: false
notes:

ARTIFACT
id: docs_08_drive_integration_md
path: docs/08-drive-integration.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: yes
truth_for: drive_integration, fuse_behavior
summary: QuarXDrive – Clients and Integration
tags: [CLI Surface, quarxtor status, quarxtor dashboard, Filesystem Integration (Read-only FUSE), Path mapping (deterministic)]
concepts: [CLI Surface, quarxtor status, quarxtor dashboard, Filesystem Integration (Read-only FUSE), Path mapping (deterministic), by-path/, by-id/, meta.json, Dependencies, fuse3, macFUSE, WinFsp, Unmount, SDKs, ZNode / Proto, quarxproto, quarxdrive, quarxnet, client_push_znode]
evidence_gap: false
notes:

ARTIFACT
id: docs_08_transport_spec_md
path: docs/08-transport-spec.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: transport_framing
summary: Transport Spec (Proto0)
tags: [Transport Spec (Proto0), Framing, QXRT, Versioning and compatibility, bad version]
concepts: [Transport Spec (Proto0), Framing, QXRT, Versioning and compatibility, bad version, Transport mapping, quarxnetd, net.addr, net.transport=unix, Security status, Determinism]
evidence_gap: false
notes:

ARTIFACT
id: docs_09_cloud_topology_md
path: docs/09-cloud-topology.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: QuarXCloud – Cloud / Multi-Node Topology
tags: [Nodes, Replication, Health, Deployment Models]
concepts: [Nodes, Replication, Health, Deployment Models]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: docs_10_drive_import_md
path: docs/10-drive-import.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: import_pipeline, import_keys
summary: Drive import pipeline
tags: [quarxtor, Drive import pipeline, 1. Scope, 2. Pipeline stages (ordered), core.repo_root]
concepts: [Drive import pipeline, 1. Scope, quarxtor, 2. Pipeline stages (ordered), core.repo_root, repo.qblk, repo.qroot, 3. Inputs and outputs, 3.1 User command surface, quarxtor status, 3.2 Internal components (high-level), quarxnetd, quarxtor_core, 3.3 Repository artifacts, 4. Required configuration keys, 4.1 Repository and path discovery, paths.data_dir, paths.cache_dir, paths.log_dir, paths.runtime_dir, docs/paths-and-repo.md, 4.2 Import options]
evidence_gap: false
notes:

ARTIFACT
id: docs_40_net_and_sync_md
path: docs/40-net-and-sync.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Net and Sync (Current State)
tags: [Net and Sync (Current State), Implemented, quarxnetd, Not implemented, Determinism]
concepts: [Net and Sync (Current State), Implemented, quarxnetd, Not implemented, Determinism, docs/08-transport-spec.md]
evidence_gap: true
notes: concepts_insufficient: 2

ARTIFACT
id: docs_plan_global_md
path: docs/PLAN-GLOBAL.md
exists: yes
vcs_state: tracked
kind: PLAN
authority: LOW
scope: dev
visibility: public
status: legacy
source_of_truth: no
truth_for:
summary: QuarXTor Global Plan (plan snapshot)
tags: [QuarXTor Global Plan (plan snapshot), Vision & Scope, Module overview (role / deps / maturity), Current progress snapshot, Next milestones]
concepts: [QuarXTor Global Plan (plan snapshot), Vision & Scope, Module overview (role / deps / maturity), Current progress snapshot, Next milestones]
evidence_gap: true
notes: concepts_insufficient: 3

ARTIFACT
id: docs_paths_and_repo_md
path: docs/paths-and-repo.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: path_rules, repo_layout
summary: Appendix A. QuarXTor path and repository model
tags: [Appendix A. QuarXTor path and repository model, A.1. Terms and invariants, [core].repo_root, [import].root, [paths].data_dir/cache_dir/log_dir/runtime_dir]
concepts: [Appendix A. QuarXTor path and repository model, A.1. Terms and invariants, [core].repo_root, [import].root, [paths].data_dir/cache_dir/log_dir/runtime_dir, QuarxConfig::validate(), [paths].data_dir, [paths].cache_dir, [paths].log_dir, quarxnetd, quarxcloudd, drived, [paths].runtime_dir, core.repo_root, import.root, repo_root = /data/repo, import.root = /data, repo_root, data_dir, cache_dir, log_dir, runtime_dir, A.2. Path model: OS mapping, A.2.1. Linux (user scope)]
evidence_gap: false
notes:

ARTIFACT
id: docs_packaging_md
path: docs/packaging.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Packaging & Installation (Single-Binary)
tags: [Packaging & Installation (Single-Binary), What gets built, dist/bin/, quarxdrive, quarxctl]
concepts: [Packaging & Installation (Single-Binary), What gets built, dist/bin/, quarxdrive, quarxctl, quarxnetd, quarxdash, FUSE quickstart prerequisites (read-only drive view), fuse3, macFUSE, WinFsp, quarxtor status, Linux (install story), dist/, bin/, macOS (install story), PATH, ~/bin, ~/.local/bin, Windows (install story), .exe, target\\release\\]
evidence_gap: false
notes:

ARTIFACT
id: docs_index_md
path: docs/index.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: product
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor Documentation Index
tags: [QuarXTor Documentation Index, Contents, quarxctl cfg, ~/.config/quarxtor/quarxctl.ini]
concepts: [QuarXTor Documentation Index, Contents, quarxctl cfg, ~/.config/quarxtor/quarxctl.ini]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: docs_core_formats_md
path: docs/core-formats.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: format_details
summary: QuarXTor core on-disk formats
tags: [QuarXTor core on-disk formats, General invariants, core-rs/src/store/*.rs, core-rs/src/codec/*.rs, Block model overview]
concepts: [QuarXTor core on-disk formats, General invariants, core-rs/src/store/*.rs, core-rs/src/codec/*.rs, Block model overview, BlockKind, core-rs/src/types.rs, L0Block, core-rs/src/block/l0.rs, MultiBlock, MultiRecipe, CodecRef, DictRef, core-rs/src/block/multi.rs, ZBlock, core-rs/src/block/zblock.rs, ZPayload, core-rs/src/codec/z.rs, Object, core-rs/src/block/object.rs, ObjectPayload, core-rs/src/codec/object.rs, L0 blocks, core-rs/src/codec/l0.rs]
evidence_gap: false
notes:

ARTIFACT
id: docs_config_reference_md
path: docs/config-reference.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: yes
truth_for: config_schema
summary: QuarXTor configuration contract
tags: [QuarXTor configuration contract, core-rs/src/config.rs, Key reference, QUARX_L0_CHUNK, QUARX_RAM_LIMIT]
concepts: [QuarXTor configuration contract, core-rs/src/config.rs, Key reference, QUARX_L0_CHUNK, QUARX_RAM_LIMIT, QUARX_ANALYSIS_ENABLE_ZNODE, QUARX_ANALYSIS_FS_STATS_FALLBACK, QUARX_REPO_ROOT, QUARX_DATASET_DIR, QUARX_IMPORT_MIN_FILE_SIZE, QUARX_IMPORT_SKIP_HIDDEN, QUARX_IMPORT_SKIP_SYMLINK, QUARX_IMPORT_SKIP_ZERO, QUARX_IMPORT_SKIP_DEVICES, QUARX_IMPORT_SKIP_SPECIAL, QUARX_IMPORT_INCLUDE_GLOB, QUARX_IMPORT_EXCLUDE_GLOB, QUARX_IMPORT_INCLUDE_DIR_GLOB, QUARX_IMPORT_EXCLUDE_DIR_GLOB, QUARX_IMPORT_GLOB_MODE, QUARX_IMPORT_USE_Z, QUARX_IMPORT_Z_THRESHOLD, QUARX_DATA_DIR, QUARX_CACHE_DIR]
evidence_gap: false
notes:

ARTIFACT
id: docs_metrics_md
path: docs/metrics.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: metric_definitions
summary: Metrics & Equations (Canonical Definitions)
tags: [Metrics & Equations (Canonical Definitions), Byte accounting, `logical_bytes`, logical_bytes, fs-import]
concepts: [Metrics & Equations (Canonical Definitions), Byte accounting, `logical_bytes`, logical_bytes, fs-import, import-l0, stats.total_input_bytes, `stored_bytes`, stored_bytes, stats.unique_bytes, repo.qblk, repo.qroot, `saved_bytes`, saved_bytes, `dedup_factor`, dedup_factor, dedup_factor = logical_bytes / stored_bytes, dedup_factor = 1.0, Z-block metrics, `z_bytes`, z_bytes, `z_ratio`, z_ratio]
evidence_gap: false
notes:

ARTIFACT
id: docs_determinism_test_pack_md
path: docs/determinism-test-pack.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: MEDIUM
scope: dev
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Determinism test pack
tags: [Determinism test pack, Fixtures, Minimal dataset (fs-import), a.txt, b"a"]
concepts: [Determinism test pack, Fixtures, Minimal dataset (fs-import), a.txt, b"a", b.txt, b"bb", Command coverage, CLI output determinism (byte-identical), quarxdrive, quarxdrive/tests/cli_format_json.rs, Repo artifact determinism (byte-identical), repo init, repo.qblk, repo.qroot, quarxdrive/tests/determinism_pack.rs, How to run]
evidence_gap: false
notes:

ARTIFACT
id: docs_format_versions_md
path: docs/format/versions.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: format_versions
summary: QuarXTor format versions
tags: [QuarXTor format versions, 1, 0]
concepts: [QuarXTor format versions, 1, 0]
evidence_gap: true
notes: tags_insufficient: 2; concepts_insufficient: 5

ARTIFACT
id: docs_format_analytics_raw_md
path: docs/format/analytics_raw.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: telemetry_raw_format
summary: Analytics Raw Format
tags: [Analytics Raw Format, File layout, QXTR, 1, Record layout (v1)]
concepts: [Analytics Raw Format, File layout, QXTR, 1, Record layout (v1), ts_unix_ms, blk_s, mb_s, rss_kb, cpu_norm, unique_blocks, z_bytes, store_entries, store_bytes, Missing values, u64::MAX, NaN, JSON export (derived), quarxctl telemetry export]
evidence_gap: false
notes:

ARTIFACT
id: docs_net_protocol_md
path: docs/net/protocol.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: product
visibility: public
status: legacy
source_of_truth: no
truth_for:
summary: QuarXNet — Minimal Protocol Surface (E9)
tags: [docs/08-transport-spec.md, Frame transport (legacy), Frame, QXRT, length]
concepts: [docs/08-transport-spec.md, Frame transport (legacy), Frame, QXRT, length, kind, flags, Block identity, hash, QBLK, frame[12..44], size, u32, frame[8..12], GetBlock / PushBlock, `GetBlock` (FrameKind = 9), GetBlock, `PushBlock` (FrameKind = 10), PushBlock, PutBlock, `PutBlock` (FrameKind = 11), Roots / snapshots (minimal), `ListRoots` (FrameKind = 12), ListRoots]
evidence_gap: false
notes:

ARTIFACT
id: docs_cloud_registry_md
path: docs/cloud/registry.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: evolving
source_of_truth: yes
truth_for: cloud_registry_rules
summary: Node Registry & Heartbeat (QuarXCloud)
tags: [Node Registry & Heartbeat (QuarXCloud), Ownership & layering, quarxcloud, Determinism requirements, node_id]
concepts: [Node Registry & Heartbeat (QuarXCloud), Ownership & layering, quarxcloud, Determinism requirements, node_id, Current implementation (in-memory), quarxcloud/src/registry.rs, NodeRegistry, BTreeMap]
evidence_gap: false
notes:

ARTIFACT
id: docs_reporting_fs_import_report_md
path: docs/reporting/fs_import_report.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: HIGH
scope: product
visibility: public
status: stable
source_of_truth: yes
truth_for: fs_import_report
summary: fs-import Report Block (Stable Contract)
tags: [fs-import Report Block (Stable Contract), JSON report (preferred), report_type, "fs_import", host_dir]
concepts: [fs-import Report Block (Stable Contract), JSON report (preferred), report_type, "fs_import", host_dir, repo_path, repo.qblk, result, metrics, config_effective_ini, render_effective_config_ini(&EffectiveConfig), `result` (stable key order), new_root_id, files, dirs, bytes, skipped, zero, symlink, hidden, devices, special, other, z_bytes]
evidence_gap: false
notes:

ARTIFACT
id: docs_analysis_semantic_layers_md
path: docs/analysis/semantic_layers.md
exists: yes
vcs_state: tracked
kind: SPEC
authority: MEDIUM
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Semantic Layers: Quarks, Local Meta, Cloud Semantics
tags: [0. Purpose, 1. Core Principle: Meaning Lives Above Core, 2. Layer Responsibilities, 2.1 QuarXCore (core-rs): Quarks and Correctness, 2.2 QuarXDrive: Local Meta and User Semantics]
concepts: [0. Purpose, 1. Core Principle: Meaning Lives Above Core, 2. Layer Responsibilities, 2.1 QuarXCore (core-rs): Quarks and Correctness, 2.2 QuarXDrive: Local Meta and User Semantics, 3. Semantic Levels (Recommended Model), ObjectId, ManifestId, BlockId, repo.qroot, 5. Protocol: Cheap Exchange of Semantics, SemanticSnapshot, SemanticHints, ManifestExchange, 6. Determinism and Safety, 8. Non-Goals, 9. Rationale (Why This Works)]
evidence_gap: false
notes:

ARTIFACT
id: docs_bench_crash_drills_md
path: docs/bench/crash_drills.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Crash/Recovery Drills
tags: [Crash/Recovery Drills, Truncated qblk (partial frame at EOF), qroot rebuild after loss, file_store_ignores_partial_frame_at_eof, qroot_rebuild_is_deterministic]
concepts: [Crash/Recovery Drills, Truncated qblk (partial frame at EOF), qroot rebuild after loss, file_store_ignores_partial_frame_at_eof, qroot_rebuild_is_deterministic, Command, Result, repo_truncate, qroot_rebuild]
evidence_gap: false
notes:

ARTIFACT
id: docs_bench_io_policy_md
path: docs/bench/io_policy.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: I/O Policy Controls (Drive-Owned)
tags: [I/O Policy Controls (Drive-Owned), Ownership boundary (ARCH-TARGET.md invariant), O_DIRECT, posix_fadvise, core-rs]
concepts: [I/O Policy Controls (Drive-Owned), Ownership boundary (ARCH-TARGET.md invariant), O_DIRECT, posix_fadvise, core-rs, quarxdrive, Determinism constraints, repo.qblk, repo.qroot, Knobs tracked in ROADMAP E8.2, Measurements (baseline), io.direct, large.bin, [import] include_glob = *.bin, io.direct=off, 1.211369s, io.direct=on, 1.219616s]
evidence_gap: false
notes:

ARTIFACT
id: docs_bench_memory_return_md
path: docs/bench/memory_return.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Memory Return Measurement
tags: [Memory Return Measurement]
concepts: [Memory Return Measurement]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: docs_bench_parallel_hashing_md
path: docs/bench/parallel_hashing.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Parallel Hashing & Safe Parallel Regions
tags: [Parallel Hashing & Safe Parallel Regions, Determinism invariants (must hold), BlockId, Safe parallel regions (pure / order-free compute), blake3(payload)]
concepts: [Parallel Hashing & Safe Parallel Regions, Determinism invariants (must hold), BlockId, Safe parallel regions (pure / order-free compute), blake3(payload), core-rs/src/store/blockstore.rs, hash_payload, make_frame_*, core-rs/src/validate.rs, verify_hash_mismatch, repo.qblk, next_id(), core-rs/src/store/file_store.rs, append_frame, next_id, core-rs/src/store/mem_store.rs, Not safe to parallelize (order-sensitive / stateful), index.len(), Implementation constraints (recommended patterns), rayon, rayon::ThreadPool, IndexedParallelIterator, par_iter(), Vec]
evidence_gap: false
notes:

ARTIFACT
id: docs_bench_reimport_delta_md
path: docs/bench/reimport_delta.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: E4.2 — Re-import with partial changes
tags: [Canonical acceptance check (portable), *.so]
concepts: [Canonical acceptance check (portable), *.so]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: docs_bench_reimport_identical_md
path: docs/bench/reimport_identical.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Re-import identical (E4.1)
tags: [Re-import identical (E4.1), block_count_delta == 0, repo_bytes_delta == 0, Dataset, .private/agent/workspace/e4-reimport-identical/data]
concepts: [Re-import identical (E4.1), block_count_delta == 0, repo_bytes_delta == 0, Dataset, .private/agent/workspace/e4-reimport-identical/data, Procedure, Results, out/summary.json, repo.size_bytes=21874, repo.block_count=18, block_count_delta=0, repo_bytes_delta=0, .private/agent/workspace/e4-reimport-identical/out/stats_run1.json, .private/agent/workspace/e4-reimport-identical/out/stats_run2.json, .private/agent/workspace/e4-reimport-identical/out/summary.json]
evidence_gap: false
notes:

ARTIFACT
id: docs_bench_telemetry_samples_md
path: docs/bench/telemetry_samples.md
exists: yes
vcs_state: tracked
kind: GUIDE
authority: LOW
scope: research
visibility: public
status: evolving
source_of_truth: no
truth_for:
summary: Telemetry Samples
tags: [Telemetry Samples]
concepts: [Telemetry Samples]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_readme_md
path: .private/agent/README.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: Private Agent Layer
tags: [Private Agent Layer, entry.md, rules.md, plans/, templates/]
concepts: [Private Agent Layer, entry.md, rules.md, plans/, templates/, workspace/, migrated/, .git/info/exclude, .gitignore]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_entry_md
path: .private/agent/entry.md
exists: yes
vcs_state: ignored
kind: WORKFLOW
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: Local Agent Entry
tags: [Local Agent Entry, rules.md, ARCH-TARGET.md, ARCH.md, docs/**]
concepts: [Local Agent Entry, rules.md, ARCH-TARGET.md, ARCH.md, docs/**, docs/index.md, docs/core-formats.md, docs/config-reference.md, docs/paths-and-repo.md, docs/packaging.md, docs/06-core-api.md, docs/07-net-api.md, docs/08-drive-integration.md, docs/09-cloud-topology.md, plans/, PLAN-*, .codex-plan-machine.json, project.json, .private/agent/**, AGENTS*.md, .git/info/exclude, .gitignore, core-rs, quarxnet]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_rules_md
path: .private/agent/rules.md
exists: yes
vcs_state: ignored
kind: WORKFLOW
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: Internal Rules
tags: [Internal Rules, Core guardrails, core-rs, quarxnet, quarxdrive]
concepts: [Internal Rules, Core guardrails, core-rs, quarxnet, quarxdrive, quarxcloud, tools, quarxproto, QuarXTor-project-*.zip, ARCH-TARGET.md, ARCH.md, .private/agent/arch/, docs/**, docs/index.md, docs/core-formats.md, docs/config-reference.md, docs/paths-and-repo.md, docs/packaging.md, Clean public surface, .codex, CodexPrompt, .private/agent/**, .gitignore]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_migrated_codex_rules_md
path: .private/agent/migrated/.codex-rules.md
exists: yes
vcs_state: ignored
kind: WORKFLOW
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: General rules
tags: [General rules, AGENTS.md, ARCH.md, ROADMAP.md, .codex-project.json]
concepts: [General rules, AGENTS.md, ARCH.md, ROADMAP.md, .codex-project.json, Branding / AI tools, Network transports, Config schema & semantics, core-rs/src/config.rs, docs/config-reference.md, tools/quarxctl/quarxctl.canonical.example.ini, tools/quarxctl/quarxctl.example.ini, docs/config-meta.json, config-meta.json]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_arch_extended_notes_md
path: .private/agent/arch/ARCH-extended-notes.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ARCH extended notes (private)
tags: [ARCH extended notes (private), .private/agent/workspace/logs/]
concepts: [ARCH extended notes (private), .private/agent/workspace/logs/]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_arch_target_summary_md
path: .private/agent/arch/ARCH-TARGET-summary.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ARCH-TARGET summary (private)
tags: [ARCH-TARGET summary (private), ARCH-TARGET.md]
concepts: [ARCH-TARGET summary (private), ARCH-TARGET.md]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_arch_summary_md
path: .private/agent/arch/ARCH-summary.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ARCH summary (private)
tags: [ARCH summary (private), ARCH.md]
concepts: [ARCH summary (private), ARCH.md]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_skills_readme_md
path: .private/agent/skills/README.md
exists: yes
vcs_state: ignored
kind: WORKFLOW
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor Local Skills Workflow
tags: [QuarXTor Local Skills Workflow, _shared/scripts]
concepts: [QuarXTor Local Skills Workflow, _shared/scripts]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_evidence_20251227_190400_recovery_md
path: .private/agent/evidence/20251227-190400-recovery.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Recovery evidence 20251227-190400
tags: [quarxtor, Recovery evidence 20251227-190400, safety-guard, gates, FORMAT.md]
concepts: [Recovery evidence 20251227-190400, safety-guard, gates, FORMAT.md, PRODUCT-CONTRACT.md, I18N-VISION.md, tools/roadmap_next.py, build/test, quarxtor, quarxtor import (temp config), config meta check, contract anchors]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_evidence_20251227T171953Z_e39_fuse_md
path: .private/agent/evidence/20251227T171953Z-e39-fuse.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence: E3.9 FUSE mount CLI (read-only)
tags: [Evidence: E3.9 FUSE mount CLI (read-only), test]
concepts: [Evidence: E3.9 FUSE mount CLI (read-only), test]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_evidence_20251227T164114Z_doc_layout_md
path: .private/agent/evidence/20251227T164114Z-doc-layout.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Doc layout canonicalization evidence
tags: [FORMAT.md, docs/core-formats.md, docs/tools/TOOLS-RECIPES.md, tools/quarxctl/quarxctl.example.ini]
concepts: [FORMAT.md, docs/core-formats.md, docs/tools/TOOLS-RECIPES.md, tools/quarxctl/quarxctl.example.ini]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: private_agent_evidence_20251227T161140Z_product_hygiene_md
path: .private/agent/evidence/20251227T161140Z-product-hygiene.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Product hygiene evidence
tags: [tools/quarxctl/quarxctl.example.ini]
concepts: [tools/quarxctl/quarxctl.example.ini]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_evidence_20251227T154526Z_e2_5_format_registry_md
path: .private/agent/evidence/20251227T154526Z-e2.5-format-registry.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: E2.5 format registry + tests evidence
tags: [docs/format/versions.md, quarxdrive]
concepts: [docs/format/versions.md, quarxdrive]
evidence_gap: true
notes: cannot derive >=5 tags from file content without inventing tokens; tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_evidence_20251227T151548Z_preflight_i18n_config_md
path: .private/agent/evidence/20251227T151548Z-preflight-i18n-config.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence: preflight + i18n + config meta
tags: [Evidence: preflight + i18n + config meta]
concepts: [Evidence: preflight + i18n + config meta]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_evidence_20251227T145523Z_e0_6_packaging_fuse_md
path: .private/agent/evidence/20251227T145523Z-E0.6-packaging-fuse.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence: E0.6 packaging FUSE quickstart
tags: [Evidence: E0.6 packaging FUSE quickstart, docs/packaging.md]
concepts: [Evidence: E0.6 packaging FUSE quickstart, docs/packaging.md]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: private_agent_evidence_20251227T145256Z_e0_6_drive_integration_md
path: .private/agent/evidence/20251227T145256Z-E0.6-drive-integration.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence: E0.6 drive integration FUSE story
tags: [Evidence: E0.6 drive integration FUSE story, docs/08-drive-integration.md, by-path/, by-id/, umount]
concepts: [Evidence: E0.6 drive integration FUSE story, docs/08-drive-integration.md, by-path/, by-id/, umount, fsptool, net use]
evidence_gap: true
notes: concepts_insufficient: 1

ARTIFACT
id: private_agent_evidence_20251227T113500Z_e11_9_ini_validation_md
path: .private/agent/evidence/20251227T113500Z-E11.9-ini-validation.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: devcheck: cargo test --workspace (OK)
tags: []
concepts: []
evidence_gap: true
notes: tags_insufficient: 5; concepts_insufficient: 8

ARTIFACT
id: private_agent_evidence_20251227T085951Z_e11_8_preflight_status_md
path: .private/agent/evidence/20251227T085951Z-E11.8-preflight-status.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence E11.8 preflight status
tags: [Evidence E11.8 preflight status]
concepts: [Evidence E11.8 preflight status]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_evidence_20251227T084305Z_e0_6_transport_docs_md
path: .private/agent/evidence/20251227T084305Z-E0.6-transport-docs.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence E0.6 transport docs
tags: [Evidence E0.6 transport docs]
concepts: [Evidence E0.6 transport docs]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_evidence_20251227T083330Z_e0_6_paths_normalization_md
path: .private/agent/evidence/20251227T083330Z-E0.6-paths-normalization.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: Evidence E0.6 cross-OS paths
tags: [Evidence E0.6 cross-OS paths]
concepts: [Evidence E0.6 cross-OS paths]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_evidence_20251227T075232Z_e0_6_drive_import_doc_md
path: .private/agent/evidence/20251227T075232Z-E0.6-drive-import-doc.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: E0.6 drive import doc evidence.
tags: [core-rs/src/config.rs, docs/10-drive-import.md, import.*, core.l0_chunk, core.analysis.*]
concepts: [core-rs/src/config.rs, docs/10-drive-import.md, import.*, core.l0_chunk, core.analysis.*, Gap audit, docs/config-reference.md, docs/paths-and-repo.md]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_evidence_20251227T074350Z_vision_cohesion_audit_md
path: .private/agent/evidence/20251227T074350Z-vision-cohesion-audit.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: I18N-VISION.md
tags: [TOOLS-RECIPES.md, I18N-VISION.md, 1. Purpose, 2. Key Naming Conventions, status.repo.bytes]
concepts: [TOOLS-RECIPES.md, I18N-VISION.md, 1. Purpose, 2. Key Naming Conventions, status.repo.bytes, 3. Supported Locales Policy, en-US, 4. Fallback Behavior, 5. CLI Output Language Policy, QUARXTOR_LANG, 6. Determinism and Output Stability, docs/**, docs/index.md, docs/core-formats.md, docs/config-reference.md, docs/paths-and-repo.md, docs/packaging.md, quarxtor, docs/10-drive-import.md]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_evidence_20251227T072332Z_audit_md
path: .private/agent/evidence/20251227T072332Z-audit.md
exists: yes
vcs_state: ignored
kind: EVIDENCE
authority: NONE
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: ROADMAP-TARGET.md
tags: [ROADMAP-TARGET.md, TOOLS-RECIPES.md, I18N-VISION.md, test]
concepts: [ROADMAP-TARGET.md, TOOLS-RECIPES.md, I18N-VISION.md, test]
evidence_gap: true
notes: tags_insufficient: 1; concepts_insufficient: 4

ARTIFACT
id: private_agent_plans_plan_machine_readme_md
path: .private/agent/plans/PLAN-MACHINE-README.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: PLAN-MACHINE.json Guide
tags: [PLAN-MACHINE.json Guide, What it is, Structure, version, axes]
concepts: [PLAN-MACHINE.json Guide, What it is, Structure, version, axes, phases, next_pass_candidates, constraints, How to use in a pass, allowed_pass_types, How to update, status, summary, axes.*.progress]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_net_sync_md
path: .private/agent/plans/PLAN-NET-SYNC.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: QuarXNet / Sync Roadmap (plan snapshot)
tags: [QuarXNet / Sync Roadmap (plan snapshot), Transport baseline, HTTP sync contract (current), Net sync plan/apply (MVP), Security & capabilities, CLI net push/pull]
concepts: [QuarXNet / Sync Roadmap (plan snapshot), .codex-plan-machine.json, net_sync, docs/PLAN-MACHINE.md, Transport baseline, Transport wiring, Minimal sync client (Done), HTTP sync, CLI net push/pull, Inventory deltas + capabilities, Advanced sync (not implemented), Security & capabilities, HTTP sync contract (current), POST /sync/list_roots, POST /sync/get_object, POST /sync/put_object, POST /sync/get_l0, POST /sync/put_l0, POST /sync/get_delta, POST /sync/push_blocks, GET /sync/health, GET /sync/ping, GET /sync/capabilities, Net sync plan/apply (MVP), SyncClient, DummySyncClient, NetDrive, DeltaDescriptor, TransportError::Validation, serde/json]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_meta_md
path: .private/agent/plans/PLAN-META.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: PLAN META (autopilot)
tags: [PLAN META (autopilot), docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, docs/PLAN-MACHINE.md, .codex-plan-machine.json]
concepts: [PLAN META (autopilot), docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, docs/PLAN-MACHINE.md, .codex-plan-machine.json, .codex-project.json, next_pass_candidates, PLAN-MACHINE, docs/PLAN-PROMPT-TEMPLATES.json]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_machine_md
path: .private/agent/plans/PLAN-MACHINE.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: PLAN machine (Codex/autopilot)
tags: [PLAN machine (Codex/autopilot), .codex-plan-machine.json, .codex-project.json, docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md]
concepts: [PLAN machine (Codex/autopilot), .codex-plan-machine.json, .codex-project.json, docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, analysis, core, drive, gc, meta, net, ux_ops, 4.4-plan-autopilot, 4.5-core-drive-mvp, 4.6-analysis-quarks-docs, drive.import.*, drive.import.analysis.*, repo.health.*, repo.gc_dry_run.*, repo.repair.*, analysis.patterns.*, blocks_scanned, zero_runs, ff_runs]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_global_v6_md
path: .private/agent/plans/PLAN-GLOBAL-v6.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor — Global PLAN v6
tags: [1. Overview, docs/PLAN-VERSIONS.md, 2. Axes Snapshot, 3. Phases & Milestones, 4. Constraints & Non-Goals]
concepts: [1. Overview, docs/PLAN-VERSIONS.md, 2. Axes Snapshot, 3. Phases & Milestones, 4. Constraints & Non-Goals, 5. Next-Pass Planning, PLAN-MACHINE.json, next_pass_candidates, .codex-project.json]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_global_v5_md
path: .private/agent/plans/PLAN-GLOBAL-v5.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: legacy
source_of_truth: no
truth_for:
summary: QuarXTor — Global PLAN v5
tags: [1. Overview & Axes, .codex-plan-machine.json, docs/PLAN-MACHINE.md, 2. Progress Snapshot, 3. Modules & Repos]
concepts: [1. Overview & Axes, .codex-plan-machine.json, docs/PLAN-MACHINE.md, 2. Progress Snapshot, 3. Modules & Repos, 4. Phases & Milestones, PatternAnalyzer/AnalysisStats/AnalysisMode, analysis.patterns.*, drive.import.analysis.*, 5. Implementation Tracks, quarxctl repo health/gc-dry-run/repair-plan, drive.import.*, docs/10-drive-import.md, SyncClient, net ping, 6. Axes Details, 7. Next Passes (strict), .codex-project.json, docs/CORE-DRIVE-MVP.md, 8. Codex Autopilot Notes, next_pass_candidates]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_axes_md
path: .private/agent/plans/PLAN-AXES.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor — Plan Axes Index
tags: [core-rs/src/analysis.rs, core-rs/src/quarks.rs, tools/quarxctl/src/analysis_cmd.rs, docs/20-analysis-layer.md, docs/ANALYSIS-MVP.md]
concepts: [core-rs/src/analysis.rs, core-rs/src/quarks.rs, tools/quarxctl/src/analysis_cmd.rs, docs/20-analysis-layer.md, docs/ANALYSIS-MVP.md, docs/PLAN-AXES.md, docs/PLAN-GLOBAL-v5.md, docs/PLAN-MACHINE.md, analysis.patterns.*, drive.import.analysis.*, DOCS/PLAN, core-rs, docs/30-gc-and-durability.md, docs/30-gc-health.md, docs/CORE-DRIVE-MVP.md, CORE/ROBUSTNESS, quarxdrive, tools/quarxctl, docs/10-drive-import.md, DRIVE/UX, core-rs/src/gc.rs, tools/quarxctl/src/main.rs, .codex-plan-machine.json, .codex-project.json]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_plan_autopilot_md
path: .private/agent/plans/PLAN-AUTOPILOT.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: PLAN AUTOPILOT
tags: [PLAN AUTOPILOT, docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, docs/PLAN-MACHINE.md, .codex-plan-machine.json]
concepts: [PLAN AUTOPILOT, docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, docs/PLAN-MACHINE.md, .codex-plan-machine.json, .codex-project.json, next_pass_candidates, PLAN-MACHINE, docs/PLAN-PROMPT-TEMPLATES.json]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_plans_codex_rules_md
path: .private/agent/plans/CODEX-RULES.md
exists: yes
vcs_state: ignored
kind: PLAN
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: QuarXTor — Codex Rules
tags: [1. Purpose, 2. Global Behavior, docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, docs/PLAN-META.md]
concepts: [1. Purpose, 2. Global Behavior, docs/PLAN-GLOBAL-v5.md, docs/PLAN-AXES.md, docs/PLAN-META.md, docs/PLAN-MACHINE.json, README, docs, web_ui, config_examples, logs, errors, codex_logs, codex_plan, codex_meta, 3. Planning & PLAN-GLOBAL-v5.md, PLAN-GLOBAL-v5.md, 4. Allowed Edit Scopes by Pass Type, docs/, .codex-project.json, .codex-plan-machine.json, docs/PLAN-AUTOPILOT.md, docs/PLAN-PROMPT-TEMPLATES.json, docs/CODEX-RULES.md]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_skills_quarxtor_patch_discipline_skill_md
path: .private/agent/skills/quarxtor-patch-discipline/SKILL.md
exists: yes
vcs_state: ignored
kind: TOOLING
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ---
tags: [_shared/scripts/qx_gate_patch_apply.sh]
concepts: [_shared/scripts/qx_gate_patch_apply.sh]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_skills_quarxtor_session_runner_skill_md
path: .private/agent/skills/quarxtor-session-runner/SKILL.md
exists: yes
vcs_state: ignored
kind: TOOLING
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ---
tags: []
concepts: []
evidence_gap: true
notes: tags_insufficient: 5; concepts_insufficient: 8

ARTIFACT
id: private_agent_skills_quarxtor_preflight_skill_md
path: .private/agent/skills/quarxtor-preflight/SKILL.md
exists: yes
vcs_state: ignored
kind: TOOLING
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ---
tags: [_shared/scripts/qx_gate_preflight.sh]
concepts: [_shared/scripts/qx_gate_preflight.sh]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_skills_quarxtor_devcheck_skill_md
path: .private/agent/skills/quarxtor-devcheck/SKILL.md
exists: yes
vcs_state: ignored
kind: TOOLING
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ---
tags: [_shared/scripts/qx_gate_devcheck.sh]
concepts: [_shared/scripts/qx_gate_devcheck.sh]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_skills_quarxtor_evidence_writer_skill_md
path: .private/agent/skills/quarxtor-evidence-writer/SKILL.md
exists: yes
vcs_state: ignored
kind: TOOLING
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ---
tags: [_shared/scripts/qx_gate_evidence_append.sh]
concepts: [_shared/scripts/qx_gate_evidence_append.sh]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_skills_quarxtor_roadmap_next_only_skill_md
path: .private/agent/skills/quarxtor-roadmap-next-only/SKILL.md
exists: yes
vcs_state: ignored
kind: TOOLING
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: ---
tags: [_shared/scripts/qx_gate_next.sh]
concepts: [_shared/scripts/qx_gate_next.sh]
evidence_gap: true
notes: tags_insufficient: 4; concepts_insufficient: 7

ARTIFACT
id: private_agent_workspace_readme_md
path: .private/agent/workspace/README.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: evolving
source_of_truth: no
truth_for:
summary: Agent Workspace (formerly `.quarxtor-codex`)
tags: [Agent Workspace (formerly `.quarxtor-codex`), .quarxtor-codex, cache/, graphs/, snapshots/]
concepts: [Agent Workspace (formerly `.quarxtor-codex`), .quarxtor-codex, cache/, graphs/, snapshots/, logs/]
evidence_gap: true
notes: concepts_insufficient: 2

ARTIFACT
id: private_agent_skills_system_skill_creator_skill_md
path: .private/agent/skills/.system/skill-creator/SKILL.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: draft
source_of_truth: no
truth_for:
summary: Skill Creator
tags: [Skill Creator, About Skills, What Skills Provide, Core Principles, Concise is Key]
concepts: [Skill Creator, About Skills, What Skills Provide, Core Principles, Concise is Key, Set Appropriate Degrees of Freedom, Anatomy of a Skill, SKILL.md (required), name, description, Bundled Resources (when present), Scripts (`scripts/`), scripts/, scripts/rotate_pdf.py, References (`references/`), references/, references/finance.md, references/mnda.md, references/policies.md, references/api_docs.md, Assets (`assets/`), assets/, assets/logo.png, assets/slides.pptx]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_skills_system_skill_installer_skill_md
path: .private/agent/skills/.system/skill-installer/SKILL.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: draft
source_of_truth: no
truth_for:
summary: Skill Installer
tags: [Skill Installer, Communication, Scripts, scripts/list-curated-skills.py, Behavior and Options]
concepts: [Skill Installer, Communication, Scripts, scripts/list-curated-skills.py, Behavior and Options, ~/.codex/skills, main, Notes, https://github.com/openai/skills/tree/main/skills/.curated, GITHUB_TOKEN, GH_TOKEN]
evidence_gap: false
notes:

ARTIFACT
id: private_agent_skills_shared_templates_evidence_block_md
path: .private/agent/skills/_shared/templates/evidence_block.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: draft
source_of_truth: no
truth_for:
summary: EVIDENCE ({{LABEL}}):
tags: []
concepts: []
evidence_gap: true
notes: tags_insufficient: 5; concepts_insufficient: 8

ARTIFACT
id: private_agent_skills_shared_templates_stoppoint_block_md
path: .private/agent/skills/_shared/templates/stoppoint_block.md
exists: yes
vcs_state: ignored
kind: GUIDE
authority: LOW
scope: internal
visibility: private
status: draft
source_of_truth: no
truth_for:
summary: STOPPOINT ({{LABEL}}):
tags: []
concepts: []
evidence_gap: true
notes: tags_insufficient: 5; concepts_insufficient: 8

ARTIFACT
id: i18n_en_us_toml
path: i18n/en-US.toml
exists: yes
vcs_state: tracked
kind: I18N
authority: MEDIUM
scope: product
visibility: public
status: draft
source_of_truth: no
truth_for:
summary: "cli.usage.main" = "Usage:\n {bin} --version\n {bin} import <DIR|FILE> [--repo <path>]\n {bin} status [--repo <path>] [--json]\n"
tags: ["cli.usage.main", "cli.error.unknown_command", "cli.error.missing_argument", "cli.error.invalid_path", "cli.error.open_repo"]
concepts: ["cli.usage.main", "cli.error.unknown_command", "cli.error.missing_argument", "cli.error.invalid_path", "cli.error.open_repo", "cli.error.import_failed", "cli.error.status_failed"]
evidence_gap: true
notes: concepts_insufficient: 1

ARTIFACT
id: quarxctl_example_ini
path: quarxctl.example.ini
exists: yes
vcs_state: tracked
kind: CONFIG
authority: MEDIUM
scope: product
visibility: public
status: legacy
source_of_truth: no
truth_for:
summary: [core]
tags: [core, l0_chunk, ram.limit, analysis.enable_znode, analysis.fs_stats_fallback]
concepts: [core, l0_chunk, ram.limit, analysis.enable_znode, analysis.fs_stats_fallback, repo_root, import, min_file_size, skip_hidden, skip_symlink, skip_zero, skip_devices, skip_special, include_glob, import_glob_mode, use_z, z_threshold, paths, net, addr, drive, daemon, cloud]
evidence_gap: false
notes: not canonical; see REGISTRY::CANONICAL_CONFIG

ARTIFACT
id: target_tmp_config_quarxtor_quarxctl_ini
path: target/tmp/config/quarxtor/quarxctl.ini
exists: yes
vcs_state: generated
kind: CONFIG
authority: MEDIUM
scope: product
visibility: private
status: draft
source_of_truth: no
truth_for:
summary: [import]
tags: [import, root]
concepts: [import, root]
evidence_gap: true
notes: tags_insufficient: 3; concepts_insufficient: 6

ARTIFACT
id: tools_quarxctl_quarxctl_canonical_example_ini
path: tools/quarxctl/quarxctl.canonical.example.ini
exists: yes
vcs_state: tracked
kind: CONFIG
authority: MEDIUM
scope: product
visibility: public
status: draft
source_of_truth: no
truth_for:
summary: [paths]
tags: [paths, data_dir, cache_dir, log_dir, runtime_dir]
concepts: [paths, data_dir, cache_dir, log_dir, runtime_dir, ui, lang, core, l0_chunk, ram.limit, analysis.enable_znode, analysis.fs_stats_fallback, repo_root, import, root, min_file_size, skip_hidden, skip_symlink, skip_zero, skip_devices, skip_special, include_glob, exclude_glob, include_dir_glob]
evidence_gap: false
notes: canonical example; see REGISTRY::CANONICAL_CONFIG

ARTIFACT
id: tools_quarxctl_quarxctl_example_ini
path: tools/quarxctl/quarxctl.example.ini
exists: yes
vcs_state: tracked
kind: CONFIG
authority: MEDIUM
scope: product
visibility: public
status: legacy
source_of_truth: no
truth_for:
summary: [paths]
tags: [paths, data_dir, cache_dir, log_dir, runtime_dir]
concepts: [paths, data_dir, cache_dir, log_dir, runtime_dir, ui, lang, core, l0_chunk, ram.limit, analysis.enable_znode, analysis.fs_stats_fallback, repo_root, import, root, min_file_size, skip_hidden, skip_symlink, skip_zero, skip_devices, skip_special, include_glob, exclude_glob, include_dir_glob]
evidence_gap: false
notes: not canonical; see REGISTRY::CANONICAL_CONFIG

```

```text
REGISTRY::EDGES
EDGE
id: edge::DEPENDS_ON::root_format_md::docs_core_formats_md
from_id: root_format_md
to_id: docs_core_formats_md
type: DEPENDS_ON
strict: true
reason: FORMAT.md points to docs/core-formats.md for details.
evidence: path:FORMAT.md lines:4-5
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::docs_config_reference_md::docs_10_drive_import_md
from_id: docs_config_reference_md
to_id: docs_10_drive_import_md
type: DEPENDS_ON
strict: true
reason: Config reference points to drive import doc for defaults.
evidence: path:docs/config-reference.md lines:42-45
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::docs_net_protocol_md::docs_08_transport_spec_md
from_id: docs_net_protocol_md
to_id: docs_08_transport_spec_md
type: DEPENDS_ON
strict: true
reason: Legacy protocol notes defer to canonical transport spec. TODO: keep 6-byte header labeled legacy.
evidence: path:docs/net/protocol.md lines:4-5
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_product_contract_md::docs_paths_and_repo_md
from_id: root_product_contract_md
to_id: docs_paths_and_repo_md
type: DEPENDS_ON
strict: true
reason: Product contract defers config path rules to docs/paths-and-repo.
evidence: path:PRODUCT-CONTRACT.md lines:46-49
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_arch_target_md::root_arch_md
from_id: root_arch_target_md
to_id: root_arch_md
type: OVERRIDES
strict: false
reason: Workflow docs state ARCH-TARGET wins on conflict.
evidence: path:AGENTS.md lines:13-14
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_arch_target_md::root_authority_map_md
from_id: root_arch_target_md
to_id: root_authority_map_md
type: OVERRIDES
strict: true
reason: Authority map states ARCH-TARGET wins on conflict.
evidence: path:AUTHORITY-MAP.md lines:15-16
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_authority_map_md::root_product_contract_md
from_id: root_authority_map_md
to_id: root_product_contract_md
type: OVERRIDES
strict: true
reason: Authority order list states earlier items win over later items.
evidence: path:AUTHORITY-MAP.md lines:57-63
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_product_contract_md::root_format_md
from_id: root_product_contract_md
to_id: root_format_md
type: OVERRIDES
strict: true
reason: Authority order list places PRODUCT-CONTRACT before FORMAT.
evidence: path:AUTHORITY-MAP.md lines:57-63
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_format_md::docs_08_transport_spec_md
from_id: root_format_md
to_id: docs_08_transport_spec_md
type: OVERRIDES
strict: true
reason: Authority order list places FORMAT before protocol spec.
evidence: path:AUTHORITY-MAP.md lines:57-65
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_authority_map_md::docs_08_transport_spec_md
from_id: root_authority_map_md
to_id: docs_08_transport_spec_md
type: OVERRIDES
strict: true
reason: Authority order list places AUTHORITY-MAP before docs/08-transport-spec.
evidence: path:AUTHORITY-MAP.md lines:57-65
evidence_gap: false

EDGE
id: edge::OVERRIDES::root_authority_map_md::root_i18n_vision_md
from_id: root_authority_map_md
to_id: root_i18n_vision_md
type: OVERRIDES
strict: true
reason: Authority order list places AUTHORITY-MAP before I18N-VISION.
evidence: path:AUTHORITY-MAP.md lines:57-69
evidence_gap: false

EDGE
id: edge::CONSTRAINS::root_product_contract_md::root_i18n_vision_md
from_id: root_product_contract_md
to_id: root_i18n_vision_md
type: CONSTRAINS
strict: true
reason: I18N-VISION must align with product contract.
evidence: path:AUTHORITY-MAP.md lines:67-68
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_authority_map_md::tools_quarxctl_quarxctl_canonical_example_ini
from_id: root_authority_map_md
to_id: tools_quarxctl_quarxctl_canonical_example_ini
type: DEPENDS_ON
strict: true
reason: Authority map declares the single canonical example config file.
evidence: path:AUTHORITY-MAP.md lines:125-127
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_authority_map_md::i18n_en_us_toml
from_id: root_authority_map_md
to_id: i18n_en_us_toml
type: DEPENDS_ON
strict: true
reason: Authority map declares i18n/en-US.toml as baseline required.
evidence: path:AUTHORITY-MAP.md lines:148-151
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_agents_md::private_agent_entry_md
from_id: root_agents_md
to_id: private_agent_entry_md
type: DEPENDS_ON
strict: true
reason: Startup sequence requires reading .private/agent/entry.md.
evidence: path:AGENTS.md lines:9-10
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_agents_md::root_roadmap_target_md
from_id: root_agents_md
to_id: root_roadmap_target_md
type: DEPENDS_ON
strict: true
reason: Startup sequence requires reading ROADMAP-TARGET.md.
evidence: path:AGENTS.md lines:9-11
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_agents_md::root_arch_target_md
from_id: root_agents_md
to_id: root_arch_target_md
type: DEPENDS_ON
strict: true
reason: Startup sequence consults ARCH-TARGET then ARCH.
evidence: path:AGENTS.md lines:13-14
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_agents_md::root_arch_md
from_id: root_agents_md
to_id: root_arch_md
type: DEPENDS_ON
strict: true
reason: Startup sequence consults ARCH-TARGET then ARCH.
evidence: path:AGENTS.md lines:13-14
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_roadmap_target_md::root_format_md
from_id: root_roadmap_target_md
to_id: root_format_md
type: DEPENDS_ON
strict: true
reason: Contract anchors list FORMAT.md as MUST READ.
evidence: path:ROADMAP-TARGET.md lines:120-121
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_roadmap_target_md::root_product_contract_md
from_id: root_roadmap_target_md
to_id: root_product_contract_md
type: DEPENDS_ON
strict: true
reason: Contract anchors list PRODUCT-CONTRACT.md as MUST READ.
evidence: path:ROADMAP-TARGET.md lines:120-122
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_roadmap_target_md::root_arch_target_md
from_id: root_roadmap_target_md
to_id: root_arch_target_md
type: DEPENDS_ON
strict: true
reason: Roadmap authority must not contradict ARCH-TARGET.md.
evidence: path:ROADMAP-TARGET.md lines:4-6
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_contributing_md::docs_config_reference_md
from_id: root_contributing_md
to_id: docs_config_reference_md
type: DEPENDS_ON
strict: false
reason: CONTRIBUTING directs readers to docs/config-reference.md.
evidence: path:CONTRIBUTING.md lines:12-12
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_contributing_md::docs_paths_and_repo_md
from_id: root_contributing_md
to_id: docs_paths_and_repo_md
type: DEPENDS_ON
strict: false
reason: CONTRIBUTING directs readers to docs/paths-and-repo.md.
evidence: path:CONTRIBUTING.md lines:12-12
evidence_gap: false

EDGE
id: edge::DEPENDS_ON::root_contributing_md::tools_quarxctl_quarxctl_canonical_example_ini
from_id: root_contributing_md
to_id: tools_quarxctl_quarxctl_canonical_example_ini
type: DEPENDS_ON
strict: true
reason: CONTRIBUTING declares tools/quarxctl/quarxctl.canonical.example.ini as canonical example.
evidence: path:CONTRIBUTING.md lines:16-16
evidence_gap: false

```
```text
REGISTRY::CANONICAL_CONFIG
canonical_runtime_ini: $XDG_CONFIG_HOME/quarxtor/quarxctl.ini
canonical_runtime_ini_evidence: "#   $XDG_CONFIG_HOME/quarxtor/quarxctl.ini"
canonical_example_ini: tools/quarxctl/quarxctl.canonical.example.ini
canonical_example_ini_evidence: "# QuarXTor canonical config (v2)"
other_example_ini_files: [tools/quarxctl/quarxctl.example.ini, quarxctl.example.ini]
rule: "Only canonical_example_ini is authoritative as example; others are historical/non-canonical."
```

```text
REGISTRY::CANONICAL_I18N
locale_files:
- i18n/en-US.toml
namespace_rules_evidence:
- I18N-VISION.md: "- Keys are **namespaced by component and context**, e.g.:"
- I18N-VISION.md: "- Root-level keys (e.g., `usage`, `error`) are forbidden to avoid collisions."
- AUTHORITY-MAP.md: "Keys MUST be dotted and namespaced by subsystem to prevent collisions:"
- AUTHORITY-MAP.md: "Forbidden: generic keys like `usage`, `error`, `missing` without namespace."
locale_selection_evidence:
- I18N-VISION.md: "- Environment overrides: `QUARX_LANG`, then `LANG` as a fallback."
- I18N-VISION.md: "- Configuration key: `[ui].lang` (BCP-47 tag)."
- docs/config-reference.md: "| ui.lang | string | en-US | `QUARX_LANG` | IMPLEMENTED | Locale tag for CLI/UI output (BCP-47). |"
ambiguity: namespace examples differ across docs; reconcile per C07
```

```text
REGISTRY::ROOT_HYGIENE
rule: "- Repo root contains no `*.sh` (and no dev-only helpers)."
rule: "- Dev tooling lives under `tools/dev/**` (tracked) and/or `.private/**` (untracked or private workflow), never in root."
root_wrappers_present:
```
