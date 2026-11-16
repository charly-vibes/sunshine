---
id: render.backends
kind: intent
statement: "WHEN the anchored-document IR is serialized to a supported format THE sunshine backend SHALL emit a document in that format's published profile such that the profile's parser recovers each anchor and claim field"
---

# backends

One IR, thin backends: pdf, md, org, html in v1. Each backend publishes
a profile — a strict subset grammar plus serialization rules plus a
version — and emission follows the profile exactly, because the audit
contract depends on parsing emitted files deterministically. A backend
without a parser is a view that cannot be verified, so the shipping rule
is absolute: every backend ships with its audit parser or does not ship.

The profiles differ in what they preserve. The pdf backend is the only
geometry renderer: highlights on the original page, manifest embedded as
an attachment, stamp in XMP, source content stream untouched. The text
backends are geometry carriers, never geometry renderers: locators ride
frontmatter and property drawers as data. The md backend emits pandoc
bracketed spans by default and a pure-commonmark flavor where anchors
degrade to HTML comments; the org backend emits a strict subset — custom
claim and entity links, property drawers per claim, org-noter-compatible
locator properties — and excludes babel, tables, and export-block
positioning from its grammar so the audit parser stays small.

## Constraints

| id | kind | expr | traces_to |
|----|------|------|-----------|
| bk_profile | invariant | each backend publishes a profile — grammar subset, serialization rules, version — and emission follows it exactly; profile versions ride the render stamp | [[render.backends]] |
| bk_pdf_annotations | invariant | the pdf backend renders anchors as highlight annotations keyed by claim key, embeds the manifest as a file attachment, and writes the render stamp as XMP; the source page content stream is never modified | [[render.backends]] |
| bk_md_spans | invariant | the md backend emits bracketed spans with claim-key attributes in the pandoc flavor; the commonmark flavor moves bindings to HTML comments; both flavors carry a frontmatter registry and a claims section with identical claim fields | [[render.backends]] |
| bk_org_subset | invariant | the org backend emits the strict org profile — claim and entity custom links inline, one heading per claim with a property drawer carrying frame, roles, locator, and lineage, locator properties org-noter-compatible; babel, tables, and export-block positioning are excluded from the profile grammar | [[render.backends]] |
| bk_html_selfcontained | invariant | the html backend emits one self-contained file: visible claim markup plus the manifest embedded as a JSON script block; no external assets or network references | [[render.backends]] |
| bk_geometry_carrier | invariant | text backends carry locators as data in frontmatter and property values and never render geometry; geometry rendering belongs to the pdf backend alone | [[render.backends]] |
| bk_audit_pairing | invariant | each backend ships with a parser implementing the audit contract for its profile; a backend without a passing round-trip gate does not ship | [[render.backends]] |
| bk_degrade | advisory | emitted md, org, and html remain legible to tools ignorant of the profile; anchors, drawers, and comments read as plain text | [[render.backends]] |
| bk_new_backend | extension_point | a new format enters by publishing a profile, an emitter, and an audit parser, amending the render contract's closed format set in the same change | [[render.backends]] |
| rb_ship_bar | invariant | a backend counts as shipped only when four criteria hold together — totality over the anchor manifest strict parser's domain (every manifest am_strict_parse accepts renders with its full claim set and no claim dropped; input outside that domain refuses by name at render time with a machine-readable reason, never a silent subset), round-trip green (audit(render(m)) == m over the generated suite), the backend's render command answers the render contract's suite envelope (rc_envelope) on both success and refusal with exit status mirroring ok, and byte-reproducible output for identical inputs; absence of any one means the backend is unshipped | [[render.backends]] |
| rb_deterministic_pdf | invariant | every timestamp, document ID, and other nondeterministic embedded value in pdf output is derived from the manifest digest; no wall-clock or random source may reach artifact bytes, so a pdf backend that cannot pin them does not ship | [[render.backends]] |
| rb_audit_priority | invariant | no backend profile choice may break the audit round-trip; a render/audit conflict is always a backend defect, never an audit relaxation | [[render.backends]] |

## Model

### States

- `selected`
- `emitted`
- `refused`

