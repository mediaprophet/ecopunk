# The Sociosphere — people, place, and knowledge in the biosphere graph

**Status:** draft v0.1 (concept, recorded from project direction 2026-10-12)
**Feeds:** [domain-model.md](domain-model.md) (planned), [ontology-stack.md](tech/ontology-stack.md)
(planned), [governance.md](tech/governance.md) §5/§5A, [australia.md](data-sources/australia.md) §5A

---

## 1. The claim being modelled

The biosphere graph is incomplete if it only models *things that live*. Peoples,
communities, and institutions have their own structured reality — call it the
**sociosphere** — which relates to the biosphere through place, knowledge, custodianship,
and practice. Two observations anchor the design:

1. **Place is not (only) property.** For Indigenous Australians, *Country* is a
   knowledge-bearing, obligation-bearing *relation*: people belong to Country; Country
   carries law, ancestry, story, and species relations. It is not reducible to a bounded
   parcel with an owner. The same class of relation exists elsewhere — kaitiakitanga for
   Māori, siida for Sámi, tribal relations for First Nations — and, in weaker
   institutional form, for anyone with a custodial relationship to a place (a ranger, a
   farm family, a research station).
2. **Standing is dimensional and domain-specific.** Authority over knowledge is not a
   credential boolean. An Elder holds authority through custodianship, seniority, kinship,
   and lived knowledge; a professor through credential, office, and publication; a senior
   political professional through office and practice; a young person starting out holds
   standing too — just a different position on different axes. The *axiom family is
   shared*; the *axiom instances differ by domain/tradition/jurisdiction*.

Design consequence: the sociosphere is a **first-class domain layer** with its own
vocabulary and pluggable axiom sets — not a set of tags on agents.

> **Terminology note — "human centric":** used here in its defined sense — the
> tools-level property the term was coined for (DIDs, W3C VCs, RWW/Solid,
> WebPayments: **human**-anchored infrastructure — human being, not juridical
> "person"), not the later industry usage as a design attitude toward AI.
> SOC-AX-14 is that property made structural. Lineage + distinction recorded
> in charter.md §4, principle 11.

## 2. Axiom family (draft — to be formalised into SHACL/N3)

