# TOOLS-RECIPES.md
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


CLI recipes for repo.qblk workflows using current tool names.

## Conventions
- `QXCTL` points to the `quarxctl` binary. If built locally: `./target/release/quarxctl`.
- `STORE` is a `.qblk` store file (a `.qroot` sidecar is created next to it).
- `SRC` is the directory to import.
- `OUT` is the output path for `fs-get`.

Examples use bash; adapt paths and variables for other shells.

## Quickstart: import, tree, stats
```bash
QXCTL="${QXCTL:-quarxctl}"
STORE="${PWD}/repo.qblk"
SRC="${PWD}/data"

mkdir -p "${SRC}"
printf "hello\n" > "${SRC}/hello.txt"

"${QXCTL}" "${STORE}" fs-import "${SRC}"
"${QXCTL}" "${STORE}" fs-tree
"${QXCTL}" "${STORE}" fs-stats
```

## Recipe: fs-import (directory)
Import a host directory into the repo root:
```bash
"${QXCTL}" "${STORE}" fs-import "${SRC}"
```

Verbose import with skip reasons:
```bash
"${QXCTL}" "${STORE}" fs-import --debug-import "${SRC}"
```

Expected output keys (stable order):
```
fs-import: host dir "... -> QuarX root"
  new_root_id = <id>
  files       = <count>
  dirs        = <count>
  skipped_zero     = <count>
  skipped_symlink  = <count>
  skipped_hidden   = <count>
  skipped_devices  = <count>
  skipped_special  = <count>
  skipped_other    = <count>
```

## Recipe: fs-tree (root listing)
List the root-level entries and their IDs:
```bash
"${QXCTL}" "${STORE}" fs-tree
```

Output columns are `UID`, `TYPE`, `NAME`.

## Recipe: fs-stats (repo verification summary)
Compute recursive counts from root:
```bash
"${QXCTL}" "${STORE}" fs-stats
```

Stable keys:
```
dirs, files, bytes
```

## Recipe: fs-get (retrieve a file)
Fetch a file by root path and write it to disk:
```bash
OUT="${PWD}/hello.out"
"${QXCTL}" "${STORE}" fs-get /hello.txt "${OUT}"
```

Compare `OUT` to the original file with your OS diff tool.

## Deterministic output expectations
- `fs-import` prints fixed key names in a fixed order; treat keys as the stable contract.
- `fs-stats` prints `dirs`, `files`, `bytes` in a fixed order.
- `fs-tree` prints a stable column set; for deterministic comparisons, sort rows by `NAME`.
- `fs-get` writes bytes deterministically for a given repo state and path.
