# PRODUCT-CONTRACT.md

## Canonical Entrypoint
- `quarxtor` is the canonical user/admin entrypoint.

## Baseline Implemented Surface
- `quarxtor import`
- `quarxtor status`

## Planned Surface (Gate Controlled)
- `config`, `repo`, `service`, `mount`, `dashboard`, `cloud`, `compress`, `export`, `convert`, `version`, `diag`
- See `docs/contracts/COMMAND-TAXONOMY.md` for namespace/subcommand taxonomy.

## Determinism
- Machine output will use a universal JSON envelope.
- Stable key ordering and locale-independent formatting are required.

## Snapshot/Sharing Model
- `disk`: mountable volume workflow.
- `non-disk`: ZIP-compatible smart archive with QuarxTor extensions.
- Snapshot import may run in degraded mode when external links are unavailable.

## Cloud/P2P Baseline Contract
- Discovery and control can run over `ipc/http/grpc/quic`.
- Mesh broadcast is bounded-only (TTL, fanout, dedupe, ACL).
- `uTP` is optional future transport and not required in baseline.
