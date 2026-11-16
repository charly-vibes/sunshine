# sunshine

[![tracked with wai](https://img.shields.io/badge/tracked%20with-wai-blue)](https://github.com/charly-vibes/wai)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![crates.io](https://img.shields.io/badge/crates.io--none-scaffolded-lightgray)

> **Why:** the fleet renders claims back into documents. `sunset` owns
> source geometry (DSC descriptors), `bajan` owns claims (the claim
> graph) — but turning a claim anchor manifest into a human-readable,
> auditable document belongs to neither. **sunshine** is that third
> concern: a stateless compiler from (manifest, descriptor, source) to
> tagged artifacts, with a round-trip audit contract.
> **Status:** scaffold — contract surface wired (render/audit CLI,
> suite envelope), spec corpus lints (6 specs, 54 constraints); backends
> land spec-first (roadmap in `.beads/issues.jsonl`).

```
manifest.json ──┐
                ├──▶ anchored-document IR ──┬──▶ pdf backend  → tagged PDF
DSC descriptor ─┘                           ├──▶ md backend   → .md (Pandoc-ready)
source document┘                            ├──▶ org backend  → .org (Emacs-native)
                                            └──▶ html backend → standalone .html

sunshine render --manifest m.json --dsc doc.dsc.json --format md  --out doc.tagged.md
sunshine audit  doc.tagged.md --against m.json                    # re-parse, diff, exit≠0 on drift
```

The name is the pipeline: what sunset takes in during the day, sunshine
renders out.

## The one property that governs everything

**`audit(render(m)) == m`** under published canonicalization. Every
output must be parseable back to the manifest it was rendered from; a
backend whose audit round-trip fails does not ship. This is what makes
rendered artifacts *evidence* rather than decoration: byte-reproducible,
traceable to their exact inputs via the render stamp, and correctable —
a human-edited `.md` that drifts from its manifest is the correction
signal, flowing back to bajan as a proposal through its review path.

## Boundaries (the deletion test)

- **geometry → sunset.** sunshine derives locations from DSC selectors;
  it contains no PDF text extraction.
- **claims → bajan.** sunshine reads claims only through the manifest;
  it has no claim-creation path.
- **views, never truth.** rendered artifacts are rebuildable from their
  inputs; deleting sunshine loses no information.

If a capability would still make sense with sunshine deleted, it belongs
to sunset or bajan — refused here (`specs/fleet-boundaries.md`,
`fb_deletion_test`).

## Seam compatibility status

The two seam contracts sunshine consumes, plus the tool version its artifacts are stamped with:

| seam | owner | pinned version |
|------|-------|----------------|
| anchor manifest schema | bajan | not yet published — sun-mqs |
| descriptor contract | sunset | sunset-owned — not pinned here |
| sunshine | this repo | 0.1.0 |

## Spec-first

Every behavior lands as a spec in `specs/` (specodelic format) before
implementation:

| spec | owns |
|------|------|
| `fleet.boundaries` | separation contract, deletion test |
| `anchor.manifest` | the bajan→sunshine seam: schema, verbatim spans, semantic locators |
| `anchored.ir` | the intermediate representation: reading order, char-range anchors, fusion |
| `render.contract` | purity, stamping, closed formats, suite envelope, atomic writes |
| `render.backends` | per-format profiles: pdf / md / org / html |
| `audit.contract` | the round-trip guarantee, named drift, read-only |

```bash
just gates         # spk lint specs + ah check with declared-backlog ratchet (runs contract tests)
just spec-compile  # regenerate specodelic mirrors after editing specs
just test          # testaruda exec loop (TDD entry)
just ci            # build + test-full + gates
```

The corpus is spec-first: scenarios land before their contract TOMLs.
`just gates` allows exactly the undeclared scenarios listed in
`tools/gates/ah-undeclared.txt` (each line names its owning beads ticket);
any other ah finding fails, a new undeclared scenario fails until it is
declared with its ticket, and a stale baseline line fails. The list only
shrinks.

Tickets run through the `tdd-ro5u` wai pipeline (TDD red→green→refactor,
Universal Rule of 5 review, repo gates, human ship checkpoint):

```bash
wai pipeline start tdd-ro5u --topic "sun-XXXX <ticket title>"
```

## Part of the charly-vibes fleet

Orchestrated by [dulce-de-leche](https://github.com/charly-vibes/dulce-de-leche)
(`ddl init --gates` wired this repo: lefthook hard gates, beads no-db,
openspec, specodelic, espectacular). Siblings: [sunset](https://github.com/charly-vibes/sunset)
(geometry/DSC), [bajan](https://github.com/charly-vibes/bajan) (claims).

## License

Apache-2.0
