---
id: anchored.ir
kind: intent
statement: "WHEN a manifest and a descriptor are joined for rendering THE sunshine pipeline SHALL construct one anchored-document intermediate representation with text in descriptor reading order and claim anchors as character ranges, as the sole input to every backend"
---

# ir

The anchored document is the heart of sunshine: the document's text in
reading order, reconstructed from the descriptor's logical-structure
layer, with claim anchors attached as character ranges. Everything
format-specific lives in thin backends; the IR is the only thing they
consume. This is the same front-end/back-end split as the document
tooling ecosystem, applied to a provenance problem.

Three disciplines are fixed here. Reading order comes from the
descriptor — the pipeline never invents ordering; if the descriptor
lacks order for a span, that gap is carried as a typed marker and
declared in the output, because silently guessing a reading order would
corrupt the geometry authority. Text is verbatim: no normalization, no
corrections, no re-wording — the renderer has no license to improve
prose. Fusion is a list: a span realizing several claims carries all of
their keys, never a merged super-claim, so the audit can split it back.

## Purpose

The anchored document is the heart of sunshine: the document's text in
reading order, reconstructed from the descriptor's logical-structure
layer, with claim anchors attached as character ranges. Everything
format-specific lives in thin backends; the IR is the only thing they
consume. This is the same front-end/back-end split as the document
tooling ecosystem, applied to a provenance problem.
Three disciplines are fixed here. Reading order comes from the
descriptor — the pipeline never invents ordering; if the descriptor
lacks order for a span, that gap is carried as a typed marker and
declared in the output, because silently guessing a reading order would
corrupt the geometry authority. Text is verbatim: no normalization, no
corrections, no re-wording — the renderer has no license to improve
prose. Fusion is a list: a span realizing several claims carries all of
their keys, never a merged super-claim, so the audit can split it back.

## Constraints

| id | kind | expr | traces_to |
|----|------|------|-----------|
| ai_single_ir | invariant | each backend consumes the anchored-document IR alone; no backend reads the manifest, the descriptor, or the source document directly | [[anchored.ir]] |
| ai_reading_order | invariant | IR text order comes from the descriptor's logical structure; a span lacking descriptor order carries a typed unordered marker and renders at a fallback position declared in the output; the pipeline never invents ordering | [[anchored.ir]] |
| ai_charrange_anchor | invariant | an anchor is a half-open character range over the IR text carrying a claim-key list; a span realizing several claims carries each key in the list, never a merged super-claim | [[anchored.ir]] |
| ai_verbatim_text | invariant | IR text is verbatim descriptor and episode text; the pipeline never corrects or re-words it; span location applies the fleet whitespace rule defined in [[anchor.manifest.am_verbatim_span]] — the corpus's one permitted normalization — identically on both sides, and collapsed matches map back to verbatim source positions deterministically for emission | [[anchored.ir]] |
| ai_deterministic_join | invariant | the same (manifest, descriptor) pair yields an identical IR; overlapping anchor ranges resolve by the published total order (start, length, claim key), never by input order | [[anchored.ir]] |
| ai_unanchored | invariant | a claim whose evidence span cannot be located in the IR text is carried as unanchored with a machine-readable reason and rendered in a claims annex; it is never silently dropped and never fuzzy-matched | [[anchored.ir]] |
| ai_descriptor_required | invariant | a join without a parseable descriptor refuses with a machine-readable reason naming the missing input; rendering from a manifest alone is impossible | [[anchored.ir]] |
| ai_no_semantics | invariant | IR construction performs no claim filtering, ranking, or re-wording; selection happened upstream at export; the IR carries what the manifest carries | [[anchored.ir]] |
| ai_ir_schema | extension_point | the IR data model is published so backends and audit parsers conform | [[anchored.ir]] |

## Model

### States

- `received`
- `joined`
- `refused`

### Transitions

| id | from | to | guard |
|----|------|----|-------|
| join | received | joined | [[anchored.ir.ai_deterministic_join]] |
| refuse_no_descriptor | received | refused | [[anchored.ir.ai_descriptor_required]] |
| rejoin | joined | joined | [[anchored.ir.ai_deterministic_join]] |

## Properties

