---
id: anchor.manifest
kind: intent
statement: "WHEN a claim store is exported for document rendering THE sunshine manifest SHALL carry each selected claim with its verbatim evidence span, source locator, status, and lineage so that rendering needs no store access"
---

# manifest

The anchor manifest is the seam between bajan and sunshine: a rendering
input that must carry everything the renderer needs about claims, so the
renderer never opens the claim store. The manifest is a published,
versioned schema — produced by bajan's export, consumed strictly by
sunshine. Two decisions are fixed here. First, locators stay semantic:
they carry the descriptor's selector vocabulary, never device
coordinates, so a corrected descriptor re-renders without a re-export.
Second, parsing is strict: an unknown field is a refusal, not a warning,
because silent tolerance at the seam is how provenance dies.

Export is deterministic and content-addressed: the same store state and
options produce a byte-identical manifest, and the manifest carries a
digest over its canonical form that every rendered artifact stamps. A
claim whose evidence span fails verbatim containment in its episode text
exports with the typed absent marker instead — a broken span must be
visible downstream, never quietly kept.

## Constraints

| id | kind | expr | traces_to |
|----|------|------|-----------|
| am_schema_version | invariant | the manifest carries a schema_version; a consumer refuses any version it does not publish support for, with a machine-readable refusal naming the supported set | [[anchor.manifest]] |
| am_claim_fields | invariant | each claim record carries exactly: claim_key, status, text, evidence_span or its typed absent marker, episode lineage of at least one episode id, locator, hedges, and provenance (extractor version, adopting actor, adoption time); confidence scores and embedding blobs never appear | [[anchor.manifest]] |
| am_verbatim_span | invariant | an evidence_span is a verbatim substring of its referenced episode text under the fleet whitespace rule — collapse runs of whitespace to a single space, trim ends, NFC — the one normalization this corpus permits, applied identically at export containment and at join location; a span failing containment exports as the typed absent marker, never silently kept | [[anchor.manifest]] |
| am_semantic_locator | invariant | a locator carries a scheme plus selector payload in the descriptor's selector vocabulary; device coordinates are resolved only at render time, so re-rendering against a corrected descriptor needs no re-export | [[anchor.manifest]] |
| am_status_filter | invariant | an export includes only claims whose status is in the explicitly requested set; the default set is active-only; a proposed claim enters a manifest only when the caller names that status | [[anchor.manifest]] |
| am_deterministic_export | invariant | exporting the same store state with the same options yields a byte-identical manifest under stable claim-key ordering; wall-clock values ride claim provenance records only | [[anchor.manifest]] |
| am_digest | invariant | the manifest carries a content digest over its canonical serialization with the digest field itself excluded from the hashed form; every rendered artifact stamps this digest | [[anchor.manifest]] |
| am_strict_parse | invariant | unknown fields, duplicate claim keys, or references to absent episodes refuse parsing with machine-readable reasons; no field is silently ignored | [[anchor.manifest]] |
| am_relations | advisory | inter-claim relations (supports, contradicts) ride an optional relations section referencing claim keys; relation types come from the closed vocabulary published with the schema | [[anchor.manifest]] |
| am_schema_published | extension_point | the manifest JSON Schema is published for the producer (bajan export) and the consumer (sunshine) to conform to | [[anchor.manifest]] |
| am_claim_key_canon | invariant | a claim_key is a nonempty, non-whitespace-only string; claim-key equality is decided on the NFC form, so two keys differing only by Unicode normalization form denote the same claim and a second occurrence refuses as a duplicate under am_strict_parse; duplicate detection and claim-key ordering both use the normalized key; NFC applies to claim-key equality only — stored claim text and episode text are never normalized by this rule | [[anchor.manifest]] |

## Model

### States

- `exported`
- `accepted`
- `refused`

### Transitions

| id | from | to | guard |
|----|------|----|-------|
| accept | exported | accepted | [[anchor.manifest.am_strict_parse]] |
| refuse_parse | exported | refused | [[anchor.manifest.am_strict_parse]] |
| refuse_version | exported | refused | [[anchor.manifest.am_schema_version]] |

## Properties

| id | kind | derives_from | generator | predicate |
|----|------|--------------|-----------|-----------|
| p_schema_version | unit | [[anchor.manifest.am_schema_version]] | manifests carrying past, current, and future schema versions | current versions parse; past versions parse only while published support holds; future versions refuse naming the supported set |
| p_claim_fields | unit | [[anchor.manifest.am_claim_fields]] | claim records with each field individually dropped or given an extra key | a record missing a required field or carrying an unknown field refuses; the emitted field set equals the published set exactly |
| p_verbatim_span | unit | [[anchor.manifest.am_verbatim_span]] | generated episode texts with spans taken verbatim, mutated, or extracted from a different episode | contained spans export as-is; mutated or foreign spans export as the typed absent marker; no span is silently altered |
| p_semantic_locator | unit | [[anchor.manifest.am_semantic_locator]] | locators over the published selector vocabulary, plus malformed attempts at device coordinates | every exported locator parses as scheme plus selector payload; no coordinate value appears anywhere in the manifest |
| p_status_filter | unit | [[anchor.manifest.am_status_filter]] | stores mixing claim statuses exported under default and explicit status sets | default export carries active claims only; a status appears exactly when the caller named it |
| p_deterministic_export | unit | [[anchor.manifest.am_deterministic_export]] | one store state exported repeatedly, including after key-order shuffles in the source | byte-identical manifests across runs; ordering follows claim-key order regardless of source order |
| p_digest | unit | [[anchor.manifest.am_digest]] | manifests and their canonical serializations | the digest equals the published hash of the canonical form with the digest field excluded; mutating any other byte changes the digest; mutating the digest field alone does not |
| p_strict_parse | unit | [[anchor.manifest.am_strict_parse]] | manifests with unknown fields, duplicate keys, and absent-episode references | each case refuses with a machine-readable reason naming the offending field or key |
| p_relations | unit | [[anchor.manifest.am_relations]] | manifests with relation sections using published and unpublished relation types | published types parse; an unpublished type refuses; a missing relations section is valid |
| p_schema_published | unit | [[anchor.manifest.am_schema_published]] | record instances sampled from the published schema | the schema document validates every accepted record and rejects every refused one |
| p_claim_key_canon | unit | [[anchor.manifest.am_claim_key_canon]] | claim-key pairs differing only by NFC vs NFD form, key sets mixing NFC and NFD forms of distinct keys whose raw and normalized orderings differ, plus empty and whitespace-only keys | normalization-variant duplicate keys refuse with am_strict_parse's duplicate reason; empty or whitespace-only keys refuse; claim-key ordering is decided on the normalized form |
