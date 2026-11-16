---
id: render.contract
kind: intent
statement: "WHEN a render command is invoked with a manifest, a descriptor, and a source document THE sunshine renderer SHALL emit exactly one tagged document in the requested format, stamped with its inputs' identities, as a pure function of those inputs"
---

# render

Render is the outward command: manifest, descriptor, and source in; one
tagged document out. The contract is purity — no network, no wall clock,
no ambient state — because a rendered artifact is evidence, and evidence
must be reproducible byte-for-byte. Every output embeds a render stamp
(manifest digest, descriptor identity, tool and backend versions) so any
artifact can be traced to its exact inputs later.

Failure is loud and early: the format set is closed, missing inputs
refuse before any work, and every command answers the fleet's suite
envelope with a non-empty remediation on every error. Output is written
atomically — a failed render leaves no half-written file — and the
source document, descriptor, and manifest are never mutated: the pdf
backend writes a new file, the text backends write new files, nothing
edits in place.

## Constraints

| id | kind | expr | traces_to |
|----|------|------|-----------|
| rc_purity | invariant | render is a pure function of (manifest, descriptor, source); no network, no wall clock, no ambient state; identical inputs yield byte-identical outputs | [[render.contract]] |
| rc_stateless | invariant | the renderer persists no state between invocations; a cache, if present, is transparent: deleting it never changes output bytes | [[render.contract]] |
| rc_closed_formats | invariant | the format set is closed and enumerated (pdf, md, org, html in v1); an unknown format refuses with remediation naming the set; adding a format amends this spec and the backends spec in one change | [[render.contract]] |
| rc_stamp | invariant | each output embeds a render stamp: manifest digest, descriptor id and version, tool name and version, backend id and profile version; an unstamped output is a defect | [[render.contract]] |
| rc_envelope | invariant | each command answers the suite envelope {ok, envelope_version, cli_version, envelope_kind, data}; exit status mirrors ok; each error carries a non-empty remediation | [[render.contract]] |
| rc_no_extraction | invariant | the renderer performs no text or claim extraction; it consumes descriptor and manifest only — geometry authority is sunset, claim authority is bajan | [[render.contract]] |
| rc_no_claim_semantics | invariant | the renderer never filters, ranks, merges, or re-words claims; presentation choices never drop a claim from the output | [[render.contract]] |
| rc_atomic_write | invariant | each output artifact is written whole or not at all; a failed render leaves no partial file | [[render.contract]] |
| rc_source_immutable | invariant | render never mutates the source document, the descriptor, or the manifest; each backend writes a new output file | [[render.contract]] |
| rc_input_present | invariant | an invocation with a missing or unreadable input refuses before any output work, with a machine-readable reason naming the missing input | [[render.contract]] |

## Model

### States

- `invoked`
- `rendered`
- `refused`

### Transitions

| id | from | to | guard |
|----|------|----|-------|
| render_ok | invoked | rendered | [[render.contract.rc_stamp]] |
| refuse_format | invoked | refused | [[render.contract.rc_closed_formats]] |
| refuse_input | invoked | refused | [[render.contract.rc_input_present]] |
| refuse_write | rendered | refused | [[render.contract.rc_atomic_write]] |

## Properties

| id | kind | derives_from | generator | predicate |
|----|------|--------------|-----------|-----------|
| p_purity | unit | [[render.contract.rc_purity]] | render invoked twice per generated input triple, including under a faked wall clock | byte-identical outputs across invocations; no network sockets opened |
| p_stateless | unit | [[render.contract.rc_stateless]] | render sequences with cache directories deleted between runs | outputs unchanged by cache deletion; no state survives an invocation |
| p_closed_formats | unit | [[render.contract.rc_closed_formats]] | format names drawn from the set plus near-miss typos | set members dispatch to a backend; typos refuse with remediation listing the set |
| p_stamp | unit | [[render.contract.rc_stamp]] | renders over generated inputs, outputs parsed for the stamp | every output carries all stamp fields with values matching the actual inputs |
| p_envelope | unit | [[render.contract.rc_envelope]] | success and failure invocations across the command surface | every answer parses as the suite envelope; exit mirrors ok; every failure carries non-empty remediation |
| p_no_extraction | unit | [[render.contract.rc_no_extraction]] | renders with source documents whose text differs from descriptor and episode text | output claim text comes from manifest and descriptor fields alone — never from re-parsing the source |
| p_no_claim_semantics | unit | [[render.contract.rc_no_claim_semantics]] | manifests whose claim set varies in size and content | the output realizes the input claim set exactly — nothing filtered, ranked, merged, or reworded |
| p_atomic_write | unit | [[render.contract.rc_atomic_write]] | renders interrupted mid-write via fault injection | no partial output file survives an interruption; the output path holds either the complete artifact or nothing |
| p_source_immutable | unit | [[render.contract.rc_source_immutable]] | renders with input hashes taken before and after | input files keep their hashes after render; outputs land at new paths |
| p_input_present | unit | [[render.contract.rc_input_present]] | invocations missing each input in turn | each missing input refuses before output work, naming the input |
