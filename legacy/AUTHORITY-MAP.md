# AUTHORITY-MAP.md
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.

Central Truth Registry (CTR) for QuarXTor

Status: CONTRACT / HIGH AUTHORITY
Scope: Whole repository (public + private workflow layer)

This file is the single registry of:
- authority order (what wins on conflicts),
- canonical entrypoints & UX invariants,
- canonical formats/paths,
- repository structure map,
- private layer policy,
- change-control rules.

If any other document contradicts this registry, THIS FILE WINS.
If this file contradicts ARCH-TARGET.md, ARCH-TARGET.md wins.

---

## 0) Core Product Invariants (Non-Negotiable)

### 0.1 Single user entrypoint
- The canonical user-facing entrypoint is a cross-OS executable: `quarxtor`.
- No shell wrappers (`*.sh`) are allowed to be used as a product entrypoint.
- Auxiliary binaries can exist (`quarxctl`, internal daemons, dev tools), but the product UX contract is anchored on `quarxtor`.

### 0.2 Import UX invariant
- The primary import UX is:
  - `quarxtor import DIR|FILE`
- Both directory and single-file import must be supported explicitly and remain stable.
- All internal effects (daemons/FS/cloud/status/analysis/dedup) are hidden behind `quarxtor`.

### 0.3 Cross-OS is mandatory
- Product behavior must be defined across supported OSes.
- Platform backends are conditional, but the axis remains mandatory (see FUSE/FS-view).

### 0.4 FUSE / FS-view is a mandatory axis
- FS-view (FUSE/FS backend) is a first-class main axis of the product, mandatory.
- Backend availability is platform/feature dependent, but:
  - `quarxtor` must expose the FS-view capability contractually (commands/status),
  - "unavailable" must be a deterministic product state with a clear reason and next-step hint.

### 0.5 Multilingual (i18n) is a mandatory invariant
- User-facing messages must be localizable.
- Dictionaries are required (baseline: `en-US`).
- Keys must be stable and namespaced to prevent collisions.
- Locale selection is part of config contract (default: `en-US`).

### 0.6 No root scripts / clean root policy
- Repo root contains no `*.sh` (and no dev-only helpers).
- Dev tooling lives under `tools/dev/**` (tracked) and/or `.private/**` (untracked or private workflow), never in root.

---

## 1) Authority Order (Conflict Resolution)

When two sources conflict, the earlier item in this list wins.

### 1.1 Target architecture & immutable product contract
1. `ARCH-TARGET.md` (highest overall target/invariants)
2. `AUTHORITY-MAP.md` (this file, CTR)
3. `PRODUCT-CONTRACT.md` (product surface contract: CLI, UX, axes, required behaviors)
4. `FORMAT.md` (formats: repo layout, on-disk/wire formats; technical contracts)
5. Protocol/Wire authoritative spec:
   - `docs/08-transport-spec.md` (proto framing, header size, wire-level rules)

### 1.2 Supporting "must-align" documents
6. `I18N-VISION.md` (i18n rules, locale policy; must align with product contract)
7. `docs/**` (guides/specs; must not contradict items 1-6)

### 1.3 Agent/operator workflow rules
8. `AGENTS.override.md` (operator guardrails; must not contradict 1-7)
9. `AGENTS.md` (public contributor notes; must not contradict 1-8)

### 1.4 Planning & execution tracking (not authoritative on contracts)
10. `ROADMAP-TARGET.md` (execution plan; NEVER overrides contracts/specs)
11. `plans/**`, `GLOBAL-PLAN.md`, `TOOLS-RECIPES.md`, other plans (planning only)

### 1.5 Private workflow layer (implementation detail of automation)
12. `.private/**` (skills/gates/runner configs):
    - can add workflow rules,
    - must not override product/format/authority contracts (1-7),
    - must not introduce contradictory "authoritative snapshots".

Rule: if `.private/**` conflicts with public contracts (1-7), treat it as a bug in the private workflow layer.

---

## 2) Canonical Entrypoints & Roles

### 2.1 User roles
The same `quarxtor` entrypoint must support different UX tiers:
- basic user: minimal commands, "just works", no daemon/port details
- power user: more verbs, inspect/status details, explicit flags
- admin/operator: diagnostic/reporting, health checks, daemon lifecycle
- developer: tooling exists but is not the product entrypoint

### 2.2 Canonical commands (product surface)
Minimum stable set (exact flags can evolve but must remain user-friendly):
- `quarxtor --version`
- `quarxtor import DIR|FILE [--repo <path>]`
- `quarxtor status [--repo <path>]`
- `quarxtor dashboard` (opens browser to correct endpoint; hides host/port by default)
- FS-view axis (names can vary, capability must exist):
  - `quarxtor drive status`
  - `quarxtor drive mount ...`
  - `quarxtor drive umount ...`

### 2.3 Daemons are internal by default
- Users should not need to run `quarxnetd` / `quarxcloudd` manually for basic scenarios.
- `quarxtor` is responsible for orchestrating or guiding the user with deterministic status output.

---

## 3) Canonical Config Contract

### 3.1 Config format
- Canonical config format: INI.
- TOML config paths must not be introduced for the product.

### 3.2 Canonical config path
- Canonical runtime config location:
  - `~/.config/quarxtor/quarxctl.ini` (platform-equivalent paths allowed; contract must define mapping)

### 3.3 Canonical example config (single source)
There must be exactly ONE canonical example file in repo:
- `tools/quarxctl/quarxctl.canonical.example.ini`

