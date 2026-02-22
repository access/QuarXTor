#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Problem:
    path: Path
    message: str

    def format(self, root: Path) -> str:
        try:
            rel = self.path.relative_to(root)
        except ValueError:
            rel = self.path
        return f"{rel}: {self.message}"


HEADING_RE = re.compile(r"^\s{0,3}#{1,6}\s+(?P<title>.+?)\s*$")
MAP_BANNED_SECTION_RE = re.compile(r"\b(Now|Resume|Next)\b", re.IGNORECASE)
MAP_TIMESTAMP_RE = re.compile(
    r"("
    r"\b20\d{2}-[01]\d-[0-3]\d\b"
    r"|"
    r"\b20\d{2}/[01]\d/[0-3]\d\b"
    r"|"
    r"\b[0-2]\d:[0-5]\d\b"
    r")"
)


def _read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        raise
    except Exception as exc:  # pragma: no cover
        raise RuntimeError(f"failed to read {path}: {exc}") from exc


def validate_ledger(path: Path, root: Path) -> list[Problem]:
    problems: list[Problem] = []
    problems.extend(validate_ledger_append_only(path, root=root))
    try:
        text = _read_text(path)
    except FileNotFoundError:
        return [Problem(path, "missing (expected NDJSON file)")]

    for idx, raw_line in enumerate(text.splitlines(), start=1):
        line = raw_line.strip()
        if not line:
            problems.append(Problem(path, f"line {idx}: empty line (NDJSON must be 1 JSON object per line)"))
            continue

        try:
            obj = json.loads(line)
        except json.JSONDecodeError as exc:
            problems.append(Problem(path, f"line {idx}: invalid JSON: {exc.msg}"))
            continue

        if not isinstance(obj, dict):
            problems.append(Problem(path, f"line {idx}: expected JSON object, got {type(obj).__name__}"))
            continue

        if obj.get("type") == "gate_transition" and obj.get("to") == "closed":
            commit = obj.get("commit")
            evidence = obj.get("evidence")

            if not isinstance(commit, str) or not commit.strip():
                problems.append(Problem(path, f"line {idx}: gate_transition to=closed requires non-empty 'commit'"))

            if not isinstance(evidence, list) or len(evidence) == 0:
                problems.append(Problem(path, f"line {idx}: gate_transition to=closed requires non-empty 'evidence[]'"))

    return problems


def validate_ledger_append_only(path: Path, root: Path) -> list[Problem]:
    rel_path = None
    try:
        rel_path = path.relative_to(root).as_posix()
    except Exception:
        rel_path = str(path)

    head = subprocess.run(
        ["git", "rev-parse", "--verify", "HEAD"],
        cwd=str(root),
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if head.returncode != 0:
        return []

    show = subprocess.run(
        ["git", "show", f"HEAD:{rel_path}"],
        cwd=str(root),
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if show.returncode != 0:
        return []

    try:
        current = path.read_bytes()
    except FileNotFoundError:
        return [Problem(path, "missing (expected NDJSON file)")]

    if not current.startswith(show.stdout):
        return [Problem(path, "not append-only relative to HEAD (only appends are allowed; no edits/inserts/deletes)")]

    return []


def validate_map(path: Path) -> list[Problem]:
    problems: list[Problem] = []
    try:
        text = _read_text(path)
    except FileNotFoundError:
        return [Problem(path, "missing")]

    if "```" in text:
        problems.append(Problem(path, "contains fenced code blocks; MAP must not contain command blocks or how-to steps"))

    for idx, line in enumerate(text.splitlines(), start=1):
        match = HEADING_RE.match(line)
        if match:
            title = match.group("title")
            if MAP_BANNED_SECTION_RE.search(title):
                problems.append(
                    Problem(path, f"line {idx}: MAP must not contain operational sections (found heading: {title!r})")
                )

        ts = MAP_TIMESTAMP_RE.search(line)
        if ts:
            problems.append(Problem(path, f"line {idx}: MAP must not contain timestamps (found {ts.group(0)!r})"))

    return problems


def validate_exec(path: Path) -> list[Problem]:
    problems: list[Problem] = []
    try:
        text = _read_text(path)
    except FileNotFoundError:
        return [Problem(path, "missing")]

    headings = []
    for line in text.splitlines():
        match = HEADING_RE.match(line)
        if match:
            headings.append(match.group("title").strip())

    required = {
        "Cursor": re.compile(r"\bCursor\b", re.IGNORECASE),
        "Resume": re.compile(r"\bResume\b", re.IGNORECASE),
        "Expected": re.compile(r"\bExpected\b", re.IGNORECASE),
        "Rollback": re.compile(r"\bRollback\b", re.IGNORECASE),
    }
    for label, pattern in required.items():
        if not any(pattern.search(h) for h in headings):
            problems.append(Problem(path, f"missing required section heading matching {label!r}"))

    return problems


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Validate QuarxTor-DAG control-system invariants.")
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="Repository root (defaults to parent of tools/).",
    )
    args = parser.parse_args(argv)

    root = args.root.resolve()
    control_dir = root / "docs" / "control"

    ledger_path = control_dir / "ROADMAP-LEDGER.ndjson"
    map_path = control_dir / "ROADMAP-MAP.md"
    exec_path = control_dir / "ROADMAP-EXEC.md"

    problems: list[Problem] = []
    problems.extend(validate_ledger(ledger_path, root=root))
    problems.extend(validate_map(map_path))
    problems.extend(validate_exec(exec_path))

    if problems:
        print("control-check: FAIL", file=sys.stderr)
        for problem in problems:
            print(f"- {problem.format(root)}", file=sys.stderr)
        return 1

    print("control-check: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
