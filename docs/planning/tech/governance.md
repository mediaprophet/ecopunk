# Data governance — requirements for the ecopunk pipeline

**Status:** draft v0.1 · **Date:** 2026-10-12
**Applies to:** every artefact produced or consumed by [pipeline.md](pipeline.md) —
`.q42` volumes, `.hmc` packs, `.10d` assets, media, documents, manifests, registries.

Requirements use IDs `GOV-<area>-<nnn>`; stable once published, retired not renumbered.

---

## 1. Licensing & publication lanes

### GOV-LIC — licence tiers

| Tier | Meaning | Examples (AU corpus) | Lane |
|---|---|---|---|
| P0 | Public domain / CC0 | Wikidata, some GBIF records | Commons-eligible |
| P1 | Attribution | CC-BY (AusTraits, DCCEEW products, DEA, GA), CC-BY-SA (Wikipedia/Commons parts), Apache/MIT text | Commons-eligible **with attribution graph** |
| P2 | Share-alike | CC-BY-SA content | Commons-eligible; derived-pack SA obligations recorded |
| P3 | Non-commercial / restricted-redistribution | per-record CC-BY-NC in ALA/GBIF/iNaturalist; SNES-type "public product, source restricted"; some museum imagery | **Restricted lane only** — never in a Commons pack |
| P4 | Registered/embargoed/community-restricted | BioNet elevated tiers, microbiome portal terms, unpublished state extracts | Internal/bilateral only; negotiate before any pack |

**GOV-LIC-1** Every record and artefact carries its own licence claim (`dcterms:license`
or pack `meta`), provenance `prov:wasDerivedFrom`/`wasAttributedTo`, and the tier it was
evaluated under. Portal-level licence statements are evidence, not truth — ALA proves
record-level licences vary inside one portal.

**GOV-LIC-2** Licence cohorts never mix downward: a pack's lane equals the *strictest*
tier of any member (P3 member ⇒ pack is restricted-lane). Machine-checkable at S5.

**GOV-LIC-3** Attribution is a computed artefact: every pack emits an attribution list
generated from the provenance graph — not hand-written.

**GOV-LIC-4** Licence text + retrieval-date snapshot is stored in the pack
(`kind="licence"` entries) so a 50-year reader can reconstruct *what was permitted when*.

### GOV-PUB — publication gates

**Two distinct flag axes** (confirmed 2026-10-12):

1. **Dataset-side lane** — the *publication* axis. Most ecopunk datasets target the
   Permissive Commons; lane = strictest member tier. Sanctuary on a *published dataset*
   volume means "not a public artefact" (restricted/bilateral holdings).
2. **User-side context** — the *working* axis. A person working with private, personal, or
   sensitive data (e.g. their own pathology results) holds a Sanctuary volume on *their*
   device. Commons datasets are queried **into** that context — the biosphere reference
   graph is public infrastructure; the person's data never leaves it. Example use-case
   recorded: personal pathology data matched against plant-compound/phytochemistry
   knowledge (commons) inside the user's Sanctuary context — results stay sovereign, the
   commons corpus is unchanged, and no private fact can mint a public address.

- **GOV-PUB-1** `.q42` volumes are written with explicit flags: Commons only when the pack
  manifest affirmatively clears every member's licence+sensitivity evaluation; Sanctuary
  for personal/restricted/bilateral holdings. Default is closed: absence of evidence ⇒
  no Commons flag. (Engine already fails closed on public magnet minting — we mirror that.)
- **GOV-PUB-2** A pack may not mint a public address (magnet/web-seed/IPFS/Solid public
  resource) unless its `.q42` volume has `FLAG_PERMISSIVE_COMMONS` and its manifest lane is
  `commons`.
- **GOV-PUB-3** Sanctuary volumes and P3/P4 members may be listed in restricted registries
  but their bytes never enter a commons channel — including previews/thumbnails derived
  from them.
