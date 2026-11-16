---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-eee-ci-workflow-pinned-tool-matrix, pipeline-step:quality-ledger]
---

QUALITY LEDGER sun-eee. Changed: .github/workflows/ci.yml (pinned installs + smoke, build, full tests, spk lint, mirror freshness, ah ratchet, pretender with root/merge-base resolution, bd sha256), versions.ddl.toml (specodelic 0.12.1, espectacular 0.12.2, testaruda 0.5.9), justfile (pretender recipe; gates/ci include it), tools/gates/ah_check_ratchet.py and tests/fleet_boundary_static.rs (metric-sized refactor, behavior unchanged). Verified: just ci green; cargo test 6 passed; ratchet 6 passed / 54; pretender exit 0; actionlint clean. Review: RO5U 1 High/3 Medium/1 Low, all fixed. Risks: the workflow itself cannot be executed locally (stated limit); GitHub topics were set out-of-band via gh. Next: commit 359ac78, close ticket, approve pipeline.
