---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-mkj-corpus-decisions-medium-tier, pipeline-step:fix-review]
---

RO5U corpus slice: 2 High fixed (am_claim_key_canon left NFD handling undetermined vs am_strict_parse duplicate refusal -> equality on NFC form with duplicate refusal, storage never normalized; fleet prose claimed seam pins the README denies -> tracked, not pinned), 4 Medium fixed (one-artifact-per-invocation wording; ordering clause made falsifiable by generator; README seam framing; normalization scope vs am_verbatim_span), 1 Low fixed (not-yet-published tense), 1 Low recorded as generated-mirror convention (no action).

verified: commands run — `spk lint specs` ok; `just gates` 6 passed / 54 declared-undeclared, no other findings.
