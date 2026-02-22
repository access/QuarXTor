# COMMAND-TAXONOMY

Canonical namespace and subcommand taxonomy for `quarxtor`.

Status model:
- `implemented`: runnable now.
- `planned`: deterministic stub, returns `not_implemented`.

## Implemented
- `import <DIR|FILE>`
- `status [--json]`

## Planned Namespaces and Subcommands
- `config`
- subcommands: `show`, `effective`, `explain`, `set`, `lint`
- `repo`
- subcommands: `init`, `verify`, `doctor`, `recover`, `stats`
- `service`
- subcommands: `start`, `stop`, `restart`, `status`, `logs`
- `mount`
- subcommands: `create`, `attach`, `mount`, `unmount`, `list`, `info`
- `dashboard`
- subcommands: `serve`, `status`, `snapshot`
- `cloud`
- subcommands: `cluster`, `peer`, `discover`, `sync`, `policy`, `stats`
- `compress`
- subcommands: `analyze`, `plan`, `apply`, `rollback`, `history`, `tune`
- `export`
- subcommands: `snapshot`, `bundle`, `manifest`
- `convert`
- subcommands: `to-disk`, `to-nondisk`, `verify`
- `version`
- subcommands: `show`, `check-compat`
- `diag`
- subcommands: `preflight`, `trace`, `report`

## I/O Contract Hooks
- `--output json` uses universal envelope from `docs/contracts/CLI-IO-CONTRACT.md`.
- Planned commands return deterministic state with exit code `2`.
- Unknown namespace or subcommand returns usage error with exit code `1`.
