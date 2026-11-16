# openspec project context

## project: sunshine

**why:** the fleet renders claims back into documents. sunset owns source
geometry (DSC descriptors), bajan owns claims (the claim graph). A third
concern — turning claim anchor manifests into human-readable, auditable
documents — belongs to neither, so it lives here: `sunshine`, a stateless
compiler from (manifest, descriptor, source) to tagged artifacts with a
round-trip audit contract.

**deletion test (the boundary in one line):** if a capability would still
make sense after deleting weave, it belongs to sunset or bajan — refuse it
here (specs/fleet-boundaries.md, `fb_deletion_test`).

## naming

the binary, crate, and repo are all `sunshine` — paired with `sunset`
(what goes in: the day's sources; what comes out: readable, claim-tagged
documents). no tool-namespace elsewhere in the fleet.

## conventions

- spec-first: every behavior lands as a spec in `specs/` (specodelic
  format, `spk lint specs` gates commits) before implementation
- suite envelope: every command answers `{ok, envelope_version,
  cli_version, envelope_kind, data}`; exit mirrors `ok`; errors carry
  remediation
- purity: render and audit are pure functions of their inputs; identical
  inputs → byte-identical outputs
- audit: `audit(render(m)) == m` under published canonicalization; a
  backend whose audit round-trip fails does not ship

## toolchain (dulce-de-leche standards)

- `just test` — testaruda exec loop (TDD entry)
- `just gates` — spk lint + ah check
- lefthook pre-commit/pre-push hard gates (wired by `ddl init --gates`)
- issues: bd (beads, no-db — `.beads/issues.jsonl` is the source of truth)
- decisions: wai; openspec changes under `openspec/changes/`