### Transitions

| id | from | to | guard |
|----|------|----|-------|
| emit | selected | emitted | [[render.backends.bk_profile]] |
| refuse_no_parser | emitted | refused | [[render.backends.bk_audit_pairing]] |
| refuse_unknown | selected | refused | [[render.backends.bk_profile]] |

## Properties

| id | kind | derives_from | generator | predicate |
|----|------|--------------|-----------|-----------|
| p_profile | unit | [[render.backends.bk_profile]] | emitted documents per backend, checked against their profiles | every emitted document validates against its published profile; the stamp names the profile version |
| p_pdf_annotations | unit | [[render.backends.bk_pdf_annotations]] | pdf renders over generated manifests | annotations exist per claim key; the manifest attachment parses back; XMP carries the stamp; source content streams are byte-identical to the original |
| p_md_spans | unit | [[render.backends.bk_md_spans]] | md renders in both flavors over the same IR | pandoc flavor spans carry claim keys; commonmark flavor carries the claims in comments; both parse back to the same claim field values |
| p_org_subset | unit | [[render.backends.bk_org_subset]] | org renders over generated IRs | emitted org stays inside the profile grammar — no babel, no tables, no positioning blocks; claim headings carry the full property drawer |
| p_html_selfcontained | unit | [[render.backends.bk_html_selfcontained]] | html renders with asset extraction attempted | the file references nothing external; the embedded JSON parses back to the manifest |
| p_geometry_carrier | unit | [[render.backends.bk_geometry_carrier]] | text-backend renders over descriptors with coordinates | text outputs contain locator data as values; no drawing or positioning operators appear |
| p_audit_pairing | unit | [[render.backends.bk_audit_pairing]] | the backend roster and its shipped parsers | each shipped backend has an audit parser passing its round-trip gate; an unpaired backend cannot ship |
| p_degrade | unit | [[render.backends.bk_degrade]] | emitted md, org, html stripped of profile syntax | the remaining plain text preserves narrative and claim prose legibly |
| p_new_backend | unit | [[render.backends.bk_new_backend]] | a candidate backend with profile, emitter, and parser | admission requires the published triple and the amended closed set in one change |
| p_ship_bar | unit | [[render.backends.rb_ship_bar]] | the backend roster evaluated against the four ship criteria | a backend counts as shipped only when totality over the anchor manifest strict parser's domain (every manifest am_strict_parse accepts renders with its full claim set and no claim dropped; input outside that domain refuses by name at render time with a machine-readable reason, never a silent subset), round-trip green (audit(render(m)) == m over the generated suite), the backend's render command answers the render contract's suite envelope (rc_envelope) on both success and refusal with exit status mirroring ok, and byte-reproducible output for identical inputs all hold together; absence of any one leaves it unshipped |
| p_deterministic_pdf | unit | [[render.backends.rb_deterministic_pdf]] | pdf renders of identical manifests across wall-clock hour and host differences | every timestamp, document ID, and other nondeterministic embedded value is derived from the manifest digest; no wall-clock or random source reaches artifact bytes, so artifact bytes are identical across runs and a pdf backend that cannot pin them does not ship |
| p_audit_priority | law | [[render.backends.rb_audit_priority]] | backend render-and-audit pairs over artifacts mutated per the law's case classes — identity, presentation-only (whitespace, NFC form, comment marker spelling, reflow, attribute order), content-bearing (claim field and evidence edits), and composed presentation edits in both orders — per backend profile | **identity:** an unmodified render audits clean for every backend **presentation-mutation:** edits confined to the published canonicalizer's presentation-only classes — whitespace, NFC form, comment marker spelling, reflow, attribute order — canonicalize clean and never drift **content-mutation:** any content-bearing edit drifts with a named class — a render/audit conflict is always a backend defect, never an audit relaxation, so no profile choice breaks the round-trip **associativity:** canonicalizing composed presentation edits in either order yields the same clean form — canonicalize(w∘t∘r(m)) = canonicalize(r∘t∘w(m)) = canonicalize(m), where w, t, and r are whitespace, NFC, and comment-format edits — so the order of presentation mutations never changes the audit verdict |
