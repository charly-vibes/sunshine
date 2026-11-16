#+TITLE: sunshine design provenance — cotorreo lineage
#+DATE: [2026-11-16]
#+PROJECT: sunshine
#+STATUS: accepted
#+TICKET: sun-07l

* Decision

Record the design lineage of the sunshine spec corpus so future sessions can
trace *why* the corpus says what it says.

* Lineage

- The anchored-document / Claim-Weave design came out of two cotorreo
  conversations, both on 2026-11-16:
  - kimi session ~1a1e21c8~ — "Generic Reversible Claim Framework"
  - zai session ~6f947d90~ — "document structure reversible weaving"
- The original 6-spec bundle was drafted in the kimi session under the name
  *weave*; the tool is named *sunshine* in this repo.
- Naming pipeline: what sunset takes in during the day, sunshine renders out.

* Why it matters

The 6 specs (fleet.boundaries, anchor.manifest, anchored.ir, render.contract,
render.backends, audit.contract) are not invented wholesale — they are the
compiled form of those two conversations. When a constraint looks arbitrary,
the lineage is where to look first.

