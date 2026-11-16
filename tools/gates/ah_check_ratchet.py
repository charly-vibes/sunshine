#!/usr/bin/env python3
"""Ratchet gate over `ah check`.

The spec corpus is authored spec-first: scenarios are declared before their
contract TOMLs and implementations land. `ah check` treats every undeclared
scenario (`no-toml`) as a gate-failing structural finding, so a raw gate can
never be green while the roadmap is open.

This wrapper keeps the gate hard without hiding drift:

- `no-toml` findings are allowed ONLY for scenario ids listed in
  tools/gates/ah-undeclared.txt (each line names its owning tickets).
- Any other finding (no-tests-declared, orphan-toml, id-mismatch,
  malformed-contract, execution failures, ...) fails the gate.
- A new `no-toml` finding fails: declaring a scenario creates backlog, and
  backlog must be recorded in the same change.
- A stale baseline line fails (the contract/scenario is now declared):
  the ratchet only turns one way.

Exit 0 when the observed state equals the declared backlog and everything
else is clean; exit 1 otherwise.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

BASELINE_REL = "tools/gates/ah-undeclared.txt"


def repo_root() -> Path:
    proc = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True
    )
    return Path(proc.stdout.strip()) if proc.returncode == 0 else Path.cwd()


def load_baseline(path: Path) -> dict[str, str]:
    """Map `spec/scenario-id` -> owning-ticket comment (may be empty)."""
    entries: dict[str, str] = {}
    if not path.exists():
        return entries
    for raw in path.read_text(encoding="utf-8").splitlines():
        body, _, comment = raw.partition("#")
        key = body.strip()
        if key:
            entries[key] = comment.strip()
    return entries


def key_of(finding: dict) -> str:
    return f"{finding['spec']}/{finding['scenario']['id']}"


def load_report(root: Path, argv: list[str]) -> dict | None:
    """Run `ah check --json` and parse its report, or emit diagnostics."""
    proc = subprocess.run(
        ["ah", "check", "--json", *argv], cwd=root, capture_output=True, text=True
    )
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError:
        sys.stdout.write(proc.stdout)
        sys.stderr.write(proc.stderr)
        print(
            f"ah-check-ratchet: `ah check --json` produced no parsable report "
            f"(exit {proc.returncode})",
            file=sys.stderr,
        )
        return None


def classify(findings: list[dict]) -> tuple[dict[str, dict], list[dict]]:
    """Split findings into baseline-keyed `no-toml` entries and everything else."""
    undeclared: dict[str, dict] = {}
    other: list[dict] = []
    for finding in findings:
        if finding.get("kind") == "no-toml":
            undeclared[key_of(finding)] = finding
        else:
            other.append(finding)
    return undeclared, other


def report_ok_clean(summary: dict, baseline: dict[str, str]) -> int:
    print(
        "ah-check-ratchet: OK — "
        f"{summary.get('passed', 0)} passed, 0 findings, "
        f"{len(baseline)} declared-undeclared scenario(s)"
    )
    return 0


def report_ok_backlog(summary: dict, undeclared: dict[str, dict]) -> int:
    print(
        "ah-check-ratchet: OK — "
        f"{summary.get('passed', 0)} passed, {len(undeclared)} declared-undeclared scenario(s), "
        "no other findings"
    )
    return 0


def print_other_findings(other: list[dict]) -> None:
    print(f"\n{len(other)} finding(s) outside the declared backlog:")
    for finding in other:
        category = finding.get("category", "?")
        print(f"  [{category}] {finding.get('kind')}: {key_of(finding)}")
        playbook = finding.get("playbook_command")
        if playbook:
            print(f"      fix path: {playbook}")


def print_new_keys(new: list[str]) -> None:
    print(
        f"\n{len(new)} undeclared scenario(s) not in the baseline. "
        "A new spec-first scenario must be declared with its owning ticket:"
    )
    for key in new:
        print(f"  + {key}")


def print_stale_keys(stale: list[str], baseline: dict[str, str]) -> None:
    print(
        f"\n{len(stale)} stale baseline line(s): the scenario is declared or gone. "
        "Remove these lines (the ratchet only turns one way):"
    )
    for key in stale:
        owner = baseline[key]
        suffix = f"  # {owner}" if owner else ""
        print(f"  - {key}{suffix}")


def report_failure(
    baseline: dict[str, str], undeclared: dict[str, dict], other: list[dict]
) -> int:
    new = sorted(set(undeclared) - set(baseline))
    stale = sorted(set(baseline) - set(undeclared))
    print(f"ah-check-ratchet: FAILED ({BASELINE_REL})")
    if other:
        print_other_findings(other)
    if new:
        print_new_keys(new)
    if stale:
        print_stale_keys(stale, baseline)
    return 1


def main() -> int:
    root = repo_root()
    baseline = load_baseline(root / BASELINE_REL)
    report = load_report(root, sys.argv[1:])
    if report is None:
        return 1

    data = report.get("data") or {}
    summary = data.get("summary") or {}
    if report.get("ok"):
        return report_ok_clean(summary, baseline)

    undeclared, other = classify(data.get("findings") or [])
    if other or set(undeclared) != set(baseline):
        return report_failure(baseline, undeclared, other)
    return report_ok_backlog(summary, undeclared)


if __name__ == "__main__":
    sys.exit(main())