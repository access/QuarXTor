# DEPENDENCY-REGISTRY (Key Dependencies)

This file is the SSOT for key dependencies that affect architecture or compatibility.

## Criteria
A dependency is key if it affects:
- format/protocol/config contracts,
- cross-crate runtime behavior,
- security/observability/performance posture.

## Entries

| ID | Name | Scope | Rationale | Alternatives | License | Exit Plan | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| DEP-0001 | `serde` | workspace | stable serialization and schema bindings | custom serializers (rejected) | MIT/Apache-2.0 | keep format adapters isolated | active |
| DEP-0002 | `bincode` | core/drive/proto | compact binary payload for internal objects | custom binary codec (rejected) | MIT | versioned payload contracts | active |
| DEP-0003 | `blake3` | core/drive | fast content hash baseline | sha2, xxhash | CC0/MIT/Apache-2.0 | hash abstraction in core types | active |
| DEP-0004 | `toml` | quarxtor | locale/config parsing for current seed | ini crate at runtime layer | MIT/Apache-2.0 | move to unified config service in later gate | active |
| DEP-0005 | `globset` | drive import | deterministic include/exclude matching | custom glob engine (rejected) | MIT/Apache-2.0 | maintain deterministic tests | active |
