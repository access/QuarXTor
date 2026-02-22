# Memory Return Measurement
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Baseline (before import):
```
Mem: 65824652 total, 29422828 used, 16069220 free, 43823488 buff/cache, 36401824 available
```

Import run (telemetry excerpt):
```
telemetry.import blk_s=7.99 mb_s=0.01 rss_kb=6452 cpu_norm=0.000 unique_blocks=130 z_bytes=20480 store_entries=130 store_bytes=174041
```

After run:
```
Mem: 65824652 total, 29458224 used, 16018280 free, 43848068 buff/cache, 36366428 available
```

Notes:
- Process RSS drops to zero after exit; any residual memory change is OS page cache.
