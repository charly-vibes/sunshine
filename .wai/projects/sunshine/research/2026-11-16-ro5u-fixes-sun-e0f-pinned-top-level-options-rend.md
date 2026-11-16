---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-e0f-fleet-boundaries-static-contract-tests, pipeline-step:ro5u-review]
---

RO5U-FIXES sun-e0f: pinned top-level options, render positionals and nested subcommands; deleted the prose-driven claim_inputs assertion; added tests/fleet_boundary_deps.txt resolved-dependency snapshot with set equality plus readable engine denylist; p_geometry_owner pins the exact render input surface via a shared surfaces() helper; TOML description narrowed; trailing newlines. verified: `cargo test --test fleet_boundary_static` 2 passed; `just gates` 6 passed / 54.
