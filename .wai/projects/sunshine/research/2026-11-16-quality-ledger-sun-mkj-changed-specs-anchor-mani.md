---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-mkj-corpus-decisions-medium-tier, pipeline-step:quality-ledger]
---

QUALITY LEDGER sun-mkj. Changed: specs/anchor-manifest.md (am_claim_key_canon, p_claim_key_canon), specs/audit-contract.md (CORR-005 canonicalization), specs/fleet-boundaries.md (value proposition, one-artifact success scenario, seam tracking), README.md (Seam compatibility status) + mirrors + ratchet line; spun sun-xgc/sun-qhc/sun-jml for EDGE-003/004/005. Verified: `spk lint specs` ok; `just gates` 6 passed / 54; `cargo test` 6 passed. Review: RO5U 2 High/4 Medium/1 Low (1 Low recorded as mirror convention), all fixed. Risks: manifest schema still unpublished (sun-mqs); descriptor contract remains sunset-owned. Next: ship-close.
