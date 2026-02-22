# STATUS-MODEL

This file defines multi-valued statuses for QuarxTor-DAG control artifacts.

## Gate Statuses
- `planned`: defined in MAP, not started.
- `in_progress`: active work toward DoD.
- `blocked`: progress halted by explicit blocker.
- `closed`: DoD satisfied with recorded evidence.
- `reopened`: contract changed; previous closure no longer applies.
- `regressed`: previously valid evidence no longer holds.
- `deprecated`: gate no longer relevant.
- `superseded`: gate replaced by another gate.

## Work Item Statuses (optional)
- `idea`, `queued`, `active`, `paused`, `done`, `dropped`

## FRAG Statuses
- `active`, `frozen`, `superseded`, `deprecated`