Any other example file name/path is either:
- deleted, or
- explicitly marked as deprecated shim (strongly discouraged).

### 3.4 Required config keys (product-critical)
The config must define, at minimum:
- locale selection (default `en-US`)
- net transport endpoint (unix socket or tcp; canonical semantics)
- repo path default
- runtime/cache/data dirs

Validation rule: `quarxtor status` must report per-component status with explicit reasons:
- ok / missing / invalid / disabled / unavailable
- include path and reason where applicable

---

## 4) i18n Contract (Keys, Dictionaries, Locale)

### 4.1 Locale IDs
- Locale IDs follow BCP-47 (e.g., `en-US`).
- Dictionary files use the locale ID as filename:
  - `i18n/en-US.toml` (baseline required)

### 4.2 Key model (namespacing)
Keys MUST be dotted and namespaced by subsystem to prevent collisions:
- `quarxtor.usage.*`
- `quarxtor.status.*`
- `quarxctl.*`
- `quarxnetd.*`
- `quarxdrive.*`
- `errors.*` (reserved shared category)

Forbidden: generic keys like `usage`, `error`, `missing` without namespace.

### 4.3 Source of truth
- Dictionaries (`i18n/<locale>.toml`) are the source of truth.
- Any code fallback map must be minimal and must not drift; policy for fallback must be documented in `I18N-VISION.md` and must not conflict with this file.

### 4.4 Localization boundary
- User-facing CLI text is localized.
- Machine-readable keys/status codes should remain stable (for scripts, dashboards, tests).

---

## 5) Transport / Protocol Contract

### 5.1 Single authoritative framing
- Proto0 framing uses the 8-byte header as defined in:
  - `docs/08-transport-spec.md`
- Any document describing a different header size must be:
  - updated to match, OR
  - explicitly marked legacy/deprecated and non-default.

### 5.2 Compatibility policy
- If legacy exists, it must be explicitly scoped:
  - "accepted only for X", "deprecated since Y", "not default"
- No ambiguous "either/or" specs.

---

## 6) FUSE / FS-view Axis Contract (Mandatory)

### 6.1 Axis is mandatory, backend is conditional
- The product must expose FS-view capability.
- Backend implementation can vary by OS:
  - Linux: FUSE (typical)
  - macOS: macFUSE (or equivalent)
  - Windows: Dokan/WinFSP (choice must be decided and documented)

### 6.2 Unavailable behavior (required)
If FS-view backend is unavailable on the current platform/build:
- `quarxtor drive status` must output a deterministic "unavailable" state
- include:
  - reason (missing dependency / build feature disabled / unsupported OS)
  - next-step hint (which backend/feature needed)

### 6.3 No "stub replaces axis"
Stub code can exist for compilation, but must not erase the axis contract:
- commands must still exist
- status must still explain availability
- roadmap must track backend completion as product work

---

## 7) Repository Structure Map (What lives where)

### 7.1 Root (repo top-level)
Allowed in root:
- contract anchors: `AUTHORITY-MAP.md`, `PRODUCT-CONTRACT.md`, `FORMAT.md`, `ARCH*.md`, `AGENTS*.md`, `ROADMAP-TARGET.md`
- workspace meta needed for build tooling (Cargo workspace files, etc.)

Forbidden in root:
- `*.sh` scripts (dev or product)
- dev-only helpers (`git-dev-*`, wrappers)
- duplicate/legacy config examples

### 7.2 docs/**
- All public documentation and specs belong here.
- Must not contradict contracts.
- Must contain clear links/index for discoverability.

### 7.3 tools/**
- Developer tooling and auxiliary CLIs.
- `tools/quarxctl/` holds canonical config example.
- `tools/dev/**` holds dev scripts/checks (tracked).

### 7.4 .private/**
- Private workflow layer for automation:
  - skills, gates, runner configs, local caches
- Policy:
  - can enforce workflow hygiene,
  - must not become a product dependency,
  - must not contradict authority order.

### 7.5 Private/public linking rule
- Public docs must not depend on `.private` existence.
- If public docs reference `.private`, it must be in an explicitly "developer workflow only" section and must not be required for product usage.

---

## 8) Change Control Rules (Prevent regressions)

### 8.1 Structural changes require updating CTR first
If you add/remove/move:
- a module,
- a contract/spec anchor,
- a canonical path,
- an axis capability,

you MUST update `AUTHORITY-MAP.md` first (or in the same change).

### 8.2 Conflict rule (stop-and-report)
If any process (automation/CI/human) detects conflicts between authoritative docs:
- STOP and report conflicts
- Do NOT auto-pick silently
- Do NOT recreate legacy artifacts to make things pass

### 8.3 No "planning files become authoritative"
Plans (`plans/**`, `GLOBAL-PLAN.md`, recipes) are never authoritative on contracts.
They can reference contracts and propose changes, but cannot override them.

### 8.4 Protected paths (should be mirrored in AGENTS.override.md)
These areas must not be deleted/restructured automatically:
- `quarxtor/**`
- `quarxdrive/src/fuse/**` (FS-view axis code)
- `i18n/**`
- `docs/**`
- `AUTHORITY-MAP.md`, `PRODUCT-CONTRACT.md`, `FORMAT.md`, `ARCH*.md`

---

## 9) Implementation Reminder (for agents/tools)
Agents must treat this file as the centralized registry:
- Read `ARCH-TARGET.md` then `AUTHORITY-MAP.md` before making decisions.
- If a referenced canonical path/file does not exist, do NOT invent alternatives; report a conflict.

---
End of AUTHORITY-MAP.md
