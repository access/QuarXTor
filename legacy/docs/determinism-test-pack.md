# Determinism test pack
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


This document describes the current determinism checks in the repository and where they live.

## Fixtures

### Minimal dataset (fs-import)

Created in tests as a temporary directory containing:

- `a.txt` with bytes `b"a"`
- `b.txt` with bytes `b"bb"`

## Command coverage

### CLI output determinism (byte-identical)

`quarxdrive` JSON output is tested for:

- `quarxdrive --format json --root-path <repo.qblk> repo stats`
- `quarxdrive --format json --root-path <repo.qblk> verify`

Implementation: `quarxdrive/tests/cli_format_json.rs`

### Repo artifact determinism (byte-identical)

Two independent fresh repositories are created and compared byte-for-byte:

- `repo init` → compare `repo.qblk` and `repo.qroot`
- `fs-import <dataset>` → compare `repo.qblk` and `repo.qroot`

Implementation: `quarxdrive/tests/determinism_pack.rs`

## How to run

- `cargo test -p quarxdrive -q`
- `cargo test -p quarxdrive -q --test cli_format_json`
- `cargo test -p quarxdrive -q --test determinism_pack`

