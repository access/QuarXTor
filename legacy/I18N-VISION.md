# I18N-VISION.md
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


## 1. Purpose
This document defines the internationalization (I18N) and locale policy for
QuarXTor user-facing output. Section ordering is stable and should not be
reordered.

## 2. Key Naming Conventions
- Keys are ASCII, lowercase, and dot-delimited.
- Keys are **namespaced by component and context**, e.g.:
  - `cli.usage.main`
  - `cli.error.unknown_command`
  - `status.check.config.label`
  - `status.state.ok`
- Root-level keys (e.g., `usage`, `error`) are forbidden to avoid collisions.
- Keys are stable identifiers and never include localized text.
- Keys are versioned only when semantics change; otherwise, keep the same key.

## 3. Supported Locales Policy
- Default locale: `en-US`.
- Locale tags follow BCP-47 (e.g., `en-US`, `et-EE`, `ru-RU`).
- Locale files live under `i18n/<locale>.toml`.
- Supported locales are an explicit allowlist in the product configuration.
- New locales require translation coverage for all public CLI strings and UI
  labels before enablement.

## 4. Fallback Behavior
- If a locale is requested but unavailable, fall back to `en-US`.
- If a specific key is missing in the requested locale, fall back to `en-US`
  for that key only.
- If locale files are missing entirely, fall back to a minimal hardcoded map.

## 5. CLI Output Language Policy
- Default CLI language is `en-US`.
- Configuration key: `[ui].lang` (BCP-47 tag).
- Environment overrides: `QUARX_LANG`, then `LANG` as a fallback.

## 6. Determinism and Output Stability
- Stable key order is invariant across locales and platforms.
- Human language is separate from machine keys; translation never changes keys.
- `--json` outputs use stable keys and canonical ordering.
- Locale selection must not affect sorting of items, only the display labels.
