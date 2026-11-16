---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-xt8-audit-contract-unparseable-refusal, pipeline-step:quality-ledger]
---

QUALITY LEDGER sun-xt8. Changed: specs/audit-contract.md (ac_unparseable, refuse_unparseable loaded->refused, p_unparseable, loaded definition in prose, canonicalization scoped, precedence sentence) + mirrors. Verified: `spk lint specs` ok; `just gates` 6 passed / 54 declared-undeclared; `cargo test` 6 passed; `just ci` green. Review: RO5U 2 High/1 Medium/2 Low, all fixed. Risks: parser behavior itself lands with sun-14e. Next: ship-close (commit + close).
