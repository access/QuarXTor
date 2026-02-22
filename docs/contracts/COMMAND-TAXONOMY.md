# COMMAND-TAXONOMY

Canonical namespace and subcommand taxonomy for `quarxtor`.

Status model:
- `implemented`: runnable now.
- `reserved`: declared but not active in current binary.

## Implemented
- `import <DIR|FILE>`
- `status [--json]`
- `config print|effective|show|explain|set|lint|init|defaults`
- `repo init|verify|doctor|recover|stats`
- `service start|stop|restart|status|logs`
- `mount create|attach|mount|unmount|list|info`
- `dashboard serve|status|snapshot`
- `cloud cluster|peer|discover|sync|policy|stats`
- `compress analyze|plan|apply|rollback|history|tune`
- `export snapshot|bundle|manifest`
- `convert to-disk|to-nondisk|verify`
- `version show|check-compat`
- `diag preflight|trace|report`

## I/O Contract Hooks
- `--output json` uses universal envelope from `docs/contracts/CLI-IO-CONTRACT.md`.
- Command errors use stable categories and deterministic exit-code groups.
- Unknown namespace or subcommand returns usage error with exit code `1`.
