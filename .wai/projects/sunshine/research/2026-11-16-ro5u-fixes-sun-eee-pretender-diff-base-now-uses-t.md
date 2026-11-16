---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-eee-ci-workflow-pinned-tool-matrix, pipeline-step:fix-review]
---

RO5U-FIXES sun-eee: pretender diff base now uses the single root commit, else merge-base with origin/main, else fails loudly; new blocking mirror-freshness step (spk compile + deploy_specs + porcelain check); pretender recipe added to justfile and run from gates (so ci includes it); bd tarball sha256-verified against the published release checksum; ah_check_ratchet.py split into metric-sized helpers with byte-level behavior parity over 8 fixtures; boundary-test helpers extracted with every assertion preserved. verified: commands run — `just ci` green (cargo test 6 passed; ratchet 6 passed / 54; pretender all green), pretender check exit 0, actionlint clean.
