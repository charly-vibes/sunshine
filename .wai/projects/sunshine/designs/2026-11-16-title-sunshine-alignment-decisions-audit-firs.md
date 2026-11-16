#+TITLE: sunshine alignment decisions — audit-first grilling (cotorreo)
#+DATE: [2026-11-16]
#+PROJECT: sunshine
#+STATUS: accepted
#+METHOD: grill-me interview, 4 questions, machine-first resolved at every branch

* Decision

Four intent-level decisions taken to make "what we want from sunshine" explicit.
All four resolve conflicts toward the machine pipeline; none adds escape hatches.

* D1: machine auditability beats presentation, unconditionally

- ~audit(render(m)) == m~ is the governing property. If a prettier layout
  breaks the round-trip, the layout is wrong, not the audit.
- No escape hatch; any conflict is a backend bug. Needs an explicit invariant
  row in ~specs/render-backends.md~ (currently only implicit).

* D2: primary consumer is the machine pipeline, humans secondary

- Success scenario: bajan emits a claim anchor manifest, sunshine renders,
  any artifact in the wild is re-verifiable headlessly in CI, no human needed.
- Consequence: backend order md (P1) → org/html (P2) → pdf (P3) is justified
  by this, matching the beads roadmap (sun-14e, sun-tiv, sun-x7d, sun-4y6).

* D3: unparseable artifacts are a refusal class inside the contract

- No render stamp / wrong shape / truncated file → distinct ~unparseable~
  refusal class, nonzero exit, remediation text — not a CLI usage error.
- Rationale: ~ac_deterministic~ says audit is a pure function of (artifact,
  manifest, profile); totality over file contents keeps that honest and keeps
  the state machine closed. Usage errors are reserved for bad CLI arguments.
- Needs: ~loaded~ defined as stamp + skeleton parse succeeded; transition to
  ~refused~; property ~p_unparseable~ in ~specs/audit-contract.md~.

* D4: ship bar is four criteria, all enforced by ~just ci~

1. *Totality* — renders every manifest the strict parser accepts; unsupported
   input is a named refusal at render time, never silent subset behavior.
2. *Round-trip green* — ~audit(render(m)) == m~ over the generated suite.
3. *Suite envelope* — render.contract envelope artifacts present.
4. *Deterministic bytes* — byte-reproducible output for the same inputs.

- Why totality: without it ~audit(render(m)) == m~ has a hidden precondition
  nothing in the manifest records; the stamp would need backend identity.
- Landmine recorded now: real PDFs embed creation timestamps and document IDs,
  which break determinism. Pdf must pin timestamps/IDs to manifest-digest-derived
  values or it can never ship. This constraint belongs in
  ~specs/render-backends.md~ now, not when sun-4y6 (P3) is picked up.

* Rejected alternatives

- "Human wins sometimes" (D1) — would require an audit escape hatch, gutting
  the one governing property.
- "Unparseable = usage error" (D3) — ad-hoc exit codes the envelope cannot
  represent; proptest cannot cover it.
- "Round-trip is the only ship bar" (D4) — hides a per-backend domain split
  inside the contract; pdf would silently ship nondeterministic output.

* Follow-ups

- Tickets filed for the three spec edits (D1, D3, D4). D2 needs no spec edit —
  the roadmap already encodes it; this record is its justification.
- Next open decision: sun-llk (episode-text home) — upstream of sun-ea6
  (multi-episode export policy) and of sun-mqs (strict parser schema).

