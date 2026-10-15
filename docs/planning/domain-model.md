# ecopunk — Domain Model

**Status:** draft v0.1 · **Date:** 2026-10-12
**Scope:** the structure of what ecopunk models — domain layers, the scale ladder, entity
and relation classes, spatio-temporal rules, and the living/made partition.
Vocabulary bindings live in `tech/ontology-stack.md` (planned); sociosphere axioms in
[sociosphere.md](sociosphere.md); governance rules in [tech/governance.md](tech/governance.md).

---

## 1. Domain layers

ecopunk is the umbrella (biocultural scope). Domains are sibling namespaces bridged by
`eco:` relations:

| Namespace | Domain | Contents |
|---|---|---|
| `bio:` | **Biosphere** | taxa, organisms, occurrences, traits, cells, genes, compounds, habitats, ecosystems, biomes, ecoregions, food webs |
| `soc:` | **Sociosphere** | agents, communities, languages, Country, custodianship, standing axes, knowledge systems — see [sociosphere.md](sociosphere.md) |
| `env:` | **Earth-system context** | climate layers, soils, hydrology, landforms, elevation, bathymetry, fire regimes — the substrate both other domains stand on |
| `stat:` | **Statistics** | governed observations about populations/areas/cohorts — datasets, dataflows, methodologies, classifications, claims — [tech/statistics.md](tech/statistics.md) |
| `eco:` | **Bridge / package core** | scale ladder, pack manifests, cross-domain relations, shared identity and depiction relations, rights-profile attachment |
| external | **QualiaDB core** | `values:` rights/duties, `q42:` internals, PROV-O, GeoSPARQL, Darwin Core, TERN/SOSA profiles — aligned, not redefined |

## 2. DOM-AX — domain axioms

### DOM-AX-1 The living world vs the made world

The primal partition. Two halves of the knowable inventory:

- **Living world** — entities of life and its contexts: organisms, taxa, cells, genes,
  compounds-in-nature, habitats, ecosystems, Country, weather. *Self-emergent* — not
  authored by humans. (Provisional naming — alternatives considered: *the given* /
  *natural world* / *wild*; "world of god" is the intent, a less sectarian term wanted —
  **open for decision**, §7-1.)
- **Made world** — artefacts: manufactured objects, commodities, settlements,
  infrastructure, data products, models, packs. *Brought-forth by human making.*

**The rule:** a living entity is never intrinsically a "thing", "resource", or
"commodity". `Commodity`/`Product`/`TradeGood` status is **act-derived**: it exists only
via a human `Act` (harvest, processing, listing-for-sale, trade) that links a living
entity to commerce. `Commodity ⊑ made:Artefact`; nothing in `bio:` is ever typed
`Commodity` directly — the link runs through the act, with provenance, licence, and
values-layer checks. Alignment: qualiaDB's values corpus already encodes *"a person is
not a thing"*; DOM-AX-1 generalises — **a life is not a thing**.

**Provenance decides class, not substance.** The partition applies to entity
*instances/lineages* by origin, not to chemical or morphological identity:

- The *same* compound can exist in both worlds: urea in mammalian metabolism is
  living-world; urea synthesised industrially from natural gas is a made-world batch of a
  living-world pattern. (Wöhler 1828 is the founding counterexample.) Same with
  biomanufactured insulin, synthesised vitamins, cultured proteins.
- Human-**engineered living entities** are dual-membership: `bio:LivingEntity` ∧
  `made:EngineeredLineage` — alive, but with lineage authored through engineering acts:
  GM crops, CRISPR-edited organisms, synthetic genomes (JCVI-syn line), engineered cell
  lines and chassis strains, iGEM/BioBrick constructs.
- A lineage-kind axis grades the engineered end: `wild` / `domesticated-or-bred`
  (cultivars, breeds, landraces — slow engineering, still living-world but human-shaped)
  / `engineered` (directed molecular/organismal modification).
- **Engineered entities are born entangled**: GM traits and constructs commonly carry
  patent claims and regulatory registration (in AU: OGTR — Office of the Gene Technology
  Regulator, Gene Technology Act dealings). Their rights profile therefore includes IP
  and biosafety instruments from creation — before any commercial act — which DOM-AX-1's
  act-rule must accommodate as a second, engineering-origin path into made-world
  relations.

