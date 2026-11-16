#!/usr/bin/env bash
# repo-gates oracle: the repo's own hard gate must pass.
# Oracle args (artifact paths) are ignored on purpose.
set -uo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$root" || exit 1

exec just gates