| Axiom | Statement | Western/legal analogue |
|---|---|---|
| **SOC-AX-1 Place-relations are a family** | `custodianship`, `belonging`, `tenure`, `stewardship`, `residence`, `ancestral tie`, `sacred relation`, `visitation right`, `exclusion` are sibling relation types between agent and place. `ownership` is *one member* of the family — never the root. | `ownership`/`title` is the sole first-class legal form; everything else is "interest in land" |
| **SOC-AX-2 Country is relational, not territorial** | A `Country` entity = a relation-bundle over agents, ancestors, species, landforms, waters, and law/story — optionally *with* a spatial extent, never *defined by* it. A polygon is a representation, not the thing. | Parcel/Lot = geometry + title |
| **SOC-AX-3 Standing is qualified, multi-axis** | Authority/standing = qualified relation `(agent, domain, axis, degree, context, evidence)` — axes like custodianship, seniority/elderhood, credential, office, practice, initiation, kinship. The same shape models Elder and professor; different axiom sets determine which axes carry authority in which contexts. | "expert" = credentials; seniority informal |
| **SOC-AX-4 Knowledge rights pair with obligations** | The right to hold/speak/teach knowledge pairs with obligations of care: speak-for-Country ↔ care-for-Country. Deontic pairing is engine-native (`values:Right` ↔ `values:correlativeDuty`). | IP regimes grant rights without custodial obligations |
| **SOC-AX-5 People↔species relations are cross-domain** | Totemic/kin relations bind people/communities to taxa — a biosphere×sociosphere bridge. A taxon can *be kin* to an agent, not merely occur near them. | species–human relation = use/conservation only |
| **SOC-AX-6 Knowledge systems record ecology** | Indigenous seasonal/phenological calendars (e.g. Nyoongar six seasons, Tiwi three) are *data* — observed ecological temporal structure — not folklore prose. | "traditional knowledge" treated as anecdote |
| **SOC-AX-7 Context may stack multiple standing systems** | The same place/knowledge can be legitimately described by custodial *and* institutional standing at once; neither erases the other; conflicts are represented, not resolved silently. | Sovereign title tends to extinguish others |
| **SOC-AX-8 Identity is relational, never genomic** | Who a person/community *is* — including indigeneity — is constituted by culture, knowledge, place of life/birth, language, and community-recognised relations. DNA/race markers are **prohibited as identity axioms**: basing identity on them constrains whom people may love, have children with, and care for — inconsistent with human rights (recorded harms: racial classification regimes, blood-quantum rules, stolen-generation policies). Identity claims are modelled as qualified social relations and self/community assertions — not biological attributes. | Legal "race"/blood-quantum definitions; genomic ancestry used as gate |
| **SOC-AX-9 Language carries meaning; translation is a gloss** | A term belongs to *a language* and carries concepts its translations may not possess ("Country" ≠ "land"; kin terms with no English counterpart). Labels in other languages are *approximating glosses*, not equivalences — modelled as concept-in-language entities with mapping relations (with provenance and fidelity notes), never assumed identity. True of all languages, English included — English is itself a sociosphere language whose terms carry their own culturally loaded meanings. | `rdfs:label`/`skos:prefLabel` multilingual tags treated as interchangeable |
| **SOC-AX-10 Community recognition is a standing source** | A person's standing may be grounded in *recognition by a community* (custodianship acknowledged by a group, acceptance into language/nation, appointed authority) — a peer relation, not a credential or registry entry. Registries (native title, electoral, institutional) are evidence, not constitution. | Registry/registry-of-record treated as constitutive |
| **SOC-AX-11 Relations carry time** | Custodianship, membership, standing, names, and statuses hold over *validity intervals*; historical states are first-class queryable records, never silently overwritten. Federation-era borders, past Country boundaries, former names — all remain. See DOM-AX-2/3 in [domain-model.md](domain-model.md). | Current-state databases that overwrite history |
| **SOC-AX-12 Border is a kind-family** | Custodial, jurisdictional, ecological, linguistic, cadastral, historical borders coexist, overlap, and version over time; none outranks another by construction (DOM-AX-3). Native-title determinations are *one kind* of overlay on Country — not Country itself. | "The" border treated as single ground truth |
| **SOC-AX-13 Use-knowledge lives in language** | Medicinal/functional/food/material uses of living things are frequently recorded inside indigenous language and teaching; the use relation attaches through the concept-in-language term and its knowledge-holders (DOM-AX-5), inheriting TK-label conditions. | Ethnobotany flattened to "plant X cures Y" factoids |
| **SOC-AX-14 Authority is human-rooted** | Foundational delegated authority terminates in **human beings** — deliberately not "persons": in many jurisdictions a "person" includes companies; a legal person is a *constituted* wrapper through which humans exercise authority collectively, never an independent root. Structured groups — *incorporated or unincorporated* — exercise authority only through the human beings who constitute/govern them; wards exercise it through guardians (`capacity`/`GuardianshipArrangement`). Software agents never hold foundational authority: every agent act must resolve through a delegation chain (`delegation` modality, ZCAP-LD-style) to an accountable human being or human-constituted body — severing that chain revokes the agent's standing (revocation cascades). `values:Agent` spans natural persons, legal persons, and ArtificialAgents, but authority originates only in human beings — groups channel it, agents receive it downstream. Consequence: no anonymous-agent discount in economics (an agent's category inherits its accountable principal's); no "the model decided" as an authority claim — machine outputs are proposals, humans ratify; and **biometric agency is non-delegable** — a human being independently maintains agency over their own biometrics (consent-gated `biosense`; no platform custodianship of face/voice/physiology). | Agents/services as first-class authorities; platforms holding authority on users' behalf; "the algorithm" as accountable actor; companies as independent authority roots; biometric data as platform asset |

## 3. Consequences for the data model

- `eco:`/`soc:` vocabulary needs: place-relation family; `Country` entity; standing axes
  vocabulary; custodianship/time-boxed qualified roles; totemic relation predicates;
  seasonal-calendar structures (phenological phases ↔ observable species/weather events);
  knowledge-holding roles with transmission constraints (who may speak, teach, display).
- Everything in §5A of governance.md (agent & stewardship graph) applies; sociosphere
  roles extend the role vocabulary with `traditionalCustodian`, `knowledgeHolder`,
  `elder`, `speakerFor`, `initiate`-family terms — each qualified, never global flags.
- **Identity modelling is relational (SOC-AX-8/10):** an agent's indigeneity, standing,
  or membership is a set of qualified relations (`recognisedBy`, `raisedAt`, `speaks`,
  `memberOf`, `custodianOf`, `bornOn` country) — never a genome attribute. Genomic
  ancestry data may exist in the corpus *as data* (e.g., ARGA reference genomes) but is
  deontically forbidden as an identity gate (GOV-SOV-7).