- **GOV-PUB-4** Redistribution of *raw source snapshots* stays in the authoring area under
  source terms; packs contain reviewed derivatives only (precedent: game-demo ecology
  contract — "keep the original source export out of the browser bundle unless its licence
  permits").
- **GOV-PUB-5** Cross-context reads are one-directional: a Sanctuary context may read
  Commons volumes; a Commons context may never read or persist a Sanctuary-derived value.
  Derived outputs produced inside Sanctuary carry the user's own rights profile and inherit
  the source licence obligations (attribution strings generated at render time).

### GOV-RGT — per-dataset rights structures

A lane flag alone is insufficient: commons datasets legitimately differ in their *ontological
structure of rights* — what may be done, by whom, under which obligations. Model it, don't
flatten it.

- **GOV-RGT-1** Every dataset/pack carries a **rights profile**: a semantic subgraph of
  undertakings (`values:`-aligned — Permission / Obligation / Prohibition + ODRL where it
  fits) scoped to that dataset: permitted uses, attribution form, share-alike cascade,
  non-commercial scope, indigenous-use conditions, jurisdiction riders, expiry/review dates.
- **GOV-RGT-2** Rights profiles are **data, evaluated, not strings**: the deontic layer
  checks an intended use against the profile; `values:violates` flags the abuse signal.
  A pack author's intent to publish/redistribute/derive is evaluated the same way as an
  agent's runtime intent.
- **GOV-RGT-3** Rights profiles are **composable**: a derived pack's profile = merge of
  member profiles under strictest-wins composition, with explicit conflict detection
  (e.g., P2 share-alike × P3 non-commercial on a commercial-reuse intent = conflict ⇒
  refuse, don't resolve silently).
- **GOV-RGT-4** Profiles are versioned with the dataset; a licence change upstream is a new
  rights-profile version linked by `prov:wasRevisionOf`, never an in-place rewrite.
- **GOV-RGT-5** The profile names its **rights-holder(s) and steward(s)** via the agent
  graph (§5A) — a licence without an identified rights-holder is incomplete data.

## 2. Sensitivity

**GOV-SEN-1** Three classes per the NQuin context field: `Public(0)`, `Restricted(1)`,
`Classified(2)`. Classified biodiversity data is real and present in scope — precise
locations of poaching/collection-vulnerable taxa, roost/colony sites, culturally sensitive
places.

**GOV-SEN-2** *Inherit, never loosen.* Where the source already generalises (SNES 1 km /
~10 km grids, ALA Sensitive Data Service obfuscation, BioNet access tiers, WildNet
published-subset), our copy preserves at least that coarseness for public artefacts. We may
classify stricter; never finer.

**GOV-SEN-3** Sensitivity is jurisdictional, not global: Australia maintains per-state
sensitive-species lists; the same taxon can be Public in one jurisdiction and Classified in
another. The graph must carry `(taxon, jurisdiction, sensitivity, source-list, date)` —
not a boolean on the species.

**GOV-SEN-4** Deontic gate, not convention: handling of Classified records is encoded as
`values:`/`eco:` prohibition+permission undertakings so the engine's deontic evaluator
*refuses* publication paths — a workflow rule, not a code review checklist.

**GOV-SEN-5** Pack manifests declare the sensitivity ceiling of their contents; a pack
containing Classified members is itself Classified regardless of payload encryption.

## 3. Provenance

**GOV-PRV-1** Every artefact answers, from its own graph/manifest: *what source, retrieved
when (date+version/DOI), transformed how (tools+versions+commits), reviewed by whom
(attestation), licensed as what, hashed as what.*

**GOV-PRV-2** Vocabulary: PROV-O spine (`prov:Entity/Activity/Agent/wasDerivedFrom/
wasGeneratedBy/qualifiedAttribution`) + dataset terms (DCAT/VoID for volume description) +
record-level `dcterms:license`, `dwc:*` identifiers, source GUIDs/LSIDs preserved verbatim.

**GOV-PRV-3** The immutable source snapshot hash → normalized-Turtle hash → `.q42` digest →
`.hmc` per-entry SHA-256 → whole-file CRC-32C chain is *one* chain; each link recorded so
any stage can be re-verified without trusting intermediates.

**GOV-PRV-4** Failed/hidden paths are recorded, not erased (QualiaDB convention): a rejected
inbox item keeps its failure reason in the manifest; a withdrawn pack stays in the ledger
marked superseded/withdrawn.

**GOV-PRV-5** Reviewer attestation is a signed semantic act — human (or mandated agent)
`attests` the promotion from inbox to corpus; the attestation triple rides in the pack's
provenance graph. (Same pattern as QApp `requires_attestation`.)

## 4. Honesty labels

**GOV-HON-1** Every assertion/asset carries an epistemic class: `observed` / `specimen` /
`modelled` / `inferred` / `generated` / `illustrative`. The UI honesty-chip rule applies:
a stylised game asset of a wallaby is labelled *authored illustration*, not *specimen model*.

**GOV-HON-2** Uncertainty is data: coordinate uncertainty, ID confidence, modelled-range
vs observed-range must be present as queryable fields, not buried in prose. (Precedent:
SNES publishes *modelled* distribution and says so.)

**GOV-HON-3** Aggregated presentation must not imply co-occurrence or finer precision than
sources support (game-demo contract language: a showcase "is not a claim that every animal
occurs together at one locality").

## 5. Sovereignty & ethics

**GOV-SOV-0** The sociosphere is a first-class domain: people/communities relate to place
and knowledge through a *family* of relations (custodianship, belonging, standing) of
which property/ownership is only one member. Indigenous Australian *Country* is
relational — knowledge- and obligation-bearing — not a bounded parcel; the same axiom
family, different instances, applies to other peoples and to institutional standing
generally. Full axiom family: [../sociosphere.md](../sociosphere.md). Everything below
enforces it technically.

**GOV-SOV-1** Indigenous Data Sovereignty: CARE principles (Collective benefit, Authority
to control, Responsibility, Ethics) alongside FAIR. Community-sourced ecological knowledge
is not "open data by default."

**GOV-SOV-2** Local Contexts TK/BC Labels are modelled as first-class metadata —
`eco:traditionalKnowledgeLabel`, notice/seasonality/gender/moiety conditions — and bound as
deontic restrictions on display/redistribution where the label requires it.

**GOV-SOV-3** Aboriginal and Torres Strait Islander cultural material restrictions
(seasonal, gendered, mourning restrictions on names/images of deceased persons where
applicable) are enforceable view conditions, not disclaimers.

**GOV-SOV-4** Biocultural benefit-sharing: where a source's use feeds a product, the
attribution/acknowledgement obligations (e.g., AusTraits' citation requirement, ALA download
DOIs) are *executed* in the pack's generated attribution artefact — a license term made
executable.

**GOV-SOV-7 — Identity criteria prohibition.** Identity, standing, membership, and
indigeneity are constituted by relations — culture, knowledge, place of life/birth,
language, community recognition — never by genomic or racial markers (SOC-AX-8). Using
DNA/ancestry data as an identity gate is recorded as a `values:Prohibition`: deriving,
filtering, or gating a person's identity/standing/rights from genomic markers violates the
values baseline. Genomic data in the corpus exists for biological purposes only and carries
that restriction in its rights profile.

**GOV-SOV-8 — Language is authoritative, translation is gloss.** Indigenous-language terms
are the authoritative semantic for their concepts; English labels are approximating glosses
(SOC-AX-9). Interfaces show language + gloss provenance; no system may silently treat a
gloss as the term (or vice versa).

## 5A. Agent & stewardship graph (social graph of assets)

**Confirmed 2026-10-12:** every asset carries a comprehensive social graph — the people,
organisations, and communities that made, hold, steward, fund, and authorise it. This is
primary data, not decoration: attribution strings, rights-holder identification, contact
paths, and stewardship succession all resolve *through* it.

**GOV-SOC-1 — Agent classes** (aligned to `values:Agent` hierarchy in qualiaDB
core-ontologies and `hcai:` distinctions):
`NaturalPerson`, `LegalPerson`/`Organisation` (W3C `org:` compatible), `OrgUnit`,
`Community` (incl. Indigenous custodial communities as collective agents),
`Institution/Collection`, `ArtificialAgent` (build tools, LLM-assisted mappers — each under
an accountable natural-person principal), `SoftwareAgent` (pipeline tool instances).

**GOV-SOC-2 — Roles.** Assets bind agents via **qualified roles**, not flat properties
(PROV-O `prov:qualifiedAttribution`/`qualifiedDelegation` pattern):
`rightsHolder`, `owner`, `steward`/`custodian`, `contributor`, `author`, `collector`,
`identifier` (taxonomic determiner), `digitiser`, `reviewer`/`attester`, `publisher`,
`distributor`, `funder`, `host` (infrastructure), `licensor`, `traditionalCustodian`.
New roles are vocabulary-governed, not free-text.

**GOV-SOC-3 — Temporal stewardship.** Roles are time-boxed qualified relations
(`validFrom/validTo`): a dataset's steward in 2026 differs from 2046's; succession is a
chain of stewardship events, preserving "who was responsible when" — essential across a
50-year corpus where organisations merge, rename, and dissolve (e.g., DCCEEW itself is a
succession of departmental names; CSIRO/TERN hosting arrangements change).

**GOV-SOC-4 — Identity.** Agents carry durable external IDs where they exist:
ORCID (people), ROR/GRID-legacy (organisations), Wikidata QIDs, ISNI, ABN/ASIC for AU
entities, DOI/RAiD for projects; plus DID (`did:`) for verifiable in-ecosystem identity
and `did:q42:` internal addressing. Agent records are themselves semantic entities in the
graph — queryable, versioned.

**GOV-SOC-5 — The graph serves the artefacts.**
- Attribution artefacts (GOV-LIC-3) are **generated from** this graph (citation strings,
  "data from X via Y, stewarded by Z").
- Rights profiles (GOV-RGT-5) name their holders through it.
- Contact/relationship discovery: a pack user can traverse asset → steward → organisation
  → contact, subject to privacy minimisation (GOV-PRV-6).
- Delegation chains are auditable: contributor → steward → publisher → distributor shows
  who authorised what, when (PROV `actedOnBehalfOf`/`qualifiedDelegation`).

**GOV-SOC-6 — Contribution provenance at record granularity.** Per-record contributors
(e.g., the herbarium that digitised a specimen, the observer of an iNaturalist record,
the 340+ AusTraits data contributors) remain attached to the record — roll-ups to
dataset level are views, not replacements.

## 6. Identity & durability

**GOV-DUR-1** Stable identifiers: prefer source-minted durable IDs (NSL LSIDs, AFD GUIDs,
Wikidata QIDs, DOIs, AGLDWG `/def/` PIDs) over ecopunk-local IDs; our IDs *link to* them.

**GOV-DUR-2** Namespace strategy for `eco:` IRIs must resolve to a 50-year plan:
either an AGLDWG `/def/` PID (government-persistence) or a w3id.org-style community PID —
not a domain we might lose. Decision recorded as charter open question → resolve before
first pack ships.

**GOV-DUR-3** Immutable releases: every published volume/pack is content-addressed
(digest-named), never edited; corrections are new versions linked `prov:wasRevisionOf`.

**GOV-DUR-4** Self-describing archives: packs carry their own ontology dependencies,
SHACL shapes, licences and vocab dictionaries, so a `.hmc` opened in 50 years explains
itself without the network.

**GOV-DUR-5** Format versions pinned: pack manifest records the QualiaDB format versions it
was built against (Q42 v4, QBDL v1, `.10d` section set, NQuin layout) — the ledger records
the upstream commit the pack was verified on.

## 7. Integrity & verification

**GOV-INT-1** Integrity is layered: per-entry SHA-256 + whole-file CRC-32C (`.hmc`),
per-section seals + whole-file seal (`.10d`), Merkle-DAG history section where temporal
indexing is enabled (`.q42`).

**GOV-INT-2** Verification receipts: at S6/S7 the build emits a receipt (input hashes,
tool versions, output digests, validation results) stored as a `kind="provenance"` entry.

**GOV-INT-3** Acceptance is test-based: a pack is not "published" until a reader harness
re-opens it, re-verifies digests, re-runs SHACL, and SPARQL spot-checks a sample of
entities — receipts filed to the capability ledger.

## 8. Roles & review gates

| Gate | Human requirement |
|---|---|
| S3 promote inbox→corpus | Human review mandatory (licence, sensitivity, sovereignty, quality) — attestation triple required |
| S5 licence-tier evaluation | Automated pre-check + human sign-off for any P2+ member |
| S8 lane decision | Human sign-off for `commons` publication; restricted publication may be automated below threshold |
| Correction/withdrawal | Signed withdrawal record; downstream dependents notified via registry version notes |

**GOV-ROL-1** Agents (build bots, LLM-assisted mappers) act under declared identity DIDs;
their activity is logged as `prov:Agent` with an accountable human principal — per the
QualiaDB agency model.

## 9. Privacy & personal data (edge case)

**GOV-PRV-6** Occurrence collectors, observers, transcribers, and photo authors are people:
personal identifiers are minimised to attribution-level (name/ORCID where the licence
requires credit), never contact/home detail. Collector-identity fields follow Bionomia-style
disambiguation without leaking private data.

## 10. Open questions

1. Does QualiaDB need a distinct "Licence/rights modality" opcode family, or are licence
   lanes fully expressible as context + deontic quins? (Engine gap candidate.)
2. Restricted-lane *distribution* mechanism (bilateral lanes exist in routing metadata —
   what does a restricted-pack exchange look like in practice?)
3. TK-label machine enforcement granularity — which conditions are deontically expressible
   today vs. require engine work?
4. Record-level licence density: GBIF-scale volumes will carry millions of per-record
   licences — confirm the context/metadata bit budget can express this or whether a
   licence-sidecar volume is the model.
