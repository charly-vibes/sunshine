# sunshine — dev commands
#
# Same commands run locally and in CI for consistent diagnostics.
# Run `just` for default (build + test), `just ci` for full pipeline,
# `just gates` for the gates (spk lint + spec mirrors + ah ratchet +
# pretender), `just pretender` for just the complexity gate.

set shell := ["bash", "-uc"]

# Default: build and test
default: build test

# === Build Commands ===

# Build debug binary
build:
    cargo build

# Build release binary (optimized)
build-release:
    cargo build --release

# Run with arguments (e.g. `just run render --manifest m.json --dsc d.json --format md --out o.md`)
run *args:
    cargo run -- {{args}}

# Install locally to ~/.cargo/bin
install:
    cargo install --path .

# === Test Commands ===

# TDD loop (DDL test-runner standard): testaruda exec — select the tests
# the working diff affects, run only those, ingest. Tolerance contract:
# 0/10/20 green; NOT CALIBRATED with 0 actual failures is a vacuous pass.
# Local store (.testaruda/, gitignored) initializes on first use.
test:
    #!/usr/bin/env bash
    set -uo pipefail
    if [ ! -s .testaruda/store.db ]; then
      testaruda init || { echo "❌ testaruda init failed" >&2; exit 1; }
    fi
    code=0
    tools/gates/testaruda-exec.sh || code=$?
    case "$code" in
      0|10|20) exit 0 ;;
      *) exit "$code" ;;
    esac

# Preview the affected test set without running anything
test-blast:
    testaruda blast

# Plain full-suite run — CI gate keeps full coverage; selection is for
# fast feedback, never a gate substitute
test-full:
    cargo test

# === Spec & Lint Gates ===

# The ah wrapper keeps undeclared scenarios gated behind the checked-in
# declared backlog (tools/gates/ah-undeclared.txt) so the ratchet only
# turns one way; every other finding fails.
# pretender runs here (not only in CI) so a green `just gates` implies a
# green CI gate — a gate CI enforces but the local oracle cannot see would
# let red land from a fully green local run.
# Gates: specodelic lint + ah ratchet + pretender complexity.
gates:
    spk lint specs
    python3 tools/gates/ah_check_ratchet.py --run-tests
    just pretender

# Diff base: CI derives it from the event (the PR base sha, or the sha a
# push landed on) and widens it to the root commit when that sha is gone.
# Locally there is no event, so the base defaults to `origin/main` — the
# honest local equivalent ONLY for a branch forked from main, where
# `origin/main..HEAD` is exactly this branch's surface. For anything else
# (a range, a squashed or forked-elsewhere branch, a stack) pass the base
# explicitly, e.g. `just pretender HEAD~3` or `just pretender <sha>`.
# Complexity gate over the changed surface — the same command CI runs.
pretender base="origin/main":
    pretender check --diff-only --diff-base "{{base}}" --mode gate

# Regenerate specodelic mirrors after editing specs (edit the spec, not
# the mirrors)
spec-compile:
    spk compile specs

# Full CI pipeline
ci: build test-full gates
