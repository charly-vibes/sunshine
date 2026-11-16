---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-e0f-fleet-boundaries-static-contract-tests, pipeline-step:ro5u-review]
---

RO5U boundary slice: 3 High fixed (unpinned top-level options/render positionals/nested subcommands; prose-driven claim_inputs assertion removed per ticket anti-goal; engine denylist replaced by a checked-in resolved-dependency snapshot with set equality), 1 Medium fixed (p_geometry_owner now pins the exact render input surface and shares a surfaces() helper; TOML description narrowed to what is mechanically proven, locator clause deferred to anchored-ir sun-30y), 2 Low fixed (segment-anchored denylist matching; trailing newlines).

verified: commands run — `cargo test --test fleet_boundary_static` 2 passed; `just gates` 6 passed / 54 declared-undeclared.
