# CLI-IO-CONTRACT

## Scope
Applies to all `quarxtor` namespaces.

## Universal Machine Envelope
All `--output json` responses use:
- `meta`
- `result`
- `errors`
- `timing`
- `operation_id`

`meta` includes:
- `namespace`
- `subcommand`
- `state` (`ok` or `error`)

## Error Objects
`errors[]` entries include:
- `code`
- `category`
- `message`
- `hint` (optional)
- `retryable` (optional)
- `source` (optional)
- `details` (optional)

## Exit Codes
Tiered stable groups:
- usage/config/io/network/compat/security/state/conflict/internal

## Determinism
- Stable key ordering.
- Locale-independent formatting.
- UTC canonical timestamps in finalized contract.

## Output Modes
- Text mode follows stable section/key ordering.
- JSON mode is normative for automation.

## Namespace Status Baseline
Current runtime supports all declared namespaces in `docs/contracts/COMMAND-TAXONOMY.md`.