| id | kind | derives_from | generator | predicate |
|----|------|--------------|-----------|-----------|
| p_single_ir | unit | [[anchored.ir.ai_single_ir]] | renders across the format set with input tracing | every backend call sees exactly the IR; no backend call touches manifest, descriptor, or source bytes |
| p_reading_order | unit | [[anchored.ir.ai_reading_order]] | descriptors with explicit order, partial order, and unordered spans | ordered spans follow descriptor order; unordered spans carry the marker and land at the declared fallback; ordering never follows input order |
| p_charrange_anchor | unit | [[anchored.ir.ai_charrange_anchor]] | manifests fusing one to N claims onto overlapping spans | each anchor is a valid half-open range in bounds; a fused span carries one key per claim with none dropped or merged |
| p_verbatim_text | unit | [[anchored.ir.ai_verbatim_text]] | descriptors whose episode text includes unusual whitespace and unicode forms | span location applies the fleet whitespace rule identically on both sides; emission stays verbatim; collapsed matches map back to source positions deterministically |
| p_deterministic_join | unit | [[anchored.ir.ai_deterministic_join]] | the same manifest and descriptor with claim and span order shuffled | identical IRs; overlaps resolve by the published total order regardless of shuffle |
| p_unanchored | unit | [[anchored.ir.ai_unanchored]] | manifests mixing locatable and unlocatable evidence spans | every unlocatable claim appears in the annex with its reason; the located claim set is unchanged; no fuzzy matches appear |
| p_descriptor_required | unit | [[anchored.ir.ai_descriptor_required]] | joins with missing, corrupt, and valid descriptors | missing or corrupt descriptors refuse naming the input; the valid descriptor joins |
| p_no_semantics | unit | [[anchored.ir.ai_no_semantics]] | manifests whose claims vary in status-like fields the IR should ignore | the IR claim set equals the manifest claim set — nothing filtered, ranked, or reworded |
| p_ir_schema | unit | [[anchored.ir.ai_ir_schema]] | IR instances sampled from the published data model | the published model validates every constructed IR |

## Requirements

### Requirement: Property coverage mirror

Every property row SHALL be verified by exactly one dedicated scenario;
`ah sync` SHALL derive one contract per VERIFIES link.

#### Scenario: p_single_ir

- **WHEN** renders across the format set with input tracing
- **THEN** every backend call sees exactly the IR; no backend call touches manifest, descriptor, or source bytes
- **VERIFIES** [[anchored.ir.p_single_ir]]

#### Scenario: p_reading_order

- **WHEN** descriptors with explicit order, partial order, and unordered spans
- **THEN** ordered spans follow descriptor order; unordered spans carry the marker and land at the declared fallback; ordering never follows input order
- **VERIFIES** [[anchored.ir.p_reading_order]]

#### Scenario: p_charrange_anchor

- **WHEN** manifests fusing one to N claims onto overlapping spans
- **THEN** each anchor is a valid half-open range in bounds; a fused span carries one key per claim with none dropped or merged
- **VERIFIES** [[anchored.ir.p_charrange_anchor]]

#### Scenario: p_verbatim_text

- **WHEN** descriptors whose episode text includes unusual whitespace and unicode forms
- **THEN** span location applies the fleet whitespace rule identically on both sides; emission stays verbatim; collapsed matches map back to source positions deterministically
- **VERIFIES** [[anchored.ir.p_verbatim_text]]

#### Scenario: p_deterministic_join

- **WHEN** the same manifest and descriptor with claim and span order shuffled
- **THEN** identical IRs; overlaps resolve by the published total order regardless of shuffle
- **VERIFIES** [[anchored.ir.p_deterministic_join]]

#### Scenario: p_unanchored

- **WHEN** manifests mixing locatable and unlocatable evidence spans
- **THEN** every unlocatable claim appears in the annex with its reason; the located claim set is unchanged; no fuzzy matches appear
- **VERIFIES** [[anchored.ir.p_unanchored]]

#### Scenario: p_descriptor_required

- **WHEN** joins with missing, corrupt, and valid descriptors
- **THEN** missing or corrupt descriptors refuse naming the input; the valid descriptor joins
- **VERIFIES** [[anchored.ir.p_descriptor_required]]

#### Scenario: p_no_semantics

- **WHEN** manifests whose claims vary in status-like fields the IR should ignore
- **THEN** the IR claim set equals the manifest claim set — nothing filtered, ranked, or reworded
- **VERIFIES** [[anchored.ir.p_no_semantics]]

#### Scenario: p_ir_schema

- **WHEN** IR instances sampled from the published data model
- **THEN** the published model validates every constructed IR
- **VERIFIES** [[anchored.ir.p_ir_schema]]

