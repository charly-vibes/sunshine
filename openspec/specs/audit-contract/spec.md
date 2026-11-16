---
id: audit.contract
kind: intent
statement: "WHEN an audit command is invoked on a rendered artifact against its manifest THE sunshine auditor SHALL deterministically reconstruct the manifest from the artifact and report equivalence or a named drift"
---

# audit

Audit is the reversibility guarantee, stated as one property: auditing a
rendered artifact against its manifest yields the manifest back —
`audit(render(m)) == m` under published canonicalization. When it fails,
the report is the product: named drift items (claim key, field, expected,
actual) in distinct machine-readable classes, never truncated, with exit
status mirroring the verdict.

The auditor is read-only and pure: it never repairs an artifact, never
rewrites anything, never sees the network. It refuses a foreign manifest
outright — the stamped digest must match the manifest under audit before
any parsing begins — and it parses only the profile version stamped in
the artifact; an unknown or newer profile refuses with remediation
rather than guessing, because a guess would turn the verifier into the
weakest link. A human-edited text artifact that drifts
from its manifest is not an error to suppress; it is the correction
signal, and it flows back to the claim store as a proposal through
bajan's review path.

An artifact that cannot be parsed at all — no render stamp, truncated,
wrong shape, or garbage — is a contract-level refusal class, not a CLI
usage error. Stamp presence and skeleton parse are decided first — an
artifact that cannot be read cannot be attributed to a profile, so the
refusal is unparseable; the profile check runs only after a successful
skeleton parse. `loaded` is the pre-decision state: the artifact's bytes
are in hand and no refusal or verdict has been chosen. An artifact whose
render stamp is present and whose skeleton parse succeeds leaves `loaded`
for comparison; one lacking a stamp or failing that parse refuses from
`loaded` as unparseable.

## Purpose

Audit is the reversibility guarantee, stated as one property: auditing a
rendered artifact against its manifest yields the manifest back —
`audit(render(m)) == m` under published canonicalization. When it fails,
the report is the product: named drift items (claim key, field, expected,
actual) in distinct machine-readable classes, never truncated, with exit
status mirroring the verdict.
The auditor is read-only and pure: it never repairs an artifact, never
rewrites anything, never sees the network. It refuses a foreign manifest
outright — the stamped digest must match the manifest under audit before
any parsing begins — and it parses only the profile version stamped in
the artifact; an unknown or newer profile refuses with remediation
rather than guessing, because a guess would turn the verifier into the
weakest link. A human-edited text artifact that drifts
from its manifest is not an error to suppress; it is the correction
signal, and it flows back to the claim store as a proposal through
bajan's review path.
An artifact that cannot be parsed at all — no render stamp, truncated,
wrong shape, or garbage — is a contract-level refusal class, not a CLI
usage error. Stamp presence and skeleton parse are decided first — an
artifact that cannot be read cannot be attributed to a profile, so the
refusal is unparseable; the profile check runs only after a successful
skeleton parse. `loaded` is the pre-decision state: the artifact's bytes
are in hand and no refusal or verdict has been chosen. An artifact whose
render stamp is present and whose skeleton parse succeeds leaves `loaded`
for comparison; one lacking a stamp or failing that parse refuses from
`loaded` as unparseable.

## Constraints

