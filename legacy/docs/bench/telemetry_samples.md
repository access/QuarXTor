# Telemetry Samples
Architecture invariants: QuarXCloud -> QuarXDrive -> QuarXNet -> QuarXCore -> Block Store; Tools, FS/FUSE, CLI, Import pipeline, Registry & validation, Documentation & contracts are mandatory and equal.


Command:
`XDG_CONFIG_HOME=/tmp/quarx-reimport-config cargo run -p quarxctl -- /tmp/quarx-reimport-identical/repo-identical.qblk fs-import /tmp/quarx-reimport-identical/data`

Output (excerpt):
```
telemetry.import blk_s=9314.63 mb_s=11.37 rss_kb=6360 cpu_norm=0.104 unique_blocks=98 z_bytes=20480 store_entries=98 store_bytes=130565
```