- **Concept-in-language modelling (SOC-AX-9):** terms are entities — `(term, language,
  concept, orthography, speakers)` — cross-language links are typed `gloss`/`closerTo`/
  `widerThan` mapping relations with provenance, not `owl:sameAs`. AUSTLANG language
  identifiers become first-class language entities. SKOS-XL-style term entities are the
  likely substrate (verify fit before committing).
- Interfaces must not render a custodianship boundary as a property line, nor reduce an
  Elder's standing to a credential chip. This is a *rendering-honesty* requirement
  (GOV-HON family) — the visual grammar must distinguish relation types.
- **Authority-graph modelling (SOC-AX-14):** agent entities carry a `kind`
  (natural person / legal person — incorporated or unincorporated / software
  agent / ward-under-guardianship) and every delegation edge records
  `delegatedBy → accountable root`; queries and deontic checks can ask *"which
  human(s) does this act resolve to?"* — an answer is mandatory. Group
  authority flows through member/governor relations (an unincorporated
  association's authority is the bundle of its members' delegations — no
  legal-fiction person required); guardianship routes through
  `ThirdPartyAnchoredObligation`-anchored `GuardianshipArrangement`s
  (cooperative-layer.md §1).
- Sensitivity (GOV-SEN) intersects: some knowledge is men-only/women-only/seasonal/
  initiated — these are deontic view conditions on the sociosphere graph itself, not
  bolt-on disclaimers.
- **The statistical shadow is a sibling layer, not sociosphere content.**
  Official statistics describe populations/areas/cohorts under governed
  methodologies — modelled in [tech/statistics.md](tech/statistics.md)
  (STAT-AX family). They interact with sociosphere entities (Country,
  communities, languages) through *geography and subject joins* — never as
  identity attributes of persons (SOC-AX-8), and statistics *about* Indigenous
  peoples carry sovereignty conditions (STAT-AX-10).

## 4. Evidence and sources (AU-first)

See [data-sources/australia.md](data-sources/australia.md) §5A for the
custodial/knowledge register, and
[data-sources/socioeconomic.md](data-sources/socioeconomic.md) for the
statistical-source register (ABS/AIHW/international systems):

- **AIATSIS** — AUSTLANG language database, Map of Indigenous Australia (nations/language
  groups as *relation-bearing regions*), subject thesaurus.
- **NNTT registers** — Native Title determinations + ILUAs (spatial custodianship polygons;
  note these are *legal* overlays over Country, not Country itself — SOC-AX-2 applies).
- **Indigenous Protected Areas** — inside CAPAD; co-managed custodial land.
- **Indigenous seasonal calendars** — CSIRO/BOM/community partnerships (Tiwi, Nyoongar,
  Gooniyandi, Ngadju…) — phenology as structured observation.
- **I-Tracker / ranger programs** — biocultural monitoring data, community-owned.
- **Local Contexts** — TK/BC label registry and model.
- **Heritage registers** (state/federal) — mostly restricted; the canonical sensitivity
  example.

## 5. Universalisation check (design constraint)

Any sociosphere construct must be testable against at least three settings before it's
accepted into the core vocab: (a) Indigenous Australian custodianship, (b) another
Indigenous/collective system elsewhere (Māori, Sámi, First Nations…), (c) an institutional
Western analogue (university, public office, professional standing). If a construct only
survives one setting, it's a *per-domain axiom set*, not core. This keeps the vocabulary
honest — culturally specific axioms live in per-tradition profile modules, structurally
equivalent under the shared axiom family.

## 6. Open questions

1. ~~Should the sociosphere be a sibling namespace (`soc:`) or inside `eco:`?~~
   **Resolved 2026-10-12:** ecopunk is the umbrella (biocultural scope); `soc:` is a
   sibling domain namespace alongside `bio:` (biosphere) and `env:`/`geo:` (earth-system
   context), bridged by `eco:` relations. See charter §1.
2. Which existing vocabularies already model custodial/place relations adequately?
   (GeoSPARQL gives geometry; DCAT/PROV give agents; nothing mainstream gives
   Country-as-relation — likely original vocabulary work, aligned to AGLDWG patterns.)
3. How do standing axes interact with the deontic evaluator — is "authority to view/teach"
   a permission check, a context class, or both?
4. Do we need a formal model of *oral/knowledge transmission* (lineage of knowing) for
   provenance, distinct from document provenance? Probably yes — `prov:` is document-centric.