| id | kind | expr | traces_to |
|----|------|------|-----------|
| ac_roundtrip | invariant | for each shipped backend, auditing a rendered artifact against its manifest reports equivalence after published canonicalization; a failing audit is a defect in the backend, never in the auditor | [[audit.contract]] |
| ac_canonical | invariant | comparison runs on canonical forms published per profile — whitespace collapsing, NFC, claim-key ordering; canonicalization strips presentation-only derivations before comparison — annex formatting, tooltips (pdf annotation popup text, excluding the annotation's claim key), and comment formatting (marker spelling, reflow, attribute order) — while comment payloads carrying claim bindings are content and are never stripped; canonicalization never repairs content differences | [[audit.contract]] |
| ac_named_drift | invariant | each mismatch is reported as a named drift item — claim key, field, expected, actual — in a distinct machine-readable class (absent claim, added claim, mutated span); the report never truncates the drift list | [[audit.contract]] |
| ac_read_only | invariant | the auditor never rewrites the artifact or the manifest; repair is a human act upstream | [[audit.contract]] |
| ac_deterministic | invariant | audit is a pure function of (artifact, manifest, profile); repeated audits yield byte-identical reports | [[audit.contract]] |
| ac_exit_mirror | invariant | audit exits nonzero on any drift, and its envelope reports ok:false naming each drift | [[audit.contract]] |
| ac_profile_match | invariant | the auditor parses only the profile version stamped in the artifact; an unknown or newer profile refuses with remediation rather than guessing | [[audit.contract]] |
| ac_manifest_identity | invariant | before parsing, the auditor compares the artifact's stamped manifest digest against the against-manifest's own digest; a mismatch is a distinct machine-readable refusal class (foreign manifest), never a drift report | [[audit.contract]] |
| ac_human_edits | advisory | a human-edited text artifact that drifts from its manifest is the correction signal; edits flow to the claim store as proposals through the store's own review path; sunshine never writes to the store | [[audit.contract]] |
| ac_unparseable | invariant | an artifact lacking a render stamp or failing skeleton parse — truncated, wrong shape, or garbage — refuses with a distinct machine-readable class (unparseable) naming the remediation; never a panic, never a usage error, never silence | [[audit.contract]] |

## Model

### States

- `loaded`
- `verified`
- `drifted`
- `refused`

### Transitions

| id | from | to | guard |
|----|------|----|-------|
| verify_clean | loaded | verified | [[audit.contract.ac_roundtrip]] |
| verify_drift | loaded | drifted | [[audit.contract.ac_named_drift]] |
| refuse_profile | loaded | refused | [[audit.contract.ac_profile_match]] |
| refuse_foreign | loaded | refused | [[audit.contract.ac_manifest_identity]] |
| reverify | drifted | drifted | [[audit.contract.ac_deterministic]] |
| refuse_unparseable | loaded | refused | [[audit.contract.ac_unparseable]] |

## Properties

| id | kind | derives_from | generator | predicate |
|----|------|--------------|-----------|-----------|
| p_roundtrip | unit | [[audit.contract.ac_roundtrip]] | generated manifests rendered through each shipped backend | audit(render(m)) reports equivalence for each backend; no backend reaches shipped state while its round-trip fails |
| p_canonical | unit | [[audit.contract.ac_canonical]] | artifacts differing only in whitespace form, NFC position, or presentation-only derivations — annex formatting, tooltips (pdf annotation popup text, excluding the annotation's claim key), and comment formatting (marker spelling, reflow, attribute order) — under the published canonicalizer | canonical differences audit clean; artifacts differing only in presentation-only derivations audit clean and never drift — comment payloads carrying claim bindings are content, so changing them drifts; content differences still drift |
| p_named_drift | unit | [[audit.contract.ac_named_drift]] | artifacts mutated per drift class — claim removed, claim added, span edited | the report names each drift with key, field, expected, actual and the correct class; the list is complete, never truncated |
| p_read_only | unit | [[audit.contract.ac_read_only]] | audits with artifact and manifest hashes taken before and after | both files keep their byte-hashes through any audit outcome |
| p_deterministic | unit | [[audit.contract.ac_deterministic]] | repeated audits over identical inputs | byte-identical reports across runs |
| p_exit_mirror | unit | [[audit.contract.ac_exit_mirror]] | audits ending in verified, drifted, and refused states | exit status and envelope ok mirror the state; drifted reports name the full drift list |
| p_profile_match | unit | [[audit.contract.ac_profile_match]] | artifacts stamped with unknown and newer profile versions | each refuses with remediation naming the supported profile set |
| p_manifest_identity | unit | [[audit.contract.ac_manifest_identity]] | audits pairing artifacts with foreign, superseded, and correct manifests | mismatched digests refuse with the foreign-manifest class before any claim parsing; the correct pairing proceeds to parsing |
| p_human_edits | unit | [[audit.contract.ac_human_edits]] | human-edited text artifacts with plausible corrections | the drift report stands as the correction signal; no store write occurs from the audit path |
| p_unparseable | unit | [[audit.contract.ac_unparseable]] | artifacts lacking a render stamp or failing the skeleton parse — truncated, wrong shape, or garbage — plus well-formed rendered artifacts | each refuses with the distinct machine-readable class unparseable naming the remediation — never a panic, never a usage error, never silence; an artifact whose render stamp is present and whose skeleton parse succeeds proceeds past `loaded` to verified or drifted, never refused |

## Requirements

### Requirement: Property coverage mirror

Every property row SHALL be verified by exactly one dedicated scenario;
`ah sync` SHALL derive one contract per VERIFIES link.

#### Scenario: p_roundtrip

- **WHEN** generated manifests rendered through each shipped backend
- **THEN** audit(render(m)) reports equivalence for each backend; no backend reaches shipped state while its round-trip fails
- **VERIFIES** [[audit.contract.p_roundtrip]]

#### Scenario: p_canonical

- **WHEN** artifacts differing only in whitespace form, NFC position, or presentation-only derivations — annex formatting, tooltips (pdf annotation popup text, excluding the annotation's claim key), and comment formatting (marker spelling, reflow, attribute order) — under the published canonicalizer
- **THEN** canonical differences audit clean; artifacts differing only in presentation-only derivations audit clean and never drift — comment payloads carrying claim bindings are content, so changing them drifts; content differences still drift
- **VERIFIES** [[audit.contract.p_canonical]]

#### Scenario: p_named_drift

- **WHEN** artifacts mutated per drift class — claim removed, claim added, span edited
- **THEN** the report names each drift with key, field, expected, actual and the correct class; the list is complete, never truncated
- **VERIFIES** [[audit.contract.p_named_drift]]

#### Scenario: p_read_only

- **WHEN** audits with artifact and manifest hashes taken before and after
- **THEN** both files keep their byte-hashes through any audit outcome
- **VERIFIES** [[audit.contract.p_read_only]]

#### Scenario: p_deterministic

- **WHEN** repeated audits over identical inputs
- **THEN** byte-identical reports across runs
- **VERIFIES** [[audit.contract.p_deterministic]]

#### Scenario: p_exit_mirror

- **WHEN** audits ending in verified, drifted, and refused states
- **THEN** exit status and envelope ok mirror the state; drifted reports name the full drift list
- **VERIFIES** [[audit.contract.p_exit_mirror]]

#### Scenario: p_profile_match

- **WHEN** artifacts stamped with unknown and newer profile versions
- **THEN** each refuses with remediation naming the supported profile set
- **VERIFIES** [[audit.contract.p_profile_match]]

#### Scenario: p_manifest_identity

- **WHEN** audits pairing artifacts with foreign, superseded, and correct manifests
- **THEN** mismatched digests refuse with the foreign-manifest class before any claim parsing; the correct pairing proceeds to parsing
- **VERIFIES** [[audit.contract.p_manifest_identity]]

#### Scenario: p_human_edits

- **WHEN** human-edited text artifacts with plausible corrections
- **THEN** the drift report stands as the correction signal; no store write occurs from the audit path
- **VERIFIES** [[audit.contract.p_human_edits]]

#### Scenario: p_unparseable

- **WHEN** artifacts lacking a render stamp or failing the skeleton parse — truncated, wrong shape, or garbage — plus well-formed rendered artifacts
- **THEN** each refuses with the distinct machine-readable class unparseable naming the remediation — never a panic, never a usage error, never silence; an artifact whose render stamp is present and whose skeleton parse succeeds proceeds past `loaded` to verified or drifted, never refused
- **VERIFIES** [[audit.contract.p_unparseable]]

