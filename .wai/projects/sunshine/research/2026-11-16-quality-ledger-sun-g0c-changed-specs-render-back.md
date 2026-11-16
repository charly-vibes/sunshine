---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-g0c-render-backends-ship-bar-pdf-determinism, pipeline-step:quality-ledger]
---

QUALITY LEDGER sun-g0c. Changed: specs/render-backends.md (rb_ship_bar with pinned domain + named refusal + rc_envelope criterion, rb_deterministic_pdf, p_ship_bar, p_deterministic_pdf) + mirrors + ratchet lines. Verified: `spk lint specs` ok; `just gates` 6 passed / 54; `cargo test` 6 passed. Review: RO5U 2 High/1 Medium/1 Low, all fixed. Risks: backend implementations land under sun-jh6/sun-14e/sun-4y6; pdf unshippability is now explicit. Next: ship-close.