### DOM-AX-2 Everything assertable has time

Relations carry **validity intervals** (Allen-interval machinery exists in-engine);
entities carry temporal aspects (first/last observation, description date, extinction
date, naming periods). Historical states are first-class, queryable, and never
overwriteable — a superseded border or synonym remains as a dated record, linked by
`prov:wasRevisionOf` / temporal succession.

### DOM-AX-3 "Border" is a kind-family, versioned over time

A border/boundary entity = `(kind, geometry-version, validity-interval, authority)`:

| Kind | Examples | Source class |
|---|---|---|
| Jurisdictional | state/Territory borders, LGAs, postcodes, territorial seas | official spatial datasets; change events (Federation 1901, Jervis Bay transfers…) |
| Custodial | native-title determinations, ILUAs, IPAs, language-group regions | NNTT, AIATSIS, CAPAD |
| Ecological | IBRA bioregions, NVIS, ecoregions, habitats | DCCEEW/TERN layers |
| Linguistic | AUSTLANG language areas | AIATSIS |
| Cadastral/tenure | parcels, leases | state cadastres (context only) |
| Historical/replaced | former shires, colonies, missionary boundaries | archival sources |

Borders of different kinds **coexist and overlap**; none is "the real" boundary; a place
is legitimately inside several at once. Geometry versions chain over time; queries take a
temporal parameter (`asOf`). This is also the model for historical *references* — place
names, species names, statuses change; each has a period of use.

### DOM-AX-4 Scale is a first-class axis

The ladder (each rung `eco:containedIn` the next):

```
molecule → macromolecular complex → organelle → cell → tissue → organ
→ organism → population → community → ecosystem → landscape
→ ecoregion → biome → biosphere
```

(with `soc:` overlay at every rung — a molecule has custodians of its knowledge; a biome
has custodial nations). Collections are organised by scale band
(nano / micro / meso / macro / planetary); "scale journeys" navigate
`contains`/`scaleOf` relations across them. Interfaces must let a user traverse
rainforest → beetle → antenna cell → receptor protein without leaving the graph.

### DOM-AX-5 Use-knowledge is language-bound

Medicinal, functional, food, material, and technological **uses of biosphere entities**
frequently live inside indigenous language and teaching — the use relation therefore
attaches through the *concept-in-language* term (SOC-AX-9), not flatly to the taxon:
`(taxon) — hasUse → (use) — recordedIn → (term in language X) — heldBy → (knowledge-holder)`.
Use-knowledge inherits TK-label conditions and may be sacred/restricted — deontic view
conditions apply. Public-domain ethnographic sources (e.g. Maiden 1889) seed the commons
lane; community-held knowledge stays under community authority.

## 3. Core entity classes (draft register)

| Class | Domain | Notes |
|---|---|---|
| `Taxon` / `TaxonName` | bio | name ≠ concept; NSL LSIDs canonical in AU |
| `Organism` / `Occurrence` | bio | Darwin Core aligned |
| `Trait` / `TraitObservation` | bio | PATO/AusTraits-aligned |
| `CellType`, `Molecule`, `Gene`, `Protein` | bio (micro) | CL/ChEBI/GO/UniProt-aligned |
| `Habitat`, `Ecosystem`, `Biome`, `Ecoregion` | bio×env | IUCN GET / IBRA / NVIS |
| `EnvironmentalLayer` | env | climate/soil/terrain raster+modelled surfaces |
| `Agent` family | soc | GOV-SOC-1 classes; standing axes qualified |
| `Country`, `Place`, `Language` | soc | SOC-AX-2/9 entities |
| `Border` | env×soc | DOM-AX-3 kind-family |
| `Season`/`PhenologicalPhase` | bio×soc | incl. indigenous calendars (SOC-AX-6) |
| `Use` / `UseRelation` | bio↔soc | DOM-AX-5 |
| `Act` (harvest/trade/processing) | soc→made | DOM-AX-1 commodification link |
| `Dataset`, `MediaAsset`, `Model3DAsset` | eco | depiction: `eco:depicts` entity ⇄ asset |
| `Attestation`, `Review` | eco | governance acts as data |
| `StatisticalObservation`, `StatisticalDataSet`, `Methodology`, `Classification` | stat | governed population statistics — Data Cube/SDMX-shaped; see [tech/statistics.md](tech/statistics.md) |
| `Claim` | stat×eco | asserted influence/correlation/causal/policy claims w/ evidence class — never observation-layer (STAT-AX-7) |
| `Stock` (aspect), `LossEvent` | eco×all | time-depth accrual, asymmetry, irreversibility classes — [tech/accrual.md](tech/accrual.md) |
| `ReconciliationEvent`, `DivergenceRecord` | eco | internal↔external view comparison acts — [tech/reconciliation.md](tech/reconciliation.md) |
| `ProbabilityField` | eco×stat | measure over typed state/factor/epistemic axes — probabilistic claims as fields, not scalars — [tech/probability-geometry.md](tech/probability-geometry.md) |
| `GeometricStructure`, `Transport` (friction) | eco | fabric-of-geometries members and typed boundary crossings — declared cost/loss/gating, asymmetric, impermeable allowed — [tech/probability-geometry.md](tech/probability-geometry.md) §5A |
| `ChoiceField`, `BenefitProfile`, `ConstraintLayer` | eco×soc | admissible-action regions carved by physics→economics→rights→capacity layers; benefits/costs as typed profiles, never scalars — [tech/probability-geometry.md](tech/probability-geometry.md) §5B |

