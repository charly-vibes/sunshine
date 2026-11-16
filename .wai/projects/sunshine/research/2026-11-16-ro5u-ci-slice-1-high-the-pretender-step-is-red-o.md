---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-eee-ci-workflow-pinned-tool-matrix, pipeline-step:ro5u-review]
---

RO5U ci slice: 1 High (the pretender step is red on this branch: tools/gates/ah_check_ratchet.py main() 19 cyclomatic / 26 cognitive / 81 lines / 47.9 abc; tests/fleet_boundary_static.rs parse_help 8/14/3-nesting/30.89 abc, p_claim_owner 85 lines, p_geometry_owner 6/8/87/3 — the pinned 0.7.0 has identical engines and no adopt baseline), 3 Medium (fallback diff base narrows to HEAD^ on force-push; no mirror-freshness step; just gates/just ci cannot see pretender), 1 Low (bd tarball installed without checksum). Fixes assigned: MetricRefactor (both files) and CIAlign (workflow + justfile).
