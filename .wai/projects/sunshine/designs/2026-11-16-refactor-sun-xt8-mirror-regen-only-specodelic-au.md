---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-xt8-audit-contract-unparseable-refusal, pipeline-step:refactor]
---

REFACTOR sun-xt8: mirror regen only (specodelic/audit-contract.*, openspec/specs/audit-contract/spec.md); no behavior change; just ci green.

verified: commands run — `spk lint specs` (ok) + `just ci` (cargo build, cargo test 6 passed, spk lint specs, ah ratchet 6 passed / 54 declared-undeclared, no other findings).
