---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-e0f-fleet-boundaries-static-contract-tests, pipeline-step:plan]
---

PLAN sun-e0f: red = two contract TOMLs + tests/fleet_boundary_static.rs whose assertions have no implementation to pass against; mechanical inputs are the built binary CLI surface (CARGO_BIN_EXE_sunshine) and the checked-in Cargo.lock, never spec/source text; green = cargo test --test fleet_boundary_static passes; ratchet lines for the two scenarios removed (one-way).
