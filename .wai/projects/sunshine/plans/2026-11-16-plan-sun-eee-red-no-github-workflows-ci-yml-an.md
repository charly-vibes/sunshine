---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-eee-ci-workflow-pinned-tool-matrix, pipeline-step:plan]
---

PLAN sun-eee: red = no .github/workflows/ci.yml and no pins for spk/ah/testaruda; green = workflow running build, full cargo test (which carries the fleet-boundary tests), spk lint specs, ah declared-backlog ratchet (never raw ah check), pretender changed-surface gate, all tools installed at versions.ddl.toml pins; verification = YAML parse + actionlint + local just ci parity; no workflow execution available locally (stated limit).
