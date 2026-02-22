#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

python3 tools/control_check.py

LEDGER="docs/control/ROADMAP-LEDGER.ndjson"

if git diff --cached --name-only -- "$LEDGER" | grep -q .; then
  if git diff --cached --unified=0 -- "$LEDGER" | grep -E '^-{1}[^-]' | grep -v '^---'; then
    echo "precommit_control: FAIL (ledger must be append-only; removed/modified lines detected)" >&2
    exit 1
  fi
fi

CONTRACT_FILES=(
  "AGENTS.md"
  ".codex/config.toml"
  "docs/control/CODEX-INTEGRATION.md"
  "docs/control/ROADMAP-MAP.md"
  "docs/control/GOVERNANCE.md"
  "docs/control/STATUS-MODEL.md"
  "docs/control/GLOSSARY.md"
  "docs/control/DEPENDENCY-REGISTRY.md"
)

contract_changed=false
for path in "${CONTRACT_FILES[@]}"; do
  if git diff --cached --name-only -- "$path" | grep -q .; then
    contract_changed=true
    break
  fi
done

if git diff --cached --name-only -- ".codex/skills" | grep -q .; then
  contract_changed=true
fi

if [[ "$contract_changed" == "true" ]]; then
  if ! git diff --cached --name-only -- "$LEDGER" | grep -q .; then
    echo "precommit_control: FAIL (contract files changed; append a ledger decision/transition event)" >&2
    exit 1
  fi
fi

if git diff --cached --name-only | grep -Eq '(^|/)(Cargo\.toml|Cargo\.lock|package\.json|package-lock\.json|pnpm-lock\.yaml|yarn\.lock|go\.mod|go\.sum|requirements[^/]*\.txt|pyproject\.toml|poetry\.lock|Gemfile|Gemfile\.lock)$'; then
  if ! git diff --cached --name-only -- "docs/control/DEPENDENCY-REGISTRY.md" | grep -q .; then
    echo "precommit_control: FAIL (dependency manifest changed; update docs/control/DEPENDENCY-REGISTRY.md)" >&2
    exit 1
  fi
fi

echo "precommit_control: OK"
