---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-mkj-corpus-decisions-medium-tier, pipeline-step:orient]
---

PLAN sun-e0f: red = two contract TOMLs + tests/fleet_boundary_static.rs before implementation; mechanical inputs are the built binary CLI surface (env!("CARGO_BIN_EXE_sunshine")) and the checked-in Cargo.lock, never spec/source text greps. green = cargo test --test fleet_boundary_static; verification cargo test + just gates; baseline lines for the two scenarios removed (ratchet turns one way). Anti-goal: no spec-text grep, no doc-only assertion.
