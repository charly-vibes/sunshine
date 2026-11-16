#!/usr/bin/env bash
set -euo pipefail

artifact_path="${1:-}"

# Evidence gate: the step artifact must record what was run and the result.
# This is not a substitute for running the gates (repo-gates oracle does that);
# it prevents advancing with an artifact that claims work without evidence.

if [[ -z "$artifact_path" ]]; then
  echo "tests-pass oracle requires an artifact path" >&2
  exit 1
fi

if [[ ! -f "$artifact_path" ]]; then
  echo "artifact not found: $artifact_path" >&2
  exit 1
fi

if grep -Eqi 'command=|commands run|verified:|tests pass|validation run|non-code|no code changes|full verification command' "$artifact_path"; then
  exit 0
fi

echo "Artifact lacks explicit verification evidence. Record commands run or note that the step was non-code work: $artifact_path" >&2
exit 1