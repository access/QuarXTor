# GUI-JOURNEYS

## Personas and Platforms
- weak/user/power/admin on desktop/server OSes
- Android consumer-first mode

## Core Flows
- create (`disk` or `non-disk`)
- mount/unmount (disk)
- browse/open (non-disk)
- snapshot/export/share
- import snapshot

## Role-Based UX Depth
- weak user: one-click presets (space-first, speed-first, balanced)
- user: basic policy toggles + cloud account linking
- power: advanced recipe/transport/pinning controls
- admin: cluster policy, ACL, observability, remote diagnostics

## Disk Modes
- `disk`: mountable volume
- `non-disk`: ZIP-compatible smart archive with QuarxTor extensions

## Snapshot Import
Default behavior:
- auto-restore linked policies/profile refs
- trust/auth verification
- partial mount with degraded wizard when dependencies are unavailable

## Overlay Model
Default runtime behavior:
- local CoW writes
- async cloud sync
- policy-controlled optimization pipeline

## Cross-OS Mount Expectations
- Linux/macOS: native mount integration with explicit unmount controls.
- Windows: drive-letter integration and safe-eject flow.
- Android consumer mode: non-disk workflow first, mount optional where supported.
