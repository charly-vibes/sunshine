---
id: fleet.boundaries
kind: intent
statement: "THE sunshine tool SHALL own only the transformation of claim anchor manifests into rendered artifacts, delegating source geometry to sunset and claim semantics to bajan"
---

# boundaries

sunshine is the fleet's render layer: it compiles claim anchor manifests
into tagged, human-readable documents and audits them back. It sits
downstream of two authorities it must never absorb: sunset owns source
geometry (DSC descriptors, selectors, reading order) and bajan owns
claims (the claim graph, adoption, contradiction). The name is the
pipeline: what sunset takes in during the day, sunshine renders out.

The boundary is enforced by the deletion test, stated here as a review
invariant: if a proposed capability would still make sense with sunshine
deleted, it belongs to sunset or bajan and is refused in this repo.
Rendered artifacts are views — the only mutation path back to the claim
store runs through bajan's human-adoption review, never through this
tool. This spec is the root of the corpus; every other spec here
consumes its separation rules.

## Purpose

sunshine is the fleet's render layer: it compiles claim anchor manifests
into tagged, human-readable documents and audits them back. It sits
downstream of two authorities it must never absorb: sunset owns source
geometry (DSC descriptors, selectors, reading order) and bajan owns
claims (the claim graph, adoption, contradiction). The name is the
pipeline: what sunset takes in during the day, sunshine renders out.
The boundary is enforced by the deletion test, stated here as a review
invariant: if a proposed capability would still make sense with sunshine
deleted, it belongs to sunset or bajan and is refused in this repo.
Rendered artifacts are views — the only mutation path back to the claim
store runs through bajan's human-adoption review, never through this
tool. This spec is the root of the corpus; every other spec here
consumes its separation rules.

## Constraints

| id | kind | expr | traces_to |
|----|------|------|-----------|
| fb_geometry_owner | invariant | sunshine derives each page and word location from DSC descriptors produced by sunset; it contains no PDF text extraction and never re-derives a locator; a resolution failure names the descriptor instead of guessing | [[fleet.boundaries]] |
| fb_claim_owner | invariant | claim lifecycle — extraction, adoption, invalidation, contradiction — lives in bajan; sunshine reads claims only through the anchor manifest and its code contains no claim-creation path | [[fleet.boundaries]] |
| fb_readonly_view | invariant | a rendered artifact is a view; mutation flows back to bajan as proposals through its human-adoption path; a rendered artifact is never a source of truth for claims or geometry | [[fleet.boundaries]] |
| fb_no_persistence | invariant | sunshine holds no durable state between invocations; every artifact is rebuildable from (manifest, descriptor, source, tool version); deleting sunshine loses no information | [[fleet.boundaries]] |
| fb_deletion_test | invariant | a proposed capability that would still make sense if sunshine were deleted belongs to sunset or bajan and is refused at this repo's review | [[fleet.boundaries]] |
| fb_version_contract | advisory | sunshine pins the compatible schema versions of the anchor manifest (bajan side) and the descriptor (sunset side); version drift is refused loudly at the seam with remediation | [[fleet.boundaries]] |
| fb_seam_published | extension_point | the two seam contracts — the anchor manifest schema and the descriptor contract — are published for the neighbor tools to conform to | [[fleet.boundaries]] |

## Model

### States

- `proposed`
- `admitted`
- `redirected`

### Transitions

| id | from | to | guard |
|----|------|----|-------|
| admit | proposed | admitted | [[fleet.boundaries.fb_deletion_test]] |
| redirect | proposed | redirected | [[fleet.boundaries.fb_deletion_test]] |

## Properties

| id | kind | derives_from | generator | predicate |
|----|------|--------------|-----------|-----------|
| p_geometry_owner | unit | [[fleet.boundaries.fb_geometry_owner]] | a build of the crate and its dependency list | the dependency graph contains no PDF text-extraction crate; locator values in emitted code come only from descriptor fields |
| p_claim_owner | unit | [[fleet.boundaries.fb_claim_owner]] | the crate's public API surface | the public API exposes no function that creates or mutates a claim; the only claim input is the manifest path |
| p_readonly_view | unit | [[fleet.boundaries.fb_readonly_view]] | render invocations over sample inputs, with file hashes taken before and after | the source document, descriptor, and manifest byte-hashes are unchanged after render; no store path is written |
| p_no_persistence | unit | [[fleet.boundaries.fb_no_persistence]] | render invocations in sequence over identical inputs in a fresh temp dir | byte-identical outputs with no state dir created; deleting any cache-like dir never changes a subsequent output |
| p_deletion_test | unit | [[fleet.boundaries.fb_deletion_test]] | each capability named in the CLI help and specs | every capability fails the deletion test — it dies with the tool — or the spec corpus is refused in review |
| p_version_contract | unit | [[fleet.boundaries.fb_version_contract]] | manifests and descriptors carrying unknown future schema versions | version mismatch refuses with a machine-readable reason naming the supported versions, before any rendering |
| p_seam_published | unit | [[fleet.boundaries.fb_seam_published]] | the published schema documents in this corpus | both seam schemas exist as published artifacts with version numbers that ride the render stamp |

## Requirements

### Requirement: Property coverage mirror

Every property row SHALL be verified by exactly one dedicated scenario;
`ah sync` SHALL derive one contract per VERIFIES link.

#### Scenario: p_geometry_owner

- **WHEN** a build of the crate and its dependency list
- **THEN** the dependency graph contains no PDF text-extraction crate; locator values in emitted code come only from descriptor fields
- **VERIFIES** [[fleet.boundaries.p_geometry_owner]]

#### Scenario: p_claim_owner

- **WHEN** the crate's public API surface
- **THEN** the public API exposes no function that creates or mutates a claim; the only claim input is the manifest path
- **VERIFIES** [[fleet.boundaries.p_claim_owner]]

#### Scenario: p_readonly_view

- **WHEN** render invocations over sample inputs, with file hashes taken before and after
- **THEN** the source document, descriptor, and manifest byte-hashes are unchanged after render; no store path is written
- **VERIFIES** [[fleet.boundaries.p_readonly_view]]

#### Scenario: p_no_persistence

- **WHEN** render invocations in sequence over identical inputs in a fresh temp dir
- **THEN** byte-identical outputs with no state dir created; deleting any cache-like dir never changes a subsequent output
- **VERIFIES** [[fleet.boundaries.p_no_persistence]]

#### Scenario: p_deletion_test

- **WHEN** each capability named in the CLI help and specs
- **THEN** every capability fails the deletion test — it dies with the tool — or the spec corpus is refused in review
- **VERIFIES** [[fleet.boundaries.p_deletion_test]]

#### Scenario: p_version_contract

- **WHEN** manifests and descriptors carrying unknown future schema versions
- **THEN** version mismatch refuses with a machine-readable reason naming the supported versions, before any rendering
- **VERIFIES** [[fleet.boundaries.p_version_contract]]

#### Scenario: p_seam_published

- **WHEN** the published schema documents in this corpus
- **THEN** both seam schemas exist as published artifacts with version numbers that ride the render stamp
- **VERIFIES** [[fleet.boundaries.p_seam_published]]

