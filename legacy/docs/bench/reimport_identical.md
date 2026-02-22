# Re-import identical (E4.1)
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Goal: importing the same dataset into the same repo twice yields **no growth**:
`block_count_delta == 0` and `repo_bytes_delta == 0`.

## Dataset

Root: `.private/agent/workspace/e4-reimport-identical/data`

Listing (portable, stable):

```
$ python3 tools/portable_ls.py .private/agent/workspace/e4-reimport-identical/data
file1.so	8192
file2.exe	8192
sub/file3.iso	4096
```

## Procedure

```
$ python3 tools/e4_reimport_identical.py \
  --repo-dir .private/agent/workspace/e4-reimport-identical/repo \
  --data-dir .private/agent/workspace/e4-reimport-identical/data \
  --runs 2
```

## Results

From `out/summary.json`:

- run1: `repo.size_bytes=21874`, `repo.block_count=18`
- run2: `repo.size_bytes=21874`, `repo.block_count=18`
- delta: `block_count_delta=0`, `repo_bytes_delta=0`

Artifacts:
- `.private/agent/workspace/e4-reimport-identical/out/stats_run1.json`
- `.private/agent/workspace/e4-reimport-identical/out/stats_run2.json`
- `.private/agent/workspace/e4-reimport-identical/out/summary.json`

