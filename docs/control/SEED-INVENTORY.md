# SEED-INVENTORY

This document lists the verified seed carried into QuarxTor-DAG.

## Active Workspace Crates
- `core-rs`
- `quarxproto`
- `quarxdrive`
- `quarxnet`
- `quarxcloud`
- `quarxtor`
- `tools/quarxctl`
- `tools/quarx-inspect`

## Carried Assets
- `i18n/en-US.toml`
- `docs/core-formats.md`
- `docs/config-reference.md`
- `quarxctl.example.ini`
- `LICENSE`
- `core-py/`
- `packaging/`

## Validation Performed
- `python3 tools/control_check.py` -> pass
- `cargo test --workspace` -> pass

## Legacy Archive
- Legacy contracts/roadmaps/docs are preserved under `legacy/` as read-only reference.
- Active contracts remain only:
  - `ARCH-TARGET.md`
  - `PRODUCT-CONTRACT.md`
  - `FORMAT.md`
  - `CONFIG-CONTRACT.md`
  - `docs/contracts/*`
