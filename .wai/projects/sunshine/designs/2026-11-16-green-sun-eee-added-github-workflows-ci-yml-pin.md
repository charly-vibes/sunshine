---
tags: [pipeline-run:tdd-ro5u-2026-11-16-sun-eee-ci-workflow-pinned-tool-matrix, pipeline-step:green]
---

GREEN sun-eee: added .github/workflows/ci.yml (pinned installs: espectacular 0.12.2, specodelic 0.12.1, testaruda 0.5.9, pretender 0.7.0, wai-cli 2026.10.10, dont-cli 0.2.2, openspec 0.19.0, bd 1.3.0 release) and versions.ddl.toml pins for specodelic/espectacular/testaruda; verified YAML parses, actionlint clean, pins equal installed versions; local just ci green.

verified: commands run — `spk lint specs` (ok) + `just ci` (cargo build, cargo test 6 passed, spk lint specs, ah ratchet 6 passed / 54 declared-undeclared, no other findings).
