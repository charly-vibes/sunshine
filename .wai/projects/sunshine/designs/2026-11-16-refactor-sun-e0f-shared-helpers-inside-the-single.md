---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-e0f-fleet-boundaries-static-contract-tests, pipeline-step:green]
---

REFACTOR sun-e0f: shared helpers inside the single test file (help surface + lockfile parse); no behavior change; just ci green.

verified: commands run — `spk lint specs` (ok) + `just ci` (cargo build, cargo test 6 passed, spk lint specs, ah ratchet 6 passed / 54 declared-undeclared, no other findings).
