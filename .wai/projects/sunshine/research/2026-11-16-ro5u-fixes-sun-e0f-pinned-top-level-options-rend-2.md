---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-e0f-fleet-boundaries-static-contract-tests, pipeline-step:fix-review]
---

RO5U-FIXES sun-e0f: pinned top-level options, render positionals and nested subcommands; deleted prose-driven claim_inputs; added tests/fleet_boundary_deps.txt resolved-dependency snapshot with set equality; shared surfaces() helper; TOML description narrowed to proven guarantees; newlines. verified: commands run — `cargo test --test fleet_boundary_static` 2 passed; `just gates` 6 passed / 54 declared-undeclared.