## 4. Relation families

`taxonomic` (subClassOf/synonymOf/circumscription) · `meronomic` (partOf at every scale) ·
`ecological` (eats/pollinates/hostOf/parasiteOf/mutualistOf/competesWith) · `spatial`
(occursIn/nativeTo/endemicTo/introducedTo — each with validity interval + evidence) ·
`temporal` (activeIn/phenologicalPhase/historicalRange) · `scalar` (contains/scaleOf) ·
`depictive` (depicts/references) · `custodial` (soc→place/taxon/landscape) ·
`use` (bio↔soc via language term) · `commodifying` (act→Artefact) · `provenance`
(prov:*) · `rights` (rightsProfile/heldBy/borneBy) · `statistical`
(measures/dimensionOf/aggregatedFrom/vintageOf/classifiedBy — observations about
populations, versioned geographies, governed classifications) · `influence`
(assertsCorrelatesWith/causesPer — claim relations with evidence class, never
silent derivation) · `interpretive` (interpretsVia/receiptedBy/producesClaim —
dataset × instrument × axiom-context bindings; see tech/interpretation-frameworks.md)

## 5. Spatio-temporal engine fit (verified)

- Allen interval algebra implemented (`modalities/spatio_temporal.rs`, 7 relations) —
  validity intervals evaluate natively.
- GeoSPARQL executes real WKT distance/topology — border/versioned-geometry queries are
  native.
- Semantic validity time is **data** (triples), distinct from the metadata-field Lamport
  clock (which is sync ordering, not domain time) — do not conflate.
- Historical query pattern: `?border a eco:Border ; eco:kind :jurisdictional ;
  geo:hasGeometry ?g . ?g eco:validDuring "1901-01-01"^^xsd:date` — spec the exact
  reification pattern in ontology-stack (named-graph-per-version vs dated-geometry
  entities: open question §7-2).

## 6. What this model deliberately refuses

- A species is never `schema:Product`; commodification is an act-linked derived status.
- A border of one kind never silently outranks another kind.
- "Historical" ≠ "discarded": extinct species, past ranges, superseded names and borders
  remain queryable first-class records.
- The made world does not claim the living world; the direction of modelling respect runs
  from artefact → living entity (`depicts`, `derivedFrom`), not the reverse.

## 7. Open questions

1. **Naming for the living half.** Candidates: `living world`, `the given` (datum/factum
   pair with "the made"), `natural world`, `wild`. Needs the user's pick — less
   theological than "god", no over-complication.
2. Temporal reification pattern — dated geometry entities vs named-graph-per-version.
   Test against IBRA v5→v7 transitions.
3. Which "use" vocabularies already exist (ethnobotany/economic-botany ontologies,
   FOODON use-facets) — survey before minting.
4. How deep `env:` goes (full Earth-system vs context-only) — the earth layers are
   context for the living graph, not a climate-science product.
