---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-g0c-render-backends-ship-bar-pdf-determinism, pipeline-step:refactor]
---

REFACTOR sun-g0c: predicate wording aligned with the ship-bar and digest-pinning constraints; no behavior change. verified: commands run — `spk lint specs` ok; `just ci` (cargo build, cargo test 6 passed, spk lint specs, ah ratchet 6 passed / 54 declared-undeclared).
