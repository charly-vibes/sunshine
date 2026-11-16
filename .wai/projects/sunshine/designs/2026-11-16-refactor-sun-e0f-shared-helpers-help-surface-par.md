---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-e0f-fleet-boundaries-static-contract-tests, pipeline-step:refactor]
---

REFACTOR sun-e0f: shared helpers (help surface parse + lockfile read) kept inside the single integration test; no behavior change. verified: commands run — `spk lint specs` ok; `just ci` (cargo build, cargo test 6 passed, spk lint specs, ah ratchet 6 passed / 54 declared-undeclared).
